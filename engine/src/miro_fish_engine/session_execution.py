from __future__ import annotations

import argparse
import json
import sys
import time
from dataclasses import asdict, dataclass
from pathlib import Path

from .moderated_execution import (
    ControlledCancellationError,
    execute_moderated_rounds,
)
from .report_data_assembly import (
    REPORT_GENERATION_STATUS_FAILED,
    REPORT_GENERATION_STATUS_SUCCEEDED,
    ReportDataAssemblyError,
    assemble_report_data_artifact,
)
from .report_pdf_generation import (
    GENERATED_REPORT_PDF_FILE_NAME,
    ReportPdfGenerationError,
    generate_report_pdf_artifact,
)
from .session_bootstrap import (
    RUN_START_LINEAGE_MANIFEST_FILE_NAME,
    bootstrap_session_from_workspace,
)
from .token_duration_tracking import (
    RUN_LOCAL_TRACKING_DIRECTORY_NAME,
    ExecutionPhaseState,
    initialize_execution_phase,
    initialize_token_duration_tracking,
    load_execution_phase,
    write_execution_phase,
)
from .transcript_archive import SPEAKER_TURNS_FILE_NAME

EXECUTION_CONTROL_FILE_NAME = "execution_control.json"
RUN_DURATION_CAP_MS = 7_200_000


@dataclass(frozen=True)
class EngineSessionExecutionResult:
    runtime_persona_manifest_path: str
    moderator_basis_path: str
    bounded_memory_directory_path: str
    seed_grounding_memory_path: str
    round_summary_memory_path: str
    argument_risk_memory_path: str
    transcript_archive_index_path: str
    transcript_turns_path: str
    token_duration_tracking_path: str
    moderator_persona_label: str
    participant_persona_labels: tuple[str, ...]
    perspective_coverage: tuple[str, ...]
    participant_persona_count: int
    exactly_one_moderator_present: bool
    stable_persona_labels: bool
    runtime_persona_foundation_materialized: bool
    bounded_memory_foundation_materialized: bool
    transcript_archive_separated: bool
    requested_round_count: int
    completed_round_count: int
    speaker_counts_per_round: tuple[int, ...]
    total_turn_count: int
    latest_round_summary: str | None
    tracked_total_input_token_units: int
    tracked_total_output_token_units: int
    tracked_total_token_units: int
    tracked_total_duration_ms: int
    average_turn_duration_ms: int
    moderated_rounds_materialized: bool
    normal_round_speaker_selection_enforced: bool
    speaker_attribution_materialized: bool
    bounded_memory_continuity_mutated: bool
    token_duration_tracking_materialized: bool
    completion_state: str
    runtime_cap_reason: str | None
    controlled_finalization_performed: bool
    finalization_started_at: int | None
    finalization_completed_at: int | None
    report_identifier: str | None
    report_generated_at: int | None
    report_data_artifact_path: str | None
    report_pdf_artifact_path: str | None
    report_generation_status: str
    report_generation_failure_reason: str | None
    report_generation_started_at: int | None
    report_generation_completed_at: int | None
    report_generation_failed_at: int | None
    detail_message: str


