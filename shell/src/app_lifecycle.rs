pub const STATE_APP_NOT_RUNNING: &str = "AppNotRunning";
pub const STATE_APP_LAUNCHING: &str = "AppLaunching";
pub const STATE_STARTUP_JANITOR_RUNNING: &str = "StartupJanitorRunning";
pub const STATE_STARTUP_CHECKS_RUNNING: &str = "StartupChecksRunning";
pub const STATE_RUNTIME_HYDRATION_REQUIRED: &str = "RuntimeHydrationRequired";
pub const STATE_RUNTIME_HYDRATING: &str = "RuntimeHydrating";
pub const STATE_ACTIVATION_REQUIRED: &str = "ActivationRequired";
pub const STATE_ACTIVATION_REVOKED: &str = "ActivationRevoked";
pub const STATE_CONFIGURATION_REQUIRED: &str = "ConfigurationRequired";
pub const STATE_READY_IDLE: &str = "ReadyIdle";
pub const STATE_APP_SHUTTING_DOWN: &str = "AppShuttingDown";
pub const STATE_STARTUP_FATAL_ERROR: &str = "StartupFatalError";

use crate::activation_readiness::activation_readiness_truth_surface;
use crate::activation_storage_boundary::ActivationValidationStatus;
use crate::cleanup_scope_mapping::cleanup_scope_mapping_state;
use crate::preflight_readiness::preflight_readiness_truth_surface;
use crate::provider_readiness::provider_readiness_truth_surface;
use crate::report_access_contract::report_access_contract_state;
use crate::report_output_contract::report_output_contract_state;
use crate::report_storage_boundary::report_storage_boundary_state;
use crate::run_bounded_memory::run_bounded_memory_surface;
use crate::run_cleanup_contract::run_cleanup_contract_state;
use crate::run_event_contract::run_event_contract_state;
use crate::run_finalization_contract::run_finalization_contract_state;
use crate::run_lifecycle_state::run_lifecycle_state_from_shell;
use crate::run_session_start::shell_shutdown_coordination_flags;
use crate::run_moderated_rounds::run_moderated_rounds_surface;
use crate::run_status_contract::run_status_contract_state;
use crate::run_start_readiness::run_start_readiness_truth_surface;
use crate::run_token_duration_tracking::run_token_duration_tracking_surface;
use crate::runtime_hydration::runtime_hydration_state;
use crate::runtime_persona_contract::runtime_persona_contract_surface;
use crate::startup_janitor_contract::startup_janitor_contract_state;
use crate::updater_contract::updater_contract_state;

#[derive(Debug, Clone, Copy)]
pub enum AppLifecycleStage {
    RunStartFoundationMaterialized,
}

