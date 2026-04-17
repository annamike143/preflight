use std::collections::HashSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, io::Write};

use serde::Deserialize;

use crate::activation_support_store::ActivationSupportStore;
use crate::run_lifecycle_state::{run_lifecycle_state_from_shell, CanonicalRunLifecycleState};
use crate::runtime_integrity::runtime_integrity_state_from_paths;
use crate::runtime_package_contract::{
    clear_rejected_runtime_candidate, default_runtime_support_paths, hex_sha256,
    persist_materialized_runtime_package,
    persist_rejected_runtime_candidate,
    runtime_expected_file_manifest_fingerprint,
    runtime_package_materialization_state_from_paths,
    MaterializedRuntimePackage, RuntimeArtifactRejectionReason, RuntimeExpectedFileManifest,
    RuntimeExpectedFileRecord, RuntimeInstallRecord, RuntimeRejectedCandidateRecord,
    RuntimeSupportPaths, RuntimeTrustManifest, RuntimeVersionRecord,
    RUNTIME_RECORD_SCHEMA_VERSION, RUNTIME_TRUST_VERIFICATION_BASIS_NAME,
};
use crate::runtime_presence::runtime_presence_state_from_paths;

pub const STATE_RUNTIME_HYDRATION_REQUIRED: &str = "RuntimeHydrationRequired";
pub const STATE_RUNTIME_HYDRATING: &str = "RuntimeHydrating";
pub const STATE_ACTIVATION_REQUIRED: &str = "ActivationRequired";
pub const STATE_CONFIGURATION_REQUIRED: &str = "ConfigurationRequired";
pub const STATE_READY_IDLE: &str = "ReadyIdle";

const RUNTIME_DELIVERY_MANIFEST_ENV_VAR: &str = "MIRO_FISH_RUNTIME_DELIVERY_MANIFEST";
const COMMERCIAL_BACKEND_DATABASE_PATH_ENV_VAR: &str = "MIRO_FISH_COMMERCIAL_BACKEND_DB_PATH";
const COMMERCIAL_BACKEND_RUNTIME_DELIVERY_MANIFEST_PATH_ENV_VAR: &str =
    "MIRO_FISH_COMMERCIAL_BACKEND_RUNTIME_DELIVERY_MANIFEST_PATH";
const PYTHON_EXECUTABLE_ENV_VAR: &str = "MIRO_FISH_PYTHON_EXE";

#[derive(Debug, Clone, Deserialize)]
struct CommercialBackendRuntimeManifestDecision {
    decision_type: String,
    authorization: Option<CommercialBackendRuntimeManifestAuthorization>,
}

