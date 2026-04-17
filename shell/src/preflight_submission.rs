use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};

use crate::activation_readiness::activation_readiness_truth_surface;
use crate::preflight_contract::{
    PreflightEstimatedUsageSnapshot, PreflightEffectivePlanSnapshot,
    PreflightProviderModelContextSnapshot, PreflightRejectionReason,
    PreflightRequestedPlanSnapshot, PreflightTechnicalFailureReason,
};
use crate::preflight_document_validation::{
    validate_seed_document, SeedDocumentValidationDecision, SelectedSeedDocumentSnapshot,
};
use crate::preflight_feasibility::{
    evaluate_preflight_submission_with_components, preflight_invalidation_note,
    preflight_invalidation_reasons_with_components, preflight_rejection_message,
    PreflightEvaluationBasis, PreflightEvaluationDecision,
};
use crate::provider_secure_store::{ProviderSecureStore, ProviderSecureStoreBackend};
use crate::provider_support_store::ProviderSupportStore;

static PREFLIGHT_SUBMISSION_STATE: OnceLock<Mutex<StoredPreflightSubmissionState>> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SeedDocumentSubmissionRequest {
    pub filename: String,
    pub mime_type: Option<String>,
    pub size_bytes: u64,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreflightSubmissionRequest {
    pub seed_document: SeedDocumentSubmissionRequest,
    pub requested_agent_count: u32,
    pub requested_round_count: u32,
    pub max_token_budget: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedSeedDocumentSubmissionRequest {
    filename: String,
    mime_type: Option<String>,
    bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedPreflightSubmissionRequest {
    seed_document: NormalizedSeedDocumentSubmissionRequest,
    requested_plan: PreflightRequestedPlanSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightSubmissionDecisionType {
    Approved,
    ApprovedWithDowngrade,
    Rejected,
    TechnicalFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightSubmissionStatus {
    NotStarted,
    Approved,
    ApprovedWithDowngrade,
    Rejected,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreflightSubmissionStateSnapshot {
    pub status: PreflightSubmissionStatus,
    pub selected_seed_document: Option<SelectedSeedDocumentSnapshot>,
    pub requested_plan: Option<PreflightRequestedPlanSnapshot>,
    pub valid_document_in_pending_configuration: bool,
    pub effective_plan: Option<PreflightEffectivePlanSnapshot>,
    pub estimated_usage: Option<PreflightEstimatedUsageSnapshot>,
    pub provider_model_context: Option<PreflightProviderModelContextSnapshot>,
    pub requested_plan_budget_feasible: Option<bool>,
    pub downgrade_warning: Option<String>,
    pub downgrade_acceptance_recorded: bool,
    pub last_rejection_reason: Option<PreflightRejectionReason>,
    pub last_technical_failure_reason: Option<PreflightTechnicalFailureReason>,
    pub detail_message: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub evaluation_basis: Option<PreflightEvaluationBasis>,
}

impl PreflightSubmissionStateSnapshot {
    pub fn state_access_failure() -> Self {
        Self {
            status: PreflightSubmissionStatus::Error,
            selected_seed_document: None,
            requested_plan: None,
            valid_document_in_pending_configuration: false,
            effective_plan: None,
            estimated_usage: None,
            provider_model_context: None,
            requested_plan_budget_feasible: None,
            downgrade_warning: None,
            downgrade_acceptance_recorded: false,
            last_rejection_reason: None,
            last_technical_failure_reason: Some(PreflightTechnicalFailureReason::StateAccessFailure),
            detail_message: "Preflight submission state could not be read from the shell".to_string(),
            evaluation_basis: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreflightSubmissionOutcome {
    pub decision_type: PreflightSubmissionDecisionType,
    pub rejection_reason: Option<PreflightRejectionReason>,
    pub technical_failure_reason: Option<PreflightTechnicalFailureReason>,
    pub submission_state: PreflightSubmissionStateSnapshot,
    pub detail_message: String,
}

impl PreflightSubmissionOutcome {
    fn approved(
        decision_type: PreflightSubmissionDecisionType,
        submission_state: PreflightSubmissionStateSnapshot,
    ) -> Self {
        Self {
            decision_type,
            rejection_reason: None,
            technical_failure_reason: None,
            detail_message: submission_state.detail_message.clone(),
            submission_state,
        }
    }

    fn rejected(
        rejection_reason: PreflightRejectionReason,
        submission_state: PreflightSubmissionStateSnapshot,
    ) -> Self {
        Self {
            decision_type: PreflightSubmissionDecisionType::Rejected,
            rejection_reason: Some(rejection_reason),
            technical_failure_reason: None,
            detail_message: submission_state.detail_message.clone(),
            submission_state,
        }
    }

    fn technical_failure(
        technical_failure_reason: PreflightTechnicalFailureReason,
        submission_state: PreflightSubmissionStateSnapshot,
        detail_message: impl Into<String>,
    ) -> Self {
        Self {
            decision_type: PreflightSubmissionDecisionType::TechnicalFailure,
            rejection_reason: None,
            technical_failure_reason: Some(technical_failure_reason),
            submission_state,
            detail_message: detail_message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreflightDowngradeAcceptanceOutcome {
    pub acceptance_recorded: bool,
    pub technical_failure_reason: Option<PreflightTechnicalFailureReason>,
    pub submission_state: PreflightSubmissionStateSnapshot,
    pub detail_message: String,
}

impl PreflightDowngradeAcceptanceOutcome {
    fn recorded(submission_state: PreflightSubmissionStateSnapshot) -> Self {
        Self {
            acceptance_recorded: true,
            technical_failure_reason: None,
            detail_message: submission_state.detail_message.clone(),
            submission_state,
        }
    }

    fn not_recorded(
        submission_state: PreflightSubmissionStateSnapshot,
        detail_message: impl Into<String>,
    ) -> Self {
        Self {
            acceptance_recorded: false,
            technical_failure_reason: None,
            submission_state,
            detail_message: detail_message.into(),
        }
    }

    fn technical_failure(
        technical_failure_reason: PreflightTechnicalFailureReason,
        submission_state: PreflightSubmissionStateSnapshot,
        detail_message: impl Into<String>,
    ) -> Self {
        Self {
            acceptance_recorded: false,
            technical_failure_reason: Some(technical_failure_reason),
            submission_state,
            detail_message: detail_message.into(),
        }
    }
}

#[derive(Debug, Clone)]
struct StoredSelectedSeedDocument {
    snapshot: SelectedSeedDocumentSnapshot,
    raw_bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
struct StoredPreflightSubmissionState {
    status: PreflightSubmissionStatus,
    selected_seed_document: Option<StoredSelectedSeedDocument>,
    requested_plan: Option<PreflightRequestedPlanSnapshot>,
    valid_document_in_pending_configuration: bool,
    effective_plan: Option<PreflightEffectivePlanSnapshot>,
    estimated_usage: Option<PreflightEstimatedUsageSnapshot>,
    provider_model_context: Option<PreflightProviderModelContextSnapshot>,
    requested_plan_budget_feasible: Option<bool>,
    downgrade_warning: Option<String>,
    downgrade_acceptance_recorded: bool,
    evaluation_basis: Option<PreflightEvaluationBasis>,
    last_rejection_reason: Option<PreflightRejectionReason>,
    last_technical_failure_reason: Option<PreflightTechnicalFailureReason>,
    detail_message: String,
}

struct RejectedSubmissionData {
    requested_plan: PreflightRequestedPlanSnapshot,
    selected_seed_document: SelectedSeedDocumentSnapshot,
    raw_bytes: Option<Vec<u8>>,
    rejection_reason: PreflightRejectionReason,
    valid_document_in_pending_configuration: bool,
    effective_plan: Option<PreflightEffectivePlanSnapshot>,
    estimated_usage: Option<PreflightEstimatedUsageSnapshot>,
    provider_model_context: Option<PreflightProviderModelContextSnapshot>,
    requested_plan_budget_feasible: Option<bool>,
    downgrade_acceptance_recorded: bool,
    evaluation_basis: Option<PreflightEvaluationBasis>,
    detail_message: String,
}

impl Default for StoredPreflightSubmissionState {
    fn default() -> Self {
        Self {
            status: PreflightSubmissionStatus::NotStarted,
            selected_seed_document: None,
            requested_plan: None,
            valid_document_in_pending_configuration: false,
            effective_plan: None,
            estimated_usage: None,
            provider_model_context: None,
            requested_plan_budget_feasible: None,
            downgrade_warning: None,
            downgrade_acceptance_recorded: false,
            evaluation_basis: None,
            last_rejection_reason: None,
            last_technical_failure_reason: None,
            detail_message: "Preflight has not received a seed document submission yet".to_string(),
        }
    }
}

impl StoredPreflightSubmissionState {
    fn snapshot(&self) -> PreflightSubmissionStateSnapshot {
        PreflightSubmissionStateSnapshot {
            status: self.status,
            selected_seed_document: self
                .selected_seed_document
                .as_ref()
                .map(|selected_seed_document| selected_seed_document.snapshot.clone()),
            requested_plan: self.requested_plan.clone(),
            valid_document_in_pending_configuration: self.valid_document_in_pending_configuration,
            effective_plan: self.effective_plan.clone(),
            estimated_usage: self.estimated_usage.clone(),
            provider_model_context: self.provider_model_context.clone(),
            requested_plan_budget_feasible: self.requested_plan_budget_feasible,
            downgrade_warning: self.downgrade_warning.clone(),
            downgrade_acceptance_recorded: self.downgrade_acceptance_recorded,
            last_rejection_reason: self.last_rejection_reason,
            last_technical_failure_reason: self.last_technical_failure_reason,
            detail_message: self.detail_message.clone(),
            evaluation_basis: self.evaluation_basis.clone(),
        }
    }
}

pub fn preflight_submission_state_from_shell() -> PreflightSubmissionStateSnapshot {
    match preflight_submission_state_store().lock() {
        Ok(state) => state.snapshot(),
        Err(_) => PreflightSubmissionStateSnapshot::state_access_failure(),
    }
}

pub(crate) fn current_selected_seed_document_bytes_from_shell(
) -> Result<Option<Vec<u8>>, PreflightTechnicalFailureReason> {
    match preflight_submission_state_store().lock() {
        Ok(state) => Ok(state
            .selected_seed_document
            .as_ref()
            .and_then(|selected_seed_document| selected_seed_document.raw_bytes.clone())),
        Err(_) => Err(PreflightTechnicalFailureReason::StateAccessFailure),
    }
}

pub fn submit_preflight_from_shell(
    request: &PreflightSubmissionRequest,
) -> PreflightSubmissionOutcome {
    let mut state = match preflight_submission_state_store().lock() {
        Ok(state) => state,
        Err(_) => {
            return PreflightSubmissionOutcome::technical_failure(
                PreflightTechnicalFailureReason::StateAccessFailure,
                PreflightSubmissionStateSnapshot::state_access_failure(),
                "Preflight submission state could not be acquired from the shell",
            )
        }
    };

    let activation_readiness = activation_readiness_truth_surface();
    let secure_store = ProviderSecureStore::os_native();
    let support_store = match ProviderSupportStore::default_store() {
        Ok(support_store) => support_store,
        Err(_) => {
            return PreflightSubmissionOutcome::technical_failure(
                PreflightTechnicalFailureReason::StateAccessFailure,
                state.snapshot(),
                "provider planning state could not be resolved from the shell",
            )
        }
    };

    submit_preflight_with_components(
        request,
        &mut state,
        &secure_store,
        &support_store,
        activation_readiness.activation_ready_for_licensed_use,
        activation_readiness.runtime_ready_now,
    )
}

pub fn accept_preflight_downgrade_from_shell() -> PreflightDowngradeAcceptanceOutcome {
    let mut state = match preflight_submission_state_store().lock() {
        Ok(state) => state,
        Err(_) => {
            return PreflightDowngradeAcceptanceOutcome::technical_failure(
                PreflightTechnicalFailureReason::StateAccessFailure,
                PreflightSubmissionStateSnapshot::state_access_failure(),
                "Preflight submission state could not be acquired from the shell",
            )
        }
    };

    let activation_readiness = activation_readiness_truth_surface();
    let secure_store = ProviderSecureStore::os_native();
    let support_store = match ProviderSupportStore::default_store() {
        Ok(support_store) => support_store,
        Err(_) => {
            return PreflightDowngradeAcceptanceOutcome::technical_failure(
                PreflightTechnicalFailureReason::StateAccessFailure,
                state.snapshot(),
                "provider planning state could not be resolved from the shell",
            )
        }
    };

    accept_preflight_downgrade_with_components(
        &mut state,
        &secure_store,
        &support_store,
        activation_readiness.activation_ready_for_licensed_use,
        activation_readiness.runtime_ready_now,
    )
}

fn submit_preflight_with_components<B: ProviderSecureStoreBackend>(
    request: &PreflightSubmissionRequest,
    state: &mut StoredPreflightSubmissionState,
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
    activation_ready_now: bool,
    runtime_ready_now: bool,
) -> PreflightSubmissionOutcome {
    let normalized_request = match normalize_submission_request(request) {
        Ok(normalized_request) => normalized_request,
        Err(detail_message) => {
            return PreflightSubmissionOutcome::technical_failure(
                PreflightTechnicalFailureReason::MalformedInput,
                state.snapshot(),
                detail_message,
            )
        }
    };

    match validate_seed_document(
        &normalized_request.seed_document.filename,
        normalized_request.seed_document.mime_type.as_deref(),
        &normalized_request.seed_document.bytes,
    ) {
        SeedDocumentValidationDecision::Accepted(validated_seed_document) => {
            let evaluated_outcome = match evaluate_preflight_submission_with_components(
                &validated_seed_document.snapshot,
                &normalized_request.requested_plan,
                secure_store,
                support_store,
                activation_ready_now,
                runtime_ready_now,
            ) {
                Ok(evaluated_outcome) => evaluated_outcome,
                Err(technical_failure_reason) => {
                    return PreflightSubmissionOutcome::technical_failure(
                        technical_failure_reason,
                        state.snapshot(),
                        "Preflight planning state could not be evaluated inside the shell",
                    )
                }
            };

            apply_evaluated_submission(
                state,
                &normalized_request,
                validated_seed_document.snapshot,
                evaluated_outcome,
            )
        }
        SeedDocumentValidationDecision::Rejected(rejected_seed_document) => apply_rejected_submission(
            state,
            RejectedSubmissionData {
                requested_plan: normalized_request.requested_plan,
                selected_seed_document: rejected_seed_document.snapshot,
                raw_bytes: None,
                rejection_reason: PreflightRejectionReason::from_document_rejection(
                    rejected_seed_document.rejection_reason,
                ),
                valid_document_in_pending_configuration: false,
                effective_plan: None,
                estimated_usage: None,
                provider_model_context: None,
                requested_plan_budget_feasible: None,
                downgrade_acceptance_recorded: false,
                evaluation_basis: None,
                detail_message: preflight_rejection_message(
                    PreflightRejectionReason::from_document_rejection(
                        rejected_seed_document.rejection_reason,
                    ),
                )
                .to_string(),
            },
        ),
    }
}

fn accept_preflight_downgrade_with_components<B: ProviderSecureStoreBackend>(
    state: &mut StoredPreflightSubmissionState,
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
    activation_ready_now: bool,
    runtime_ready_now: bool,
) -> PreflightDowngradeAcceptanceOutcome {
    if state.status != PreflightSubmissionStatus::ApprovedWithDowngrade {
        return PreflightDowngradeAcceptanceOutcome::not_recorded(
            state.snapshot(),
            "Current Preflight result does not require downgrade acceptance",
        );
    }

    if state.downgrade_acceptance_recorded {
        return PreflightDowngradeAcceptanceOutcome::recorded(state.snapshot());
    }

    let Some(evaluation_basis) = state.evaluation_basis.as_ref() else {
        return PreflightDowngradeAcceptanceOutcome::technical_failure(
            PreflightTechnicalFailureReason::StateAccessFailure,
            state.snapshot(),
            "Preflight downgrade acceptance could not be matched to a stored evaluation basis",
        );
    };

    let invalidation_reasons = match preflight_invalidation_reasons_with_components(
        evaluation_basis,
        state.selected_seed_document.as_ref().map(|document| &document.snapshot),
        state.requested_plan.as_ref(),
        secure_store,
        support_store,
        activation_ready_now,
        runtime_ready_now,
    ) {
        Ok(invalidation_reasons) => invalidation_reasons,
        Err(technical_failure_reason) => {
            return PreflightDowngradeAcceptanceOutcome::technical_failure(
                technical_failure_reason,
                state.snapshot(),
                "Preflight downgrade freshness could not be verified inside the shell",
            )
        }
    };

    if !invalidation_reasons.is_empty() {
        return PreflightDowngradeAcceptanceOutcome::not_recorded(
            state.snapshot(),
            preflight_invalidation_note(&invalidation_reasons),
        );
    }

    state.downgrade_acceptance_recorded = true;
    state.detail_message =
        "Downgraded effective plan was explicitly accepted; run start now depends on the result staying current"
            .to_string();

    PreflightDowngradeAcceptanceOutcome::recorded(state.snapshot())
}

fn apply_evaluated_submission(
    state: &mut StoredPreflightSubmissionState,
    normalized_request: &NormalizedPreflightSubmissionRequest,
    selected_seed_document: SelectedSeedDocumentSnapshot,
    evaluated_outcome: crate::preflight_feasibility::PreflightEvaluatedOutcome,
) -> PreflightSubmissionOutcome {
    match evaluated_outcome.decision {
        PreflightEvaluationDecision::Approved => {
            state.status = PreflightSubmissionStatus::Approved;
            state.selected_seed_document = Some(StoredSelectedSeedDocument {
                snapshot: selected_seed_document,
                raw_bytes: Some(normalized_request.seed_document.bytes.clone()),
            });
            state.requested_plan = Some(normalized_request.requested_plan.clone());
            state.valid_document_in_pending_configuration = true;
            state.effective_plan = evaluated_outcome.effective_plan;
            state.estimated_usage = evaluated_outcome.estimated_usage;
            state.provider_model_context = evaluated_outcome.provider_model_context;
            state.requested_plan_budget_feasible = evaluated_outcome.requested_plan_budget_feasible;
            state.downgrade_warning = None;
            state.downgrade_acceptance_recorded = false;
            state.evaluation_basis = Some(evaluated_outcome.evaluation_basis);
            state.last_rejection_reason = None;
            state.last_technical_failure_reason = None;
            state.detail_message = evaluated_outcome.detail_message;

            PreflightSubmissionOutcome::approved(
                PreflightSubmissionDecisionType::Approved,
                state.snapshot(),
            )
        }
        PreflightEvaluationDecision::ApprovedWithDowngrade => {
            state.status = PreflightSubmissionStatus::ApprovedWithDowngrade;
            state.selected_seed_document = Some(StoredSelectedSeedDocument {
                snapshot: selected_seed_document,
                raw_bytes: Some(normalized_request.seed_document.bytes.clone()),
            });
            state.requested_plan = Some(normalized_request.requested_plan.clone());
            state.valid_document_in_pending_configuration = true;
            state.effective_plan = evaluated_outcome.effective_plan;
            state.estimated_usage = evaluated_outcome.estimated_usage;
            state.provider_model_context = evaluated_outcome.provider_model_context;
            state.requested_plan_budget_feasible = evaluated_outcome.requested_plan_budget_feasible;
            state.downgrade_warning = evaluated_outcome.downgrade_warning;
            state.downgrade_acceptance_recorded = false;
            state.evaluation_basis = Some(evaluated_outcome.evaluation_basis);
            state.last_rejection_reason = None;
            state.last_technical_failure_reason = None;
            state.detail_message = evaluated_outcome.detail_message;

            PreflightSubmissionOutcome::approved(
                PreflightSubmissionDecisionType::ApprovedWithDowngrade,
                state.snapshot(),
            )
        }
        PreflightEvaluationDecision::Rejected => apply_rejected_submission(
            state,
            RejectedSubmissionData {
                requested_plan: normalized_request.requested_plan.clone(),
                selected_seed_document,
                raw_bytes: Some(normalized_request.seed_document.bytes.clone()),
                rejection_reason: evaluated_outcome
                    .rejection_reason
                    .expect("rejected evaluation must carry a rejection reason"),
                valid_document_in_pending_configuration: true,
                effective_plan: evaluated_outcome.effective_plan,
                estimated_usage: evaluated_outcome.estimated_usage,
                provider_model_context: evaluated_outcome.provider_model_context,
                requested_plan_budget_feasible: evaluated_outcome.requested_plan_budget_feasible,
                downgrade_acceptance_recorded: false,
                evaluation_basis: Some(evaluated_outcome.evaluation_basis),
                detail_message: evaluated_outcome.detail_message,
            },
        ),
    }
}

fn apply_rejected_submission(
    state: &mut StoredPreflightSubmissionState,
    payload: RejectedSubmissionData,
) -> PreflightSubmissionOutcome {
    state.status = PreflightSubmissionStatus::Rejected;
    state.selected_seed_document = Some(StoredSelectedSeedDocument {
        snapshot: payload.selected_seed_document,
        raw_bytes: payload.raw_bytes,
    });
    state.requested_plan = Some(payload.requested_plan);
    state.valid_document_in_pending_configuration = payload.valid_document_in_pending_configuration;
    state.effective_plan = payload.effective_plan;
    state.estimated_usage = payload.estimated_usage;
    state.provider_model_context = payload.provider_model_context;
    state.requested_plan_budget_feasible = payload.requested_plan_budget_feasible;
    state.downgrade_warning = None;
    state.downgrade_acceptance_recorded = payload.downgrade_acceptance_recorded;
    state.evaluation_basis = payload.evaluation_basis;
    state.last_rejection_reason = Some(payload.rejection_reason);
    state.last_technical_failure_reason = None;
    state.detail_message = payload.detail_message;

    PreflightSubmissionOutcome::rejected(payload.rejection_reason, state.snapshot())
}

fn normalize_submission_request(
    request: &PreflightSubmissionRequest,
) -> Result<NormalizedPreflightSubmissionRequest, String> {
    let filename = request.seed_document.filename.trim().to_string();
    if filename.is_empty() {
        return Err("seed document filename is required".to_string());
    }

    if request.requested_agent_count == 0 {
        return Err("requested agent count must be greater than zero".to_string());
    }

    if request.requested_round_count == 0 {
        return Err("requested round count must be greater than zero".to_string());
    }

    if request.max_token_budget == 0 {
        return Err("max token budget must be greater than zero".to_string());
    }

    let actual_size_bytes = request.seed_document.bytes.len() as u64;
    if request.seed_document.size_bytes != actual_size_bytes {
        return Err(format!(
            "declared seed document size {} bytes does not match payload size {} bytes",
            request.seed_document.size_bytes, actual_size_bytes
        ));
    }

    Ok(NormalizedPreflightSubmissionRequest {
        seed_document: NormalizedSeedDocumentSubmissionRequest {
            filename,
            mime_type: normalize_optional_text(request.seed_document.mime_type.as_deref()),
            bytes: request.seed_document.bytes.clone(),
        },
        requested_plan: PreflightRequestedPlanSnapshot {
            requested_agent_count: request.requested_agent_count,
            requested_round_count: request.requested_round_count,
            max_token_budget: request.max_token_budget,
        },
    })
}

fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn preflight_submission_state_store() -> &'static Mutex<StoredPreflightSubmissionState> {
    PREFLIGHT_SUBMISSION_STATE.get_or_init(|| Mutex::new(StoredPreflightSubmissionState::default()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use tempfile::tempdir;

    use super::*;
    use crate::provider_configuration_contract::{ProviderConfigurationStatus, SupportedProvider};
    use crate::provider_support_store::{
        ProviderConfigurationOutcomeMarker, ProviderSupportStateRecord,
        PROVIDER_SUPPORT_RECORD_FILE_NAME, PROVIDER_SUPPORT_STATE_SCHEMA_VERSION,
    };

    #[derive(Clone, Default)]
    struct InMemorySecureStoreBackend {
        secrets: Arc<Mutex<HashMap<String, String>>>,
    }

    impl ProviderSecureStoreBackend for InMemorySecureStoreBackend {
        fn set_secret(
            &self,
            _service_name: &str,
            lookup_reference: &str,
            secret: &str,
        ) -> Result<(), String> {
            self.secrets
                .lock()
                .expect("lock poisoned")
                .insert(lookup_reference.to_string(), secret.to_string());
            Ok(())
        }

        fn get_secret(
            &self,
            _service_name: &str,
            lookup_reference: &str,
        ) -> Result<Option<String>, String> {
            Ok(self
                .secrets
                .lock()
                .expect("lock poisoned")
                .get(lookup_reference)
                .cloned())
        }

        fn delete_secret(&self, _service_name: &str, lookup_reference: &str) -> Result<(), String> {
            self.secrets
                .lock()
                .expect("lock poisoned")
                .remove(lookup_reference);
            Ok(())
        }
    }

    fn english_seed_bytes() -> Vec<u8> {
        "This is a governed English seed document used to exercise deterministic submission behavior. "
            .repeat(20)
            .into_bytes()
    }

    fn valid_request() -> PreflightSubmissionRequest {
        let bytes = english_seed_bytes();

        PreflightSubmissionRequest {
            seed_document: SeedDocumentSubmissionRequest {
                filename: "seed.txt".to_string(),
                mime_type: Some("text/plain".to_string()),
                size_bytes: bytes.len() as u64,
                bytes,
            },
            requested_agent_count: 5,
            requested_round_count: 4,
            max_token_budget: 12_000,
        }
    }

    fn configured_provider_environment(
        provider: SupportedProvider,
        secret: &str,
    ) -> (ProviderSecureStore<InMemorySecureStoreBackend>, ProviderSupportStore) {
        let secure_store = ProviderSecureStore::with_backend(InMemorySecureStoreBackend::default());
        secure_store
            .commit_secret(provider, secret)
            .expect("secret should commit");

        let temp_dir = tempdir().expect("temp dir should exist");
        let record_path = temp_dir.path().join(PROVIDER_SUPPORT_RECORD_FILE_NAME);
        std::mem::forget(temp_dir);

        let support_store = ProviderSupportStore::from_record_path(record_path);
        let record = ProviderSupportStateRecord {
            schema_version: PROVIDER_SUPPORT_STATE_SCHEMA_VERSION,
            selected_provider: provider,
            secure_store_lookup_reference: secure_store.lookup_reference(provider),
            provider_secret_fingerprint_sha256: {
                use sha2::{Digest, Sha256};

                let digest = Sha256::digest(secret.as_bytes());
                let mut output = String::with_capacity(digest.len() * 2);
                for byte in digest {
                    output.push_str(&format!("{byte:02x}"));
                }
                output
            },
            configuration_status: ProviderConfigurationStatus::Present,
            last_configuration_outcome: ProviderConfigurationOutcomeMarker::ConfiguredLocally,
            last_updated_timestamp_epoch_seconds: 100,
        };

        support_store
            .persist_record(&record)
            .expect("support record should persist");

        (secure_store, support_store)
    }

    #[test]
    fn approved_submission_persists_effective_plan_and_requested_plan() {
        let mut state = StoredPreflightSubmissionState::default();
        let (secure_store, support_store) =
            configured_provider_environment(SupportedProvider::OpenAi, "sk-openai-001");

        let outcome = submit_preflight_with_components(
            &valid_request(),
            &mut state,
            &secure_store,
            &support_store,
            true,
            true,
        );

        assert_eq!(outcome.decision_type, PreflightSubmissionDecisionType::Approved);
        assert!(outcome.rejection_reason.is_none());
        assert_eq!(outcome.submission_state.status, PreflightSubmissionStatus::Approved);
        assert!(outcome.submission_state.valid_document_in_pending_configuration);
        assert_eq!(
            outcome
                .submission_state
                .effective_plan
                .as_ref()
                .map(|plan| plan.model.as_str()),
            Some("gpt-4o-mini")
        );
        assert_eq!(
            outcome
                .submission_state
                .requested_plan
                .as_ref()
                .map(|requested_plan| requested_plan.max_token_budget),
            Some(12_000)
        );
    }

    #[test]
    fn downgraded_submission_requires_and_records_explicit_acceptance() {
        let mut state = StoredPreflightSubmissionState::default();
        let (secure_store, support_store) =
            configured_provider_environment(SupportedProvider::OpenAi, "sk-openai-001");
        let mut request = valid_request();
        request.requested_agent_count = 7;
        request.requested_round_count = 8;

        let outcome = submit_preflight_with_components(
            &request,
            &mut state,
            &secure_store,
            &support_store,
            true,
            true,
        );

        assert_eq!(
            outcome.decision_type,
            PreflightSubmissionDecisionType::ApprovedWithDowngrade
        );
        assert_eq!(
            outcome.submission_state.status,
            PreflightSubmissionStatus::ApprovedWithDowngrade
        );
        assert!(!outcome.submission_state.downgrade_acceptance_recorded);

        let acceptance_outcome = accept_preflight_downgrade_with_components(
            &mut state,
            &secure_store,
            &support_store,
            true,
            true,
        );

        assert!(acceptance_outcome.acceptance_recorded);
        assert!(acceptance_outcome.submission_state.downgrade_acceptance_recorded);
    }

    #[test]
    fn rejected_document_submission_replaces_prior_result_without_accumulating() {
        let mut state = StoredPreflightSubmissionState::default();
        let (secure_store, support_store) =
            configured_provider_environment(SupportedProvider::OpenAi, "sk-openai-001");
        let first_outcome = submit_preflight_with_components(
            &valid_request(),
            &mut state,
            &secure_store,
            &support_store,
            true,
            true,
        );
        assert_eq!(first_outcome.submission_state.status, PreflightSubmissionStatus::Approved);

        let mut unsupported_request = valid_request();
        unsupported_request.seed_document.filename = "seed.csv".to_string();
        unsupported_request.seed_document.mime_type = Some("text/csv".to_string());
        let second_outcome = submit_preflight_with_components(
            &unsupported_request,
            &mut state,
            &secure_store,
            &support_store,
            true,
            true,
        );

        assert_eq!(second_outcome.decision_type, PreflightSubmissionDecisionType::Rejected);
        assert_eq!(
            second_outcome.rejection_reason,
            Some(PreflightRejectionReason::UnsupportedFileType)
        );
        assert_eq!(
            second_outcome
                .submission_state
                .selected_seed_document
                .as_ref()
                .map(|seed_document| seed_document.file_name.as_str()),
            Some("seed.csv")
        );
        assert!(!second_outcome.submission_state.valid_document_in_pending_configuration);
    }

    #[test]
    fn malformed_input_returns_technical_failure_without_mutating_existing_state() {
        let mut state = StoredPreflightSubmissionState::default();
        let (secure_store, support_store) =
            configured_provider_environment(SupportedProvider::OpenAi, "sk-openai-001");
        let approved_outcome = submit_preflight_with_components(
            &valid_request(),
            &mut state,
            &secure_store,
            &support_store,
            true,
            true,
        );
        assert_eq!(approved_outcome.decision_type, PreflightSubmissionDecisionType::Approved);

        let mut malformed_request = valid_request();
        malformed_request.requested_agent_count = 0;
        let failure_outcome = submit_preflight_with_components(
            &malformed_request,
            &mut state,
            &secure_store,
            &support_store,
            true,
            true,
        );

        assert_eq!(
            failure_outcome.decision_type,
            PreflightSubmissionDecisionType::TechnicalFailure
        );
        assert_eq!(
            failure_outcome.technical_failure_reason,
            Some(PreflightTechnicalFailureReason::MalformedInput)
        );
        assert_eq!(
            state
                .snapshot()
                .selected_seed_document
                .as_ref()
                .map(|seed_document| seed_document.file_name.as_str()),
            Some("seed.txt")
        );
    }
}