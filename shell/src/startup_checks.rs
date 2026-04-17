use crate::activation_contract::{activation_contract_basis, ActivationContractSnapshot};
use crate::activation_local_validation::{
    activation_local_validation_surface, LocalActivationValidationSnapshot,
};
use crate::activation_secure_store::{
    activation_secure_store_surface, ActivationSecureStoreSnapshot,
};
use crate::activation_readiness::{activation_readiness_truth_surface, ActivationReadinessSnapshot};
use crate::activation_storage_boundary::{
    activation_storage_boundary_surface, ActivationStorageBoundarySnapshot,
};
use crate::activation_support_store::{
    activation_support_store_surface, ActivationSupportStoreSnapshot,
};
use crate::app_session_trust_boundary::{
    app_session_trust_boundary_scaffold, AppSessionTrustBoundarySnapshot,
};
use crate::app_lifecycle::{app_lifecycle_truth_surface, AppLifecycleSnapshot};
use crate::cleanup_scope_mapping::{cleanup_scope_mapping_state, CleanupScopeMappingSnapshot};
use crate::controlled_shutdown::{controlled_shutdown_scaffold, ControlledShutdownSnapshot};
use crate::engine_supervision::{engine_supervision_scaffold, EngineSupervisionSnapshot};
use crate::local_interface_contract::{
    local_interface_contract_scaffold, LocalInterfaceContractSnapshot,
};
use crate::preflight_contract::{preflight_contract_basis, PreflightContractSnapshot};
use crate::preflight_readiness::{preflight_readiness_truth_surface, PreflightReadinessSnapshot};
use crate::provider_configuration_contract::{
    provider_configuration_contract_basis, ProviderConfigurationContractSnapshot,
};
use crate::provider_readiness::{provider_readiness_truth_surface, ProviderReadinessSnapshot};
use crate::report_access_contract::{report_access_contract_state, ReportAccessContractSnapshot};
use crate::report_output_contract::{report_output_contract_state, ReportOutputContractSnapshot};
use crate::report_storage_boundary::{
    report_storage_boundary_state, ReportStorageBoundarySnapshot,
};
use crate::run_bounded_memory::{run_bounded_memory_surface, RunBoundedMemorySnapshot};
use crate::run_cleanup_contract::{run_cleanup_contract_state, RunCleanupContractSnapshot};
use crate::run_event_contract::{run_event_contract_state, RunEventContractSnapshot};
use crate::run_finalization_contract::{
    run_finalization_contract_state, RunFinalizationContractSnapshot,
};
use crate::run_lifecycle_state::{run_lifecycle_state_from_shell, RunLifecycleStateSnapshot};
use crate::run_moderated_rounds::{run_moderated_rounds_surface, RunModeratedRoundsSnapshot};
use crate::provider_storage_boundary::{
    provider_storage_boundary_surface, ProviderStorageBoundarySnapshot,
};
use crate::run_session_contract::{run_session_contract_basis, RunSessionContractSnapshot};
use crate::run_status_contract::{run_status_contract_state, RunStatusContractSnapshot};
use crate::run_start_lineage::{run_start_lineage_surface, RunStartLineageSnapshot};
use crate::run_start_readiness::{run_start_readiness_truth_surface, RunStartReadinessSnapshot};
use crate::run_token_duration_tracking::{
    run_token_duration_tracking_surface, RunTokenDurationTrackingSnapshot,
};
use crate::run_workspace_boundary::{run_workspace_boundary_surface, RunWorkspaceBoundarySnapshot};
use crate::runtime_persona_contract::{
    runtime_persona_contract_surface, RuntimePersonaContractSnapshot,
};
use crate::runtime_package_contract::{
    runtime_package_contract_basis, RuntimePackageContractSnapshot,
};
use crate::runtime_readiness::{runtime_readiness_truth_surface, RuntimeReadinessSnapshot};
use crate::runtime_hydration::{runtime_hydration_state, RuntimeHydrationSnapshot};
use crate::runtime_integrity::{runtime_integrity_state, RuntimeIntegritySnapshot};
use crate::runtime_presence::{runtime_presence_state, RuntimePresenceSnapshot};
use crate::runtime_delivery_contract::{
    runtime_delivery_contract_state, RuntimeDeliveryContractSnapshot,
};
use crate::startup_janitor_contract::{
    startup_janitor_contract_state, StartupJanitorContractSnapshot,
};
use crate::updater_contract::{updater_contract_state, UpdaterContractSnapshot};

#[derive(Debug, Clone, Copy)]
pub enum StartupCheckStage {
    BoundedMaterialized,
}