#[derive(Debug, Clone, Deserialize)]
struct CommercialBackendRuntimeManifestAuthorization {
    manifest_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidatedRuntimeExpectedFile {
    relative_path: String,
    relative_path_buf: PathBuf,
    sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidatedRuntimeManifest {
    runtime_version: String,
    source_root: PathBuf,
    executable_relative_path: String,
    compatibility_target_os: String,
    compatibility_target_arch: String,
    expected_files: Vec<ValidatedRuntimeExpectedFile>,
}

#[derive(Debug, Clone, Deserialize)]
struct AuthorizedRuntimeDeliveryManifest {
    schema_version: u32,
    runtime_version: String,
    source_root: String,
    executable_relative_path: String,
    compatibility_target_os: String,
    compatibility_target_arch: String,
    expected_files: Vec<AuthorizedRuntimeExpectedFile>,
}

#[derive(Debug, Clone, Deserialize)]
struct AuthorizedRuntimeExpectedFile {
    relative_path: String,
    sha256: String,
}

#[derive(Debug, Clone)]
struct ConfiguredRuntimeManifestInspection {
    manifest_configured: bool,
    artifact_source_configured: bool,
    manifest_ready: bool,
    runtime_version: Option<String>,
    manifest_fingerprint_sha256: Option<String>,
    validated_manifest: Option<ValidatedRuntimeManifest>,
    rejection_reason: Option<RuntimeArtifactRejectionReason>,
    failure_detail: Option<String>,
    technical_failure: bool,
    status_note: String,
}

#[derive(Debug, Clone, Default)]
struct RuntimeMetadataSnapshot {
    install_record_bytes: Option<Vec<u8>>,
    expected_file_manifest_bytes: Option<Vec<u8>>,
    version_record_bytes: Option<Vec<u8>>,
    trust_manifest_bytes: Option<Vec<u8>>,
    rejected_candidate_record_bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeHydrationStage {
    ExecutionMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeHydrationExecutionClassification {
    Completed,
    BlockedActiveRun,
    RejectedArtifact,
}

impl RuntimeHydrationExecutionClassification {
    fn label(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::BlockedActiveRun => "blocked_active_run",
            Self::RejectedArtifact => "rejected_artifact",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHydrationSnapshot {
    pub stage: RuntimeHydrationStage,
    pub required_state_name: &'static str,
    pub hydrating_state_name: &'static str,
    pub success_activation_state_name: &'static str,
    pub success_configuration_state_name: &'static str,
    pub success_ready_state_name: &'static str,
    pub retryable_failure_returns_to_required: bool,
    pub hydration_required_now: bool,
    pub hydration_active_now: bool,
    pub download_authorized: bool,
    pub install_started: bool,
    pub retryable_failure_present: bool,
    pub authorization_manifest_configured: bool,
    pub artifact_source_configured: bool,
    pub active_run_mutation_blocked_now: bool,
    pub installed_runtime_version: Option<String>,
    pub rejected_candidate_present: bool,
    pub rejected_candidate_version: Option<String>,
    pub status_note: String,
}

impl RuntimeHydrationSnapshot {
    pub fn summary(&self) -> String {
        format!(
            "Runtime hydration surface is materialized. Stage is {:?}; canonical states are {} and {}; success paths lead to {}, {}, or {}; retryable failure returns to required is {}; hydration required now is {}; hydration active now is {}; download authorized is {}; install started is {}; retryable failure present is {}; authorization manifest configured is {}; artifact source configured is {}; active-run mutation blocked now is {}; installed runtime version is {:?}; rejected candidate present is {}; rejected candidate version is {:?}; status note is {}.",
            self.stage,
            self.required_state_name,
            self.hydrating_state_name,
            self.success_activation_state_name,
            self.success_configuration_state_name,
            self.success_ready_state_name,
            self.retryable_failure_returns_to_required,
            self.hydration_required_now,
            self.hydration_active_now,
            self.download_authorized,
            self.install_started,
            self.retryable_failure_present,
            self.authorization_manifest_configured,
            self.artifact_source_configured,
            self.active_run_mutation_blocked_now,
            self.installed_runtime_version,
            self.rejected_candidate_present,
            self.rejected_candidate_version,
            self.status_note,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHydrationExecutionOutcome {
    pub classification: RuntimeHydrationExecutionClassification,
    pub detail_message: String,
    pub hydration_state: RuntimeHydrationSnapshot,
}

impl RuntimeHydrationExecutionOutcome {
    pub fn summary(&self) -> String {
        format!(
            "Managed runtime hydration outcome is {}. {} {}",
            self.classification.label(),
            self.detail_message,
            self.hydration_state.summary()
        )
    }
}

pub fn runtime_hydration_state() -> RuntimeHydrationSnapshot {
    let manifest_path = resolve_configured_runtime_manifest_path();
    let active_run_detected = active_run_detected_for_runtime_mutation();

    match default_runtime_support_paths() {
        Ok(paths) => {
            runtime_hydration_state_from_components(&paths, manifest_path.as_deref(), active_run_detected)
        }
        Err(error) => RuntimeHydrationSnapshot {
            stage: RuntimeHydrationStage::ExecutionMaterialized,
            required_state_name: STATE_RUNTIME_HYDRATION_REQUIRED,
            hydrating_state_name: STATE_RUNTIME_HYDRATING,
            success_activation_state_name: STATE_ACTIVATION_REQUIRED,
            success_configuration_state_name: STATE_CONFIGURATION_REQUIRED,
            success_ready_state_name: STATE_READY_IDLE,
            retryable_failure_returns_to_required: true,
            hydration_required_now: true,
            hydration_active_now: false,
            download_authorized: false,
            install_started: false,
            retryable_failure_present: false,
            authorization_manifest_configured: manifest_path.is_some(),
            artifact_source_configured: false,
            active_run_mutation_blocked_now: active_run_detected,
            installed_runtime_version: None,
            rejected_candidate_present: false,
            rejected_candidate_version: None,
            status_note: format!(
                "Shell-owned runtime hydration cannot resolve the app-controlled support directory yet: {error}"
            ),
        },
    }
}

pub fn hydrate_managed_runtime_from_shell() -> Result<RuntimeHydrationExecutionOutcome, String> {
    let support_paths = default_runtime_support_paths()?;
    let manifest_path = resolve_configured_runtime_manifest_path();

    hydrate_managed_runtime_with_components(
        &support_paths,
        manifest_path.as_deref(),
        active_run_detected_for_runtime_mutation(),
        current_epoch_seconds()?,
    )
}

fn runtime_hydration_state_from_components(
    support_paths: &RuntimeSupportPaths,
    configured_manifest_path: Option<&Path>,
    active_run_detected: bool,
) -> RuntimeHydrationSnapshot {
    let materialization_state = runtime_package_materialization_state_from_paths(support_paths)
        .ok()
        .unwrap_or_else(crate::runtime_package_contract::runtime_package_materialization_state);
    let manifest_inspection = inspect_configured_runtime_manifest(configured_manifest_path);
    let installed_runtime_version = materialization_state
        .version_record
        .as_ref()
        .map(|record| record.runtime_version.clone());
    let rejected_candidate_present = materialization_state.rejected_candidate.is_some();
    let rejected_candidate_version = materialization_state
        .rejected_candidate
        .as_ref()
        .and_then(|record| record.runtime_version.clone());
    let runtime_presence = runtime_presence_state_from_paths(support_paths);
    let runtime_integrity = runtime_integrity_state_from_paths(support_paths);
    let installed_runtime_is_usable = runtime_presence.runtime_detected == Some(true)
        && runtime_integrity.runtime_integrity_passed
        && runtime_integrity.execution_unblocked;
    let hydration_required_now = !installed_runtime_is_usable;

    let status_note = if installed_runtime_is_usable {
        let installed_runtime_version = installed_runtime_version.as_deref().unwrap_or("unknown");

        if let Some(rejection) = materialization_state.rejected_candidate.as_ref() {
            format!(
                "Managed runtime version {} remains installed under shell authority. The last replacement candidate was rejected as {}: {}.",
                installed_runtime_version,
                rejection.rejection_reason.label(),
                rejection.rejection_detail,
            )
        } else {
            format!(
                "Managed runtime version {} is installed under shell authority inside the app-controlled support location.",
                installed_runtime_version,
            )
        }
    } else if let Some(installed_runtime_version) = installed_runtime_version.as_deref() {
        if runtime_presence.runtime_detected == Some(true) {
            format!(
                "Managed runtime version {} is still present on disk, but shell-owned integrity verification is no longer accepted. Runtime hydration stays required until the installed package is repaired or replaced.",
                installed_runtime_version,
            )
        } else {
            format!(
                "Managed runtime installation metadata still references version {}, but the on-disk runtime is missing or incomplete. Runtime hydration stays required until a valid package is restored.",
                installed_runtime_version,
            )
        }
    } else if let Some(rejection) = materialization_state.rejected_candidate.as_ref() {
        format!(
            "The last managed runtime candidate was rejected as {}: {}. Runtime hydration stays in the required state until a corrected candidate is provided.",
            rejection.rejection_reason.label(),
            rejection.rejection_detail,
        )
    } else if active_run_detected && manifest_inspection.manifest_ready {
        "A managed runtime candidate is configured, but shell-owned runtime mutation is blocked while a run is active."
            .to_string()
    } else if manifest_inspection.manifest_ready {
        "Managed runtime hydration is required and the shell can now acquire the configured candidate on explicit command."
            .to_string()
    } else {
        manifest_inspection.status_note.clone()
    };

    RuntimeHydrationSnapshot {
        stage: RuntimeHydrationStage::ExecutionMaterialized,
        required_state_name: STATE_RUNTIME_HYDRATION_REQUIRED,
        hydrating_state_name: STATE_RUNTIME_HYDRATING,
        success_activation_state_name: STATE_ACTIVATION_REQUIRED,
        success_configuration_state_name: STATE_CONFIGURATION_REQUIRED,
        success_ready_state_name: STATE_READY_IDLE,
        retryable_failure_returns_to_required: true,
        hydration_required_now,
        hydration_active_now: false,
        download_authorized: manifest_inspection.manifest_ready
            && !active_run_detected
            && hydration_required_now,
        install_started: false,
        retryable_failure_present: rejected_candidate_present,
        authorization_manifest_configured: manifest_inspection.manifest_configured,
        artifact_source_configured: manifest_inspection.artifact_source_configured,
        active_run_mutation_blocked_now: active_run_detected && hydration_required_now,
        installed_runtime_version,
        rejected_candidate_present,
        rejected_candidate_version,
        status_note,
    }
}

fn hydrate_managed_runtime_with_components(
    support_paths: &RuntimeSupportPaths,
    configured_manifest_path: Option<&Path>,
    active_run_detected: bool,
    current_timestamp_epoch_seconds: u64,
) -> Result<RuntimeHydrationExecutionOutcome, String> {
    hydrate_managed_runtime_with_components_and_hooks(
        support_paths,
        configured_manifest_path,
        active_run_detected,
        current_timestamp_epoch_seconds,
        persist_materialized_runtime_package,
        clear_rejected_runtime_candidate,
    )
}

fn hydrate_managed_runtime_with_components_and_hooks<P, C>(
    support_paths: &RuntimeSupportPaths,
    configured_manifest_path: Option<&Path>,
    active_run_detected: bool,
    current_timestamp_epoch_seconds: u64,
    persist_runtime_package: P,
    clear_rejected_candidate: C,
) -> Result<RuntimeHydrationExecutionOutcome, String>
where
    P: Fn(&RuntimeSupportPaths, &MaterializedRuntimePackage) -> Result<(), String>,
    C: Fn(&RuntimeSupportPaths) -> Result<(), String>,
{
    if active_run_detected {
        return Ok(RuntimeHydrationExecutionOutcome {
            classification: RuntimeHydrationExecutionClassification::BlockedActiveRun,
            detail_message:
                "Managed runtime hydration is blocked because runtime mutation remains forbidden during an active run."
                    .to_string(),
            hydration_state: runtime_hydration_state_from_components(
                support_paths,
                configured_manifest_path,
                active_run_detected,
            ),
        });
    }

    let manifest_inspection = inspect_configured_runtime_manifest(configured_manifest_path);

    if !manifest_inspection.manifest_configured {
        return Err(format!(
            "Set {} to an authorized runtime manifest path before shell-owned runtime hydration can begin.",
            RUNTIME_DELIVERY_MANIFEST_ENV_VAR,
        ));
    }

    if manifest_inspection.technical_failure {
        return Err(manifest_inspection.failure_detail.unwrap_or_else(|| {
            "managed runtime authorization manifest could not be loaded".to_string()
        }));
    }

    let validated_manifest = match manifest_inspection.validated_manifest {
        Some(validated_manifest) => validated_manifest,
        None => {
            let rejection_reason = manifest_inspection
                .rejection_reason
                .unwrap_or(RuntimeArtifactRejectionReason::ManifestInvalid);
            let rejection_detail = manifest_inspection.failure_detail.unwrap_or_else(|| {
                "managed runtime authorization manifest is not acceptable yet".to_string()
            });

            persist_rejection(
                support_paths,
                manifest_inspection.runtime_version,
                manifest_inspection.manifest_fingerprint_sha256,
                rejection_reason,
                rejection_detail.clone(),
                current_timestamp_epoch_seconds,
            )?;

            return Ok(RuntimeHydrationExecutionOutcome {
                classification: RuntimeHydrationExecutionClassification::RejectedArtifact,
                detail_message: rejection_detail,
                hydration_state: runtime_hydration_state_from_components(
                    support_paths,
                    configured_manifest_path,
                    false,
                ),
            });
        }
    };

    let expected_file_records = match verify_expected_source_files(&validated_manifest) {
        Ok(expected_file_records) => expected_file_records,
        Err((rejection_reason, rejection_detail)) => {
            persist_rejection(
                support_paths,
                Some(validated_manifest.runtime_version.clone()),
                manifest_inspection.manifest_fingerprint_sha256,
                rejection_reason,
                rejection_detail.clone(),
                current_timestamp_epoch_seconds,
            )?;

            return Ok(RuntimeHydrationExecutionOutcome {
                classification: RuntimeHydrationExecutionClassification::RejectedArtifact,
                detail_message: rejection_detail,
                hydration_state: runtime_hydration_state_from_components(
                    support_paths,
                    configured_manifest_path,
                    false,
                ),
            });
        }
    };

    let previous_install_present = support_paths.install_dir.exists();
    let previous_metadata_snapshot = snapshot_runtime_metadata(support_paths)?;

    fs::create_dir_all(&support_paths.root_dir).map_err(|error| error.to_string())?;
    clear_install_workdirs(support_paths)?;
    copy_expected_files_to_staging(support_paths, &validated_manifest)?;

    let install_backup_present = if previous_install_present {
        fs::rename(&support_paths.install_dir, &support_paths.backup_install_dir)
            .map_err(|error| error.to_string())?;
        true
    } else {
        false
    };

    if let Err(error) = fs::rename(&support_paths.staging_install_dir, &support_paths.install_dir) {
        if install_backup_present {
            let _ = fs::rename(&support_paths.backup_install_dir, &support_paths.install_dir);
        }

        return Err(format!(
            "managed runtime candidate could not be moved into the app-controlled support location: {error}"
        ));
    }

    let install_record = RuntimeInstallRecord {
        schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
        executable_relative_path: validated_manifest.executable_relative_path.clone(),
        installed_at_epoch_seconds: current_timestamp_epoch_seconds,
    };
    let expected_file_manifest = RuntimeExpectedFileManifest {
        schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
        runtime_version: validated_manifest.runtime_version.clone(),
        files: expected_file_records,
    };
    let version_record = RuntimeVersionRecord {
        schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
        runtime_version: validated_manifest.runtime_version.clone(),
        compatibility_target_os: validated_manifest.compatibility_target_os.clone(),
        compatibility_target_arch: validated_manifest.compatibility_target_arch.clone(),
    };
    let trust_manifest = RuntimeTrustManifest {
        schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
        verification_basis_name: RUNTIME_TRUST_VERIFICATION_BASIS_NAME.to_string(),
        authorization_manifest_fingerprint_sha256: manifest_inspection
            .manifest_fingerprint_sha256
            .unwrap_or_else(|| "unavailable".to_string()),
        verified_file_count: expected_file_manifest.files.len(),
        runtime_version: Some(version_record.runtime_version.clone()),
        executable_relative_path: Some(install_record.executable_relative_path.clone()),
        expected_file_manifest_fingerprint_sha256: Some(
            runtime_expected_file_manifest_fingerprint(&expected_file_manifest)?,
        ),
    };

    let materialized_runtime_package = MaterializedRuntimePackage {
        install_record,
        expected_file_manifest,
        version_record,
        trust_manifest,
    };

    if let Err(error) = persist_runtime_package(support_paths, &materialized_runtime_package) {
        return rollback_runtime_install_after_failed_commit(
            support_paths,
            previous_install_present,
            &previous_metadata_snapshot,
            format!(
                "managed runtime installation metadata could not be persisted after file copy: {error}"
            ),
        );
    }

    if let Err(error) = clear_rejected_candidate(support_paths) {
        return rollback_runtime_install_after_failed_commit(
            support_paths,
            previous_install_present,
            &previous_metadata_snapshot,
            format!(
                "managed runtime installation metadata could not be finalized after file copy: {error}"
            ),
        );
    }

    if install_backup_present && support_paths.backup_install_dir.exists() {
        fs::remove_dir_all(&support_paths.backup_install_dir).map_err(|error| error.to_string())?;
    }

    Ok(RuntimeHydrationExecutionOutcome {
        classification: RuntimeHydrationExecutionClassification::Completed,
        detail_message: format!(
            "Managed runtime version {} was verified and installed under shell authority.",
            validated_manifest.runtime_version
        ),
        hydration_state: runtime_hydration_state_from_components(
            support_paths,
            configured_manifest_path,
            false,
        ),
    })
}

fn inspect_configured_runtime_manifest(
    configured_manifest_path: Option<&Path>,
) -> ConfiguredRuntimeManifestInspection {
    let Some(configured_manifest_path) = configured_manifest_path else {
        return ConfiguredRuntimeManifestInspection {
            manifest_configured: false,
            artifact_source_configured: false,
            manifest_ready: false,
            runtime_version: None,
            manifest_fingerprint_sha256: None,
            validated_manifest: None,
            rejection_reason: None,
            failure_detail: None,
            technical_failure: false,
            status_note: format!(
                "Set {} to an authorized runtime manifest path before shell-owned runtime hydration can begin.",
                RUNTIME_DELIVERY_MANIFEST_ENV_VAR,
            ),
        };
    };

    let payload = match fs::read(configured_manifest_path) {
        Ok(payload) => payload,
        Err(error) => {
            return ConfiguredRuntimeManifestInspection {
                manifest_configured: true,
                artifact_source_configured: false,
                manifest_ready: false,
                runtime_version: None,
                manifest_fingerprint_sha256: None,
                validated_manifest: None,
                rejection_reason: None,
                failure_detail: Some(format!(
                    "managed runtime authorization manifest could not be read from {}: {error}",
                    configured_manifest_path.display(),
                )),
                technical_failure: true,
                status_note: format!(
                    "Managed runtime authorization manifest could not be read from {}.",
                    configured_manifest_path.display(),
                ),
            }
        }
    };

    let manifest_fingerprint_sha256 = hex_sha256(&payload);
    let manifest: AuthorizedRuntimeDeliveryManifest = match serde_json::from_slice(&payload) {
        Ok(manifest) => manifest,
        Err(error) => {
            return ConfiguredRuntimeManifestInspection {
                manifest_configured: true,
                artifact_source_configured: false,
                manifest_ready: false,
                runtime_version: None,
                manifest_fingerprint_sha256: Some(manifest_fingerprint_sha256),
                validated_manifest: None,
                rejection_reason: Some(RuntimeArtifactRejectionReason::ManifestInvalid),
                failure_detail: Some(format!(
                    "managed runtime authorization manifest is not valid JSON: {error}"
                )),
                technical_failure: false,
                status_note:
                    "Managed runtime authorization manifest is present but malformed and cannot authorize hydration."
                        .to_string(),
            }
        }
    };

    let runtime_version = Some(manifest.runtime_version.clone());
    let validated_manifest = match validate_authorized_runtime_manifest(
        &manifest,
        configured_manifest_path.parent().unwrap_or_else(|| Path::new(".")),
    ) {
        Ok(validated_manifest) => validated_manifest,
        Err(error) => {
            return ConfiguredRuntimeManifestInspection {
                manifest_configured: true,
                artifact_source_configured: false,
                manifest_ready: false,
                runtime_version,
                manifest_fingerprint_sha256: Some(manifest_fingerprint_sha256),
                validated_manifest: None,
                rejection_reason: Some(RuntimeArtifactRejectionReason::ManifestInvalid),
                failure_detail: Some(error.clone()),
                technical_failure: false,
                status_note: format!(
                    "Managed runtime authorization manifest is present but invalid: {}",
                    error,
                ),
            }
        }
    };

    if !validated_manifest.source_root.exists() {
        return ConfiguredRuntimeManifestInspection {
            manifest_configured: true,
            artifact_source_configured: false,
            manifest_ready: false,
            runtime_version,
            manifest_fingerprint_sha256: Some(manifest_fingerprint_sha256),
            validated_manifest: None,
            rejection_reason: Some(RuntimeArtifactRejectionReason::MissingArtifact),
            failure_detail: Some(format!(
                "managed runtime source root {} does not exist",
                validated_manifest.source_root.display(),
            )),
            technical_failure: false,
            status_note: format!(
                "Managed runtime authorization manifest is present, but the declared source root {} does not exist.",
                validated_manifest.source_root.display(),
            ),
        };
    }

    if validated_manifest.compatibility_target_os != std::env::consts::OS
        || validated_manifest.compatibility_target_arch != std::env::consts::ARCH
    {
        return ConfiguredRuntimeManifestInspection {
            manifest_configured: true,
            artifact_source_configured: true,
            manifest_ready: false,
            runtime_version,
            manifest_fingerprint_sha256: Some(manifest_fingerprint_sha256),
            validated_manifest: None,
            rejection_reason: Some(RuntimeArtifactRejectionReason::CompatibilityMismatch),
            failure_detail: Some(format!(
                "managed runtime candidate targets {}/{} instead of {}/{}",
                validated_manifest.compatibility_target_os,
                validated_manifest.compatibility_target_arch,
                std::env::consts::OS,
                std::env::consts::ARCH,
            )),
            technical_failure: false,
            status_note: format!(
                "Managed runtime authorization manifest is present, but the candidate targets {}/{} instead of the current {}/{} platform.",
                validated_manifest.compatibility_target_os,
                validated_manifest.compatibility_target_arch,
                std::env::consts::OS,
                std::env::consts::ARCH,
            ),
        };
    }

    ConfiguredRuntimeManifestInspection {
        manifest_configured: true,
        artifact_source_configured: true,
        manifest_ready: true,
        runtime_version,
        manifest_fingerprint_sha256: Some(manifest_fingerprint_sha256),
        validated_manifest: Some(validated_manifest),
        rejection_reason: None,
        failure_detail: None,
        technical_failure: false,
        status_note:
            "Managed runtime authorization manifest is configured and ready for shell-owned verification and acquisition."
                .to_string(),
    }
}

fn validate_authorized_runtime_manifest(
    manifest: &AuthorizedRuntimeDeliveryManifest,
    manifest_parent: &Path,
) -> Result<ValidatedRuntimeManifest, String> {
    if manifest.schema_version != RUNTIME_RECORD_SCHEMA_VERSION {
        return Err(format!(
            "managed runtime authorization manifest schema version {} is not supported",
            manifest.schema_version,
        ));
    }

    let runtime_version = manifest.runtime_version.trim();
    if runtime_version.is_empty() {
        return Err("managed runtime authorization manifest is missing runtime_version".to_string());
    }

    if manifest.expected_files.len() < 2 {
        return Err(
            "managed runtime authorization manifest must include the runtime executable and at least one support file"
                .to_string(),
        );
    }

    let executable_relative_path = normalize_relative_path(
        &manifest.executable_relative_path,
        "managed runtime executable_relative_path",
    )?;
    let executable_relative_path_string = path_buf_to_forward_slash_string(&executable_relative_path);
    let source_root = if Path::new(manifest.source_root.trim()).is_absolute() {
        PathBuf::from(manifest.source_root.trim())
    } else {
        manifest_parent.join(manifest.source_root.trim())
    };

    let mut seen_paths = HashSet::new();
    let mut expected_files = Vec::new();

    for file in &manifest.expected_files {
        let relative_path = normalize_relative_path(
            &file.relative_path,
            "managed runtime expected file relative_path",
        )?;
        let relative_path_string = path_buf_to_forward_slash_string(&relative_path);

        if !seen_paths.insert(relative_path_string.clone()) {
            return Err(format!(
                "managed runtime authorization manifest contains duplicate expected file path {}",
                relative_path_string,
            ));
        }

        let normalized_sha256 = file.sha256.trim().to_ascii_lowercase();
        if normalized_sha256.len() != 64
            || !normalized_sha256.chars().all(|character| character.is_ascii_hexdigit())
        {
            return Err(format!(
                "managed runtime authorization manifest contains a non-sha256 digest for {}",
                relative_path_string,
            ));
        }

        expected_files.push(ValidatedRuntimeExpectedFile {
            relative_path: relative_path_string,
            relative_path_buf: relative_path,
            sha256: normalized_sha256,
        });
    }

    if !expected_files
        .iter()
        .any(|file| file.relative_path == executable_relative_path_string)
    {
        return Err(
            "managed runtime authorization manifest does not include the declared executable inside expected_files"
                .to_string(),
        );
    }

    if manifest.compatibility_target_os.trim().is_empty()
        || manifest.compatibility_target_arch.trim().is_empty()
    {
        return Err(
            "managed runtime authorization manifest must declare compatibility_target_os and compatibility_target_arch"
                .to_string(),
        );
    }

    Ok(ValidatedRuntimeManifest {
        runtime_version: runtime_version.to_string(),
        source_root,
        executable_relative_path: executable_relative_path_string,
        compatibility_target_os: manifest.compatibility_target_os.trim().to_string(),
        compatibility_target_arch: manifest.compatibility_target_arch.trim().to_string(),
        expected_files,
    })
}

fn verify_expected_source_files(
    manifest: &ValidatedRuntimeManifest,
) -> Result<Vec<RuntimeExpectedFileRecord>, (RuntimeArtifactRejectionReason, String)> {
    let mut records = Vec::new();

    for expected_file in &manifest.expected_files {
        let source_path = manifest.source_root.join(&expected_file.relative_path_buf);
        let bytes = fs::read(&source_path).map_err(|error| {
            (
                RuntimeArtifactRejectionReason::MissingArtifact,
                format!(
                    "managed runtime expected file {} could not be read from {}: {error}",
                    expected_file.relative_path,
                    source_path.display(),
                ),
            )
        })?;
        let fingerprint = hex_sha256(&bytes);

        if fingerprint != expected_file.sha256 {
            return Err((
                RuntimeArtifactRejectionReason::ChecksumMismatch,
                format!(
                    "managed runtime expected file {} fingerprint {} did not match declared {}",
                    expected_file.relative_path,
                    fingerprint,
                    expected_file.sha256,
                ),
            ));
        }

        records.push(RuntimeExpectedFileRecord {
            relative_path: expected_file.relative_path.clone(),
            sha256: expected_file.sha256.clone(),
        });
    }

    Ok(records)
}

fn copy_expected_files_to_staging(
    support_paths: &RuntimeSupportPaths,
    manifest: &ValidatedRuntimeManifest,
) -> Result<(), String> {
    fs::create_dir_all(&support_paths.staging_install_dir).map_err(|error| error.to_string())?;

    for expected_file in &manifest.expected_files {
        let source_path = manifest.source_root.join(&expected_file.relative_path_buf);
        let destination_path = support_paths.install_dir.join(&expected_file.relative_path_buf);
        let staging_destination_path = support_paths
            .staging_install_dir
            .join(&expected_file.relative_path_buf);

        if let Some(parent_dir) = staging_destination_path.parent() {
            fs::create_dir_all(parent_dir).map_err(|error| error.to_string())?;
        }

        fs::copy(&source_path, &staging_destination_path).map_err(|error| {
            format!(
                "managed runtime expected file {} could not be copied into {}: {error}",
                expected_file.relative_path,
                destination_path.display(),
            )
        })?;
    }

    Ok(())
}

fn clear_install_workdirs(support_paths: &RuntimeSupportPaths) -> Result<(), String> {
    for path in [
        &support_paths.staging_install_dir,
        &support_paths.backup_install_dir,
    ] {
        if path.exists() {
            fs::remove_dir_all(path).map_err(|error| error.to_string())?;
        }
    }

    Ok(())
}

fn snapshot_runtime_metadata(paths: &RuntimeSupportPaths) -> Result<RuntimeMetadataSnapshot, String> {
    Ok(RuntimeMetadataSnapshot {
        install_record_bytes: read_optional_file_bytes(&paths.install_record_path)?,
        expected_file_manifest_bytes: read_optional_file_bytes(&paths.expected_file_manifest_path)?,
        version_record_bytes: read_optional_file_bytes(&paths.version_record_path)?,
        trust_manifest_bytes: read_optional_file_bytes(&paths.trust_manifest_path)?,
        rejected_candidate_record_bytes: read_optional_file_bytes(
            &paths.rejected_candidate_record_path,
        )?,
    })
}

fn read_optional_file_bytes(path: &Path) -> Result<Option<Vec<u8>>, String> {
    if !path.exists() {
        return Ok(None);
    }

    fs::read(path).map(Some).map_err(|error| error.to_string())
}

fn rollback_runtime_install_after_failed_commit(
    support_paths: &RuntimeSupportPaths,
    previous_install_present: bool,
    previous_metadata_snapshot: &RuntimeMetadataSnapshot,
    failure_message: String,
) -> Result<RuntimeHydrationExecutionOutcome, String> {
    match restore_previous_runtime_state(
        support_paths,
        previous_install_present,
        previous_metadata_snapshot,
    ) {
        Ok(()) => Err(format!(
            "{failure_message}. The previous managed runtime installation was restored."
        )),
        Err(restore_error) => Err(format!(
            "{failure_message}. The previous managed runtime installation could not be fully restored: {restore_error}"
        )),
    }
}

fn restore_previous_runtime_state(
    support_paths: &RuntimeSupportPaths,
    previous_install_present: bool,
    previous_metadata_snapshot: &RuntimeMetadataSnapshot,
) -> Result<(), String> {
    if support_paths.staging_install_dir.exists() {
        fs::remove_dir_all(&support_paths.staging_install_dir).map_err(|error| error.to_string())?;
    }

    if support_paths.install_dir.exists() {
        fs::remove_dir_all(&support_paths.install_dir).map_err(|error| error.to_string())?;
    }

    if support_paths.backup_install_dir.exists() {
        if previous_install_present {
            fs::rename(&support_paths.backup_install_dir, &support_paths.install_dir)
                .map_err(|error| error.to_string())?;
        } else {
            fs::remove_dir_all(&support_paths.backup_install_dir)
                .map_err(|error| error.to_string())?;
        }
    }

    restore_runtime_metadata_snapshot(support_paths, previous_metadata_snapshot)
}

fn restore_runtime_metadata_snapshot(
    paths: &RuntimeSupportPaths,
    snapshot: &RuntimeMetadataSnapshot,
) -> Result<(), String> {
    restore_optional_path(&paths.install_record_path, snapshot.install_record_bytes.as_deref())?;
    restore_optional_path(
        &paths.expected_file_manifest_path,
        snapshot.expected_file_manifest_bytes.as_deref(),
    )?;
    restore_optional_path(&paths.version_record_path, snapshot.version_record_bytes.as_deref())?;
    restore_optional_path(&paths.trust_manifest_path, snapshot.trust_manifest_bytes.as_deref())?;
    restore_optional_path(
        &paths.rejected_candidate_record_path,
        snapshot.rejected_candidate_record_bytes.as_deref(),
    )
}

fn restore_optional_path(path: &Path, bytes: Option<&[u8]>) -> Result<(), String> {
    remove_path_if_exists(path)?;

    if let Some(bytes) = bytes {
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir).map_err(|error| error.to_string())?;
        }

        fs::write(path, bytes).map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn remove_path_if_exists(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }

    if path.is_dir() {
        fs::remove_dir_all(path).map_err(|error| error.to_string())
    } else {
        fs::remove_file(path).map_err(|error| error.to_string())
    }
}

fn persist_rejection(
    support_paths: &RuntimeSupportPaths,
    runtime_version: Option<String>,
    manifest_fingerprint_sha256: Option<String>,
    rejection_reason: RuntimeArtifactRejectionReason,
    rejection_detail: String,
    rejected_at_epoch_seconds: u64,
) -> Result<(), String> {
    persist_rejected_runtime_candidate(
        support_paths,
        &RuntimeRejectedCandidateRecord {
            schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
            runtime_version,
            rejection_reason,
            rejection_detail,
            authorization_manifest_fingerprint_sha256: manifest_fingerprint_sha256,
            rejected_at_epoch_seconds,
        },
    )
}

fn normalize_relative_path(raw_path: &str, field_name: &str) -> Result<PathBuf, String> {
    let raw_path = raw_path.trim();

    if raw_path.is_empty() {
        return Err(format!("{} must not be empty", field_name));
    }

    let candidate = PathBuf::from(raw_path);
    if candidate.is_absolute() {
        return Err(format!("{} must stay relative to the managed runtime root", field_name));
    }

    if candidate.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::Prefix(_) | Component::RootDir
        )
    }) {
        return Err(format!(
            "{} must not escape the managed runtime root",
            field_name,
        ));
    }

    Ok(candidate)
}

fn path_buf_to_forward_slash_string(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(segment) => Some(segment.to_string_lossy().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn resolve_configured_runtime_manifest_path() -> Option<PathBuf> {
    env::var(RUNTIME_DELIVERY_MANIFEST_ENV_VAR)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or_else(resolve_runtime_manifest_path_from_commercial_backend)
}

fn resolve_runtime_manifest_path_from_commercial_backend() -> Option<PathBuf> {
    let support_store = ActivationSupportStore::default_store().ok()?;
    let support_record = support_store.load_record().ok().flatten()?;
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent()?.to_path_buf();
    let python_executable = default_repo_python_executable(&repo_root);
    let database_path = env::var_os(COMMERCIAL_BACKEND_DATABASE_PATH_ENV_VAR)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            repo_root
                .join("commercial_backend")
                .join(".state")
                .join("commercial_ledger.sqlite3")
        });
    let requested_timestamp_epoch_seconds = current_epoch_seconds().ok()?;
    let payload = serde_json::json!({
        "license_id": support_record.license_reference,
        "requested_timestamp_epoch_seconds": requested_timestamp_epoch_seconds,
    });
    let mut command = Command::new(python_executable);
    command
        .arg("-m")
        .arg("commercial_backend")
        .arg("resolve-runtime-delivery-manifest")
        .arg("--database-path")
        .arg(database_path)
        .current_dir(repo_root.join("commercial_backend"))
        .env(
            "PYTHONPATH",
            repo_root.join("commercial_backend").join("src"),
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(manifest_path) = env::var_os(COMMERCIAL_BACKEND_RUNTIME_DELIVERY_MANIFEST_PATH_ENV_VAR)
    {
        command
            .arg("--runtime-delivery-manifest-path")
            .arg(manifest_path);
    }

    let mut child = command.spawn().ok()?;
    let payload_bytes = serde_json::to_vec(&payload).ok()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(&payload_bytes).ok()?;
    }

    let output = child.wait_with_output().ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let decision: CommercialBackendRuntimeManifestDecision = serde_json::from_str(stdout.trim()).ok()?;

    if decision.decision_type != "authorized" {
        return None;
    }

    decision
        .authorization
        .map(|authorization| PathBuf::from(authorization.manifest_path))
}

fn default_repo_python_executable(repo_root: &Path) -> PathBuf {
    env::var_os(PYTHON_EXECUTABLE_ENV_VAR)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let repo_local_windows_python = repo_root.join(".venv").join("Scripts").join("python.exe");
            if repo_local_windows_python.exists() {
                return repo_local_windows_python;
            }

            let repo_local_unix_python = repo_root.join(".venv").join("bin").join("python");
            if repo_local_unix_python.exists() {
                return repo_local_unix_python;
            }

            PathBuf::from("python")
        })
}

pub(crate) fn active_run_detected_for_runtime_mutation() -> bool {
    let run_lifecycle_state = run_lifecycle_state_from_shell();

    !run_lifecycle_state.current_state_terminal
        && run_lifecycle_state.current_state != CanonicalRunLifecycleState::RunDraft
}

fn current_epoch_seconds() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())
        .map(|duration| duration.as_secs())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::runtime_package_contract::runtime_package_contract_basis_from_paths;

    #[test]
    fn runtime_hydration_state_requires_manifest_when_no_runtime_is_installed() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_paths = crate::runtime_package_contract::runtime_support_paths_from_root(
            temp_dir
                .path()
                .join(crate::runtime_package_contract::RUNTIME_SUPPORT_DIRECTORY_NAME),
        );

        let snapshot = runtime_hydration_state_from_components(&support_paths, None, false);

        assert_eq!(snapshot.required_state_name, STATE_RUNTIME_HYDRATION_REQUIRED);
        assert!(snapshot.hydration_required_now);
        assert!(!snapshot.download_authorized);
        assert!(!snapshot.retryable_failure_present);
        assert!(!snapshot.authorization_manifest_configured);
    }

