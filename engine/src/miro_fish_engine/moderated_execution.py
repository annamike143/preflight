from __future__ import annotations

import asyncio
import concurrent.futures
import json
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from .bounded_memory import (
    BoundedMemoryFoundation,
    load_bounded_memory_foundation,
    update_bounded_memory_for_finalization,
    update_bounded_memory_for_round,
    write_bounded_memory_foundation,
)
from .openai_client import LLMTurnResponse, MiroFishOpenAIClient
from .prompts import PERSONA_TURN_SYSTEM_PROMPT
from .token_duration_tracking import (
    ExecutionPhaseState,
    TokenDurationTrackingState,
    TurnTokenDurationRecord,
    estimate_token_units,
    record_turn_tracking,
    write_execution_phase,
    write_token_duration_tracking,
)
from .transcript_archive import append_speaker_turn, update_transcript_archive_index

TOKEN_BUDGET_CAP_REASON = "token_budget"
DURATION_CAP_REASON = "duration_cap"


class ControlledCancellationError(RuntimeError):
    def __init__(self, *, shutdown_requested: bool, detail_message: str) -> None:
        super().__init__(detail_message)
        self.shutdown_requested = shutdown_requested


@dataclass(frozen=True)
class ParticipantRuntimeState:
    stable_label: str
    perspective: str
    focus_note: str


@dataclass(frozen=True)
class RoundExecutionSnapshot:
    round_number: int
    selected_speaker_labels: tuple[str, ...]
    speaker_count: int
    round_summary: str


@dataclass(frozen=True)
class ModeratedExecutionResult:
    bounded_memory: BoundedMemoryFoundation
    token_tracking_state: TokenDurationTrackingState
    completed_round_count: int
    total_turn_count: int
    speaker_counts_per_round: tuple[int, ...]
    latest_round_summary: str | None
    speaker_attribution_materialized: bool
    normal_round_speaker_selection_enforced: bool
    bounded_memory_continuity_mutated: bool
    token_duration_tracking_materialized: bool
    completion_state: str
    runtime_cap_reason: str | None
    controlled_finalization_performed: bool
    finalization_started_at: int | None
    finalization_completed_at: int | None
    rounds: tuple[RoundExecutionSnapshot, ...]


