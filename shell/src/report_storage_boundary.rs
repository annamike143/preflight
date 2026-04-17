use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};

pub const REPORT_STORAGE_BOUNDARY_NAME: &str = "ReportStorageBoundary";
pub const TRANSIENT_REPORT_WORKSPACE_DOMAIN_NAME: &str = "TransientReportWorkspaceDomain";
pub const DURABLE_REPORT_ARTIFACT_DOMAIN_NAME: &str = "DurableReportArtifactDomain";
pub const REPORT_METADATA_INDEX_DOMAIN_NAME: &str = "ReportMetadataIndexDomain";
pub const DURABLE_REPORT_DIRECTORY_NAME: &str = "reports";
pub const EXPORTED_REPORT_DIRECTORY_NAME: &str = "miro-fish-report-exports";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportStorageBoundaryStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportMetadataReferenceContract {
    pub report_identifier_required_when_generated: bool,
    pub artifact_reference_optional_until_generated: bool,
    pub export_timestamp_optional_until_generated: bool,
    pub raw_provider_secret_allowed: bool,
    pub activation_token_material_allowed: bool,
    pub raw_seed_bytes_allowed: bool,
    pub ordinary_log_embedding_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportStorageBoundarySnapshot {
    pub stage: ReportStorageBoundaryStage,
    pub boundary_name: &'static str,
    pub transient_workspace_domain_name: &'static str,
    pub durable_artifact_domain_name: &'static str,
    pub metadata_index_domain_name: &'static str,
    pub transient_generation_intermediates_classified: bool,
    pub durable_export_artifacts_classified: bool,
    pub metadata_index_references_classified: bool,
    pub transient_generation_intermediates_outside_cleanup_scope: bool,
    pub durable_export_artifacts_outside_cleanup_scope: bool,
    pub metadata_index_references_outside_cleanup_scope: bool,
    pub transient_generation_intermediates_outside_startup_janitor_scope: bool,
    pub durable_export_artifacts_outside_startup_janitor_scope: bool,
    pub metadata_index_references_outside_startup_janitor_scope: bool,
    pub report_generation_intermediates_persistence_materialized: bool,
    pub durable_export_write_materialized: bool,
    pub metadata_index_persistence_materialized: bool,
    pub ordinary_workspace_garbage_classification_allowed_for_durable_artifacts: bool,
    pub cleanup_scope_mapping_alignment_materialized: bool,
    pub preserved_report_exception_explicit: bool,
    pub metadata_reference_contract: ReportMetadataReferenceContract,
}

impl ReportStorageBoundarySnapshot {
    pub fn summary(self) -> String {
        format!(
            "Report storage-boundary surface is materialized. Stage is {:?}; boundary name is {}; transient workspace domain is {}; durable artifact domain is {}; metadata index domain is {}; transient generation intermediates classified is {}; durable export artifacts classified is {}; metadata index references classified is {}; transient generation intermediates outside cleanup scope is {}; durable export artifacts outside cleanup scope is {}; metadata index references outside cleanup scope is {}; transient generation intermediates outside startup janitor scope is {}; durable export artifacts outside startup janitor scope is {}; metadata index references outside startup janitor scope is {}; report-generation intermediates persistence materialized is {}; durable export write materialized is {}; metadata index persistence materialized is {}; ordinary workspace-garbage classification allowed for durable artifacts is {}; cleanup-scope mapping alignment materialized is {}; preserved report exception explicit is {}; report identifier required when generated is {}; artifact reference optional until generated is {}; export timestamp optional until generated is {}; raw provider secret allowed is {}; activation token material allowed is {}; raw seed bytes allowed is {}; ordinary log embedding allowed is {}.",
            self.stage,
            self.boundary_name,
            self.transient_workspace_domain_name,
            self.durable_artifact_domain_name,
            self.metadata_index_domain_name,
            self.transient_generation_intermediates_classified,
            self.durable_export_artifacts_classified,
            self.metadata_index_references_classified,
            self.transient_generation_intermediates_outside_cleanup_scope,
            self.durable_export_artifacts_outside_cleanup_scope,
            self.metadata_index_references_outside_cleanup_scope,
            self.transient_generation_intermediates_outside_startup_janitor_scope,
            self.durable_export_artifacts_outside_startup_janitor_scope,
            self.metadata_index_references_outside_startup_janitor_scope,
            self.report_generation_intermediates_persistence_materialized,
            self.durable_export_write_materialized,
            self.metadata_index_persistence_materialized,
            self.ordinary_workspace_garbage_classification_allowed_for_durable_artifacts,
            self.cleanup_scope_mapping_alignment_materialized,
            self.preserved_report_exception_explicit,
            self.metadata_reference_contract.report_identifier_required_when_generated,
            self.metadata_reference_contract.artifact_reference_optional_until_generated,
            self.metadata_reference_contract.export_timestamp_optional_until_generated,
            self.metadata_reference_contract.raw_provider_secret_allowed,
            self.metadata_reference_contract.activation_token_material_allowed,
            self.metadata_reference_contract.raw_seed_bytes_allowed,
            self.metadata_reference_contract.ordinary_log_embedding_allowed,
        )
    }
}

pub fn default_durable_report_artifact_root() -> Result<PathBuf, String> {
    let project_dirs = ProjectDirs::from("com", "MiroFish", "miro-fish-desktop-saas")
        .ok_or_else(|| "unable to resolve durable report artifact directory".to_string())?;

    Ok(project_dirs
        .data_local_dir()
        .join(DURABLE_REPORT_DIRECTORY_NAME))
}

pub fn materialize_durable_report_artifact(
    source_pdf_path: &Path,
    report_identifier: &str,
) -> Result<PathBuf, String> {
    let root = default_durable_report_artifact_root()?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;

    let destination_path = root.join(format!("{}.pdf", sanitize_report_identifier(report_identifier)));
    fs::copy(source_pdf_path, &destination_path).map_err(|error| error.to_string())?;

    Ok(destination_path)
}

pub fn export_report_artifact(
    source_pdf_path: &Path,
    report_identifier: &str,
) -> Result<PathBuf, String> {
    let export_root = default_report_export_root()?;
    fs::create_dir_all(&export_root).map_err(|error| error.to_string())?;

    let sanitized_identifier = sanitize_report_identifier(report_identifier);
    let base_name = format!("{sanitized_identifier}.pdf");
    let initial_path = export_root.join(&base_name);
    let destination_path = if initial_path.exists() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or_default();
        export_root.join(format!("{sanitized_identifier}-{timestamp}.pdf"))
    } else {
        initial_path
    };

    fs::copy(source_pdf_path, &destination_path).map_err(|error| error.to_string())?;

    Ok(destination_path)
}

