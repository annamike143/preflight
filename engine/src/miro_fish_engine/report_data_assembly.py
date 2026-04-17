from __future__ import annotations

import json
from dataclasses import asdict, dataclass
from pathlib import Path

from .bounded_memory import load_bounded_memory_foundation
from .token_duration_tracking import load_token_duration_tracking
from .transcript_archive import ARCHIVE_INDEX_FILE_NAME, SPEAKER_TURNS_FILE_NAME

REPORT_GENERATION_DIRECTORY_NAME = "report-generation"
ASSEMBLED_REPORT_DATA_FILE_NAME = "assembled_report_data.json"
REPORT_DATA_VERSION = "phase8_report_data_v1"

REPORT_GENERATION_STATUS_SUCCEEDED = "succeeded"
REPORT_GENERATION_STATUS_FAILED = "failed"

FAILURE_REASON_MISSING_ARTIFACT_INPUT = "missing_artifact_input"
FAILURE_REASON_MALFORMED_ARTIFACT_INPUT = "malformed_artifact_input"
FAILURE_REASON_INSUFFICIENT_COMPLETED_MATERIAL = "insufficient_completed_material"
FAILURE_REASON_SECTION_ASSEMBLY_FAILED = "section_assembly_failed"

REQUIRED_SECTION_COUNT = 7


class ReportDataAssemblyError(RuntimeError):
    def __init__(self, failure_reason: str, detail_message: str) -> None:
        super().__init__(detail_message)
        self.failure_reason = failure_reason
        self.detail_message = detail_message


@dataclass(frozen=True)
class ReportMetadata:
    report_identifier: str
    run_outcome_classification: str
    report_generation_status: str
    report_availability_status: str
    generated_at: int
    constrained_completion_flag: bool
    token_cost_summary_reference: str
    artifact_location_reference: str


@dataclass(frozen=True)
class ExecutiveSummarySection:
    summary_heading: str
    high_level_synthesis_body: str
    summary_scope_note: str | None
    completed_material_note: str


@dataclass(frozen=True)
class MarketViabilityScoreSection:
    score_label: str
    score_value: int
    score_range_min: int
    score_range_max: int
    scoring_actor_reference: str
    constrained_context_note: str | None


@dataclass(frozen=True)
class RankedRiskEntry:
    rank: int
    risk_title: str
    risk_type: str
    risk_description: str
    originating_actor_class_reference: str | None
    emphasis_note: str


@dataclass(frozen=True)
class RankedRisksSection:
    ranked_risks: tuple[RankedRiskEntry, ...]
    completeness_note: str | None


@dataclass(frozen=True)
class ArgumentEntry:
    argument_title: str
    argument_body: str
    argument_side: str
    source_actor_class_reference: str | None
    supporting_debate_linkage_note: str


@dataclass(frozen=True)
class StrongestArgumentsSection:
    arguments: tuple[ArgumentEntry, ...]
    completeness_note: str | None


@dataclass(frozen=True)
class CostAndTokenSummarySection:
    token_budget_requested: int | None
    token_usage_total: int
    cost_estimate_or_actual_cost_value: float | None
    cost_currency_or_cost_unit_note: str
    duration_elapsed_reference_ms: int
    constrained_stop_reason_reference: str | None


@dataclass(frozen=True)
class TranscriptAppendixEntry:
    sequence_reference: int
    ordered_occurrence_reference: int | None
    round_number: int
    turn_index: int
    speaker_reference: str
    speaker_perspective: str | None
    event_body_content: str


@dataclass(frozen=True)
class TranscriptAppendixSection:
    ordered_transcript_entries: tuple[TranscriptAppendixEntry, ...]
    appendix_completeness_note: str | None


@dataclass(frozen=True)
class AssembledReportSections:
    executive_summary: ExecutiveSummarySection
    market_viability_score: MarketViabilityScoreSection
    ranked_risks: RankedRisksSection
    strongest_pro_arguments: StrongestArgumentsSection
    strongest_anti_arguments: StrongestArgumentsSection
    cost_and_token_summary: CostAndTokenSummarySection
    transcript_appendix: TranscriptAppendixSection


@dataclass(frozen=True)
class AssembledReportData:
    report_data_version: str
    report_metadata: ReportMetadata
    report_sections: AssembledReportSections


@dataclass(frozen=True)
class ReportDataArtifact:
    report_data_path: str
    report_generation_status: str
    report_generation_failure_reason: str | None
    report_generation_detail_message: str
    required_section_count: int
    generated_at: int


