use serde::{Deserialize, Serialize};

use crate::startup_janitor_execution::startup_janitor_execution_state;

pub const STARTUP_JANITOR_CONTRACT_NAME: &str = "StartupJanitorContract";
pub const STARTUP_JANITOR_CONTRACT_COMMAND_NAME: &str = "StartupJanitorContractState";
pub const SUPPORTED_STARTUP_JANITOR_MARKERS: [StartupJanitorMarker; 4] = [
    StartupJanitorMarker::PendingNextLaunch,
    StartupJanitorMarker::JanitorRunning,
    StartupJanitorMarker::JanitorPurgeSucceeded,
    StartupJanitorMarker::JanitorPurgeFailed,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StartupJanitorContractStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StartupJanitorMarker {
    PendingNextLaunch,
    JanitorRunning,
    JanitorPurgeSucceeded,
    JanitorPurgeFailed,
}

impl StartupJanitorMarker {
    pub fn name(self) -> &'static str {
        match self {
            Self::PendingNextLaunch => "PendingNextLaunch",
            Self::JanitorRunning => "JanitorRunning",
            Self::JanitorPurgeSucceeded => "JanitorPurgeSucceeded",
            Self::JanitorPurgeFailed => "JanitorPurgeFailed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StartupJanitorContractSnapshot {
    pub stage: StartupJanitorContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub supported_markers: [StartupJanitorMarker; 4],
    pub shell_authority_required: bool,
    pub abnormal_termination_detection_required: bool,
    pub pending_next_launch_distinct: bool,
    pub janitor_running_distinct: bool,
    pub janitor_purge_succeeded_distinct: bool,
    pub janitor_purge_failed_distinct: bool,
    pub janitor_outcome_distinct_from_run_outcome: bool,
    pub janitor_requires_shell_owned_workspace_classification: bool,
    pub janitor_purge_implies_crash_resume: bool,
    pub real_startup_janitor_execution_materialized: bool,
    pub real_abnormal_termination_detection_materialized: bool,
    pub execution_checked_this_launch: bool,
    pub latest_marker: Option<StartupJanitorMarker>,
    pub latest_marker_name: Option<&'static str>,
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

impl StartupJanitorContractSnapshot {
    pub fn summary(self) -> String {
        let supported_markers = self
            .supported_markers
            .map(StartupJanitorMarker::name)
            .join(", ");

        format!(
            "Startup janitor contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; supported markers are {}; shell authority required is {}; abnormal termination detection required is {}; pending next launch distinct is {}; janitor running distinct is {}; janitor purge succeeded distinct is {}; janitor purge failed distinct is {}; janitor outcome distinct from run outcome is {}; janitor requires shell-owned workspace classification is {}; janitor purge implies crash resume is {}; real startup janitor execution materialized is {}; real abnormal termination detection materialized is {}; execution checked this launch is {}; latest marker is {:?}; janitor running now is {}; abnormal residual detected this launch is {}; residual workspace count is {}; residual marker count is {}; purged workspace count is {}; failed workspace count is {}; preserved report artifact count is {}; preserved report artifact retained count is {}; started at is {:?}; completed at is {:?}; timed out is {}; latest status note is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            supported_markers,
            self.shell_authority_required,
            self.abnormal_termination_detection_required,
            self.pending_next_launch_distinct,
            self.janitor_running_distinct,
            self.janitor_purge_succeeded_distinct,
            self.janitor_purge_failed_distinct,
            self.janitor_outcome_distinct_from_run_outcome,
            self.janitor_requires_shell_owned_workspace_classification,
            self.janitor_purge_implies_crash_resume,
            self.real_startup_janitor_execution_materialized,
            self.real_abnormal_termination_detection_materialized,
            self.execution_checked_this_launch,
            self.latest_marker_name,
            self.janitor_running_now,
            self.abnormal_residual_detected_this_launch,
            self.residual_workspace_count,
            self.residual_marker_count,
            self.purged_workspace_count,
            self.failed_workspace_count,
            self.preserved_report_artifact_count,
            self.preserved_report_artifact_retained_count,
            self.started_at,
            self.completed_at,
            self.timed_out,
            self.latest_status_note,
        )
    }
}

pub fn startup_janitor_contract_state() -> StartupJanitorContractSnapshot {
    let janitor_execution = startup_janitor_execution_state();

    StartupJanitorContractSnapshot {
        stage: StartupJanitorContractStage::FoundationMaterialized,
        contract_name: STARTUP_JANITOR_CONTRACT_NAME,
        shell_command_name: STARTUP_JANITOR_CONTRACT_COMMAND_NAME,
        supported_markers: SUPPORTED_STARTUP_JANITOR_MARKERS,
        shell_authority_required: true,
        abnormal_termination_detection_required: true,
        pending_next_launch_distinct: true,
        janitor_running_distinct: true,
        janitor_purge_succeeded_distinct: true,
        janitor_purge_failed_distinct: true,
        janitor_outcome_distinct_from_run_outcome: true,
        janitor_requires_shell_owned_workspace_classification: true,
        janitor_purge_implies_crash_resume: false,
        real_startup_janitor_execution_materialized: true,
        real_abnormal_termination_detection_materialized: true,
        execution_checked_this_launch: janitor_execution.execution_checked_this_launch,
        latest_marker: janitor_execution.latest_marker,
        latest_marker_name: janitor_execution.latest_marker.map(StartupJanitorMarker::name),
        janitor_running_now: janitor_execution.janitor_running_now,
        abnormal_residual_detected_this_launch: janitor_execution
            .abnormal_residual_detected_this_launch,
        residual_workspace_count: janitor_execution.residual_workspace_count,
        residual_marker_count: janitor_execution.residual_marker_count,
        purged_workspace_count: janitor_execution.purged_workspace_count,
        failed_workspace_count: janitor_execution.failed_workspace_count,
        preserved_report_artifact_count: janitor_execution.preserved_report_artifact_count,
        preserved_report_artifact_retained_count: janitor_execution
            .preserved_report_artifact_retained_count,
        started_at: janitor_execution.started_at,
        completed_at: janitor_execution.completed_at,
        timed_out: janitor_execution.timed_out,
        latest_status_note: janitor_execution.latest_status_note,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_janitor_contract_freezes_next_launch_cleanup_truth_without_claiming_resume() {
        let snapshot = startup_janitor_contract_state();

        assert_eq!(snapshot.contract_name, STARTUP_JANITOR_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, STARTUP_JANITOR_CONTRACT_COMMAND_NAME);
        assert_eq!(snapshot.supported_markers.len(), 4);
        assert!(snapshot.shell_authority_required);
        assert!(snapshot.abnormal_termination_detection_required);
        assert!(snapshot.pending_next_launch_distinct);
        assert!(snapshot.janitor_running_distinct);
        assert!(snapshot.janitor_purge_succeeded_distinct);
        assert!(snapshot.janitor_purge_failed_distinct);
        assert!(snapshot.janitor_outcome_distinct_from_run_outcome);
        assert!(snapshot.janitor_requires_shell_owned_workspace_classification);
        assert!(!snapshot.janitor_purge_implies_crash_resume);
        assert!(snapshot.real_startup_janitor_execution_materialized);
        assert!(snapshot.real_abnormal_termination_detection_materialized);
        assert!(!snapshot.execution_checked_this_launch);
        assert!(snapshot.latest_marker.is_none());
    }
}