def execute_moderated_rounds(
    *,
    moderator_basis: dict[str, Any],
    participant_personas: list[dict[str, Any]],
    seed_grounding: dict[str, Any],
    bounded_memory_dir: Path,
    transcript_archive_dir: Path,
    token_duration_tracking_path: Path,
    initial_token_tracking_state: TokenDurationTrackingState,
    requested_round_count: int,
    max_token_budget: int,
    max_duration_cap_ms: int,
    execution_control_path: Path | None = None,
    execution_phase_path: Path | None = None,
    openai_client: MiroFishOpenAIClient | None = None,
) -> ModeratedExecutionResult:
    if openai_client is None:
        openai_client = MiroFishOpenAIClient()

    participants = tuple(
        ParticipantRuntimeState(
            stable_label=persona["stable_label"],
            perspective=persona["perspective"],
            focus_note=persona["focus_note"],
        )
        for persona in participant_personas
    )
    round_count = max(1, int(requested_round_count))
    min_speakers = int(moderator_basis.get("normal_round_min_speakers", 3))
    max_speakers = int(moderator_basis.get("normal_round_max_speakers", 5))
    memory = load_bounded_memory_foundation(bounded_memory_dir)
    tracking_state = initial_token_tracking_state
    seed_excerpt = str(seed_grounding.get("seed_excerpt", ""))
    topic_keywords = tuple(
        str(keyword) for keyword in seed_grounding.get("topic_keywords") or ("seed-context",)
    )
    speaker_counts_per_round: list[int] = []
    round_snapshots: list[RoundExecutionSnapshot] = []
    cap_reason: str | None = None
    cap_triggered_round_number: int | None = None
    cap_selected_speakers: tuple[ParticipantRuntimeState, ...] = ()
    cap_round_arguments: list[str] = []
    cap_round_objections: list[str] = []
    cap_round_risks: list[str] = []
    cap_interrupted_mid_round = False

    if execution_phase_path is not None:
        write_execution_phase(
            execution_phase_path,
            ExecutionPhaseState(
                phase_version="wave8_execution_phase_v1",
                current_phase="running_round",
                phase_changed_at=epoch_ms_now(),
                cap_reason=None,
                detail_message=(
                    "Moderator-controlled execution is actively scheduling bounded normal rounds."
                ),
                finalization_started_at=None,
                finalization_completed_at=None,
                report_generation_started_at=None,
                report_generation_completed_at=None,
                report_generation_failed_at=None,
                report_generation_failure_reason=None,
            ),
        )

    for round_number in range(1, round_count + 1):
        raise_if_execution_interrupted(execution_control_path)
        runtime_cap_reason = resolve_runtime_cap_reason(
            tracking_state,
            max_token_budget=max_token_budget,
            max_duration_cap_ms=max_duration_cap_ms,
        )
        if runtime_cap_reason is not None:
            cap_reason = runtime_cap_reason
            cap_triggered_round_number = round_number
            break

        selected_speakers = select_round_speakers(
            participants,
            round_number=round_number,
            min_speakers=min_speakers,
            max_speakers=max_speakers,
        )
        prior_summary = memory.round_summary.latest_round_summary
        round_arguments: list[str] = []
        round_objections: list[str] = []
        round_risks: list[str] = []

        for turn_index, participant in enumerate(selected_speakers, start=1):
            raise_if_execution_interrupted(execution_control_path)
            turn_started_at = time.perf_counter_ns()
            turn_text: str | None = None
            llm_usage = None

            if openai_client and openai_client.is_configured:
                try:
                    sys_prompt = PERSONA_TURN_SYSTEM_PROMPT.format(
                        persona_name=participant.stable_label,
                        stable_label=participant.stable_label,
                        perspective=participant.perspective,
                        bias_description=participant.perspective,
                        focus_domain=participant.focus_note,
                        key_critique_vector=participant.focus_note,
                    )
                    user_msg = (
                        f"Seed Excerpt:\n{seed_excerpt[:600]}\n\n"
                        f"Round {round_number}, Turn {turn_index}.\n"
                        f"Prior Round Context:\n{prior_summary or 'Opening round.'}\n\n"
                        f"State your direct, critical perspective on the seed assumptions."
                    )
                    try:
                        loop = asyncio.get_event_loop()
                    except RuntimeError:
                        loop = asyncio.new_event_loop()
                        asyncio.set_event_loop(loop)

                    if loop.is_running():
                        with concurrent.futures.ThreadPoolExecutor(max_workers=1) as pool:
                            turn_resp: LLMTurnResponse = pool.submit(
                                asyncio.run,
                                openai_client.generate_chat_turn(
                                    system_prompt=sys_prompt,
                                    messages=[{"role": "user", "content": user_msg}],
                                ),
                            ).result()
                    else:
                        turn_resp = loop.run_until_complete(
                            openai_client.generate_chat_turn(
                                system_prompt=sys_prompt,
                                messages=[{"role": "user", "content": user_msg}],
                            )
                        )
                    turn_text = turn_resp.content
                    llm_usage = turn_resp.usage
                except Exception:
                    turn_text = None

            if turn_text is None:
                turn_text = build_turn_text(
                    participant,
                    round_number=round_number,
                    turn_index=turn_index,
                    seed_excerpt=seed_excerpt,
                    topic_keywords=topic_keywords,
                    prior_summary=prior_summary,
                )

            duration_ms = max(1, int((time.perf_counter_ns() - turn_started_at) / 1_000_000))
            occurred_at = max(1, int(time.time_ns() / 1_000_000))
            if llm_usage is not None:
                input_token_units = llm_usage.input_tokens
                output_token_units = llm_usage.output_tokens
            else:
                input_token_units = estimate_token_units(
                    seed_excerpt,
                    participant.focus_note,
                    prior_summary or "",
                    " ".join(memory.argument_risk.arguments[-3:]),
                    " ".join(memory.argument_risk.objections[-3:]),
                    " ".join(memory.argument_risk.risks[-3:]),
                )
                output_token_units = estimate_token_units(turn_text)
            turn_record = TurnTokenDurationRecord(
                round_number=round_number,
                turn_index=turn_index,
                speaker_label=participant.stable_label,
                input_token_units=input_token_units,
                output_token_units=output_token_units,
                total_token_units=input_token_units + output_token_units,
                duration_ms=duration_ms,
                occurred_at=occurred_at,
            )
            tracking_state = record_turn_tracking(tracking_state, turn_record)
            write_token_duration_tracking(token_duration_tracking_path, tracking_state)

            append_speaker_turn(
                transcript_archive_dir,
                {
                    "round_number": round_number,
                    "turn_index": turn_index,
                    "speaker_label": participant.stable_label,
                    "speaker_perspective": participant.perspective,
                    "focus_note": participant.focus_note,
                    "turn_text": turn_text,
                    "input_token_units": turn_record.input_token_units,
                    "output_token_units": turn_record.output_token_units,
                    "total_token_units": turn_record.total_token_units,
                    "duration_ms": turn_record.duration_ms,
                    "occurred_at": turn_record.occurred_at,
                },
            )

            arguments, objections, risks = classify_turn_contributions(
                participant,
                round_number=round_number,
                turn_index=turn_index,
                topic_keywords=topic_keywords,
            )
            round_arguments.extend(arguments)
            round_objections.extend(objections)
            round_risks.extend(risks)

            runtime_cap_reason = resolve_runtime_cap_reason(
                tracking_state,
                max_token_budget=max_token_budget,
                max_duration_cap_ms=max_duration_cap_ms,
            )
            if runtime_cap_reason is not None and (
                turn_index < len(selected_speakers) or round_number < round_count
            ):
                cap_reason = runtime_cap_reason
                cap_triggered_round_number = round_number
                cap_selected_speakers = selected_speakers
                cap_round_arguments = list(round_arguments)
                cap_round_objections = list(round_objections)
                cap_round_risks = list(round_risks)
                cap_interrupted_mid_round = turn_index < len(selected_speakers)
                break

        if cap_interrupted_mid_round:
            break

        round_summary = build_round_summary(
            round_number=round_number,
            selected_speakers=selected_speakers,
            topic_keywords=topic_keywords,
            prior_summary=prior_summary,
            arguments=round_arguments,
            objections=round_objections,
            risks=round_risks,
        )
        memory = update_bounded_memory_for_round(
            memory,
            round_summary=round_summary,
            arguments=round_arguments,
            objections=round_objections,
            risks=round_risks,
        )
        write_bounded_memory_foundation(bounded_memory_dir, memory)
        update_transcript_archive_index(
            transcript_archive_dir,
            rounds_completed=round_number,
            total_turns=tracking_state.total_turns_tracked,
        )
        speaker_counts_per_round.append(len(selected_speakers))
        round_snapshots.append(
            RoundExecutionSnapshot(
                round_number=round_number,
                selected_speaker_labels=tuple(
                    speaker.stable_label for speaker in selected_speakers
                ),
                speaker_count=len(selected_speakers),
                round_summary=round_summary,
            )
        )

        if cap_reason is not None:
            break

    finalization_started_at = None
    finalization_completed_at = None
    controlled_finalization_performed = False

    if cap_reason is not None:
        finalization_started_at = epoch_ms_now()
        if execution_phase_path is not None:
            write_execution_phase(
                execution_phase_path,
                ExecutionPhaseState(
                    phase_version="wave8_execution_phase_v1",
                    current_phase="finalizing",
                    phase_changed_at=finalization_started_at,
                    cap_reason=cap_reason,
                    detail_message=build_finalization_detail_message(
                        cap_reason=cap_reason,
                        completed_round_count=len(round_snapshots),
                        requested_round_count=round_count,
                    ),
                    finalization_started_at=finalization_started_at,
                    finalization_completed_at=None,
                    report_generation_started_at=None,
                    report_generation_completed_at=None,
                    report_generation_failed_at=None,
                    report_generation_failure_reason=None,
                ),
            )

        finalization_summary = build_capped_finalization_summary(
            cap_reason=cap_reason,
            completed_round_count=len(round_snapshots),
            requested_round_count=round_count,
            cap_triggered_round_number=cap_triggered_round_number,
            selected_speakers=cap_selected_speakers,
            prior_summary=memory.round_summary.latest_round_summary,
            arguments=cap_round_arguments,
            objections=cap_round_objections,
            risks=cap_round_risks,
        )
        memory = update_bounded_memory_for_finalization(
            memory,
            finalization_summary=finalization_summary,
            arguments=cap_round_arguments,
            objections=cap_round_objections,
            risks=cap_round_risks,
        )
        write_bounded_memory_foundation(bounded_memory_dir, memory)
        update_transcript_archive_index(
            transcript_archive_dir,
            rounds_completed=len(round_snapshots),
            total_turns=tracking_state.total_turns_tracked,
        )
        finalization_completed_at = epoch_ms_now()
        controlled_finalization_performed = True

        if execution_phase_path is not None:
            write_execution_phase(
                execution_phase_path,
                ExecutionPhaseState(
                    phase_version="wave8_execution_phase_v1",
                    current_phase="finalizing",
                    phase_changed_at=finalization_completed_at,
                    cap_reason=cap_reason,
                    detail_message=build_report_generation_handoff_detail_message(
                        cap_reason=cap_reason,
                        completed_round_count=len(round_snapshots),
                        requested_round_count=round_count,
                    ),
                    finalization_started_at=finalization_started_at,
                    finalization_completed_at=finalization_completed_at,
                    report_generation_started_at=None,
                    report_generation_completed_at=None,
                    report_generation_failed_at=None,
                    report_generation_failure_reason=None,
                ),
            )

    normal_round_speaker_selection_enforced = bool(speaker_counts_per_round) and all(
        min_speakers <= speaker_count <= min(max_speakers, len(participants))
        for speaker_count in speaker_counts_per_round
    )
    bounded_memory_continuity_mutated = memory.round_summary.latest_round_summary is not None

    return ModeratedExecutionResult(
        bounded_memory=memory,
        token_tracking_state=tracking_state,
        completed_round_count=len(round_snapshots),
        total_turn_count=tracking_state.total_turns_tracked,
        speaker_counts_per_round=tuple(speaker_counts_per_round),
        latest_round_summary=memory.round_summary.latest_round_summary,
        speaker_attribution_materialized=tracking_state.total_turns_tracked > 0,
        normal_round_speaker_selection_enforced=normal_round_speaker_selection_enforced,
        bounded_memory_continuity_mutated=bounded_memory_continuity_mutated,
        token_duration_tracking_materialized=tracking_state.total_turns_tracked > 0,
        completion_state=("completed_budget_capped" if cap_reason is not None else "completed"),
        runtime_cap_reason=cap_reason,
        controlled_finalization_performed=controlled_finalization_performed,
        finalization_started_at=finalization_started_at,
        finalization_completed_at=finalization_completed_at,
        rounds=tuple(round_snapshots),
    )


