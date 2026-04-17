use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::report_storage_boundary::export_report_artifact;
use crate::run_session_start::{run_session_state_from_shell, RunSessionStateSnapshot};

pub const REPORT_ACCESS_CONTRACT_NAME: &str = "ReportAccessContract";
pub const REPORT_ACCESS_CONTRACT_COMMAND_NAME: &str = "ReportAccessContractState";
pub const SUPPORTED_REPORT_ACCESS_REQUEST_KINDS: [ReportAccessRequestKind; 3] = [
    ReportAccessRequestKind::RetrieveMetadata,
    ReportAccessRequestKind::OpenLocalPdf,
    ReportAccessRequestKind::ExportLocalPdf,
];
pub const SUPPORTED_REPORT_ACCESS_FAILURE_CLASSIFICATIONS: [ReportAccessFailureClassification; 4] = [
    ReportAccessFailureClassification::Unavailable,
    ReportAccessFailureClassification::MissingArtifact,
    ReportAccessFailureClassification::InaccessibleArtifact,
    ReportAccessFailureClassification::MalformedRequest,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportAccessContractStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportAvailabilityStatus {
    Unavailable,
    Available,
    MissingArtifact,
    InaccessibleArtifact,
}

impl ReportAvailabilityStatus {
    pub fn name(self) -> &'static str {
        match self {
            Self::Unavailable => "ReportUnavailable",
            Self::Available => "ReportAvailable",
            Self::MissingArtifact => "ReportMissingArtifact",
            Self::InaccessibleArtifact => "ReportInaccessibleArtifact",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportAccessRequestKind {
    RetrieveMetadata,
    OpenLocalPdf,
    ExportLocalPdf,
}

impl ReportAccessRequestKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::RetrieveMetadata => "RetrieveMetadata",
            Self::OpenLocalPdf => "OpenLocalPdf",
            Self::ExportLocalPdf => "ExportLocalPdf",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportAccessFailureClassification {
    Unavailable,
    MissingArtifact,
    InaccessibleArtifact,
    MalformedRequest,
}

impl ReportAccessFailureClassification {
    pub fn name(self) -> &'static str {
        match self {
            Self::Unavailable => "Unavailable",
            Self::MissingArtifact => "MissingArtifact",
            Self::InaccessibleArtifact => "InaccessibleArtifact",
            Self::MalformedRequest => "MalformedRequest",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportAccessMetadataSnapshot {
    pub report_identifier: Option<String>,
    pub run_identifier: Option<String>,
    pub session_identifier: Option<String>,
    pub completion_classification: Option<String>,
    pub report_generation_status: Option<String>,
    pub report_generation_failure_reason: Option<String>,
    pub availability_status: ReportAvailabilityStatus,
    pub availability_status_name: &'static str,
    pub local_artifact_path: Option<String>,
    pub file_size_bytes: Option<u64>,
    pub generated_at: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportAccessExecutionDecisionType {
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportAccessExecutionOutcome {
    pub request_kind: ReportAccessRequestKind,
    pub decision_type: ReportAccessExecutionDecisionType,
    pub failure_classification: Option<ReportAccessFailureClassification>,
    pub availability_status: ReportAvailabilityStatus,
    pub availability_status_name: &'static str,
    pub local_artifact_path: Option<String>,
    pub exported_artifact_path: Option<String>,
    pub detail_message: String,
}

impl ReportAccessExecutionOutcome {
    fn success(
        request_kind: ReportAccessRequestKind,
        metadata: &ReportAccessMetadataSnapshot,
        detail_message: String,
        exported_artifact_path: Option<String>,
    ) -> Self {
        Self {
            request_kind,
            decision_type: ReportAccessExecutionDecisionType::Succeeded,
            failure_classification: None,
            availability_status: metadata.availability_status,
            availability_status_name: metadata.availability_status_name,
            local_artifact_path: metadata.local_artifact_path.clone(),
            exported_artifact_path,
            detail_message,
        }
    }

    fn failure(
        request_kind: ReportAccessRequestKind,
        metadata: &ReportAccessMetadataSnapshot,
        failure_classification: ReportAccessFailureClassification,
        detail_message: String,
    ) -> Self {
        Self {
            request_kind,
            decision_type: ReportAccessExecutionDecisionType::Failed,
            failure_classification: Some(failure_classification),
            availability_status: metadata.availability_status,
            availability_status_name: metadata.availability_status_name,
            local_artifact_path: metadata.local_artifact_path.clone(),
            exported_artifact_path: None,
            detail_message,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportMetadataRetrievalContract {
    pub report_identifier_required_when_generated: bool,
    pub run_identifier_required: bool,
    pub session_identifier_required: bool,
    pub completion_classification_required: bool,
    pub availability_status_required: bool,
    pub local_artifact_path_optional_until_accessible: bool,
    pub file_size_bytes_optional_until_accessible: bool,
    pub secret_material_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportAccessRequestContract {
    pub shell_command_boundary_required: bool,
    pub frontend_direct_filesystem_access_allowed: bool,
    pub local_artifact_reference_required_for_open_or_export: bool,
    pub offline_access_possible_when_file_exists: bool,
    pub open_request_shape_defined: bool,
    pub export_request_shape_defined: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportAccessResponseContract {
    pub metadata_retrieval_supported: bool,
    pub availability_representation_supported: bool,
    pub open_request_supported: bool,
    pub export_request_supported: bool,
    pub unavailable_response_defined: bool,
    pub missing_artifact_response_defined: bool,
    pub inaccessible_artifact_response_defined: bool,
    pub local_open_execution_materialized: bool,
    pub local_export_execution_materialized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportAccessContractSnapshot {
    pub stage: ReportAccessContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub supported_request_kinds: [ReportAccessRequestKind; 3],
    pub supported_failure_classifications: [ReportAccessFailureClassification; 4],
    pub current_availability_status: ReportAvailabilityStatus,
    pub current_availability_status_name: &'static str,
    pub shell_authority_required: bool,
    pub metadata_retrieval_contract: ReportMetadataRetrievalContract,
    pub access_request_contract: ReportAccessRequestContract,
    pub response_contract: ReportAccessResponseContract,
    pub report_available_now: bool,
    pub local_artifact_accessible_now: bool,
    pub phantom_report_state_allowed: bool,
    pub completion_classification_implies_report_available: bool,
    pub report_generation_success_implies_local_accessibility: bool,
    pub cleanup_outcome_implies_report_deletion: bool,
}

impl ReportAccessContractSnapshot {
    pub fn summary(self) -> String {
        let request_kinds = self
            .supported_request_kinds
            .map(ReportAccessRequestKind::name)
            .join(", ");
        let failure_classifications = self
            .supported_failure_classifications
            .map(ReportAccessFailureClassification::name)
            .join(", ");

        format!(
            "Report access contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; supported request kinds are {}; supported failure classifications are {}; current availability status is {} ({:?}); shell authority required is {}; report identifier required when generated is {}; run identifier required is {}; session identifier required is {}; completion classification required is {}; availability status required is {}; local artifact path optional until accessible is {}; file-size-bytes optional until accessible is {}; secret material allowed is {}; shell command boundary required is {}; frontend direct filesystem access allowed is {}; local artifact reference required for open or export is {}; offline access possible when file exists is {}; open request shape defined is {}; export request shape defined is {}; metadata retrieval supported is {}; availability representation supported is {}; open request supported is {}; export request supported is {}; unavailable response defined is {}; missing artifact response defined is {}; inaccessible artifact response defined is {}; local open execution materialized is {}; local export execution materialized is {}; report available now is {}; local artifact accessible now is {}; phantom report state allowed is {}; completion classification implies report available is {}; report generation success implies local accessibility is {}; cleanup outcome implies report deletion is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            request_kinds,
            failure_classifications,
            self.current_availability_status_name,
            self.current_availability_status,
            self.shell_authority_required,
            self.metadata_retrieval_contract.report_identifier_required_when_generated,
            self.metadata_retrieval_contract.run_identifier_required,
            self.metadata_retrieval_contract.session_identifier_required,
            self.metadata_retrieval_contract.completion_classification_required,
            self.metadata_retrieval_contract.availability_status_required,
            self.metadata_retrieval_contract.local_artifact_path_optional_until_accessible,
            self.metadata_retrieval_contract.file_size_bytes_optional_until_accessible,
            self.metadata_retrieval_contract.secret_material_allowed,
            self.access_request_contract.shell_command_boundary_required,
            self.access_request_contract.frontend_direct_filesystem_access_allowed,
            self.access_request_contract.local_artifact_reference_required_for_open_or_export,
            self.access_request_contract.offline_access_possible_when_file_exists,
            self.access_request_contract.open_request_shape_defined,
            self.access_request_contract.export_request_shape_defined,
            self.response_contract.metadata_retrieval_supported,
            self.response_contract.availability_representation_supported,
            self.response_contract.open_request_supported,
            self.response_contract.export_request_supported,
            self.response_contract.unavailable_response_defined,
            self.response_contract.missing_artifact_response_defined,
            self.response_contract.inaccessible_artifact_response_defined,
            self.response_contract.local_open_execution_materialized,
            self.response_contract.local_export_execution_materialized,
            self.report_available_now,
            self.local_artifact_accessible_now,
            self.phantom_report_state_allowed,
            self.completion_classification_implies_report_available,
            self.report_generation_success_implies_local_accessibility,
            self.cleanup_outcome_implies_report_deletion,
        )
    }
}

pub fn report_access_metadata_state() -> ReportAccessMetadataSnapshot {
    report_access_metadata_from_run_session_state(&run_session_state_from_shell())
}

pub fn report_access_contract_state() -> ReportAccessContractSnapshot {
    report_access_contract_from_metadata(&report_access_metadata_state())
}

pub fn open_local_report_pdf_from_shell() -> ReportAccessExecutionOutcome {
    let metadata = report_access_metadata_state();
    let artifact_path = match resolve_accessible_artifact_path(&metadata, ReportAccessRequestKind::OpenLocalPdf) {
        Ok(path) => path,
        Err(outcome) => return outcome,
    };

    match open_path_with_default_application(&artifact_path) {
        Ok(()) => ReportAccessExecutionOutcome::success(
            ReportAccessRequestKind::OpenLocalPdf,
            &metadata,
            format!(
                "Shell opened the durable local PDF artifact at {}.",
                artifact_path.display()
            ),
            None,
        ),
        Err(error) => ReportAccessExecutionOutcome::failure(
            ReportAccessRequestKind::OpenLocalPdf,
            &metadata,
            ReportAccessFailureClassification::InaccessibleArtifact,
            format!(
                "Shell could not open the durable local PDF artifact: {error}"
            ),
        ),
    }
}

pub fn export_local_report_pdf_from_shell() -> ReportAccessExecutionOutcome {
    let metadata = report_access_metadata_state();
    let artifact_path = match resolve_accessible_artifact_path(&metadata, ReportAccessRequestKind::ExportLocalPdf) {
        Ok(path) => path,
        Err(outcome) => return outcome,
    };
    let report_identifier = metadata
        .report_identifier
        .clone()
        .unwrap_or_else(|| "report".to_string());

    match export_report_artifact(&artifact_path, &report_identifier) {
        Ok(exported_path) => ReportAccessExecutionOutcome::success(
            ReportAccessRequestKind::ExportLocalPdf,
            &metadata,
            format!(
                "Shell exported the local PDF artifact to {}.",
                exported_path.display()
            ),
            Some(exported_path.display().to_string()),
        ),
        Err(error) => ReportAccessExecutionOutcome::failure(
            ReportAccessRequestKind::ExportLocalPdf,
            &metadata,
            ReportAccessFailureClassification::InaccessibleArtifact,
            format!("Shell could not export the local PDF artifact: {error}"),
        ),
    }
}

fn report_access_metadata_from_run_session_state(
    run_session_state: &RunSessionStateSnapshot,
) -> ReportAccessMetadataSnapshot {
    let availability_status = derive_availability_status(run_session_state);
    let availability_status_name = availability_status.name();
    let file_size_bytes = run_session_state
        .durable_report_artifact_path
        .as_ref()
        .and_then(|artifact_path| match availability_status {
            ReportAvailabilityStatus::Available => fs::metadata(artifact_path).ok().map(|metadata| metadata.len()),
            _ => None,
        });

    ReportAccessMetadataSnapshot {
        report_identifier: run_session_state.report_identifier.clone(),
        run_identifier: run_session_state.run_id.clone(),
        session_identifier: run_session_state.session_id.clone(),
        completion_classification: run_session_state.execution_completion_state.clone(),
        report_generation_status: run_session_state.report_generation_status.clone(),
        report_generation_failure_reason: run_session_state.report_generation_failure_reason.clone(),
        availability_status,
        availability_status_name,
        local_artifact_path: run_session_state.durable_report_artifact_path.clone(),
        file_size_bytes,
        generated_at: run_session_state.report_generated_at,
    }
}

fn report_access_contract_from_metadata(
    metadata: &ReportAccessMetadataSnapshot,
) -> ReportAccessContractSnapshot {
    let report_available_now = metadata.availability_status == ReportAvailabilityStatus::Available;

    ReportAccessContractSnapshot {
        stage: ReportAccessContractStage::FoundationMaterialized,
        contract_name: REPORT_ACCESS_CONTRACT_NAME,
        shell_command_name: REPORT_ACCESS_CONTRACT_COMMAND_NAME,
        supported_request_kinds: SUPPORTED_REPORT_ACCESS_REQUEST_KINDS,
        supported_failure_classifications: SUPPORTED_REPORT_ACCESS_FAILURE_CLASSIFICATIONS,
        current_availability_status: metadata.availability_status,
        current_availability_status_name: metadata.availability_status_name,
        shell_authority_required: true,
        metadata_retrieval_contract: ReportMetadataRetrievalContract {
            report_identifier_required_when_generated: true,
            run_identifier_required: true,
            session_identifier_required: true,
            completion_classification_required: true,
            availability_status_required: true,
            local_artifact_path_optional_until_accessible: true,
            file_size_bytes_optional_until_accessible: true,
            secret_material_allowed: false,
        },
        access_request_contract: ReportAccessRequestContract {
            shell_command_boundary_required: true,
            frontend_direct_filesystem_access_allowed: false,
            local_artifact_reference_required_for_open_or_export: true,
            offline_access_possible_when_file_exists: true,
            open_request_shape_defined: true,
            export_request_shape_defined: true,
        },
        response_contract: ReportAccessResponseContract {
            metadata_retrieval_supported: true,
            availability_representation_supported: true,
            open_request_supported: true,
            export_request_supported: true,
            unavailable_response_defined: true,
            missing_artifact_response_defined: true,
            inaccessible_artifact_response_defined: true,
            local_open_execution_materialized: true,
            local_export_execution_materialized: true,
        },
        report_available_now,
        local_artifact_accessible_now: report_available_now,
        phantom_report_state_allowed: false,
        completion_classification_implies_report_available: false,
        report_generation_success_implies_local_accessibility: report_available_now,
        cleanup_outcome_implies_report_deletion: false,
    }
}

fn derive_availability_status(run_session_state: &RunSessionStateSnapshot) -> ReportAvailabilityStatus {
    if run_session_state.report_generation_status.as_deref() != Some("succeeded") {
        return ReportAvailabilityStatus::Unavailable;
    }

    let Some(artifact_path) = run_session_state.durable_report_artifact_path.as_ref() else {
        return ReportAvailabilityStatus::Unavailable;
    };

    match fs::metadata(artifact_path) {
        Ok(metadata) if metadata.is_file() && metadata.len() > 0 => ReportAvailabilityStatus::Available,
        Ok(_) => ReportAvailabilityStatus::InaccessibleArtifact,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            ReportAvailabilityStatus::MissingArtifact
        }
        Err(_) => ReportAvailabilityStatus::InaccessibleArtifact,
    }
}

fn resolve_accessible_artifact_path(
    metadata: &ReportAccessMetadataSnapshot,
    request_kind: ReportAccessRequestKind,
) -> Result<PathBuf, ReportAccessExecutionOutcome> {
    let failure_classification = match metadata.availability_status {
        ReportAvailabilityStatus::Unavailable => ReportAccessFailureClassification::Unavailable,
        ReportAvailabilityStatus::MissingArtifact => ReportAccessFailureClassification::MissingArtifact,
        ReportAvailabilityStatus::InaccessibleArtifact => {
            ReportAccessFailureClassification::InaccessibleArtifact
        }
        ReportAvailabilityStatus::Available => {
            let Some(artifact_path) = metadata.local_artifact_path.as_ref() else {
                return Err(ReportAccessExecutionOutcome::failure(
                    request_kind,
                    metadata,
                    ReportAccessFailureClassification::MalformedRequest,
                    "Shell report access could not resolve the local artifact path.".to_string(),
                ));
            };

            return Ok(PathBuf::from(artifact_path));
        }
    };

    Err(ReportAccessExecutionOutcome::failure(
        request_kind,
        metadata,
        failure_classification,
        format!(
            "Shell report access rejected {} because the current availability state is {}.",
            request_kind.name(),
            metadata.availability_status_name,
        ),
    ))
}

fn open_path_with_default_application(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let status = Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(path)
        .status()
        .map_err(|error| error.to_string())?;

    #[cfg(target_os = "macos")]
    let status = Command::new("open")
        .arg(path)
        .status()
        .map_err(|error| error.to_string())?;

    #[cfg(all(unix, not(target_os = "macos")))]
    let status = Command::new("xdg-open")
        .arg(path)
        .status()
        .map_err(|error| error.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "default application command exited with status {:?}",
            status.code()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_session_start::RunSessionStateSnapshot;

    #[test]
    fn report_access_contract_tracks_available_artifacts_from_shell_state() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let durable_report_path = temp_dir.path().join("report-data-run-1-1000.pdf");
        fs::write(&durable_report_path, b"%PDF-1.4\n%%EOF").expect("pdf should exist");

        let mut snapshot = RunSessionStateSnapshot::not_started();
        snapshot.run_id = Some("run-1".to_string());
        snapshot.session_id = Some("session-1".to_string());
        snapshot.execution_completion_state = Some("completed".to_string());
        snapshot.report_identifier = Some("report-data-run-1-1000".to_string());
        snapshot.report_generation_status = Some("succeeded".to_string());
        snapshot.report_generated_at = Some(1000);
        snapshot.durable_report_artifact_path = Some(durable_report_path.display().to_string());

        let metadata = report_access_metadata_from_run_session_state(&snapshot);
        let contract = report_access_contract_from_metadata(&metadata);

        assert_eq!(metadata.availability_status, ReportAvailabilityStatus::Available);
        assert_eq!(metadata.file_size_bytes, Some(14));
        assert!(contract.report_available_now);
        assert!(contract.local_artifact_accessible_now);
        assert!(contract.response_contract.local_open_execution_materialized);
        assert!(contract.response_contract.local_export_execution_materialized);
    }

    #[test]
    fn report_access_contract_keeps_availability_and_local_access_truth_separate() {
        let snapshot = report_access_contract_state();

        assert_eq!(snapshot.contract_name, REPORT_ACCESS_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, REPORT_ACCESS_CONTRACT_COMMAND_NAME);
        assert_eq!(snapshot.supported_request_kinds.len(), 3);
        assert_eq!(snapshot.supported_failure_classifications.len(), 4);
        assert_eq!(
            snapshot.current_availability_status,
            ReportAvailabilityStatus::Unavailable
        );
        assert_eq!(snapshot.current_availability_status_name, "ReportUnavailable");
        assert!(snapshot.shell_authority_required);
        assert!(snapshot
            .metadata_retrieval_contract
            .report_identifier_required_when_generated);
        assert!(snapshot.metadata_retrieval_contract.run_identifier_required);
        assert!(snapshot.metadata_retrieval_contract.session_identifier_required);
        assert!(snapshot
            .metadata_retrieval_contract
            .completion_classification_required);
        assert!(snapshot.metadata_retrieval_contract.availability_status_required);
        assert!(snapshot
            .metadata_retrieval_contract
            .local_artifact_path_optional_until_accessible);
        assert!(snapshot
            .metadata_retrieval_contract
            .file_size_bytes_optional_until_accessible);
        assert!(!snapshot.metadata_retrieval_contract.secret_material_allowed);
        assert!(snapshot.access_request_contract.shell_command_boundary_required);
        assert!(!snapshot.access_request_contract.frontend_direct_filesystem_access_allowed);
        assert!(snapshot
            .access_request_contract
            .local_artifact_reference_required_for_open_or_export);
        assert!(snapshot.access_request_contract.offline_access_possible_when_file_exists);
        assert!(snapshot.access_request_contract.open_request_shape_defined);
        assert!(snapshot.access_request_contract.export_request_shape_defined);
        assert!(snapshot.response_contract.metadata_retrieval_supported);
        assert!(snapshot.response_contract.availability_representation_supported);
        assert!(snapshot.response_contract.open_request_supported);
        assert!(snapshot.response_contract.export_request_supported);
        assert!(snapshot.response_contract.unavailable_response_defined);
        assert!(snapshot.response_contract.missing_artifact_response_defined);
        assert!(snapshot.response_contract.inaccessible_artifact_response_defined);
        assert!(snapshot.response_contract.local_open_execution_materialized);
        assert!(snapshot.response_contract.local_export_execution_materialized);
        assert!(!snapshot.report_available_now);
        assert!(!snapshot.local_artifact_accessible_now);
        assert!(!snapshot.phantom_report_state_allowed);
        assert!(!snapshot.completion_classification_implies_report_available);
        assert!(!snapshot.report_generation_success_implies_local_accessibility);
        assert!(!snapshot.cleanup_outcome_implies_report_deletion);
    }
}