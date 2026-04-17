from __future__ import annotations

import json
from pathlib import Path

import pytest

from miro_fish_engine.moderated_execution import execute_moderated_rounds
from miro_fish_engine.report_data_assembly import (
    FAILURE_REASON_INSUFFICIENT_COMPLETED_MATERIAL,
    ReportDataAssemblyError,
    assemble_report_data_artifact,
)
from miro_fish_engine.session_bootstrap import bootstrap_session_from_workspace
from miro_fish_engine.token_duration_tracking import (
    initialize_execution_phase,
    initialize_token_duration_tracking,
)


def test_report_data_assembly_materializes_required_sections(
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

    execution_result = execute_moderated_rounds(
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
        execution_phase_path=execution_phase_path,
    )

    artifact = assemble_report_data_artifact(
        workspace_root=seeded_workspace,
        bounded_memory_dir=Path(bootstrap_result.bounded_memory_directory_path),
        transcript_archive_dir=seeded_workspace / "transcript-archive",
        token_duration_tracking_path=token_duration_tracking_path,
        execution_phase_path=execution_phase_path,
        moderator_persona_label=bootstrap_result.moderator_persona_label,
        completion_state=execution_result.completion_state,
        requested_round_count=4,
        completed_round_count=execution_result.completed_round_count,
        requested_token_budget=12000,
        runtime_cap_reason=execution_result.runtime_cap_reason,
        generated_at=1234,
    )
    payload = json.loads(Path(artifact.report_data_path).read_text(encoding="utf-8"))

    assert artifact.report_generation_status == "succeeded"
    assert artifact.required_section_count == 7
    assert payload["report_sections"]["executive_summary"]["summary_heading"] == "Executive Summary"
    assert payload["report_sections"]["ranked_risks"]["ranked_risks"]
    assert payload["report_sections"]["strongest_pro_arguments"]["arguments"]
    assert payload["report_sections"]["strongest_anti_arguments"]["arguments"]
    assert payload["report_sections"]["cost_and_token_summary"]["token_usage_total"] > 0
    assert (
        len(
            payload["report_sections"]["transcript_appendix"][
                "ordered_transcript_entries"
            ]
        )
        == 15
    )


def test_report_data_assembly_rejects_missing_transcript_material(
    seeded_workspace: Path,
) -> None:
    bootstrap_session_from_workspace(seeded_workspace)
    tracking_directory = seeded_workspace / "run-local-tracking"
    token_duration_tracking_path, _ = initialize_token_duration_tracking(tracking_directory)
    execution_phase_path, _ = initialize_execution_phase(
        tracking_directory,
        phase_changed_at=1,
        detail_message=(
            "Moderator-controlled execution is entering bounded normal round "
            "scheduling."
        ),
    )

    with pytest.raises(ReportDataAssemblyError) as error_info:
        assemble_report_data_artifact(
            workspace_root=seeded_workspace,
            bounded_memory_dir=seeded_workspace / "bounded-memory",
            transcript_archive_dir=seeded_workspace / "transcript-archive",
            token_duration_tracking_path=token_duration_tracking_path,
            execution_phase_path=execution_phase_path,
            moderator_persona_label="Moderator",
            completion_state="completed",
            requested_round_count=4,
            completed_round_count=0,
            requested_token_budget=12000,
            runtime_cap_reason=None,
            generated_at=1234,
        )

    assert error_info.value.failure_reason == FAILURE_REASON_INSUFFICIENT_COMPLETED_MATERIAL