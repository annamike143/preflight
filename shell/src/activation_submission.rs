use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::activation_contract::{
    ActivationDenialClassification, ActivationTechnicalFailureClassification,
};
use crate::activation_local_validation::{
    commit_approved_activation, current_device_binding_hash, validate_approved_activation_payload,
    LocalActivationApprovalPayload,
};
use crate::activation_secure_store::{ActivationSecureStore, ActivationSecureStoreBackend};
use crate::activation_storage_boundary::ActivationValidationStatus;
use crate::activation_support_store::ActivationSupportStore;

const PYTHON_EXECUTABLE_ENV_VAR: &str = "MIRO_FISH_PYTHON_EXE";
const BACKEND_DATABASE_PATH_ENV_VAR: &str = "MIRO_FISH_COMMERCIAL_BACKEND_DB_PATH";
const BACKEND_SIGNING_KEY_ENV_VAR: &str = "MIRO_FISH_COMMERCIAL_BACKEND_SIGNING_KEY";
const BACKEND_KEY_IDENTIFIER_ENV_VAR: &str = "MIRO_FISH_COMMERCIAL_BACKEND_KEY_IDENTIFIER";
const BACKEND_ISSUER_ENV_VAR: &str = "MIRO_FISH_COMMERCIAL_BACKEND_ISSUER";
const DEFAULT_BACKEND_SIGNING_KEY: &str = "repo-local-commercial-backend-signing-key";
const DEFAULT_BACKEND_KEY_IDENTIFIER: &str = "repo-local-kid-001";
const DEFAULT_BACKEND_ISSUER: &str = "miro-fish-commercial-backend";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ActivationSubmissionRequest {
    pub license_id: String,
    pub existing_activation_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationSubmissionDecisionType {
    Approved,
    Denied,
    TechnicalFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ActivationApprovalOutcome {
    pub license_id: String,
    pub activation_id: String,
    pub activation_slot_index: u32,
    pub plan_tier: String,
    pub key_identifier: String,
    pub reused_existing_slot: bool,
    pub validation_status: ActivationValidationStatus,
    pub activation_committed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ActivationSubmissionOutcome {
    pub decision_type: ActivationSubmissionDecisionType,
    pub approval: Option<ActivationApprovalOutcome>,
    pub denial_reason: Option<ActivationDenialClassification>,
    pub technical_failure_reason: Option<ActivationTechnicalFailureClassification>,
    pub current_device_hash: String,
    pub detail_message: String,
}

impl ActivationSubmissionOutcome {
    fn approved(
        current_device_hash: impl Into<String>,
        approval: ActivationApprovalOutcome,
    ) -> Self {
        Self {
            decision_type: ActivationSubmissionDecisionType::Approved,
            approval: Some(approval),
            denial_reason: None,
            technical_failure_reason: None,
            current_device_hash: current_device_hash.into(),
            detail_message:
                "device activation was approved by the commercial backend and committed locally"
                    .to_string(),
        }
    }

    fn denied(
        current_device_hash: impl Into<String>,
        denial_reason: ActivationDenialClassification,
    ) -> Self {
        Self {
            decision_type: ActivationSubmissionDecisionType::Denied,
            approval: None,
            denial_reason: Some(denial_reason),
            technical_failure_reason: None,
            current_device_hash: current_device_hash.into(),
            detail_message: denial_message(denial_reason).to_string(),
        }
    }

    fn technical_failure(
        current_device_hash: impl Into<String>,
        technical_failure_reason: ActivationTechnicalFailureClassification,
        detail_message: impl Into<String>,
    ) -> Self {
        Self {
            decision_type: ActivationSubmissionDecisionType::TechnicalFailure,
            approval: None,
            denial_reason: None,
            technical_failure_reason: Some(technical_failure_reason),
            current_device_hash: current_device_hash.into(),
            detail_message: detail_message.into(),
        }
    }
}

#[derive(Debug, Clone)]
struct PythonCommercialBackendCommandConfig {
    repo_root: PathBuf,
    python_executable: PathBuf,
    database_path: PathBuf,
    signing_key: String,
    key_identifier: String,
    issuer: String,
}

impl PythonCommercialBackendCommandConfig {
    fn default_for_repo() -> Result<Self, String> {
        let repo_root = repo_root()?;
        let default_python = default_python_executable(&repo_root);
        let default_database_path = repo_root
            .join("commercial_backend")
            .join(".state")
            .join("commercial_ledger.sqlite3");

        Ok(Self {
            repo_root: repo_root.clone(),
            python_executable: env::var_os(PYTHON_EXECUTABLE_ENV_VAR)
                .map(PathBuf::from)
                .unwrap_or(default_python),
            database_path: env::var_os(BACKEND_DATABASE_PATH_ENV_VAR)
                .map(PathBuf::from)
                .unwrap_or(default_database_path),
            signing_key: env::var(BACKEND_SIGNING_KEY_ENV_VAR)
                .unwrap_or_else(|_| DEFAULT_BACKEND_SIGNING_KEY.to_string()),
            key_identifier: env::var(BACKEND_KEY_IDENTIFIER_ENV_VAR)
                .unwrap_or_else(|_| DEFAULT_BACKEND_KEY_IDENTIFIER.to_string()),
            issuer: env::var(BACKEND_ISSUER_ENV_VAR)
                .unwrap_or_else(|_| DEFAULT_BACKEND_ISSUER.to_string()),
        })
    }
}

trait CommercialBackendActivationClient {
    fn evaluate_activation_request(
        &self,
        request: &CommercialBackendActivationRequest,
    ) -> Result<CommercialBackendActivationDecision, String>;
}

struct PythonCommercialBackendActivationClient {
    config: PythonCommercialBackendCommandConfig,
}

impl PythonCommercialBackendActivationClient {
    fn default_for_repo() -> Result<Self, String> {
        Ok(Self {
            config: PythonCommercialBackendCommandConfig::default_for_repo()?,
        })
    }

    #[cfg(test)]
    fn from_config(config: PythonCommercialBackendCommandConfig) -> Self {
        Self { config }
    }

    fn run_command<T: Serialize>(&self, subcommand: &str, payload: &T, extra_args: &[&str]) -> Result<String, String> {
        let mut child = Command::new(&self.config.python_executable)
            .arg("-m")
            .arg("commercial_backend")
            .arg(subcommand)
            .arg("--database-path")
            .arg(&self.config.database_path)
            .args(extra_args)
            .current_dir(self.config.repo_root.join("commercial_backend"))
            .env(
                "PYTHONPATH",
                self.config.repo_root.join("commercial_backend").join("src"),
            )
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| error.to_string())?;

        let serialized_payload = serde_json::to_vec(payload).map_err(|error| error.to_string())?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(&serialized_payload)
                .map_err(|error| error.to_string())?;
        }

        let output = child.wait_with_output().map_err(|error| error.to_string())?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let detail = if !stderr.is_empty() { stderr } else { stdout };
            return Err(if detail.is_empty() {
                format!(
                    "commercial backend command {subcommand} exited with status {}",
                    output.status
                )
            } else {
                detail
            });
        }

        String::from_utf8(output.stdout)
            .map(|stdout| stdout.trim().to_string())
            .map_err(|error| error.to_string())
    }

    #[cfg(test)]
    fn apply_payment_event_for_tests(
        &self,
        payment_event: &CommercialBackendPaymentEventInput,
    ) -> Result<CommercialBackendPaymentEventResult, String> {
        let stdout = self.run_command("apply-payment-event", payment_event, &[])?;
        serde_json::from_str(&stdout).map_err(|error| error.to_string())
    }
}