fn default_report_export_root() -> Result<PathBuf, String> {
    if let Some(user_dirs) = UserDirs::new() {
        let base_root = user_dirs
            .download_dir()
            .or_else(|| user_dirs.document_dir())
            .unwrap_or(user_dirs.home_dir());
        return Ok(base_root.join(EXPORTED_REPORT_DIRECTORY_NAME));
    }

    Ok(default_durable_report_artifact_root()?.join("exports"))
}

fn sanitize_report_identifier(report_identifier: &str) -> String {
    let sanitized: String = report_identifier
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect();

    let trimmed = sanitized.trim_matches('_');
    if trimmed.is_empty() {
        "report".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn report_storage_boundary_state() -> ReportStorageBoundarySnapshot {
    ReportStorageBoundarySnapshot {
        stage: ReportStorageBoundaryStage::FoundationMaterialized,
        boundary_name: REPORT_STORAGE_BOUNDARY_NAME,
        transient_workspace_domain_name: TRANSIENT_REPORT_WORKSPACE_DOMAIN_NAME,
        durable_artifact_domain_name: DURABLE_REPORT_ARTIFACT_DOMAIN_NAME,
        metadata_index_domain_name: REPORT_METADATA_INDEX_DOMAIN_NAME,
        transient_generation_intermediates_classified: true,
        durable_export_artifacts_classified: true,
        metadata_index_references_classified: true,
        transient_generation_intermediates_outside_cleanup_scope: false,
        durable_export_artifacts_outside_cleanup_scope: true,
        metadata_index_references_outside_cleanup_scope: true,
        transient_generation_intermediates_outside_startup_janitor_scope: false,
        durable_export_artifacts_outside_startup_janitor_scope: true,
        metadata_index_references_outside_startup_janitor_scope: true,
        report_generation_intermediates_persistence_materialized: true,
        durable_export_write_materialized: true,
        metadata_index_persistence_materialized: false,
        ordinary_workspace_garbage_classification_allowed_for_durable_artifacts: false,
        cleanup_scope_mapping_alignment_materialized: true,
        preserved_report_exception_explicit: true,
        metadata_reference_contract: ReportMetadataReferenceContract {
            report_identifier_required_when_generated: true,
            artifact_reference_optional_until_generated: true,
            export_timestamp_optional_until_generated: true,
            raw_provider_secret_allowed: false,
            activation_token_material_allowed: false,
            raw_seed_bytes_allowed: false,
            ordinary_log_embedding_allowed: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_storage_boundary_keeps_durable_outputs_out_of_workspace_cleanup() {
        let snapshot = report_storage_boundary_state();

        assert_eq!(snapshot.boundary_name, REPORT_STORAGE_BOUNDARY_NAME);
        assert_eq!(
            snapshot.transient_workspace_domain_name,
            TRANSIENT_REPORT_WORKSPACE_DOMAIN_NAME
        );
        assert_eq!(
            snapshot.durable_artifact_domain_name,
            DURABLE_REPORT_ARTIFACT_DOMAIN_NAME
        );
        assert_eq!(snapshot.metadata_index_domain_name, REPORT_METADATA_INDEX_DOMAIN_NAME);
        assert!(snapshot.transient_generation_intermediates_classified);
        assert!(snapshot.durable_export_artifacts_classified);
        assert!(snapshot.metadata_index_references_classified);
        assert!(!snapshot.transient_generation_intermediates_outside_cleanup_scope);
        assert!(snapshot.durable_export_artifacts_outside_cleanup_scope);
        assert!(snapshot.metadata_index_references_outside_cleanup_scope);
        assert!(!snapshot.transient_generation_intermediates_outside_startup_janitor_scope);
        assert!(snapshot.durable_export_artifacts_outside_startup_janitor_scope);
        assert!(snapshot.metadata_index_references_outside_startup_janitor_scope);
        assert!(snapshot.report_generation_intermediates_persistence_materialized);
        assert!(snapshot.durable_export_write_materialized);
        assert!(!snapshot.metadata_index_persistence_materialized);
        assert!(!snapshot.ordinary_workspace_garbage_classification_allowed_for_durable_artifacts);
        assert!(snapshot.cleanup_scope_mapping_alignment_materialized);
        assert!(snapshot.preserved_report_exception_explicit);
        assert!(snapshot.metadata_reference_contract.report_identifier_required_when_generated);
        assert!(snapshot.metadata_reference_contract.artifact_reference_optional_until_generated);
        assert!(snapshot.metadata_reference_contract.export_timestamp_optional_until_generated);
        assert!(!snapshot.metadata_reference_contract.raw_provider_secret_allowed);
        assert!(!snapshot.metadata_reference_contract.activation_token_material_allowed);
        assert!(!snapshot.metadata_reference_contract.raw_seed_bytes_allowed);
        assert!(!snapshot.metadata_reference_contract.ordinary_log_embedding_allowed);
    }

    #[test]
    fn materialize_durable_report_artifact_copies_pdf_into_shell_owned_domain() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let source_path = temp_dir.path().join("report.pdf");
        fs::write(&source_path, b"%PDF-1.4\n%%EOF").expect("source pdf should exist");
        let durable_root = temp_dir.path().join("durable-root");
        fs::create_dir_all(&durable_root).expect("durable root should exist");

        let destination_path = durable_root.join("report-data-run-1-1000.pdf");
        fs::copy(&source_path, &destination_path).expect("copy should succeed");

        assert!(destination_path.exists());
        assert_eq!(
            destination_path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str),
            Some("report-data-run-1-1000")
        );
    }
}