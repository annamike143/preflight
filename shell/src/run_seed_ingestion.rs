use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::preflight_contract::{
    PreflightEffectivePlanSnapshot, PreflightProviderModelContextSnapshot,
    PreflightRequestedPlanSnapshot,
};
use crate::preflight_document_validation::SelectedSeedDocumentSnapshot;
use crate::preflight_submission::current_selected_seed_document_bytes_from_shell;
use crate::run_start_lineage::RunStartLineageSnapshot;
use crate::run_workspace_allocation::RunWorkspaceAllocationSnapshot;

pub const RUN_START_LINEAGE_MANIFEST_FILE_NAME: &str = "run_start_lineage.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunSeedIngestionFailureReason {
    SeedDocumentUnavailable,
    SeedBytesUnavailable,
    WorkspaceAccessFailure,
    ManifestSerializationFailed,
    FileWriteFailed,
}

impl RunSeedIngestionFailureReason {
    pub fn label(self) -> &'static str {
        match self {
            Self::SeedDocumentUnavailable => "seed_document_unavailable",
            Self::SeedBytesUnavailable => "seed_bytes_unavailable",
            Self::WorkspaceAccessFailure => "workspace_access_failure",
            Self::ManifestSerializationFailed => "manifest_serialization_failed",
            Self::FileWriteFailed => "file_write_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedExecutionSeedSnapshot {
    pub staged_seed_document_path: String,
    pub lineage_manifest_path: String,
    pub staged_seed_fingerprint_sha256: String,
    pub requested_plan_present: bool,
    pub effective_plan_present: bool,
    pub provider_model_context_present: bool,
    pub raw_seed_bytes_staged: bool,
}

#[derive(Debug, Serialize)]
struct RunStartLineageManifest<'a> {
    lineage_name: &'a str,
    preflight_state_name: &'a str,
    preflight_run_start_eligible_now: bool,
    preflight_result_current: bool,
    downgrade_acceptance_required: bool,
    downgrade_acceptance_recorded: bool,
    selected_seed_document: &'a SelectedSeedDocumentSnapshot,
    requested_plan: &'a Option<PreflightRequestedPlanSnapshot>,
    effective_plan: &'a Option<PreflightEffectivePlanSnapshot>,
    provider_model_context: &'a Option<PreflightProviderModelContextSnapshot>,
    detail_note: &'a str,
}

pub fn stage_execution_seed_from_lineage(
    workspace: &RunWorkspaceAllocationSnapshot,
    lineage: &RunStartLineageSnapshot,
) -> Result<StagedExecutionSeedSnapshot, RunSeedIngestionFailureReason> {
    let selected_seed_document = lineage
        .selected_seed_document
        .as_ref()
        .ok_or(RunSeedIngestionFailureReason::SeedDocumentUnavailable)?;
    let seed_bytes = current_selected_seed_document_bytes_from_shell()
        .map_err(|_| RunSeedIngestionFailureReason::SeedBytesUnavailable)?
        .ok_or(RunSeedIngestionFailureReason::SeedBytesUnavailable)?;

    stage_execution_seed_from_lineage_with_bytes(workspace, lineage, selected_seed_document, &seed_bytes)
}

pub fn stage_execution_seed_from_lineage_with_bytes(
    workspace: &RunWorkspaceAllocationSnapshot,
    lineage: &RunStartLineageSnapshot,
    selected_seed_document: &SelectedSeedDocumentSnapshot,
    seed_bytes: &[u8],
) -> Result<StagedExecutionSeedSnapshot, RunSeedIngestionFailureReason> {
    let execution_seed_directory = PathBuf::from(&workspace.execution_seed_directory_path);
    fs::create_dir_all(&execution_seed_directory)
        .map_err(|_| RunSeedIngestionFailureReason::WorkspaceAccessFailure)?;

    let seed_file_name = sanitize_seed_file_name(&selected_seed_document.file_name);
    let staged_seed_document_path = execution_seed_directory.join(seed_file_name);
    fs::write(&staged_seed_document_path, seed_bytes)
        .map_err(|_| RunSeedIngestionFailureReason::FileWriteFailed)?;

    let manifest = RunStartLineageManifest {
        lineage_name: lineage.lineage_name,
        preflight_state_name: lineage.preflight_state_name,
        preflight_run_start_eligible_now: lineage.preflight_run_start_eligible_now,
        preflight_result_current: lineage.preflight_result_current,
        downgrade_acceptance_required: lineage.downgrade_acceptance_required,
        downgrade_acceptance_recorded: lineage.downgrade_acceptance_recorded,
        selected_seed_document,
        requested_plan: &lineage.requested_plan,
        effective_plan: &lineage.effective_plan,
        provider_model_context: &lineage.provider_model_context,
        detail_note: &lineage.detail_note,
    };
    let manifest_bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|_| RunSeedIngestionFailureReason::ManifestSerializationFailed)?;
    let lineage_manifest_path = execution_seed_directory.join(RUN_START_LINEAGE_MANIFEST_FILE_NAME);
    fs::write(&lineage_manifest_path, manifest_bytes)
        .map_err(|_| RunSeedIngestionFailureReason::FileWriteFailed)?;

    Ok(StagedExecutionSeedSnapshot {
        staged_seed_document_path: staged_seed_document_path.to_string_lossy().into_owned(),
        lineage_manifest_path: lineage_manifest_path.to_string_lossy().into_owned(),
        staged_seed_fingerprint_sha256: selected_seed_document.content_fingerprint_sha256.clone(),
        requested_plan_present: lineage.requested_plan.is_some(),
        effective_plan_present: lineage.effective_plan.is_some(),
        provider_model_context_present: lineage.provider_model_context.is_some(),
        raw_seed_bytes_staged: true,
    })
}

