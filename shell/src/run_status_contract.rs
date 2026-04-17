use serde::{Deserialize, Serialize};

use crate::cleanup_scope_mapping::{cleanup_scope_mapping_state, CLEANUP_SCOPE_MAPPING_NAME};
use crate::report_access_contract::{report_access_contract_state, REPORT_ACCESS_CONTRACT_NAME};
use crate::run_cleanup_contract::{run_cleanup_contract_state, RUN_CLEANUP_CONTRACT_NAME};
use crate::run_lifecycle_state::{
    project_run_lifecycle_state, CanonicalRunLifecycleState, RUN_LIFECYCLE_STATE_NAME,
};
use crate::run_session_start::{run_session_state_from_shell, RunSessionStateSnapshot};
use crate::run_status_projection::project_run_status_projection;
use crate::run_finalization_contract::{
    run_finalization_contract_state, RUN_FINALIZATION_CONTRACT_NAME,
};
use crate::startup_janitor_contract::{
    startup_janitor_contract_state, STARTUP_JANITOR_CONTRACT_NAME,
};
use crate::updater_contract::{
    updater_contract_state, UpdaterContractSnapshot, UPDATER_CONTRACT_NAME,
};

pub const RUN_STATUS_CONTRACT_NAME: &str = "RunStatusContract";
pub const RUN_STATUS_STATE_COMMAND_NAME: &str = "RunStatusState";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatusContractStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunCancellationRequestContract {
    pub shell_command_boundary_required: bool,
    pub shell_owned_cancelability_authority_required: bool,
    pub active_run_identifier_required_when_cancelable: bool,
    pub frontend_direct_engine_signal_allowed: bool,
    pub cancelability_derived_from_canonical_state: bool,
    pub cancellation_request_shape_materialized: bool,
    pub cancellation_command_explicit: bool,
    pub cancellation_command_implemented: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunCancellationResponseContract {
    pub accepted_response_defined: bool,
    pub rejected_response_defined: bool,
    pub already_terminal_response_defined: bool,
    pub no_active_run_response_defined: bool,
    pub hidden_post_cancel_progress_allowed: bool,
    pub forceful_termination_implied_by_acceptance: bool,
    pub controlled_cancellation_behavior_materialized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunStatusContractSnapshot {
    pub stage: RunStatusContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub lifecycle_surface_name: &'static str,
    pub finalization_contract_surface_name: &'static str,
    pub report_access_contract_surface_name: &'static str,
    pub run_cleanup_contract_surface_name: &'static str,
    pub cleanup_scope_mapping_surface_name: &'static str,
    pub startup_janitor_contract_surface_name: &'static str,
    pub updater_contract_surface_name: &'static str,
    pub current_lifecycle_state: CanonicalRunLifecycleState,
    pub current_lifecycle_state_name: &'static str,
    pub current_updater_state_name: &'static str,
    pub current_run_id: Option<String>,
    pub current_session_id: Option<String>,
    pub current_workspace_reference: Option<String>,
    pub requested_round_count: Option<usize>,
    pub completed_round_count: Option<usize>,
    pub tracked_total_token_units: Option<usize>,
    pub tracked_total_duration_ms: Option<usize>,
    pub latest_status_note: String,
    pub updater_status_note: String,
    pub latest_visible_sequence_number: Option<u64>,
    pub updater_available_version: Option<String>,
    pub shell_authority_required: bool,
    pub current_status_retrieval_explicit: bool,
    pub current_status_retrieval_implemented: bool,
    pub canonical_state_included: bool,
    pub run_session_identity_included: bool,
    pub workspace_reference_included: bool,
    pub round_count_visibility_included: bool,
    pub token_duration_visibility_included: bool,
    pub degradation_flags_included: bool,
    pub cancelability_flags_included: bool,
    pub latest_status_note_included: bool,
    pub latest_visible_sequence_marker_included: bool,
    pub event_cursor_alignment_defined: bool,
    pub finalization_output_distinction_preserved: bool,
    pub cleanup_attempt_outcome_distinction_preserved: bool,
    pub startup_janitor_outcome_distinct_from_run_outcome: bool,
    pub preserved_report_cleanup_exception_materialized: bool,
    pub completion_classification_implies_report_generation_success: bool,
    pub report_generation_success_implies_report_available: bool,
    pub report_availability_implies_export_accessibility: bool,
    pub cleanup_outcome_implies_report_deletion: bool,
    pub real_cleanup_execution_materialized: bool,
    pub real_startup_janitor_execution_materialized: bool,
    pub updater_status_visible_separately_from_run_lifecycle: bool,
    pub updater_discovery_check_materialized: bool,
    pub updater_apply_deferred_during_active_run: bool,
    pub updater_coordination_visible_during_active_run: bool,
    pub frontend_state_inference_allowed: bool,
    pub current_state_cancelable: bool,
    pub current_state_terminal: bool,
    pub current_state_retryable: bool,
    pub current_state_degraded: bool,
    pub ordered_event_history_materialized: bool,
    pub live_status_push_materialized: bool,
    pub cancellation_request_contract: RunCancellationRequestContract,
    pub cancellation_response_contract: RunCancellationResponseContract,
}

impl RunStatusContractSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Run status contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; lifecycle surface name is {}; finalization contract surface name is {}; report access contract surface name is {}; run cleanup contract surface name is {}; cleanup-scope mapping surface name is {}; startup janitor contract surface name is {}; updater contract surface name is {}; current lifecycle state is {} ({:?}); current updater state is {}; current run id is {:?}; current session id is {:?}; current workspace reference is {:?}; requested round count is {:?}; completed round count is {:?}; tracked total token units is {:?}; tracked total duration ms is {:?}; latest status note is {}; updater status note is {}; latest visible sequence number is {:?}; updater available version is {:?}; shell authority required is {}; current status retrieval explicit is {}; current status retrieval implemented is {}; canonical state included is {}; run/session identity included is {}; workspace reference included is {}; round-count visibility included is {}; token-and-duration visibility included is {}; degradation flags included is {}; cancelability flags included is {}; latest status note included is {}; latest visible sequence marker included is {}; event cursor alignment defined is {}; finalization/output distinction preserved is {}; cleanup attempt/outcome distinction preserved is {}; startup janitor outcome distinct from run outcome is {}; preserved report cleanup exception materialized is {}; completion classification implies report generation success is {}; report generation success implies report available is {}; report availability implies export accessibility is {}; cleanup outcome implies report deletion is {}; real cleanup execution materialized is {}; real startup janitor execution materialized is {}; updater status visible separately from run lifecycle is {}; updater discovery check materialized is {}; updater apply deferred during active run is {}; updater coordination visible during active run is {}; frontend state inference allowed is {}; current state cancelable is {}; current state terminal is {}; current state retryable is {}; current state degraded is {}; ordered event history materialized is {}; live status push materialized is {}; cancellation request command explicit is {}; cancellation request command implemented is {}; controlled cancellation behavior materialized is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            self.lifecycle_surface_name,
            self.finalization_contract_surface_name,
            self.report_access_contract_surface_name,
            self.run_cleanup_contract_surface_name,
            self.cleanup_scope_mapping_surface_name,
            self.startup_janitor_contract_surface_name,
            self.updater_contract_surface_name,
            self.current_lifecycle_state_name,
            self.current_lifecycle_state,
            self.current_updater_state_name,
            self.current_run_id,
            self.current_session_id,
            self.current_workspace_reference,
            self.requested_round_count,
            self.completed_round_count,
            self.tracked_total_token_units,
            self.tracked_total_duration_ms,
            self.latest_status_note,
            self.updater_status_note,
            self.latest_visible_sequence_number,
            self.updater_available_version,
            self.shell_authority_required,
            self.current_status_retrieval_explicit,
            self.current_status_retrieval_implemented,
            self.canonical_state_included,
            self.run_session_identity_included,
            self.workspace_reference_included,
            self.round_count_visibility_included,
            self.token_duration_visibility_included,
            self.degradation_flags_included,
            self.cancelability_flags_included,
            self.latest_status_note_included,
            self.latest_visible_sequence_marker_included,
            self.event_cursor_alignment_defined,
            self.finalization_output_distinction_preserved,
            self.cleanup_attempt_outcome_distinction_preserved,
            self.startup_janitor_outcome_distinct_from_run_outcome,
            self.preserved_report_cleanup_exception_materialized,
            self.completion_classification_implies_report_generation_success,
            self.report_generation_success_implies_report_available,
            self.report_availability_implies_export_accessibility,
            self.cleanup_outcome_implies_report_deletion,
            self.real_cleanup_execution_materialized,
            self.real_startup_janitor_execution_materialized,
            self.updater_status_visible_separately_from_run_lifecycle,
            self.updater_discovery_check_materialized,
            self.updater_apply_deferred_during_active_run,
            self.updater_coordination_visible_during_active_run,
            self.frontend_state_inference_allowed,
            self.current_state_cancelable,
            self.current_state_terminal,
            self.current_state_retryable,
            self.current_state_degraded,
            self.ordered_event_history_materialized,
            self.live_status_push_materialized,
            self.cancellation_request_contract.cancellation_command_explicit,
            self.cancellation_request_contract.cancellation_command_implemented,
            self.cancellation_response_contract.controlled_cancellation_behavior_materialized,
        )
    }
}