def execute_session_from_workspace(workspace_root: str | Path) -> EngineSessionExecutionResult:
    workspace_root_path = Path(workspace_root)
    bootstrap_result = bootstrap_session_from_workspace(workspace_root_path)
    manifest_payload = json.loads(
        (
            workspace_root_path
            / "execution-seed"
            / RUN_START_LINEAGE_MANIFEST_FILE_NAME
        ).read_text(encoding="utf-8")
    )
    persona_manifest = json.loads(
        Path(bootstrap_result.runtime_persona_manifest_path).read_text(encoding="utf-8")
    )
    moderator_basis = json.loads(
        Path(bootstrap_result.moderator_basis_path).read_text(encoding="utf-8")
    )
    seed_grounding = json.loads(
        Path(bootstrap_result.seed_grounding_memory_path).read_text(encoding="utf-8")
    )
    requested_round_count = int(
        (manifest_payload.get("effective_plan") or {}).get("effective_round_count")
        or (manifest_payload.get("requested_plan") or {}).get("requested_round_count")
        or 1
    )
    max_token_budget = int(
        (manifest_payload.get("effective_plan") or {}).get("max_token_budget")
        or (manifest_payload.get("requested_plan") or {}).get("max_token_budget")
        or 0
    )
    tracking_directory = workspace_root_path / RUN_LOCAL_TRACKING_DIRECTORY_NAME
    token_duration_tracking_path, initial_tracking_state = initialize_token_duration_tracking(
        tracking_directory
    )
    execution_phase_path, _ = initialize_execution_phase(
        tracking_directory,
        phase_changed_at=max(1, int(time.time_ns() / 1_000_000)),
        detail_message=(
            "Moderator-controlled execution is entering bounded normal round scheduling."
        ),
    )
    execution_result = execute_moderated_rounds(
        moderator_basis=moderator_basis,
        participant_personas=persona_manifest["participants"],
        seed_grounding=seed_grounding,
        bounded_memory_dir=Path(bootstrap_result.bounded_memory_directory_path),
        transcript_archive_dir=workspace_root_path / "transcript-archive",
        token_duration_tracking_path=token_duration_tracking_path,
        initial_token_tracking_state=initial_tracking_state,
        requested_round_count=requested_round_count,
        max_token_budget=max_token_budget,
        max_duration_cap_ms=RUN_DURATION_CAP_MS,
        execution_control_path=(
            workspace_root_path
            / RUN_LOCAL_TRACKING_DIRECTORY_NAME
            / EXECUTION_CONTROL_FILE_NAME
        ),
        execution_phase_path=execution_phase_path,
    )
    tracking_state = execution_result.token_tracking_state
    current_execution_phase = load_execution_phase(execution_phase_path)

    finalization_started_at = (
        execution_result.finalization_started_at or epoch_ms_now()
    )
    finalization_completed_at = execution_result.finalization_completed_at
    if finalization_completed_at is None:
        finalization_completed_at = finalization_started_at

    write_execution_phase(
        execution_phase_path,
        ExecutionPhaseState(
            phase_version=current_execution_phase.phase_version,
            current_phase="finalizing",
            phase_changed_at=finalization_started_at,
            cap_reason=execution_result.runtime_cap_reason,
            detail_message=build_report_generation_start_detail_message(
                completion_state=execution_result.completion_state,
                completed_round_count=execution_result.completed_round_count,
                requested_round_count=requested_round_count,
            ),
            finalization_started_at=finalization_started_at,
            finalization_completed_at=finalization_completed_at,
            report_generation_started_at=None,
            report_generation_completed_at=None,
            report_generation_failed_at=None,
            report_generation_failure_reason=None,
        ),
    )

    report_generation_started_at = epoch_ms_now()
    write_execution_phase(
        execution_phase_path,
        ExecutionPhaseState(
            phase_version=current_execution_phase.phase_version,
            current_phase="finalizing",
            phase_changed_at=report_generation_started_at,
            cap_reason=execution_result.runtime_cap_reason,
            detail_message=build_report_generation_running_detail_message(
                completion_state=execution_result.completion_state,
                completed_round_count=execution_result.completed_round_count,
                requested_round_count=requested_round_count,
            ),
            finalization_started_at=finalization_started_at,
            finalization_completed_at=finalization_completed_at,
            report_generation_started_at=report_generation_started_at,
            report_generation_completed_at=None,
            report_generation_failed_at=None,
            report_generation_failure_reason=None,
        ),
    )

    report_data_artifact_path: str | None = None
    report_pdf_artifact_path: str | None = None
    report_identifier: str | None = None
    report_generated_at: int | None = None
    report_generation_status = REPORT_GENERATION_STATUS_SUCCEEDED
    report_generation_failure_reason: str | None = None
    report_generation_completed_at: int | None = None
    report_generation_failed_at: int | None = None

    try:
        report_artifact = assemble_report_data_artifact(
            workspace_root=workspace_root_path,
            bounded_memory_dir=Path(bootstrap_result.bounded_memory_directory_path),
            transcript_archive_dir=workspace_root_path / "transcript-archive",
            token_duration_tracking_path=token_duration_tracking_path,
            execution_phase_path=execution_phase_path,
            moderator_persona_label=bootstrap_result.moderator_persona_label,
            completion_state=execution_result.completion_state,
            requested_round_count=requested_round_count,
            completed_round_count=execution_result.completed_round_count,
            requested_token_budget=max_token_budget,
            runtime_cap_reason=execution_result.runtime_cap_reason,
            generated_at=report_generation_started_at,
        )
        report_data_artifact_path = report_artifact.report_data_path
        report_pdf_artifact = generate_report_pdf_artifact(
            report_data_path=Path(report_artifact.report_data_path),
            output_pdf_path=(
                workspace_root_path
                / "report-generation"
                / GENERATED_REPORT_PDF_FILE_NAME
            ),
        )
        report_pdf_artifact_path = report_pdf_artifact.report_pdf_path
        report_identifier = report_pdf_artifact.report_identifier
        report_generated_at = report_pdf_artifact.generated_at
        report_generation_completed_at = epoch_ms_now()
        detail_message = build_success_detail_message(
            completion_state=execution_result.completion_state,
            report_data_artifact_path=report_data_artifact_path,
            report_pdf_artifact_path=report_pdf_artifact_path,
        )
        terminal_phase = execution_result.completion_state
    except (ReportDataAssemblyError, ReportPdfGenerationError) as error:
        report_generation_status = REPORT_GENERATION_STATUS_FAILED
        report_generation_failure_reason = error.failure_reason
        report_generation_failed_at = epoch_ms_now()
        detail_message = (
            "Moderator-controlled execution finished debate material, but "
            "report generation failed from the actual run artifacts. "
            f"{error.detail_message}"
        )
        terminal_phase = "failed_terminal"

    terminal_phase_changed_at = (
        report_generation_completed_at
        or report_generation_failed_at
        or report_generation_started_at
    )
    write_execution_phase(
        execution_phase_path,
        ExecutionPhaseState(
            phase_version=current_execution_phase.phase_version,
            current_phase=terminal_phase,
            phase_changed_at=terminal_phase_changed_at,
            cap_reason=execution_result.runtime_cap_reason,
            detail_message=detail_message,
            finalization_started_at=finalization_started_at,
            finalization_completed_at=finalization_completed_at,
            report_generation_started_at=report_generation_started_at,
            report_generation_completed_at=report_generation_completed_at,
            report_generation_failed_at=report_generation_failed_at,
            report_generation_failure_reason=report_generation_failure_reason,
        ),
    )

    return EngineSessionExecutionResult(
        runtime_persona_manifest_path=bootstrap_result.runtime_persona_manifest_path,
        moderator_basis_path=bootstrap_result.moderator_basis_path,
        bounded_memory_directory_path=bootstrap_result.bounded_memory_directory_path,
        seed_grounding_memory_path=bootstrap_result.seed_grounding_memory_path,
        round_summary_memory_path=bootstrap_result.round_summary_memory_path,
        argument_risk_memory_path=bootstrap_result.argument_risk_memory_path,
        transcript_archive_index_path=bootstrap_result.transcript_archive_index_path,
        transcript_turns_path=str(
            workspace_root_path / "transcript-archive" / SPEAKER_TURNS_FILE_NAME
        ),
        token_duration_tracking_path=str(token_duration_tracking_path),
        moderator_persona_label=bootstrap_result.moderator_persona_label,
        participant_persona_labels=bootstrap_result.participant_persona_labels,
        perspective_coverage=bootstrap_result.perspective_coverage,
        participant_persona_count=bootstrap_result.participant_persona_count,
        exactly_one_moderator_present=bootstrap_result.exactly_one_moderator_present,
        stable_persona_labels=bootstrap_result.stable_persona_labels,
        runtime_persona_foundation_materialized=(
            bootstrap_result.runtime_persona_foundation_materialized
        ),
        bounded_memory_foundation_materialized=(
            bootstrap_result.bounded_memory_foundation_materialized
        ),
        transcript_archive_separated=bootstrap_result.transcript_archive_separated,
        requested_round_count=requested_round_count,
        completed_round_count=execution_result.completed_round_count,
        speaker_counts_per_round=execution_result.speaker_counts_per_round,
        total_turn_count=execution_result.total_turn_count,
        latest_round_summary=execution_result.latest_round_summary,
        tracked_total_input_token_units=tracking_state.total_input_token_units,
        tracked_total_output_token_units=tracking_state.total_output_token_units,
        tracked_total_token_units=tracking_state.total_token_units,
        tracked_total_duration_ms=tracking_state.total_duration_ms,
        average_turn_duration_ms=tracking_state.average_turn_duration_ms,
        moderated_rounds_materialized=(
            execution_result.completion_state == "completed"
            and execution_result.completed_round_count == requested_round_count
        ),
        normal_round_speaker_selection_enforced=(
            execution_result.normal_round_speaker_selection_enforced
        ),
        speaker_attribution_materialized=execution_result.speaker_attribution_materialized,
        bounded_memory_continuity_mutated=(
            execution_result.bounded_memory_continuity_mutated
        ),
        token_duration_tracking_materialized=(
            execution_result.token_duration_tracking_materialized
        ),
        completion_state=terminal_phase,
        runtime_cap_reason=execution_result.runtime_cap_reason,
        controlled_finalization_performed=True,
        finalization_started_at=finalization_started_at,
        finalization_completed_at=finalization_completed_at,
        report_identifier=report_identifier,
        report_generated_at=report_generated_at,
        report_data_artifact_path=report_data_artifact_path,
        report_pdf_artifact_path=report_pdf_artifact_path,
        report_generation_status=report_generation_status,
        report_generation_failure_reason=report_generation_failure_reason,
        report_generation_started_at=report_generation_started_at,
        report_generation_completed_at=report_generation_completed_at,
        report_generation_failed_at=report_generation_failed_at,
        detail_message=detail_message,
    )