def assemble_report_data_artifact(
    *,
    workspace_root: Path,
    bounded_memory_dir: Path,
    transcript_archive_dir: Path,
    token_duration_tracking_path: Path,
    execution_phase_path: Path,
    moderator_persona_label: str,
    completion_state: str,
    requested_round_count: int,
    completed_round_count: int,
    requested_token_budget: int,
    runtime_cap_reason: str | None,
    generated_at: int,
) -> ReportDataArtifact:
    try:
        if not execution_phase_path.exists():
            raise ReportDataAssemblyError(
                FAILURE_REASON_MISSING_ARTIFACT_INPUT,
                "Report data assembly could not find the execution-phase artifact.",
            )
        memory = load_bounded_memory_foundation(bounded_memory_dir)
        tracking_state = load_token_duration_tracking(token_duration_tracking_path)
        archive_index_payload = json.loads(
            (transcript_archive_dir / ARCHIVE_INDEX_FILE_NAME).read_text(encoding="utf-8")
        )
        transcript_turns = load_transcript_turns(
            transcript_archive_dir / SPEAKER_TURNS_FILE_NAME
        )
    except ReportDataAssemblyError:
        raise
    except FileNotFoundError as error:
        raise ReportDataAssemblyError(
            FAILURE_REASON_MISSING_ARTIFACT_INPUT,
            f"Report data assembly could not load required run-local artifact: {error.filename}",
        ) from error
    except json.JSONDecodeError as error:
        raise ReportDataAssemblyError(
            FAILURE_REASON_MALFORMED_ARTIFACT_INPUT,
            f"Report data assembly could not decode a required run-local artifact: {error}",
        ) from error

    if not transcript_turns:
        raise ReportDataAssemblyError(
            FAILURE_REASON_INSUFFICIENT_COMPLETED_MATERIAL,
            (
                "Report data assembly requires at least one actual transcript turn "
                "from the completed run."
            ),
        )

    indexed_total_turns = int(archive_index_payload.get("total_turns") or 0)
    if indexed_total_turns != len(transcript_turns):
        raise ReportDataAssemblyError(
            FAILURE_REASON_MALFORMED_ARTIFACT_INPUT,
            (
                "Transcript archive index and transcript turn material disagree "
                "about the number of recorded turns."
            ),
        )

    if tracking_state.total_turns_tracked != len(transcript_turns):
        raise ReportDataAssemblyError(
            FAILURE_REASON_MALFORMED_ARTIFACT_INPUT,
            (
                "Token-duration tracking and transcript archive material disagree "
                "about the number of recorded turns."
            ),
        )

    constrained_completion_flag = completion_state == "completed_budget_capped"
    unique_arguments = dedupe_preserving_order(memory.argument_risk.arguments)
    unique_objections = dedupe_preserving_order(memory.argument_risk.objections)
    unique_risks = dedupe_preserving_order(memory.argument_risk.risks)
    latest_continuity_summary = (
        memory.round_summary.latest_round_summary
        or "No bounded round summary was preserved."
    )
    executive_summary_body = (
        f"This run debated {memory.seed_grounding.seed_source_name} across "
        f"{completed_round_count} completed rounds and {len(transcript_turns)} "
        "attributed turns. "
        f"Bounded continuity closed on: {latest_continuity_summary} "
        f"The preserved finalization basis includes {len(unique_arguments)} pro "
        f"arguments, {len(unique_objections)} anti arguments, and "
        f"{len(unique_risks)} ranked risk candidates drawn only from this run's "
        "actual artifacts."
    )

    executive_summary = ExecutiveSummarySection(
        summary_heading="Executive Summary",
        high_level_synthesis_body=executive_summary_body,
        summary_scope_note=(
            "Only "
            f"{completed_round_count} of {requested_round_count} requested rounds "
            "completed before the governed runtime cap, so the synthesis "
            "reflects constrained completion only."
            if constrained_completion_flag
            else None
        ),
        completed_material_note=(
            f"Transcript appendix coverage is bounded to {len(transcript_turns)} "
            "recorded turns and the continuity material actually preserved "
            "during execution."
        ),
    )

    score_value = compute_viability_score(
        argument_count=len(unique_arguments),
        objection_count=len(unique_objections),
        risk_count=len(unique_risks),
        completed_round_count=completed_round_count,
        requested_round_count=requested_round_count,
        constrained_completion_flag=constrained_completion_flag,
    )
    score_range_delta = 8 if constrained_completion_flag else 5
    market_viability_score = MarketViabilityScoreSection(
        score_label=score_label_for_value(score_value),
        score_value=score_value,
        score_range_min=max(0, score_value - score_range_delta),
        score_range_max=min(100, score_value + score_range_delta),
        scoring_actor_reference=moderator_persona_label,
        constrained_context_note=(
            f"A governed {runtime_cap_reason or 'runtime'} cap limited "
            "completion before all requested rounds were debated."
            if constrained_completion_flag
            else None
        ),
    )

    ranked_risk_entries = tuple(
        RankedRiskEntry(
            rank=index,
            risk_title=derive_entry_title(risk_text),
            risk_type=classify_risk_type(risk_text),
            risk_description=risk_text,
            originating_actor_class_reference=extract_actor_reference(risk_text),
            emphasis_note=(
                "Preserved in ranked order from actual skeptical or analytical "
                "continuity material only."
            ),
        )
        for index, risk_text in enumerate(unique_risks[:3], start=1)
    )
    ranked_risks = RankedRisksSection(
        ranked_risks=ranked_risk_entries,
        completeness_note=section_completeness_note(
            actual_count=len(ranked_risk_entries),
            expected_count=3,
            section_name="ranked risk entries",
        ),
    )

    strongest_pro_arguments = StrongestArgumentsSection(
        arguments=tuple(
            build_argument_entry(argument_text, argument_side="pro")
            for argument_text in unique_arguments[:3]
        ),
        completeness_note=section_completeness_note(
            actual_count=min(len(unique_arguments), 3),
            expected_count=3,
            section_name="strongest pro arguments",
        ),
    )
    strongest_anti_arguments = StrongestArgumentsSection(
        arguments=tuple(
            build_argument_entry(argument_text, argument_side="anti")
            for argument_text in unique_objections[:3]
        ),
        completeness_note=section_completeness_note(
            actual_count=min(len(unique_objections), 3),
            expected_count=3,
            section_name="strongest anti arguments",
        ),
    )

    cost_and_token_summary = CostAndTokenSummarySection(
        token_budget_requested=requested_token_budget or None,
        token_usage_total=tracking_state.total_token_units,
        cost_estimate_or_actual_cost_value=None,
        cost_currency_or_cost_unit_note=(
            "Tracked local token units only; no external billing or vendor "
            "price equivalence is claimed in this run-local artifact."
        ),
        duration_elapsed_reference_ms=tracking_state.total_duration_ms,
        constrained_stop_reason_reference=runtime_cap_reason,
    )

    transcript_appendix = TranscriptAppendixSection(
        ordered_transcript_entries=tuple(
            TranscriptAppendixEntry(
                sequence_reference=index,
                ordered_occurrence_reference=turn_payload.get("occurred_at"),
                round_number=int(turn_payload["round_number"]),
                turn_index=int(turn_payload["turn_index"]),
                speaker_reference=str(turn_payload["speaker_label"]),
                speaker_perspective=optional_string(turn_payload.get("speaker_perspective")),
                event_body_content=str(turn_payload["turn_text"]),
            )
            for index, turn_payload in enumerate(transcript_turns, start=1)
        ),
        appendix_completeness_note=(
            "Appendix chronology is bounded to the "
            f"{len(transcript_turns)} actual transcript turns preserved before "
            "constrained completion."
            if constrained_completion_flag
            else None
        ),
    )

    report_generation_dir = workspace_root / REPORT_GENERATION_DIRECTORY_NAME
    report_generation_dir.mkdir(parents=True, exist_ok=True)
    report_data_path = report_generation_dir / ASSEMBLED_REPORT_DATA_FILE_NAME

    assembled_report = AssembledReportData(
        report_data_version=REPORT_DATA_VERSION,
        report_metadata=ReportMetadata(
            report_identifier=f"report-data-{workspace_root.name}-{generated_at}",
            run_outcome_classification=completion_state,
            report_generation_status=REPORT_GENERATION_STATUS_SUCCEEDED,
            report_availability_status="report_unavailable",
            generated_at=generated_at,
            constrained_completion_flag=constrained_completion_flag,
            token_cost_summary_reference="#/report_sections/cost_and_token_summary",
            artifact_location_reference=str(report_data_path),
        ),
        report_sections=AssembledReportSections(
            executive_summary=executive_summary,
            market_viability_score=market_viability_score,
            ranked_risks=ranked_risks,
            strongest_pro_arguments=strongest_pro_arguments,
            strongest_anti_arguments=strongest_anti_arguments,
            cost_and_token_summary=cost_and_token_summary,
            transcript_appendix=transcript_appendix,
        ),
    )

    try:
        report_data_path.write_text(
            json.dumps(asdict(assembled_report), indent=2),
            encoding="utf-8",
        )
    except OSError as error:
        raise ReportDataAssemblyError(
            FAILURE_REASON_SECTION_ASSEMBLY_FAILED,
            f"Report data assembly could not persist the transient report artifact: {error}",
        ) from error

    return ReportDataArtifact(
        report_data_path=str(report_data_path),
        report_generation_status=REPORT_GENERATION_STATUS_SUCCEEDED,
        report_generation_failure_reason=None,
        report_generation_detail_message=(
            "Required report data sections were assembled from actual run "
            f"artifacts and persisted to {report_data_path.name}."
        ),
        required_section_count=REQUIRED_SECTION_COUNT,
        generated_at=generated_at,
    )