pub fn project_run_status_contract(
    run_session_state: &RunSessionStateSnapshot,
) -> RunStatusContractSnapshot {
    project_run_status_contract_with_updater_contract(run_session_state, updater_contract_state())
}

fn project_run_status_contract_with_updater_contract(
    run_session_state: &RunSessionStateSnapshot,
    updater_contract: UpdaterContractSnapshot,
) -> RunStatusContractSnapshot {
    let lifecycle_state = project_run_lifecycle_state(run_session_state);
    let status_projection = project_run_status_projection(run_session_state);
    let finalization_contract = run_finalization_contract_state();
    let report_access_contract = report_access_contract_state();
    let run_cleanup_contract = run_cleanup_contract_state();
    let cleanup_scope_mapping = cleanup_scope_mapping_state();
    let startup_janitor_contract = startup_janitor_contract_state();

    RunStatusContractSnapshot {
        stage: RunStatusContractStage::FoundationMaterialized,
        contract_name: RUN_STATUS_CONTRACT_NAME,
        shell_command_name: RUN_STATUS_STATE_COMMAND_NAME,
        lifecycle_surface_name: RUN_LIFECYCLE_STATE_NAME,
        finalization_contract_surface_name: RUN_FINALIZATION_CONTRACT_NAME,
        report_access_contract_surface_name: REPORT_ACCESS_CONTRACT_NAME,
        run_cleanup_contract_surface_name: RUN_CLEANUP_CONTRACT_NAME,
        cleanup_scope_mapping_surface_name: CLEANUP_SCOPE_MAPPING_NAME,
        startup_janitor_contract_surface_name: STARTUP_JANITOR_CONTRACT_NAME,
        updater_contract_surface_name: UPDATER_CONTRACT_NAME,
        current_lifecycle_state: lifecycle_state.current_state,
        current_lifecycle_state_name: lifecycle_state.current_state_name,
        current_updater_state_name: updater_contract.current_state_name,
        current_run_id: run_session_state.run_id.clone(),
        current_session_id: run_session_state.session_id.clone(),
        current_workspace_reference: run_session_state.workspace_reference.clone(),
        requested_round_count: run_session_state.requested_round_count,
        completed_round_count: status_projection.completed_round_count,
        tracked_total_token_units: status_projection.tracked_total_token_units,
        tracked_total_duration_ms: status_projection.tracked_total_duration_ms,
        latest_status_note: status_projection.latest_status_note,
        updater_status_note: format!(
            "{} {}",
            updater_contract.current_status_note,
            updater_contract.apply_status_note,
        )
        .trim()
        .to_string(),
        latest_visible_sequence_number: status_projection.latest_visible_sequence_number,
        updater_available_version: updater_contract.last_discovered_version,
        shell_authority_required: true,
        current_status_retrieval_explicit: true,
        current_status_retrieval_implemented: true,
        canonical_state_included: true,
        run_session_identity_included: true,
        workspace_reference_included: true,
        round_count_visibility_included: true,
        token_duration_visibility_included: true,
        degradation_flags_included: true,
        cancelability_flags_included: true,
        latest_status_note_included: true,
        latest_visible_sequence_marker_included: true,
        event_cursor_alignment_defined: true,
        finalization_output_distinction_preserved: finalization_contract
            .finalization_entered_distinct
            && finalization_contract.finalization_succeeded_distinct
            && finalization_contract.report_generation_succeeded_distinct
            && finalization_contract.report_available_distinct
            && report_access_contract.response_contract.availability_representation_supported,
        cleanup_attempt_outcome_distinction_preserved: run_cleanup_contract
            .cleanup_attempt_distinct_from_cleanup_outcome
            && run_cleanup_contract.cleanup_outcome_distinct_from_run_outcome,
        startup_janitor_outcome_distinct_from_run_outcome: startup_janitor_contract
            .janitor_outcome_distinct_from_run_outcome,
        preserved_report_cleanup_exception_materialized: cleanup_scope_mapping
            .preserved_report_exception_explicit,
        completion_classification_implies_report_generation_success: true,
        report_generation_success_implies_report_available: false,
        report_availability_implies_export_accessibility: false,
        cleanup_outcome_implies_report_deletion: run_cleanup_contract
            .cleanup_outcome_implies_report_deletion,
        real_cleanup_execution_materialized: run_cleanup_contract.real_cleanup_execution_materialized,
        real_startup_janitor_execution_materialized: startup_janitor_contract
            .real_startup_janitor_execution_materialized,
        updater_status_visible_separately_from_run_lifecycle: updater_contract
            .updater_state_separate_from_run_lifecycle,
        updater_discovery_check_materialized: updater_contract.real_update_check_materialized,
        updater_apply_deferred_during_active_run: updater_contract.apply_deferred_during_active_run,
        updater_coordination_visible_during_active_run: !lifecycle_state.current_state_terminal
            && lifecycle_state.current_state != CanonicalRunLifecycleState::RunDraft
            && updater_contract.discovery_allowed_during_active_run
            && updater_contract.apply_deferred_during_active_run,
        frontend_state_inference_allowed: false,
        current_state_cancelable: lifecycle_state.current_state_cancelable,
        current_state_terminal: lifecycle_state.current_state_terminal,
        current_state_retryable: lifecycle_state.current_state_retryable,
        current_state_degraded: matches!(
            lifecycle_state.current_state,
            CanonicalRunLifecycleState::PausedRateLimit
                | CanonicalRunLifecycleState::PausedNetwork
        ),
        ordered_event_history_materialized: status_projection.ordered_event_history_materialized,
        live_status_push_materialized: false,
        cancellation_request_contract: RunCancellationRequestContract {
            shell_command_boundary_required: true,
            shell_owned_cancelability_authority_required: true,
            active_run_identifier_required_when_cancelable: true,
            frontend_direct_engine_signal_allowed: false,
            cancelability_derived_from_canonical_state: true,
            cancellation_request_shape_materialized: true,
            cancellation_command_explicit: true,
            cancellation_command_implemented: true,
        },
        cancellation_response_contract: RunCancellationResponseContract {
            accepted_response_defined: true,
            rejected_response_defined: true,
            already_terminal_response_defined: true,
            no_active_run_response_defined: true,
            hidden_post_cancel_progress_allowed: false,
            forceful_termination_implied_by_acceptance: false,
            controlled_cancellation_behavior_materialized: true,
        },
    }
}