def build_report_generation_start_detail_message(
    *,
    completion_state: str,
    completed_round_count: int,
    requested_round_count: int,
) -> str:
    if completion_state == "completed_budget_capped":
        return (
            "Constrained finalization preserved bounded continuity and is now "
            "handing off report data assembly after "
            f"{completed_round_count} of {requested_round_count} completed rounds."
        )
    return (
        "Moderator-controlled execution completed all requested bounded rounds "
        "and is now entering finalization-owned report data assembly."
    )


def build_report_generation_running_detail_message(
    *,
    completion_state: str,
    completed_round_count: int,
    requested_round_count: int,
) -> str:
    if completion_state == "completed_budget_capped":
        return (
            "Report data assembly is synthesizing required sections from actual "
            "constrained-completion artifacts after "
            f"{completed_round_count} of {requested_round_count} completed rounds."
        )
    return (
        "Report data assembly is synthesizing required sections from actual "
        "successful-completion artifacts."
    )


def build_success_detail_message(
    *,
    completion_state: str,
    report_data_artifact_path: str,
    report_pdf_artifact_path: str,
) -> str:
    if completion_state == "completed_budget_capped":
        return (
            "Moderator-controlled execution resolved as CompletedBudgetCapped and "
            "generated the required report data sections and PDF from actual "
            "constrained-completion artifacts. "
            f"Transient report data now exists at {report_data_artifact_path}, "
            f"and the transient PDF now exists at {report_pdf_artifact_path}."
        )
    return (
        "Moderator-controlled execution completed bounded rounds and generated "
        "the required report data sections and PDF from actual run artifacts. "
        f"Transient report data now exists at {report_data_artifact_path}, "
        f"and the transient PDF now exists at {report_pdf_artifact_path}."
    )


def epoch_ms_now() -> int:
    return max(1, int(time.time_ns() / 1_000_000))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--workspace-root", required=True)
    args = parser.parse_args()
    try:
        result = execute_session_from_workspace(args.workspace_root)
    except ControlledCancellationError as error:
        prefix = "shutdown_canceled" if error.shutdown_requested else "canceled"
        print(f"{prefix}: {error}", file=sys.stderr)
        raise SystemExit(3) from error

    print(json.dumps(asdict(result)))


if __name__ == "__main__":
    main()