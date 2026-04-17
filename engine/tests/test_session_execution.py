import json
from pathlib import Path

import pytest
from pypdf import PdfReader

from miro_fish_engine import session_execution
from miro_fish_engine.moderated_execution import ControlledCancellationError
from miro_fish_engine.session_execution import execute_session_from_workspace
from miro_fish_engine.token_duration_tracking import (
    load_execution_phase,
    load_token_duration_tracking,
)


def test_session_execution_materializes_rounds_tracking_and_continuity(
    seeded_workspace: Path,
) -> None:
    result = execute_session_from_workspace(seeded_workspace)
    execution_phase = load_execution_phase(
        Path(result.token_duration_tracking_path).with_name("execution_phase.json")
    )
    report_data_payload = json.loads(
        Path(result.report_data_artifact_path or "").read_text(encoding="utf-8")
    )

    assert Path(result.transcript_turns_path).exists()
    assert Path(result.token_duration_tracking_path).exists()
    assert result.report_identifier is not None
    assert result.report_generated_at is not None
    assert Path(result.report_data_artifact_path or "").exists()
    assert Path(result.report_pdf_artifact_path or "").exists()
    assert result.requested_round_count == 4
    assert result.completed_round_count == 4
    assert result.speaker_counts_per_round == (3, 4, 5, 3)
    assert result.total_turn_count == 15
    assert result.latest_round_summary is not None
    assert result.tracked_total_input_token_units > 0
    assert result.tracked_total_output_token_units > 0
    assert result.tracked_total_token_units > 0
    assert result.tracked_total_duration_ms > 0
    assert result.average_turn_duration_ms > 0
    assert result.moderated_rounds_materialized is True
    assert result.normal_round_speaker_selection_enforced is True
    assert result.speaker_attribution_materialized is True
    assert result.bounded_memory_continuity_mutated is True
    assert result.token_duration_tracking_materialized is True
    assert result.completion_state == "completed"
    assert result.runtime_cap_reason is None
    assert result.controlled_finalization_performed is True
    assert result.finalization_started_at is not None
    assert result.finalization_completed_at is not None
    assert result.report_generation_status == "succeeded"
    assert result.report_generation_failure_reason is None
    assert result.report_generation_started_at is not None
    assert result.report_generation_completed_at is not None
    assert result.report_generation_failed_at is None
    assert execution_phase.current_phase == "completed"
    assert execution_phase.report_generation_started_at is not None
    assert execution_phase.report_generation_completed_at is not None
    assert (
        report_data_payload["report_sections"]["executive_summary"]["summary_heading"]
        == "Executive Summary"
    )
    pdf_text = " ".join(
        page.extract_text() or ""
        for page in PdfReader(result.report_pdf_artifact_path or "").pages
    )
    assert "Miro Fish Report" in pdf_text
    assert "Executive Summary" in pdf_text
    assert result.report_identifier in pdf_text
    assert (
        0
        <= report_data_payload["report_sections"]["market_viability_score"][
            "score_value"
        ]
        <= 100
    )
    assert (
        len(
            report_data_payload["report_sections"]["transcript_appendix"][
                "ordered_transcript_entries"
            ]
        )
        == 15
    )

    tracking_state = load_token_duration_tracking(Path(result.token_duration_tracking_path))
    assert all(record.occurred_at > 0 for record in tracking_state.turn_records)


def test_session_execution_honors_graceful_cancellation_control_file(
    seeded_workspace: Path,
) -> None:
    control_directory = seeded_workspace / "run-local-tracking"
    control_directory.mkdir(parents=True, exist_ok=True)
    (control_directory / "execution_control.json").write_text(
        json.dumps(
            {
                "cancellation_requested": True,
                "shutdown_requested": False,
            }
        ),
        encoding="utf-8",
    )

    with pytest.raises(ControlledCancellationError):
        execute_session_from_workspace(seeded_workspace)


def test_session_execution_resolves_completed_budget_capped_on_runtime_token_budget(
    seeded_workspace: Path,
) -> None:
    manifest_path = seeded_workspace / "execution-seed" / "run_start_lineage.json"
    manifest_payload = json.loads(manifest_path.read_text(encoding="utf-8"))
    manifest_payload["requested_plan"]["max_token_budget"] = 300
    manifest_payload["effective_plan"]["max_token_budget"] = 300
    manifest_path.write_text(json.dumps(manifest_payload, indent=2), encoding="utf-8")

    result = execute_session_from_workspace(seeded_workspace)
    execution_phase = load_execution_phase(
        Path(result.token_duration_tracking_path).with_name("execution_phase.json")
    )

    assert result.completion_state == "completed_budget_capped"
    assert result.runtime_cap_reason == "token_budget"
    assert result.controlled_finalization_performed is True
    assert result.completed_round_count < result.requested_round_count
    assert result.moderated_rounds_materialized is False
    assert result.finalization_started_at is not None
    assert result.finalization_completed_at is not None
    assert result.report_generation_status == "succeeded"
    assert result.report_identifier is not None
    assert result.report_pdf_artifact_path is not None
    assert result.report_data_artifact_path is not None
    assert execution_phase.current_phase == "completed_budget_capped"
    assert execution_phase.cap_reason == "token_budget"
    assert execution_phase.report_generation_started_at is not None
    assert execution_phase.report_generation_completed_at is not None


def test_session_execution_resolves_completed_budget_capped_on_hard_duration_cap(
    seeded_workspace: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    monkeypatch.setattr(session_execution, "RUN_DURATION_CAP_MS", 1)

    result = execute_session_from_workspace(seeded_workspace)
    execution_phase = load_execution_phase(
        Path(result.token_duration_tracking_path).with_name("execution_phase.json")
    )

    assert result.completion_state == "completed_budget_capped"
    assert result.runtime_cap_reason == "duration_cap"
    assert result.controlled_finalization_performed is True
    assert result.completed_round_count < result.requested_round_count
    assert result.report_generation_status == "succeeded"
    assert result.report_identifier is not None
    assert result.report_pdf_artifact_path is not None
    assert result.report_data_artifact_path is not None
    assert execution_phase.current_phase == "completed_budget_capped"
    assert execution_phase.cap_reason == "duration_cap"
    assert execution_phase.report_generation_started_at is not None
    assert execution_phase.report_generation_completed_at is not None