pub const RUN_WORKSPACE_BOUNDARY_NAME: &str = "RunWorkspaceBoundary";
pub const RUN_WORKSPACE_DOMAIN_NAME: &str = "PerRunWorkspace";
pub const RUN_SUPPORT_METADATA_DOMAIN_NAME: &str = "RunSupportMetadata";
pub const RUN_TRANSCRIPT_ARCHIVE_NAME: &str = "RunTranscriptArchive";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunWorkspaceBoundaryStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunSupportMetadataContract {
    pub run_session_identifier_required: bool,
    pub workspace_reference_required: bool,
    pub coarse_run_state_supported: bool,
    pub terminal_reason_supported: bool,
    pub preflight_lineage_fingerprint_required: bool,
    pub bounded_usage_counters_allowed: bool,
    pub raw_seed_archive_allowed: bool,
    pub raw_provider_secret_allowed: bool,
    pub raw_activation_token_allowed: bool,
    pub transcript_replay_payload_allowed: bool,
    pub crash_resume_continuation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunWorkspaceBoundarySnapshot {
    pub stage: RunWorkspaceBoundaryStage,
    pub boundary_name: &'static str,
    pub workspace_domain_name: &'static str,
    pub support_metadata_domain_name: &'static str,
    pub transcript_archive_name: &'static str,
    pub fresh_workspace_required: bool,
    pub no_active_workspace_reuse: bool,
    pub shell_owned_workspace_allocation: bool,
    pub workspace_allocation_implemented: bool,
    pub workspace_cleanup_implemented: bool,
    pub hidden_continuation_allowed: bool,
    pub crash_resume_supported: bool,
    pub workspace_reuse_as_active_execution_allowed: bool,
    pub cleanup_scope_mapping_materialized: bool,
    pub startup_janitor_scope_alignment_defined: bool,
    pub durable_report_artifacts_outside_workspace_cleanup_scope: bool,
    pub durable_app_state_outside_workspace_cleanup_scope: bool,
    pub support_metadata_local_only: bool,
    pub support_metadata_persistence_implemented: bool,
    pub transcript_archive_distinct_from_prompt_memory: bool,
    pub transcript_archive_unbounded_prompt_replay_allowed: bool,
    pub support_metadata_contract: RunSupportMetadataContract,
}

impl RunWorkspaceBoundarySnapshot {
    pub fn summary(self) -> String {
        format!(
            "Run workspace-boundary surface is materialized. Stage is {:?}; boundary name is {}; workspace domain name is {}; support-metadata domain name is {}; transcript archive name is {}; fresh workspace required is {}; no active workspace reuse is {}; shell-owned workspace allocation is {}; workspace allocation implemented is {}; workspace cleanup implemented is {}; hidden continuation allowed is {}; crash-resume supported is {}; workspace reuse as active execution allowed is {}; cleanup-scope mapping materialized is {}; startup-janitor scope alignment defined is {}; durable report artifacts outside workspace cleanup scope is {}; durable app state outside workspace cleanup scope is {}; support metadata local only is {}; support metadata persistence implemented is {}; transcript archive distinct from prompt memory is {}; transcript archive unbounded prompt replay allowed is {}; support metadata requires run/session identifier is {}; support metadata requires workspace reference is {}; support metadata supports coarse run state is {}; support metadata supports terminal reason is {}; support metadata requires Preflight-lineage fingerprint is {}; support metadata allows bounded usage counters is {}; support metadata allows raw seed archive is {}; support metadata allows raw provider secret is {}; support metadata allows raw activation token is {}; support metadata allows transcript replay payload is {}; support metadata allows crash-resume continuation is {}.",
            self.stage,
            self.boundary_name,
            self.workspace_domain_name,
            self.support_metadata_domain_name,
            self.transcript_archive_name,
            self.fresh_workspace_required,
            self.no_active_workspace_reuse,
            self.shell_owned_workspace_allocation,
            self.workspace_allocation_implemented,
            self.workspace_cleanup_implemented,
            self.hidden_continuation_allowed,
            self.crash_resume_supported,
            self.workspace_reuse_as_active_execution_allowed,
            self.cleanup_scope_mapping_materialized,
            self.startup_janitor_scope_alignment_defined,
            self.durable_report_artifacts_outside_workspace_cleanup_scope,
            self.durable_app_state_outside_workspace_cleanup_scope,
            self.support_metadata_local_only,
            self.support_metadata_persistence_implemented,
            self.transcript_archive_distinct_from_prompt_memory,
            self.transcript_archive_unbounded_prompt_replay_allowed,
            self.support_metadata_contract.run_session_identifier_required,
            self.support_metadata_contract.workspace_reference_required,
            self.support_metadata_contract.coarse_run_state_supported,
            self.support_metadata_contract.terminal_reason_supported,
            self.support_metadata_contract.preflight_lineage_fingerprint_required,
            self.support_metadata_contract.bounded_usage_counters_allowed,
            self.support_metadata_contract.raw_seed_archive_allowed,
            self.support_metadata_contract.raw_provider_secret_allowed,
            self.support_metadata_contract.raw_activation_token_allowed,
            self.support_metadata_contract.transcript_replay_payload_allowed,
            self.support_metadata_contract.crash_resume_continuation_allowed,
        )
    }
}

pub fn run_workspace_boundary_surface() -> RunWorkspaceBoundarySnapshot {
    RunWorkspaceBoundarySnapshot {
        stage: RunWorkspaceBoundaryStage::FoundationMaterialized,
        boundary_name: RUN_WORKSPACE_BOUNDARY_NAME,
        workspace_domain_name: RUN_WORKSPACE_DOMAIN_NAME,
        support_metadata_domain_name: RUN_SUPPORT_METADATA_DOMAIN_NAME,
        transcript_archive_name: RUN_TRANSCRIPT_ARCHIVE_NAME,
        fresh_workspace_required: true,
        no_active_workspace_reuse: true,
        shell_owned_workspace_allocation: true,
        workspace_allocation_implemented: true,
        workspace_cleanup_implemented: true,
        hidden_continuation_allowed: false,
        crash_resume_supported: false,
        workspace_reuse_as_active_execution_allowed: false,
        cleanup_scope_mapping_materialized: true,
        startup_janitor_scope_alignment_defined: true,
        durable_report_artifacts_outside_workspace_cleanup_scope: true,
        durable_app_state_outside_workspace_cleanup_scope: true,
        support_metadata_local_only: true,
        support_metadata_persistence_implemented: true,
        transcript_archive_distinct_from_prompt_memory: true,
        transcript_archive_unbounded_prompt_replay_allowed: false,
        support_metadata_contract: RunSupportMetadataContract {
            run_session_identifier_required: true,
            workspace_reference_required: true,
            coarse_run_state_supported: true,
            terminal_reason_supported: true,
            preflight_lineage_fingerprint_required: true,
            bounded_usage_counters_allowed: true,
            raw_seed_archive_allowed: false,
            raw_provider_secret_allowed: false,
            raw_activation_token_allowed: false,
            transcript_replay_payload_allowed: false,
            crash_resume_continuation_allowed: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_workspace_boundary_surface_keeps_run_workspaces_isolated_and_non_reusable() {
        let snapshot = run_workspace_boundary_surface();

        assert_eq!(snapshot.boundary_name, RUN_WORKSPACE_BOUNDARY_NAME);
        assert_eq!(snapshot.workspace_domain_name, RUN_WORKSPACE_DOMAIN_NAME);
        assert_eq!(snapshot.support_metadata_domain_name, RUN_SUPPORT_METADATA_DOMAIN_NAME);
        assert_eq!(snapshot.transcript_archive_name, RUN_TRANSCRIPT_ARCHIVE_NAME);
        assert!(snapshot.fresh_workspace_required);
        assert!(snapshot.no_active_workspace_reuse);
        assert!(snapshot.shell_owned_workspace_allocation);
        assert!(snapshot.workspace_allocation_implemented);
        assert!(snapshot.workspace_cleanup_implemented);
        assert!(!snapshot.hidden_continuation_allowed);
        assert!(!snapshot.crash_resume_supported);
        assert!(!snapshot.workspace_reuse_as_active_execution_allowed);
        assert!(snapshot.cleanup_scope_mapping_materialized);
        assert!(snapshot.startup_janitor_scope_alignment_defined);
        assert!(snapshot.durable_report_artifacts_outside_workspace_cleanup_scope);
        assert!(snapshot.durable_app_state_outside_workspace_cleanup_scope);
        assert!(snapshot.support_metadata_local_only);
        assert!(snapshot.support_metadata_persistence_implemented);
        assert!(snapshot.transcript_archive_distinct_from_prompt_memory);
        assert!(!snapshot.transcript_archive_unbounded_prompt_replay_allowed);
        assert!(snapshot.support_metadata_contract.run_session_identifier_required);
        assert!(snapshot.support_metadata_contract.workspace_reference_required);
        assert!(snapshot.support_metadata_contract.coarse_run_state_supported);
        assert!(snapshot.support_metadata_contract.terminal_reason_supported);
        assert!(snapshot.support_metadata_contract.preflight_lineage_fingerprint_required);
        assert!(snapshot.support_metadata_contract.bounded_usage_counters_allowed);
        assert!(!snapshot.support_metadata_contract.raw_seed_archive_allowed);
        assert!(!snapshot.support_metadata_contract.raw_provider_secret_allowed);
        assert!(!snapshot.support_metadata_contract.raw_activation_token_allowed);
        assert!(!snapshot.support_metadata_contract.transcript_replay_payload_allowed);
        assert!(!snapshot.support_metadata_contract.crash_resume_continuation_allowed);
    }
}