def load_transcript_turns(transcript_turns_path: Path) -> list[dict[str, object]]:
    transcript_turns: list[dict[str, object]] = []
    for raw_line in transcript_turns_path.read_text(encoding="utf-8").splitlines():
        if not raw_line.strip():
            continue
        payload = json.loads(raw_line)
        if not isinstance(payload, dict):
            raise ReportDataAssemblyError(
                FAILURE_REASON_MALFORMED_ARTIFACT_INPUT,
                "Transcript appendix material contained a non-object entry.",
            )
        transcript_turns.append(payload)
    return transcript_turns


def compute_viability_score(
    *,
    argument_count: int,
    objection_count: int,
    risk_count: int,
    completed_round_count: int,
    requested_round_count: int,
    constrained_completion_flag: bool,
) -> int:
    completion_ratio = completed_round_count / max(1, requested_round_count)
    raw_score = 50
    raw_score += min(argument_count, 6) * 6
    raw_score -= min(objection_count, 6) * 5
    raw_score -= min(risk_count, 6) * 7
    raw_score += int(round(completion_ratio * 10))
    if constrained_completion_flag:
        raw_score -= 8
    return max(0, min(100, raw_score))


def score_label_for_value(score_value: int) -> str:
    if score_value >= 80:
        return "Strong"
    if score_value >= 60:
        return "Promising"
    if score_value >= 45:
        return "Mixed"
    if score_value >= 25:
        return "Low"
    return "Very Low"