#[derive(Debug, Clone, Copy)]
pub struct AppLifecycleSnapshot {
    pub stage: AppLifecycleStage,
    pub canonical_state_surface_materialized: bool,
    pub lifecycle_execution_materialized: bool,
    pub start_state_name: &'static str,
    pub launch_state_name: &'static str,
    pub startup_janitor_state_name: &'static str,
    pub startup_checks_state_name: &'static str,
    pub runtime_hydration_required_state_name: &'static str,
    pub runtime_hydrating_state_name: &'static str,
    pub activation_required_state_name: &'static str,
    pub activation_revoked_state_name: &'static str,
    pub configuration_required_state_name: &'static str,
    pub ready_idle_state_name: &'static str,
    pub shutting_down_state_name: &'static str,
    pub startup_fatal_error_state_name: &'static str,
    pub current_state_name: &'static str,
    pub readiness_domains_separated: bool,
    pub activation_gating_surface_materialized: bool,
    pub activation_to_revoked_transition_defined: bool,
    pub activation_to_configuration_transition_defined: bool,
    pub activation_to_ready_transition_defined: bool,
    pub preflight_readiness_evaluated_separately: bool,
    pub preflight_state_name: &'static str,
    pub preflight_run_start_eligible: bool,
    pub run_start_blocked_without_preflight_eligibility: bool,
    pub ready_idle_requires_preflight_before_run_start: bool,
    pub run_start_readiness_evaluated_separately: bool,
    pub run_start_foundation_state_name: &'static str,
    pub run_start_foundation_blocked: bool,
    pub run_start_execution_materialized: bool,
    pub runtime_persona_foundation_materialized: bool,
    pub bounded_memory_foundation_materialized: bool,
    pub transcript_archive_separation_materialized: bool,
    pub moderated_round_execution_materialized: bool,
    pub token_duration_tracking_materialized: bool,
    pub phase_seven_run_lifecycle_surface_materialized: bool,
    pub phase_seven_run_status_contract_materialized: bool,
    pub phase_seven_run_event_contract_materialized: bool,
    pub phase_nine_run_cleanup_contract_materialized: bool,
    pub phase_nine_cleanup_scope_mapping_materialized: bool,
    pub phase_nine_startup_janitor_contract_materialized: bool,
    pub phase_eight_run_finalization_contract_materialized: bool,
    pub phase_eight_report_output_contract_materialized: bool,
    pub phase_eight_report_storage_boundary_materialized: bool,
    pub phase_eight_report_access_contract_materialized: bool,
    pub cleanup_outcome_distinct_from_run_outcome: bool,
    pub startup_janitor_outcome_distinct_from_run_outcome: bool,
    pub preserved_report_cleanup_exception_visible: bool,
    pub current_report_availability_state_name: &'static str,
    pub current_report_available: bool,
    pub completion_state_implies_report_availability: bool,
    pub cleanup_state_implies_report_deletion: bool,
    pub updater_contract_surface_materialized: bool,
    pub updater_current_state_name: &'static str,
    pub updater_discovery_configuration_ready: bool,
    pub updater_state_separate_from_lifecycle_truth: bool,
    pub updater_discovery_not_required_for_ready_idle: bool,
    pub updater_apply_deferred_during_active_run: bool,
    pub current_run_lifecycle_state_name: &'static str,
    pub current_run_identity_visible: bool,
    pub current_run_cancelability_known: bool,
    pub current_run_cancelable: bool,
    pub current_run_status_sequence_visible: bool,
    pub ordered_run_event_history_materialized: bool,
    pub run_start_authorization_ready_now: bool,
    pub runtime_readiness_satisfied: bool,
    pub activation_readiness_satisfied: bool,
    pub configuration_readiness_satisfied: bool,
    pub startup_janitor_behavior_implemented: bool,
    pub startup_checks_complete: bool,
    pub ready_idle_reached: bool,
    pub app_shutting_down_active: bool,
    pub startup_fatal_error_active: bool,
}

