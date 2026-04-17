use serde::{Deserialize, Serialize};

pub const RUN_CLEANUP_CONTRACT_NAME: &str = "RunCleanupContract";
pub const RUN_CLEANUP_CONTRACT_COMMAND_NAME: &str = "RunCleanupContractState";
pub const SUPPORTED_RUN_CLEANUP_MARKERS: [RunCleanupMarker; 5] = [
    RunCleanupMarker::CleanupPending,
    RunCleanupMarker::CleanupRunning,
    RunCleanupMarker::CleanupSucceeded,
    RunCleanupMarker::CleanupPartiallyFailed,
    RunCleanupMarker::CleanupFailed,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunCleanupContractStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum RunCleanupMarker {
    CleanupPending,
    CleanupRunning,
    CleanupSucceeded,
    CleanupPartiallyFailed,
    CleanupFailed,
}

impl RunCleanupMarker {
    pub fn name(self) -> &'static str {
        match self {
            Self::CleanupPending => "CleanupPending",
            Self::CleanupRunning => "CleanupRunning",
            Self::CleanupSucceeded => "CleanupSucceeded",
            Self::CleanupPartiallyFailed => "CleanupPartiallyFailed",
            Self::CleanupFailed => "CleanupFailed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunCleanupContractSnapshot {
    pub stage: RunCleanupContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub supported_markers: [RunCleanupMarker; 5],
    pub shell_authority_required: bool,
    pub cleanup_pending_distinct: bool,
    pub cleanup_running_distinct: bool,
    pub cleanup_succeeded_distinct: bool,
    pub cleanup_partially_failed_distinct: bool,
    pub cleanup_failed_distinct: bool,
    pub cleanup_attempt_distinct_from_cleanup_outcome: bool,
    pub cleanup_outcome_distinct_from_run_outcome: bool,
    pub cleanup_partial_failure_requires_visible_reason: bool,
    pub cleanup_outcome_implies_report_deletion: bool,
    pub real_cleanup_execution_materialized: bool,
    pub real_cleanup_result_emission_materialized: bool,
}

impl RunCleanupContractSnapshot {
    pub fn summary(self) -> String {
        let supported_markers = self.supported_markers.map(RunCleanupMarker::name).join(", ");

        format!(
            "Run cleanup contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; supported markers are {}; shell authority required is {}; cleanup pending distinct is {}; cleanup running distinct is {}; cleanup succeeded distinct is {}; cleanup partially failed distinct is {}; cleanup failed distinct is {}; cleanup attempt distinct from cleanup outcome is {}; cleanup outcome distinct from run outcome is {}; cleanup partial failure requires visible reason is {}; cleanup outcome implies report deletion is {}; real cleanup execution materialized is {}; real cleanup result emission materialized is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            supported_markers,
            self.shell_authority_required,
            self.cleanup_pending_distinct,
            self.cleanup_running_distinct,
            self.cleanup_succeeded_distinct,
            self.cleanup_partially_failed_distinct,
            self.cleanup_failed_distinct,
            self.cleanup_attempt_distinct_from_cleanup_outcome,
            self.cleanup_outcome_distinct_from_run_outcome,
            self.cleanup_partial_failure_requires_visible_reason,
            self.cleanup_outcome_implies_report_deletion,
            self.real_cleanup_execution_materialized,
            self.real_cleanup_result_emission_materialized,
        )
    }
}

pub fn run_cleanup_contract_state() -> RunCleanupContractSnapshot {
    RunCleanupContractSnapshot {
        stage: RunCleanupContractStage::FoundationMaterialized,
        contract_name: RUN_CLEANUP_CONTRACT_NAME,
        shell_command_name: RUN_CLEANUP_CONTRACT_COMMAND_NAME,
        supported_markers: SUPPORTED_RUN_CLEANUP_MARKERS,
        shell_authority_required: true,
        cleanup_pending_distinct: true,
        cleanup_running_distinct: true,
        cleanup_succeeded_distinct: true,
        cleanup_partially_failed_distinct: true,
        cleanup_failed_distinct: true,
        cleanup_attempt_distinct_from_cleanup_outcome: true,
        cleanup_outcome_distinct_from_run_outcome: true,
        cleanup_partial_failure_requires_visible_reason: true,
        cleanup_outcome_implies_report_deletion: false,
        real_cleanup_execution_materialized: true,
        real_cleanup_result_emission_materialized: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_cleanup_contract_keeps_attempt_and_outcome_distinct_without_claiming_execution() {
        let snapshot = run_cleanup_contract_state();

        assert_eq!(snapshot.contract_name, RUN_CLEANUP_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, RUN_CLEANUP_CONTRACT_COMMAND_NAME);
        assert_eq!(snapshot.supported_markers.len(), 5);
        assert!(snapshot.shell_authority_required);
        assert!(snapshot.cleanup_pending_distinct);
        assert!(snapshot.cleanup_running_distinct);
        assert!(snapshot.cleanup_succeeded_distinct);
        assert!(snapshot.cleanup_partially_failed_distinct);
        assert!(snapshot.cleanup_failed_distinct);
        assert!(snapshot.cleanup_attempt_distinct_from_cleanup_outcome);
        assert!(snapshot.cleanup_outcome_distinct_from_run_outcome);
        assert!(snapshot.cleanup_partial_failure_requires_visible_reason);
        assert!(!snapshot.cleanup_outcome_implies_report_deletion);
        assert!(snapshot.real_cleanup_execution_materialized);
        assert!(snapshot.real_cleanup_result_emission_materialized);
    }
}