#[derive(Debug, Clone)]
pub struct StartupCheckSnapshot {
    pub stage: StartupCheckStage,
    pub activation_contract: ActivationContractSnapshot,
    pub activation_secure_store: ActivationSecureStoreSnapshot,
    pub activation_support_store: ActivationSupportStoreSnapshot,
    pub activation_local_validation: LocalActivationValidationSnapshot,
    pub activation_storage_boundary: ActivationStorageBoundarySnapshot,
    pub activation_readiness: ActivationReadinessSnapshot,
    pub preflight_contract: PreflightContractSnapshot,
    pub preflight_readiness: PreflightReadinessSnapshot,
    pub provider_configuration_contract: ProviderConfigurationContractSnapshot,
    pub provider_storage_boundary: ProviderStorageBoundarySnapshot,
    pub provider_readiness: ProviderReadinessSnapshot,
    pub run_session_contract: RunSessionContractSnapshot,
    pub run_workspace_boundary: RunWorkspaceBoundarySnapshot,
    pub run_start_lineage: RunStartLineageSnapshot,
    pub run_start_readiness: RunStartReadinessSnapshot,
    pub run_lifecycle_state: RunLifecycleStateSnapshot,
    pub run_status_contract: RunStatusContractSnapshot,
    pub run_cleanup_contract: RunCleanupContractSnapshot,
    pub cleanup_scope_mapping: CleanupScopeMappingSnapshot,
    pub startup_janitor_contract: StartupJanitorContractSnapshot,
    pub run_event_contract: RunEventContractSnapshot,
    pub run_finalization_contract: RunFinalizationContractSnapshot,
    pub report_output_contract: ReportOutputContractSnapshot,
    pub report_storage_boundary: ReportStorageBoundarySnapshot,
    pub report_access_contract: ReportAccessContractSnapshot,
    pub runtime_persona_contract: RuntimePersonaContractSnapshot,
    pub run_bounded_memory: RunBoundedMemorySnapshot,
    pub run_moderated_rounds: RunModeratedRoundsSnapshot,
    pub run_token_duration_tracking: RunTokenDurationTrackingSnapshot,
    pub runtime_presence: RuntimePresenceSnapshot,
    pub runtime_hydration: RuntimeHydrationSnapshot,
    pub runtime_integrity: RuntimeIntegritySnapshot,
    pub app_session_trust_boundary: AppSessionTrustBoundarySnapshot,
    pub app_lifecycle: AppLifecycleSnapshot,
    pub engine_supervision: EngineSupervisionSnapshot,
    pub runtime_package_contract: RuntimePackageContractSnapshot,
    pub updater_contract: UpdaterContractSnapshot,
    pub runtime_delivery_contract: RuntimeDeliveryContractSnapshot,
    pub runtime_readiness: RuntimeReadinessSnapshot,
    pub local_interface_contract: LocalInterfaceContractSnapshot,
    pub controlled_shutdown: ControlledShutdownSnapshot,
    pub runtime_readiness_established: bool,
}

