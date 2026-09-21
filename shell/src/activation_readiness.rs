use crate::activation_contract::activation_contract_basis;
use crate::activation_local_validation::{
    activation_local_validation_surface, LocalActivationValidationDetail,
};
use crate::activation_storage_boundary::activation_storage_boundary_surface;
use crate::app_lifecycle::{
    STATE_ACTIVATION_REQUIRED, STATE_ACTIVATION_REVOKED, STATE_CONFIGURATION_REQUIRED,
    STATE_READY_IDLE,
};
use crate::runtime_integrity::runtime_integrity_state;
use crate::runtime_presence::runtime_presence_state;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationReadinessStage {
    LocalValidationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationReadinessSnapshot {
    pub stage: ActivationReadinessStage,
    pub activation_required_state_name: &'static str,
    pub activation_revoked_state_name: &'static str,
    pub configuration_required_state_name: &'static str,
    pub ready_state_name: &'static str,
    pub readiness_surface_materialized: bool,
    pub activation_contract_defined: bool,
    pub storage_boundary_defined: bool,
    pub runtime_readiness_evaluated_separately: bool,
    pub provider_readiness_evaluated_separately: bool,
    pub runtime_ready_now: bool,
    pub local_secure_token_present: bool,
    pub local_support_record_present: bool,
    pub local_validation_capability_materialized: bool,
    pub current_validation_status: crate::activation_storage_boundary::ActivationValidationStatus,
    pub current_validation_detail: LocalActivationValidationDetail,
    pub activation_ready_for_licensed_use: bool,
    pub activation_not_ready_for_licensed_use: bool,
    pub activation_revoked_detected: bool,
    pub may_progress_past_activation_gate: bool,
    pub execution_mode: ExecutionMode,
}

pub const PREFLIGHT_EXECUTION_MODE_ENV_VAR: &str = "PREFLIGHT_EXECUTION_MODE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Community,
    Enterprise,
}

pub fn current_execution_mode() -> ExecutionMode {
    match std::env::var(PREFLIGHT_EXECUTION_MODE_ENV_VAR).as_deref() {
        Ok("community") => ExecutionMode::Community,
        _ => ExecutionMode::Enterprise,
    }
}

impl ActivationReadinessSnapshot {
    pub fn blocking_reasons(self) -> Vec<&'static str> {
        let mut reasons = Vec::new();

        if self.execution_mode == ExecutionMode::Community {
            return reasons;
        }

        if !self.local_validation_capability_materialized {
            reasons.push("local activation validation behavior is not materialized");
            return reasons;
        }

        if self.current_validation_status
            == crate::activation_storage_boundary::ActivationValidationStatus::ActivatedValid
        {
            return reasons;
        }

        reasons.push(self.current_validation_detail.description());

        reasons
    }

    pub fn summary(self) -> String {
        let blocking_reasons = self.blocking_reasons().join(", ");

        format!(
            "Activation readiness truth surface exists only at boundary level. Stage is {:?}; activation-required state is {}; activation-revoked state is {}; configuration-required state is {}; ready state is {}; readiness surface materialized is {}; activation contract defined is {}; storage boundary defined is {}; runtime readiness evaluated separately is {}; provider readiness evaluated separately is {}; runtime ready now is {}; local secure token present is {}; local support record present is {}; local validation capability materialized is {}; current validation status is {:?}; current validation detail is {:?}; activation ready for licensed use is {}; activation not ready for licensed use is {}; activation revoked detected is {}; may progress past activation gate is {}; blocking reasons are {}.",
            self.stage,
            self.activation_required_state_name,
            self.activation_revoked_state_name,
            self.configuration_required_state_name,
            self.ready_state_name,
            self.readiness_surface_materialized,
            self.activation_contract_defined,
            self.storage_boundary_defined,
            self.runtime_readiness_evaluated_separately,
            self.provider_readiness_evaluated_separately,
            self.runtime_ready_now,
            self.local_secure_token_present,
            self.local_support_record_present,
            self.local_validation_capability_materialized,
            self.current_validation_status,
            self.current_validation_detail,
            self.activation_ready_for_licensed_use,
            self.activation_not_ready_for_licensed_use,
            self.activation_revoked_detected,
            self.may_progress_past_activation_gate,
            blocking_reasons
        )
    }
}

pub fn activation_readiness_truth_surface() -> ActivationReadinessSnapshot {
    activation_readiness_truth_surface_with_mode(current_execution_mode())
}