impl AppLifecycleSnapshot {
    pub fn summary(self) -> String {
        format!(
            "App lifecycle truth surface is materialized. Stage is {:?}; canonical state surface materialized is {}; lifecycle execution materialized is {}; current state is {}; start state is {}; launch state is {}; startup janitor state is {}; startup checks state is {}; runtime hydration states are {} and {}; activation state is {}; activation revoked state is {}; configuration state is {}; ready state is {}; shutting down state is {}; startup fatal state is {}; readiness domains separated is {}; activation gating surface materialized is {}; activation-to-revoked transition defined is {}; activation-to-configuration transition defined is {}; activation-to-ready transition defined is {}; Preflight readiness evaluated separately is {}; current Preflight state is {}; current Preflight run-start eligible is {}; run start blocked without eligible Preflight is {}; ready-idle requires Preflight before run start is {}; run-start readiness evaluated separately is {}; run-start foundation state is {}; run-start foundation blocked is {}; run-start execution materialized is {}; runtime persona foundation materialized is {}; bounded-memory foundation materialized is {}; transcript-archive separation materialized is {}; moderated-round execution materialized is {}; token-and-duration tracking materialized is {}; Phase 7 run-lifecycle surface materialized is {}; Phase 7 run-status contract materialized is {}; Phase 7 run-event contract materialized is {}; Phase 9 run-cleanup contract materialized is {}; Phase 9 cleanup-scope mapping materialized is {}; Phase 9 startup-janitor contract materialized is {}; Phase 8 run-finalization contract materialized is {}; Phase 8 report-output contract materialized is {}; Phase 8 report-storage boundary materialized is {}; Phase 8 report-access contract materialized is {}; cleanup outcome distinct from run outcome is {}; startup janitor outcome distinct from run outcome is {}; preserved report cleanup exception visible is {}; current report availability state is {}; current report available is {}; completion state implies report availability is {}; cleanup state implies report deletion is {}; updater contract surface materialized is {}; current updater state is {}; updater discovery configuration ready is {}; updater state separate from lifecycle truth is {}; updater discovery not required for ready idle is {}; updater apply deferred during active run is {}; current run lifecycle state is {}; current run identity visible is {}; current run cancelability known is {}; current run cancelable is {}; current run status sequence visible is {}; ordered run-event history materialized is {}; run-start authorization ready now is {}; runtime readiness satisfied is {}; activation readiness satisfied is {}; configuration readiness satisfied is {}; startup janitor behavior implemented is {}; startup checks complete is {}; ready idle reached is {}; app shutting down active is {}; startup fatal error active is {}.",
            self.stage,
            self.canonical_state_surface_materialized,
            self.lifecycle_execution_materialized,
            self.current_state_name,
            self.start_state_name,
            self.launch_state_name,
            self.startup_janitor_state_name,
            self.startup_checks_state_name,
            self.runtime_hydration_required_state_name,
            self.runtime_hydrating_state_name,
            self.activation_required_state_name,
            self.activation_revoked_state_name,
            self.configuration_required_state_name,
            self.ready_idle_state_name,
            self.shutting_down_state_name,
            self.startup_fatal_error_state_name,
            self.readiness_domains_separated,
            self.activation_gating_surface_materialized,
            self.activation_to_revoked_transition_defined,
            self.activation_to_configuration_transition_defined,
            self.activation_to_ready_transition_defined,
            self.preflight_readiness_evaluated_separately,
            self.preflight_state_name,
            self.preflight_run_start_eligible,
            self.run_start_blocked_without_preflight_eligibility,
            self.ready_idle_requires_preflight_before_run_start,
            self.run_start_readiness_evaluated_separately,
            self.run_start_foundation_state_name,
            self.run_start_foundation_blocked,
            self.run_start_execution_materialized,
            self.runtime_persona_foundation_materialized,
            self.bounded_memory_foundation_materialized,
            self.transcript_archive_separation_materialized,
            self.moderated_round_execution_materialized,
            self.token_duration_tracking_materialized,
            self.phase_seven_run_lifecycle_surface_materialized,
            self.phase_seven_run_status_contract_materialized,
            self.phase_seven_run_event_contract_materialized,
            self.phase_nine_run_cleanup_contract_materialized,
            self.phase_nine_cleanup_scope_mapping_materialized,
            self.phase_nine_startup_janitor_contract_materialized,
            self.phase_eight_run_finalization_contract_materialized,
            self.phase_eight_report_output_contract_materialized,
            self.phase_eight_report_storage_boundary_materialized,
            self.phase_eight_report_access_contract_materialized,
            self.cleanup_outcome_distinct_from_run_outcome,
            self.startup_janitor_outcome_distinct_from_run_outcome,
            self.preserved_report_cleanup_exception_visible,
            self.current_report_availability_state_name,
            self.current_report_available,
            self.completion_state_implies_report_availability,
            self.cleanup_state_implies_report_deletion,
            self.updater_contract_surface_materialized,
            self.updater_current_state_name,
            self.updater_discovery_configuration_ready,
            self.updater_state_separate_from_lifecycle_truth,
            self.updater_discovery_not_required_for_ready_idle,
            self.updater_apply_deferred_during_active_run,
            self.current_run_lifecycle_state_name,
            self.current_run_identity_visible,
            self.current_run_cancelability_known,
            self.current_run_cancelable,
            self.current_run_status_sequence_visible,
            self.ordered_run_event_history_materialized,
            self.run_start_authorization_ready_now,
            self.runtime_readiness_satisfied,
            self.activation_readiness_satisfied,
            self.configuration_readiness_satisfied,
            self.startup_janitor_behavior_implemented,
            self.startup_checks_complete,
            self.ready_idle_reached,
            self.app_shutting_down_active,
            self.startup_fatal_error_active
        )
    }
}

