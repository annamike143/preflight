use serde::{Deserialize, Serialize};

use crate::activation_readiness::activation_readiness_truth_surface;
use crate::preflight_contract::{
    preflight_contract_basis, PreflightEstimatedUsageSnapshot,
    PreflightEffectivePlanSnapshot, PreflightInvalidationReason,
    PreflightProviderModelContextSnapshot, PreflightRejectionReason,
    PreflightRequestedPlanSnapshot, PreflightTechnicalFailureReason,
};
use crate::preflight_document_validation::SelectedSeedDocumentSnapshot;
use crate::preflight_feasibility::{
    preflight_invalidation_note, preflight_invalidation_reasons_for_state,
};
use crate::preflight_submission::{
    preflight_submission_state_from_shell, PreflightSubmissionStateSnapshot,
    PreflightSubmissionStatus,
};
use crate::provider_readiness::provider_readiness_truth_surface;

pub const STATE_PREFLIGHT_NOT_STARTED: &str = "PreflightNotStarted";
pub const STATE_PREFLIGHT_RUNNING: &str = "PreflightRunning";
pub const STATE_PREFLIGHT_APPROVED: &str = "PreflightApproved";
pub const STATE_PREFLIGHT_APPROVED_WITH_DOWNGRADE: &str = "PreflightApprovedWithDowngrade";
pub const STATE_PREFLIGHT_REJECTED: &str = "PreflightRejected";
pub const STATE_PREFLIGHT_INVALIDATED: &str = "PreflightInvalidated";
pub const STATE_PREFLIGHT_ERROR: &str = "PreflightError";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightReadinessStage {
    FeasibilityAndInvalidationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightState {
    NotStarted,
    Running,
    Approved,
    ApprovedWithDowngrade,
    Rejected,
    Invalidated,
    Error,
}

impl PreflightState {
    pub fn state_name(self) -> &'static str {
        match self {
            Self::NotStarted => STATE_PREFLIGHT_NOT_STARTED,
            Self::Running => STATE_PREFLIGHT_RUNNING,
            Self::Approved => STATE_PREFLIGHT_APPROVED,
            Self::ApprovedWithDowngrade => STATE_PREFLIGHT_APPROVED_WITH_DOWNGRADE,
            Self::Rejected => STATE_PREFLIGHT_REJECTED,
            Self::Invalidated => STATE_PREFLIGHT_INVALIDATED,
            Self::Error => STATE_PREFLIGHT_ERROR,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreflightReadinessSnapshot {
    pub stage: PreflightReadinessStage,
    pub state_surface_materialized: bool,
    pub preflight_contract_defined: bool,
    pub requested_plan_contract_defined: bool,
    pub effective_plan_contract_defined: bool,
    pub invalidation_semantics_defined: bool,
    pub run_start_gate_truth_surface_materialized: bool,
    pub preflight_execution_materialized: bool,
    pub runtime_readiness_evaluated_separately: bool,
    pub activation_readiness_evaluated_separately: bool,
    pub provider_readiness_evaluated_separately: bool,
    pub runtime_ready_now: bool,
    pub activation_ready_now: bool,
    pub provider_configuration_ready_now: bool,
    pub pending_seed_document_selected: bool,
    pub requested_plan_present: bool,
    pub selected_seed_document: Option<SelectedSeedDocumentSnapshot>,
    pub requested_plan: Option<PreflightRequestedPlanSnapshot>,
    pub effective_plan: Option<PreflightEffectivePlanSnapshot>,
    pub estimated_usage: Option<PreflightEstimatedUsageSnapshot>,
    pub provider_model_context: Option<PreflightProviderModelContextSnapshot>,
    pub requested_plan_budget_feasible: Option<bool>,
    pub downgrade_warning: Option<String>,
    pub document_validation_completed: bool,
    pub document_validation_passed: bool,
    pub current_state: PreflightState,
    pub current_state_name: &'static str,
    pub last_rejection_reason: Option<PreflightRejectionReason>,
    pub last_technical_failure_reason: Option<PreflightTechnicalFailureReason>,
    pub invalidation_reasons: Vec<PreflightInvalidationReason>,
    pub current_status_note: String,
    pub downgrade_acceptance_required: bool,
    pub downgrade_acceptance_recorded: bool,
    pub current_result_stale: bool,
    pub run_start_eligible_now: bool,
    pub run_start_blocked_without_eligible_preflight: bool,
}

impl PreflightReadinessSnapshot {
    pub fn supported_states(&self) -> [PreflightState; 7] {
        [
            PreflightState::NotStarted,
            PreflightState::Running,
            PreflightState::Approved,
            PreflightState::ApprovedWithDowngrade,
            PreflightState::Rejected,
            PreflightState::Invalidated,
            PreflightState::Error,
        ]
    }

    pub fn blocking_reasons(&self) -> Vec<&'static str> {
        let mut reasons = Vec::new();

        if !self.runtime_ready_now {
            reasons.push("runtime readiness is not satisfied");
        }

        if !self.activation_ready_now {
            reasons.push("activation readiness is not satisfied");
        }

        if !self.provider_configuration_ready_now {
            reasons.push("provider configuration readiness is not satisfied");
        }

        if !self.pending_seed_document_selected {
            reasons.push("seed document is not selected");
        }

        if !self.requested_plan_present {
            reasons.push("requested plan inputs are not present");
        }

        if !self.run_start_eligible_now {
            reasons.push("current Preflight state is not run-start eligible");
        }

        reasons
    }

    pub fn summary(&self) -> String {
        let blocking_reasons = self.blocking_reasons().join(", ");
        let supported_state_count = self.supported_states().len();
        let selected_seed_document_name = self
            .selected_seed_document
            .as_ref()
            .map(|seed_document| seed_document.file_name.as_str())
            .unwrap_or("none");
        let invalidation_reasons = self
            .invalidation_reasons
            .iter()
            .map(|reason| reason.label())
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "Preflight readiness truth surface exists with Wave 4 feasibility, downgrade, and invalidation state. Stage is {:?}; state surface materialized is {}; Preflight contract defined is {}; requested-plan contract defined is {}; effective-plan contract defined is {}; invalidation semantics defined is {}; run-start-gate truth surface materialized is {}; Preflight execution materialized is {}; runtime readiness evaluated separately is {}; activation readiness evaluated separately is {}; provider readiness evaluated separately is {}; runtime ready now is {}; activation ready now is {}; provider configuration ready now is {}; pending seed document selected is {}; requested plan present is {}; selected seed document name is {}; document validation completed is {}; document validation passed is {}; current state is {:?}; current state name is {}; last rejection reason is {:?}; last technical failure reason is {:?}; invalidation reasons are {}; current status note is {}; downgrade acceptance required is {}; downgrade acceptance recorded is {}; current result stale is {}; run-start eligible now is {}; run-start blocked without eligible Preflight is {}; supported state count is {}; blocking reasons are {}.",
            self.stage,
            self.state_surface_materialized,
            self.preflight_contract_defined,
            self.requested_plan_contract_defined,
            self.effective_plan_contract_defined,
            self.invalidation_semantics_defined,
            self.run_start_gate_truth_surface_materialized,
            self.preflight_execution_materialized,
            self.runtime_readiness_evaluated_separately,
            self.activation_readiness_evaluated_separately,
            self.provider_readiness_evaluated_separately,
            self.runtime_ready_now,
            self.activation_ready_now,
            self.provider_configuration_ready_now,
            self.pending_seed_document_selected,
            self.requested_plan_present,
            selected_seed_document_name,
            self.document_validation_completed,
            self.document_validation_passed,
            self.current_state,
            self.current_state_name,
            self.last_rejection_reason,
            self.last_technical_failure_reason,
            invalidation_reasons,
            self.current_status_note,
            self.downgrade_acceptance_required,
            self.downgrade_acceptance_recorded,
            self.current_result_stale,
            self.run_start_eligible_now,
            self.run_start_blocked_without_eligible_preflight,
            supported_state_count,
            blocking_reasons
        )
    }
}

pub fn preflight_readiness_truth_surface() -> PreflightReadinessSnapshot {
    preflight_readiness_from_submission_state(preflight_submission_state_from_shell())
}

fn preflight_readiness_from_submission_state(
    submission_state: PreflightSubmissionStateSnapshot,
) -> PreflightReadinessSnapshot {
    let preflight_contract = preflight_contract_basis();
    let activation_readiness = activation_readiness_truth_surface();
    let provider_readiness = provider_readiness_truth_surface();
    let invalidation_result = submission_state
        .evaluation_basis
        .as_ref()
        .map(|evaluation_basis| {
            preflight_invalidation_reasons_for_state(
                evaluation_basis,
                submission_state.selected_seed_document.as_ref(),
                submission_state.requested_plan.as_ref(),
            )
        })
        .transpose();
    let invalidation_failure = invalidation_result.as_ref().err().copied();
    let invalidation_reasons = invalidation_result.ok().flatten().unwrap_or_default();
    let current_result_stale = !invalidation_reasons.is_empty();
    let current_state = resolve_current_state(
        submission_state.status,
        current_result_stale,
        invalidation_failure.is_some(),
    );
    let downgrade_acceptance_required = current_state == PreflightState::ApprovedWithDowngrade;
    let downgrade_acceptance_recorded = downgrade_acceptance_required
        && submission_state.downgrade_acceptance_recorded;

    PreflightReadinessSnapshot {
        stage: PreflightReadinessStage::FeasibilityAndInvalidationMaterialized,
        state_surface_materialized: true,
        preflight_contract_defined: preflight_contract.response_contract.approved_supported
            && preflight_contract.response_contract.approved_with_downgrade_supported
            && preflight_contract.response_contract.rejected_supported
            && preflight_contract.response_contract.technical_failure_distinct_from_rejection,
        requested_plan_contract_defined: preflight_contract
            .requested_plan_contract
            .requested_plan_echo_required,
        effective_plan_contract_defined: preflight_contract
            .requested_plan_contract
            .effective_plan_fields_supported,
        invalidation_semantics_defined: preflight_contract
            .response_contract
            .stale_invalidation_semantics_supported
            && preflight_contract.invalidation_contract.seed_document_change_invalidates
            && preflight_contract.invalidation_contract.provider_change_invalidates,
        run_start_gate_truth_surface_materialized: true,
        preflight_execution_materialized: preflight_contract.preflight_execution_materialized,
        runtime_readiness_evaluated_separately: true,
        activation_readiness_evaluated_separately: true,
        provider_readiness_evaluated_separately: true,
        runtime_ready_now: activation_readiness.runtime_ready_now,
        activation_ready_now: activation_readiness.activation_ready_for_licensed_use,
        provider_configuration_ready_now: provider_readiness.provider_configuration_ready,
        pending_seed_document_selected: submission_state.selected_seed_document.is_some(),
        requested_plan_present: submission_state.requested_plan.is_some(),
        selected_seed_document: submission_state.selected_seed_document,
        requested_plan: submission_state.requested_plan,
        effective_plan: submission_state.effective_plan,
        estimated_usage: submission_state.estimated_usage,
        provider_model_context: submission_state.provider_model_context,
        requested_plan_budget_feasible: submission_state.requested_plan_budget_feasible,
        downgrade_warning: submission_state.downgrade_warning,
        document_validation_completed: submission_state.status != PreflightSubmissionStatus::NotStarted,
        document_validation_passed: submission_state.valid_document_in_pending_configuration,
        current_state,
        current_state_name: current_state.state_name(),
        last_rejection_reason: submission_state.last_rejection_reason,
        last_technical_failure_reason: invalidation_failure.or(submission_state.last_technical_failure_reason),
        invalidation_reasons: invalidation_reasons.clone(),
        current_status_note: if let Some(technical_failure_reason) = invalidation_failure {
            match technical_failure_reason {
                PreflightTechnicalFailureReason::MalformedInput => {
                    "Preflight readiness could not compare current inputs because the stored basis was malformed"
                        .to_string()
                }
                PreflightTechnicalFailureReason::StateAccessFailure => {
                    "Preflight readiness could not compare current material inputs to the last result because shell state access failed"
                        .to_string()
                }
            }
        } else if current_result_stale {
            preflight_invalidation_note(&invalidation_reasons)
        } else {
            submission_state.detail_message
        },
        downgrade_acceptance_required,
        downgrade_acceptance_recorded,
        current_result_stale,
        run_start_eligible_now: match current_state {
            PreflightState::Approved => true,
            PreflightState::ApprovedWithDowngrade => downgrade_acceptance_recorded,
            _ => false,
        },
        run_start_blocked_without_eligible_preflight: match current_state {
            PreflightState::Approved => false,
            PreflightState::ApprovedWithDowngrade => !downgrade_acceptance_recorded,
            _ => true,
        },
    }
}

fn resolve_current_state(
    submission_status: PreflightSubmissionStatus,
    current_result_stale: bool,
    invalidation_check_failed: bool,
) -> PreflightState {
    if invalidation_check_failed {
        return PreflightState::Error;
    }

    if current_result_stale {
        return PreflightState::Invalidated;
    }

    match submission_status {
        PreflightSubmissionStatus::NotStarted => PreflightState::NotStarted,
        PreflightSubmissionStatus::Approved => PreflightState::Approved,
        PreflightSubmissionStatus::ApprovedWithDowngrade => PreflightState::ApprovedWithDowngrade,
        PreflightSubmissionStatus::Rejected => PreflightState::Rejected,
        PreflightSubmissionStatus::Error => PreflightState::Error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preflight_feasibility::PreflightEvaluationBasis;

    fn selected_seed_document() -> SelectedSeedDocumentSnapshot {
        SelectedSeedDocumentSnapshot {
            file_name: "seed.txt".to_string(),
            mime_type: Some("text/plain".to_string()),
            detected_kind: Some(crate::preflight_contract::SupportedSeedDocumentKind::Txt),
            size_bytes: 1_200,
            content_fingerprint_sha256: "abc123".to_string(),
            extracted_text_character_count: Some(1_200),
            english_confidence_percent: Some(99),
        }
    }

    fn requested_plan() -> PreflightRequestedPlanSnapshot {
        PreflightRequestedPlanSnapshot {
            requested_agent_count: 5,
            requested_round_count: 4,
            max_token_budget: 12_000,
        }
    }

    #[test]
    fn preflight_readiness_truth_surface_keeps_run_start_blocked_without_preflight() {
        let snapshot = preflight_readiness_truth_surface();
        let blocking_reasons = snapshot.blocking_reasons();

        assert!(snapshot.state_surface_materialized);
        assert!(snapshot.preflight_contract_defined);
        assert!(snapshot.requested_plan_contract_defined);
        assert!(snapshot.effective_plan_contract_defined);
        assert!(snapshot.invalidation_semantics_defined);
        assert!(snapshot.run_start_gate_truth_surface_materialized);
        assert!(snapshot.preflight_execution_materialized);
        assert!(snapshot.runtime_readiness_evaluated_separately);
        assert!(snapshot.activation_readiness_evaluated_separately);
        assert!(snapshot.provider_readiness_evaluated_separately);
        assert!(!snapshot.runtime_ready_now);
        assert!(!snapshot.activation_ready_now);
        assert!(!snapshot.provider_configuration_ready_now);
        assert!(!snapshot.pending_seed_document_selected);
        assert!(!snapshot.requested_plan_present);
        assert!(!snapshot.document_validation_completed);
        assert!(!snapshot.document_validation_passed);
        assert_eq!(snapshot.current_state, PreflightState::NotStarted);
        assert_eq!(snapshot.current_state_name, STATE_PREFLIGHT_NOT_STARTED);
        assert!(!snapshot.downgrade_acceptance_required);
        assert!(!snapshot.downgrade_acceptance_recorded);
        assert!(!snapshot.current_result_stale);
        assert!(!snapshot.run_start_eligible_now);
        assert!(snapshot.run_start_blocked_without_eligible_preflight);
        assert_eq!(snapshot.supported_states().len(), 7);
        assert_eq!(blocking_reasons.len(), 6);
    }

    #[test]
    fn approved_with_downgrade_requires_acceptance_before_run_start() {
        let snapshot = preflight_readiness_from_submission_state(PreflightSubmissionStateSnapshot {
            status: PreflightSubmissionStatus::ApprovedWithDowngrade,
            selected_seed_document: Some(selected_seed_document()),
            requested_plan: Some(requested_plan()),
            valid_document_in_pending_configuration: true,
            effective_plan: Some(PreflightEffectivePlanSnapshot {
                effective_agent_count: 5,
                effective_round_count: 3,
                provider: crate::provider_configuration_contract::SupportedProvider::OpenAi,
                model: "gpt-4o-mini".to_string(),
                max_token_budget: 12_000,
                bounded_speaker_model: "moderated_3_to_5_speakers_per_round".to_string(),
                bounded_concurrency_policy: "single_moderated_speaker_set_per_round".to_string(),
            }),
            estimated_usage: Some(PreflightEstimatedUsageSnapshot {
                estimated_total_tokens: 10_500,
                estimated_duration_minutes_low: 21,
                estimated_duration_minutes_high: 33,
            }),
            provider_model_context: None,
            requested_plan_budget_feasible: Some(false),
            downgrade_warning: Some("downgraded".to_string()),
            downgrade_acceptance_recorded: false,
            last_rejection_reason: None,
            last_technical_failure_reason: None,
            detail_message: "Preflight approved a downgraded effective plan".to_string(),
            evaluation_basis: None,
        });

        assert_eq!(snapshot.current_state, PreflightState::ApprovedWithDowngrade);
        assert!(snapshot.downgrade_acceptance_required);
        assert!(!snapshot.downgrade_acceptance_recorded);
        assert!(!snapshot.run_start_eligible_now);
    }

    #[test]
    fn materially_changed_requested_plan_invalidates_prior_result() {
        let snapshot = preflight_readiness_from_submission_state(PreflightSubmissionStateSnapshot {
            status: PreflightSubmissionStatus::Approved,
            selected_seed_document: Some(selected_seed_document()),
            requested_plan: Some(PreflightRequestedPlanSnapshot {
                requested_agent_count: 6,
                requested_round_count: 4,
                max_token_budget: 12_000,
            }),
            valid_document_in_pending_configuration: true,
            effective_plan: None,
            estimated_usage: None,
            provider_model_context: None,
            requested_plan_budget_feasible: Some(true),
            downgrade_warning: None,
            downgrade_acceptance_recorded: false,
            last_rejection_reason: None,
            last_technical_failure_reason: None,
            detail_message: "approved".to_string(),
            evaluation_basis: Some(PreflightEvaluationBasis {
                seed_document_fingerprint_sha256: "abc123".to_string(),
                requested_plan: requested_plan(),
                selected_provider: None,
                provider_secret_fingerprint_sha256: None,
                activation_ready_now: false,
                runtime_ready_now: false,
            }),
        });

        assert_eq!(snapshot.current_state, PreflightState::Invalidated);
        assert!(snapshot.current_result_stale);
        assert!(snapshot
            .invalidation_reasons
            .contains(&PreflightInvalidationReason::RequestedAgentCountChanged));
        assert!(!snapshot.run_start_eligible_now);
    }
}