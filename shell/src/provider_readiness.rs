use crate::activation_readiness::activation_readiness_truth_surface;
use crate::app_lifecycle::{STATE_CONFIGURATION_REQUIRED, STATE_READY_IDLE};
use crate::provider_configuration_contract::{
    provider_configuration_contract_basis, ProviderConfigurationStatus,
};
use crate::provider_configuration_submission::{
    provider_configuration_state_from_shell, ProviderConfigurationStateDetail,
    ProviderConfigurationStateSnapshot,
};
use crate::provider_storage_boundary::provider_storage_boundary_surface;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderReadinessStage {
    LocalConfigurationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderReadinessDetail {
    FlowNotMaterialized,
    SelectionMissing,
    SecretMissing,
    SecretFingerprintMismatch,
    StateAccessFailure,
    Configured,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderReadinessSnapshot {
    pub stage: ProviderReadinessStage,
    pub configuration_required_state_name: &'static str,
    pub ready_state_name: &'static str,
    pub readiness_surface_materialized: bool,
    pub provider_configuration_contract_defined: bool,
    pub provider_storage_boundary_defined: bool,
    pub runtime_readiness_evaluated_separately: bool,
    pub activation_readiness_evaluated_separately: bool,
    pub preflight_readiness_evaluated_separately: bool,
    pub runtime_ready_now: bool,
    pub activation_ready_now: bool,
    pub provider_configuration_flow_materialized: bool,
    pub provider_selection_present: bool,
    pub provider_secret_reference_present: bool,
    pub provider_configuration_ready: bool,
    pub current_status: ProviderConfigurationStatus,
    pub current_detail: ProviderReadinessDetail,
    pub may_progress_past_configuration_gate: bool,
}

impl ProviderReadinessSnapshot {
    pub fn supported_statuses(self) -> [ProviderConfigurationStatus; 3] {
        [
            ProviderConfigurationStatus::Required,
            ProviderConfigurationStatus::Present,
            ProviderConfigurationStatus::Error,
        ]
    }

    pub fn supported_details(self) -> [ProviderReadinessDetail; 6] {
        [
            ProviderReadinessDetail::FlowNotMaterialized,
            ProviderReadinessDetail::SelectionMissing,
            ProviderReadinessDetail::SecretMissing,
            ProviderReadinessDetail::SecretFingerprintMismatch,
            ProviderReadinessDetail::StateAccessFailure,
            ProviderReadinessDetail::Configured,
        ]
    }

    pub fn blocking_reasons(self) -> Vec<&'static str> {
        let mut reasons = Vec::new();

        if !self.provider_configuration_contract_defined {
            reasons.push("provider configuration contract is not defined");
        }

        if !self.provider_storage_boundary_defined {
            reasons.push("provider storage boundary is not defined");
        }

        if !self.provider_configuration_flow_materialized {
            reasons.push("provider configuration flow is not materialized");
        }

        if !self.provider_selection_present {
            reasons.push("supported provider selection is missing");
        }

        if !self.provider_secret_reference_present {
            reasons.push("provider API key is not configured");
        }

        reasons
    }

    pub fn summary(self) -> String {
        let blocking_reasons = self.blocking_reasons().join(", ");
        let supported_status_count = self.supported_statuses().len();
        let supported_detail_count = self.supported_details().len();

        format!(
            "Provider readiness truth surface exists at boundary level only. Stage is {:?}; configuration-required state is {}; ready state is {}; readiness surface materialized is {}; provider configuration contract defined is {}; provider storage boundary defined is {}; runtime readiness evaluated separately is {}; activation readiness evaluated separately is {}; Preflight readiness evaluated separately is {}; runtime ready now is {}; activation ready now is {}; provider configuration flow materialized is {}; provider selection present is {}; provider secret reference present is {}; provider configuration ready is {}; current status is {:?}; current detail is {:?}; supported status count is {}; supported detail count is {}; may progress past configuration gate is {}; blocking reasons are {}.",
            self.stage,
            self.configuration_required_state_name,
            self.ready_state_name,
            self.readiness_surface_materialized,
            self.provider_configuration_contract_defined,
            self.provider_storage_boundary_defined,
            self.runtime_readiness_evaluated_separately,
            self.activation_readiness_evaluated_separately,
            self.preflight_readiness_evaluated_separately,
            self.runtime_ready_now,
            self.activation_ready_now,
            self.provider_configuration_flow_materialized,
            self.provider_selection_present,
            self.provider_secret_reference_present,
            self.provider_configuration_ready,
            self.current_status,
            self.current_detail,
            supported_status_count,
            supported_detail_count,
            self.may_progress_past_configuration_gate,
            blocking_reasons
        )
    }
}

pub fn provider_readiness_truth_surface() -> ProviderReadinessSnapshot {
    let provider_contract = provider_configuration_contract_basis();
    let provider_storage_boundary = provider_storage_boundary_surface();
    let activation_readiness = activation_readiness_truth_surface();
    let provider_configuration_state = provider_configuration_state_from_shell();
    let provider_configuration_flow_materialized = provider_contract.provider_submission_command_materialized
        && provider_storage_boundary.secure_store_commit_implemented
        && provider_storage_boundary.secure_store_retrieval_implemented
        && provider_storage_boundary.support_state_persistence_implemented;
    let current_detail = resolve_provider_readiness_detail(&provider_configuration_state);

    ProviderReadinessSnapshot {
        stage: ProviderReadinessStage::LocalConfigurationMaterialized,
        configuration_required_state_name: STATE_CONFIGURATION_REQUIRED,
        ready_state_name: STATE_READY_IDLE,
        readiness_surface_materialized: true,
        provider_configuration_contract_defined: provider_contract.request_contract.provider_selection_required
            && provider_contract.request_contract.provider_api_key_required
            && provider_contract.response_contract.success_supported
            && provider_contract.response_contract.failure_supported,
        provider_storage_boundary_defined: provider_storage_boundary.secure_store_abstraction_defined
            && provider_storage_boundary.raw_provider_secret_secure_store_only,
        runtime_readiness_evaluated_separately: true,
        activation_readiness_evaluated_separately: true,
        preflight_readiness_evaluated_separately: true,
        runtime_ready_now: activation_readiness.runtime_ready_now,
        activation_ready_now: activation_readiness.activation_ready_for_licensed_use,
        provider_configuration_flow_materialized,
        provider_selection_present: provider_configuration_state.selected_provider.is_some(),
        provider_secret_reference_present: provider_configuration_state.provider_secret_reference_present,
        provider_configuration_ready: provider_configuration_state.configuration_present,
        current_status: provider_configuration_state.current_status,
        current_detail,
        may_progress_past_configuration_gate: provider_configuration_state.configuration_present,
    }
}

fn resolve_provider_readiness_detail(
    provider_configuration_state: &ProviderConfigurationStateSnapshot,
) -> ProviderReadinessDetail {
    match provider_configuration_state.current_detail {
        ProviderConfigurationStateDetail::FlowNotMaterialized => {
            ProviderReadinessDetail::FlowNotMaterialized
        }
        ProviderConfigurationStateDetail::SelectionMissing => {
            ProviderReadinessDetail::SelectionMissing
        }
        ProviderConfigurationStateDetail::SecretMissing => ProviderReadinessDetail::SecretMissing,
        ProviderConfigurationStateDetail::SecretFingerprintMismatch => {
            ProviderReadinessDetail::SecretFingerprintMismatch
        }
        ProviderConfigurationStateDetail::StateAccessFailure => {
            ProviderReadinessDetail::StateAccessFailure
        }
        ProviderConfigurationStateDetail::Configured => ProviderReadinessDetail::Configured,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_readiness_truth_surface_stays_not_ready_without_local_provider_configuration() {
        let snapshot = provider_readiness_truth_surface();
        let blocking_reasons = snapshot.blocking_reasons();

        assert_eq!(snapshot.configuration_required_state_name, STATE_CONFIGURATION_REQUIRED);
        assert_eq!(snapshot.ready_state_name, STATE_READY_IDLE);
        assert!(snapshot.readiness_surface_materialized);
        assert!(snapshot.provider_configuration_contract_defined);
        assert!(snapshot.provider_storage_boundary_defined);
        assert!(snapshot.runtime_readiness_evaluated_separately);
        assert!(snapshot.activation_readiness_evaluated_separately);
        assert!(snapshot.preflight_readiness_evaluated_separately);
        assert_eq!(snapshot.supported_statuses().len(), 3);
        assert_eq!(snapshot.supported_details().len(), 6);
        assert!(!snapshot.runtime_ready_now);
        assert!(!snapshot.activation_ready_now);
        assert!(snapshot.provider_configuration_flow_materialized);
        assert!(!snapshot.provider_selection_present);
        assert!(!snapshot.provider_secret_reference_present);
        assert!(!snapshot.provider_configuration_ready);
        assert_eq!(snapshot.current_status, ProviderConfigurationStatus::Required);
        assert_eq!(snapshot.current_detail, ProviderReadinessDetail::SelectionMissing);
        assert!(!snapshot.may_progress_past_configuration_gate);
        assert_eq!(blocking_reasons.len(), 2);
    }
}