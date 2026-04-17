from __future__ import annotations

import json
from pathlib import Path

from miro_fish_engine.moderated_execution import execute_moderated_rounds
from miro_fish_engine.session_bootstrap import bootstrap_session_from_workspace
from miro_fish_engine.token_duration_tracking import (
    initialize_execution_phase,
    initialize_token_duration_tracking,
    load_execution_phase,
)


def test_moderated_execution_enforces_3_to_5_speaker_rounds(seeded_workspace: Path) -> None:
    bootstrap_result = bootstrap_session_from_workspace(seeded_workspace)
    persona_manifest = json.loads(
        Path(bootstrap_result.runtime_persona_manifest_path).read_text(encoding="utf-8")
    )
    moderator_basis = json.loads(
        Path(bootstrap_result.moderator_basis_path).read_text(encoding="utf-8")
    )
    seed_grounding = json.loads(
        Path(bootstrap_result.seed_grounding_memory_path).read_text(encoding="utf-8")
    )
    token_duration_tracking_path, initial_tracking_state = initialize_token_duration_tracking(
        seeded_workspace / "run-local-tracking"
    )

    result = execute_moderated_rounds(
        moderator_basis=moderator_basis,
        participant_personas=persona_manifest["participants"],
        seed_grounding=seed_grounding,
        bounded_memory_dir=Path(bootstrap_result.bounded_memory_directory_path),
        transcript_archive_dir=seeded_workspace / "transcript-archive",
        token_duration_tracking_path=token_duration_tracking_path,
        initial_token_tracking_state=initial_tracking_state,
        requested_round_count=4,
        max_token_budget=12000,
        max_duration_cap_ms=7_200_000,
    )

    speaker_turns = [
        json.loads(line)
        for line in (
            seeded_workspace / "transcript-archive" / "speaker_turns.jsonl"
        ).read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]

    assert result.completed_round_count == 4
    assert result.speaker_counts_per_round == (3, 4, 5, 3)
    assert result.total_turn_count == 15
    assert result.normal_round_speaker_selection_enforced is True
    assert result.speaker_attribution_materialized is True
    assert result.bounded_memory_continuity_mutated is True
    assert result.token_duration_tracking_materialized is True
    assert len(speaker_turns) == 15
    assert all(turn["speaker_label"] for turn in speaker_turns)
    assert all(turn["speaker_perspective"] for turn in speaker_turns)
    assert all(turn["occurred_at"] > 0 for turn in speaker_turns)


def test_moderated_execution_stops_new_normal_rounds_after_runtime_cap(
    seeded_workspace: Path,
) -> None:
    bootstrap_result = bootstrap_session_from_workspace(seeded_workspace)
    persona_manifest = json.loads(
        Path(bootstrap_result.runtime_persona_manifest_path).read_text(encoding="utf-8")
    )
    moderator_basis = json.loads(
        Path(bootstrap_result.moderator_basis_path).read_text(encoding="utf-8")
    )
    seed_grounding = json.loads(
        Path(bootstrap_result.seed_grounding_memory_path).read_text(encoding="utf-8")
    )
    tracking_directory = seeded_workspace / "run-local-tracking"
    token_duration_tracking_path, initial_tracking_state = initialize_token_duration_tracking(
        tracking_directory
    )
    execution_phase_path, _ = initialize_execution_phase(
        tracking_directory,
        phase_changed_at=1,
        detail_message=(
            "Moderator-controlled execution is entering bounded normal round "
            "scheduling."
        ),
    )

    result = execute_moderated_rounds(
        moderator_basis=moderator_basis,
        participant_personas=persona_manifest["participants"],
        seed_grounding=seed_grounding,
        bounded_memory_dir=Path(bootstrap_result.bounded_memory_directory_path),
        transcript_archive_dir=seeded_workspace / "transcript-archive",
        token_duration_tracking_path=token_duration_tracking_path,
        initial_token_tracking_state=initial_tracking_state,
        requested_round_count=4,
        max_token_budget=300,
        max_duration_cap_ms=7_200_000,
        execution_phase_path=execution_phase_path,
    )
    execution_phase = load_execution_phase(execution_phase_path)

    assert result.completion_state == "completed_budget_capped"
    assert result.runtime_cap_reason == "token_budget"
    assert result.controlled_finalization_performed is True
    assert result.completed_round_count < 4
    assert len(result.speaker_counts_per_round) == result.completed_round_count
    assert execution_phase.current_phase == "finalizing"
    assert execution_phase.cap_reason == "token_budget"