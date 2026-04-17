use std::fs;

use serde::Deserialize;

use crate::run_cleanup_contract::RunCleanupMarker;
use crate::run_event_history::{run_event_history_overview, run_event_history_overview_from_shell};
use crate::run_lifecycle_state::{project_run_lifecycle_state, CanonicalRunLifecycleState};
use crate::run_session_start::{
    load_execution_phase_from_run_session, run_session_state_from_shell,
    terminal_run_outcome_name_from_snapshot, RunCapReason, RunLifecycleOverrideState,
    RunSessionStateSnapshot,
};

const RUN_DURATION_CAP_MS: usize = 7_200_000;
const CAP_WARNING_THRESHOLD_PERCENT: usize = 80;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunStatusProjectionSnapshot {
    pub completed_round_count: Option<usize>,
    pub tracked_total_token_units: Option<usize>,
    pub tracked_total_duration_ms: Option<usize>,
    pub latest_visible_sequence_number: Option<u64>,
    pub ordered_event_history_materialized: bool,
    pub latest_status_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TranscriptArchiveIndexPayload {
    rounds_completed: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TokenDurationTrackingPayload {
    total_token_units: usize,
    total_duration_ms: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct RunStartLineagePayload {
    requested_plan: Option<RunStartLineagePlanPayload>,
    effective_plan: Option<RunStartLineagePlanPayload>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct RunStartLineagePlanPayload {
    max_token_budget: usize,
}

pub fn project_run_status_projection(
    run_session_state: &RunSessionStateSnapshot,
) -> RunStatusProjectionSnapshot {
    let lifecycle_state = project_run_lifecycle_state(run_session_state);
    let completed_round_count = load_transcript_archive_index(run_session_state)
        .map(|payload| payload.rounds_completed)
        .or(run_session_state.completed_round_count);
    let token_duration_tracking = load_token_duration_tracking(run_session_state);
    let tracked_total_token_units = token_duration_tracking
        .as_ref()
        .map(|payload| payload.total_token_units)
        .or(run_session_state.tracked_total_token_units);
    let tracked_total_duration_ms = token_duration_tracking
        .as_ref()
        .map(|payload| payload.total_duration_ms)
        .or(run_session_state.tracked_total_duration_ms);
    let max_token_budget = load_max_token_budget(run_session_state);
    let runtime_cap_reason = run_session_state.runtime_cap_reason.or_else(|| {
        load_execution_phase_from_run_session(run_session_state).and_then(|phase| phase.cap_reason)
    });
    let current_shell_snapshot = run_session_state_from_shell();
    let event_history_overview = if current_shell_snapshot.run_id == run_session_state.run_id
        && current_shell_snapshot.session_id == run_session_state.session_id
        && current_shell_snapshot.current_state_name == run_session_state.current_state_name
    {
        run_event_history_overview_from_shell()
    } else {
        run_event_history_overview(run_session_state)
    };

    let latest_status_note = if matches!(
        run_session_state.cleanup_marker,
        Some(RunCleanupMarker::CleanupPending | RunCleanupMarker::CleanupRunning)
    ) {
        format!(
            "Shell status projection confirms that the run already resolved as {} and shell-owned terminal cleanup is active. {}",
            terminal_run_outcome_name_from_snapshot(run_session_state),
            run_session_state.detail_message
        )
    } else if matches!(
        run_session_state.cleanup_marker,
        Some(RunCleanupMarker::CleanupSucceeded)
    ) {
        format!(
            "Shell status projection confirms that the run resolved as {} and shell-owned terminal cleanup completed while preserved report artifacts remained outside cleanup scope. {}",
            terminal_run_outcome_name_from_snapshot(run_session_state),
            run_session_state.detail_message
        )
    } else if matches!(
        run_session_state.cleanup_marker,
        Some(RunCleanupMarker::CleanupPartiallyFailed | RunCleanupMarker::CleanupFailed)
    ) {
        format!(
            "Shell status projection confirms that the run resolved as {}, but terminal cleanup did not finish cleanly and the cleanup result remains distinct from the run outcome. {}",
            terminal_run_outcome_name_from_snapshot(run_session_state),
            run_session_state.detail_message
        )
    } else if matches!(
        run_session_state.lifecycle_override,
        Some(RunLifecycleOverrideState::PausedRateLimit)
    ) {
        format!(
            "Shell status projection is exposing a paused rate-limit state before bounded retry attempt {} of {}. {}",
            run_session_state.retry_attempt_count,
            run_session_state.max_retry_attempts,
            run_session_state.detail_message
        )
    } else if matches!(
        run_session_state.lifecycle_override,
        Some(RunLifecycleOverrideState::PausedNetwork)
    ) {
        format!(
            "Shell status projection is exposing a paused network state before bounded retry attempt {} of {}. {}",
            run_session_state.retry_attempt_count,
            run_session_state.max_retry_attempts,
            run_session_state.detail_message
        )
    } else if lifecycle_state.current_state == CanonicalRunLifecycleState::Finalizing {
        format!(
            "Shell status projection confirms that {} was reached and constrained finalization is now active. {}",
            runtime_cap_reason
                .map(RunCapReason::label)
                .unwrap_or("a governed runtime cap"),
            run_session_state.detail_message
        )
    } else if lifecycle_state.current_state == CanonicalRunLifecycleState::CompletedBudgetCapped {
        format!(
            "Shell status projection confirms that the run resolved as CompletedBudgetCapped after the {} stopped further normal rounds. {}",
            runtime_cap_reason
                .map(RunCapReason::label)
                .unwrap_or("governed runtime cap"),
            run_session_state.detail_message
        )
    } else if run_session_state.cancellation_request_accepted
        && !matches!(run_session_state.lifecycle_override, Some(RunLifecycleOverrideState::Canceled))
    {
        format!(
            "Shell status projection confirms that interruption control was accepted and healthy forward progression has stopped while the engine acknowledges the request. {}",
            run_session_state.detail_message
        )
    } else if let Some(warning_note) = build_cap_warning_note(
        tracked_total_token_units,
        max_token_budget,
        tracked_total_duration_ms,
    ) {
        format!("{} {}", warning_note, run_session_state.detail_message)
    } else if run_session_state.run_id.is_none() || run_session_state.session_id.is_none() {
        run_session_state.detail_message.clone()
    } else if let Some(gap_reason) = event_history_overview.gap_reason.as_deref() {
        format!(
            "Shell status projection is grounded in current run/session artifacts, but ordered event history is incomplete: {} {}",
            gap_reason,
            run_session_state.detail_message
        )
    } else if let Some(latest_sequence_number) = event_history_overview.latest_available_sequence_number {
        format!(
            "Shell status projection is grounded in workspace-local transcript and tracking artifacts. Ordered event history is available through sequence {}. {}",
            latest_sequence_number,
            run_session_state.detail_message
        )
    } else {
        format!(
            "Shell status projection is grounded in current run/session truth, but the current run has not emitted ordered history yet. {}",
            run_session_state.detail_message
        )
    };

    RunStatusProjectionSnapshot {
        completed_round_count,
        tracked_total_token_units,
        tracked_total_duration_ms,
        latest_visible_sequence_number: event_history_overview.latest_available_sequence_number,
        ordered_event_history_materialized: event_history_overview.event_history_materialized,
        latest_status_note,
    }
}

fn load_transcript_archive_index(
    run_session_state: &RunSessionStateSnapshot,
) -> Option<TranscriptArchiveIndexPayload> {
    let path = run_session_state.transcript_archive_index_path.as_deref()?;
    let payload = fs::read_to_string(path).ok()?;
    serde_json::from_str(&payload).ok()
}

fn load_token_duration_tracking(
    run_session_state: &RunSessionStateSnapshot,
) -> Option<TokenDurationTrackingPayload> {
    let path = run_session_state.token_duration_tracking_path.as_deref()?;
    let payload = fs::read_to_string(path).ok()?;
    serde_json::from_str(&payload).ok()
}

fn load_max_token_budget(run_session_state: &RunSessionStateSnapshot) -> Option<usize> {
    let path = run_session_state.lineage_manifest_path.as_deref()?;
    let payload = fs::read_to_string(path).ok()?;
    let lineage = serde_json::from_str::<RunStartLineagePayload>(&payload).ok()?;
    lineage
        .effective_plan
        .map(|plan| plan.max_token_budget)
        .or_else(|| lineage.requested_plan.map(|plan| plan.max_token_budget))
}

fn build_cap_warning_note(
    tracked_total_token_units: Option<usize>,
    max_token_budget: Option<usize>,
    tracked_total_duration_ms: Option<usize>,
) -> Option<String> {
    let mut warnings = Vec::new();

    if let (Some(tracked_total_token_units), Some(max_token_budget)) = (
        tracked_total_token_units,
        max_token_budget,
    ) {
        if max_token_budget > 0 {
            let token_progress_percent = (tracked_total_token_units.saturating_mul(100)) / max_token_budget;
            if (CAP_WARNING_THRESHOLD_PERCENT..100).contains(&token_progress_percent) {
                warnings.push(format!(
                    "Runtime token budget warning: tracked usage is at {}% of the governed cap.",
                    token_progress_percent
                ));
            }
        }
    }

    if let Some(tracked_total_duration_ms) = tracked_total_duration_ms {
        let duration_progress_percent = (tracked_total_duration_ms.saturating_mul(100)) / RUN_DURATION_CAP_MS;
        if (CAP_WARNING_THRESHOLD_PERCENT..100).contains(&duration_progress_percent) {
            warnings.push(format!(
                "Runtime duration warning: tracked elapsed time is at {}% of the hard two-hour cap.",
                duration_progress_percent
            ));
        }
    }

    if warnings.is_empty() {
        None
    } else {
        Some(warnings.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    use tempfile::TempDir;

    use crate::run_cleanup_contract::RunCleanupMarker;
    use crate::run_session_contract::RunSessionStateClass;
    use crate::run_session_start::{RunSessionStateSnapshot, RunSessionStateStage};

    fn finished_snapshot_with_projection_files(temp_dir: &TempDir) -> RunSessionStateSnapshot {
        let workspace_root = temp_dir.path().join("workspace");
        let transcript_archive_directory = workspace_root.join("transcript-archive");
        let tracking_directory = workspace_root.join("run-local-tracking");
        fs::create_dir_all(&transcript_archive_directory).unwrap();
        fs::create_dir_all(&tracking_directory).unwrap();
        fs::create_dir_all(workspace_root.join("execution-seed")).unwrap();

        let transcript_archive_index_path = transcript_archive_directory.join("archive_index.json");
        let transcript_turns_path = transcript_archive_directory.join("speaker_turns.jsonl");
        let token_duration_tracking_path = tracking_directory.join("token_duration_tracking.json");
        let runtime_persona_manifest_path = workspace_root.join("runtime_personas.json");
        let moderator_basis_path = workspace_root.join("moderator_basis.json");
        let staged_seed_document_path = workspace_root.join("execution-seed").join("seed.txt");
        let lineage_manifest_path = workspace_root.join("execution-seed").join("run_start_lineage.json");

        fs::write(&runtime_persona_manifest_path, "{}\n").unwrap();
        fs::write(&moderator_basis_path, "{}\n").unwrap();
        fs::write(&staged_seed_document_path, "seed\n").unwrap();
        fs::write(&lineage_manifest_path, "{}\n").unwrap();
        fs::write(
            &transcript_archive_index_path,
            serde_json::to_string_pretty(&serde_json::json!({
                "rounds_completed": 1,
                "total_turns": 1,
            }))
            .unwrap(),
        )
        .unwrap();
        let mut transcript_file = fs::File::create(&transcript_turns_path).unwrap();
        writeln!(
            transcript_file,
            "{}",
            serde_json::to_string(&serde_json::json!({
                "round_number": 1,
                "turn_index": 1,
                "speaker_label": "Advocate",
                "speaker_perspective": "favorable",
                "turn_text": "Single turn.",
                "total_token_units": 10,
                "duration_ms": 5,
                "occurred_at": 1000,
            }))
            .unwrap()
        )
        .unwrap();
        fs::write(
            &token_duration_tracking_path,
            serde_json::to_string_pretty(&serde_json::json!({
                "total_rounds_completed": 1,
                "total_turns_tracked": 1,
                "total_input_token_units": 5,
                "total_output_token_units": 5,
                "total_token_units": 10,
                "total_duration_ms": 5,
                "average_turn_duration_ms": 5,
                "turn_records": [
                    {
                        "round_number": 1,
                        "turn_index": 1,
                        "speaker_label": "Advocate",
                        "input_token_units": 5,
                        "output_token_units": 5,
                        "total_token_units": 10,
                        "duration_ms": 5,
                        "occurred_at": 1000,
                    }
                ]
            }))
            .unwrap(),
        )
        .unwrap();

        RunSessionStateSnapshot {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class: RunSessionStateClass::Finished,
            current_state_name: RunSessionStateClass::Finished.label(),
            run_id: Some("run-1".to_string()),
            session_id: Some("session-1".to_string()),
            workspace_reference: Some("workspace-run-1".to_string()),
            workspace_root_path: Some(workspace_root.to_string_lossy().to_string()),
            execution_seed_directory_path: Some(
                workspace_root.join("execution-seed").to_string_lossy().to_string(),
            ),
            transcript_archive_directory_path: Some(
                transcript_archive_directory.to_string_lossy().to_string(),
            ),
            staged_seed_document_path: Some(staged_seed_document_path.to_string_lossy().to_string()),
            lineage_manifest_path: Some(lineage_manifest_path.to_string_lossy().to_string()),
            current_seed_fingerprint_sha256: Some("seed-hash".to_string()),
            runtime_persona_manifest_path: Some(
                runtime_persona_manifest_path.to_string_lossy().to_string(),
            ),
            moderator_basis_path: Some(moderator_basis_path.to_string_lossy().to_string()),
            bounded_memory_directory_path: None,
            seed_grounding_memory_path: None,
            round_summary_memory_path: None,
            argument_risk_memory_path: None,
            transcript_archive_index_path: Some(transcript_archive_index_path.to_string_lossy().to_string()),
            transcript_turns_path: Some(transcript_turns_path.to_string_lossy().to_string()),
            token_duration_tracking_path: Some(token_duration_tracking_path.to_string_lossy().to_string()),
            moderator_persona_label: Some("Moderator".to_string()),
            participant_persona_labels: vec!["Advocate".to_string()],
            perspective_coverage: vec!["favorable".to_string()],
            participant_persona_count: Some(1),
            requested_round_count: Some(1),
            completed_round_count: Some(1),
            speaker_counts_per_round: vec![1],
            total_turn_count: Some(1),
            latest_round_summary: Some("Summary".to_string()),
            tracked_total_input_token_units: Some(5),
            tracked_total_output_token_units: Some(5),
            tracked_total_token_units: Some(10),
            tracked_total_duration_ms: Some(5),
            average_turn_duration_ms: Some(5),
            runtime_persona_foundation_materialized: true,
            bounded_memory_foundation_materialized: true,
            transcript_archive_separated: true,
            exactly_one_moderator_present: true,
            stable_persona_labels: true,
            latest_start_lineage_captured: true,
            workspace_allocated: true,
            execution_seed_staged: true,
            pending_execution_ready: false,
            moderated_rounds_materialized: true,
            normal_round_speaker_selection_enforced: true,
            speaker_attribution_materialized: true,
            bounded_memory_continuity_mutated: true,
            token_duration_tracking_materialized: true,
            start_blocking_reasons: Vec::new(),
            technical_failure_reason: None,
            detail_message: "Execution completed".to_string(),
            ..RunSessionStateSnapshot::not_started()
        }
    }

    #[test]
    fn status_projection_reads_artifact_backed_counts_and_latest_sequence() {
        let temp_dir = tempfile::tempdir().unwrap();
        let snapshot = finished_snapshot_with_projection_files(&temp_dir);

        let projection = project_run_status_projection(&snapshot);

        assert_eq!(projection.completed_round_count, Some(1));
        assert_eq!(projection.tracked_total_token_units, Some(10));
        assert_eq!(projection.tracked_total_duration_ms, Some(5));
        assert_eq!(projection.latest_visible_sequence_number, Some(12));
        assert!(projection.ordered_event_history_materialized);
        assert!(projection.latest_status_note.contains("sequence 12"));
    }

    #[test]
    fn status_projection_prioritizes_cleanup_failure_truth_over_history_gap_notes() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut snapshot = finished_snapshot_with_projection_files(&temp_dir);
        snapshot.cleanup_marker = Some(RunCleanupMarker::CleanupFailed);
        snapshot.detail_message =
            "Run resolved as Completed, but terminal cleanup did not finish cleanly because a transient target remained locked."
                .to_string();

        let projection = project_run_status_projection(&snapshot);

        assert!(projection.latest_status_note.contains("run resolved as Completed"));
        assert!(projection.latest_status_note.contains("cleanup did not finish cleanly"));
    }
}