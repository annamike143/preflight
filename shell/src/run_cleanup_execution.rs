use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::run_cleanup_contract::RunCleanupMarker;
use crate::run_session_start::RunSessionStateSnapshot;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunCleanupFailureRecord {
    pub target_path: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunCleanupExecutionOutcome {
    pub cleanup_marker: RunCleanupMarker,
    pub started_at: u64,
    pub completed_at: u64,
    pub removed_targets: Vec<String>,
    pub failed_targets: Vec<RunCleanupFailureRecord>,
    pub preserved_report_artifact_path: Option<String>,
    pub preserved_report_artifact_retained: bool,
    pub detail_message: String,
}

pub fn execute_terminal_run_cleanup(
    run_session_state: &RunSessionStateSnapshot,
) -> RunCleanupExecutionOutcome {
    execute_terminal_run_cleanup_with(run_session_state, remove_cleanup_target)
}

fn execute_terminal_run_cleanup_with<Remover>(
    run_session_state: &RunSessionStateSnapshot,
    mut remover: Remover,
) -> RunCleanupExecutionOutcome
where
    Remover: FnMut(&str) -> Result<bool, String>,
{
    let started_at = current_epoch_ms();
    let cleanup_targets = cleanup_target_paths(run_session_state);
    let mut removed_targets = Vec::new();
    let mut failed_targets = Vec::new();

    for target_path in cleanup_targets {
        match remover(&target_path) {
            Ok(true) => removed_targets.push(target_path),
            Ok(false) => {}
            Err(reason) => failed_targets.push(RunCleanupFailureRecord {
                target_path,
                reason,
            }),
        }
    }

    let cleanup_marker = classify_cleanup_outcome(&removed_targets, &failed_targets);
    let preserved_report_artifact_path = run_session_state.durable_report_artifact_path.clone();
    let preserved_report_artifact_retained = preserved_report_artifact_path
        .as_deref()
        .map(Path::new)
        .is_some_and(Path::exists);
    let preserved_report_note = if let Some(path) = preserved_report_artifact_path.as_deref() {
        if preserved_report_artifact_retained {
            format!(
                "Preserved durable report artifact at {} remained outside cleanup scope.",
                path
            )
        } else {
            format!(
                "Preserved durable report artifact at {} remained outside cleanup scope even though it is not currently accessible.",
                path
            )
        }
    } else {
        "No preserved durable report artifact needed cleanup exemption.".to_string()
    };
    let detail_message = match cleanup_marker {
        RunCleanupMarker::CleanupSucceeded => {
            if removed_targets.is_empty() {
                format!(
                    "Best-effort terminal cleanup found no remaining transient cleanup targets. {}",
                    preserved_report_note
                )
            } else {
                format!(
                    "Best-effort terminal cleanup removed {} transient cleanup target(s). {}",
                    removed_targets.len(),
                    preserved_report_note
                )
            }
        }
        RunCleanupMarker::CleanupPartiallyFailed => format!(
            "Best-effort terminal cleanup removed {} transient cleanup target(s), but {} target(s) could not be removed: {}. {}",
            removed_targets.len(),
            failed_targets.len(),
            summarize_failed_targets(&failed_targets),
            preserved_report_note
        ),
        RunCleanupMarker::CleanupFailed => format!(
            "Best-effort terminal cleanup could not remove any transient cleanup targets: {}. {}",
            summarize_failed_targets(&failed_targets),
            preserved_report_note
        ),
        RunCleanupMarker::CleanupPending | RunCleanupMarker::CleanupRunning => {
            unreachable!("cleanup execution should only classify terminal cleanup outcomes")
        }
    };

    RunCleanupExecutionOutcome {
        cleanup_marker,
        started_at,
        completed_at: current_epoch_ms(),
        removed_targets,
        failed_targets,
        preserved_report_artifact_path,
        preserved_report_artifact_retained,
        detail_message,
    }
}

fn cleanup_target_paths(run_session_state: &RunSessionStateSnapshot) -> Vec<String> {
    let tracking_directory_path = run_session_state
        .token_duration_tracking_path
        .as_deref()
        .and_then(|path| Path::new(path).parent())
        .map(|path| path.to_string_lossy().to_string());

    let mut targets = Vec::new();
    let mut seen = BTreeSet::new();

    push_target(
        &mut targets,
        &mut seen,
        run_session_state.execution_seed_directory_path.clone(),
    );
    push_target(
        &mut targets,
        &mut seen,
        run_session_state.bounded_memory_directory_path.clone(),
    );
    push_target(
        &mut targets,
        &mut seen,
        run_session_state.transcript_archive_directory_path.clone(),
    );
    push_target(&mut targets, &mut seen, tracking_directory_path);
    push_target(
        &mut targets,
        &mut seen,
        run_session_state.runtime_persona_manifest_path.clone(),
    );
    push_target(
        &mut targets,
        &mut seen,
        run_session_state.moderator_basis_path.clone(),
    );
    push_target(
        &mut targets,
        &mut seen,
        run_session_state.seed_grounding_memory_path.clone(),
    );
    push_target(
        &mut targets,
        &mut seen,
        run_session_state.round_summary_memory_path.clone(),
    );
    push_target(
        &mut targets,
        &mut seen,
        run_session_state.argument_risk_memory_path.clone(),
    );
    push_target(
        &mut targets,
        &mut seen,
        run_session_state.workspace_root_path.clone(),
    );

    targets
}

fn push_target(targets: &mut Vec<String>, seen: &mut BTreeSet<String>, candidate: Option<String>) {
    let Some(candidate) = candidate else {
        return;
    };
    if candidate.is_empty() {
        return;
    }
    if seen.insert(candidate.clone()) {
        targets.push(candidate);
    }
}

fn remove_cleanup_target(target_path: &str) -> Result<bool, String> {
    let path = Path::new(target_path);
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.to_string()),
    };

    if metadata.is_dir() {
        fs::remove_dir_all(path).map_err(|error| error.to_string())?;
    } else {
        fs::remove_file(path).map_err(|error| error.to_string())?;
    }

    Ok(true)
}

