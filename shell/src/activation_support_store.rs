use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::activation_storage_boundary::{
    ActivationValidationStatus, ACTIVATION_LOCAL_SUPPORT_DOMAIN_NAME,
};

pub const ACTIVATION_SUPPORT_STORE_ENGINE_NAME: &str = "JsonFileActivationSupportRecordStore";
pub const ACTIVATION_SUPPORT_RECORD_FILE_NAME: &str = "activation_support_record.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationSupportStoreStage {
    PersistenceMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationSupportStoreSnapshot {
    pub stage: ActivationSupportStoreStage,
    pub support_domain_name: &'static str,
    pub support_engine_name: &'static str,
    pub persistence_implemented: bool,
    pub recovery_implemented: bool,
    pub record_path_resolution_available: bool,
    pub raw_token_persistence_allowed: bool,
}

impl ActivationSupportStoreSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Activation support-store surface is materialized. Stage is {:?}; support domain name is {}; support engine name is {}; persistence implemented is {}; recovery implemented is {}; record path resolution available is {}; raw token persistence allowed is {}.",
            self.stage,
            self.support_domain_name,
            self.support_engine_name,
            self.persistence_implemented,
            self.recovery_implemented,
            self.record_path_resolution_available,
            self.raw_token_persistence_allowed
        )
    }
}

pub fn activation_support_store_surface() -> ActivationSupportStoreSnapshot {
    ActivationSupportStoreSnapshot {
        stage: ActivationSupportStoreStage::PersistenceMaterialized,
        support_domain_name: ACTIVATION_LOCAL_SUPPORT_DOMAIN_NAME,
        support_engine_name: ACTIVATION_SUPPORT_STORE_ENGINE_NAME,
        persistence_implemented: true,
        recovery_implemented: true,
        record_path_resolution_available: ActivationSupportStore::default_record_path().is_ok(),
        raw_token_persistence_allowed: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationSupportValidityMarker {
    Active,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivationSupportRecord {
    pub schema_version: u32,
    pub license_reference: String,
    pub activation_reference: String,
    pub activation_slot_index: u32,
    pub plan_tier: String,
    pub device_hash: String,
    pub key_identifier: String,
    pub token_lineage_marker: String,
    pub token_fingerprint_sha256: String,
    pub validity_marker: ActivationSupportValidityMarker,
    pub secure_store_lookup_reference: String,
    pub issued_timestamp_epoch_seconds: u64,
    pub expiration_timestamp_epoch_seconds: Option<u64>,
    pub last_validation_status: ActivationValidationStatus,
    pub last_validation_timestamp_epoch_seconds: u64,
}

pub struct ActivationSupportStore {
    record_path: PathBuf,
}

impl ActivationSupportStore {
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
            .ok_or_else(|| "unable to resolve app-controlled local support directory".to_string())?;

        Ok(project_dirs
            .data_local_dir()
            .join("activation")
            .join(ACTIVATION_SUPPORT_RECORD_FILE_NAME))
    }

    pub fn persist_record(&self, record: &ActivationSupportRecord) -> Result<(), String> {
        if let Some(parent_dir) = self.record_path.parent() {
            fs::create_dir_all(parent_dir).map_err(|error| error.to_string())?;
        }

        let serialized = serde_json::to_string_pretty(record).map_err(|error| error.to_string())?;
        fs::write(&self.record_path, serialized).map_err(|error| error.to_string())
    }

    pub fn load_record(&self) -> Result<Option<ActivationSupportRecord>, String> {
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
    fn activation_support_store_surface_stays_bounded_to_non_secret_metadata() {
        let snapshot = activation_support_store_surface();

        assert_eq!(snapshot.support_domain_name, ACTIVATION_LOCAL_SUPPORT_DOMAIN_NAME);
        assert_eq!(snapshot.support_engine_name, ACTIVATION_SUPPORT_STORE_ENGINE_NAME);
        assert!(snapshot.persistence_implemented);
        assert!(snapshot.recovery_implemented);
        assert!(!snapshot.raw_token_persistence_allowed);
    }

    #[test]
    fn activation_support_store_persists_and_recovers_record_without_raw_token() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let record_path = temp_dir.path().join(ACTIVATION_SUPPORT_RECORD_FILE_NAME);
        let store = ActivationSupportStore::from_record_path(record_path.clone());
        let record = ActivationSupportRecord {
            schema_version: 1,
            license_reference: "license-001".to_string(),
            activation_reference: "activation-001".to_string(),
            activation_slot_index: 1,
            plan_tier: "annual".to_string(),
            device_hash: "device-hash-001".to_string(),
            key_identifier: "kid-001".to_string(),
            token_lineage_marker: "jti-001".to_string(),
            token_fingerprint_sha256: "fingerprint-001".to_string(),
            validity_marker: ActivationSupportValidityMarker::Active,
            secure_store_lookup_reference: "device-bound-activation-token".to_string(),
            issued_timestamp_epoch_seconds: 100,
            expiration_timestamp_epoch_seconds: Some(200),
            last_validation_status: ActivationValidationStatus::ActivatedValid,
            last_validation_timestamp_epoch_seconds: 100,
        };

        store.persist_record(&record).expect("persist should succeed");
        let recovered = store
            .load_record()
            .expect("load should succeed")
            .expect("record should exist");

        assert_eq!(recovered, record);

        let content = fs::read_to_string(record_path).expect("record content should exist");
        assert!(!content.contains("header.payload.signature"));

        store.clear_record().expect("clear should succeed");
        assert!(store.load_record().expect("load should succeed").is_none());
    }
}