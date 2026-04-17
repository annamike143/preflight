from __future__ import annotations

import json
from dataclasses import asdict, dataclass, replace
from pathlib import Path
from typing import Iterable

from .runtime_personas import RuntimePersonaFoundation, SeedContext

SEED_GROUNDING_FILE_NAME = "seed_grounding.json"
ROUND_SUMMARY_FILE_NAME = "round_summary.json"
ARGUMENT_RISK_FILE_NAME = "argument_risk_memory.json"
SUMMARY_HISTORY_LIMIT = 8
CONTINUITY_ITEM_LIMIT = 12


@dataclass(frozen=True)
class BoundedMemoryArtifactPaths:
    bounded_memory_directory_path: str
    seed_grounding_memory_path: str
    round_summary_memory_path: str
    argument_risk_memory_path: str


@dataclass(frozen=True)
class SeedGroundingMemory:
    seed_fingerprint_sha256: str
    seed_source_name: str
    seed_excerpt: str
    topic_keywords: tuple[str, ...]
    prompt_context_window_chars: int
    transcript_archive_prompt_source_allowed: bool


@dataclass(frozen=True)
class RoundSummaryMemory:
    completed_rounds: int
    latest_round_summary: str | None
    summary_history: tuple[str, ...]
    continuity_policy: str


@dataclass(frozen=True)
class ArgumentRiskMemory:
    arguments: tuple[str, ...]
    objections: tuple[str, ...]
    risks: tuple[str, ...]
    continuity_policy: str


@dataclass(frozen=True)
class BoundedMemoryFoundation:
    seed_grounding: SeedGroundingMemory
    round_summary: RoundSummaryMemory
    argument_risk: ArgumentRiskMemory
    persona_labels: tuple[str, ...]
    cross_run_semantic_leakage_allowed: bool
    transcript_archive_separated: bool


def initialize_bounded_memory(
    seed_context: SeedContext,
    personas: RuntimePersonaFoundation,
) -> BoundedMemoryFoundation:
    seed_grounding = SeedGroundingMemory(
        seed_fingerprint_sha256=seed_context.fingerprint_sha256,
        seed_source_name=seed_context.source_name,
        seed_excerpt=seed_context.text_excerpt,
        topic_keywords=seed_context.topic_keywords,
        prompt_context_window_chars=len(seed_context.text_excerpt),
        transcript_archive_prompt_source_allowed=False,
    )
    round_summary = RoundSummaryMemory(
        completed_rounds=0,
        latest_round_summary=None,
        summary_history=(),
        continuity_policy="bounded_round_summary_continuity",
    )
    argument_risk = ArgumentRiskMemory(
        arguments=(),
        objections=(),
        risks=(),
        continuity_policy="bounded_argument_and_risk_continuity",
    )
    return BoundedMemoryFoundation(
        seed_grounding=seed_grounding,
        round_summary=round_summary,
        argument_risk=argument_risk,
        persona_labels=personas.stable_labels(),
        cross_run_semantic_leakage_allowed=False,
        transcript_archive_separated=True,
    )


def write_bounded_memory_foundation(
    bounded_memory_dir: Path,
    foundation: BoundedMemoryFoundation,
) -> BoundedMemoryArtifactPaths:
    bounded_memory_dir.mkdir(parents=True, exist_ok=True)
    seed_grounding_memory_path = bounded_memory_dir / SEED_GROUNDING_FILE_NAME
    round_summary_memory_path = bounded_memory_dir / ROUND_SUMMARY_FILE_NAME
    argument_risk_memory_path = bounded_memory_dir / ARGUMENT_RISK_FILE_NAME

    seed_grounding_memory_path.write_text(
        json.dumps(asdict(foundation.seed_grounding), indent=2),
        encoding="utf-8",
    )
    round_summary_memory_path.write_text(
        json.dumps(asdict(foundation.round_summary), indent=2),
        encoding="utf-8",
    )
    argument_risk_memory_path.write_text(
        json.dumps(asdict(foundation.argument_risk), indent=2),
        encoding="utf-8",
    )

    return BoundedMemoryArtifactPaths(
        bounded_memory_directory_path=str(bounded_memory_dir),
        seed_grounding_memory_path=str(seed_grounding_memory_path),
        round_summary_memory_path=str(round_summary_memory_path),
        argument_risk_memory_path=str(argument_risk_memory_path),
    )


