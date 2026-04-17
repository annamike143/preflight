use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::abnormal_termination_residual::{
    list_abnormal_residual_candidates_under, ResidualWorkspaceCandidate,
};
use crate::run_workspace_allocation::default_run_workspace_root;
use crate::startup_janitor_contract::StartupJanitorMarker;

const STARTUP_JANITOR_TIMEOUT_MS: u64 = 2_000;

static STARTUP_JANITOR_STATE: OnceLock<Mutex<StartupJanitorExecutionSnapshot>> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq)]
struct StartupJanitorFailureRecord {
    workspace_root_path: String,
    reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupJanitorExecutionSnapshot {
    pub execution_checked_this_launch: bool,
    pub latest_marker: Option<StartupJanitorMarker>,
    pub janitor_running_now: bool,
    pub abnormal_residual_detected_this_launch: bool,
    pub residual_workspace_count: usize,
    pub residual_marker_count: usize,
    pub purged_workspace_count: usize,
    pub failed_workspace_count: usize,
    pub preserved_report_artifact_count: usize,
    pub preserved_report_artifact_retained_count: usize,
    pub started_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub timed_out: bool,
    pub latest_status_note: String,
}

impl Default for StartupJanitorExecutionSnapshot {
    fn default() -> Self {
        Self {
            execution_checked_this_launch: false,
            latest_marker: None,
            janitor_running_now: false,
            abnormal_residual_detected_this_launch: false,
            residual_workspace_count: 0,
            residual_marker_count: 0,
            purged_workspace_count: 0,
            failed_workspace_count: 0,
            preserved_report_artifact_count: 0,
            preserved_report_artifact_retained_count: 0,
            started_at: None,
            completed_at: None,
            timed_out: false,
            latest_status_note:
                "Startup janitor has not executed yet in the current shell launch.".to_string(),
        }
    }
}

struct StartupJanitorOutcome {
    purged_workspace_count: usize,
    failed_workspace_count: usize,
    timed_out: bool,
    latest_status_note: String,
}

pub fn startup_janitor_execution_state() -> StartupJanitorExecutionSnapshot {
    startup_janitor_state_store()
        .lock()
        .map(|state| state.clone())
        .unwrap_or_default()
}

pub fn ensure_startup_janitor_executed() -> StartupJanitorExecutionSnapshot {
    if let Ok(state) = startup_janitor_state_store().lock() {
        if state.execution_checked_this_launch {
            return state.clone();
        }
    }

    let snapshot = execute_startup_janitor_under(
        &default_run_workspace_root(),
        STARTUP_JANITOR_TIMEOUT_MS,
    );

    if let Ok(mut state) = startup_janitor_state_store().lock() {
        *state = snapshot.clone();
    }

    snapshot
}

pub fn execute_startup_janitor_under(
    base_root: &Path,
    timeout_ms: u64,
) -> StartupJanitorExecutionSnapshot {
    let candidates = list_abnormal_residual_candidates_under(base_root);
    let started_at = current_epoch_ms();
    let residual_marker_count = candidates
        .iter()
        .filter(|candidate| candidate.marker_path.is_some())
        .count();
    let preserved_report_artifacts = candidates
        .iter()
        .filter_map(|candidate| {
            candidate
                .marker_record
                .as_ref()
                .and_then(|record| record.durable_report_artifact_path.clone())
        })
        .collect::<BTreeSet<_>>();
    let preserved_report_artifact_count = preserved_report_artifacts.len();
    let preserved_report_artifact_retained_count = preserved_report_artifacts
        .iter()
        .filter(|path| Path::new(path).exists())
        .count();

    if candidates.is_empty() {
        return StartupJanitorExecutionSnapshot {
            execution_checked_this_launch: true,
            latest_marker: None,
            janitor_running_now: false,
            abnormal_residual_detected_this_launch: false,
            residual_workspace_count: 0,
            residual_marker_count: 0,
            purged_workspace_count: 0,
            failed_workspace_count: 0,
            preserved_report_artifact_count,
            preserved_report_artifact_retained_count,
            started_at: Some(started_at),
            completed_at: Some(current_epoch_ms()),
            timed_out: false,
            latest_status_note:
                "Startup janitor found no abandoned run workspaces, so normal readiness may continue without purge work.".to_string(),
        };
    }

    let outcome = execute_startup_janitor_candidates_with(
        &candidates,
        timeout_ms,
        remove_residual_workspace,
    );

    StartupJanitorExecutionSnapshot {
        execution_checked_this_launch: true,
        latest_marker: Some(if outcome.failed_workspace_count == 0 {
            StartupJanitorMarker::JanitorPurgeSucceeded
        } else {
            StartupJanitorMarker::JanitorPurgeFailed
        }),
        janitor_running_now: false,
        abnormal_residual_detected_this_launch: true,
        residual_workspace_count: candidates.len(),
        residual_marker_count,
        purged_workspace_count: outcome.purged_workspace_count,
        failed_workspace_count: outcome.failed_workspace_count,
        preserved_report_artifact_count,
        preserved_report_artifact_retained_count,
        started_at: Some(started_at),
        completed_at: Some(current_epoch_ms()),
        timed_out: outcome.timed_out,
        latest_status_note: outcome.latest_status_note,
    }
}

fn startup_janitor_state_store() -> &'static Mutex<StartupJanitorExecutionSnapshot> {
    STARTUP_JANITOR_STATE.get_or_init(|| Mutex::new(StartupJanitorExecutionSnapshot::default()))
}