impl CommercialBackendActivationClient for PythonCommercialBackendActivationClient {
    fn evaluate_activation_request(
        &self,
        request: &CommercialBackendActivationRequest,
    ) -> Result<CommercialBackendActivationDecision, String> {
        let stdout = self.run_command(
            "evaluate-activation-request",
            request,
            &[
                "--signing-key",
                &self.config.signing_key,
                "--key-identifier",
                &self.config.key_identifier,
                "--issuer",
                &self.config.issuer,
            ],
        )?;

        serde_json::from_str(&stdout).map_err(|error| error.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct CommercialBackendActivationRequest {
    license_id: String,
    device_hash: String,
    client_platform_identity: String,
    requested_timestamp_epoch_seconds: u64,
    existing_activation_id: Option<String>,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct CommercialBackendPaymentEventInput {
    payment_provider_identifier: String,
    provider_event_identifier: String,
    event_type: String,
    purchaser_email: String,
    plan_tier: String,
    receipt_timestamp_epoch_seconds: u64,
    license_id: Option<String>,
    expiration_timestamp_epoch_seconds: Option<u64>,
    payload_reference: Option<String>,
    entitlement_active: bool,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CommercialBackendPaymentEventResult {
    license_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CommercialBackendActivationDecision {
    decision_type: CommercialBackendDecisionType,
    approval: Option<CommercialBackendActivationApproval>,
    denial_reason: Option<CommercialBackendDenialReason>,
    technical_failure_reason: Option<CommercialBackendTechnicalFailureReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CommercialBackendDecisionType {
    Approved,
    Denied,
    TechnicalFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CommercialBackendActivationApproval {
    activation_id: String,
    license_id: String,
    activation_slot_index: u32,
    plan_tier: String,
    device_hash: String,
    device_bound_token: String,
    issued_timestamp_epoch_seconds: u64,
    expiration_timestamp_epoch_seconds: Option<u64>,
    key_identifier: String,
    reused_existing_slot: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CommercialBackendDenialReason {
    InvalidLicense,
    UnusableLicense,
    ActivationLimitExceeded,
    DeviceMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CommercialBackendTechnicalFailureReason {
    BackendFailure,
    TokenIssuanceFailure,
}

pub fn submit_device_activation_from_shell(
    request: &ActivationSubmissionRequest,
) -> ActivationSubmissionOutcome {
    let current_device_hash = match current_device_binding_hash() {
        Ok(current_device_hash) => current_device_hash,
        Err(error) => {
            return ActivationSubmissionOutcome::technical_failure(
                "unavailable",
                ActivationTechnicalFailureClassification::BackendFailure,
                format!("current device binding could not be derived: {error}"),
            )
        }
    };

    let support_store = match ActivationSupportStore::default_store() {
        Ok(support_store) => support_store,
        Err(error) => {
            return ActivationSubmissionOutcome::technical_failure(
                current_device_hash,
                ActivationTechnicalFailureClassification::SecureStoreFailure,
                format!("local activation support store could not be resolved: {error}"),
            )
        }
    };

    let current_timestamp_epoch_seconds = match current_epoch_seconds() {
        Ok(current_timestamp_epoch_seconds) => current_timestamp_epoch_seconds,
        Err(error) => {
            return ActivationSubmissionOutcome::technical_failure(
                current_device_hash,
                ActivationTechnicalFailureClassification::BackendFailure,
                error,
            )
        }
    };

    let backend_client = match PythonCommercialBackendActivationClient::default_for_repo() {
        Ok(backend_client) => backend_client,
        Err(error) => {
            return ActivationSubmissionOutcome::technical_failure(
                current_device_hash,
                ActivationTechnicalFailureClassification::BackendFailure,
                format!("commercial backend client could not be resolved: {error}"),
            )
        }
    };

    let secure_store = ActivationSecureStore::os_native();

    submit_device_activation_with_components(
        request,
        &backend_client,
        &secure_store,
        &support_store,
        &current_device_hash,
        current_timestamp_epoch_seconds,
        &client_platform_identity(),
    )
}

fn submit_device_activation_with_components<C, B>(
    request: &ActivationSubmissionRequest,
    backend_client: &C,
    secure_store: &ActivationSecureStore<B>,
    support_store: &ActivationSupportStore,
    current_device_hash: &str,
    current_timestamp_epoch_seconds: u64,
    client_platform_identity: &str,
) -> ActivationSubmissionOutcome
where
    C: CommercialBackendActivationClient,
    B: ActivationSecureStoreBackend,
{
    let normalized_request = normalize_request(request);
    let backend_request = CommercialBackendActivationRequest {
        license_id: normalized_request.license_id,
        device_hash: current_device_hash.to_string(),
        client_platform_identity: client_platform_identity.to_string(),
        requested_timestamp_epoch_seconds: current_timestamp_epoch_seconds,
        existing_activation_id: normalized_request.existing_activation_id,
    };

    let decision = match backend_client.evaluate_activation_request(&backend_request) {
        Ok(decision) => decision,
        Err(error) => {
            return ActivationSubmissionOutcome::technical_failure(
                current_device_hash,
                ActivationTechnicalFailureClassification::BackendFailure,
                format!("commercial backend activation request failed: {error}"),
            )
        }
    };

    match decision.decision_type {
        CommercialBackendDecisionType::Approved => {
            let Some(approval) = decision.approval else {
                return ActivationSubmissionOutcome::technical_failure(
                    current_device_hash,
                    ActivationTechnicalFailureClassification::MalformedTokenResponse,
                    "commercial backend approval omitted the approval payload",
                );
            };

            let local_approval = LocalActivationApprovalPayload {
                license_id: approval.license_id.clone(),
                activation_id: approval.activation_id.clone(),
                activation_slot_index: approval.activation_slot_index,
                plan_tier: approval.plan_tier.clone(),
                device_hash: approval.device_hash.clone(),
                device_bound_token: approval.device_bound_token.clone(),
                issued_timestamp_epoch_seconds: approval.issued_timestamp_epoch_seconds,
                expiration_timestamp_epoch_seconds: approval.expiration_timestamp_epoch_seconds,
                key_identifier: approval.key_identifier.clone(),
            };

            if let Err(error) =
                validate_approved_activation_payload(&local_approval, current_device_hash)
            {
                return ActivationSubmissionOutcome::technical_failure(
                    current_device_hash,
                    ActivationTechnicalFailureClassification::MalformedTokenResponse,
                    format!(
                        "commercial backend approval payload failed shell validation: {error}"
                    ),
                );
            }

            match commit_approved_activation(
                &local_approval,
                secure_store,
                support_store,
                current_device_hash,
                current_timestamp_epoch_seconds,
            ) {
                Ok(commit_result) => ActivationSubmissionOutcome::approved(
                    current_device_hash,
                    ActivationApprovalOutcome {
                        license_id: approval.license_id,
                        activation_id: approval.activation_id,
                        activation_slot_index: approval.activation_slot_index,
                        plan_tier: approval.plan_tier,
                        key_identifier: approval.key_identifier,
                        reused_existing_slot: approval.reused_existing_slot,
                        validation_status: commit_result.validation_status,
                        activation_committed: commit_result.activation_committed,
                    },
                ),
                Err(error) => ActivationSubmissionOutcome::technical_failure(
                    current_device_hash,
                    ActivationTechnicalFailureClassification::SecureStoreFailure,
                    format!("approved activation could not be committed locally: {error}"),
                ),
            }
        }
        CommercialBackendDecisionType::Denied => match decision.denial_reason {
            Some(denial_reason) => ActivationSubmissionOutcome::denied(
                current_device_hash,
                map_backend_denial_reason(denial_reason),
            ),
            None => ActivationSubmissionOutcome::technical_failure(
                current_device_hash,
                ActivationTechnicalFailureClassification::BackendFailure,
                "commercial backend denied activation without a denial reason",
            ),
        },
        CommercialBackendDecisionType::TechnicalFailure => {
            let Some(technical_failure_reason) = decision.technical_failure_reason else {
                return ActivationSubmissionOutcome::technical_failure(
                    current_device_hash,
                    ActivationTechnicalFailureClassification::BackendFailure,
                    "commercial backend returned a technical failure without a failure reason",
                );
            };

            ActivationSubmissionOutcome::technical_failure(
                current_device_hash,
                map_backend_technical_failure_reason(technical_failure_reason),
                technical_failure_message(technical_failure_reason),
            )
        }
    }
}

fn normalize_request(request: &ActivationSubmissionRequest) -> ActivationSubmissionRequest {
    ActivationSubmissionRequest {
        license_id: request.license_id.trim().to_string(),
        existing_activation_id: request
            .existing_activation_id
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned),
    }
}

fn map_backend_denial_reason(
    denial_reason: CommercialBackendDenialReason,
) -> ActivationDenialClassification {
    match denial_reason {
        CommercialBackendDenialReason::InvalidLicense => {
            ActivationDenialClassification::InvalidLicense
        }
        CommercialBackendDenialReason::UnusableLicense => {
            ActivationDenialClassification::UnusableLicense
        }
        CommercialBackendDenialReason::ActivationLimitExceeded => {
            ActivationDenialClassification::ActivationLimitExceeded
        }
        CommercialBackendDenialReason::DeviceMismatch => {
            ActivationDenialClassification::DeviceMismatch
        }
    }
}

fn map_backend_technical_failure_reason(
    technical_failure_reason: CommercialBackendTechnicalFailureReason,
) -> ActivationTechnicalFailureClassification {
    match technical_failure_reason {
        CommercialBackendTechnicalFailureReason::BackendFailure => {
            ActivationTechnicalFailureClassification::BackendFailure
        }
        CommercialBackendTechnicalFailureReason::TokenIssuanceFailure => {
            ActivationTechnicalFailureClassification::BackendFailure
        }
    }
}

fn denial_message(denial_reason: ActivationDenialClassification) -> &'static str {
    match denial_reason {
        ActivationDenialClassification::InvalidLicense => {
            "the provided license reference is not valid for activation"
        }
        ActivationDenialClassification::UnusableLicense => {
            "the provided license is not currently usable for activation"
        }
        ActivationDenialClassification::ActivationLimitExceeded => {
            "the license has already consumed its allowed device activations"
        }
        ActivationDenialClassification::DeviceMismatch => {
            "the provided activation reference does not belong to this device"
        }
    }
}

fn technical_failure_message(
    technical_failure_reason: CommercialBackendTechnicalFailureReason,
) -> &'static str {
    match technical_failure_reason {
        CommercialBackendTechnicalFailureReason::BackendFailure => {
            "the commercial backend reported a technical activation failure"
        }
        CommercialBackendTechnicalFailureReason::TokenIssuanceFailure => {
            "the commercial backend could not issue a device-bound activation token"
        }
    }
}

fn current_epoch_seconds() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())
        .map(|duration| duration.as_secs())
}

fn client_platform_identity() -> String {
    format!("{}-{}", env::consts::OS, env::consts::ARCH)
}

fn repo_root() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "unable to resolve repo root from shell crate location".to_string())
}

fn default_python_executable(repo_root: &Path) -> PathBuf {
    let repo_local_windows_python = repo_root.join(".venv").join("Scripts").join("python.exe");
    if repo_local_windows_python.exists() {
        return repo_local_windows_python;
    }

    let repo_local_unix_python = repo_root.join(".venv").join("bin").join("python");
    if repo_local_unix_python.exists() {
        return repo_local_unix_python;
    }

    PathBuf::from("python")
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    use serde_json::json;
    use tempfile::tempdir;

    use super::*;

    #[derive(Clone, Default)]
    struct InMemorySecureStoreBackend {
        secret: Arc<Mutex<Option<String>>>,
    }

    impl ActivationSecureStoreBackend for InMemorySecureStoreBackend {
        fn set_secret(
            &self,
            _service_name: &str,
            _lookup_reference: &str,
            token: &str,
        ) -> Result<(), String> {
            *self.secret.lock().expect("lock poisoned") = Some(token.to_string());
            Ok(())
        }

        fn get_secret(
            &self,
            _service_name: &str,
            _lookup_reference: &str,
        ) -> Result<Option<String>, String> {
            Ok(self.secret.lock().expect("lock poisoned").clone())
        }

        fn delete_secret(
            &self,
            _service_name: &str,
            _lookup_reference: &str,
        ) -> Result<(), String> {
            *self.secret.lock().expect("lock poisoned") = None;
            Ok(())
        }
    }

    enum StubDecision {
        Ok(CommercialBackendActivationDecision),
        Err(String),
    }

    struct StubCommercialBackendActivationClient {
        decision: StubDecision,
    }

    impl CommercialBackendActivationClient for StubCommercialBackendActivationClient {
        fn evaluate_activation_request(
            &self,
            _request: &CommercialBackendActivationRequest,
        ) -> Result<CommercialBackendActivationDecision, String> {
            match &self.decision {
                StubDecision::Ok(decision) => Ok(decision.clone()),
                StubDecision::Err(error) => Err(error.clone()),
            }
        }
    }

    #[test]
    fn approval_path_commits_locally_after_backend_approval() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );
        let client = StubCommercialBackendActivationClient {
            decision: StubDecision::Ok(CommercialBackendActivationDecision {
                decision_type: CommercialBackendDecisionType::Approved,
                approval: Some(CommercialBackendActivationApproval {
                    activation_id: "activation-001".to_string(),
                    license_id: "license-001".to_string(),
                    activation_slot_index: 1,
                    plan_tier: "annual".to_string(),
                    device_hash: "device-hash-a".to_string(),
                    device_bound_token: token_for(
                        "license-001",
                        "annual",
                        "device-hash-a",
                        1,
                        100,
                        Some(200),
                        "kid-001",
                        "jti-001",
                    ),
                    issued_timestamp_epoch_seconds: 100,
                    expiration_timestamp_epoch_seconds: Some(200),
                    key_identifier: "kid-001".to_string(),
                    reused_existing_slot: false,
                }),
                denial_reason: None,
                technical_failure_reason: None,
            }),
        };

        let outcome = submit_device_activation_with_components(
            &ActivationSubmissionRequest {
                license_id: "license-001".to_string(),
                existing_activation_id: None,
            },
            &client,
            &secure_store,
            &support_store,
            "device-hash-a",
            100,
            "windows-x86_64",
        );

        assert_eq!(outcome.decision_type, ActivationSubmissionDecisionType::Approved);
        assert_eq!(
            outcome.approval.expect("approval should exist").validation_status,
            ActivationValidationStatus::ActivatedValid
        );
        assert!(secure_store.read_token().expect("read should succeed").is_some());
        assert!(support_store.load_record().expect("load should succeed").is_some());
    }

    #[test]
    fn denial_path_does_not_commit_local_activation_state() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );
        let client = StubCommercialBackendActivationClient {
            decision: StubDecision::Ok(CommercialBackendActivationDecision {
                decision_type: CommercialBackendDecisionType::Denied,
                approval: None,
                denial_reason: Some(CommercialBackendDenialReason::InvalidLicense),
                technical_failure_reason: None,
            }),
        };

