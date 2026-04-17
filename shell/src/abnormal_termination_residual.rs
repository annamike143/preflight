use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::run_session_start::RunSessionStateSnapshot;

const RUN_LOCAL_TRACKING_DIRECTORY_NAME: &str = "run-local-tracking";
pub const ABNORMAL_TERMINATION_RESIDUAL_FILE_NAME: &str = "abnormal_termination_residual.json";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AbnormalTerminationResidualRecord {
    pub run_id: Option<String>,
    pub session_id: Option<String>,
    pub workspace_reference: Option<String>,
    pub workspace_root_path: String,
    pub durable_report_artifact_path: Option<String>,
    pub marker_created_at: u64,
    pub shutdown_requested: bool,
    pub cleanup_handoff_required: bool,
    pub detail_message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidualWorkspaceCandidate {
    pub workspace_root_path: String,
    pub marker_path: Option<String>,
    pub marker_record: Option<AbnormalTerminationResidualRecord>,
}

pub fn abnormal_residual_marker_path(workspace_root_path: &str) -> String {
    PathBuf::from(workspace_root_path)
        .join(RUN_LOCAL_TRACKING_DIRECTORY_NAME)
        .join(ABNORMAL_TERMINATION_RESIDUAL_FILE_NAME)
        .to_string_lossy()
        .to_string()
}

pub fn update_abnormal_residual_marker_from_snapshot(
    run_session_state: &RunSessionStateSnapshot,
    shutdown_requested: bool,
    cleanup_handoff_required: bool,
    detail_message: String,
) -> Result<Option<String>, String> {
    let Some(workspace_root_path) = run_session_state.workspace_root_path.as_deref() else {
        return Ok(None);
    };

    let marker_path = abnormal_residual_marker_path(workspace_root_path);
    let marker_directory = Path::new(&marker_path)
        .parent()
        .ok_or_else(|| "residual marker directory could not be derived".to_string())?;

    fs::create_dir_all(marker_directory)
        .map_err(|error| format!("residual marker directory could not be created: {error}"))?;

    let marker = AbnormalTerminationResidualRecord {
        run_id: run_session_state.run_id.clone(),
        session_id: run_session_state.session_id.clone(),
        workspace_reference: run_session_state.workspace_reference.clone(),
        workspace_root_path: workspace_root_path.to_string(),
        durable_report_artifact_path: run_session_state.durable_report_artifact_path.clone(),
        marker_created_at: current_epoch_ms(),
        shutdown_requested,
        cleanup_handoff_required,
        detail_message,
    };

    fs::write(
        &marker_path,
        serde_json::to_string_pretty(&marker)
            .map_err(|error| format!("residual marker could not be encoded: {error}"))?,
    )
    .map_err(|error| format!("residual marker could not be written: {error}"))?;

    Ok(Some(marker_path))
}

pub fn list_abnormal_residual_candidates_under(base_root: &Path) -> Vec<ResidualWorkspaceCandidate> {
    let Ok(entries) = fs::read_dir(base_root) else {
        return Vec::new();
    };

    let mut candidates = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .map(|workspace_root| {
            let workspace_root_path = workspace_root.to_string_lossy().to_string();
            let marker_path = abnormal_residual_marker_path(&workspace_root_path);
            let marker_record = fs::read_to_string(&marker_path)
                .ok()
                .and_then(|payload| serde_json::from_str::<AbnormalTerminationResidualRecord>(&payload).ok());

            ResidualWorkspaceCandidate {
                workspace_root_path,
                marker_path: Path::new(&marker_path)
                    .exists()
                    .then_some(marker_path),
                marker_record,
            }
        })
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| left.workspace_root_path.cmp(&right.workspace_root_path));
    candidates
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
    use crate::run_session_start::RunSessionStateSnapshot;

    fn snapshot_with_workspace(root: &Path) -> RunSessionStateSnapshot {
        let workspace_root = root.join("run-123");
        fs::create_dir_all(&workspace_root).expect("workspace root should exist");

        let mut snapshot = RunSessionStateSnapshot::not_started();
        snapshot.run_id = Some("run-123".to_string());
        snapshot.session_id = Some("session-123".to_string());
        snapshot.workspace_reference = Some("workspace::run-123".to_string());
        snapshot.workspace_root_path = Some(workspace_root.to_string_lossy().to_string());
        snapshot.durable_report_artifact_path = Some(
            root.join("preserved-report.pdf")
                .to_string_lossy()
                .to_string(),
        );
        snapshot
    }

    #[test]
    fn update_abnormal_residual_marker_writes_workspace_local_manifest() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let snapshot = snapshot_with_workspace(temp_dir.path());

        let marker_path = update_abnormal_residual_marker_from_snapshot(
            &snapshot,
            true,
            true,
            "Startup janitor handoff is armed for this workspace.".to_string(),
        )
        .expect("marker write should succeed")
        .expect("workspace snapshot should produce a marker path");

        let marker_payload = fs::read_to_string(&marker_path).expect("marker should be readable");
        let marker = serde_json::from_str::<AbnormalTerminationResidualRecord>(&marker_payload)
            .expect("marker should decode");

        assert_eq!(marker.run_id.as_deref(), Some("run-123"));
        assert_eq!(marker.session_id.as_deref(), Some("session-123"));
        assert!(marker.shutdown_requested);
        assert!(marker.cleanup_handoff_required);
        assert!(marker.detail_message.contains("handoff"));
    }

    #[test]
    fn list_abnormal_residual_candidates_includes_orphaned_workspace_without_marker() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let orphan_workspace = temp_dir.path().join("run-orphan");
        fs::create_dir_all(&orphan_workspace).expect("orphan workspace should exist");

        let mut candidates = list_abnormal_residual_candidates_under(temp_dir.path());

        assert_eq!(candidates.len(), 1);
        let candidate = candidates.pop().expect("candidate should exist");
        assert_eq!(
            candidate.workspace_root_path,
            orphan_workspace.to_string_lossy().to_string()
        );
        assert!(candidate.marker_path.is_none());
        assert!(candidate.marker_record.is_none());
    }
}