def raise_if_execution_interrupted(execution_control_path: Path | None) -> None:
    if execution_control_path is None or not execution_control_path.exists():
        return

    payload = json.loads(execution_control_path.read_text(encoding="utf-8"))
    shutdown_requested = bool(payload.get("shutdown_requested"))
    cancellation_requested = bool(payload.get("cancellation_requested"))

    if shutdown_requested:
        raise ControlledCancellationError(
            shutdown_requested=True,
            detail_message=(
                "shell-supervised shutdown requested a graceful stop before the next moderated step"
            ),
        )

    if cancellation_requested:
        raise ControlledCancellationError(
            shutdown_requested=False,
            detail_message=(
                "shell-owned cancellation requested a graceful stop before the next moderated step"
            ),
        )


def select_round_speakers(
    participants: tuple[ParticipantRuntimeState, ...],
    *,
    round_number: int,
    min_speakers: int,
    max_speakers: int,
) -> tuple[ParticipantRuntimeState, ...]:
    if len(participants) < min_speakers:
        raise ValueError("moderated rounds require at least three participants")

    speaker_window = max_speakers - min_speakers + 1
    target_speaker_count = min(
        len(participants),
        min_speakers + ((round_number - 1) % speaker_window),
    )
    selected: list[ParticipantRuntimeState] = []
    favorable = tuple(
        participant for participant in participants if participant.perspective == "favorable"
    )
    skeptical = tuple(
        participant for participant in participants if participant.perspective == "skeptical"
    )
    analytical_or_neutral = tuple(
        participant
        for participant in participants
        if participant.perspective in {"analytical", "neutral"}
    )

    for group_offset, group in enumerate((favorable, skeptical, analytical_or_neutral)):
        if not group:
            continue
        candidate = group[(round_number - 1 + group_offset) % len(group)]
        append_unique_participant(selected, candidate)

    for participant in rotated_participants(participants, round_number - 1):
        if len(selected) >= target_speaker_count:
            break
        append_unique_participant(selected, participant)

    return tuple(selected[:target_speaker_count])