def load_bounded_memory_foundation(bounded_memory_dir: Path) -> BoundedMemoryFoundation:
    seed_grounding_payload = json.loads(
        (bounded_memory_dir / SEED_GROUNDING_FILE_NAME).read_text(encoding="utf-8")
    )
    round_summary_payload = json.loads(
        (bounded_memory_dir / ROUND_SUMMARY_FILE_NAME).read_text(encoding="utf-8")
    )
    argument_risk_payload = json.loads(
        (bounded_memory_dir / ARGUMENT_RISK_FILE_NAME).read_text(encoding="utf-8")
    )

    seed_grounding = SeedGroundingMemory(**seed_grounding_payload)
    round_summary = RoundSummaryMemory(
        completed_rounds=round_summary_payload["completed_rounds"],
        latest_round_summary=round_summary_payload.get("latest_round_summary"),
        summary_history=tuple(round_summary_payload.get("summary_history", [])),
        continuity_policy=round_summary_payload["continuity_policy"],
    )
    argument_risk = ArgumentRiskMemory(
        arguments=tuple(argument_risk_payload.get("arguments", [])),
        objections=tuple(argument_risk_payload.get("objections", [])),
        risks=tuple(argument_risk_payload.get("risks", [])),
        continuity_policy=argument_risk_payload["continuity_policy"],
    )

    persona_labels = tuple(
        label
        for label in (
            "Moderator",
            "Advocate",
            "Skeptic",
            "Analyst",
            "Builder",
            "Auditor",
            "Synthesizer",
        )
    )
    return BoundedMemoryFoundation(
        seed_grounding=seed_grounding,
        round_summary=round_summary,
        argument_risk=argument_risk,
        persona_labels=persona_labels,
        cross_run_semantic_leakage_allowed=False,
        transcript_archive_separated=True,
    )


def update_bounded_memory_for_round(
    foundation: BoundedMemoryFoundation,
    *,
    round_summary: str,
    arguments: Iterable[str],
    objections: Iterable[str],
    risks: Iterable[str],
) -> BoundedMemoryFoundation:
    next_summary_history = _bounded_append(
        foundation.round_summary.summary_history,
        round_summary,
        SUMMARY_HISTORY_LIMIT,
    )
    next_round_summary = replace(
        foundation.round_summary,
        completed_rounds=foundation.round_summary.completed_rounds + 1,
        latest_round_summary=round_summary,
        summary_history=next_summary_history,
    )
    next_argument_risk = replace(
        foundation.argument_risk,
        arguments=_bounded_extend(
            foundation.argument_risk.arguments,
            arguments,
            CONTINUITY_ITEM_LIMIT,
        ),
        objections=_bounded_extend(
            foundation.argument_risk.objections,
            objections,
            CONTINUITY_ITEM_LIMIT,
        ),
        risks=_bounded_extend(
            foundation.argument_risk.risks,
            risks,
            CONTINUITY_ITEM_LIMIT,
        ),
    )
    return replace(
        foundation,
        round_summary=next_round_summary,
        argument_risk=next_argument_risk,
    )


def update_bounded_memory_for_finalization(
    foundation: BoundedMemoryFoundation,
    *,
    finalization_summary: str,
    arguments: Iterable[str],
    objections: Iterable[str],
    risks: Iterable[str],
) -> BoundedMemoryFoundation:
    next_summary_history = _bounded_append(
        foundation.round_summary.summary_history,
        finalization_summary,
        SUMMARY_HISTORY_LIMIT,
    )
    next_round_summary = replace(
        foundation.round_summary,
        latest_round_summary=finalization_summary,
        summary_history=next_summary_history,
    )
    next_argument_risk = replace(
        foundation.argument_risk,
        arguments=_bounded_extend(
            foundation.argument_risk.arguments,
            arguments,
            CONTINUITY_ITEM_LIMIT,
        ),
        objections=_bounded_extend(
            foundation.argument_risk.objections,
            objections,
            CONTINUITY_ITEM_LIMIT,
        ),
        risks=_bounded_extend(
            foundation.argument_risk.risks,
            risks,
            CONTINUITY_ITEM_LIMIT,
        ),
    )
    return replace(
        foundation,
        round_summary=next_round_summary,
        argument_risk=next_argument_risk,
    )


def _bounded_append(
    existing_items: tuple[str, ...],
    next_item: str,
    limit: int,
) -> tuple[str, ...]:
    cleaned_item = next_item.strip()
    if not cleaned_item:
        return existing_items
    return tuple((*existing_items, cleaned_item)[-limit:])


def _bounded_extend(
    existing_items: tuple[str, ...],
    new_items: Iterable[str],
    limit: int,
) -> tuple[str, ...]:
    merged_items = list(existing_items)
    for item in new_items:
        cleaned_item = item.strip()
        if cleaned_item and cleaned_item not in merged_items:
            merged_items.append(cleaned_item)
    return tuple(merged_items[-limit:])