fn execute_startup_janitor_candidates_with<Remover>(
    candidates: &[ResidualWorkspaceCandidate],
    timeout_ms: u64,
    mut remover: Remover,
) -> StartupJanitorOutcome
where
    Remover: FnMut(&str) -> Result<bool, String>,
{
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    let mut purged_workspace_count = 0;
    let mut failure_records = Vec::new();

    for (index, candidate) in candidates.iter().enumerate() {
        if Instant::now() >= deadline {
            failure_records.extend(candidates[index..].iter().map(|pending_candidate| {
                StartupJanitorFailureRecord {
                    workspace_root_path: pending_candidate.workspace_root_path.clone(),
                    reason: "safe startup-janitor timeout was reached before the abandoned workspace could be purged".to_string(),
                }
            }));
            break;
        }

        match remover(&candidate.workspace_root_path) {
            Ok(true) => purged_workspace_count += 1,
            Ok(false) => {}
            Err(reason) => failure_records.push(StartupJanitorFailureRecord {
                workspace_root_path: candidate.workspace_root_path.clone(),
                reason,
            }),
        }
    }

    let timed_out = failure_records.iter().any(|failure| {
        failure
            .reason
            .contains("safe startup-janitor timeout")
    });
    let failure_summary = failure_records
        .iter()
        .map(|failure| format!("{} ({})", failure.workspace_root_path, failure.reason))
        .collect::<Vec<_>>()
        .join("; ");

    let latest_status_note = if failure_records.is_empty() {
        format!(
            "Startup janitor removed {} abandoned workspace(s) before normal readiness continued. Any preserved durable report artifacts referenced by residual markers remained outside janitor scope.",
            purged_workspace_count
        )
    } else if purged_workspace_count > 0 {
        format!(
            "Startup janitor removed {} abandoned workspace(s), but {} workspace(s) still need later best-effort purge attention: {}. No abandoned workspace is reused as active execution state.",
            purged_workspace_count,
            failure_records.len(),
            failure_summary
        )
    } else {
        format!(
            "Startup janitor could not purge the {} abandoned workspace(s) detected at launch: {}. No abandoned workspace is reused as active execution state.",
            failure_records.len(),
            failure_summary
        )
    };

    StartupJanitorOutcome {
        purged_workspace_count,
        failed_workspace_count: failure_records.len(),
        timed_out,
        latest_status_note,
    }
}

fn remove_residual_workspace(workspace_root_path: &str) -> Result<bool, String> {
    match fs::remove_dir_all(workspace_root_path) {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.to_string()),
    }
}

