use serde::{Deserialize, Serialize};

pub const ACTIVATION_STORAGE_BOUNDARY_NAME: &str = "ActivationArtifactStorageBoundary";
pub const ACTIVATION_SECURE_STORE_DOMAIN_NAME: &str = "OsNativeSecureStorageDomain";
pub const ACTIVATION_LOCAL_SUPPORT_DOMAIN_NAME: &str = "LocalPersistentSupportDomain";
pub const ACTIVATION_SECURE_STORE_ABSTRACTION_NAME: &str = "ShellOwnedActivationSecureStore";
pub const ACTIVATION_SUPPORT_RECORD_NAME: &str = "ActivationValidationSupportRecord";
pub const ACTIVATION_LOCAL_SUPPORT_ENGINE_NAME: &str = "JsonFileActivationSupportRecordStore";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationStorageBoundaryStage {
    LocalPersistenceMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationValidationStatus {
    ActivationRequired,
    ActivationDenied,
    ActivationError,
    ActivatedValid,
    ActivationRevoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationSupportRecordContract {
    pub schema_version_required: bool,
    pub bounded_license_reference_required: bool,
    pub bounded_activation_reference_required: bool,
    pub slot_binding_context_required: bool,
    pub token_lineage_marker_required: bool,
    pub validity_marker_required: bool,
    pub secure_store_lookup_reference_required: bool,
    pub last_validation_status_required: bool,
    pub last_validation_timestamp_required: bool,
    pub raw_device_bound_token_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationStorageBoundarySnapshot {
    pub stage: ActivationStorageBoundaryStage,
    pub boundary_name: &'static str,
    pub secure_store_domain_name: &'static str,
    pub local_support_domain_name: &'static str,
    pub secure_store_abstraction_name: &'static str,
    pub support_record_name: &'static str,
    pub local_support_engine_name: &'static str,
    pub secure_store_shell_owned: bool,
    pub secure_store_abstraction_defined: bool,
    pub raw_token_material_secure_store_only: bool,
    pub raw_token_plaintext_mirror_allowed: bool,
    pub local_support_record_raw_token_allowed: bool,
    pub workspace_storage_allowed: bool,
    pub log_storage_allowed: bool,
    pub report_artifact_storage_allowed: bool,
    pub ordinary_config_storage_allowed: bool,
    pub secure_store_commit_implemented: bool,
    pub secure_store_retrieval_implemented: bool,
    pub local_support_record_persistence_implemented: bool,
    pub local_support_engine_selected: bool,
    pub support_record_contract: ActivationSupportRecordContract,
}

impl ActivationStorageBoundarySnapshot {
    pub fn supported_validation_statuses(self) -> [ActivationValidationStatus; 5] {
        [
            ActivationValidationStatus::ActivationRequired,
            ActivationValidationStatus::ActivationDenied,
            ActivationValidationStatus::ActivationError,
            ActivationValidationStatus::ActivatedValid,
            ActivationValidationStatus::ActivationRevoked,
        ]
    }

    pub fn summary(self) -> String {
        let validation_status_count = self.supported_validation_statuses().len();

        format!(
            "Activation storage-boundary surface exists only at boundary level. Stage is {:?}; boundary name is {}; secure-store domain is {}; local support domain is {}; secure-store abstraction name is {}; support record name is {}; local support engine name is {}; secure-store shell-owned is {}; secure-store abstraction defined is {}; raw token material secure-store only is {}; raw token plaintext mirror allowed is {}; local support record raw token allowed is {}; workspace storage allowed is {}; log storage allowed is {}; report artifact storage allowed is {}; ordinary config storage allowed is {}; secure-store commit implemented is {}; secure-store retrieval implemented is {}; local support record persistence implemented is {}; local support engine selected is {}; supported validation status count is {}; support record requires schema version is {}; support record requires bounded license reference is {}; support record requires bounded activation reference is {}; support record requires slot binding context is {}; support record requires token lineage marker is {}; support record requires validity marker is {}; support record requires secure-store lookup reference is {}; support record requires last validation status is {}; support record requires last validation timestamp is {}; support record allows raw device-bound token is {}.",
            self.stage,
            self.boundary_name,
            self.secure_store_domain_name,
            self.local_support_domain_name,
            self.secure_store_abstraction_name,
            self.support_record_name,
            self.local_support_engine_name,
            self.secure_store_shell_owned,
            self.secure_store_abstraction_defined,
            self.raw_token_material_secure_store_only,
            self.raw_token_plaintext_mirror_allowed,
            self.local_support_record_raw_token_allowed,
            self.workspace_storage_allowed,
            self.log_storage_allowed,
            self.report_artifact_storage_allowed,
            self.ordinary_config_storage_allowed,
            self.secure_store_commit_implemented,
            self.secure_store_retrieval_implemented,
            self.local_support_record_persistence_implemented,
            self.local_support_engine_selected,
            validation_status_count,
            self.support_record_contract.schema_version_required,
            self.support_record_contract.bounded_license_reference_required,
            self.support_record_contract.bounded_activation_reference_required,
            self.support_record_contract.slot_binding_context_required,
            self.support_record_contract.token_lineage_marker_required,
            self.support_record_contract.validity_marker_required,
            self.support_record_contract.secure_store_lookup_reference_required,
            self.support_record_contract.last_validation_status_required,
            self.support_record_contract.last_validation_timestamp_required,
            self.support_record_contract.raw_device_bound_token_allowed
        )
    }
}

pub fn activation_storage_boundary_surface() -> ActivationStorageBoundarySnapshot {
    ActivationStorageBoundarySnapshot {
        stage: ActivationStorageBoundaryStage::LocalPersistenceMaterialized,
        boundary_name: ACTIVATION_STORAGE_BOUNDARY_NAME,
        secure_store_domain_name: ACTIVATION_SECURE_STORE_DOMAIN_NAME,
        local_support_domain_name: ACTIVATION_LOCAL_SUPPORT_DOMAIN_NAME,
        secure_store_abstraction_name: ACTIVATION_SECURE_STORE_ABSTRACTION_NAME,
        support_record_name: ACTIVATION_SUPPORT_RECORD_NAME,
        local_support_engine_name: ACTIVATION_LOCAL_SUPPORT_ENGINE_NAME,
        secure_store_shell_owned: true,
        secure_store_abstraction_defined: true,
        raw_token_material_secure_store_only: true,
        raw_token_plaintext_mirror_allowed: false,
        local_support_record_raw_token_allowed: false,
        workspace_storage_allowed: false,
        log_storage_allowed: false,
        report_artifact_storage_allowed: false,
        ordinary_config_storage_allowed: false,
        secure_store_commit_implemented: true,
        secure_store_retrieval_implemented: true,
        local_support_record_persistence_implemented: true,
        local_support_engine_selected: true,
        support_record_contract: ActivationSupportRecordContract {
            schema_version_required: true,
            bounded_license_reference_required: true,
            bounded_activation_reference_required: true,
            slot_binding_context_required: true,
            token_lineage_marker_required: true,
            validity_marker_required: true,
            secure_store_lookup_reference_required: true,
            last_validation_status_required: true,
            last_validation_timestamp_required: true,
            raw_device_bound_token_allowed: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_storage_boundary_surface_keeps_raw_token_material_out_of_ordinary_persistence() {
        let snapshot = activation_storage_boundary_surface();

        assert_eq!(snapshot.boundary_name, ACTIVATION_STORAGE_BOUNDARY_NAME);
        assert_eq!(snapshot.secure_store_domain_name, ACTIVATION_SECURE_STORE_DOMAIN_NAME);
        assert_eq!(snapshot.local_support_domain_name, ACTIVATION_LOCAL_SUPPORT_DOMAIN_NAME);
        assert_eq!(snapshot.secure_store_abstraction_name, ACTIVATION_SECURE_STORE_ABSTRACTION_NAME);
        assert_eq!(snapshot.support_record_name, ACTIVATION_SUPPORT_RECORD_NAME);
        assert_eq!(snapshot.local_support_engine_name, ACTIVATION_LOCAL_SUPPORT_ENGINE_NAME);
        assert!(snapshot.secure_store_shell_owned);
        assert!(snapshot.secure_store_abstraction_defined);
        assert!(snapshot.raw_token_material_secure_store_only);
        assert!(!snapshot.raw_token_plaintext_mirror_allowed);
        assert!(!snapshot.local_support_record_raw_token_allowed);
        assert!(!snapshot.workspace_storage_allowed);
        assert!(!snapshot.log_storage_allowed);
        assert!(!snapshot.report_artifact_storage_allowed);
        assert!(!snapshot.ordinary_config_storage_allowed);
        assert!(snapshot.secure_store_commit_implemented);
        assert!(snapshot.secure_store_retrieval_implemented);
        assert!(snapshot.local_support_record_persistence_implemented);
        assert!(snapshot.local_support_engine_selected);
        assert_eq!(snapshot.supported_validation_statuses().len(), 5);
        assert!(snapshot.support_record_contract.schema_version_required);
        assert!(snapshot.support_record_contract.bounded_license_reference_required);
        assert!(snapshot.support_record_contract.bounded_activation_reference_required);
        assert!(snapshot.support_record_contract.slot_binding_context_required);
        assert!(snapshot.support_record_contract.token_lineage_marker_required);
        assert!(snapshot.support_record_contract.validity_marker_required);
        assert!(snapshot.support_record_contract.secure_store_lookup_reference_required);
        assert!(snapshot.support_record_contract.last_validation_status_required);
        assert!(snapshot.support_record_contract.last_validation_timestamp_required);
        assert!(!snapshot.support_record_contract.raw_device_bound_token_allowed);
    }
}