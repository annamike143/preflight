use std::fs;
use std::path::PathBuf;

use crate::runtime_package_contract::{
    default_runtime_support_paths, hex_sha256, runtime_expected_file_manifest_fingerprint,
    runtime_package_contract_basis, runtime_package_contract_basis_from_paths,
    runtime_package_materialization_state_from_paths, RuntimePackageContractSnapshot,
    RuntimeSupportPaths, RUNTIME_TRUST_VERIFICATION_BASIS_NAME,
};

#[derive(Debug, Clone, Copy)]
pub enum RuntimeIntegrityStage {
    VerificationMaterialized,
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimeIntegritySnapshot {
    pub stage: RuntimeIntegrityStage,
    pub runtime_package_contract: RuntimePackageContractSnapshot,
    pub integrity_basis_defined: bool,
    pub runtime_delivery_contract_defined: bool,
    pub pre_install_verification_started: bool,
    pub post_install_verification_started: bool,
    pub expected_files_contract_defined: bool,
    pub version_contract_defined: bool,
    pub checksum_or_signature_required: bool,
    pub compatibility_basis_defined: bool,
    pub trust_rejection_distinct_from_technical_failure: bool,
    pub runtime_use_blocked_until_verified: bool,
    pub expected_files_verified: bool,
    pub version_compatibility_verified: bool,
    pub checksum_or_signature_verified: bool,
    pub runtime_integrity_passed: bool,
    pub execution_unblocked: bool,
}

impl RuntimeIntegritySnapshot {
    pub fn summary(self) -> String {
        format!(
            "Runtime integrity surface is materialized. Stage is {:?}; integrity basis defined is {}; runtime-delivery contract defined is {}; contract name is {}; pre-install verification started is {}; post-install verification started is {}; expected files contract defined is {}; version contract defined is {}; checksum or signature required is {}; compatibility basis defined is {}; trust rejection distinct from technical failure is {}; runtime use blocked until verified is {}; expected files verified is {}; version compatibility verified is {}; checksum or signature verified is {}; runtime integrity passed is {}; execution unblocked is {}.",
            self.stage,
            self.integrity_basis_defined,
            self.runtime_delivery_contract_defined,
            self.runtime_package_contract.contract_name,
            self.pre_install_verification_started,
            self.post_install_verification_started,
            self.expected_files_contract_defined,
            self.version_contract_defined,
            self.checksum_or_signature_required,
            self.compatibility_basis_defined,
            self.trust_rejection_distinct_from_technical_failure,
            self.runtime_use_blocked_until_verified,
            self.expected_files_verified,
            self.version_compatibility_verified,
            self.checksum_or_signature_verified,
            self.runtime_integrity_passed,
            self.execution_unblocked
        )
    }
}

pub fn runtime_integrity_state() -> RuntimeIntegritySnapshot {
    match default_runtime_support_paths() {
        Ok(paths) => runtime_integrity_state_from_paths(&paths),
        Err(_) => RuntimeIntegritySnapshot {
            stage: RuntimeIntegrityStage::VerificationMaterialized,
            runtime_package_contract: runtime_package_contract_basis(),
            integrity_basis_defined: false,
            runtime_delivery_contract_defined: true,
            pre_install_verification_started: false,
            post_install_verification_started: false,
            expected_files_contract_defined: true,
            version_contract_defined: true,
            checksum_or_signature_required: true,
            compatibility_basis_defined: true,
            trust_rejection_distinct_from_technical_failure: true,
            runtime_use_blocked_until_verified: true,
            expected_files_verified: false,
            version_compatibility_verified: false,
            checksum_or_signature_verified: false,
            runtime_integrity_passed: false,
            execution_unblocked: false,
        },
    }
}

pub(crate) fn runtime_integrity_state_from_paths(
    paths: &RuntimeSupportPaths,
) -> RuntimeIntegritySnapshot {
    let runtime_package_contract = runtime_package_contract_basis_from_paths(paths);
    let materialization_state = runtime_package_materialization_state_from_paths(paths).ok();
    let expected_files_verified = materialization_state
        .as_ref()
        .map(|state| verify_expected_files(paths, state))
        .unwrap_or(false);
    let version_compatibility_verified = materialization_state
        .as_ref()
        .and_then(|state| state.version_record.as_ref())
        .map(|record| {
            record.compatibility_target_os == std::env::consts::OS
                && record.compatibility_target_arch == std::env::consts::ARCH
        })
        .unwrap_or(false);
    let checksum_or_signature_verified = expected_files_verified
        && materialization_state
            .as_ref()
            .map(verify_trust_manifest)
            .unwrap_or(false);
    let runtime_integrity_passed = expected_files_verified
        && version_compatibility_verified
        && checksum_or_signature_verified
        && runtime_package_contract.local_install_record_materialized;
    let pre_install_verification_started = materialization_state
        .as_ref()
        .and_then(|state| state.rejected_candidate.as_ref())
        .is_some()
        || runtime_package_contract.local_trust_manifest_materialized;

    RuntimeIntegritySnapshot {
        stage: RuntimeIntegrityStage::VerificationMaterialized,
        runtime_package_contract,
        integrity_basis_defined: true,
        runtime_delivery_contract_defined: true,
        pre_install_verification_started,
        post_install_verification_started: runtime_package_contract.local_trust_manifest_materialized,
        expected_files_contract_defined: runtime_package_contract.runtime_executable_class_required
            && runtime_package_contract.support_files_class_required,
        version_contract_defined: runtime_package_contract.version_record_required,
        checksum_or_signature_required: runtime_package_contract.trust_manifest_required,
        compatibility_basis_defined: true,
        trust_rejection_distinct_from_technical_failure: true,
        runtime_use_blocked_until_verified: runtime_package_contract
            .verification_required_before_runtime_use,
        expected_files_verified,
        version_compatibility_verified,
        checksum_or_signature_verified,
        runtime_integrity_passed,
        execution_unblocked: runtime_integrity_passed,
    }
}

fn verify_expected_files(
    paths: &RuntimeSupportPaths,
    state: &crate::runtime_package_contract::RuntimePackageMaterializationState,
) -> bool {
    let Some(expected_file_manifest) = state.expected_file_manifest.as_ref() else {
        return false;
    };

    expected_file_manifest.files.iter().all(|file| {
        let file_path = paths.install_dir.join(relative_path_to_path_buf(&file.relative_path));
        let Ok(bytes) = fs::read(file_path) else {
            return false;
        };

        hex_sha256(&bytes) == file.sha256
    })
}

fn verify_trust_manifest(
    state: &crate::runtime_package_contract::RuntimePackageMaterializationState,
) -> bool {
    let Some(expected_file_manifest) = state.expected_file_manifest.as_ref() else {
        return false;
    };
    let Some(version_record) = state.version_record.as_ref() else {
        return false;
    };
    let Some(install_record) = state.install_record.as_ref() else {
        return false;
    };
    let Some(trust_manifest) = state.trust_manifest.as_ref() else {
        return false;
    };

    if trust_manifest.verification_basis_name != RUNTIME_TRUST_VERIFICATION_BASIS_NAME {
        return false;
    }

    if trust_manifest.verified_file_count == 0
        || trust_manifest.verified_file_count != expected_file_manifest.files.len()
    {
        return false;
    }

    if expected_file_manifest.runtime_version != version_record.runtime_version {
        return false;
    }

    if trust_manifest.runtime_version.as_deref() != Some(version_record.runtime_version.as_str()) {
        return false;
    }

    if trust_manifest.executable_relative_path.as_deref()
        != Some(install_record.executable_relative_path.as_str())
    {
        return false;
    }

    if !expected_file_manifest
        .files
        .iter()
        .any(|file| file.relative_path == install_record.executable_relative_path)
    {
        return false;
    }

    let Ok(expected_manifest_fingerprint) =
        runtime_expected_file_manifest_fingerprint(expected_file_manifest)
    else {
        return false;
    };

    if trust_manifest.expected_file_manifest_fingerprint_sha256.as_deref()
        != Some(expected_manifest_fingerprint.as_str())
    {
        return false;
    }

    is_valid_sha256_hex(&trust_manifest.authorization_manifest_fingerprint_sha256)
}

fn is_valid_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|character| character.is_ascii_hexdigit())
}