def build_turn_text(
    participant: ParticipantRuntimeState,
    *,
    round_number: int,
    turn_index: int,
    seed_excerpt: str,
    topic_keywords: tuple[str, ...],
    prior_summary: str | None,
) -> str:
    primary_topic, secondary_topic = resolve_topic_pair(topic_keywords, round_number + turn_index)
    seed_anchor = excerpt_window(seed_excerpt, round_number + turn_index)
    prior_clause = resolve_prior_clause(prior_summary)

    if participant.perspective == "favorable":
        return (
            f"{participant.stable_label} argues that {primary_topic} is the clearest upside "
            f"because the seed grounds it in {seed_anchor}. {prior_clause} The leverage point "
            f"to protect next is {secondary_topic}."
        )

    if participant.perspective == "skeptical":
        return (
            f"{participant.stable_label} warns that {primary_topic} still rests on unproven "
            f"assumptions, especially where the seed says {seed_anchor}. {prior_clause} The "
            f"group should pressure-test {secondary_topic} before escalating confidence."
        )

    if participant.perspective == "analytical":
        return (
            f"{participant.stable_label} separates evidence from inference by tying "
            f"{primary_topic} back to {seed_anchor}. {prior_clause} The next proof obligation "
            f"is to validate {secondary_topic} with bounded evidence."
        )

    return (
        f"{participant.stable_label} converts {primary_topic} into a bounded next move using "
        f"the seed anchor {seed_anchor}. {prior_clause} The operational sequence should keep "
        f"{secondary_topic} visible while decisions stay grounded."
    )