pub fn app_lifecycle_truth_surface() -> AppLifecycleSnapshot {
    let activation_readiness = activation_readiness_truth_surface();
    let provider_readiness = provider_readiness_truth_surface();
    let preflight_readiness = preflight_readiness_truth_surface();
    let run_start_readiness = run_start_readiness_truth_surface();
    let runtime_persona_contract = runtime_persona_contract_surface();
    let bounded_memory = run_bounded_memory_surface();
    let moderated_rounds = run_moderated_rounds_surface();
    let token_duration_tracking = run_token_duration_tracking_surface();
    let run_lifecycle_state = run_lifecycle_state_from_shell();
    let run_status_contract = run_status_contract_state();
    let run_cleanup_contract = run_cleanup_contract_state();
    let cleanup_scope_mapping = cleanup_scope_mapping_state();
    let startup_janitor_contract = startup_janitor_contract_state();
    let run_event_contract = run_event_contract_state();
    let run_finalization_contract = run_finalization_contract_state();
    let report_output_contract = report_output_contract_state();
    let report_storage_boundary = report_storage_boundary_state();
    let report_access_contract = report_access_contract_state();
    let runtime_hydration = runtime_hydration_state();
    let updater_contract = updater_contract_state();
    let (shutdown_requested, graceful_shutdown_started, forceful_termination_required, forceful_termination_executed) =
        shell_shutdown_coordination_flags();
    let runtime_ready = activation_readiness.runtime_ready_now;
    let activation_ready = activation_readiness.activation_ready_for_licensed_use;
    let current_state_name = if startup_janitor_contract.janitor_running_now {
        STATE_STARTUP_JANITOR_RUNNING
    } else if shutdown_requested
        || graceful_shutdown_started
        || forceful_termination_required
        || forceful_termination_executed
    {
        STATE_APP_SHUTTING_DOWN
    } else {
        resolve_lifecycle_state(
            runtime_ready,
            runtime_hydration.hydration_active_now,
            activation_readiness.current_validation_status,
            provider_readiness.provider_configuration_ready,
        )
    };

    AppLifecycleSnapshot {
        stage: AppLifecycleStage::RunStartFoundationMaterialized,
        canonical_state_surface_materialized: true,
        lifecycle_execution_materialized: false,
        start_state_name: STATE_APP_NOT_RUNNING,
        launch_state_name: STATE_APP_LAUNCHING,
        startup_janitor_state_name: STATE_STARTUP_JANITOR_RUNNING,
        startup_checks_state_name: STATE_STARTUP_CHECKS_RUNNING,
        runtime_hydration_required_state_name: STATE_RUNTIME_HYDRATION_REQUIRED,
        runtime_hydrating_state_name: STATE_RUNTIME_HYDRATING,
        activation_required_state_name: STATE_ACTIVATION_REQUIRED,
        activation_revoked_state_name: STATE_ACTIVATION_REVOKED,
        configuration_required_state_name: STATE_CONFIGURATION_REQUIRED,
        ready_idle_state_name: STATE_READY_IDLE,
        shutting_down_state_name: STATE_APP_SHUTTING_DOWN,
        startup_fatal_error_state_name: STATE_STARTUP_FATAL_ERROR,
        current_state_name,
        readiness_domains_separated: true,
        activation_gating_surface_materialized: true,
        activation_to_revoked_transition_defined: true,
        activation_to_configuration_transition_defined: true,
        activation_to_ready_transition_defined: true,
        preflight_readiness_evaluated_separately: true,
        preflight_state_name: preflight_readiness.current_state_name,
        preflight_run_start_eligible: preflight_readiness.run_start_eligible_now,
        run_start_blocked_without_preflight_eligibility: preflight_readiness.run_start_blocked_without_eligible_preflight,
        ready_idle_requires_preflight_before_run_start: true,
        run_start_readiness_evaluated_separately: true,
        run_start_foundation_state_name: run_start_readiness.current_state_name,
        run_start_foundation_blocked: run_start_readiness.current_state_name
            == crate::run_start_readiness::STATE_RUN_START_FOUNDATION_BLOCKED,
        run_start_execution_materialized: run_start_readiness.run_start_execution_materialized,
        runtime_persona_foundation_materialized: runtime_persona_contract
            .runtime_persona_generation_materialized,
        bounded_memory_foundation_materialized: bounded_memory.bounded_memory_foundation_materialized,
        transcript_archive_separation_materialized: bounded_memory
            .transcript_archive_foundation_materialized,
        moderated_round_execution_materialized: moderated_rounds
            .moderated_round_execution_materialized,
        token_duration_tracking_materialized: token_duration_tracking
            .token_duration_tracking_materialized,
        phase_seven_run_lifecycle_surface_materialized: run_lifecycle_state
            .canonical_state_family_frozen,
        phase_seven_run_status_contract_materialized: run_status_contract
            .current_status_retrieval_implemented,
        phase_seven_run_event_contract_materialized: run_event_contract.ordered_retrieval_explicit,
        phase_nine_run_cleanup_contract_materialized: run_cleanup_contract.shell_authority_required,
        phase_nine_cleanup_scope_mapping_materialized: cleanup_scope_mapping
            .explicit_scope_mapping_materialized,
        phase_nine_startup_janitor_contract_materialized: startup_janitor_contract
            .shell_authority_required,
        phase_eight_run_finalization_contract_materialized: run_finalization_contract
            .shell_authority_required,
        phase_eight_report_output_contract_materialized: report_output_contract
            .stable_output_structure_defined,
        phase_eight_report_storage_boundary_materialized: report_storage_boundary
            .durable_export_artifacts_classified,
        phase_eight_report_access_contract_materialized: report_access_contract
            .response_contract
            .availability_representation_supported,
        cleanup_outcome_distinct_from_run_outcome: run_cleanup_contract
            .cleanup_outcome_distinct_from_run_outcome,
        startup_janitor_outcome_distinct_from_run_outcome: startup_janitor_contract
            .janitor_outcome_distinct_from_run_outcome,
        preserved_report_cleanup_exception_visible: cleanup_scope_mapping
            .preserved_report_exception_explicit,
        current_report_availability_state_name: report_access_contract.current_availability_status_name,
        current_report_available: report_access_contract.report_available_now,
        completion_state_implies_report_availability: report_access_contract
            .completion_classification_implies_report_available,
        cleanup_state_implies_report_deletion: run_cleanup_contract.cleanup_outcome_implies_report_deletion,
        updater_contract_surface_materialized: updater_contract.shell_authority_required
            && updater_contract.frontend_presentation_only,
        updater_current_state_name: updater_contract.current_state_name,
        updater_discovery_configuration_ready: updater_contract.discovery_configuration_ready,
        updater_state_separate_from_lifecycle_truth: updater_contract
            .updater_state_separate_from_run_lifecycle,
        updater_discovery_not_required_for_ready_idle: updater_contract
            .updater_state_separate_from_run_lifecycle,
        updater_apply_deferred_during_active_run: updater_contract.apply_deferred_during_active_run,
        current_run_lifecycle_state_name: run_lifecycle_state.current_state_name,
        current_run_identity_visible: run_status_contract.current_run_id.is_some()
            && run_status_contract.current_session_id.is_some(),
        current_run_cancelability_known: run_status_contract.cancelability_flags_included,
        current_run_cancelable: run_status_contract.current_state_cancelable,
        current_run_status_sequence_visible: run_status_contract
            .latest_visible_sequence_number
            .is_some(),
        ordered_run_event_history_materialized: run_event_contract.event_history_materialized,
        run_start_authorization_ready_now: run_start_readiness.real_run_start_authorized_now,
        runtime_readiness_satisfied: runtime_ready,
        activation_readiness_satisfied: activation_ready,
        configuration_readiness_satisfied: provider_readiness.provider_configuration_ready,
        startup_janitor_behavior_implemented: startup_janitor_contract
            .real_startup_janitor_execution_materialized,
        startup_checks_complete: runtime_ready
            && activation_ready
            && provider_readiness.readiness_surface_materialized,
        ready_idle_reached: current_state_name == STATE_READY_IDLE,
        app_shutting_down_active: current_state_name == STATE_APP_SHUTTING_DOWN,
        startup_fatal_error_active: false,
    }
}

