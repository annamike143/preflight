use serde::{Deserialize, Serialize};

use crate::report_storage_boundary::{
    DURABLE_REPORT_ARTIFACT_DOMAIN_NAME, REPORT_METADATA_INDEX_DOMAIN_NAME,
};
use crate::run_workspace_boundary::{RUN_SUPPORT_METADATA_DOMAIN_NAME, RUN_WORKSPACE_DOMAIN_NAME};

pub const CLEANUP_SCOPE_MAPPING_NAME: &str = "CleanupScopeMapping";
pub const CLEANUP_SCOPE_MAPPING_COMMAND_NAME: &str = "CleanupScopeMappingState";
pub const DURABLE_APP_LOCAL_STATE_DOMAIN_NAME: &str = "DurableAppLocalState";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupScopeMappingStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct CleanupScopeEntry {
    pub data_class_name: &'static str,
    pub storage_domain_name: &'static str,
    pub cleanup_scope_applies: bool,
    pub startup_janitor_scope_applies: bool,
    pub shell_owned_boundary: bool,
    pub preserved_report_artifact: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CleanupScopeMappingSnapshot {
    pub stage: CleanupScopeMappingStage,
    pub mapping_name: &'static str,
    pub shell_command_name: &'static str,
    pub cleanup_entries: [CleanupScopeEntry; 5],
    pub explicit_scope_mapping_materialized: bool,
    pub shell_authority_required: bool,
    pub terminal_cleanup_scope_defined: bool,
    pub startup_janitor_scope_defined: bool,
    pub preserved_report_exception_explicit: bool,
    pub durable_support_state_exclusion_explicit: bool,
    pub crash_resume_continuation_allowed: bool,
    pub report_reclassification_as_workspace_garbage_allowed: bool,
}

impl CleanupScopeMappingSnapshot {
    pub fn summary(self) -> String {
        let cleanup_entries = self
            .cleanup_entries
            .map(|entry| {
                format!(
                    "{} in {} (cleanup applies: {}, startup janitor applies: {}, shell-owned boundary: {}, preserved report artifact: {})",
                    entry.data_class_name,
                    entry.storage_domain_name,
                    entry.cleanup_scope_applies,
                    entry.startup_janitor_scope_applies,
                    entry.shell_owned_boundary,
                    entry.preserved_report_artifact,
                )
            })
            .join("; ");

        format!(
            "Cleanup scope-mapping surface is materialized. Stage is {:?}; mapping name is {}; shell command name is {}; cleanup entries are {}; explicit scope mapping materialized is {}; shell authority required is {}; terminal cleanup scope defined is {}; startup janitor scope defined is {}; preserved report exception explicit is {}; durable support-state exclusion explicit is {}; crash-resume continuation allowed is {}; report reclassification as workspace garbage allowed is {}.",
            self.stage,
            self.mapping_name,
            self.shell_command_name,
            cleanup_entries,
            self.explicit_scope_mapping_materialized,
            self.shell_authority_required,
            self.terminal_cleanup_scope_defined,
            self.startup_janitor_scope_defined,
            self.preserved_report_exception_explicit,
            self.durable_support_state_exclusion_explicit,
            self.crash_resume_continuation_allowed,
            self.report_reclassification_as_workspace_garbage_allowed,
        )
    }
}

pub fn cleanup_scope_mapping_state() -> CleanupScopeMappingSnapshot {
    CleanupScopeMappingSnapshot {
        stage: CleanupScopeMappingStage::FoundationMaterialized,
        mapping_name: CLEANUP_SCOPE_MAPPING_NAME,
        shell_command_name: CLEANUP_SCOPE_MAPPING_COMMAND_NAME,
        cleanup_entries: [
            CleanupScopeEntry {
                data_class_name: "TransientRunWorkspaceData",
                storage_domain_name: RUN_WORKSPACE_DOMAIN_NAME,
                cleanup_scope_applies: true,
                startup_janitor_scope_applies: true,
                shell_owned_boundary: true,
                preserved_report_artifact: false,
            },
            CleanupScopeEntry {
                data_class_name: "RunSupportMetadata",
                storage_domain_name: RUN_SUPPORT_METADATA_DOMAIN_NAME,
                cleanup_scope_applies: true,
                startup_janitor_scope_applies: true,
                shell_owned_boundary: true,
                preserved_report_artifact: false,
            },
            CleanupScopeEntry {
                data_class_name: "DurableReportArtifact",
                storage_domain_name: DURABLE_REPORT_ARTIFACT_DOMAIN_NAME,
                cleanup_scope_applies: false,
                startup_janitor_scope_applies: false,
                shell_owned_boundary: true,
                preserved_report_artifact: true,
            },
            CleanupScopeEntry {
                data_class_name: "ReportMetadataIndexReference",
                storage_domain_name: REPORT_METADATA_INDEX_DOMAIN_NAME,
                cleanup_scope_applies: false,
                startup_janitor_scope_applies: false,
                shell_owned_boundary: true,
                preserved_report_artifact: false,
            },
            CleanupScopeEntry {
                data_class_name: "DurableAppLocalState",
                storage_domain_name: DURABLE_APP_LOCAL_STATE_DOMAIN_NAME,
                cleanup_scope_applies: false,
                startup_janitor_scope_applies: false,
                shell_owned_boundary: true,
                preserved_report_artifact: false,
            },
        ],
        explicit_scope_mapping_materialized: true,
        shell_authority_required: true,
        terminal_cleanup_scope_defined: true,
        startup_janitor_scope_defined: true,
        preserved_report_exception_explicit: true,
        durable_support_state_exclusion_explicit: true,
        crash_resume_continuation_allowed: false,
        report_reclassification_as_workspace_garbage_allowed: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleanup_scope_mapping_preserves_reports_and_durable_app_state() {
        let snapshot = cleanup_scope_mapping_state();

        assert_eq!(snapshot.mapping_name, CLEANUP_SCOPE_MAPPING_NAME);
        assert_eq!(snapshot.shell_command_name, CLEANUP_SCOPE_MAPPING_COMMAND_NAME);
        assert_eq!(snapshot.cleanup_entries.len(), 5);
        assert!(snapshot.explicit_scope_mapping_materialized);
        assert!(snapshot.shell_authority_required);
        assert!(snapshot.terminal_cleanup_scope_defined);
        assert!(snapshot.startup_janitor_scope_defined);
        assert!(snapshot.preserved_report_exception_explicit);
        assert!(snapshot.durable_support_state_exclusion_explicit);
        assert!(!snapshot.crash_resume_continuation_allowed);
        assert!(!snapshot.report_reclassification_as_workspace_garbage_allowed);
        assert!(snapshot.cleanup_entries[0].cleanup_scope_applies);
        assert!(snapshot.cleanup_entries[0].startup_janitor_scope_applies);
        assert!(snapshot.cleanup_entries[2].preserved_report_artifact);
        assert!(!snapshot.cleanup_entries[2].cleanup_scope_applies);
        assert!(!snapshot.cleanup_entries[4].cleanup_scope_applies);
    }
}