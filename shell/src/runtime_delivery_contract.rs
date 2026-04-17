use serde::{Deserialize, Serialize};

use crate::runtime_package_contract::runtime_package_contract_basis;
use crate::runtime_hydration::runtime_hydration_state;
use crate::runtime_integrity::runtime_integrity_state;
use crate::runtime_presence::runtime_presence_state;

pub const RUNTIME_DELIVERY_CONTRACT_NAME: &str = "ManagedRuntimeDeliveryContract";
pub const RUNTIME_DELIVERY_CONTRACT_COMMAND_NAME: &str = "RuntimeDeliveryContractState";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDeliveryContractStage {
    ExecutionMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct RuntimeDeliveryContractSnapshot {
    pub stage: RuntimeDeliveryContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub runtime_package_contract_name: &'static str,
    pub install_location_class_name: &'static str,
    pub app_update_mechanism_separate_from_runtime_delivery: bool,
    pub shell_authority_required: bool,
    pub frontend_presentation_only: bool,
    pub delivery_authorization_required: bool,
    pub delivery_manifest_support_allowed: bool,
    pub artifact_acquisition_path_defined: bool,
    pub artifact_presence_required_before_use: bool,
    pub expected_files_verification_required_before_use: bool,
    pub compatibility_verification_required_before_use: bool,
    pub trust_manifest_verification_required_before_use: bool,
    pub trust_rejection_distinct_from_technical_failure: bool,
    pub runtime_use_blocked_until_verified: bool,
    pub runtime_use_blocked_when_active_run_mutation_required: bool,
    pub runtime_delivery_mutation_during_active_run_forbidden: bool,
    pub runtime_repair_during_active_run_forbidden: bool,
    pub updater_state_separate_from_runtime_delivery: bool,
    pub runtime_readiness_separate_from_delivery_state: bool,
    pub authorization_basis_defined: bool,
    pub compatibility_basis_defined: bool,
    pub verification_basis_defined: bool,
    pub runtime_authorized_now: bool,
    pub runtime_compatible_now: bool,
    pub runtime_usable_now: bool,
    pub real_authorization_exchange_materialized: bool,
    pub real_artifact_acquisition_materialized: bool,
    pub real_verification_execution_materialized: bool,
    pub real_runtime_apply_materialized: bool,
}

impl RuntimeDeliveryContractSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Runtime delivery contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; runtime package contract name is {}; install location class is {}; app-update mechanism separate from runtime delivery is {}; shell authority required is {}; frontend presentation only is {}; delivery authorization required is {}; delivery manifest support allowed is {}; artifact acquisition path defined is {}; artifact presence required before use is {}; expected-files verification required before use is {}; compatibility verification required before use is {}; trust-manifest verification required before use is {}; trust rejection distinct from technical failure is {}; runtime use blocked until verified is {}; runtime use blocked when active-run mutation would be required is {}; runtime-delivery mutation during active run forbidden is {}; runtime repair during active run forbidden is {}; updater state separate from runtime delivery is {}; runtime readiness separate from delivery state is {}; authorization basis defined is {}; compatibility basis defined is {}; verification basis defined is {}; runtime authorized now is {}; runtime compatible now is {}; runtime usable now is {}; real authorization exchange materialized is {}; real artifact acquisition materialized is {}; real verification execution materialized is {}; real runtime apply materialized is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            self.runtime_package_contract_name,
            self.install_location_class_name,
            self.app_update_mechanism_separate_from_runtime_delivery,
            self.shell_authority_required,
            self.frontend_presentation_only,
            self.delivery_authorization_required,
            self.delivery_manifest_support_allowed,
            self.artifact_acquisition_path_defined,
            self.artifact_presence_required_before_use,
            self.expected_files_verification_required_before_use,
            self.compatibility_verification_required_before_use,
            self.trust_manifest_verification_required_before_use,
            self.trust_rejection_distinct_from_technical_failure,
            self.runtime_use_blocked_until_verified,
            self.runtime_use_blocked_when_active_run_mutation_required,
            self.runtime_delivery_mutation_during_active_run_forbidden,
            self.runtime_repair_during_active_run_forbidden,
            self.updater_state_separate_from_runtime_delivery,
            self.runtime_readiness_separate_from_delivery_state,
            self.authorization_basis_defined,
            self.compatibility_basis_defined,
            self.verification_basis_defined,
            self.runtime_authorized_now,
            self.runtime_compatible_now,
            self.runtime_usable_now,
            self.real_authorization_exchange_materialized,
            self.real_artifact_acquisition_materialized,
            self.real_verification_execution_materialized,
            self.real_runtime_apply_materialized,
        )
    }
}

