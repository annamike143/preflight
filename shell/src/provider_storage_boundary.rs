use crate::provider_secure_store::{
    provider_secure_store_surface, PROVIDER_SECURE_STORE_ABSTRACTION_NAME,
    PROVIDER_SECURE_STORE_DOMAIN_NAME,
};
use crate::provider_support_store::{
    provider_support_store_surface, PROVIDER_LOCAL_SUPPORT_DOMAIN_NAME,
    PROVIDER_SUPPORT_STATE_NAME,
};

pub const PROVIDER_STORAGE_BOUNDARY_NAME: &str = "ProviderSecretStorageBoundary";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderStorageBoundaryStage {
    Materialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderSupportStateContract {
    pub schema_version_required: bool,
    pub selected_provider_required: bool,
    pub secure_store_lookup_reference_required: bool,
    pub configuration_status_required: bool,
    pub last_configuration_outcome_supported: bool,
    pub raw_provider_secret_allowed: bool,
    pub preflight_claims_allowed: bool,
    pub report_embedding_allowed: bool,
    pub ordinary_log_embedding_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderStorageBoundarySnapshot {
    pub stage: ProviderStorageBoundaryStage,
    pub boundary_name: &'static str,
    pub secure_store_domain_name: &'static str,
    pub local_support_domain_name: &'static str,
    pub secure_store_abstraction_name: &'static str,
    pub support_state_name: &'static str,
    pub local_support_engine_name: &'static str,
    pub secure_store_shell_owned: bool,
    pub secure_store_abstraction_defined: bool,
    pub raw_provider_secret_secure_store_only: bool,
    pub raw_provider_secret_plaintext_mirror_allowed: bool,
    pub support_state_raw_secret_allowed: bool,
    pub workspace_storage_allowed: bool,
    pub log_storage_allowed: bool,
    pub report_storage_allowed: bool,
    pub ordinary_config_storage_allowed: bool,
    pub secure_store_commit_implemented: bool,
    pub secure_store_retrieval_implemented: bool,
    pub secure_store_delete_implemented: bool,
    pub local_support_engine_selected: bool,
    pub support_state_persistence_implemented: bool,
    pub support_state_contract: ProviderSupportStateContract,
}

impl ProviderStorageBoundarySnapshot {
    pub fn summary(self) -> String {
        format!(
            "Provider storage-boundary surface is materialized. Stage is {:?}; boundary name is {}; secure-store domain is {}; local support domain is {}; secure-store abstraction name is {}; support-state name is {}; local support engine name is {}; secure-store shell-owned is {}; secure-store abstraction defined is {}; raw provider secret secure-store only is {}; raw provider secret plaintext mirror allowed is {}; support-state raw secret allowed is {}; workspace storage allowed is {}; log storage allowed is {}; report storage allowed is {}; ordinary config storage allowed is {}; secure-store commit implemented is {}; secure-store retrieval implemented is {}; secure-store delete implemented is {}; local support engine selected is {}; support-state persistence implemented is {}; support-state requires schema version is {}; support-state requires selected provider is {}; support-state requires secure-store lookup reference is {}; support-state requires configuration status is {}; support-state supports last configuration outcome is {}; support-state allows raw provider secret is {}; support-state allows Preflight claims is {}; support-state allows report embedding is {}; support-state allows ordinary log embedding is {}.",
            self.stage,
            self.boundary_name,
            self.secure_store_domain_name,
            self.local_support_domain_name,
            self.secure_store_abstraction_name,
            self.support_state_name,
            self.local_support_engine_name,
            self.secure_store_shell_owned,
            self.secure_store_abstraction_defined,
            self.raw_provider_secret_secure_store_only,
            self.raw_provider_secret_plaintext_mirror_allowed,
            self.support_state_raw_secret_allowed,
            self.workspace_storage_allowed,
            self.log_storage_allowed,
            self.report_storage_allowed,
            self.ordinary_config_storage_allowed,
            self.secure_store_commit_implemented,
            self.secure_store_retrieval_implemented,
            self.secure_store_delete_implemented,
            self.local_support_engine_selected,
            self.support_state_persistence_implemented,
            self.support_state_contract.schema_version_required,
            self.support_state_contract.selected_provider_required,
            self.support_state_contract.secure_store_lookup_reference_required,
            self.support_state_contract.configuration_status_required,
            self.support_state_contract.last_configuration_outcome_supported,
            self.support_state_contract.raw_provider_secret_allowed,
            self.support_state_contract.preflight_claims_allowed,
            self.support_state_contract.report_embedding_allowed,
            self.support_state_contract.ordinary_log_embedding_allowed
        )
    }
}

pub fn provider_storage_boundary_surface() -> ProviderStorageBoundarySnapshot {
    let secure_store_snapshot = provider_secure_store_surface();
    let support_store_snapshot = provider_support_store_surface();

    ProviderStorageBoundarySnapshot {
        stage: ProviderStorageBoundaryStage::Materialized,
        boundary_name: PROVIDER_STORAGE_BOUNDARY_NAME,
        secure_store_domain_name: PROVIDER_SECURE_STORE_DOMAIN_NAME,
        local_support_domain_name: PROVIDER_LOCAL_SUPPORT_DOMAIN_NAME,
        secure_store_abstraction_name: PROVIDER_SECURE_STORE_ABSTRACTION_NAME,
        support_state_name: PROVIDER_SUPPORT_STATE_NAME,
        local_support_engine_name: support_store_snapshot.support_engine_name,
        secure_store_shell_owned: true,
        secure_store_abstraction_defined: true,
        raw_provider_secret_secure_store_only: true,
        raw_provider_secret_plaintext_mirror_allowed: false,
        support_state_raw_secret_allowed: false,
        workspace_storage_allowed: false,
        log_storage_allowed: false,
        report_storage_allowed: false,
        ordinary_config_storage_allowed: false,
        secure_store_commit_implemented: secure_store_snapshot.secure_store_commit_implemented,
        secure_store_retrieval_implemented: secure_store_snapshot
            .secure_store_retrieval_implemented,
        secure_store_delete_implemented: secure_store_snapshot.secure_store_delete_implemented,
        local_support_engine_selected: true,
        support_state_persistence_implemented: support_store_snapshot.persistence_implemented,
        support_state_contract: ProviderSupportStateContract {
            schema_version_required: true,
            selected_provider_required: true,
            secure_store_lookup_reference_required: true,
            configuration_status_required: true,
            last_configuration_outcome_supported: true,
            raw_provider_secret_allowed: false,
            preflight_claims_allowed: false,
            report_embedding_allowed: false,
            ordinary_log_embedding_allowed: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_storage_boundary_surface_keeps_provider_secrets_out_of_ordinary_persistence() {
        let snapshot = provider_storage_boundary_surface();

        assert_eq!(snapshot.boundary_name, PROVIDER_STORAGE_BOUNDARY_NAME);
        assert_eq!(snapshot.secure_store_domain_name, PROVIDER_SECURE_STORE_DOMAIN_NAME);
        assert_eq!(snapshot.local_support_domain_name, PROVIDER_LOCAL_SUPPORT_DOMAIN_NAME);
        assert_eq!(snapshot.secure_store_abstraction_name, PROVIDER_SECURE_STORE_ABSTRACTION_NAME);
        assert_eq!(snapshot.support_state_name, PROVIDER_SUPPORT_STATE_NAME);
        assert_eq!(
            snapshot.local_support_engine_name,
            crate::provider_support_store::PROVIDER_SUPPORT_STORE_ENGINE_NAME
        );
        assert!(snapshot.secure_store_shell_owned);
        assert!(snapshot.secure_store_abstraction_defined);
        assert!(snapshot.raw_provider_secret_secure_store_only);
        assert!(!snapshot.raw_provider_secret_plaintext_mirror_allowed);
        assert!(!snapshot.support_state_raw_secret_allowed);
        assert!(!snapshot.workspace_storage_allowed);
        assert!(!snapshot.log_storage_allowed);
        assert!(!snapshot.report_storage_allowed);
        assert!(!snapshot.ordinary_config_storage_allowed);
        assert!(snapshot.secure_store_commit_implemented);
        assert!(snapshot.secure_store_retrieval_implemented);
        assert!(snapshot.secure_store_delete_implemented);
        assert!(snapshot.local_support_engine_selected);
        assert!(snapshot.support_state_persistence_implemented);
        assert!(snapshot.support_state_contract.schema_version_required);
        assert!(snapshot.support_state_contract.selected_provider_required);
        assert!(snapshot.support_state_contract.secure_store_lookup_reference_required);
        assert!(snapshot.support_state_contract.configuration_status_required);
        assert!(snapshot.support_state_contract.last_configuration_outcome_supported);
        assert!(!snapshot.support_state_contract.raw_provider_secret_allowed);
        assert!(!snapshot.support_state_contract.preflight_claims_allowed);
        assert!(!snapshot.support_state_contract.report_embedding_allowed);
        assert!(!snapshot.support_state_contract.ordinary_log_embedding_allowed);
    }
}