        let outcome = submit_device_activation_with_components(
            &ActivationSubmissionRequest {
                license_id: "missing-license".to_string(),
                existing_activation_id: None,
            },
            &client,
            &secure_store,
            &support_store,
            "device-hash-a",
            100,
            "windows-x86_64",
        );

        assert_eq!(outcome.decision_type, ActivationSubmissionDecisionType::Denied);
        assert_eq!(
            outcome.denial_reason,
            Some(ActivationDenialClassification::InvalidLicense)
        );
        assert_eq!(secure_store.read_token().expect("read should succeed"), None);
        assert!(support_store.load_record().expect("load should succeed").is_none());
    }

    #[test]
    fn malformed_approval_payload_is_reported_as_a_technical_failure() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );
        let client = StubCommercialBackendActivationClient {
            decision: StubDecision::Ok(CommercialBackendActivationDecision {
                decision_type: CommercialBackendDecisionType::Approved,
                approval: Some(CommercialBackendActivationApproval {
                    activation_id: "activation-001".to_string(),
                    license_id: "license-001".to_string(),
                    activation_slot_index: 1,
                    plan_tier: "annual".to_string(),
                    device_hash: "device-hash-b".to_string(),
                    device_bound_token: token_for(
                        "license-001",
                        "annual",
                        "device-hash-b",
                        1,
                        100,
                        Some(200),
                        "kid-001",
                        "jti-001",
                    ),
                    issued_timestamp_epoch_seconds: 100,
                    expiration_timestamp_epoch_seconds: Some(200),
                    key_identifier: "kid-001".to_string(),
                    reused_existing_slot: false,
                }),
                denial_reason: None,
                technical_failure_reason: None,
            }),
        };

        let outcome = submit_device_activation_with_components(
            &ActivationSubmissionRequest {
                license_id: "license-001".to_string(),
                existing_activation_id: None,
            },
            &client,
            &secure_store,
            &support_store,
            "device-hash-a",
            100,
            "windows-x86_64",
        );

        assert_eq!(
            outcome.decision_type,
            ActivationSubmissionDecisionType::TechnicalFailure
        );
        assert_eq!(
            outcome.technical_failure_reason,
            Some(ActivationTechnicalFailureClassification::MalformedTokenResponse)
        );
        assert_eq!(secure_store.read_token().expect("read should succeed"), None);
        assert!(support_store.load_record().expect("load should succeed").is_none());
    }

    #[test]
    fn backend_command_failure_is_reported_as_a_technical_failure() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );
        let client = StubCommercialBackendActivationClient {
            decision: StubDecision::Err("backend subprocess failed".to_string()),
        };

        let outcome = submit_device_activation_with_components(
            &ActivationSubmissionRequest {
                license_id: "license-001".to_string(),
                existing_activation_id: None,
            },
            &client,
            &secure_store,
            &support_store,
            "device-hash-a",
            100,
            "windows-x86_64",
        );

        assert_eq!(
            outcome.decision_type,
            ActivationSubmissionDecisionType::TechnicalFailure
        );
        assert_eq!(
            outcome.technical_failure_reason,
            Some(ActivationTechnicalFailureClassification::BackendFailure)
        );
    }

    #[test]
    fn repo_local_python_backend_round_trip_supports_approval() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let config = test_backend_config(temp_dir.path().join("ledger.sqlite3"), "signing-key-001");
        let client = PythonCommercialBackendActivationClient::from_config(config);
        let license_id = seed_payment_event(&client, "evt-approval-001");
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );

        let outcome = submit_device_activation_with_components(
            &ActivationSubmissionRequest {
                license_id,
                existing_activation_id: None,
            },
            &client,
            &secure_store,
            &support_store,
            "device-hash-a",
            100,
            "windows-x86_64",
        );

        assert_eq!(outcome.decision_type, ActivationSubmissionDecisionType::Approved);
        assert!(secure_store.read_token().expect("read should succeed").is_some());
        assert!(support_store.load_record().expect("load should succeed").is_some());
    }

    #[test]
    fn repo_local_python_backend_round_trip_supports_denial() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let client = PythonCommercialBackendActivationClient::from_config(test_backend_config(
            temp_dir.path().join("ledger.sqlite3"),
            "signing-key-001",
        ));
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );

        let outcome = submit_device_activation_with_components(
            &ActivationSubmissionRequest {
                license_id: "missing-license".to_string(),
                existing_activation_id: None,
            },
            &client,
            &secure_store,
            &support_store,
            "device-hash-a",
            100,
            "windows-x86_64",
        );

        assert_eq!(outcome.decision_type, ActivationSubmissionDecisionType::Denied);
        assert_eq!(
            outcome.denial_reason,
            Some(ActivationDenialClassification::InvalidLicense)
        );
    }

    #[test]
    fn repo_local_python_backend_round_trip_supports_technical_failure() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let config = test_backend_config(temp_dir.path().join("ledger.sqlite3"), "signing-key-001");
        let client = PythonCommercialBackendActivationClient::from_config(config.clone());
        let license_id = seed_payment_event(&client, "evt-technical-001");
        let failing_client = PythonCommercialBackendActivationClient::from_config(
            PythonCommercialBackendCommandConfig {
                signing_key: String::new(),
                ..config
            },
        );
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );

        let outcome = submit_device_activation_with_components(
            &ActivationSubmissionRequest {
                license_id,
                existing_activation_id: None,
            },
            &failing_client,
            &secure_store,
            &support_store,
            "device-hash-a",
            100,
            "windows-x86_64",
        );

        assert_eq!(
            outcome.decision_type,
            ActivationSubmissionDecisionType::TechnicalFailure
        );
        assert_eq!(
            outcome.technical_failure_reason,
            Some(ActivationTechnicalFailureClassification::BackendFailure)
        );
    }

    fn seed_payment_event(
        client: &PythonCommercialBackendActivationClient,
        provider_event_identifier: &str,
    ) -> String {
        client
            .apply_payment_event_for_tests(&CommercialBackendPaymentEventInput {
                payment_provider_identifier: "stripe".to_string(),
                provider_event_identifier: provider_event_identifier.to_string(),
                event_type: "payment_succeeded".to_string(),
                purchaser_email: "owner@example.com".to_string(),
                plan_tier: "annual".to_string(),
                receipt_timestamp_epoch_seconds: 100,
                license_id: None,
                expiration_timestamp_epoch_seconds: None,
                payload_reference: Some("evt_ref_001".to_string()),
                entitlement_active: true,
            })
            .expect("payment event should seed the commercial backend")
            .license_id
    }

    fn test_backend_config(
        database_path: PathBuf,
        signing_key: &str,
    ) -> PythonCommercialBackendCommandConfig {
        let repo_root = repo_root().expect("repo root should resolve");

        PythonCommercialBackendCommandConfig {
            python_executable: default_python_executable(&repo_root),
            repo_root,
            database_path,
            signing_key: signing_key.to_string(),
            key_identifier: "kid-001".to_string(),
            issuer: DEFAULT_BACKEND_ISSUER.to_string(),
        }
    }

    fn token_for(
        license_id: &str,
        plan_tier: &str,
        device_hash: &str,
        activation_slot: u32,
        issued_at: u64,
        expiration_timestamp: Option<u64>,
        key_identifier: &str,
        token_lineage_marker: &str,
    ) -> String {
        let header = json!({"alg": "HS256", "kid": key_identifier, "typ": "JWT"});
        let claims = json!({
            "sub": "owner@example.com",
            "license_id": license_id,
            "plan_tier": plan_tier,
            "device_hash": device_hash,
            "activation_slot": activation_slot,
            "iat": issued_at,
            "exp": expiration_timestamp,
            "iss": DEFAULT_BACKEND_ISSUER,
            "jti": token_lineage_marker,
        });

        let encoded_header = URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&header).expect("header should serialize"),
        );
        let encoded_claims = URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&claims).expect("claims should serialize"),
        );

        format!("{encoded_header}.{encoded_claims}.signature")
    }
}