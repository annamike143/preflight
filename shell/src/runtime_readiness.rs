use crate::app_lifecycle::{app_lifecycle_truth_surface, STATE_READY_IDLE};
use crate::runtime_delivery_contract::runtime_delivery_contract_state;
use crate::runtime_hydration::runtime_hydration_state;
use crate::runtime_integrity::runtime_integrity_state;
use crate::runtime_presence::runtime_presence_state;
use crate::updater_contract::updater_contract_state;

#[derive(Debug, Clone, Copy)]
pub enum RuntimeReadinessStage {
    ExecutionMaterialized,
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimeReadinessSnapshot {
    pub stage: RuntimeReadinessStage,
    pub current_lifecycle_state_name: &'static str,
    pub ready_state_name: &'static str,
    pub updater_current_state_name: &'static str,
    pub updater_discovery_configuration_ready: bool,
    pub updater_discovery_not_required_for_runtime_ready: bool,
    pub runtime_ready: bool,
    pub runtime_not_ready: bool,
    pub simulation_features_available: bool,
    pub runtime_presence_determined: bool,
    pub runtime_presence_confirmed: bool,
    pub runtime_integrity_accepted: bool,
    pub runtime_hydration_required_now: bool,
    pub runtime_candidate_rejected: bool,
    pub runtime_mutation_blocked_by_active_run: bool,
    pub runtime_delivery_contract_defined: bool,
    pub runtime_delivery_authorization_basis_defined: bool,
    pub runtime_delivery_compatibility_basis_defined: bool,
    pub runtime_delivery_verification_required_before_use: bool,
    pub updater_state_family_separated: bool,
    pub updater_apply_deferred_during_active_run: bool,
    pub active_run_runtime_mutation_forbidden: bool,
}

impl RuntimeReadinessSnapshot {
    pub fn blocking_reasons(self) -> Vec<&'static str> {
        let mut reasons = Vec::new();

        if self.runtime_candidate_rejected {
            reasons.push("last runtime candidate was rejected");
        }

        if !self.runtime_presence_determined || !self.runtime_presence_confirmed {
            reasons.push("approved runtime package is not present");
        }

        if !self.runtime_integrity_accepted {
            reasons.push("runtime integrity is not accepted");
        }

        if self.runtime_mutation_blocked_by_active_run {
            reasons.push("runtime mutation is blocked during an active run");
        }

        reasons
    }

    pub fn summary(self) -> String {
        let blocking_reasons = self.blocking_reasons().join(", ");

        format!(
            "Runtime readiness truth surface is materialized. Stage is {:?}; current lifecycle state is {}; ready state is {}; updater current state is {}; updater discovery configuration ready is {}; updater discovery not required for runtime ready is {}; runtime ready is {}; runtime not ready is {}; simulation features available is {}; runtime presence determined is {}; runtime presence confirmed is {}; runtime integrity accepted is {}; runtime hydration required now is {}; runtime candidate rejected is {}; runtime mutation blocked by active run is {}; runtime-delivery contract defined is {}; runtime-delivery authorization basis defined is {}; runtime-delivery compatibility basis defined is {}; runtime-delivery verification required before use is {}; updater state family separated is {}; updater apply deferred during active run is {}; active-run runtime mutation forbidden is {}; blocking reasons are {}.",
            self.stage,
            self.current_lifecycle_state_name,
            self.ready_state_name,
            self.updater_current_state_name,
            self.updater_discovery_configuration_ready,
            self.updater_discovery_not_required_for_runtime_ready,
            self.runtime_ready,
            self.runtime_not_ready,
            self.simulation_features_available,
            self.runtime_presence_determined,
            self.runtime_presence_confirmed,
            self.runtime_integrity_accepted,
            self.runtime_hydration_required_now,
            self.runtime_candidate_rejected,
            self.runtime_mutation_blocked_by_active_run,
            self.runtime_delivery_contract_defined,
            self.runtime_delivery_authorization_basis_defined,
            self.runtime_delivery_compatibility_basis_defined,
            self.runtime_delivery_verification_required_before_use,
            self.updater_state_family_separated,
            self.updater_apply_deferred_during_active_run,
            self.active_run_runtime_mutation_forbidden,
            blocking_reasons
        )
    }
}

