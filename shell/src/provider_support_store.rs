use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::provider_configuration_contract::{ProviderConfigurationStatus, SupportedProvider};

pub const PROVIDER_LOCAL_SUPPORT_DOMAIN_NAME: &str = "LocalPersistentSupportDomain";
pub const PROVIDER_SUPPORT_STATE_NAME: &str = "ProviderConfigurationSupportState";
pub const PROVIDER_SUPPORT_STORE_ENGINE_NAME: &str =
    "JsonFileProviderConfigurationSupportStateStore";
pub const PROVIDER_SUPPORT_RECORD_FILE_NAME: &str =
    "provider_configuration_support_state.json";
pub const PROVIDER_SUPPORT_STATE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderSupportStoreStage {
    PersistenceMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderSupportStoreSnapshot {
    pub stage: ProviderSupportStoreStage,
    pub support_domain_name: &'static str,
    pub support_state_name: &'static str,
    pub support_engine_name: &'static str,
    pub persistence_implemented: bool,
    pub recovery_implemented: bool,
    pub record_path_resolution_available: bool,
    pub raw_provider_secret_persistence_allowed: bool,
}

pub fn provider_support_store_surface() -> ProviderSupportStoreSnapshot {
    ProviderSupportStoreSnapshot {
        stage: ProviderSupportStoreStage::PersistenceMaterialized,
        support_domain_name: PROVIDER_LOCAL_SUPPORT_DOMAIN_NAME,
        support_state_name: PROVIDER_SUPPORT_STATE_NAME,
        support_engine_name: PROVIDER_SUPPORT_STORE_ENGINE_NAME,
        persistence_implemented: true,
        recovery_implemented: true,
        record_path_resolution_available: ProviderSupportStore::default_record_path().is_ok(),
        raw_provider_secret_persistence_allowed: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderConfigurationOutcomeMarker {
    ConfiguredLocally,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProviderSupportStateRecord {
    pub schema_version: u32,
    pub selected_provider: SupportedProvider,
    pub secure_store_lookup_reference: String,
    pub provider_secret_fingerprint_sha256: String,
    pub configuration_status: ProviderConfigurationStatus,
    pub last_configuration_outcome: ProviderConfigurationOutcomeMarker,
    pub last_updated_timestamp_epoch_seconds: u64,
}

pub struct ProviderSupportStore {
    record_path: PathBuf,
}

impl ProviderSupportStore {
    pub fn default_store() -> Result<Self, String> {
        Ok(Self {
            record_path: Self::default_record_path()?,
        })
    }

    #[cfg(test)]
    pub fn from_record_path(record_path: PathBuf) -> Self {
        Self { record_path }
    }

    pub fn default_record_path() -> Result<PathBuf, String> {
        let project_dirs = ProjectDirs::from("com", "MiroFish", "miro-fish-desktop-saas")
            .ok_or_else(|| "unable to resolve provider support-state directory".to_string())?;

        Ok(project_dirs
            .data_local_dir()
            .join("provider")
            .join(PROVIDER_SUPPORT_RECORD_FILE_NAME))
    }

    pub fn persist_record(&self, record: &ProviderSupportStateRecord) -> Result<(), String> {
        if let Some(parent_dir) = self.record_path.parent() {
            fs::create_dir_all(parent_dir).map_err(|error| error.to_string())?;
        }

        let serialized = serde_json::to_string_pretty(record).map_err(|error| error.to_string())?;
        fs::write(&self.record_path, serialized).map_err(|error| error.to_string())
    }

    pub fn load_record(&self) -> Result<Option<ProviderSupportStateRecord>, String> {
        if !self.record_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.record_path).map_err(|error| error.to_string())?;
        let record = serde_json::from_str(&content).map_err(|error| error.to_string())?;
        Ok(Some(record))
    }

    pub fn clear_record(&self) -> Result<(), String> {
        if !self.record_path.exists() {
            return Ok(());
        }

        fs::remove_file(&self.record_path).map_err(|error| error.to_string())
    }

    pub fn record_path(&self) -> &Path {
        &self.record_path
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn provider_support_store_surface_remains_bounded_to_non_secret_state() {
        let snapshot = provider_support_store_surface();

        assert_eq!(snapshot.support_domain_name, PROVIDER_LOCAL_SUPPORT_DOMAIN_NAME);
        assert_eq!(snapshot.support_state_name, PROVIDER_SUPPORT_STATE_NAME);
        assert_eq!(snapshot.support_engine_name, PROVIDER_SUPPORT_STORE_ENGINE_NAME);
        assert!(snapshot.persistence_implemented);
        assert!(snapshot.recovery_implemented);
        assert!(!snapshot.raw_provider_secret_persistence_allowed);
    }

    #[test]
    fn provider_support_store_persists_and_recovers_non_secret_configuration_state() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let record_path = temp_dir.path().join(PROVIDER_SUPPORT_RECORD_FILE_NAME);
        let store = ProviderSupportStore::from_record_path(record_path.clone());
        let record = ProviderSupportStateRecord {
            schema_version: PROVIDER_SUPPORT_STATE_SCHEMA_VERSION,
            selected_provider: SupportedProvider::OpenAi,
            secure_store_lookup_reference: "provider-api-key.open_ai".to_string(),
            provider_secret_fingerprint_sha256: "fingerprint-001".to_string(),
            configuration_status: ProviderConfigurationStatus::Present,
            last_configuration_outcome: ProviderConfigurationOutcomeMarker::ConfiguredLocally,
            last_updated_timestamp_epoch_seconds: 100,
        };

        store.persist_record(&record).expect("persist should succeed");
        let recovered = store
            .load_record()
            .expect("load should succeed")
            .expect("record should exist");

        assert_eq!(recovered, record);

        let content = fs::read_to_string(record_path).expect("record content should exist");
        assert!(!content.contains("sk-test-secret-001"));

        store.clear_record().expect("clear should succeed");
        assert!(store.load_record().expect("load should succeed").is_none());
    }
}