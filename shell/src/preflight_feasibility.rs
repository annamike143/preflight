use sha2::{Digest, Sha256};

use crate::activation_readiness::activation_readiness_truth_surface;
use crate::preflight_contract::{
    PreflightEstimatedUsageSnapshot, PreflightEffectivePlanSnapshot,
    PreflightInvalidationReason, PreflightProviderModelContextSnapshot,
    PreflightRejectionReason, PreflightRequestedPlanSnapshot,
    PreflightTechnicalFailureReason,
};
use crate::preflight_document_validation::SelectedSeedDocumentSnapshot;
use crate::preflight_effective_plan::{default_model_for_provider, derive_effective_plan};
use crate::provider_configuration_contract::{ProviderConfigurationStatus, SupportedProvider};
use crate::provider_secure_store::{ProviderSecureStore, ProviderSecureStoreBackend};
use crate::provider_support_store::ProviderSupportStore;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightEvaluationDecision {
    Approved,
    ApprovedWithDowngrade,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightEvaluationBasis {
    pub seed_document_fingerprint_sha256: String,
    pub requested_plan: PreflightRequestedPlanSnapshot,
    pub selected_provider: Option<SupportedProvider>,
    pub provider_secret_fingerprint_sha256: Option<String>,
    pub activation_ready_now: bool,
    pub runtime_ready_now: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightEvaluatedOutcome {
    pub decision: PreflightEvaluationDecision,
    pub rejection_reason: Option<PreflightRejectionReason>,
    pub effective_plan: Option<PreflightEffectivePlanSnapshot>,
    pub estimated_usage: Option<PreflightEstimatedUsageSnapshot>,
    pub requested_plan_budget_feasible: Option<bool>,
    pub provider_model_context: Option<PreflightProviderModelContextSnapshot>,
    pub downgrade_warning: Option<String>,
    pub detail_message: String,
    pub evaluation_basis: PreflightEvaluationBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProviderPlanningStatus {
    Missing,
    Invalid,
    Available,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProviderPlanningComparison {
    status: ProviderPlanningStatus,
    selected_provider: Option<SupportedProvider>,
    provider_secret_fingerprint_sha256: Option<String>,
    provider_model_context: Option<PreflightProviderModelContextSnapshot>,
}

pub fn preflight_invalidation_reasons_for_state(
    evaluation_basis: &PreflightEvaluationBasis,
    current_selected_seed_document: Option<&SelectedSeedDocumentSnapshot>,
    current_requested_plan: Option<&PreflightRequestedPlanSnapshot>,
) -> Result<Vec<PreflightInvalidationReason>, PreflightTechnicalFailureReason> {
    let activation_readiness = activation_readiness_truth_surface();
    let secure_store = ProviderSecureStore::os_native();
    let support_store =
        ProviderSupportStore::default_store().map_err(|_| PreflightTechnicalFailureReason::StateAccessFailure)?;

    preflight_invalidation_reasons_with_components(
        evaluation_basis,
        current_selected_seed_document,
        current_requested_plan,
        &secure_store,
        &support_store,
        activation_readiness.activation_ready_for_licensed_use,
        activation_readiness.runtime_ready_now,
    )
}

pub fn preflight_rejection_message(reason: PreflightRejectionReason) -> &'static str {
    match reason {
        PreflightRejectionReason::UnsupportedFileType => {
            "Seed document was rejected because its file type is not approved for MVP use"
        }
        PreflightRejectionReason::FileTooLarge => {
            "Seed document was rejected because it exceeds the governed 10 MiB file-size ceiling"
        }
        PreflightRejectionReason::TextExtractionFailed => {
            "Seed document was rejected because extractable text could not be derived from the submitted file"
        }
        PreflightRejectionReason::InsufficientExtractableText => {
            "Seed document was rejected because it does not contain the governed minimum amount of extractable text"
        }
        PreflightRejectionReason::NonEnglishSeedDocument => {
            "Seed document was rejected because English confidence did not meet the governed minimum threshold"
        }
        PreflightRejectionReason::ActivationNotReady => {
            "Preflight rejected the run because activation is not currently valid for licensed use"
        }
        PreflightRejectionReason::RuntimeNotReady => {
            "Preflight rejected the run because runtime integrity is not currently ready"
        }
        PreflightRejectionReason::ProviderConfigurationMissing => {
            "Preflight rejected the run because provider configuration is incomplete for planning"
        }
        PreflightRejectionReason::ProviderConfigurationInvalid => {
            "Preflight rejected the run because the stored provider configuration could not be verified"
        }
        PreflightRejectionReason::RequestedAgentCountBelowMinimum => {
            "Preflight rejected the run because the requested agent count is below the governed minimum of 3"
        }
        PreflightRejectionReason::BudgetTooLowForMinimumPlan => {
            "Preflight rejected the run because the max token budget cannot support the minimum bounded plan"
        }
    }
}

pub fn preflight_invalidation_note(
    invalidation_reasons: &[PreflightInvalidationReason],
) -> String {
    let reasons = invalidation_reasons
        .iter()
        .map(|reason| reason.label())
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "Current Preflight result is stale because material inputs changed after evaluation: {}.",
        reasons
    )
}

pub fn evaluate_preflight_submission_with_components<B: ProviderSecureStoreBackend>(
    selected_seed_document: &SelectedSeedDocumentSnapshot,
    requested_plan: &PreflightRequestedPlanSnapshot,
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
    activation_ready_now: bool,
    runtime_ready_now: bool,
) -> Result<PreflightEvaluatedOutcome, PreflightTechnicalFailureReason> {
    let provider_planning = provider_planning_comparison_with_components(secure_store, support_store)?;
    let evaluation_basis = PreflightEvaluationBasis {
        seed_document_fingerprint_sha256: selected_seed_document.content_fingerprint_sha256.clone(),
        requested_plan: requested_plan.clone(),
        selected_provider: provider_planning.selected_provider,
        provider_secret_fingerprint_sha256: provider_planning.provider_secret_fingerprint_sha256.clone(),
        activation_ready_now,
        runtime_ready_now,
    };

    if !activation_ready_now {
        return Ok(rejected_outcome(
            PreflightRejectionReason::ActivationNotReady,
            provider_planning.provider_model_context,
            evaluation_basis,
        ));
    }

    if !runtime_ready_now {
        return Ok(rejected_outcome(
            PreflightRejectionReason::RuntimeNotReady,
            provider_planning.provider_model_context,
            evaluation_basis,
        ));
    }

    match provider_planning.status {
        ProviderPlanningStatus::Missing => Ok(rejected_outcome(
            PreflightRejectionReason::ProviderConfigurationMissing,
            provider_planning.provider_model_context,
            evaluation_basis,
        )),
        ProviderPlanningStatus::Invalid => Ok(rejected_outcome(
            PreflightRejectionReason::ProviderConfigurationInvalid,
            provider_planning.provider_model_context,
            evaluation_basis,
        )),
        ProviderPlanningStatus::Available => {
            let provider = provider_planning
                .selected_provider
                .expect("available planning must include a provider");
            let derivation = match derive_effective_plan(provider, requested_plan) {
                Ok(derivation) => derivation,
                Err(rejection_reason) => {
                    return Ok(rejected_outcome(
                        rejection_reason,
                        provider_planning.provider_model_context,
                        evaluation_basis,
                    ))
                }
            };

            let detail_message = if derivation.is_downgraded {
                "Preflight approved a downgraded effective plan under the bounded local provider/model planning basis"
                    .to_string()
            } else {
                "Preflight approved the requested plan under the bounded local provider/model planning basis"
                    .to_string()
            };

            Ok(PreflightEvaluatedOutcome {
                decision: if derivation.is_downgraded {
                    PreflightEvaluationDecision::ApprovedWithDowngrade
                } else {
                    PreflightEvaluationDecision::Approved
                },
                rejection_reason: None,
                effective_plan: Some(derivation.effective_plan),
                estimated_usage: Some(derivation.estimated_usage),
                requested_plan_budget_feasible: Some(derivation.requested_plan_budget_feasible),
                provider_model_context: provider_planning.provider_model_context,
                downgrade_warning: derivation.downgrade_warning,
                detail_message,
                evaluation_basis,
            })
        }
    }
}

pub fn preflight_invalidation_reasons_with_components<B: ProviderSecureStoreBackend>(
    evaluation_basis: &PreflightEvaluationBasis,
    current_selected_seed_document: Option<&SelectedSeedDocumentSnapshot>,
    current_requested_plan: Option<&PreflightRequestedPlanSnapshot>,
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
    activation_ready_now: bool,
    runtime_ready_now: bool,
) -> Result<Vec<PreflightInvalidationReason>, PreflightTechnicalFailureReason> {
    let current_provider_planning = provider_planning_comparison_with_components(secure_store, support_store)?;
    let mut reasons = Vec::new();

    let Some(current_selected_seed_document) = current_selected_seed_document else {
        reasons.push(PreflightInvalidationReason::SeedDocumentChanged);
        return Ok(reasons);
    };

    if current_selected_seed_document.content_fingerprint_sha256
        != evaluation_basis.seed_document_fingerprint_sha256
    {
        reasons.push(PreflightInvalidationReason::SeedDocumentChanged);
    }

    let Some(current_requested_plan) = current_requested_plan else {
        reasons.push(PreflightInvalidationReason::RequestedAgentCountChanged);
        reasons.push(PreflightInvalidationReason::RequestedRoundCountChanged);
        reasons.push(PreflightInvalidationReason::MaxTokenBudgetChanged);
        return Ok(reasons);
    };

    if current_requested_plan.requested_agent_count
        != evaluation_basis.requested_plan.requested_agent_count
    {
        reasons.push(PreflightInvalidationReason::RequestedAgentCountChanged);
    }

    if current_requested_plan.requested_round_count
        != evaluation_basis.requested_plan.requested_round_count
    {
        reasons.push(PreflightInvalidationReason::RequestedRoundCountChanged);
    }

    if current_requested_plan.max_token_budget != evaluation_basis.requested_plan.max_token_budget {
        reasons.push(PreflightInvalidationReason::MaxTokenBudgetChanged);
    }

    if current_provider_planning.selected_provider != evaluation_basis.selected_provider {
        reasons.push(PreflightInvalidationReason::ProviderChanged);
    } else if current_provider_planning.provider_secret_fingerprint_sha256
        != evaluation_basis.provider_secret_fingerprint_sha256
    {
        reasons.push(PreflightInvalidationReason::ActiveProviderKeyChanged);
    }

    if runtime_ready_now != evaluation_basis.runtime_ready_now {
        reasons.push(PreflightInvalidationReason::RuntimeValidityChanged);
    }

    if activation_ready_now != evaluation_basis.activation_ready_now {
        reasons.push(PreflightInvalidationReason::ActivationValidityChanged);
    }

    Ok(reasons)
}

fn rejected_outcome(
    rejection_reason: PreflightRejectionReason,
    provider_model_context: Option<PreflightProviderModelContextSnapshot>,
    evaluation_basis: PreflightEvaluationBasis,
) -> PreflightEvaluatedOutcome {
    PreflightEvaluatedOutcome {
        decision: PreflightEvaluationDecision::Rejected,
        rejection_reason: Some(rejection_reason),
        effective_plan: None,
        estimated_usage: None,
        requested_plan_budget_feasible: None,
        provider_model_context,
        downgrade_warning: None,
        detail_message: preflight_rejection_message(rejection_reason).to_string(),
        evaluation_basis,
    }
}

fn provider_planning_comparison_with_components<B: ProviderSecureStoreBackend>(
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
) -> Result<ProviderPlanningComparison, PreflightTechnicalFailureReason> {
    let support_record =
        support_store.load_record().map_err(|_| PreflightTechnicalFailureReason::StateAccessFailure)?;

    let Some(support_record) = support_record else {
        return Ok(ProviderPlanningComparison {
            status: ProviderPlanningStatus::Missing,
            selected_provider: None,
            provider_secret_fingerprint_sha256: None,
            provider_model_context: None,
        });
    };

    if support_record.secure_store_lookup_reference.trim().is_empty() {
        return Err(PreflightTechnicalFailureReason::StateAccessFailure);
    }

    let expected_lookup_reference = secure_store.lookup_reference(support_record.selected_provider);
    if support_record.secure_store_lookup_reference != expected_lookup_reference {
        return Err(PreflightTechnicalFailureReason::StateAccessFailure);
    }

    let provider_secret = secure_store
        .read_secret(support_record.selected_provider)
        .map_err(|_| PreflightTechnicalFailureReason::StateAccessFailure)?;
    let provider_secret_fingerprint_sha256 =
        provider_secret.as_deref().map(|secret| hex_sha256(secret.as_bytes()));

    let status = match support_record.configuration_status {
        ProviderConfigurationStatus::Present => {
            match provider_secret_fingerprint_sha256.as_deref() {
                Some(fingerprint)
                    if fingerprint == support_record.provider_secret_fingerprint_sha256 =>
                {
                    ProviderPlanningStatus::Available
                }
                Some(_) => ProviderPlanningStatus::Invalid,
                None => ProviderPlanningStatus::Missing,
            }
        }
        ProviderConfigurationStatus::Required => ProviderPlanningStatus::Missing,
        ProviderConfigurationStatus::Error => ProviderPlanningStatus::Invalid,
    };

    Ok(ProviderPlanningComparison {
        status,
        selected_provider: Some(support_record.selected_provider),
        provider_secret_fingerprint_sha256,
        provider_model_context: Some(provider_model_context(
            support_record.selected_provider,
            status,
        )),
    })
}

fn provider_model_context(
    provider: SupportedProvider,
    status: ProviderPlanningStatus,
) -> PreflightProviderModelContextSnapshot {
    let (local_configuration_supports_planning, detail_note) = match status {
        ProviderPlanningStatus::Available => (
            true,
            "Planning uses the bounded local provider selection, secure-store-backed key presence, and frozen default MVP model mapping only; live provider reachability is not probed at this layer.".to_string(),
        ),
        ProviderPlanningStatus::Missing => (
            false,
            "The selected provider does not currently have enough verified local configuration to support bounded Preflight planning.".to_string(),
        ),
        ProviderPlanningStatus::Invalid => (
            false,
            "The selected provider has local support metadata, but the active secure-store material could not be verified for bounded Preflight planning.".to_string(),
        ),
    };

    PreflightProviderModelContextSnapshot {
        provider,
        model: default_model_for_provider(provider).to_string(),
        local_configuration_supports_planning,
        live_reachability_probed: false,
        detail_note,
    }
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);

    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }

    output
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use tempfile::tempdir;

    use super::*;
    use crate::provider_configuration_contract::SupportedProvider;
    use crate::provider_secure_store::ProviderSecureStore;
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

    fn seed_document() -> SelectedSeedDocumentSnapshot {
        SelectedSeedDocumentSnapshot {
            file_name: "seed.txt".to_string(),
            mime_type: Some("text/plain".to_string()),
            detected_kind: Some(crate::preflight_contract::SupportedSeedDocumentKind::Txt),
            size_bytes: 1_400,
            content_fingerprint_sha256: "seed-fingerprint-001".to_string(),
            extracted_text_character_count: Some(1_400),
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
            provider_secret_fingerprint_sha256: hex_sha256(secret.as_bytes()),
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
    fn feasibility_can_approve_bounded_requested_plan() {
        let (secure_store, support_store) =
            configured_provider_environment(SupportedProvider::OpenAi, "sk-openai-001");

        let outcome = evaluate_preflight_submission_with_components(
            &seed_document(),
            &requested_plan(),
            &secure_store,
            &support_store,
            true,
            true,
        )
        .expect("evaluation should succeed");

        assert_eq!(outcome.decision, PreflightEvaluationDecision::Approved);
        assert!(outcome.rejection_reason.is_none());
        assert_eq!(
            outcome.effective_plan.as_ref().map(|plan| plan.model.as_str()),
            Some("gpt-4o-mini")
        );
        assert_eq!(
            outcome
                .provider_model_context
                .as_ref()
                .map(|context| context.local_configuration_supports_planning),
            Some(true)
        );
    }

    #[test]
    fn feasibility_can_downgrade_large_requested_scope() {
        let (secure_store, support_store) =
            configured_provider_environment(SupportedProvider::OpenAi, "sk-openai-001");
        let requested_plan = PreflightRequestedPlanSnapshot {
            requested_agent_count: 7,
            requested_round_count: 8,
            max_token_budget: 12_000,
        };

        let outcome = evaluate_preflight_submission_with_components(
            &seed_document(),
            &requested_plan,
            &secure_store,
            &support_store,
            true,
            true,
        )
        .expect("evaluation should succeed");

        assert_eq!(outcome.decision, PreflightEvaluationDecision::ApprovedWithDowngrade);
        assert!(outcome.downgrade_warning.is_some());
        assert!(outcome.effective_plan.is_some());
    }

    #[test]
    fn invalidation_reasons_detect_requested_plan_and_seed_changes() {
        let (secure_store, support_store) =
            configured_provider_environment(SupportedProvider::OpenAi, "sk-openai-001");
        let requested_plan = requested_plan();
        let evaluation_basis = PreflightEvaluationBasis {
            seed_document_fingerprint_sha256: seed_document().content_fingerprint_sha256,
            requested_plan: requested_plan.clone(),
            selected_provider: Some(SupportedProvider::OpenAi),
            provider_secret_fingerprint_sha256: Some(hex_sha256("sk-openai-001".as_bytes())),
            activation_ready_now: true,
            runtime_ready_now: true,
        };
        let changed_seed_document = SelectedSeedDocumentSnapshot {
            content_fingerprint_sha256: "seed-fingerprint-002".to_string(),
            ..seed_document()
        };
        let changed_requested_plan = PreflightRequestedPlanSnapshot {
            requested_round_count: 5,
            ..requested_plan
        };

        let reasons = preflight_invalidation_reasons_with_components(
            &evaluation_basis,
            Some(&changed_seed_document),
            Some(&changed_requested_plan),
            &secure_store,
            &support_store,
            true,
            true,
        )
        .expect("comparison should succeed");

        assert!(reasons.contains(&PreflightInvalidationReason::SeedDocumentChanged));
        assert!(reasons.contains(&PreflightInvalidationReason::RequestedRoundCountChanged));
    }
}