pub fn runtime_delivery_contract_state() -> RuntimeDeliveryContractSnapshot {
    let runtime_package_contract = runtime_package_contract_basis();
    let runtime_presence = runtime_presence_state();
    let runtime_integrity = runtime_integrity_state();
    let runtime_hydration = runtime_hydration_state();
    let runtime_usable_now = runtime_presence.runtime_detected == Some(true)
        && runtime_integrity.runtime_integrity_passed
        && runtime_integrity.execution_unblocked;

    RuntimeDeliveryContractSnapshot {
        stage: RuntimeDeliveryContractStage::ExecutionMaterialized,
        contract_name: RUNTIME_DELIVERY_CONTRACT_NAME,
        shell_command_name: RUNTIME_DELIVERY_CONTRACT_COMMAND_NAME,
        runtime_package_contract_name: runtime_package_contract.contract_name,
        install_location_class_name: runtime_package_contract.install_location_class_name,
        app_update_mechanism_separate_from_runtime_delivery: true,
        shell_authority_required: true,
        frontend_presentation_only: true,
        delivery_authorization_required: true,
        delivery_manifest_support_allowed: true,
        artifact_acquisition_path_defined: true,
        artifact_presence_required_before_use: true,
        expected_files_verification_required_before_use: true,
        compatibility_verification_required_before_use: true,
        trust_manifest_verification_required_before_use: true,
        trust_rejection_distinct_from_technical_failure: true,
        runtime_use_blocked_until_verified: true,
        runtime_use_blocked_when_active_run_mutation_required: true,
        runtime_delivery_mutation_during_active_run_forbidden: true,
        runtime_repair_during_active_run_forbidden: true,
        updater_state_separate_from_runtime_delivery: true,
        runtime_readiness_separate_from_delivery_state: true,
        authorization_basis_defined: true,
        compatibility_basis_defined: true,
        verification_basis_defined: runtime_package_contract.shell_integrity_basis_defined,
        runtime_authorized_now: runtime_integrity.checksum_or_signature_verified,
        runtime_compatible_now: runtime_integrity.version_compatibility_verified,
        runtime_usable_now,
        real_authorization_exchange_materialized: runtime_hydration.authorization_manifest_configured
            || runtime_package_contract.local_trust_manifest_materialized
            || runtime_hydration.rejected_candidate_present,
        real_artifact_acquisition_materialized: runtime_package_contract
            .local_install_record_materialized,
        real_verification_execution_materialized: runtime_package_contract
            .local_trust_manifest_materialized
            || runtime_hydration.rejected_candidate_present,
        real_runtime_apply_materialized: runtime_package_contract.local_install_record_materialized,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_delivery_contract_requires_authorization_verification_and_active_run_safety() {
        let snapshot = runtime_delivery_contract_state();

        assert_eq!(snapshot.contract_name, RUNTIME_DELIVERY_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, RUNTIME_DELIVERY_CONTRACT_COMMAND_NAME);
        assert_eq!(snapshot.runtime_package_contract_name, "ManagedRuntimePackageContract");
        assert_eq!(snapshot.install_location_class_name, "AppControlledSupportLocation");
        assert!(snapshot.app_update_mechanism_separate_from_runtime_delivery);
        assert!(snapshot.shell_authority_required);
        assert!(snapshot.frontend_presentation_only);
        assert!(snapshot.delivery_authorization_required);
        assert!(snapshot.delivery_manifest_support_allowed);
        assert!(snapshot.artifact_acquisition_path_defined);
        assert!(snapshot.artifact_presence_required_before_use);
        assert!(snapshot.expected_files_verification_required_before_use);
        assert!(snapshot.compatibility_verification_required_before_use);
        assert!(snapshot.trust_manifest_verification_required_before_use);
        assert!(snapshot.trust_rejection_distinct_from_technical_failure);
        assert!(snapshot.runtime_use_blocked_until_verified);
        assert!(snapshot.runtime_use_blocked_when_active_run_mutation_required);
        assert!(snapshot.runtime_delivery_mutation_during_active_run_forbidden);
        assert!(snapshot.runtime_repair_during_active_run_forbidden);
        assert!(snapshot.updater_state_separate_from_runtime_delivery);
        assert!(snapshot.runtime_readiness_separate_from_delivery_state);
        assert!(snapshot.authorization_basis_defined);
        assert!(snapshot.compatibility_basis_defined);
        assert!(snapshot.verification_basis_defined);
        assert_eq!(snapshot.runtime_usable_now, snapshot.runtime_authorized_now && snapshot.runtime_compatible_now);
    }
}