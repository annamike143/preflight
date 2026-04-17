use serde::{Deserialize, Serialize};

use crate::preflight_contract::{
    PreflightEffectivePlanSnapshot, PreflightProviderModelContextSnapshot,
    PreflightRequestedPlanSnapshot,
};
use crate::preflight_document_validation::SelectedSeedDocumentSnapshot;
use crate::preflight_readiness::preflight_readiness_truth_surface;
use crate::preflight_submission::preflight_submission_state_from_shell;

pub const RUN_START_LINEAGE_NAME: &str = "RunStartLineage";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStartLineageStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunStartLineageSnapshot {
    pub stage: RunStartLineageStage,
    pub lineage_name: &'static str,
    pub lineage_basis_materialized: bool,
    pub current_eligible_preflight_required: bool,
    pub preflight_state_name: &'static str,
    pub preflight_run_start_eligible_now: bool,
    pub preflight_result_current: bool,
    pub selected_seed_document: Option<SelectedSeedDocumentSnapshot>,
    pub requested_plan: Option<PreflightRequestedPlanSnapshot>,
    pub effective_plan: Option<PreflightEffectivePlanSnapshot>,
    pub provider_model_context: Option<PreflightProviderModelContextSnapshot>,
    pub downgrade_acceptance_required: bool,
    pub downgrade_acceptance_recorded: bool,
    pub lineage_derived_from_current_preflight: bool,
    pub raw_seed_bytes_embedded: bool,
    pub transcript_archive_embedded: bool,
    pub prior_run_memory_embedded: bool,
    pub future_execution_lineage_ready: bool,
    pub detail_note: String,
}

impl RunStartLineageSnapshot {
    pub fn summary(&self) -> String {
        let selected_seed_document_name = self
            .selected_seed_document
            .as_ref()
            .map(|document| document.file_name.as_str())
            .unwrap_or("none");

        format!(
            "Run-start lineage surface is materialized. Stage is {:?}; lineage name is {}; lineage basis materialized is {}; current eligible Preflight required is {}; current Preflight state is {}; current eligible Preflight now is {}; current Preflight result current is {}; selected seed document name is {}; requested plan present is {}; effective plan present is {}; provider/model context present is {}; downgrade acceptance required is {}; downgrade acceptance recorded is {}; lineage derived from current Preflight is {}; raw seed bytes embedded is {}; transcript archive embedded is {}; prior-run memory embedded is {}; future execution lineage ready is {}; detail note is {}.",
            self.stage,
            self.lineage_name,
            self.lineage_basis_materialized,
            self.current_eligible_preflight_required,
            self.preflight_state_name,
            self.preflight_run_start_eligible_now,
            self.preflight_result_current,
            selected_seed_document_name,
            self.requested_plan.is_some(),
            self.effective_plan.is_some(),
            self.provider_model_context.is_some(),
            self.downgrade_acceptance_required,
            self.downgrade_acceptance_recorded,
            self.lineage_derived_from_current_preflight,
            self.raw_seed_bytes_embedded,
            self.transcript_archive_embedded,
            self.prior_run_memory_embedded,
            self.future_execution_lineage_ready,
            self.detail_note,
        )
    }
}

pub fn run_start_lineage_surface() -> RunStartLineageSnapshot {
    let preflight_readiness = preflight_readiness_truth_surface();
    let submission_state = preflight_submission_state_from_shell();
    let preflight_result_current = !preflight_readiness.current_result_stale
        && preflight_readiness.last_technical_failure_reason.is_none()
        && preflight_readiness.document_validation_passed;
    let future_execution_lineage_ready = preflight_readiness.run_start_eligible_now
        && submission_state.selected_seed_document.is_some()
        && submission_state.requested_plan.is_some()
        && submission_state.effective_plan.is_some()
        && submission_state.provider_model_context.is_some();
    let detail_note = if future_execution_lineage_ready {
        "Current eligible Preflight truth already contains the bounded seed fingerprint, requested plan, effective plan, and provider/model planning context required for a future run start"
            .to_string()
    } else if !preflight_readiness.pending_seed_document_selected {
        "Run-start lineage is waiting for a current seed document selection and a current Preflight result"
            .to_string()
    } else if preflight_readiness.current_result_stale {
        preflight_readiness.current_status_note.clone()
    } else {
        format!(
            "Run-start lineage is not ready yet. {}",
            preflight_readiness.current_status_note
        )
    };

    RunStartLineageSnapshot {
        stage: RunStartLineageStage::FoundationMaterialized,
        lineage_name: RUN_START_LINEAGE_NAME,
        lineage_basis_materialized: true,
        current_eligible_preflight_required: true,
        preflight_state_name: preflight_readiness.current_state_name,
        preflight_run_start_eligible_now: preflight_readiness.run_start_eligible_now,
        preflight_result_current,
        selected_seed_document: submission_state.selected_seed_document,
        requested_plan: submission_state.requested_plan,
        effective_plan: submission_state.effective_plan,
        provider_model_context: submission_state.provider_model_context,
        downgrade_acceptance_required: preflight_readiness.downgrade_acceptance_required,
        downgrade_acceptance_recorded: preflight_readiness.downgrade_acceptance_recorded,
        lineage_derived_from_current_preflight: true,
        raw_seed_bytes_embedded: false,
        transcript_archive_embedded: false,
        prior_run_memory_embedded: false,
        future_execution_lineage_ready,
        detail_note,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_start_lineage_surface_stays_preflight_derived_and_non_archival() {
        let snapshot = run_start_lineage_surface();

        assert_eq!(snapshot.lineage_name, RUN_START_LINEAGE_NAME);
        assert!(snapshot.lineage_basis_materialized);
        assert!(snapshot.current_eligible_preflight_required);
        assert!(snapshot.lineage_derived_from_current_preflight);
        assert!(!snapshot.raw_seed_bytes_embedded);
        assert!(!snapshot.transcript_archive_embedded);
        assert!(!snapshot.prior_run_memory_embedded);
        assert!(!snapshot.preflight_state_name.is_empty());
    }
}