use std::path::PathBuf;

use crate::runtime_package_contract::{
    default_runtime_support_paths, runtime_package_contract_basis,
    runtime_package_contract_basis_from_paths, runtime_package_materialization_state_from_paths,
    RuntimePackageContractSnapshot, RuntimeSupportPaths,
};

#[derive(Debug, Clone, Copy)]
pub enum RuntimePresenceStage {
    DetectionMaterialized,
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimePresenceSnapshot {
    pub stage: RuntimePresenceStage,
    pub runtime_package_contract: RuntimePackageContractSnapshot,
    pub detection_basis_defined: bool,
    pub runtime_delivery_contract_defined: bool,
    pub runtime_use_blocked_until_verified: bool,
    pub runtime_detected: Option<bool>,
    pub runtime_ready: bool,
    pub hydration_implemented: bool,
    pub integrity_implemented: bool,
}

impl RuntimePresenceSnapshot {
    pub fn summary(self) -> String {
        let detection = match self.runtime_detected {
            Some(true) => "shell-detected-present",
            Some(false) => "shell-detected-missing",
            None => "shell-detection-unavailable",
        };

        format!(
            "Runtime presence surface is materialized. Runtime presence stage is {:?}; detection basis defined is {}; runtime-delivery contract defined is {}; runtime use blocked until verified is {}; contract name is {}; install location class is {}; detection status is {detection}; local install record materialized is {}; local expected-file manifest materialized is {}; runtime readiness is {}; hydration implemented is {}; integrity implemented is {}.",
            self.stage,
            self.detection_basis_defined,
            self.runtime_delivery_contract_defined,
            self.runtime_use_blocked_until_verified,
            self.runtime_package_contract.contract_name,
            self.runtime_package_contract.install_location_class_name,
            self.runtime_package_contract.local_install_record_materialized,
            self.runtime_package_contract.local_expected_file_manifest_materialized,
            self.runtime_ready,
            self.hydration_implemented,
            self.integrity_implemented
        )
    }
}

pub fn runtime_presence_state() -> RuntimePresenceSnapshot {
    match default_runtime_support_paths() {
        Ok(paths) => runtime_presence_state_from_paths(&paths),
        Err(_) => RuntimePresenceSnapshot {
            stage: RuntimePresenceStage::DetectionMaterialized,
            runtime_package_contract: runtime_package_contract_basis(),
            detection_basis_defined: false,
            runtime_delivery_contract_defined: true,
            runtime_use_blocked_until_verified: true,
            runtime_detected: None,
            runtime_ready: false,
            hydration_implemented: true,
            integrity_implemented: true,
        },
    }
}

pub(crate) fn runtime_presence_state_from_paths(
    paths: &RuntimeSupportPaths,
) -> RuntimePresenceSnapshot {
    let runtime_package_contract = runtime_package_contract_basis_from_paths(paths);
    let materialization_state = runtime_package_materialization_state_from_paths(paths).ok();
    let runtime_detected = materialization_state
        .as_ref()
        .map(|state| runtime_detected_from_paths(paths, state))
        .unwrap_or(None);
    let runtime_ready = runtime_detected == Some(true)
        && runtime_package_contract.local_version_record_materialized
        && runtime_package_contract.local_trust_manifest_materialized;

    RuntimePresenceSnapshot {
        stage: RuntimePresenceStage::DetectionMaterialized,
        runtime_package_contract,
        detection_basis_defined: true,
        runtime_delivery_contract_defined: true,
        runtime_use_blocked_until_verified: runtime_package_contract
            .verification_required_before_runtime_use,
        runtime_detected,
        runtime_ready,
        hydration_implemented: true,
        integrity_implemented: true,
    }
}

fn runtime_detected_from_paths(
    paths: &RuntimeSupportPaths,
    state: &crate::runtime_package_contract::RuntimePackageMaterializationState,
) -> Option<bool> {
    let Some(install_record) = state.install_record.as_ref() else {
        return Some(false);
    };
    let Some(expected_file_manifest) = state.expected_file_manifest.as_ref() else {
        return Some(false);
    };

    let executable_exists = paths
        .install_dir
        .join(relative_path_to_path_buf(&install_record.executable_relative_path))
        .exists();
    let expected_files_exist = expected_file_manifest.files.iter().all(|file| {
        paths.install_dir
            .join(relative_path_to_path_buf(&file.relative_path))
            .exists()
    });

    Some(executable_exists && expected_files_exist)
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
    fn runtime_presence_surface_uses_local_installation_state() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let paths = runtime_support_paths_from_root(temp_dir.path().join(RUNTIME_SUPPORT_DIRECTORY_NAME));

        fs::create_dir_all(paths.install_dir.join("bin")).expect("install bin dir should exist");
        fs::create_dir_all(paths.install_dir.join("lib")).expect("install lib dir should exist");
        fs::write(paths.install_dir.join("bin/python.exe"), b"python-bytes")
            .expect("runtime executable should exist");
        fs::write(paths.install_dir.join("lib/runtime.zip"), b"support-bytes")
            .expect("runtime support file should exist");
        let expected_file_manifest = RuntimeExpectedFileManifest {
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
                    authorization_manifest_fingerprint_sha256: format!("{:064x}", 5),
                    verified_file_count: expected_file_manifest.files.len(),
                    runtime_version: Some("3.11.9".to_string()),
                    executable_relative_path: Some("bin/python.exe".to_string()),
                    expected_file_manifest_fingerprint_sha256: Some(
                        runtime_expected_file_manifest_fingerprint(&expected_file_manifest)
                            .expect("manifest fingerprint should serialize"),
                    ),
                },
            },
        )
        .expect("runtime package should persist");

        let snapshot = runtime_presence_state_from_paths(&paths);

        assert!(snapshot.detection_basis_defined);
        assert!(snapshot.runtime_delivery_contract_defined);
        assert!(snapshot.runtime_use_blocked_until_verified);
        assert_eq!(snapshot.runtime_detected, Some(true));
        assert_eq!(
            snapshot.runtime_package_contract.install_location_class_name,
            "AppControlledSupportLocation"
        );
        assert!(snapshot.runtime_ready);
        assert!(snapshot.hydration_implemented);
        assert!(snapshot.integrity_implemented);
    }
}