impl StartupCheckSnapshot {
    pub fn summary(self) -> String {
        let activation_contract = self.activation_contract.summary();
        let activation_secure_store = self.activation_secure_store.summary();
        let activation_support_store = self.activation_support_store.summary();
        let activation_local_validation = self.activation_local_validation.summary();
        let activation_storage_boundary = self.activation_storage_boundary.summary();
        let activation_readiness = self.activation_readiness.summary();
        let preflight_contract = self.preflight_contract.summary();
        let preflight_readiness = self.preflight_readiness.summary();
        let provider_configuration_contract = self.provider_configuration_contract.summary();
        let provider_storage_boundary = self.provider_storage_boundary.summary();
        let provider_readiness = self.provider_readiness.summary();
        let run_session_contract = self.run_session_contract.summary();
        let run_workspace_boundary = self.run_workspace_boundary.summary();
        let run_start_lineage = self.run_start_lineage.summary();
        let run_start_readiness = self.run_start_readiness.summary();
        let run_lifecycle_state = self.run_lifecycle_state.summary();
        let run_status_contract = self.run_status_contract.summary();
        let run_cleanup_contract = self.run_cleanup_contract.summary();
        let cleanup_scope_mapping = self.cleanup_scope_mapping.summary();
        let startup_janitor_contract = self.startup_janitor_contract.summary();
        let run_event_contract = self.run_event_contract.summary();
        let run_finalization_contract = self.run_finalization_contract.summary();
        let report_output_contract = self.report_output_contract.summary();
        let report_storage_boundary = self.report_storage_boundary.summary();
        let report_access_contract = self.report_access_contract.summary();
        let runtime_persona_contract = self.runtime_persona_contract.summary();
        let run_bounded_memory = self.run_bounded_memory.summary();
        let run_moderated_rounds = self.run_moderated_rounds.summary();
        let run_token_duration_tracking = self.run_token_duration_tracking.summary();
        let runtime_presence = self.runtime_presence.summary();
        let runtime_hydration = self.runtime_hydration.summary();
        let runtime_integrity = self.runtime_integrity.summary();
        let app_session_trust_boundary = self.app_session_trust_boundary.summary();
        let app_lifecycle = self.app_lifecycle.summary();
        let engine_supervision = self.engine_supervision.summary();
        let runtime_package_contract = self.runtime_package_contract.summary();
        let updater_contract = self.updater_contract.summary();
        let runtime_delivery_contract = self.runtime_delivery_contract.summary();
        let runtime_readiness = self.runtime_readiness.summary();
        let local_interface_contract = self.local_interface_contract.summary();
        let controlled_shutdown = self.controlled_shutdown.summary();
        let surfaces = [
            activation_contract,
            activation_secure_store,
            activation_support_store,
            activation_local_validation,
            activation_storage_boundary,
            activation_readiness,
            preflight_contract,
            preflight_readiness,
            provider_configuration_contract,
            provider_storage_boundary,
            provider_readiness,
            run_session_contract,
            run_workspace_boundary,
            run_start_lineage,
            run_start_readiness,
            run_lifecycle_state,
            run_status_contract,
            run_cleanup_contract,
            cleanup_scope_mapping,
            startup_janitor_contract,
            run_event_contract,
            run_finalization_contract,
            report_output_contract,
            report_storage_boundary,
            report_access_contract,
            runtime_persona_contract,
            run_bounded_memory,
            run_moderated_rounds,
            run_token_duration_tracking,
            runtime_presence,
            runtime_hydration,
            runtime_integrity,
            app_session_trust_boundary,
            app_lifecycle,
            engine_supervision,
            runtime_package_contract,
            updater_contract,
            runtime_delivery_contract,
            runtime_readiness,
            local_interface_contract,
            controlled_shutdown,
        ]
        .join(" ");

        format!(
            "Shell startup-check surface is materialized. Startup-check stage is {:?}; runtime readiness established is {}; {}",
            self.stage,
            self.runtime_readiness_established,
            surfaces
        )
    }
}