fn classify_cleanup_outcome(
    removed_targets: &[String],
    failed_targets: &[RunCleanupFailureRecord],
) -> RunCleanupMarker {
    if failed_targets.is_empty() {
        RunCleanupMarker::CleanupSucceeded
    } else if removed_targets.is_empty() {
        RunCleanupMarker::CleanupFailed
    } else {
        RunCleanupMarker::CleanupPartiallyFailed
    }
}

fn summarize_failed_targets(failed_targets: &[RunCleanupFailureRecord]) -> String {
    failed_targets
        .iter()
        .map(|failure| format!("{} ({})", failure.target_path, failure.reason))
        .collect::<Vec<_>>()
        .join("; ")
}

fn current_epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;
    use crate::run_session_contract::RunSessionStateClass;
    use crate::run_session_start::{RunSessionStateSnapshot, RunSessionStateStage};

    fn cleanup_snapshot_with_workspace(temp_dir: &TempDir) -> RunSessionStateSnapshot {
        let workspace_root = temp_dir.path().join("workspace");
        let execution_seed_directory = workspace_root.join("execution-seed");
        let bounded_memory_directory = workspace_root.join("bounded-memory");
        let transcript_archive_directory = workspace_root.join("transcript-archive");
        let tracking_directory = workspace_root.join("run-local-tracking");
        let runtime_persona_manifest_path = workspace_root.join("runtime_personas.json");
        let moderator_basis_path = workspace_root.join("moderator_basis.json");
        let seed_grounding_memory_path = bounded_memory_directory.join("seed_grounding.json");
        let round_summary_memory_path = bounded_memory_directory.join("round_summary.json");
        let argument_risk_memory_path = bounded_memory_directory.join("argument_risk.json");
        let durable_report_path = temp_dir.path().join("durable-report.pdf");

        fs::create_dir_all(&execution_seed_directory).unwrap();
        fs::create_dir_all(&bounded_memory_directory).unwrap();
        fs::create_dir_all(&transcript_archive_directory).unwrap();
        fs::create_dir_all(&tracking_directory).unwrap();
        fs::write(execution_seed_directory.join("seed.txt"), "seed\n").unwrap();
        fs::write(&runtime_persona_manifest_path, "{}\n").unwrap();
        fs::write(&moderator_basis_path, "{}\n").unwrap();
        fs::write(&seed_grounding_memory_path, "{}\n").unwrap();
        fs::write(&round_summary_memory_path, "{}\n").unwrap();
        fs::write(&argument_risk_memory_path, "{}\n").unwrap();
        fs::write(
            transcript_archive_directory.join("speaker_turns.jsonl"),
            "{}\n",
        )
        .unwrap();
        fs::write(
            tracking_directory.join("token_duration_tracking.json"),
            "{}\n",
        )
        .unwrap();
        fs::write(&durable_report_path, b"%PDF-1.4\n%%EOF").unwrap();

        RunSessionStateSnapshot {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class: RunSessionStateClass::Finished,
            current_state_name: RunSessionStateClass::Finished.label(),
            workspace_root_path: Some(workspace_root.to_string_lossy().to_string()),
            execution_seed_directory_path: Some(execution_seed_directory.to_string_lossy().to_string()),
            transcript_archive_directory_path: Some(
                transcript_archive_directory.to_string_lossy().to_string(),
            ),
            runtime_persona_manifest_path: Some(
                runtime_persona_manifest_path.to_string_lossy().to_string(),
            ),
            moderator_basis_path: Some(moderator_basis_path.to_string_lossy().to_string()),
            bounded_memory_directory_path: Some(
                bounded_memory_directory.to_string_lossy().to_string(),
            ),
            seed_grounding_memory_path: Some(seed_grounding_memory_path.to_string_lossy().to_string()),
            round_summary_memory_path: Some(round_summary_memory_path.to_string_lossy().to_string()),
            argument_risk_memory_path: Some(argument_risk_memory_path.to_string_lossy().to_string()),
            token_duration_tracking_path: Some(
                tracking_directory
                    .join("token_duration_tracking.json")
                    .to_string_lossy()
                    .to_string(),
            ),
            durable_report_artifact_path: Some(durable_report_path.to_string_lossy().to_string()),
            detail_message: "Terminal run outcome ready for cleanup".to_string(),
            ..RunSessionStateSnapshot::not_started()
        }
    }

    #[test]
    fn terminal_cleanup_removes_transient_workspace_targets_but_preserves_durable_report_artifact() {
        let temp_dir = tempfile::tempdir().unwrap();
        let snapshot = cleanup_snapshot_with_workspace(&temp_dir);
        let workspace_root = snapshot.workspace_root_path.clone().unwrap();
        let durable_report_artifact_path = snapshot.durable_report_artifact_path.clone().unwrap();

        let outcome = execute_terminal_run_cleanup(&snapshot);

        assert_eq!(outcome.cleanup_marker, RunCleanupMarker::CleanupSucceeded);
        assert!(outcome.failed_targets.is_empty());
        assert!(outcome.removed_targets.iter().any(|target| target == &workspace_root));
        assert!(!Path::new(&workspace_root).exists());
        assert!(Path::new(&durable_report_artifact_path).exists());
        assert!(outcome.preserved_report_artifact_retained);
    }

    #[test]
    fn terminal_cleanup_classifies_partial_failure_without_rewriting_preserved_report_exemption() {
        let snapshot = RunSessionStateSnapshot {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class: RunSessionStateClass::Finished,
            current_state_name: RunSessionStateClass::Finished.label(),
            execution_seed_directory_path: Some("C:/temp/workspace/execution-seed".to_string()),
            bounded_memory_directory_path: Some("C:/temp/workspace/bounded-memory".to_string()),
            workspace_root_path: Some("C:/temp/workspace".to_string()),
            durable_report_artifact_path: Some("C:/durable/report.pdf".to_string()),
            ..RunSessionStateSnapshot::not_started()
        };

        let outcome = execute_terminal_run_cleanup_with(&snapshot, |target_path| {
            if target_path.ends_with("execution-seed") {
                Ok(true)
            } else if target_path.ends_with("bounded-memory") {
                Err("locked".to_string())
            } else {
                Ok(false)
            }
        });

        assert_eq!(outcome.cleanup_marker, RunCleanupMarker::CleanupPartiallyFailed);
        assert_eq!(outcome.removed_targets, vec!["C:/temp/workspace/execution-seed".to_string()]);
        assert_eq!(outcome.failed_targets.len(), 1);
        assert!(outcome.detail_message.contains("could not be removed"));
        assert!(outcome.detail_message.contains("outside cleanup scope"));
    }
}