def build_argument_entry(argument_text: str, *, argument_side: str) -> ArgumentEntry:
    return ArgumentEntry(
        argument_title=derive_entry_title(argument_text),
        argument_body=argument_text,
        argument_side=argument_side,
        source_actor_class_reference=extract_actor_reference(argument_text),
        supporting_debate_linkage_note=(
            "Lifted from actual bounded argument continuity preserved during this run only."
        ),
    )


def dedupe_preserving_order(items: tuple[str, ...]) -> tuple[str, ...]:
    unique_items: list[str] = []
    seen_items: set[str] = set()
    for item in items:
        normalized = item.strip()
        if not normalized or normalized in seen_items:
            continue
        unique_items.append(normalized)
        seen_items.add(normalized)
    return tuple(unique_items)


def derive_entry_title(entry_text: str) -> str:
    body = entry_text.split(":", 1)[1].strip() if ":" in entry_text else entry_text.strip()
    words = body.split()
    if not words:
        return "Actual run material"
    if len(words) <= 8:
        return " ".join(words)
    return " ".join(words[:8])


def extract_actor_reference(entry_text: str) -> str | None:
    if ":" not in entry_text:
        return None
    actor_reference = entry_text.split(":", 1)[0].strip()
    return actor_reference or None


def classify_risk_type(risk_text: str) -> str:
    lowered = risk_text.lower()
    if any(token in lowered for token in ("proof", "evidence", "assumption")):
        return "evidence"
    if any(token in lowered for token in ("execution", "operational", "sequence")):
        return "execution"
    if any(token in lowered for token in ("market", "adoption", "demand")):
        return "market"
    if any(token in lowered for token in ("budget", "cost", "spend")):
        return "cost"
    return "general"


def section_completeness_note(
    *,
    actual_count: int,
    expected_count: int,
    section_name: str,
) -> str | None:
    if actual_count >= expected_count:
        return None
    return (
        f"Only {actual_count} actual {section_name} were preserved in bounded "
        "continuity for this run, so the section remains partial rather than "
        "padded with invented material."
    )


def optional_string(value: object) -> str | None:
    if value is None:
        return None
    value_as_string = str(value).strip()
    return value_as_string or None