pub fn resolve_lifecycle_state(
    runtime_ready: bool,
    runtime_hydration_active: bool,
    activation_validation_status: ActivationValidationStatus,
    configuration_ready: bool,
) -> &'static str {
    if !runtime_ready {
        if runtime_hydration_active {
            return STATE_RUNTIME_HYDRATING;
        }

        return STATE_RUNTIME_HYDRATION_REQUIRED;
    }

    match activation_validation_status {
        ActivationValidationStatus::ActivationRevoked => STATE_ACTIVATION_REVOKED,
        ActivationValidationStatus::ActivatedValid => {
            if configuration_ready {
                STATE_READY_IDLE
            } else {
                STATE_CONFIGURATION_REQUIRED
            }
        }
        _ => STATE_ACTIVATION_REQUIRED,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_lifecycle_truth_surface_stays_in_startup_checks_pending_readiness() {
        let snapshot = app_lifecycle_truth_surface();

        assert!(snapshot.canonical_state_surface_materialized);
        assert!(!snapshot.lifecycle_execution_materialized);
        assert_eq!(snapshot.current_state_name, STATE_RUNTIME_HYDRATION_REQUIRED);
        assert_eq!(snapshot.activation_revoked_state_name, STATE_ACTIVATION_REVOKED);
        assert_eq!(snapshot.ready_idle_state_name, STATE_READY_IDLE);
        assert!(snapshot.readiness_domains_separated);
        assert!(snapshot.activation_gating_surface_materialized);
        assert!(snapshot.activation_to_revoked_transition_defined);
        assert!(snapshot.activation_to_configuration_transition_defined);
        assert!(snapshot.activation_to_ready_transition_defined);
        assert!(snapshot.preflight_readiness_evaluated_separately);
        assert_eq!(snapshot.preflight_state_name, "PreflightNotStarted");
        assert!(!snapshot.preflight_run_start_eligible);
        assert!(snapshot.run_start_blocked_without_preflight_eligibility);
        assert!(snapshot.ready_idle_requires_preflight_before_run_start);
        assert!(snapshot.run_start_readiness_evaluated_separately);
        assert_eq!(
            snapshot.run_start_foundation_state_name,
            crate::run_start_readiness::STATE_RUN_START_FOUNDATION_NOT_STARTED
        );
        assert!(!snapshot.run_start_foundation_blocked);
        assert!(snapshot.run_start_execution_materialized);
        assert!(snapshot.runtime_persona_foundation_materialized);
        assert!(snapshot.bounded_memory_foundation_materialized);
        assert!(snapshot.transcript_archive_separation_materialized);
        assert!(snapshot.moderated_round_execution_materialized);
        assert!(snapshot.token_duration_tracking_materialized);
        assert!(snapshot.phase_seven_run_lifecycle_surface_materialized);
        assert!(snapshot.phase_seven_run_status_contract_materialized);
        assert!(snapshot.phase_seven_run_event_contract_materialized);
        assert!(snapshot.phase_nine_run_cleanup_contract_materialized);
        assert!(snapshot.phase_nine_cleanup_scope_mapping_materialized);
        assert!(snapshot.phase_nine_startup_janitor_contract_materialized);
        assert!(snapshot.phase_eight_run_finalization_contract_materialized);
        assert!(snapshot.phase_eight_report_output_contract_materialized);
        assert!(snapshot.phase_eight_report_storage_boundary_materialized);
        assert!(snapshot.phase_eight_report_access_contract_materialized);
        assert!(snapshot.cleanup_outcome_distinct_from_run_outcome);
        assert!(snapshot.startup_janitor_outcome_distinct_from_run_outcome);
        assert!(snapshot.preserved_report_cleanup_exception_visible);
        assert_eq!(snapshot.current_report_availability_state_name, "ReportUnavailable");
        assert!(!snapshot.current_report_available);
        assert!(!snapshot.completion_state_implies_report_availability);
        assert!(!snapshot.cleanup_state_implies_report_deletion);
        assert!(snapshot.updater_contract_surface_materialized);
        assert_eq!(snapshot.updater_current_state_name, "UpdaterIdle");
        assert!(snapshot.updater_state_separate_from_lifecycle_truth);
        assert!(snapshot.updater_discovery_not_required_for_ready_idle);
        assert!(snapshot.updater_apply_deferred_during_active_run);
        assert_eq!(snapshot.current_run_lifecycle_state_name, "RunDraft");
        assert!(snapshot.current_run_cancelability_known);
        assert!(!snapshot.current_run_cancelable);
        assert!(!snapshot.current_run_status_sequence_visible);
        assert!(snapshot.ordered_run_event_history_materialized);
        assert!(!snapshot.run_start_authorization_ready_now);
        assert!(!snapshot.runtime_readiness_satisfied);
        assert!(!snapshot.activation_readiness_satisfied);
        assert!(!snapshot.configuration_readiness_satisfied);
        assert!(snapshot.startup_janitor_behavior_implemented);
        assert!(!snapshot.startup_checks_complete);
        assert!(!snapshot.ready_idle_reached);
        assert!(!snapshot.app_shutting_down_active);
        assert!(!snapshot.startup_fatal_error_active);
    }

    #[test]
    fn resolve_lifecycle_state_distinguishes_activation_required_revoked_and_ready_paths() {
        assert_eq!(
            resolve_lifecycle_state(false, false, ActivationValidationStatus::ActivatedValid, true),
            STATE_RUNTIME_HYDRATION_REQUIRED
        );
        assert_eq!(
            resolve_lifecycle_state(false, true, ActivationValidationStatus::ActivatedValid, true),
            STATE_RUNTIME_HYDRATING
        );
        assert_eq!(
            resolve_lifecycle_state(true, false, ActivationValidationStatus::ActivationRequired, false),
            STATE_ACTIVATION_REQUIRED
        );
        assert_eq!(
            resolve_lifecycle_state(true, false, ActivationValidationStatus::ActivationRevoked, false),
            STATE_ACTIVATION_REVOKED
        );
        assert_eq!(
            resolve_lifecycle_state(true, false, ActivationValidationStatus::ActivatedValid, false),
            STATE_CONFIGURATION_REQUIRED
        );
        assert_eq!(
            resolve_lifecycle_state(true, false, ActivationValidationStatus::ActivatedValid, true),
            STATE_READY_IDLE
        );
    }
}