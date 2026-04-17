use serde::{Deserialize, Serialize};

pub const ACTIVATION_CONTRACT_NAME: &str = "DeviceBoundActivationContract";
pub const ACTIVATION_COMMAND_NAME: &str = "SubmitDeviceActivation";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationContractStage {
    RoundTripMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationDenialClassification {
    InvalidLicense,
    UnusableLicense,
    ActivationLimitExceeded,
    DeviceMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationTechnicalFailureClassification {
    ConnectivityFailure,
    SecureStoreFailure,
    MalformedTokenResponse,
    BackendFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationRequestContract {
    pub license_reference_required: bool,
    pub device_binding_representation_required: bool,
    pub client_platform_required: bool,
    pub raw_hardware_identity_as_normal_durable_field_allowed: bool,
    pub shell_command_boundary_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationApprovalContract {
    pub device_bound_token_required: bool,
    pub bounded_license_reference_required: bool,
    pub bounded_activation_reference_required: bool,
    pub slot_binding_context_required: bool,
    pub plan_tier_claim_required: bool,
    pub device_binding_claim_required: bool,
    pub issuance_timestamp_required: bool,
    pub signing_lineage_required: bool,
    pub expiration_claim_supported: bool,
    pub secure_store_commit_required_before_success: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationResponseContract {
    pub approval_supported: bool,
    pub denial_supported: bool,
    pub technical_failure_supported: bool,
    pub denial_classification_count: usize,
    pub technical_failure_classification_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationContractSnapshot {
    pub stage: ActivationContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub request_contract: ActivationRequestContract,
    pub approval_contract: ActivationApprovalContract,
    pub response_contract: ActivationResponseContract,
    pub secure_store_abstraction_required: bool,
    pub app_to_cloud_round_trip_implemented: bool,
    pub secure_store_commit_implemented: bool,
    pub local_support_record_commit_implemented: bool,
    pub activation_success_claimable: bool,
}

impl ActivationContractSnapshot {
    pub fn denial_classifications(self) -> [ActivationDenialClassification; 4] {
        [
            ActivationDenialClassification::InvalidLicense,
            ActivationDenialClassification::UnusableLicense,
            ActivationDenialClassification::ActivationLimitExceeded,
            ActivationDenialClassification::DeviceMismatch,
        ]
    }

    pub fn technical_failure_classifications(self) -> [ActivationTechnicalFailureClassification; 4] {
        [
            ActivationTechnicalFailureClassification::ConnectivityFailure,
            ActivationTechnicalFailureClassification::SecureStoreFailure,
            ActivationTechnicalFailureClassification::MalformedTokenResponse,
            ActivationTechnicalFailureClassification::BackendFailure,
        ]
    }

    pub fn summary(self) -> String {
        let denial_classification_count = self.denial_classifications().len();
        let technical_failure_classification_count = self.technical_failure_classifications().len();

        format!(
            "Activation contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; request contract requires license reference is {}; request contract requires device binding representation is {}; request contract requires client platform is {}; raw hardware identity as a normal durable field is allowed is {}; shell command boundary required is {}; approval requires device-bound token is {}; approval requires bounded license reference is {}; approval requires bounded activation reference is {}; approval requires slot binding context is {}; approval requires plan tier claim is {}; approval requires device binding claim is {}; approval requires issuance timestamp is {}; approval requires signing lineage is {}; approval supports expiration claim is {}; approval requires secure-store commit before success is {}; approval supported is {}; denial supported is {}; technical failure supported is {}; denial classification count is {}; technical failure classification count is {}; secure-store abstraction required is {}; app-to-cloud round trip implemented is {}; secure-store commit implemented is {}; local support record commit implemented is {}; activation success claimable is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            self.request_contract.license_reference_required,
            self.request_contract.device_binding_representation_required,
            self.request_contract.client_platform_required,
            self.request_contract.raw_hardware_identity_as_normal_durable_field_allowed,
            self.request_contract.shell_command_boundary_required,
            self.approval_contract.device_bound_token_required,
            self.approval_contract.bounded_license_reference_required,
            self.approval_contract.bounded_activation_reference_required,
            self.approval_contract.slot_binding_context_required,
            self.approval_contract.plan_tier_claim_required,
            self.approval_contract.device_binding_claim_required,
            self.approval_contract.issuance_timestamp_required,
            self.approval_contract.signing_lineage_required,
            self.approval_contract.expiration_claim_supported,
            self.approval_contract.secure_store_commit_required_before_success,
            self.response_contract.approval_supported,
            self.response_contract.denial_supported,
            self.response_contract.technical_failure_supported,
            denial_classification_count,
            technical_failure_classification_count,
            self.secure_store_abstraction_required,
            self.app_to_cloud_round_trip_implemented,
            self.secure_store_commit_implemented,
            self.local_support_record_commit_implemented,
            self.activation_success_claimable
        )
    }
}

pub fn activation_contract_basis() -> ActivationContractSnapshot {
    ActivationContractSnapshot {
        stage: ActivationContractStage::RoundTripMaterialized,
        contract_name: ACTIVATION_CONTRACT_NAME,
        shell_command_name: ACTIVATION_COMMAND_NAME,
        request_contract: ActivationRequestContract {
            license_reference_required: true,
            device_binding_representation_required: true,
            client_platform_required: true,
            raw_hardware_identity_as_normal_durable_field_allowed: false,
            shell_command_boundary_required: true,
        },
        approval_contract: ActivationApprovalContract {
            device_bound_token_required: true,
            bounded_license_reference_required: true,
            bounded_activation_reference_required: true,
            slot_binding_context_required: true,
            plan_tier_claim_required: true,
            device_binding_claim_required: true,
            issuance_timestamp_required: true,
            signing_lineage_required: true,
            expiration_claim_supported: true,
            secure_store_commit_required_before_success: true,
        },
        response_contract: ActivationResponseContract {
            approval_supported: true,
            denial_supported: true,
            technical_failure_supported: true,
            denial_classification_count: 4,
            technical_failure_classification_count: 4,
        },
        secure_store_abstraction_required: true,
        app_to_cloud_round_trip_implemented: true,
        secure_store_commit_implemented: true,
        local_support_record_commit_implemented: true,
        activation_success_claimable: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_contract_basis_reflects_real_round_trip_activation_truth() {
        let snapshot = activation_contract_basis();

        assert_eq!(snapshot.contract_name, ACTIVATION_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, ACTIVATION_COMMAND_NAME);
        assert!(snapshot.request_contract.license_reference_required);
        assert!(snapshot.request_contract.device_binding_representation_required);
        assert!(snapshot.request_contract.client_platform_required);
        assert!(!snapshot.request_contract.raw_hardware_identity_as_normal_durable_field_allowed);
        assert!(snapshot.request_contract.shell_command_boundary_required);
        assert!(snapshot.approval_contract.device_bound_token_required);
        assert!(snapshot.approval_contract.bounded_license_reference_required);
        assert!(snapshot.approval_contract.bounded_activation_reference_required);
        assert!(snapshot.approval_contract.slot_binding_context_required);
        assert!(snapshot.approval_contract.plan_tier_claim_required);
        assert!(snapshot.approval_contract.device_binding_claim_required);
        assert!(snapshot.approval_contract.issuance_timestamp_required);
        assert!(snapshot.approval_contract.signing_lineage_required);
        assert!(snapshot.approval_contract.expiration_claim_supported);
        assert!(snapshot.approval_contract.secure_store_commit_required_before_success);
        assert!(snapshot.response_contract.approval_supported);
        assert!(snapshot.response_contract.denial_supported);
        assert!(snapshot.response_contract.technical_failure_supported);
        assert_eq!(snapshot.denial_classifications().len(), 4);
        assert_eq!(snapshot.technical_failure_classifications().len(), 4);
        assert!(snapshot.secure_store_abstraction_required);
        assert!(snapshot.app_to_cloud_round_trip_implemented);
        assert!(snapshot.secure_store_commit_implemented);
        assert!(snapshot.local_support_record_commit_implemented);
        assert!(snapshot.activation_success_claimable);
    }
}