pub fn run_status_contract_state() -> RunStatusContractSnapshot {
    project_run_status_contract(&run_session_state_from_shell())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_app_updater::mock_update_available_runtime_state_for_test;
    use crate::run_session_contract::RunSessionStateClass;
    use crate::updater_contract::mock_updater_contract_state_for_test;

    fn run_session_state_for_class(class: RunSessionStateClass) -> RunSessionStateSnapshot {
        let mut snapshot = RunSessionStateSnapshot::not_started();
        snapshot.current_state_class = class;
        snapshot.current_state_name = class.label();
        snapshot.detail_message = format!("Mock {:?} detail", class);
        snapshot
    }

    #[test]
    fn status_contract_keeps_shell_authority_and_cancelability_logic_shell_side() {
        let snapshot = project_run_status_contract(&run_session_state_for_class(
            RunSessionStateClass::Running,
        ));

        assert_eq!(snapshot.contract_name, RUN_STATUS_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, RUN_STATUS_STATE_COMMAND_NAME);
        assert_eq!(snapshot.lifecycle_surface_name, RUN_LIFECYCLE_STATE_NAME);
        assert_eq!(
            snapshot.finalization_contract_surface_name,
            RUN_FINALIZATION_CONTRACT_NAME
        );
        assert_eq!(
            snapshot.report_access_contract_surface_name,
            REPORT_ACCESS_CONTRACT_NAME
        );
        assert_eq!(snapshot.run_cleanup_contract_surface_name, RUN_CLEANUP_CONTRACT_NAME);
        assert_eq!(
            snapshot.cleanup_scope_mapping_surface_name,
            CLEANUP_SCOPE_MAPPING_NAME
        );
        assert_eq!(
            snapshot.startup_janitor_contract_surface_name,
            STARTUP_JANITOR_CONTRACT_NAME
        );
        assert_eq!(snapshot.updater_contract_surface_name, UPDATER_CONTRACT_NAME);
        assert_eq!(snapshot.current_updater_state_name, "UpdaterIdle");
        assert!(snapshot.shell_authority_required);
        assert!(snapshot.current_status_retrieval_explicit);
        assert!(snapshot.current_status_retrieval_implemented);
        assert!(snapshot.cancelability_flags_included);
        assert!(snapshot.finalization_output_distinction_preserved);
        assert!(snapshot.cleanup_attempt_outcome_distinction_preserved);
        assert!(snapshot.startup_janitor_outcome_distinct_from_run_outcome);
        assert!(snapshot.preserved_report_cleanup_exception_materialized);
        assert!(snapshot.completion_classification_implies_report_generation_success);
        assert!(!snapshot.report_generation_success_implies_report_available);
        assert!(!snapshot.report_availability_implies_export_accessibility);
        assert!(!snapshot.cleanup_outcome_implies_report_deletion);
        assert!(snapshot.real_cleanup_execution_materialized);
        assert!(snapshot.real_startup_janitor_execution_materialized);
        assert!(snapshot.updater_status_visible_separately_from_run_lifecycle);
        assert!(!snapshot.updater_discovery_check_materialized);
        assert!(snapshot.updater_apply_deferred_during_active_run);
        assert!(snapshot.updater_coordination_visible_during_active_run);
        assert!(snapshot.current_state_cancelable);
        assert!(!snapshot.current_state_terminal);
        assert!(!snapshot.frontend_state_inference_allowed);
        assert!(snapshot.ordered_event_history_materialized);
        assert!(!snapshot.live_status_push_materialized);
        assert!(snapshot
            .cancellation_request_contract
            .cancelability_derived_from_canonical_state);
        assert!(snapshot
            .cancellation_request_contract
            .cancellation_command_explicit);
        assert!(snapshot
            .cancellation_request_contract
            .cancellation_command_implemented);
        assert!(snapshot
            .cancellation_response_contract
            .accepted_response_defined);
        assert!(snapshot
            .cancellation_response_contract
            .controlled_cancellation_behavior_materialized);
    }

    #[test]
    fn completed_status_contract_does_not_claim_visible_event_history() {
        let snapshot = project_run_status_contract(&run_session_state_for_class(
            RunSessionStateClass::Finished,
        ));

        assert_eq!(snapshot.current_lifecycle_state_name, "Completed");
        assert!(snapshot.current_state_terminal);
        assert!(!snapshot.current_state_cancelable);
        assert!(!snapshot.updater_coordination_visible_during_active_run);
        assert_eq!(snapshot.latest_visible_sequence_number, None);
        assert!(snapshot.latest_visible_sequence_marker_included);
        assert!(snapshot.finalization_output_distinction_preserved);
        assert!(snapshot.ordered_event_history_materialized);
    }

    #[test]
    fn status_contract_projects_active_run_updater_coordination_after_discovery_check() {
        let updater_contract = mock_updater_contract_state_for_test(
            mock_update_available_runtime_state_for_test(true),
        );
        let snapshot = project_run_status_contract_with_updater_contract(
            &run_session_state_for_class(RunSessionStateClass::Running),
            updater_contract,
        );

        assert_eq!(snapshot.current_lifecycle_state_name, "RunningRound");
        assert_eq!(snapshot.current_updater_state_name, "UpdateAvailable");
        assert_eq!(snapshot.updater_available_version.as_deref(), Some("1.2.3"));
        assert!(snapshot.updater_discovery_check_materialized);
        assert!(snapshot.updater_apply_deferred_during_active_run);
        assert!(snapshot.updater_coordination_visible_during_active_run);
        assert!(snapshot
            .updater_status_note
            .contains("during an active run"));
        assert!(snapshot
            .updater_status_note
            .contains("blocked while the run is active"));
    }
}