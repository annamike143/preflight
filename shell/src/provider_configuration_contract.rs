use serde::{Deserialize, Serialize};

pub const PROVIDER_CONFIGURATION_CONTRACT_NAME: &str = "ProviderConfigurationContract";
pub const PROVIDER_CONFIGURATION_COMMAND_NAME: &str = "SubmitProviderConfiguration";
pub const SUPPORTED_PROVIDERS: [SupportedProvider; 2] =
    [SupportedProvider::OpenAi, SupportedProvider::Google];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderConfigurationContractStage {
    Materialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedProvider {
    OpenAi,
    Google,
}

impl SupportedProvider {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "open_ai" | "openai" => Some(Self::OpenAi),
            "google" => Some(Self::Google),
            _ => None,
        }
    }

    pub fn lookup_key(self) -> &'static str {
        match self {
            Self::OpenAi => "open_ai",
            Self::Google => "google",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderConfigurationStatus {
    Required,
    Present,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderConfigurationFailureClassification {
    UnsupportedProvider,
    MalformedInput,
    SecureStorageFailure,
    OsPermissionFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderConfigurationRequestContract {
    pub provider_selection_required: bool,
    pub provider_api_key_required: bool,
    pub shell_command_boundary_required: bool,
    pub frontend_direct_secure_store_access_allowed: bool,
    pub supported_provider_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderConfigurationSuccessContract {
    pub selected_provider_required: bool,
    pub secure_store_commit_required_before_success: bool,
    pub configuration_readiness_update_required: bool,
    pub provider_reachability_implied_by_success: bool,
    pub model_availability_implied_by_success: bool,
    pub preflight_validation_still_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderConfigurationResponseContract {
    pub success_supported: bool,
    pub failure_supported: bool,
    pub failure_classification_count: usize,
    pub configuration_present_truth_supported: bool,
    pub configuration_missing_truth_supported: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderConfigurationContractSnapshot {
    pub stage: ProviderConfigurationContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub request_contract: ProviderConfigurationRequestContract,
    pub success_contract: ProviderConfigurationSuccessContract,
    pub response_contract: ProviderConfigurationResponseContract,
    pub secure_store_abstraction_required: bool,
    pub supported_provider_selection_materialized: bool,
    pub provider_submission_command_materialized: bool,
    pub secure_store_commit_materialized: bool,
    pub configuration_success_claimable: bool,
    pub preflight_readiness_implied: bool,
}

impl ProviderConfigurationContractSnapshot {
    pub fn supported_providers(self) -> [SupportedProvider; 2] {
        SUPPORTED_PROVIDERS
    }

    pub fn failure_classifications(self) -> [ProviderConfigurationFailureClassification; 4] {
        [
            ProviderConfigurationFailureClassification::UnsupportedProvider,
            ProviderConfigurationFailureClassification::MalformedInput,
            ProviderConfigurationFailureClassification::SecureStorageFailure,
            ProviderConfigurationFailureClassification::OsPermissionFailure,
        ]
    }

    pub fn summary(self) -> String {
        let supported_provider_count = self.supported_providers().len();
        let failure_classification_count = self.failure_classifications().len();

        format!(
            "Provider configuration contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; supported provider count is {}; provider selection required is {}; provider API key required is {}; shell command boundary required is {}; frontend direct secure-store access allowed is {}; success requires selected provider is {}; success requires secure-store commit before success is {}; success requires configuration readiness update is {}; provider reachability implied by success is {}; model availability implied by success is {}; Preflight validation still required is {}; success supported is {}; failure supported is {}; failure classification count is {}; configuration-present truth supported is {}; configuration-missing truth supported is {}; secure-store abstraction required is {}; supported provider selection materialized is {}; provider submission command materialized is {}; secure-store commit materialized is {}; configuration success claimable is {}; Preflight readiness implied is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            supported_provider_count,
            self.request_contract.provider_selection_required,
            self.request_contract.provider_api_key_required,
            self.request_contract.shell_command_boundary_required,
            self.request_contract.frontend_direct_secure_store_access_allowed,
            self.success_contract.selected_provider_required,
            self.success_contract.secure_store_commit_required_before_success,
            self.success_contract.configuration_readiness_update_required,
            self.success_contract.provider_reachability_implied_by_success,
            self.success_contract.model_availability_implied_by_success,
            self.success_contract.preflight_validation_still_required,
            self.response_contract.success_supported,
            self.response_contract.failure_supported,
            failure_classification_count,
            self.response_contract.configuration_present_truth_supported,
            self.response_contract.configuration_missing_truth_supported,
            self.secure_store_abstraction_required,
            self.supported_provider_selection_materialized,
            self.provider_submission_command_materialized,
            self.secure_store_commit_materialized,
            self.configuration_success_claimable,
            self.preflight_readiness_implied
        )
    }
}

pub fn provider_configuration_contract_basis() -> ProviderConfigurationContractSnapshot {
    ProviderConfigurationContractSnapshot {
        stage: ProviderConfigurationContractStage::Materialized,
        contract_name: PROVIDER_CONFIGURATION_CONTRACT_NAME,
        shell_command_name: PROVIDER_CONFIGURATION_COMMAND_NAME,
        request_contract: ProviderConfigurationRequestContract {
            provider_selection_required: true,
            provider_api_key_required: true,
            shell_command_boundary_required: true,
            frontend_direct_secure_store_access_allowed: false,
            supported_provider_count: SUPPORTED_PROVIDERS.len(),
        },
        success_contract: ProviderConfigurationSuccessContract {
            selected_provider_required: true,
            secure_store_commit_required_before_success: true,
            configuration_readiness_update_required: true,
            provider_reachability_implied_by_success: false,
            model_availability_implied_by_success: false,
            preflight_validation_still_required: true,
        },
        response_contract: ProviderConfigurationResponseContract {
            success_supported: true,
            failure_supported: true,
            failure_classification_count: 4,
            configuration_present_truth_supported: true,
            configuration_missing_truth_supported: true,
        },
        secure_store_abstraction_required: true,
        supported_provider_selection_materialized: true,
        provider_submission_command_materialized: true,
        secure_store_commit_materialized: true,
        configuration_success_claimable: true,
        preflight_readiness_implied: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_configuration_contract_basis_stays_boundary_only_and_bounded() {
        let snapshot = provider_configuration_contract_basis();

        assert_eq!(snapshot.contract_name, PROVIDER_CONFIGURATION_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, PROVIDER_CONFIGURATION_COMMAND_NAME);
        assert_eq!(snapshot.supported_providers().len(), 2);
        assert_eq!(snapshot.failure_classifications().len(), 4);
        assert!(snapshot.request_contract.provider_selection_required);
        assert!(snapshot.request_contract.provider_api_key_required);
        assert!(snapshot.request_contract.shell_command_boundary_required);
        assert!(!snapshot.request_contract.frontend_direct_secure_store_access_allowed);
        assert!(snapshot.success_contract.selected_provider_required);
        assert!(snapshot.success_contract.secure_store_commit_required_before_success);
        assert!(snapshot.success_contract.configuration_readiness_update_required);
        assert!(!snapshot.success_contract.provider_reachability_implied_by_success);
        assert!(!snapshot.success_contract.model_availability_implied_by_success);
        assert!(snapshot.success_contract.preflight_validation_still_required);
        assert!(snapshot.response_contract.success_supported);
        assert!(snapshot.response_contract.failure_supported);
        assert!(snapshot.response_contract.configuration_present_truth_supported);
        assert!(snapshot.response_contract.configuration_missing_truth_supported);
        assert!(snapshot.secure_store_abstraction_required);
        assert!(snapshot.supported_provider_selection_materialized);
        assert!(snapshot.provider_submission_command_materialized);
        assert!(snapshot.secure_store_commit_materialized);
        assert!(snapshot.configuration_success_claimable);
        assert!(!snapshot.preflight_readiness_implied);
    }
}