use serde::{Deserialize, Serialize};

use crate::native_app_updater::{
    active_run_detected_for_updater_coordination, native_app_updater_apply_gate_for_runtime_state,
    native_app_updater_configuration_status, native_app_updater_runtime_state,
    NativeAppUpdaterConfigurationStatus, NativeAppUpdaterRuntimeState,
    NATIVE_APP_UPDATER_APPLY_COMMAND_NAME, NATIVE_APP_UPDATER_APPLY_GATE_COMMAND_NAME,
    NATIVE_APP_UPDATER_DISCOVERY_COMMAND_NAME, NATIVE_APP_UPDATER_PREPARE_COMMAND_NAME,
};

pub const UPDATER_CONTRACT_NAME: &str = "UpdaterLifecycleContract";
pub const UPDATER_CONTRACT_COMMAND_NAME: &str = "UpdaterContractState";
pub const ALLOWED_UPDATER_TRANSITIONS: [&str; 17] = [
    "UpdaterIdle -> CheckingForUpdates",
    "CheckingForUpdates -> NoUpdateAvailable",
    "CheckingForUpdates -> UpdateAvailable",
    "UpdateAvailable -> DownloadingUpdate",
    "DownloadingUpdate -> VerifyingUpdate",
    "VerifyingUpdate -> UpdateReady",
    "VerifyingUpdate -> UpdateRejected",
    "CheckingForUpdates -> UpdaterError",
    "DownloadingUpdate -> UpdaterError",
    "VerifyingUpdate -> UpdaterError",
    "UpdateReady -> ApplyingUpdate",
    "ApplyingUpdate -> UpdateApplied",
    "ApplyingUpdate -> UpdaterError",
    "NoUpdateAvailable -> UpdaterIdle",
    "UpdateRejected -> UpdaterIdle",
    "UpdateApplied -> UpdaterIdle",
    "UpdaterError -> UpdaterIdle",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdaterContractStage {
    DiscoveryMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdaterState {
    UpdaterIdle,
    CheckingForUpdates,
    NoUpdateAvailable,
    UpdateAvailable,
    DownloadingUpdate,
    VerifyingUpdate,
    UpdateReady,
    UpdateRejected,
    ApplyingUpdate,
    UpdateApplied,
    UpdaterError,
}

impl UpdaterState {
    pub const fn name(self) -> &'static str {
        match self {
            Self::UpdaterIdle => "UpdaterIdle",
            Self::CheckingForUpdates => "CheckingForUpdates",
            Self::NoUpdateAvailable => "NoUpdateAvailable",
            Self::UpdateAvailable => "UpdateAvailable",
            Self::DownloadingUpdate => "DownloadingUpdate",
            Self::VerifyingUpdate => "VerifyingUpdate",
            Self::UpdateReady => "UpdateReady",
            Self::UpdateRejected => "UpdateRejected",
            Self::ApplyingUpdate => "ApplyingUpdate",
            Self::UpdateApplied => "UpdateApplied",
            Self::UpdaterError => "UpdaterError",
        }
    }
}

pub const CANONICAL_UPDATER_STATES: [UpdaterState; 11] = [
    UpdaterState::UpdaterIdle,
    UpdaterState::CheckingForUpdates,
    UpdaterState::NoUpdateAvailable,
    UpdaterState::UpdateAvailable,
    UpdaterState::DownloadingUpdate,
    UpdaterState::VerifyingUpdate,
    UpdaterState::UpdateReady,
    UpdaterState::UpdateRejected,
    UpdaterState::ApplyingUpdate,
    UpdaterState::UpdateApplied,
    UpdaterState::UpdaterError,
];
pub const STABLE_UPDATER_STATES: [UpdaterState; 6] = [
    UpdaterState::UpdaterIdle,
    UpdaterState::NoUpdateAvailable,
    UpdaterState::UpdateAvailable,
    UpdaterState::UpdateReady,
    UpdaterState::UpdateRejected,
    UpdaterState::UpdaterError,
];
pub const TRANSIENT_UPDATER_STATES: [UpdaterState; 5] = [
    UpdaterState::CheckingForUpdates,
    UpdaterState::DownloadingUpdate,
    UpdaterState::VerifyingUpdate,
    UpdaterState::ApplyingUpdate,
    UpdaterState::UpdateApplied,
];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UpdaterContractSnapshot {
    pub stage: UpdaterContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub shell_discovery_command_name: &'static str,
    pub shell_prepare_command_name: &'static str,
    pub shell_apply_command_name: &'static str,
    pub shell_apply_gate_command_name: &'static str,
    pub canonical_state_names: [&'static str; 11],
    pub stable_state_names: [&'static str; 6],
    pub transient_state_names: [&'static str; 5],
    pub allowed_transition_names: [&'static str; 17],
    pub start_state_name: &'static str,
    pub current_state_name: &'static str,
    pub current_status_note: String,
    pub current_update_available: bool,
    pub current_update_ready: bool,
    pub current_candidate_rejected: bool,
    pub current_error_active: bool,
    pub discovery_configuration_source_name: &'static str,
    pub discovery_configuration_ready: bool,
    pub discovery_endpoints_configured: bool,
    pub discovery_pubkey_configured: bool,
    pub discovery_endpoint_count: usize,
    pub discovery_configuration_note: String,
    pub shell_authority_required: bool,
    pub frontend_presentation_only: bool,
    pub updater_state_separate_from_run_lifecycle: bool,
    pub no_update_state_distinct_from_error: bool,
    pub available_state_distinct_from_ready: bool,
    pub rejection_distinct_from_error: bool,
    pub verification_required_before_ready: bool,
    pub verification_required_before_apply: bool,
    pub reset_required_after_rejection_or_error: bool,
    pub discovery_allowed_during_active_run: bool,
    pub active_run_apply_forbidden: bool,
    pub apply_deferred_during_active_run: bool,
    pub apply_allowed_now: bool,
    pub apply_blocked_by_active_run_now: bool,
    pub apply_blocked_because_update_not_ready_now: bool,
    pub restart_coordination_required_after_apply: bool,
    pub apply_status_note: String,
    pub active_run_detected_during_last_check: bool,
    pub ready_state_requires_verified_artifact: bool,
    pub last_check_attempted: bool,
    pub last_check_started_at_epoch_ms: Option<u64>,
    pub last_check_completed_at_epoch_ms: Option<u64>,
    pub last_error_detail: Option<String>,
    pub last_discovered_version: Option<String>,
    pub last_discovered_release_notes: Option<String>,
    pub last_discovered_publication_date: Option<String>,
    pub real_update_check_materialized: bool,
    pub real_update_download_materialized: bool,
    pub real_update_verification_materialized: bool,
    pub real_update_apply_materialized: bool,
}

impl UpdaterContractSnapshot {
    pub fn summary(self) -> String {
        let canonical_state_names = self.canonical_state_names.join(", ");
        let stable_state_names = self.stable_state_names.join(", ");
        let transient_state_names = self.transient_state_names.join(", ");
        let allowed_transition_names = self.allowed_transition_names.join(", ");

        format!(
            "Updater lifecycle contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; shell discovery command name is {}; shell prepare command name is {}; shell apply command name is {}; shell apply-gate command name is {}; canonical state names are {}; stable state names are {}; transient state names are {}; allowed transition names are {}; start state name is {}; current state name is {}; current status note is {}; current update available is {}; current update ready is {}; current candidate rejected is {}; current error active is {}; discovery configuration source is {}; discovery configuration ready is {}; discovery endpoints configured is {}; discovery pubkey configured is {}; discovery endpoint count is {}; discovery configuration note is {}; shell authority required is {}; frontend presentation only is {}; updater state separate from run lifecycle is {}; no-update state distinct from error is {}; available state distinct from ready is {}; rejection distinct from error is {}; verification required before ready is {}; verification required before apply is {}; reset required after rejection or error is {}; discovery allowed during active run is {}; active-run apply forbidden is {}; apply deferred during active run is {}; apply allowed now is {}; apply blocked by active run now is {}; apply blocked because update is not ready now is {}; restart coordination required after apply is {}; apply status note is {}; active run detected during last check is {}; ready state requires verified artifact is {}; last check attempted is {}; last check started at epoch ms is {:?}; last check completed at epoch ms is {:?}; last error detail is {:?}; last discovered version is {:?}; last discovered release notes are {:?}; last discovered publication date is {:?}; real update-check materialized is {}; real update-download materialized is {}; real update-verification materialized is {}; real update-apply materialized is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            self.shell_discovery_command_name,
            self.shell_prepare_command_name,
            self.shell_apply_command_name,
            self.shell_apply_gate_command_name,
            canonical_state_names,
            stable_state_names,
            transient_state_names,
            allowed_transition_names,
            self.start_state_name,
            self.current_state_name,
            self.current_status_note,
            self.current_update_available,
            self.current_update_ready,
            self.current_candidate_rejected,
            self.current_error_active,
            self.discovery_configuration_source_name,
            self.discovery_configuration_ready,
            self.discovery_endpoints_configured,
            self.discovery_pubkey_configured,
            self.discovery_endpoint_count,
            self.discovery_configuration_note,
            self.shell_authority_required,
            self.frontend_presentation_only,
            self.updater_state_separate_from_run_lifecycle,
            self.no_update_state_distinct_from_error,
            self.available_state_distinct_from_ready,
            self.rejection_distinct_from_error,
            self.verification_required_before_ready,
            self.verification_required_before_apply,
            self.reset_required_after_rejection_or_error,
            self.discovery_allowed_during_active_run,
            self.active_run_apply_forbidden,
            self.apply_deferred_during_active_run,
            self.apply_allowed_now,
            self.apply_blocked_by_active_run_now,
            self.apply_blocked_because_update_not_ready_now,
            self.restart_coordination_required_after_apply,
            self.apply_status_note,
            self.active_run_detected_during_last_check,
            self.ready_state_requires_verified_artifact,
            self.last_check_attempted,
            self.last_check_started_at_epoch_ms,
            self.last_check_completed_at_epoch_ms,
            self.last_error_detail,
            self.last_discovered_version,
            self.last_discovered_release_notes,
            self.last_discovered_publication_date,
            self.real_update_check_materialized,
            self.real_update_download_materialized,
            self.real_update_verification_materialized,
            self.real_update_apply_materialized,
        )
    }
}

fn project_updater_contract_snapshot(
    configuration_status: NativeAppUpdaterConfigurationStatus,
    runtime_state: NativeAppUpdaterRuntimeState,
) -> UpdaterContractSnapshot {
    let apply_gate = native_app_updater_apply_gate_for_runtime_state(
        &runtime_state,
        active_run_detected_for_updater_coordination(),
    );

    UpdaterContractSnapshot {
        stage: UpdaterContractStage::DiscoveryMaterialized,
        contract_name: UPDATER_CONTRACT_NAME,
        shell_command_name: UPDATER_CONTRACT_COMMAND_NAME,
        shell_discovery_command_name: NATIVE_APP_UPDATER_DISCOVERY_COMMAND_NAME,
        shell_prepare_command_name: NATIVE_APP_UPDATER_PREPARE_COMMAND_NAME,
        shell_apply_command_name: NATIVE_APP_UPDATER_APPLY_COMMAND_NAME,
        shell_apply_gate_command_name: NATIVE_APP_UPDATER_APPLY_GATE_COMMAND_NAME,
        canonical_state_names: CANONICAL_UPDATER_STATES.map(UpdaterState::name),
        stable_state_names: STABLE_UPDATER_STATES.map(UpdaterState::name),
        transient_state_names: TRANSIENT_UPDATER_STATES.map(UpdaterState::name),
        allowed_transition_names: ALLOWED_UPDATER_TRANSITIONS,
        start_state_name: UpdaterState::UpdaterIdle.name(),
        current_state_name: runtime_state.current_state.name(),
        current_status_note: runtime_state.current_status_note,
        current_update_available: runtime_state.current_update_available,
        current_update_ready: runtime_state.current_update_ready,
        current_candidate_rejected: runtime_state.current_candidate_rejected,
        current_error_active: runtime_state.current_error_active,
        discovery_configuration_source_name: configuration_status.configuration_source_name,
        discovery_configuration_ready: configuration_status.configuration_ready,
        discovery_endpoints_configured: configuration_status.endpoints_configured,
        discovery_pubkey_configured: configuration_status.pubkey_configured,
        discovery_endpoint_count: configuration_status.endpoint_count,
        discovery_configuration_note: configuration_status.configuration_note,
        shell_authority_required: true,
        frontend_presentation_only: true,
        updater_state_separate_from_run_lifecycle: true,
        no_update_state_distinct_from_error: true,
        available_state_distinct_from_ready: true,
        rejection_distinct_from_error: true,
        verification_required_before_ready: true,
        verification_required_before_apply: true,
        reset_required_after_rejection_or_error: true,
        discovery_allowed_during_active_run: true,
        active_run_apply_forbidden: true,
        apply_deferred_during_active_run: true,
        apply_allowed_now: apply_gate.apply_allowed_now,
        apply_blocked_by_active_run_now: apply_gate.apply_blocked_by_active_run,
        apply_blocked_because_update_not_ready_now: apply_gate
            .apply_blocked_because_update_not_ready,
        restart_coordination_required_after_apply: apply_gate.restart_coordination_required,
        apply_status_note: apply_gate.apply_status_note,
        active_run_detected_during_last_check: runtime_state.active_run_detected_during_last_check,
        ready_state_requires_verified_artifact: true,
        last_check_attempted: runtime_state.last_check_attempted,
        last_check_started_at_epoch_ms: runtime_state.last_check_started_at_epoch_ms,
        last_check_completed_at_epoch_ms: runtime_state.last_check_completed_at_epoch_ms,
        last_error_detail: runtime_state.last_error_detail,
        last_discovered_version: runtime_state.last_discovered_version,
        last_discovered_release_notes: runtime_state.last_discovered_release_notes,
        last_discovered_publication_date: runtime_state.last_discovered_publication_date,
        real_update_check_materialized: runtime_state.real_update_check_materialized,
        real_update_download_materialized: runtime_state.real_update_download_materialized,
        real_update_verification_materialized: runtime_state.real_update_verification_materialized,
        real_update_apply_materialized: runtime_state.real_update_apply_materialized,
    }
}

pub fn updater_contract_state() -> UpdaterContractSnapshot {
    project_updater_contract_snapshot(
        native_app_updater_configuration_status(),
        native_app_updater_runtime_state(),
    )
}

#[cfg(test)]
pub(crate) fn mock_updater_contract_state_for_test(
    runtime_state: NativeAppUpdaterRuntimeState,
) -> UpdaterContractSnapshot {
    project_updater_contract_snapshot(
        NativeAppUpdaterConfigurationStatus {
            configuration_source_name: "EnvironmentVariables",
            configuration_ready: true,
            endpoints_configured: true,
            pubkey_configured: true,
            endpoint_count: 1,
            configuration_note:
                "Signed native app-updater discovery is configured through MIRO_FISH_APP_UPDATER_ENDPOINTS and MIRO_FISH_APP_UPDATER_PUBKEY."
                    .to_string(),
        },
        runtime_state,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updater_contract_keeps_canonical_state_vocabulary_without_claiming_execution() {
        let snapshot = updater_contract_state();

        assert_eq!(snapshot.contract_name, UPDATER_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, UPDATER_CONTRACT_COMMAND_NAME);
        assert_eq!(
            snapshot.shell_discovery_command_name,
            NATIVE_APP_UPDATER_DISCOVERY_COMMAND_NAME
        );
        assert_eq!(
            snapshot.shell_prepare_command_name,
            NATIVE_APP_UPDATER_PREPARE_COMMAND_NAME
        );
        assert_eq!(
            snapshot.shell_apply_command_name,
            NATIVE_APP_UPDATER_APPLY_COMMAND_NAME
        );
        assert_eq!(
            snapshot.shell_apply_gate_command_name,
            NATIVE_APP_UPDATER_APPLY_GATE_COMMAND_NAME
        );
        assert_eq!(snapshot.canonical_state_names.len(), 11);
        assert_eq!(snapshot.stable_state_names.len(), 6);
        assert_eq!(snapshot.transient_state_names.len(), 5);
        assert_eq!(snapshot.allowed_transition_names.len(), 17);
        assert_eq!(snapshot.start_state_name, "UpdaterIdle");
        assert_eq!(snapshot.current_state_name, "UpdaterIdle");
        assert!(snapshot
            .current_status_note
            .contains("no signed discovery check has run yet"));
        assert!(snapshot.shell_authority_required);
        assert!(snapshot.frontend_presentation_only);
        assert!(snapshot.updater_state_separate_from_run_lifecycle);
        assert!(snapshot.no_update_state_distinct_from_error);
        assert!(snapshot.available_state_distinct_from_ready);
        assert!(snapshot.rejection_distinct_from_error);
        assert!(snapshot.verification_required_before_ready);
        assert!(snapshot.verification_required_before_apply);
        assert!(snapshot.reset_required_after_rejection_or_error);
        assert!(snapshot.discovery_allowed_during_active_run);
        assert!(snapshot.active_run_apply_forbidden);
        assert!(snapshot.apply_deferred_during_active_run);
        assert!(!snapshot.apply_allowed_now);
        assert!(!snapshot.apply_blocked_by_active_run_now);
        assert!(snapshot.apply_blocked_because_update_not_ready_now);
        assert!(!snapshot.restart_coordination_required_after_apply);
        assert!(snapshot.ready_state_requires_verified_artifact);
        assert!(!snapshot.current_update_available);
        assert!(!snapshot.current_update_ready);
        assert!(!snapshot.current_candidate_rejected);
        assert!(!snapshot.current_error_active);
        assert!(!snapshot.last_check_attempted);
        assert!(snapshot.last_discovered_version.is_none());
        assert!(!snapshot.real_update_check_materialized);
        assert!(!snapshot.real_update_download_materialized);
        assert!(!snapshot.real_update_verification_materialized);
        assert!(!snapshot.real_update_apply_materialized);
        assert!(snapshot
            .allowed_transition_names
            .contains(&"VerifyingUpdate -> UpdateReady"));
        assert!(!snapshot
            .allowed_transition_names
            .contains(&"UpdateAvailable -> ApplyingUpdate"));
    }
}