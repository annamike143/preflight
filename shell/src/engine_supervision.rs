use crate::run_session_start::{shell_engine_process_active, shell_shutdown_coordination_flags};

#[derive(Debug, Clone, Copy)]
pub enum EngineSupervisionStage {
    ModeratedExecutionMaterialized,
}

#[derive(Debug, Clone, Copy)]
pub struct EngineSupervisionSnapshot {
    pub stage: EngineSupervisionStage,
    pub shell_authoritative_owner: bool,
    pub local_interface_authentication_required: bool,
    pub local_interface_authentication_implemented: bool,
    pub local_interface_contract_bounded: bool,
    pub run_session_authorization_shell_only: bool,
    pub workspace_allocation_authority_shell_only: bool,
    pub preflight_lineage_required_before_engine_start: bool,
    pub start_foundation_surface_materialized: bool,
    pub run_start_command_implemented: bool,
    pub execution_seed_staging_materialized: bool,
    pub one_shot_engine_foundation_bootstrap_implemented: bool,
    pub one_shot_moderated_execution_implemented: bool,
    pub runtime_persona_generation_materialized: bool,
    pub moderator_basis_materialized: bool,
    pub bounded_memory_foundation_materialized: bool,
    pub transcript_archive_separation_materialized: bool,
    pub speaker_attribution_materialized: bool,
    pub run_local_token_duration_tracking_materialized: bool,
    pub canonical_run_lifecycle_projection_materialized: bool,
    pub run_status_contract_materialized: bool,
    pub run_cleanup_contract_materialized: bool,
    pub cleanup_scope_mapping_materialized: bool,
    pub startup_janitor_contract_materialized: bool,
    pub run_event_contract_materialized: bool,
    pub cancellation_contract_materialized: bool,
    pub engine_process_spawned: bool,
    pub supervision_active: bool,
    pub restart_supported: bool,
    pub shutdown_coordination_ready: bool,
    pub health_monitoring_active: bool,
    pub ordered_event_history_materialized: bool,
    pub in_flight_snapshot_publication_materialized: bool,
    pub pending_history_truth_supported_during_one_shot_execution: bool,
    pub controlled_cancellation_behavior_materialized: bool,
    pub shutdown_to_cleanup_execution_materialized: bool,
    pub anti_zombie_supervision_required_if_cleanup_deferred: bool,
}

impl EngineSupervisionSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Engine supervision surface is materialized. Stage is {:?}; shell authoritative owner is {}; local interface authentication required is {}; local interface authentication implemented is {}; local interface contract bounded is {}; run/session authorization shell-only is {}; workspace-allocation authority shell-only is {}; Preflight-lineage required before engine start is {}; start-foundation surface materialized is {}; run-start command implemented is {}; execution-seed staging materialized is {}; one-shot engine foundation bootstrap implemented is {}; one-shot moderated execution implemented is {}; runtime persona generation materialized is {}; Moderator basis materialized is {}; bounded-memory foundation materialized is {}; transcript-archive separation materialized is {}; speaker attribution materialized is {}; run-local token-and-duration tracking materialized is {}; canonical run-lifecycle projection materialized is {}; run-status contract materialized is {}; run-cleanup contract materialized is {}; cleanup-scope mapping materialized is {}; startup-janitor contract materialized is {}; run-event contract materialized is {}; cancellation contract materialized is {}; engine process spawned is {}; supervision active is {}; restart supported is {}; shutdown coordination ready is {}; health monitoring active is {}; ordered event history materialized is {}; in-flight snapshot publication materialized is {}; pending history truth supported during one-shot execution is {}; controlled cancellation behavior materialized is {}; shutdown-to-cleanup execution materialized is {}; anti-zombie supervision required if cleanup deferred is {}.",
            self.stage,
            self.shell_authoritative_owner,
            self.local_interface_authentication_required,
            self.local_interface_authentication_implemented,
            self.local_interface_contract_bounded,
            self.run_session_authorization_shell_only,
            self.workspace_allocation_authority_shell_only,
            self.preflight_lineage_required_before_engine_start,
            self.start_foundation_surface_materialized,
            self.run_start_command_implemented,
            self.execution_seed_staging_materialized,
            self.one_shot_engine_foundation_bootstrap_implemented,
            self.one_shot_moderated_execution_implemented,
            self.runtime_persona_generation_materialized,
            self.moderator_basis_materialized,
            self.bounded_memory_foundation_materialized,
            self.transcript_archive_separation_materialized,
            self.speaker_attribution_materialized,
            self.run_local_token_duration_tracking_materialized,
            self.canonical_run_lifecycle_projection_materialized,
            self.run_status_contract_materialized,
            self.run_cleanup_contract_materialized,
            self.cleanup_scope_mapping_materialized,
            self.startup_janitor_contract_materialized,
            self.run_event_contract_materialized,
            self.cancellation_contract_materialized,
            self.engine_process_spawned,
            self.supervision_active,
            self.restart_supported,
            self.shutdown_coordination_ready,
            self.health_monitoring_active,
            self.ordered_event_history_materialized,
            self.in_flight_snapshot_publication_materialized,
            self.pending_history_truth_supported_during_one_shot_execution,
            self.controlled_cancellation_behavior_materialized,
            self.shutdown_to_cleanup_execution_materialized,
            self.anti_zombie_supervision_required_if_cleanup_deferred,
        )
    }
}