fn relative_path_to_path_buf(raw_path: &str) -> PathBuf {
    raw_path
        .split('/')
        .fold(PathBuf::new(), |path, segment| path.join(segment))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;
    use crate::runtime_package_contract::{
        persist_materialized_runtime_package, runtime_expected_file_manifest_fingerprint,
        runtime_support_paths_from_root, MaterializedRuntimePackage,
        RuntimeExpectedFileManifest, RuntimeExpectedFileRecord, RuntimeInstallRecord,
        RuntimeTrustManifest, RuntimeVersionRecord, RUNTIME_RECORD_SCHEMA_VERSION,
        RUNTIME_SUPPORT_DIRECTORY_NAME, RUNTIME_TRUST_VERIFICATION_BASIS_NAME,
    };

    #[test]
    fn runtime_integrity_surface_verifies_installed_runtime_files_and_platform() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let paths = runtime_support_paths_from_root(temp_dir.path().join(RUNTIME_SUPPORT_DIRECTORY_NAME));

        fs::create_dir_all(paths.install_dir.join("bin")).expect("bin dir should exist");
        fs::create_dir_all(paths.install_dir.join("lib")).expect("lib dir should exist");
        fs::write(paths.install_dir.join("bin/python.exe"), b"python-bytes")
            .expect("python bytes should be written");
        fs::write(paths.install_dir.join("lib/runtime.zip"), b"support-bytes")
            .expect("support bytes should be written");
        let install_record = RuntimeInstallRecord {
            schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
            executable_relative_path: "bin/python.exe".to_string(),
            installed_at_epoch_seconds: 100,
        };
        let expected_file_manifest = RuntimeExpectedFileManifest {
            schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
            runtime_version: "3.11.9".to_string(),
            files: vec![
                RuntimeExpectedFileRecord {
                    relative_path: "bin/python.exe".to_string(),
                    sha256: hex_sha256(b"python-bytes"),
                },
                RuntimeExpectedFileRecord {
                    relative_path: "lib/runtime.zip".to_string(),
                    sha256: hex_sha256(b"support-bytes"),
                },
            ],
        };
        let version_record = RuntimeVersionRecord {
            schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
            runtime_version: "3.11.9".to_string(),
            compatibility_target_os: std::env::consts::OS.to_string(),
            compatibility_target_arch: std::env::consts::ARCH.to_string(),
        };

        persist_materialized_runtime_package(
            &paths,
            &MaterializedRuntimePackage {
                install_record: install_record.clone(),
                expected_file_manifest: expected_file_manifest.clone(),
                version_record: version_record.clone(),
                trust_manifest: RuntimeTrustManifest {
                    schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                    verification_basis_name: RUNTIME_TRUST_VERIFICATION_BASIS_NAME.to_string(),
                    authorization_manifest_fingerprint_sha256: format!("{:064x}", 1),
                    verified_file_count: expected_file_manifest.files.len(),
                    runtime_version: Some(version_record.runtime_version.clone()),
                    executable_relative_path: Some(install_record.executable_relative_path.clone()),
                    expected_file_manifest_fingerprint_sha256: Some(
                        runtime_expected_file_manifest_fingerprint(&expected_file_manifest)
                            .expect("expected-file manifest should serialize"),
                    ),
                },
            },
        )
        .expect("runtime package should persist");

        let snapshot = runtime_integrity_state_from_paths(&paths);

        assert!(snapshot.integrity_basis_defined);
        assert!(snapshot.runtime_delivery_contract_defined);
        assert!(snapshot.pre_install_verification_started);
        assert!(snapshot.post_install_verification_started);
        assert!(snapshot.expected_files_contract_defined);
        assert!(snapshot.version_contract_defined);
        assert!(snapshot.checksum_or_signature_required);
        assert!(snapshot.compatibility_basis_defined);
        assert!(snapshot.trust_rejection_distinct_from_technical_failure);
        assert!(snapshot.runtime_use_blocked_until_verified);
        assert!(snapshot.expected_files_verified);
        assert!(snapshot.version_compatibility_verified);
        assert!(snapshot.checksum_or_signature_verified);
        assert!(snapshot.runtime_integrity_passed);
        assert!(snapshot.execution_unblocked);
    }

    #[test]
    fn runtime_integrity_rejects_stale_trust_metadata() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let paths = runtime_support_paths_from_root(temp_dir.path().join(RUNTIME_SUPPORT_DIRECTORY_NAME));

        fs::create_dir_all(paths.install_dir.join("bin")).expect("bin dir should exist");
        fs::create_dir_all(paths.install_dir.join("lib")).expect("lib dir should exist");
        fs::write(paths.install_dir.join("bin/python.exe"), b"python-bytes")
            .expect("python bytes should be written");
        fs::write(paths.install_dir.join("lib/runtime.zip"), b"support-bytes")
            .expect("support bytes should be written");

        let expected_file_manifest = RuntimeExpectedFileManifest {
            schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
            runtime_version: "3.11.9".to_string(),
            files: vec![
                RuntimeExpectedFileRecord {
                    relative_path: "bin/python.exe".to_string(),
                    sha256: hex_sha256(b"python-bytes"),
                },
                RuntimeExpectedFileRecord {
                    relative_path: "lib/runtime.zip".to_string(),
                    sha256: hex_sha256(b"support-bytes"),
                },
            ],
        };

        persist_materialized_runtime_package(
            &paths,
            &MaterializedRuntimePackage {
                install_record: RuntimeInstallRecord {
                    schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                    executable_relative_path: "bin/python.exe".to_string(),
                    installed_at_epoch_seconds: 100,
                },
                expected_file_manifest: expected_file_manifest.clone(),
                version_record: RuntimeVersionRecord {
                    schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                    runtime_version: "3.11.9".to_string(),
                    compatibility_target_os: std::env::consts::OS.to_string(),
                    compatibility_target_arch: std::env::consts::ARCH.to_string(),
                },
                trust_manifest: RuntimeTrustManifest {
                    schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                    verification_basis_name: RUNTIME_TRUST_VERIFICATION_BASIS_NAME.to_string(),
                    authorization_manifest_fingerprint_sha256: format!("{:064x}", 2),
                    verified_file_count: expected_file_manifest.files.len(),
                    runtime_version: Some("3.11.8".to_string()),
                    executable_relative_path: Some("bin/python.exe".to_string()),
                    expected_file_manifest_fingerprint_sha256: Some(
                        runtime_expected_file_manifest_fingerprint(&expected_file_manifest)
                            .expect("expected-file manifest should serialize"),
                    ),
                },
            },
        )
        .expect("runtime package should persist");

        let snapshot = runtime_integrity_state_from_paths(&paths);

        assert!(snapshot.expected_files_verified);
        assert!(snapshot.version_compatibility_verified);
        assert!(!snapshot.checksum_or_signature_verified);
        assert!(!snapshot.runtime_integrity_passed);
        assert!(!snapshot.execution_unblocked);
    }
}