pub fn activation_readiness_truth_surface_with_mode(execution_mode: ExecutionMode) -> ActivationReadinessSnapshot {
    let activation_contract = activation_contract_basis();
    let storage_boundary = activation_storage_boundary_surface();
    let runtime_presence = runtime_presence_state();
    let runtime_integrity = runtime_integrity_state();
    let local_validation = activation_local_validation_surface();
    let runtime_ready = runtime_presence.runtime_detected == Some(true)
        && runtime_integrity.runtime_integrity_passed
        && runtime_integrity.execution_unblocked;
    let is_community = execution_mode == ExecutionMode::Community;
    let activation_ready = is_community
        || (local_validation.validation_status
            == crate::activation_storage_boundary::ActivationValidationStatus::ActivatedValid);

    ActivationReadinessSnapshot {
        stage: ActivationReadinessStage::LocalValidationMaterialized,
        execution_mode,
        activation_required_state_name: STATE_ACTIVATION_REQUIRED,
        activation_revoked_state_name: STATE_ACTIVATION_REVOKED,
        configuration_required_state_name: STATE_CONFIGURATION_REQUIRED,
        ready_state_name: STATE_READY_IDLE,
        readiness_surface_materialized: true,
        activation_contract_defined: activation_contract.response_contract.approval_supported
            && activation_contract.response_contract.denial_supported
            && activation_contract.response_contract.technical_failure_supported,
        storage_boundary_defined: storage_boundary.secure_store_abstraction_defined
            && storage_boundary.raw_token_material_secure_store_only,
        runtime_readiness_evaluated_separately: true,
        provider_readiness_evaluated_separately: true,
        runtime_ready_now: runtime_ready,
        local_secure_token_present: local_validation.local_secure_token_present,
        local_support_record_present: local_validation.local_support_record_present,
        local_validation_capability_materialized: local_validation.validation_capability_materialized,
        current_validation_status: local_validation.validation_status,
        current_validation_detail: local_validation.validation_detail,
        activation_ready_for_licensed_use: activation_ready,
        activation_not_ready_for_licensed_use: !activation_ready,
        activation_revoked_detected: !is_community
            && local_validation.validation_status
                == crate::activation_storage_boundary::ActivationValidationStatus::ActivationRevoked,
        may_progress_past_activation_gate: activation_ready,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_readiness_truth_surface_keeps_activation_gating_separate_and_not_ready() {
        let snapshot = activation_readiness_truth_surface();
        let blocking_reasons = snapshot.blocking_reasons();

        assert_eq!(snapshot.activation_required_state_name, STATE_ACTIVATION_REQUIRED);
        assert_eq!(snapshot.activation_revoked_state_name, STATE_ACTIVATION_REVOKED);
        assert_eq!(snapshot.configuration_required_state_name, STATE_CONFIGURATION_REQUIRED);
        assert_eq!(snapshot.ready_state_name, STATE_READY_IDLE);
        assert!(snapshot.readiness_surface_materialized);
        assert!(snapshot.activation_contract_defined);
        assert!(snapshot.storage_boundary_defined);
        assert!(snapshot.runtime_readiness_evaluated_separately);
        assert!(snapshot.provider_readiness_evaluated_separately);
        assert!(!snapshot.runtime_ready_now);
        assert!(snapshot.local_validation_capability_materialized);
        assert_eq!(
            snapshot.current_validation_status,
            crate::activation_storage_boundary::ActivationValidationStatus::ActivationRequired
        );
        assert_eq!(
            snapshot.current_validation_detail,
            LocalActivationValidationDetail::SupportRecordMissing
        );
        assert!(!snapshot.activation_ready_for_licensed_use);
        assert!(snapshot.activation_not_ready_for_licensed_use);
        assert!(!snapshot.activation_revoked_detected);
        assert!(!snapshot.may_progress_past_activation_gate);
        assert_eq!(blocking_reasons.len(), 1);
    }

    #[test]
    fn activation_readiness_truth_surface_unblocks_gate_in_community_mode() {
        let snapshot = activation_readiness_truth_surface_with_mode(ExecutionMode::Community);
        let blocking_reasons = snapshot.blocking_reasons();

        assert!(snapshot.activation_ready_for_licensed_use);
        assert!(!snapshot.activation_not_ready_for_licensed_use);
        assert!(snapshot.may_progress_past_activation_gate);
        assert!(blocking_reasons.is_empty());
    }
}