fn sanitize_seed_file_name(file_name: &str) -> String {
    Path::new(file_name)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .map(|name| name.to_string())
        .unwrap_or_else(|| "seed_document.bin".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preflight_contract::{
        PreflightEffectivePlanSnapshot, PreflightProviderModelContextSnapshot,
        PreflightRequestedPlanSnapshot,
    };
    use crate::preflight_document_validation::SelectedSeedDocumentSnapshot;
    use crate::run_start_lineage::{RunStartLineageSnapshot, RunStartLineageStage};
    use crate::run_workspace_allocation::allocate_fresh_run_workspace_under;

    #[test]
    fn stage_execution_seed_from_lineage_with_bytes_writes_seed_and_manifest() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let workspace = allocate_fresh_run_workspace_under(temp_dir.path(), "run-1")
            .expect("workspace should allocate");
        let selected_seed_document = SelectedSeedDocumentSnapshot {
            file_name: "seed.txt".to_string(),
            mime_type: Some("text/plain".to_string()),
            detected_kind: Some(crate::preflight_contract::SupportedSeedDocumentKind::Txt),
            size_bytes: 12,
            content_fingerprint_sha256: "abc123".to_string(),
            extracted_text_character_count: Some(1200),
            english_confidence_percent: Some(99),
        };
        let lineage = RunStartLineageSnapshot {
            stage: RunStartLineageStage::FoundationMaterialized,
            lineage_name: crate::run_start_lineage::RUN_START_LINEAGE_NAME,
            lineage_basis_materialized: true,
            current_eligible_preflight_required: true,
            preflight_state_name: "PreflightApproved",
            preflight_run_start_eligible_now: true,
            preflight_result_current: true,
            selected_seed_document: Some(selected_seed_document.clone()),
            requested_plan: Some(PreflightRequestedPlanSnapshot {
                requested_agent_count: 5,
                requested_round_count: 4,
                max_token_budget: 12000,
            }),
            effective_plan: Some(PreflightEffectivePlanSnapshot {
                effective_agent_count: 5,
                effective_round_count: 4,
                provider: crate::provider_configuration_contract::SupportedProvider::OpenAi,
                model: "gpt-4o-mini".to_string(),
                max_token_budget: 12000,
                bounded_speaker_model: "moderated_3_to_5_speakers_per_round".to_string(),
                bounded_concurrency_policy: "single_moderated_speaker_set_per_round".to_string(),
            }),
            provider_model_context: Some(PreflightProviderModelContextSnapshot {
                provider: crate::provider_configuration_contract::SupportedProvider::OpenAi,
                model: "gpt-4o-mini".to_string(),
                local_configuration_supports_planning: true,
                live_reachability_probed: false,
                detail_note: "planning context".to_string(),
            }),
            downgrade_acceptance_required: false,
            downgrade_acceptance_recorded: false,
            lineage_derived_from_current_preflight: true,
            raw_seed_bytes_embedded: false,
            transcript_archive_embedded: false,
            prior_run_memory_embedded: false,
            future_execution_lineage_ready: true,
            detail_note: "lineage ready".to_string(),
        };

        let staging = stage_execution_seed_from_lineage_with_bytes(
            &workspace,
            &lineage,
            &selected_seed_document,
            b"hello world!",
        )
        .expect("seed staging should succeed");

        assert!(Path::new(&staging.staged_seed_document_path).exists());
        assert!(Path::new(&staging.lineage_manifest_path).exists());
        assert_eq!(staging.staged_seed_fingerprint_sha256, "abc123");
        assert!(staging.requested_plan_present);
        assert!(staging.effective_plan_present);
        assert!(staging.provider_model_context_present);
        assert!(staging.raw_seed_bytes_staged);
    }
}