    #[test]
    fn hydrate_managed_runtime_blocks_mutation_during_active_run() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_paths = crate::runtime_package_contract::runtime_support_paths_from_root(
            temp_dir
                .path()
                .join(crate::runtime_package_contract::RUNTIME_SUPPORT_DIRECTORY_NAME),
        );
        let manifest_path = write_manifest_fixture(temp_dir.path(), std::env::consts::OS, std::env::consts::ARCH, "python.exe", b"python-bytes", "runtime.zip", b"support-bytes");

        let outcome = hydrate_managed_runtime_with_components(
            &support_paths,
            Some(&manifest_path),
            true,
            100,
        )
        .expect("active-run blocking should not surface as technical failure");

        assert_eq!(
            outcome.classification,
            RuntimeHydrationExecutionClassification::BlockedActiveRun
        );
        assert!(outcome
            .detail_message
            .contains("runtime mutation remains forbidden during an active run"));
        assert!(!support_paths.install_dir.exists());
    }

    #[test]
    fn hydrate_managed_runtime_rejects_incompatible_candidate() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_paths = crate::runtime_package_contract::runtime_support_paths_from_root(
            temp_dir
                .path()
                .join(crate::runtime_package_contract::RUNTIME_SUPPORT_DIRECTORY_NAME),
        );
        let manifest_path = write_manifest_fixture(
            temp_dir.path(),
            "linux",
            std::env::consts::ARCH,
            "python.exe",
            b"python-bytes",
            "runtime.zip",
            b"support-bytes",
        );

        let outcome = hydrate_managed_runtime_with_components(
            &support_paths,
            Some(&manifest_path),
            false,
            100,
        )
        .expect("artifact rejection should not surface as technical failure");

        assert_eq!(
            outcome.classification,
            RuntimeHydrationExecutionClassification::RejectedArtifact
        );
        assert!(outcome.detail_message.contains("targets linux"));

        let materialization_state = runtime_package_materialization_state_from_paths(&support_paths)
            .expect("runtime package materialization state should load");
        assert!(materialization_state.rejected_candidate.is_some());
        assert!(materialization_state.install_record.is_none());
    }

    #[test]
    fn hydrate_managed_runtime_verifies_and_installs_candidate() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_paths = crate::runtime_package_contract::runtime_support_paths_from_root(
            temp_dir
                .path()
                .join(crate::runtime_package_contract::RUNTIME_SUPPORT_DIRECTORY_NAME),
        );
        let manifest_path = write_manifest_fixture(
            temp_dir.path(),
            std::env::consts::OS,
            std::env::consts::ARCH,
            "bin/python.exe",
            b"python-bytes",
            "lib/runtime.zip",
            b"support-bytes",
        );

        let outcome = hydrate_managed_runtime_with_components(
            &support_paths,
            Some(&manifest_path),
            false,
            100,
        )
        .expect("hydration should succeed");

        assert_eq!(
            outcome.classification,
            RuntimeHydrationExecutionClassification::Completed
        );
        assert!(support_paths.install_dir.exists());
        assert!(support_paths.install_dir.join("bin/python.exe").exists());
        assert!(support_paths.install_dir.join("lib/runtime.zip").exists());

        let package_contract = runtime_package_contract_basis_from_paths(&support_paths);
        assert!(package_contract.local_install_record_materialized);
        assert!(package_contract.local_expected_file_manifest_materialized);
        assert!(package_contract.local_version_record_materialized);
        assert!(package_contract.local_trust_manifest_materialized);
        assert!(!support_paths.rejected_candidate_record_path.exists());
    }

    #[test]
    fn hydrate_managed_runtime_restores_previous_runtime_when_metadata_commit_fails() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_paths = crate::runtime_package_contract::runtime_support_paths_from_root(
            temp_dir
                .path()
                .join(crate::runtime_package_contract::RUNTIME_SUPPORT_DIRECTORY_NAME),
        );

        fs::create_dir_all(support_paths.install_dir.join("bin"))
            .expect("previous runtime bin dir should exist");
        fs::create_dir_all(support_paths.install_dir.join("lib"))
            .expect("previous runtime lib dir should exist");
        fs::write(support_paths.install_dir.join("bin/python.exe"), b"old-python")
            .expect("previous runtime executable should be written");
        fs::write(support_paths.install_dir.join("lib/runtime.zip"), b"old-support")
            .expect("previous runtime support file should be written");

        let previous_expected_file_manifest = RuntimeExpectedFileManifest {
            schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
            runtime_version: "3.11.8".to_string(),
            files: vec![
                RuntimeExpectedFileRecord {
                    relative_path: "bin/python.exe".to_string(),
                    sha256: hex_sha256(b"old-python"),
                },
                RuntimeExpectedFileRecord {
                    relative_path: "lib/runtime.zip".to_string(),
                    sha256: hex_sha256(b"old-support"),
                },
            ],
        };

        persist_materialized_runtime_package(
            &support_paths,
            &MaterializedRuntimePackage {
                install_record: RuntimeInstallRecord {
                    schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                    executable_relative_path: "bin/python.exe".to_string(),
                    installed_at_epoch_seconds: 50,
                },
                expected_file_manifest: previous_expected_file_manifest.clone(),
                version_record: RuntimeVersionRecord {
                    schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                    runtime_version: "3.11.8".to_string(),
                    compatibility_target_os: std::env::consts::OS.to_string(),
                    compatibility_target_arch: std::env::consts::ARCH.to_string(),
                },
                trust_manifest: RuntimeTrustManifest {
                    schema_version: RUNTIME_RECORD_SCHEMA_VERSION,
                    verification_basis_name: RUNTIME_TRUST_VERIFICATION_BASIS_NAME.to_string(),
                    authorization_manifest_fingerprint_sha256: format!("{:064x}", 3),
                    verified_file_count: previous_expected_file_manifest.files.len(),
                    runtime_version: Some("3.11.8".to_string()),
                    executable_relative_path: Some("bin/python.exe".to_string()),
                    expected_file_manifest_fingerprint_sha256: Some(
                        runtime_expected_file_manifest_fingerprint(
                            &previous_expected_file_manifest,
                        )
                        .expect("previous manifest fingerprint should serialize"),
                    ),
                },
            },
        )
        .expect("previous runtime package should persist");

        let manifest_path = write_manifest_fixture(
            temp_dir.path(),
            std::env::consts::OS,
            std::env::consts::ARCH,
            "bin/python.exe",
            b"new-python",
            "lib/runtime.zip",
            b"new-support",
        );

        let error = hydrate_managed_runtime_with_components_and_hooks(
            &support_paths,
            Some(&manifest_path),
            false,
            100,
            |_paths, _package| Err("forced metadata failure".to_string()),
            clear_rejected_runtime_candidate,
        )
        .expect_err("forced metadata failure should be surfaced");

        assert!(error.contains("previous managed runtime installation was restored"));
        assert_eq!(
            fs::read(support_paths.install_dir.join("bin/python.exe"))
                .expect("previous runtime executable should still exist"),
            b"old-python"
        );
        assert_eq!(
            fs::read(support_paths.install_dir.join("lib/runtime.zip"))
                .expect("previous runtime support file should still exist"),
            b"old-support"
        );

        let restored_state = runtime_package_materialization_state_from_paths(&support_paths)
            .expect("restored runtime package state should load");
        assert_eq!(
            restored_state
                .version_record
                .as_ref()
                .map(|record| record.runtime_version.as_str()),
            Some("3.11.8")
        );
    }

    #[test]
    fn runtime_hydration_state_requires_repair_when_runtime_files_are_missing() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_paths = crate::runtime_package_contract::runtime_support_paths_from_root(
            temp_dir
                .path()
                .join(crate::runtime_package_contract::RUNTIME_SUPPORT_DIRECTORY_NAME),
        );

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
            &support_paths,
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
                    authorization_manifest_fingerprint_sha256: format!("{:064x}", 4),
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
        .expect("runtime package metadata should persist");

        let snapshot = runtime_hydration_state_from_components(&support_paths, None, false);

        assert!(snapshot.hydration_required_now);
        assert_eq!(snapshot.installed_runtime_version.as_deref(), Some("3.11.9"));
        assert!(snapshot
            .status_note
            .contains("on-disk runtime is missing or incomplete"));
    }

    fn write_manifest_fixture(
        base_dir: &Path,
        target_os: &str,
        target_arch: &str,
        executable_relative_path: &str,
        executable_bytes: &[u8],
        support_relative_path: &str,
        support_bytes: &[u8],
    ) -> PathBuf {
        let source_root = base_dir.join("candidate");
        let executable_path = source_root.join(executable_relative_path);
        let support_path = source_root.join(support_relative_path);

        fs::create_dir_all(executable_path.parent().expect("executable parent should exist"))
            .expect("executable parent should be created");
        fs::create_dir_all(support_path.parent().expect("support parent should exist"))
            .expect("support parent should be created");
        fs::write(&executable_path, executable_bytes).expect("executable should be written");
        fs::write(&support_path, support_bytes).expect("support file should be written");

        let manifest_path = base_dir.join("runtime_delivery_manifest.json");
        let manifest = serde_json::json!({
            "schema_version": RUNTIME_RECORD_SCHEMA_VERSION,
            "runtime_version": "3.11.9",
            "source_root": source_root.to_string_lossy(),
            "executable_relative_path": executable_relative_path,
            "compatibility_target_os": target_os,
            "compatibility_target_arch": target_arch,
            "expected_files": [
                {
                    "relative_path": executable_relative_path,
                    "sha256": hex_sha256(executable_bytes),
                },
                {
                    "relative_path": support_relative_path,
                    "sha256": hex_sha256(support_bytes),
                }
            ]
        });

        fs::write(
            &manifest_path,
            serde_json::to_string_pretty(&manifest).expect("manifest should serialize"),
        )
        .expect("manifest should be written");

        manifest_path
    }
}