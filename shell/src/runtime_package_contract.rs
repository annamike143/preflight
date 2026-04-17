use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const RUNTIME_PACKAGE_CONTRACT_NAME: &str = "ManagedRuntimePackageContract";
pub const RUNTIME_PACKAGE_INSTALL_LOCATION_CLASS: &str = "AppControlledSupportLocation";
pub(crate) const RUNTIME_SUPPORT_DIRECTORY_NAME: &str = "runtime";

const RUNTIME_INSTALL_DIRECTORY_NAME: &str = "managed_runtime";
const RUNTIME_STAGING_INSTALL_DIRECTORY_NAME: &str = "managed_runtime_staging";
const RUNTIME_BACKUP_INSTALL_DIRECTORY_NAME: &str = "managed_runtime_previous";
const RUNTIME_INSTALL_RECORD_FILE_NAME: &str = "runtime_install_record.json";
const RUNTIME_EXPECTED_FILE_MANIFEST_FILE_NAME: &str = "runtime_expected_file_manifest.json";
const RUNTIME_VERSION_RECORD_FILE_NAME: &str = "runtime_version_record.json";
const RUNTIME_TRUST_MANIFEST_FILE_NAME: &str = "runtime_trust_manifest.json";
const RUNTIME_REJECTED_CANDIDATE_RECORD_FILE_NAME: &str = "runtime_rejected_candidate_record.json";
pub(crate) const RUNTIME_RECORD_SCHEMA_VERSION: u32 = 1;
pub(crate) const RUNTIME_TRUST_VERIFICATION_BASIS_NAME: &str = "sha256_expected_files_manifest";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeSupportPaths {
    pub root_dir: PathBuf,
    pub install_dir: PathBuf,
    pub staging_install_dir: PathBuf,
    pub backup_install_dir: PathBuf,
    pub install_record_path: PathBuf,
    pub expected_file_manifest_path: PathBuf,
    pub version_record_path: PathBuf,
    pub trust_manifest_path: PathBuf,
    pub rejected_candidate_record_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RuntimeInstallRecord {
    pub schema_version: u32,
    pub executable_relative_path: String,
    pub installed_at_epoch_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RuntimeExpectedFileRecord {
    pub relative_path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RuntimeExpectedFileManifest {
    pub schema_version: u32,
    pub runtime_version: String,
    pub files: Vec<RuntimeExpectedFileRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RuntimeVersionRecord {
    pub schema_version: u32,
    pub runtime_version: String,
    pub compatibility_target_os: String,
    pub compatibility_target_arch: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RuntimeTrustManifest {
    pub schema_version: u32,
    pub verification_basis_name: String,
    pub authorization_manifest_fingerprint_sha256: String,
    pub verified_file_count: usize,
    pub runtime_version: Option<String>,
    pub executable_relative_path: Option<String>,
    pub expected_file_manifest_fingerprint_sha256: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeArtifactRejectionReason {
    ManifestInvalid,
    MissingArtifact,
    ChecksumMismatch,
    CompatibilityMismatch,
}

impl RuntimeArtifactRejectionReason {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ManifestInvalid => "manifest_invalid",
            Self::MissingArtifact => "missing_artifact",
            Self::ChecksumMismatch => "checksum_mismatch",
            Self::CompatibilityMismatch => "compatibility_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RuntimeRejectedCandidateRecord {
    pub schema_version: u32,
    pub runtime_version: Option<String>,
    pub rejection_reason: RuntimeArtifactRejectionReason,
    pub rejection_detail: String,
    pub authorization_manifest_fingerprint_sha256: Option<String>,
    pub rejected_at_epoch_seconds: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct MaterializedRuntimePackage {
    pub install_record: RuntimeInstallRecord,
    pub expected_file_manifest: RuntimeExpectedFileManifest,
    pub version_record: RuntimeVersionRecord,
    pub trust_manifest: RuntimeTrustManifest,
}

#[derive(Debug, Clone)]
pub(crate) struct RuntimePackageMaterializationState {
    pub install_record: Option<RuntimeInstallRecord>,
    pub expected_file_manifest: Option<RuntimeExpectedFileManifest>,
    pub version_record: Option<RuntimeVersionRecord>,
    pub trust_manifest: Option<RuntimeTrustManifest>,
    pub rejected_candidate: Option<RuntimeRejectedCandidateRecord>,
}

impl RuntimePackageMaterializationState {
    fn unresolved() -> Self {
        Self {
            install_record: None,
            expected_file_manifest: None,
            version_record: None,
            trust_manifest: None,
            rejected_candidate: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RuntimePackageContractStage {
    ContractBasisDefined,
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimePackageContractSnapshot {
    pub stage: RuntimePackageContractStage,
    pub contract_name: &'static str,
    pub install_location_class_name: &'static str,
    pub persistent_installation_required: bool,
    pub install_location_shell_owned: bool,
    pub run_workspace_storage_allowed: bool,
    pub runtime_executable_class_required: bool,
    pub support_files_class_required: bool,
    pub version_record_required: bool,
    pub trust_manifest_required: bool,
    pub shell_presence_basis_defined: bool,
    pub shell_integrity_basis_defined: bool,
    pub managed_runtime_delivery_contract_defined: bool,
    pub managed_runtime_delivery_separate_from_app_updater: bool,
    pub verification_required_before_runtime_use: bool,
    pub active_run_mutation_forbidden: bool,
    pub runtime_download_flow_selected: bool,
    pub updater_toolchain_selected: bool,
    pub local_install_record_materialized: bool,
    pub local_expected_file_manifest_materialized: bool,
    pub local_version_record_materialized: bool,
    pub local_trust_manifest_materialized: bool,
}

impl RuntimePackageContractSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Runtime package contract basis now exists. Stage is {:?}; contract name is {}; install location class is {}; persistent installation required is {}; install location shell-owned is {}; run workspace storage allowed is {}; runtime executable class required is {}; support files class required is {}; version record required is {}; trust manifest required is {}; shell presence basis defined is {}; shell integrity basis defined is {}; managed runtime-delivery contract defined is {}; managed runtime-delivery separate from app updater is {}; verification required before runtime use is {}; active-run mutation forbidden is {}; runtime download flow selected is {}; updater toolchain selected is {}; local install record materialized is {}; local expected-file manifest materialized is {}; local version record materialized is {}; local trust manifest materialized is {}.",
            self.stage,
            self.contract_name,
            self.install_location_class_name,
            self.persistent_installation_required,
            self.install_location_shell_owned,
            self.run_workspace_storage_allowed,
            self.runtime_executable_class_required,
            self.support_files_class_required,
            self.version_record_required,
            self.trust_manifest_required,
            self.shell_presence_basis_defined,
            self.shell_integrity_basis_defined,
            self.managed_runtime_delivery_contract_defined,
            self.managed_runtime_delivery_separate_from_app_updater,
            self.verification_required_before_runtime_use,
            self.active_run_mutation_forbidden,
            self.runtime_download_flow_selected,
            self.updater_toolchain_selected,
            self.local_install_record_materialized,
            self.local_expected_file_manifest_materialized,
            self.local_version_record_materialized,
            self.local_trust_manifest_materialized
        )
    }
}

pub(crate) fn default_runtime_support_paths() -> Result<RuntimeSupportPaths, String> {
    let project_dirs = ProjectDirs::from("com", "MiroFish", "miro-fish-desktop-saas")
        .ok_or_else(|| "unable to resolve app-controlled managed-runtime support directory".to_string())?;

    Ok(runtime_support_paths_from_root(
        project_dirs.data_local_dir().join(RUNTIME_SUPPORT_DIRECTORY_NAME),
    ))
}

pub(crate) fn runtime_support_paths_from_root(root_dir: PathBuf) -> RuntimeSupportPaths {
    RuntimeSupportPaths {
        install_dir: root_dir.join(RUNTIME_INSTALL_DIRECTORY_NAME),
        staging_install_dir: root_dir.join(RUNTIME_STAGING_INSTALL_DIRECTORY_NAME),
        backup_install_dir: root_dir.join(RUNTIME_BACKUP_INSTALL_DIRECTORY_NAME),
        install_record_path: root_dir.join(RUNTIME_INSTALL_RECORD_FILE_NAME),
        expected_file_manifest_path: root_dir.join(RUNTIME_EXPECTED_FILE_MANIFEST_FILE_NAME),
        version_record_path: root_dir.join(RUNTIME_VERSION_RECORD_FILE_NAME),
        trust_manifest_path: root_dir.join(RUNTIME_TRUST_MANIFEST_FILE_NAME),
        rejected_candidate_record_path: root_dir.join(RUNTIME_REJECTED_CANDIDATE_RECORD_FILE_NAME),
        root_dir,
    }
}

pub(crate) fn runtime_package_materialization_state() -> RuntimePackageMaterializationState {
    match default_runtime_support_paths()
        .and_then(|paths| runtime_package_materialization_state_from_paths(&paths))
    {
        Ok(state) => state,
        Err(_) => RuntimePackageMaterializationState::unresolved(),
    }
}

pub(crate) fn runtime_package_materialization_state_from_paths(
    paths: &RuntimeSupportPaths,
) -> Result<RuntimePackageMaterializationState, String> {
    Ok(RuntimePackageMaterializationState {
        install_record: load_json_file(&paths.install_record_path)?,
        expected_file_manifest: load_json_file(&paths.expected_file_manifest_path)?,
        version_record: load_json_file(&paths.version_record_path)?,
        trust_manifest: load_json_file(&paths.trust_manifest_path)?,
        rejected_candidate: load_json_file(&paths.rejected_candidate_record_path)?,
    })
}

pub(crate) fn persist_materialized_runtime_package(
    paths: &RuntimeSupportPaths,
    package: &MaterializedRuntimePackage,
) -> Result<(), String> {
    fs::create_dir_all(&paths.root_dir).map_err(|error| error.to_string())?;
    write_json_file(&paths.install_record_path, &package.install_record)?;
    write_json_file(
        &paths.expected_file_manifest_path,
        &package.expected_file_manifest,
    )?;
    write_json_file(&paths.version_record_path, &package.version_record)?;
    write_json_file(&paths.trust_manifest_path, &package.trust_manifest)
}

pub(crate) fn persist_rejected_runtime_candidate(
    paths: &RuntimeSupportPaths,
    rejection: &RuntimeRejectedCandidateRecord,
) -> Result<(), String> {
    write_json_file(&paths.rejected_candidate_record_path, rejection)
}

pub(crate) fn clear_rejected_runtime_candidate(paths: &RuntimeSupportPaths) -> Result<(), String> {
    remove_file_if_exists(&paths.rejected_candidate_record_path)
}

pub(crate) fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

pub(crate) fn runtime_expected_file_manifest_fingerprint(
    manifest: &RuntimeExpectedFileManifest,
) -> Result<String, String> {
    let serialized = serde_json::to_vec(manifest).map_err(|error| error.to_string())?;
    Ok(hex_sha256(&serialized))
}

pub fn runtime_package_contract_basis() -> RuntimePackageContractSnapshot {
    match default_runtime_support_paths() {
        Ok(paths) => runtime_package_contract_basis_from_paths(&paths),
        Err(_) => project_runtime_package_contract_snapshot(RuntimePackageMaterializationState::unresolved()),
    }
}

pub(crate) fn runtime_package_contract_basis_from_paths(
    paths: &RuntimeSupportPaths,
) -> RuntimePackageContractSnapshot {
    let materialization_state = runtime_package_materialization_state_from_paths(paths)
        .unwrap_or_else(|_| RuntimePackageMaterializationState::unresolved());

    project_runtime_package_contract_snapshot(materialization_state)
}

fn project_runtime_package_contract_snapshot(
    materialization_state: RuntimePackageMaterializationState,
) -> RuntimePackageContractSnapshot {
    RuntimePackageContractSnapshot {
        stage: RuntimePackageContractStage::ContractBasisDefined,
        contract_name: RUNTIME_PACKAGE_CONTRACT_NAME,
        install_location_class_name: RUNTIME_PACKAGE_INSTALL_LOCATION_CLASS,
        persistent_installation_required: true,
        install_location_shell_owned: true,
        run_workspace_storage_allowed: false,
        runtime_executable_class_required: true,
        support_files_class_required: true,
        version_record_required: true,
        trust_manifest_required: true,
        shell_presence_basis_defined: true,
        shell_integrity_basis_defined: true,
        managed_runtime_delivery_contract_defined: true,
        managed_runtime_delivery_separate_from_app_updater: true,
        verification_required_before_runtime_use: true,
        active_run_mutation_forbidden: true,
        runtime_download_flow_selected: false,
        updater_toolchain_selected: false,
        local_install_record_materialized: materialization_state.install_record.is_some(),
        local_expected_file_manifest_materialized: materialization_state
            .expected_file_manifest
            .is_some(),
        local_version_record_materialized: materialization_state.version_record.is_some(),
        local_trust_manifest_materialized: materialization_state.trust_manifest.is_some(),
    }
}

fn load_json_file<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, String> {
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let value = serde_json::from_str(&content).map_err(|error| error.to_string())?;
    Ok(Some(value))
}

fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir).map_err(|error| error.to_string())?;
    }

    let serialized = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    fs::write(path, serialized).map_err(|error| error.to_string())
}

fn remove_file_if_exists(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }

    fs::remove_file(path).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn runtime_package_contract_basis_freezes_local_verification_basis_without_selecting_delivery_toolchain() {
        let snapshot = runtime_package_contract_basis();

        assert_eq!(snapshot.contract_name, RUNTIME_PACKAGE_CONTRACT_NAME);
        assert_eq!(
            snapshot.install_location_class_name,
            RUNTIME_PACKAGE_INSTALL_LOCATION_CLASS
        );
        assert!(snapshot.persistent_installation_required);
        assert!(snapshot.install_location_shell_owned);
        assert!(!snapshot.run_workspace_storage_allowed);
        assert!(snapshot.runtime_executable_class_required);
        assert!(snapshot.support_files_class_required);
        assert!(snapshot.version_record_required);
        assert!(snapshot.trust_manifest_required);
        assert!(snapshot.shell_presence_basis_defined);
        assert!(snapshot.shell_integrity_basis_defined);
        assert!(snapshot.managed_runtime_delivery_contract_defined);
        assert!(snapshot.managed_runtime_delivery_separate_from_app_updater);
        assert!(snapshot.verification_required_before_runtime_use);
        assert!(snapshot.active_run_mutation_forbidden);
        assert!(!snapshot.runtime_download_flow_selected);
        assert!(!snapshot.updater_toolchain_selected);
    }

    #[test]
    fn runtime_package_contract_reflects_materialized_runtime_records() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let paths = runtime_support_paths_from_root(temp_dir.path().join(RUNTIME_SUPPORT_DIRECTORY_NAME));
        let package = MaterializedRuntimePackage {
            install_record: RuntimeInstallRecord {
                schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                executable_relative_path: "bin/python.exe".to_string(),
                installed_at_epoch_seconds: 100,
            },
            expected_file_manifest: RuntimeExpectedFileManifest {
                schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                runtime_version: "3.11.9".to_string(),
                files: vec![
                    RuntimeExpectedFileRecord {
                        relative_path: "bin/python.exe".to_string(),
                        sha256: "abc".to_string(),
                    },
                    RuntimeExpectedFileRecord {
                        relative_path: "lib/runtime.zip".to_string(),
                        sha256: "def".to_string(),
                    },
                ],
            },
            version_record: RuntimeVersionRecord {
                schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                runtime_version: "3.11.9".to_string(),
                compatibility_target_os: std::env::consts::OS.to_string(),
                compatibility_target_arch: std::env::consts::ARCH.to_string(),
            },
            trust_manifest: RuntimeTrustManifest {
                schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                verification_basis_name: RUNTIME_TRUST_VERIFICATION_BASIS_NAME.to_string(),
                authorization_manifest_fingerprint_sha256: "manifest-fingerprint".to_string(),
                verified_file_count: 2,
                runtime_version: Some("3.11.9".to_string()),
                executable_relative_path: Some("bin/python.exe".to_string()),
                expected_file_manifest_fingerprint_sha256: Some(
                    runtime_expected_file_manifest_fingerprint(&RuntimeExpectedFileManifest {
                        schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                        runtime_version: "3.11.9".to_string(),
                        files: vec![
                            RuntimeExpectedFileRecord {
                                relative_path: "bin/python.exe".to_string(),
                                sha256: "abc".to_string(),
                            },
                            RuntimeExpectedFileRecord {
                                relative_path: "lib/runtime.zip".to_string(),
                                sha256: "def".to_string(),
                            },
                        ],
                    })
                    .expect("expected-file manifest fingerprint should serialize"),
                ),
            },
        };

        persist_materialized_runtime_package(&paths, &package)
            .expect("runtime package persistence should succeed");

        let snapshot = runtime_package_contract_basis_from_paths(&paths);

        assert!(snapshot.local_install_record_materialized);
        assert!(snapshot.local_expected_file_manifest_materialized);
        assert!(snapshot.local_version_record_materialized);
        assert!(snapshot.local_trust_manifest_materialized);
        assert!(!snapshot.runtime_download_flow_selected);
    }
}