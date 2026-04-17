use serde::{Deserialize, Serialize};

use crate::provider_configuration_contract::SupportedProvider;

pub const PREFLIGHT_CONTRACT_NAME: &str = "PreflightContract";
pub const PREFLIGHT_COMMAND_NAME: &str = "SubmitPreflight";
pub const PREFLIGHT_DOWNGRADE_ACCEPTANCE_COMMAND_NAME: &str = "AcceptPreflightDowngrade";
pub const MAX_SEED_DOCUMENT_SIZE_BYTES: u64 = 10_485_760;
pub const MINIMUM_EXTRACTABLE_TEXT_CHARACTERS: usize = 1_000;
pub const MINIMUM_ENGLISH_CONFIDENCE_PERCENT: u8 = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightContractStage {
    FeasibilityAndInvalidationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedSeedDocumentKind {
    Pdf,
    Docx,
    Txt,
    Markdown,
}

impl SupportedSeedDocumentKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Docx => "docx",
            Self::Txt => "txt",
            Self::Markdown => "markdown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightDocumentRejectionReason {
    UnsupportedFileType,
    FileTooLarge,
    TextExtractionFailed,
    InsufficientExtractableText,
    NonEnglishSeedDocument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightRejectionReason {
    UnsupportedFileType,
    FileTooLarge,
    TextExtractionFailed,
    InsufficientExtractableText,
    NonEnglishSeedDocument,
    ActivationNotReady,
    RuntimeNotReady,
    ProviderConfigurationMissing,
    ProviderConfigurationInvalid,
    RequestedAgentCountBelowMinimum,
    BudgetTooLowForMinimumPlan,
}

impl PreflightRejectionReason {
    pub fn label(self) -> &'static str {
        match self {
            Self::UnsupportedFileType => "unsupported_file_type",
            Self::FileTooLarge => "file_too_large",
            Self::TextExtractionFailed => "text_extraction_failed",
            Self::InsufficientExtractableText => "insufficient_extractable_text",
            Self::NonEnglishSeedDocument => "non_english_seed_document",
            Self::ActivationNotReady => "activation_not_ready",
            Self::RuntimeNotReady => "runtime_not_ready",
            Self::ProviderConfigurationMissing => "provider_configuration_missing",
            Self::ProviderConfigurationInvalid => "provider_configuration_invalid",
            Self::RequestedAgentCountBelowMinimum => "requested_agent_count_below_minimum",
            Self::BudgetTooLowForMinimumPlan => "budget_too_low_for_minimum_plan",
        }
    }

    pub fn from_document_rejection(reason: PreflightDocumentRejectionReason) -> Self {
        match reason {
            PreflightDocumentRejectionReason::UnsupportedFileType => Self::UnsupportedFileType,
            PreflightDocumentRejectionReason::FileTooLarge => Self::FileTooLarge,
            PreflightDocumentRejectionReason::TextExtractionFailed => Self::TextExtractionFailed,
            PreflightDocumentRejectionReason::InsufficientExtractableText => {
                Self::InsufficientExtractableText
            }
            PreflightDocumentRejectionReason::NonEnglishSeedDocument => {
                Self::NonEnglishSeedDocument
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightTechnicalFailureReason {
    MalformedInput,
    StateAccessFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum PreflightInvalidationReason {
    SeedDocumentChanged,
    ProviderChanged,
    RequestedAgentCountChanged,
    RequestedRoundCountChanged,
    MaxTokenBudgetChanged,
    ActiveProviderKeyChanged,
    RuntimeValidityChanged,
    ActivationValidityChanged,
}

impl PreflightInvalidationReason {
    pub fn label(self) -> &'static str {
        match self {
            Self::SeedDocumentChanged => "seed_document_changed",
            Self::ProviderChanged => "provider_changed",
            Self::RequestedAgentCountChanged => "requested_agent_count_changed",
            Self::RequestedRoundCountChanged => "requested_round_count_changed",
            Self::MaxTokenBudgetChanged => "max_token_budget_changed",
            Self::ActiveProviderKeyChanged => "active_provider_key_changed",
            Self::RuntimeValidityChanged => "runtime_validity_changed",
            Self::ActivationValidityChanged => "activation_validity_changed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreflightRequestedPlanSnapshot {
    pub requested_agent_count: u32,
    pub requested_round_count: u32,
    pub max_token_budget: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreflightEffectivePlanSnapshot {
    pub effective_agent_count: u32,
    pub effective_round_count: u32,
    pub provider: SupportedProvider,
    pub model: String,
    pub max_token_budget: u64,
    pub bounded_speaker_model: String,
    pub bounded_concurrency_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreflightEstimatedUsageSnapshot {
    pub estimated_total_tokens: u64,
    pub estimated_duration_minutes_low: u32,
    pub estimated_duration_minutes_high: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreflightProviderModelContextSnapshot {
    pub provider: SupportedProvider,
    pub model: String,
    pub local_configuration_supports_planning: bool,
    pub live_reachability_probed: bool,
    pub detail_note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeedIntakeRequestContract {
    pub single_seed_document_required: bool,
    pub supported_file_type_boundary_required: bool,
    pub supported_seed_document_kinds: [SupportedSeedDocumentKind; 4],
    pub max_file_size_boundary_required: bool,
    pub maximum_file_size_bytes: u64,
    pub minimum_extractable_text_required: bool,
    pub minimum_extractable_text_characters: usize,
    pub english_validation_required: bool,
    pub minimum_english_confidence_percent: u8,
    pub file_metadata_capture_required: bool,
    pub requested_agent_count_required: bool,
    pub requested_round_count_required: bool,
    pub max_token_budget_required: bool,
    pub shell_command_boundary_required: bool,
    pub durable_raw_seed_storage_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestedPlanContract {
    pub requested_plan_echo_required: bool,
    pub effective_plan_fields_supported: bool,
    pub estimated_usage_supported: bool,
    pub provider_model_context_supported: bool,
    pub downgrade_disclosure_required: bool,
    pub silent_requested_plan_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreflightResponseContract {
    pub approved_supported: bool,
    pub approved_with_downgrade_supported: bool,
    pub rejected_supported: bool,
    pub technical_failure_distinct_from_rejection: bool,
    pub rejection_reason_required_when_known: bool,
    pub known_rejection_reasons: [PreflightRejectionReason; 11],
    pub stale_invalidation_semantics_supported: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreflightInvalidationContract {
    pub seed_document_change_invalidates: bool,
    pub provider_change_invalidates: bool,
    pub requested_agent_count_change_invalidates: bool,
    pub requested_round_count_change_invalidates: bool,
    pub max_token_budget_change_invalidates: bool,
    pub active_provider_key_change_invalidates: bool,
    pub runtime_validity_change_invalidates: bool,
    pub activation_validity_change_invalidates: bool,
    pub cosmetic_only_change_invalidates: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreflightContractSnapshot {
    pub stage: PreflightContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub downgrade_acceptance_command_name: &'static str,
    pub seed_intake_request_contract: SeedIntakeRequestContract,
    pub requested_plan_contract: RequestedPlanContract,
    pub response_contract: PreflightResponseContract,
    pub invalidation_contract: PreflightInvalidationContract,
    pub seed_selection_surface_materialized: bool,
    pub preflight_command_boundary_explicit: bool,
    pub preflight_execution_materialized: bool,
    pub requested_plan_to_effective_plan_contract_materialized: bool,
    pub downgrade_acceptance_recording_materialized: bool,
    pub run_start_permitted_without_eligible_preflight: bool,
}

impl PreflightContractSnapshot {
    pub fn summary(self) -> String {
        let supported_seed_kinds = self
            .seed_intake_request_contract
            .supported_seed_document_kinds
            .map(SupportedSeedDocumentKind::label)
            .join(", ");
        let rejection_reasons = self
            .response_contract
            .known_rejection_reasons
            .map(PreflightRejectionReason::label)
            .join(", ");

        format!(
            "Preflight contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; downgrade-acceptance command name is {}; single seed document required is {}; supported file-type boundary required is {}; supported seed-document kinds are {}; max file-size boundary required is {}; maximum file size is {} bytes; minimum extractable text required is {}; minimum extractable text characters are {}; English validation required is {}; minimum English confidence percent is {}; file metadata capture required is {}; requested agent count required is {}; requested round count required is {}; max token budget required is {}; shell command boundary required is {}; durable raw seed storage allowed is {}; requested-plan echo required is {}; effective-plan fields supported is {}; estimated-usage supported is {}; provider-model context supported is {}; downgrade disclosure required is {}; silent requested-plan mutation allowed is {}; approved supported is {}; approved-with-downgrade supported is {}; rejected supported is {}; technical failure distinct from rejection is {}; rejection reason required when known is {}; known rejection reasons are {}; stale invalidation semantics supported is {}; seed-document change invalidates is {}; provider change invalidates is {}; requested agent-count change invalidates is {}; requested round-count change invalidates is {}; max-token-budget change invalidates is {}; active provider-key change invalidates is {}; runtime-validity change invalidates is {}; activation-validity change invalidates is {}; cosmetic-only change invalidates is {}; seed-selection surface materialized is {}; Preflight command boundary explicit is {}; Preflight execution materialized is {}; requested-plan to effective-plan contract materialized is {}; downgrade acceptance recording materialized is {}; run start permitted without eligible Preflight is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            self.downgrade_acceptance_command_name,
            self.seed_intake_request_contract.single_seed_document_required,
            self.seed_intake_request_contract.supported_file_type_boundary_required,
            supported_seed_kinds,
            self.seed_intake_request_contract.max_file_size_boundary_required,
            self.seed_intake_request_contract.maximum_file_size_bytes,
            self.seed_intake_request_contract.minimum_extractable_text_required,
            self.seed_intake_request_contract.minimum_extractable_text_characters,
            self.seed_intake_request_contract.english_validation_required,
            self.seed_intake_request_contract.minimum_english_confidence_percent,
            self.seed_intake_request_contract.file_metadata_capture_required,
            self.seed_intake_request_contract.requested_agent_count_required,
            self.seed_intake_request_contract.requested_round_count_required,
            self.seed_intake_request_contract.max_token_budget_required,
            self.seed_intake_request_contract.shell_command_boundary_required,
            self.seed_intake_request_contract.durable_raw_seed_storage_allowed,
            self.requested_plan_contract.requested_plan_echo_required,
            self.requested_plan_contract.effective_plan_fields_supported,
            self.requested_plan_contract.estimated_usage_supported,
            self.requested_plan_contract.provider_model_context_supported,
            self.requested_plan_contract.downgrade_disclosure_required,
            self.requested_plan_contract.silent_requested_plan_mutation_allowed,
            self.response_contract.approved_supported,
            self.response_contract.approved_with_downgrade_supported,
            self.response_contract.rejected_supported,
            self.response_contract.technical_failure_distinct_from_rejection,
            self.response_contract.rejection_reason_required_when_known,
            rejection_reasons,
            self.response_contract.stale_invalidation_semantics_supported,
            self.invalidation_contract.seed_document_change_invalidates,
            self.invalidation_contract.provider_change_invalidates,
            self.invalidation_contract.requested_agent_count_change_invalidates,
            self.invalidation_contract.requested_round_count_change_invalidates,
            self.invalidation_contract.max_token_budget_change_invalidates,
            self.invalidation_contract.active_provider_key_change_invalidates,
            self.invalidation_contract.runtime_validity_change_invalidates,
            self.invalidation_contract.activation_validity_change_invalidates,
            self.invalidation_contract.cosmetic_only_change_invalidates,
            self.seed_selection_surface_materialized,
            self.preflight_command_boundary_explicit,
            self.preflight_execution_materialized,
            self.requested_plan_to_effective_plan_contract_materialized,
            self.downgrade_acceptance_recording_materialized,
            self.run_start_permitted_without_eligible_preflight
        )
    }
}

pub fn preflight_contract_basis() -> PreflightContractSnapshot {
    PreflightContractSnapshot {
        stage: PreflightContractStage::FeasibilityAndInvalidationMaterialized,
        contract_name: PREFLIGHT_CONTRACT_NAME,
        shell_command_name: PREFLIGHT_COMMAND_NAME,
        downgrade_acceptance_command_name: PREFLIGHT_DOWNGRADE_ACCEPTANCE_COMMAND_NAME,
        seed_intake_request_contract: SeedIntakeRequestContract {
            single_seed_document_required: true,
            supported_file_type_boundary_required: true,
            supported_seed_document_kinds: [
                SupportedSeedDocumentKind::Pdf,
                SupportedSeedDocumentKind::Docx,
                SupportedSeedDocumentKind::Txt,
                SupportedSeedDocumentKind::Markdown,
            ],
            max_file_size_boundary_required: true,
            maximum_file_size_bytes: MAX_SEED_DOCUMENT_SIZE_BYTES,
            minimum_extractable_text_required: true,
            minimum_extractable_text_characters: MINIMUM_EXTRACTABLE_TEXT_CHARACTERS,
            english_validation_required: true,
            minimum_english_confidence_percent: MINIMUM_ENGLISH_CONFIDENCE_PERCENT,
            file_metadata_capture_required: true,
            requested_agent_count_required: true,
            requested_round_count_required: true,
            max_token_budget_required: true,
            shell_command_boundary_required: true,
            durable_raw_seed_storage_allowed: false,
        },
        requested_plan_contract: RequestedPlanContract {
            requested_plan_echo_required: true,
            effective_plan_fields_supported: true,
            estimated_usage_supported: true,
            provider_model_context_supported: true,
            downgrade_disclosure_required: true,
            silent_requested_plan_mutation_allowed: false,
        },
        response_contract: PreflightResponseContract {
            approved_supported: true,
            approved_with_downgrade_supported: true,
            rejected_supported: true,
            technical_failure_distinct_from_rejection: true,
            rejection_reason_required_when_known: true,
            known_rejection_reasons: [
                PreflightRejectionReason::UnsupportedFileType,
                PreflightRejectionReason::FileTooLarge,
                PreflightRejectionReason::TextExtractionFailed,
                PreflightRejectionReason::InsufficientExtractableText,
                PreflightRejectionReason::NonEnglishSeedDocument,
                PreflightRejectionReason::ActivationNotReady,
                PreflightRejectionReason::RuntimeNotReady,
                PreflightRejectionReason::ProviderConfigurationMissing,
                PreflightRejectionReason::ProviderConfigurationInvalid,
                PreflightRejectionReason::RequestedAgentCountBelowMinimum,
                PreflightRejectionReason::BudgetTooLowForMinimumPlan,
            ],
            stale_invalidation_semantics_supported: true,
        },
        invalidation_contract: PreflightInvalidationContract {
            seed_document_change_invalidates: true,
            provider_change_invalidates: true,
            requested_agent_count_change_invalidates: true,
            requested_round_count_change_invalidates: true,
            max_token_budget_change_invalidates: true,
            active_provider_key_change_invalidates: true,
            runtime_validity_change_invalidates: true,
            activation_validity_change_invalidates: true,
            cosmetic_only_change_invalidates: false,
        },
        seed_selection_surface_materialized: true,
        preflight_command_boundary_explicit: true,
        preflight_execution_materialized: true,
        requested_plan_to_effective_plan_contract_materialized: true,
        downgrade_acceptance_recording_materialized: true,
        run_start_permitted_without_eligible_preflight: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preflight_contract_basis_keeps_wave_four_constants_and_boundaries_explicit() {
        let snapshot = preflight_contract_basis();

        assert_eq!(snapshot.contract_name, PREFLIGHT_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, PREFLIGHT_COMMAND_NAME);
        assert_eq!(
            snapshot.downgrade_acceptance_command_name,
            PREFLIGHT_DOWNGRADE_ACCEPTANCE_COMMAND_NAME
        );
        assert!(snapshot.seed_intake_request_contract.single_seed_document_required);
        assert!(snapshot.seed_intake_request_contract.supported_file_type_boundary_required);
        assert_eq!(
            snapshot.seed_intake_request_contract.supported_seed_document_kinds,
            [
                SupportedSeedDocumentKind::Pdf,
                SupportedSeedDocumentKind::Docx,
                SupportedSeedDocumentKind::Txt,
                SupportedSeedDocumentKind::Markdown,
            ]
        );
        assert_eq!(
            snapshot.seed_intake_request_contract.maximum_file_size_bytes,
            MAX_SEED_DOCUMENT_SIZE_BYTES
        );
        assert_eq!(
            snapshot.seed_intake_request_contract.minimum_extractable_text_characters,
            MINIMUM_EXTRACTABLE_TEXT_CHARACTERS
        );
        assert_eq!(
            snapshot.seed_intake_request_contract.minimum_english_confidence_percent,
            MINIMUM_ENGLISH_CONFIDENCE_PERCENT
        );
        assert!(snapshot.seed_intake_request_contract.file_metadata_capture_required);
        assert!(snapshot.seed_intake_request_contract.requested_agent_count_required);
        assert!(snapshot.seed_intake_request_contract.requested_round_count_required);
        assert!(snapshot.seed_intake_request_contract.max_token_budget_required);
        assert!(snapshot.seed_intake_request_contract.shell_command_boundary_required);
        assert!(!snapshot.seed_intake_request_contract.durable_raw_seed_storage_allowed);
        assert!(snapshot.requested_plan_contract.requested_plan_echo_required);
        assert!(snapshot.requested_plan_contract.effective_plan_fields_supported);
        assert!(snapshot.requested_plan_contract.estimated_usage_supported);
        assert!(snapshot.requested_plan_contract.provider_model_context_supported);
        assert!(snapshot.requested_plan_contract.downgrade_disclosure_required);
        assert!(!snapshot.requested_plan_contract.silent_requested_plan_mutation_allowed);
        assert!(snapshot.response_contract.approved_supported);
        assert!(snapshot.response_contract.approved_with_downgrade_supported);
        assert!(snapshot.response_contract.rejected_supported);
        assert!(snapshot.response_contract.technical_failure_distinct_from_rejection);
        assert!(snapshot.response_contract.rejection_reason_required_when_known);
        assert_eq!(snapshot.response_contract.known_rejection_reasons.len(), 11);
        assert!(snapshot.response_contract.stale_invalidation_semantics_supported);
        assert!(snapshot.invalidation_contract.seed_document_change_invalidates);
        assert!(snapshot.invalidation_contract.provider_change_invalidates);
        assert!(snapshot.invalidation_contract.requested_agent_count_change_invalidates);
        assert!(snapshot.invalidation_contract.requested_round_count_change_invalidates);
        assert!(snapshot.invalidation_contract.max_token_budget_change_invalidates);
        assert!(snapshot.invalidation_contract.active_provider_key_change_invalidates);
        assert!(snapshot.invalidation_contract.runtime_validity_change_invalidates);
        assert!(snapshot.invalidation_contract.activation_validity_change_invalidates);
        assert!(!snapshot.invalidation_contract.cosmetic_only_change_invalidates);
        assert!(snapshot.seed_selection_surface_materialized);
        assert!(snapshot.preflight_command_boundary_explicit);
        assert!(snapshot.preflight_execution_materialized);
        assert!(snapshot.requested_plan_to_effective_plan_contract_materialized);
        assert!(snapshot.downgrade_acceptance_recording_materialized);
        assert!(!snapshot.run_start_permitted_without_eligible_preflight);
    }
}