def classify_turn_contributions(
    participant: ParticipantRuntimeState,
    *,
    round_number: int,
    turn_index: int,
    topic_keywords: tuple[str, ...],
) -> tuple[tuple[str, ...], tuple[str, ...], tuple[str, ...]]:
    primary_topic, secondary_topic = resolve_topic_pair(topic_keywords, round_number + turn_index)

    if participant.perspective == "favorable":
        return (
            (
                f"{participant.stable_label}: advance {primary_topic} because it can compound "
                f"through {secondary_topic}.",
            ),
            (),
            (),
        )

    if participant.perspective == "skeptical":
        return (
            (),
            (
                f"{participant.stable_label}: the case for {primary_topic} is still too soft "
                f"without stronger proof.",
            ),
            (
                f"{participant.stable_label}: unmanaged downside around {secondary_topic} could "
                f"break the run thesis.",
            ),
        )

    if participant.perspective == "analytical":
        return (
            (
                f"{participant.stable_label}: validate {primary_topic} with concrete evidence "
                f"before broadening scope.",
            ),
            (),
            (
                f"{participant.stable_label}: evidence gaps around {secondary_topic} remain an "
                f"execution risk.",
            ),
        )

    return (
        (
            f"{participant.stable_label}: sequence a bounded next move for {primary_topic} and "
            f"keep {secondary_topic} in the operating plan.",
        ),
        (),
        (),
    )


def build_round_summary(
    *,
    round_number: int,
    selected_speakers: tuple[ParticipantRuntimeState, ...],
    topic_keywords: tuple[str, ...],
    prior_summary: str | None,
    arguments: list[str],
    objections: list[str],
    risks: list[str],
) -> str:
    primary_topic, secondary_topic = resolve_topic_pair(topic_keywords, round_number)
    selected_labels = ", ".join(speaker.stable_label for speaker in selected_speakers)
    continuity_anchor = compact_text(
        objections[0] if objections else arguments[0] if arguments else risks[0] if risks else "",
        limit_words=14,
    )
    prior_clause = resolve_prior_clause(prior_summary)
    return (
        f"Round {round_number} kept {selected_labels} focused on {primary_topic} and "
        f"{secondary_topic}. {prior_clause} Continuity now carries forward {continuity_anchor}."
    )