pub fn engine_supervision_scaffold() -> EngineSupervisionSnapshot {
    let (shutdown_requested, _, _, _) = shell_shutdown_coordination_flags();
    let engine_process_active = shell_engine_process_active();

    EngineSupervisionSnapshot {
        stage: EngineSupervisionStage::ModeratedExecutionMaterialized,
        shell_authoritative_owner: true,
        local_interface_authentication_required: true,
        local_interface_authentication_implemented: false,
        local_interface_contract_bounded: true,
        run_session_authorization_shell_only: true,
        workspace_allocation_authority_shell_only: true,
        preflight_lineage_required_before_engine_start: true,
        start_foundation_surface_materialized: true,
        run_start_command_implemented: true,
        execution_seed_staging_materialized: true,
        one_shot_engine_foundation_bootstrap_implemented: true,
        one_shot_moderated_execution_implemented: true,
        runtime_persona_generation_materialized: true,
        moderator_basis_materialized: true,
        bounded_memory_foundation_materialized: true,
        transcript_archive_separation_materialized: true,
        speaker_attribution_materialized: true,
        run_local_token_duration_tracking_materialized: true,
        canonical_run_lifecycle_projection_materialized: true,
        run_status_contract_materialized: true,
        run_cleanup_contract_materialized: true,
        cleanup_scope_mapping_materialized: true,
        startup_janitor_contract_materialized: true,
        run_event_contract_materialized: true,
        cancellation_contract_materialized: true,
        engine_process_spawned: engine_process_active,
        supervision_active: engine_process_active,
        restart_supported: false,
        shutdown_coordination_ready: true,
        health_monitoring_active: engine_process_active || shutdown_requested,
        ordered_event_history_materialized: true,
        in_flight_snapshot_publication_materialized: true,
        pending_history_truth_supported_during_one_shot_execution: true,
        controlled_cancellation_behavior_materialized: true,
        shutdown_to_cleanup_execution_materialized: true,
        anti_zombie_supervision_required_if_cleanup_deferred: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_supervision_scaffold_tracks_shutdown_cleanup_coordination() {
        let snapshot = engine_supervision_scaffold();

        assert!(snapshot.shell_authoritative_owner);
        assert!(snapshot.local_interface_authentication_required);
        assert!(!snapshot.local_interface_authentication_implemented);
        assert!(snapshot.local_interface_contract_bounded);
        assert!(snapshot.run_session_authorization_shell_only);
        assert!(snapshot.workspace_allocation_authority_shell_only);
        assert!(snapshot.preflight_lineage_required_before_engine_start);
        assert!(snapshot.start_foundation_surface_materialized);
        assert!(snapshot.run_start_command_implemented);
        assert!(snapshot.execution_seed_staging_materialized);
        assert!(snapshot.one_shot_engine_foundation_bootstrap_implemented);
        assert!(snapshot.one_shot_moderated_execution_implemented);
        assert!(snapshot.runtime_persona_generation_materialized);
        assert!(snapshot.moderator_basis_materialized);
        assert!(snapshot.bounded_memory_foundation_materialized);
        assert!(snapshot.transcript_archive_separation_materialized);
        assert!(snapshot.speaker_attribution_materialized);
        assert!(snapshot.run_local_token_duration_tracking_materialized);
        assert!(snapshot.canonical_run_lifecycle_projection_materialized);
        assert!(snapshot.run_status_contract_materialized);
        assert!(snapshot.run_cleanup_contract_materialized);
        assert!(snapshot.cleanup_scope_mapping_materialized);
        assert!(snapshot.startup_janitor_contract_materialized);
        assert!(snapshot.run_event_contract_materialized);
        assert!(snapshot.cancellation_contract_materialized);
        assert!(!snapshot.engine_process_spawned);
        assert!(!snapshot.supervision_active);
        assert!(!snapshot.restart_supported);
        assert!(snapshot.shutdown_coordination_ready);
        assert!(!snapshot.health_monitoring_active);
        assert!(snapshot.ordered_event_history_materialized);
        assert!(snapshot.in_flight_snapshot_publication_materialized);
        assert!(snapshot.pending_history_truth_supported_during_one_shot_execution);
        assert!(snapshot.controlled_cancellation_behavior_materialized);
        assert!(snapshot.shutdown_to_cleanup_execution_materialized);
        assert!(snapshot.anti_zombie_supervision_required_if_cleanup_deferred);
    }
}