pub fn startup_checks_scaffold() -> StartupCheckSnapshot {
    let runtime_readiness = runtime_readiness_truth_surface();

    StartupCheckSnapshot {
        stage: StartupCheckStage::BoundedMaterialized,
        activation_contract: activation_contract_basis(),
        activation_secure_store: activation_secure_store_surface(),
        activation_support_store: activation_support_store_surface(),
        activation_local_validation: activation_local_validation_surface(),
        activation_storage_boundary: activation_storage_boundary_surface(),
        activation_readiness: activation_readiness_truth_surface(),
        preflight_contract: preflight_contract_basis(),
        preflight_readiness: preflight_readiness_truth_surface(),
        provider_configuration_contract: provider_configuration_contract_basis(),
        provider_storage_boundary: provider_storage_boundary_surface(),
        provider_readiness: provider_readiness_truth_surface(),
        run_session_contract: run_session_contract_basis(),
        run_workspace_boundary: run_workspace_boundary_surface(),
        run_start_lineage: run_start_lineage_surface(),
        run_start_readiness: run_start_readiness_truth_surface(),
        run_lifecycle_state: run_lifecycle_state_from_shell(),
        run_status_contract: run_status_contract_state(),
        run_cleanup_contract: run_cleanup_contract_state(),
        cleanup_scope_mapping: cleanup_scope_mapping_state(),
        startup_janitor_contract: startup_janitor_contract_state(),
        run_event_contract: run_event_contract_state(),
        run_finalization_contract: run_finalization_contract_state(),
        report_output_contract: report_output_contract_state(),
        report_storage_boundary: report_storage_boundary_state(),
        report_access_contract: report_access_contract_state(),
        runtime_persona_contract: runtime_persona_contract_surface(),
        run_bounded_memory: run_bounded_memory_surface(),
        run_moderated_rounds: run_moderated_rounds_surface(),
        run_token_duration_tracking: run_token_duration_tracking_surface(),
        runtime_presence: runtime_presence_state(),
        runtime_hydration: runtime_hydration_state(),
        runtime_integrity: runtime_integrity_state(),
        app_session_trust_boundary: app_session_trust_boundary_scaffold(),
        app_lifecycle: app_lifecycle_truth_surface(),
        engine_supervision: engine_supervision_scaffold(),
        runtime_package_contract: runtime_package_contract_basis(),
        updater_contract: updater_contract_state(),
        runtime_delivery_contract: runtime_delivery_contract_state(),
        runtime_readiness,
        local_interface_contract: local_interface_contract_scaffold(),
        controlled_shutdown: controlled_shutdown_scaffold(),
        runtime_readiness_established: runtime_readiness.runtime_ready,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_checks_scaffold_keeps_runtime_readiness_blocked() {
        let snapshot = startup_checks_scaffold();

        assert!(!snapshot.runtime_readiness_established);
        assert!(snapshot.activation_contract.response_contract.approval_supported);
        assert!(snapshot.activation_secure_store.secure_store_commit_implemented);
        assert!(snapshot.activation_support_store.persistence_implemented);
        assert!(snapshot.activation_storage_boundary.raw_token_material_secure_store_only);
        assert_eq!(
            snapshot.activation_local_validation.validation_status,
            crate::activation_storage_boundary::ActivationValidationStatus::ActivationRequired
        );
        assert!(!snapshot.activation_readiness.activation_ready_for_licensed_use);
        assert!(snapshot.preflight_contract.response_contract.approved_supported);
        assert!(snapshot.preflight_contract.response_contract.approved_with_downgrade_supported);
        assert!(snapshot.preflight_contract.response_contract.rejected_supported);
        assert!(snapshot.preflight_contract.response_contract.technical_failure_distinct_from_rejection);
        assert_eq!(snapshot.preflight_readiness.current_state_name, "PreflightNotStarted");
        assert!(!snapshot.preflight_readiness.run_start_eligible_now);
        assert_eq!(snapshot.provider_configuration_contract.supported_providers().len(), 2);
        assert!(snapshot.provider_storage_boundary.raw_provider_secret_secure_store_only);
        assert!(snapshot.provider_storage_boundary.secure_store_commit_implemented);
        assert_eq!(
            snapshot.provider_readiness.current_status,
            crate::provider_configuration_contract::ProviderConfigurationStatus::Required
        );
        assert_eq!(
            snapshot.provider_readiness.current_detail,
            crate::provider_readiness::ProviderReadinessDetail::SelectionMissing
        );
        assert!(!snapshot.provider_readiness.provider_configuration_ready);
        assert_eq!(snapshot.run_session_contract.supported_state_classes().len(), 6);
        assert!(snapshot.run_workspace_boundary.fresh_workspace_required);
        assert!(snapshot.run_workspace_boundary.no_active_workspace_reuse);
        assert!(snapshot.run_start_lineage.current_eligible_preflight_required);
        assert!(!snapshot.run_start_lineage.raw_seed_bytes_embedded);
        assert!(snapshot.run_start_readiness.state_surface_materialized);
        assert!(snapshot.run_start_readiness.run_start_execution_materialized);
        assert!(snapshot.run_lifecycle_state.canonical_state_family_frozen);
        assert_eq!(snapshot.run_lifecycle_state.current_state_name, "RunDraft");
        assert!(snapshot.run_status_contract.current_status_retrieval_implemented);
        assert!(snapshot.run_cleanup_contract.cleanup_attempt_distinct_from_cleanup_outcome);
        assert!(snapshot.cleanup_scope_mapping.preserved_report_exception_explicit);
        assert!(snapshot.startup_janitor_contract.abnormal_termination_detection_required);
        assert!(snapshot
            .startup_janitor_contract
            .real_startup_janitor_execution_materialized);
        assert!(snapshot
            .startup_janitor_contract
            .real_abnormal_termination_detection_materialized);
        assert!(snapshot
            .run_status_contract
            .cancelability_flags_included);
        assert!(snapshot.run_status_contract.ordered_event_history_materialized);
        assert!(snapshot.run_event_contract.ordered_retrieval_explicit);
        assert!(snapshot.run_event_contract.event_history_materialized);
        assert!(snapshot.run_finalization_contract.finalization_entered_distinct);
        assert!(snapshot.run_finalization_contract.real_report_generation_materialized);
        assert!(snapshot.run_finalization_contract.real_pdf_generation_materialized);
        assert_eq!(snapshot.report_output_contract.required_sections.len(), 7);
        assert_eq!(snapshot.report_output_contract.output_classes.len(), 6);
        assert!(snapshot.report_output_contract.pdf_contract.successful_generation_implied);
        assert!(snapshot.report_storage_boundary.durable_export_artifacts_classified);
        assert!(snapshot.report_storage_boundary.durable_export_write_materialized);
        assert_eq!(
            snapshot.report_access_contract.current_availability_status_name,
            "ReportUnavailable"
        );
        assert!(!snapshot.report_access_contract.report_available_now);
        assert!(snapshot.report_access_contract.response_contract.local_open_execution_materialized);
        assert!(snapshot.report_access_contract.response_contract.local_export_execution_materialized);
        assert!(snapshot.runtime_persona_contract.runtime_persona_generation_materialized);
        assert!(snapshot.runtime_persona_contract.moderator_basis_materialized);
        assert!(snapshot.run_bounded_memory.bounded_memory_foundation_materialized);
        assert!(snapshot.run_bounded_memory.transcript_archive_foundation_materialized);
        assert!(snapshot.run_moderated_rounds.moderated_round_execution_materialized);
        assert!(snapshot.run_token_duration_tracking.token_duration_tracking_materialized);
        assert!(!snapshot.run_start_readiness.real_run_start_authorized_now);
        assert!(!snapshot.runtime_hydration.hydration_active_now);
        assert!(!snapshot.runtime_integrity.runtime_integrity_passed);
        assert!(!snapshot.app_session_trust_boundary.local_app_session_mechanism_materialized);
        assert_eq!(snapshot.app_lifecycle.current_state_name, "RuntimeHydrationRequired");
        assert_eq!(snapshot.app_lifecycle.preflight_state_name, "PreflightNotStarted");
        assert!(snapshot.engine_supervision.run_start_command_implemented);
        assert!(snapshot.engine_supervision.execution_seed_staging_materialized);
        assert!(!snapshot.engine_supervision.engine_process_spawned);
        assert!(!snapshot.engine_supervision.supervision_active);
        assert!(snapshot.runtime_package_contract.shell_presence_basis_defined);
        assert!(snapshot.runtime_package_contract.shell_integrity_basis_defined);
        assert_eq!(snapshot.updater_contract.current_state_name, "UpdaterIdle");
        assert_eq!(snapshot.app_lifecycle.updater_current_state_name, "UpdaterIdle");
        assert!(snapshot
            .app_lifecycle
            .updater_state_separate_from_lifecycle_truth);
        assert!(snapshot
            .app_lifecycle
            .updater_discovery_not_required_for_ready_idle);
        assert!(snapshot.updater_contract.available_state_distinct_from_ready);
        assert!(snapshot.updater_contract.rejection_distinct_from_error);
        assert_eq!(snapshot.runtime_readiness.updater_current_state_name, "UpdaterIdle");
        assert!(snapshot
            .runtime_readiness
            .updater_discovery_not_required_for_runtime_ready);
        assert_eq!(snapshot.run_status_contract.current_updater_state_name, "UpdaterIdle");
        assert!(snapshot
            .run_status_contract
            .updater_status_visible_separately_from_run_lifecycle);
        assert!(!snapshot.run_status_contract.updater_coordination_visible_during_active_run);
        assert!(!snapshot.updater_contract.real_update_check_materialized);
        assert!(snapshot.runtime_delivery_contract.authorization_basis_defined);
        assert!(snapshot.runtime_delivery_contract.compatibility_basis_defined);
        assert!(snapshot.runtime_delivery_contract.runtime_use_blocked_until_verified);
        assert!(!snapshot.runtime_delivery_contract.real_artifact_acquisition_materialized);
        assert!(!snapshot.runtime_readiness.runtime_ready);
        assert!(snapshot.runtime_readiness.runtime_not_ready);
        assert!(snapshot.runtime_readiness.runtime_presence_determined);
        assert!(!snapshot.local_interface_contract.shell_to_engine_authentication_implemented);
        assert!(snapshot.local_interface_contract.frontend_to_shell_run_cancellation_command_implemented);
        assert!(snapshot
            .local_interface_contract
            .frontend_to_shell_run_finalization_contract_command_implemented);
        assert!(snapshot
            .local_interface_contract
            .frontend_to_shell_report_access_contract_command_implemented);
        assert!(snapshot
            .local_interface_contract
            .frontend_to_shell_report_access_metadata_command_implemented);
        assert!(snapshot
            .local_interface_contract
            .frontend_to_shell_report_open_command_implemented);
        assert!(snapshot
            .local_interface_contract
            .frontend_to_shell_report_export_command_implemented);
        assert!(snapshot.engine_supervision.shutdown_coordination_ready);
        assert!(snapshot.engine_supervision.controlled_cancellation_behavior_materialized);
        assert!(snapshot.controlled_shutdown.shutdown_coordination_ready);
    }
}