fn current_epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abnormal_termination_residual::update_abnormal_residual_marker_from_snapshot;
    use crate::run_session_start::RunSessionStateSnapshot;

    fn residual_candidate(name: &str) -> ResidualWorkspaceCandidate {
        ResidualWorkspaceCandidate {
            workspace_root_path: format!("C:/temp/{name}"),
            marker_path: Some(format!(
                "C:/temp/{name}/run-local-tracking/abnormal_termination_residual.json"
            )),
            marker_record: None,
        }
    }

    fn residual_snapshot(root: &Path) -> (RunSessionStateSnapshot, String) {
        let workspace_root = root.join("run-janitor");
        let workspace_tracking = workspace_root.join("run-local-tracking");
        let preserved_report = root.join("preserved-report.pdf");

        fs::create_dir_all(&workspace_tracking).expect("workspace tracking should exist");
        fs::write(workspace_root.join("leftover.txt"), "leftover\n")
            .expect("leftover workspace file should exist");
        fs::write(&preserved_report, b"%PDF-1.4\n%%EOF").expect("report should exist");

        let mut snapshot = RunSessionStateSnapshot::not_started();
        snapshot.run_id = Some("run-janitor".to_string());
        snapshot.session_id = Some("session-janitor".to_string());
        snapshot.workspace_reference = Some("workspace::run-janitor".to_string());
        snapshot.workspace_root_path = Some(workspace_root.to_string_lossy().to_string());
        snapshot.durable_report_artifact_path = Some(preserved_report.to_string_lossy().to_string());

        (snapshot, preserved_report.to_string_lossy().to_string())
    }

    #[test]
    fn execute_startup_janitor_under_leaves_clean_launch_state_when_no_residuals_exist() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");

        let snapshot = execute_startup_janitor_under(temp_dir.path(), 1_000);

        assert!(snapshot.execution_checked_this_launch);
        assert!(snapshot.latest_marker.is_none());
        assert_eq!(snapshot.residual_workspace_count, 0);
        assert_eq!(snapshot.failed_workspace_count, 0);
    }

    #[test]
    fn execute_startup_janitor_under_purges_residual_workspace_and_preserves_report() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let (snapshot, preserved_report_path) = residual_snapshot(temp_dir.path());
        let workspace_root_path = snapshot
            .workspace_root_path
            .clone()
            .expect("workspace root should exist on snapshot");

        update_abnormal_residual_marker_from_snapshot(
            &snapshot,
            true,
            true,
            "Residual workspace is pending next-launch janitor purge.".to_string(),
        )
        .expect("marker write should succeed");

        let janitor_snapshot = execute_startup_janitor_under(temp_dir.path(), 1_000);

        assert_eq!(
            janitor_snapshot.latest_marker,
            Some(StartupJanitorMarker::JanitorPurgeSucceeded)
        );
        assert_eq!(janitor_snapshot.residual_workspace_count, 1);
        assert_eq!(janitor_snapshot.purged_workspace_count, 1);
        assert_eq!(janitor_snapshot.failed_workspace_count, 0);
        assert!(!Path::new(&workspace_root_path).exists());
        assert!(Path::new(&preserved_report_path).exists());
    }

    #[test]
    fn execute_startup_janitor_candidates_with_reports_partial_failure_without_resume_semantics() {
        let candidates = vec![residual_candidate("run-one"), residual_candidate("run-two")];

        let outcome = execute_startup_janitor_candidates_with(
            &candidates,
            1_000,
            |workspace_root_path| {
                if workspace_root_path.ends_with("run-one") {
                    Ok(true)
                } else {
                    Err("workspace lock persisted after abnormal shutdown".to_string())
                }
            },
        );

        assert_eq!(outcome.purged_workspace_count, 1);
        assert_eq!(outcome.failed_workspace_count, 1);
        assert!(!outcome.timed_out);
        assert!(outcome.latest_status_note.contains("still need later best-effort purge attention"));
        assert!(outcome
            .latest_status_note
            .contains("workspace lock persisted after abnormal shutdown"));
        assert!(outcome
            .latest_status_note
            .contains("No abandoned workspace is reused as active execution state"));
    }

    #[test]
    fn execute_startup_janitor_candidates_with_marks_safe_timeout_for_unprocessed_residuals() {
        let candidates = vec![residual_candidate("run-one"), residual_candidate("run-two")];

        let outcome = execute_startup_janitor_candidates_with(&candidates, 0, |_| Ok(true));

        assert_eq!(outcome.purged_workspace_count, 0);
        assert_eq!(outcome.failed_workspace_count, 2);
        assert!(outcome.timed_out);
        assert!(outcome
            .latest_status_note
            .contains("safe startup-janitor timeout was reached"));
        assert!(outcome
            .latest_status_note
            .contains("No abandoned workspace is reused as active execution state"));
    }
}