pub fn runtime_readiness_truth_surface() -> RuntimeReadinessSnapshot {
    let lifecycle = app_lifecycle_truth_surface();
    let runtime_presence = runtime_presence_state();
    let runtime_integrity = runtime_integrity_state();
    let runtime_hydration = runtime_hydration_state();
    let updater_contract = updater_contract_state();
    let runtime_delivery_contract = runtime_delivery_contract_state();
    let runtime_ready = runtime_presence.runtime_detected == Some(true)
        && runtime_integrity.runtime_integrity_passed
        && runtime_integrity.execution_unblocked;

    RuntimeReadinessSnapshot {
        stage: RuntimeReadinessStage::ExecutionMaterialized,
        current_lifecycle_state_name: lifecycle.current_state_name,
        ready_state_name: STATE_READY_IDLE,
        updater_current_state_name: updater_contract.current_state_name,
        updater_discovery_configuration_ready: updater_contract.discovery_configuration_ready,
        updater_discovery_not_required_for_runtime_ready: updater_contract
            .updater_state_separate_from_run_lifecycle,
        runtime_ready,
        runtime_not_ready: !runtime_ready,
        simulation_features_available: runtime_ready,
        runtime_presence_determined: runtime_presence.runtime_detected.is_some(),
        runtime_presence_confirmed: runtime_presence.runtime_detected == Some(true),
        runtime_integrity_accepted: runtime_integrity.runtime_integrity_passed
            && runtime_integrity.execution_unblocked,
        runtime_hydration_required_now: runtime_hydration.hydration_required_now,
        runtime_candidate_rejected: runtime_hydration.rejected_candidate_present,
        runtime_mutation_blocked_by_active_run: runtime_hydration.active_run_mutation_blocked_now,
        runtime_delivery_contract_defined: runtime_presence.runtime_delivery_contract_defined,
        runtime_delivery_authorization_basis_defined: runtime_delivery_contract
            .authorization_basis_defined,
        runtime_delivery_compatibility_basis_defined: runtime_delivery_contract
            .compatibility_basis_defined,
        runtime_delivery_verification_required_before_use: runtime_delivery_contract
            .runtime_use_blocked_until_verified,
        updater_state_family_separated: updater_contract.updater_state_separate_from_run_lifecycle
            && runtime_delivery_contract.updater_state_separate_from_runtime_delivery,
        updater_apply_deferred_during_active_run: updater_contract.apply_deferred_during_active_run,
        active_run_runtime_mutation_forbidden: runtime_delivery_contract
            .runtime_delivery_mutation_during_active_run_forbidden,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_readiness_truth_surface_reports_explicit_not_ready_reasons() {
        let snapshot = runtime_readiness_truth_surface();
        let blocking_reasons = snapshot.blocking_reasons();

        assert_eq!(snapshot.current_lifecycle_state_name, "RuntimeHydrationRequired");
        assert_eq!(snapshot.ready_state_name, STATE_READY_IDLE);
        assert_eq!(snapshot.updater_current_state_name, "UpdaterIdle");
        assert!(snapshot.updater_discovery_not_required_for_runtime_ready);
        assert!(!snapshot.runtime_ready);
        assert!(snapshot.runtime_not_ready);
        assert!(!snapshot.simulation_features_available);
        assert!(snapshot.runtime_hydration_required_now);
        assert!(!snapshot.runtime_presence_confirmed);
        assert!(!snapshot.runtime_integrity_accepted);
        assert!(snapshot.runtime_delivery_contract_defined);
        assert!(snapshot.runtime_delivery_authorization_basis_defined);
        assert!(snapshot.runtime_delivery_compatibility_basis_defined);
        assert!(snapshot.runtime_delivery_verification_required_before_use);
        assert!(snapshot.updater_state_family_separated);
        assert!(snapshot.updater_apply_deferred_during_active_run);
        assert!(snapshot.active_run_runtime_mutation_forbidden);
        assert_eq!(blocking_reasons.len(), 2);
    }
}