def build_capped_finalization_summary(
    *,
    cap_reason: str,
    completed_round_count: int,
    requested_round_count: int,
    cap_triggered_round_number: int | None,
    selected_speakers: tuple[ParticipantRuntimeState, ...],
    prior_summary: str | None,
    arguments: list[str],
    objections: list[str],
    risks: list[str],
) -> str:
    selected_labels = ", ".join(speaker.stable_label for speaker in selected_speakers)
    continuity_source = (
        objections[0]
        if objections
        else arguments[0]
        if arguments
        else risks[0]
        if risks
        else (prior_summary or "")
    )
    continuity_anchor = compact_text(
        continuity_source,
        limit_words=16,
    )
    round_clause = (
        f"during Round {cap_triggered_round_number}"
        if isinstance(cap_triggered_round_number, int)
        else "before another bounded round could start"
    )
    speaker_clause = (
        f" with {selected_labels} in the active speaker set"
        if selected_labels
        else ""
    )
    return (
        "Controlled finalization stopped new normal rounds "
        f"{round_clause} after the {cap_reason_label(cap_reason)}"
        f"{speaker_clause}. "
        f"Completed rounds remain {completed_round_count} of {requested_round_count}. "
        f"{resolve_prior_clause(prior_summary)} Continuity now carries forward "
        f"{continuity_anchor}."
    )


def resolve_runtime_cap_reason(
    tracking_state: TokenDurationTrackingState,
    *,
    max_token_budget: int,
    max_duration_cap_ms: int,
) -> str | None:
    if max_token_budget > 0 and tracking_state.total_token_units >= max_token_budget:
        return TOKEN_BUDGET_CAP_REASON
    if max_duration_cap_ms > 0 and tracking_state.total_duration_ms >= max_duration_cap_ms:
        return DURATION_CAP_REASON
    return None


def build_finalization_detail_message(
    *,
    cap_reason: str,
    completed_round_count: int,
    requested_round_count: int,
) -> str:
    return (
        f"The {cap_reason_label(cap_reason)} was reached after "
        f"{completed_round_count} of {requested_round_count} completed rounds, "
        "so normal round scheduling has stopped and constrained finalization is "
        "preserving bounded continuity."
    )


def build_capped_completion_detail_message(
    *,
    cap_reason: str,
    completed_round_count: int,
    requested_round_count: int,
) -> str:
    return (
        "The run resolved as CompletedBudgetCapped after the "
        f"{cap_reason_label(cap_reason)} truncated further normal rounds at "
        f"{completed_round_count} of {requested_round_count}."
    )


def build_report_generation_handoff_detail_message(
    *,
    cap_reason: str,
    completed_round_count: int,
    requested_round_count: int,
) -> str:
    return (
        "Controlled finalization preserved bounded continuity after the "
        f"{cap_reason_label(cap_reason)} stopped further normal rounds at "
        f"{completed_round_count} of {requested_round_count}; "
        "report data assembly is now the next governed step."
    )


def cap_reason_label(cap_reason: str) -> str:
    if cap_reason == TOKEN_BUDGET_CAP_REASON:
        return "runtime token budget"
    return "hard duration cap"


def epoch_ms_now() -> int:
    return max(1, int(time.time_ns() / 1_000_000))


def resolve_topic_pair(topic_keywords: tuple[str, ...], seed_index: int) -> tuple[str, str]:
    if not topic_keywords:
        return "seed-context", "bounded-execution"
    primary_topic = topic_keywords[(seed_index - 1) % len(topic_keywords)]
    secondary_topic = topic_keywords[seed_index % len(topic_keywords)]
    return primary_topic, secondary_topic


def excerpt_window(seed_excerpt: str, seed_index: int, window_size: int = 14) -> str:
    words = seed_excerpt.split()
    if not words:
        return "the approved seed context"
    start_index = ((seed_index - 1) * 5) % len(words)
    selected_words = words[start_index : start_index + window_size]
    if len(selected_words) < window_size:
        selected_words.extend(words[: window_size - len(selected_words)])
    return " ".join(selected_words)


def resolve_prior_clause(prior_summary: str | None) -> str:
    if not prior_summary:
        return "No prior round continuity exists yet."
    return f"Previous continuity emphasized {compact_text(prior_summary, limit_words=14)}."


def compact_text(value: str, *, limit_words: int) -> str:
    words = value.split()
    if not words:
        return "the approved seed context"
    if len(words) <= limit_words:
        return " ".join(words)
    return " ".join(words[:limit_words])


def rotated_participants(
    participants: tuple[ParticipantRuntimeState, ...],
    offset: int,
) -> tuple[ParticipantRuntimeState, ...]:
    start_index = offset % len(participants)
    return participants[start_index:] + participants[:start_index]


def append_unique_participant(
    selected: list[ParticipantRuntimeState],
    candidate: ParticipantRuntimeState,
) -> None:
    if all(existing.stable_label != candidate.stable_label for existing in selected):
        selected.append(candidate)