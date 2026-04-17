use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::provider_configuration_contract::{
    provider_configuration_contract_basis, ProviderConfigurationFailureClassification,
    ProviderConfigurationStatus, SupportedProvider,
};
use crate::provider_secure_store::{ProviderSecureStore, ProviderSecureStoreBackend};
use crate::provider_storage_boundary::provider_storage_boundary_surface;
use crate::provider_support_store::{
    ProviderConfigurationOutcomeMarker, ProviderSupportStateRecord, ProviderSupportStore,
    PROVIDER_SUPPORT_STATE_SCHEMA_VERSION,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProviderConfigurationSubmissionRequest {
    pub selected_provider: String,
    pub api_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedProviderConfigurationRequest {
    selected_provider: SupportedProvider,
    api_key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderConfigurationSubmissionDecisionType {
    Success,
    Failure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderConfigurationStateDetail {
    FlowNotMaterialized,
    SelectionMissing,
    SecretMissing,
    SecretFingerprintMismatch,
    StateAccessFailure,
    Configured,
}

impl ProviderConfigurationStateDetail {
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProviderConfigurationStateSnapshot {
    pub flow_materialized: bool,
    pub support_record_present: bool,
    pub selected_provider: Option<SupportedProvider>,
    pub secure_store_lookup_reference: Option<String>,
    pub last_configuration_outcome: Option<ProviderConfigurationOutcomeMarker>,
    pub provider_secret_reference_present: bool,
    pub provider_secret_fingerprint_verified: bool,
    pub configuration_present: bool,
    pub current_status: ProviderConfigurationStatus,
    pub current_detail: ProviderConfigurationStateDetail,
}

impl ProviderConfigurationStateSnapshot {
    fn flow_not_materialized() -> Self {
        Self {
            flow_materialized: false,
            support_record_present: false,
            selected_provider: None,
            secure_store_lookup_reference: None,
            last_configuration_outcome: None,
            provider_secret_reference_present: false,
            provider_secret_fingerprint_verified: false,
            configuration_present: false,
            current_status: ProviderConfigurationStatus::Required,
            current_detail: ProviderConfigurationStateDetail::FlowNotMaterialized,
        }
    }

    fn selection_missing() -> Self {
        Self {
            flow_materialized: true,
            support_record_present: false,
            selected_provider: None,
            secure_store_lookup_reference: None,
            last_configuration_outcome: None,
            provider_secret_reference_present: false,
            provider_secret_fingerprint_verified: false,
            configuration_present: false,
            current_status: ProviderConfigurationStatus::Required,
            current_detail: ProviderConfigurationStateDetail::SelectionMissing,
        }
    }

    fn state_access_failure() -> Self {
        Self {
            flow_materialized: true,
            support_record_present: false,
            selected_provider: None,
            secure_store_lookup_reference: None,
            last_configuration_outcome: None,
            provider_secret_reference_present: false,
            provider_secret_fingerprint_verified: false,
            configuration_present: false,
            current_status: ProviderConfigurationStatus::Error,
            current_detail: ProviderConfigurationStateDetail::StateAccessFailure,
        }
    }

    fn from_support_record(
        record: &ProviderSupportStateRecord,
        provider_secret_reference_present: bool,
        provider_secret_fingerprint_verified: bool,
        current_status: ProviderConfigurationStatus,
        current_detail: ProviderConfigurationStateDetail,
    ) -> Self {
        Self {
            flow_materialized: true,
            support_record_present: true,
            selected_provider: Some(record.selected_provider),
            secure_store_lookup_reference: Some(record.secure_store_lookup_reference.clone()),
            last_configuration_outcome: Some(record.last_configuration_outcome),
            provider_secret_reference_present,
            provider_secret_fingerprint_verified,
            configuration_present: current_status == ProviderConfigurationStatus::Present,
            current_status,
            current_detail,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProviderConfigurationSuccessOutcome {
    pub selected_provider: SupportedProvider,
    pub secure_store_lookup_reference: String,
    pub support_record_path: String,
    pub configuration_status: ProviderConfigurationStatus,
    pub configuration_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProviderConfigurationSubmissionOutcome {
    pub decision_type: ProviderConfigurationSubmissionDecisionType,
    pub success: Option<ProviderConfigurationSuccessOutcome>,
    pub failure_reason: Option<ProviderConfigurationFailureClassification>,
    pub configuration_state: ProviderConfigurationStateSnapshot,
    pub detail_message: String,
}

impl ProviderConfigurationSubmissionOutcome {
    fn success(
        success: ProviderConfigurationSuccessOutcome,
        configuration_state: ProviderConfigurationStateSnapshot,
    ) -> Self {
        Self {
            decision_type: ProviderConfigurationSubmissionDecisionType::Success,
            success: Some(success),
            failure_reason: None,
            configuration_state,
            detail_message:
                "provider configuration committed locally; Preflight still remains a separate later gate"
                    .to_string(),
        }
    }

    fn failure(
        failure_reason: ProviderConfigurationFailureClassification,
        configuration_state: ProviderConfigurationStateSnapshot,
        detail_message: impl Into<String>,
    ) -> Self {
        Self {
            decision_type: ProviderConfigurationSubmissionDecisionType::Failure,
            success: None,
            failure_reason: Some(failure_reason),
            configuration_state,
            detail_message: detail_message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediatedProviderSecret {
    pub provider: SupportedProvider,
    pub api_key: String,
    pub secure_store_lookup_reference: String,
}

pub fn provider_configuration_state_from_shell() -> ProviderConfigurationStateSnapshot {
    if !provider_configuration_flow_materialized() {
        return ProviderConfigurationStateSnapshot::flow_not_materialized();
    }

    let secure_store = ProviderSecureStore::os_native();
    let support_store = match ProviderSupportStore::default_store() {
        Ok(support_store) => support_store,
        Err(_) => return ProviderConfigurationStateSnapshot::state_access_failure(),
    };

    provider_configuration_state_with_components(&secure_store, &support_store)
}

pub fn submit_provider_configuration_from_shell(
    request: &ProviderConfigurationSubmissionRequest,
) -> ProviderConfigurationSubmissionOutcome {
    if !provider_configuration_flow_materialized() {
        return ProviderConfigurationSubmissionOutcome::failure(
            ProviderConfigurationFailureClassification::SecureStorageFailure,
            ProviderConfigurationStateSnapshot::flow_not_materialized(),
            "provider configuration flow is not materialized yet",
        );
    }

    let secure_store = ProviderSecureStore::os_native();
    let support_store = match ProviderSupportStore::default_store() {
        Ok(support_store) => support_store,
        Err(error) => {
            return ProviderConfigurationSubmissionOutcome::failure(
                ProviderConfigurationFailureClassification::SecureStorageFailure,
                ProviderConfigurationStateSnapshot::state_access_failure(),
                format!(
                    "local provider support-state store could not be resolved: {error}"
                ),
            )
        }
    };

    let current_timestamp_epoch_seconds = match current_epoch_seconds() {
        Ok(current_timestamp_epoch_seconds) => current_timestamp_epoch_seconds,
        Err(error) => {
            return ProviderConfigurationSubmissionOutcome::failure(
                ProviderConfigurationFailureClassification::SecureStorageFailure,
                provider_configuration_state_with_components(&secure_store, &support_store),
                format!("provider configuration timestamp could not be derived: {error}"),
            )
        }
    };

    submit_provider_configuration_request_with_components(
        request,
        &secure_store,
        &support_store,
        current_timestamp_epoch_seconds,
    )
}

fn submit_provider_configuration_request_with_components<B: ProviderSecureStoreBackend>(
    request: &ProviderConfigurationSubmissionRequest,
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
    current_timestamp_epoch_seconds: u64,
) -> ProviderConfigurationSubmissionOutcome {
    let normalized_request = match normalize_request(request) {
        Ok(normalized_request) => normalized_request,
        Err(submitted_provider) => {
            return ProviderConfigurationSubmissionOutcome::failure(
                ProviderConfigurationFailureClassification::UnsupportedProvider,
                ProviderConfigurationStateSnapshot::selection_missing(),
                format!(
                    "provider selection '{}' is not approved for MVP use",
                    submitted_provider
                ),
            )
        }
    };

    submit_provider_configuration_with_components(
        &normalized_request,
        secure_store,
        support_store,
        current_timestamp_epoch_seconds,
    )
}

pub fn provider_configuration_state_with_components<B: ProviderSecureStoreBackend>(
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
) -> ProviderConfigurationStateSnapshot {
    if !provider_configuration_flow_materialized() {
        return ProviderConfigurationStateSnapshot::flow_not_materialized();
    }

    let support_record = match support_store.load_record() {
        Ok(support_record) => support_record,
        Err(_) => return ProviderConfigurationStateSnapshot::state_access_failure(),
    };

    let Some(support_record) = support_record else {
        return ProviderConfigurationStateSnapshot::selection_missing();
    };

    if support_record.secure_store_lookup_reference.trim().is_empty() {
        return ProviderConfigurationStateSnapshot::state_access_failure();
    }

    let expected_lookup_reference = secure_store.lookup_reference(support_record.selected_provider);
    if support_record.secure_store_lookup_reference != expected_lookup_reference {
        return ProviderConfigurationStateSnapshot::state_access_failure();
    }

    let provider_secret = match secure_store.read_secret(support_record.selected_provider) {
        Ok(provider_secret) => provider_secret,
        Err(_) => return ProviderConfigurationStateSnapshot::state_access_failure(),
    };

    let Some(provider_secret) = provider_secret else {
        return ProviderConfigurationStateSnapshot::from_support_record(
            &support_record,
            false,
            false,
            ProviderConfigurationStatus::Required,
            ProviderConfigurationStateDetail::SecretMissing,
        );
    };

    let provider_secret_fingerprint_verified =
        hex_sha256(&provider_secret) == support_record.provider_secret_fingerprint_sha256;
    if !provider_secret_fingerprint_verified {
        return ProviderConfigurationStateSnapshot::from_support_record(
            &support_record,
            true,
            false,
            ProviderConfigurationStatus::Error,
            ProviderConfigurationStateDetail::SecretFingerprintMismatch,
        );
    }

    let current_status = support_record.configuration_status;
    let current_detail = match current_status {
        ProviderConfigurationStatus::Present => ProviderConfigurationStateDetail::Configured,
        ProviderConfigurationStatus::Required => ProviderConfigurationStateDetail::SelectionMissing,
        ProviderConfigurationStatus::Error => ProviderConfigurationStateDetail::StateAccessFailure,
    };

    ProviderConfigurationStateSnapshot::from_support_record(
        &support_record,
        true,
        true,
        current_status,
        current_detail,
    )
}

fn submit_provider_configuration_with_components<B: ProviderSecureStoreBackend>(
    request: &NormalizedProviderConfigurationRequest,
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
    current_timestamp_epoch_seconds: u64,
) -> ProviderConfigurationSubmissionOutcome {
    let api_key = request.api_key.trim();

    if api_key.is_empty() {
        return ProviderConfigurationSubmissionOutcome::failure(
            ProviderConfigurationFailureClassification::MalformedInput,
            provider_configuration_state_with_components(secure_store, support_store),
            "provider API key is required before configuration can be committed",
        );
    }

    let previous_support_record = match support_store.load_record() {
        Ok(previous_support_record) => previous_support_record,
        Err(error) => {
            return ProviderConfigurationSubmissionOutcome::failure(
                ProviderConfigurationFailureClassification::SecureStorageFailure,
                ProviderConfigurationStateSnapshot::state_access_failure(),
                format!("existing provider support state could not be loaded: {error}"),
            )
        }
    };

    let previous_target_secret = match secure_store.read_secret(request.selected_provider) {
        Ok(previous_target_secret) => previous_target_secret,
        Err(error) => {
            return ProviderConfigurationSubmissionOutcome::failure(
                classify_local_storage_error(&error),
                provider_configuration_state_with_components(secure_store, support_store),
                format!(
                    "existing secure-store provider key could not be read for rollback safety: {error}"
                ),
            )
        }
    };

    let mut previous_other_provider_secret = None;
    let mut previous_other_provider = None;
    if let Some(previous_support_record) = previous_support_record.as_ref() {
        if previous_support_record.selected_provider != request.selected_provider {
            previous_other_provider = Some(previous_support_record.selected_provider);
            previous_other_provider_secret = match secure_store.read_secret(previous_support_record.selected_provider) {
                Ok(previous_other_provider_secret) => previous_other_provider_secret,
                Err(error) => {
                    return ProviderConfigurationSubmissionOutcome::failure(
                        classify_local_storage_error(&error),
                        provider_configuration_state_with_components(secure_store, support_store),
                        format!(
                            "previous secure-store provider key could not be read for rollback safety: {error}"
                        ),
                    )
                }
            };
        }
    }

    if let Err(error) = secure_store.commit_secret(
        request.selected_provider,
        api_key,
    ) {
        return ProviderConfigurationSubmissionOutcome::failure(
            classify_local_storage_error(&error),
            provider_configuration_state_with_components(secure_store, support_store),
            format!("provider API key could not be committed to secure storage: {error}"),
        );
    }

    if let Some(previous_other_provider) = previous_other_provider {
        if let Err(error) = secure_store.clear_secret(previous_other_provider) {
            let restoration_error = restore_previous_provider_state(
                request.selected_provider,
                &previous_target_secret,
                Some(previous_other_provider),
                &previous_other_provider_secret,
                previous_support_record.as_ref(),
                secure_store,
                support_store,
            )
            .err();

            return ProviderConfigurationSubmissionOutcome::failure(
                classify_local_storage_error(&error),
                provider_configuration_state_with_components(secure_store, support_store),
                with_restoration_context(
                    format!("previous provider key could not be cleared during provider switch: {error}"),
                    restoration_error,
                ),
            );
        }
    }

    let support_record = ProviderSupportStateRecord {
        schema_version: PROVIDER_SUPPORT_STATE_SCHEMA_VERSION,
        selected_provider: request.selected_provider,
        secure_store_lookup_reference: secure_store.lookup_reference(request.selected_provider),
        provider_secret_fingerprint_sha256: hex_sha256(api_key),
        configuration_status: ProviderConfigurationStatus::Present,
        last_configuration_outcome: ProviderConfigurationOutcomeMarker::ConfiguredLocally,
        last_updated_timestamp_epoch_seconds: current_timestamp_epoch_seconds,
    };

    if let Err(error) = support_store.persist_record(&support_record) {
        let restoration_error = restore_previous_provider_state(
            request.selected_provider,
            &previous_target_secret,
            previous_other_provider,
            &previous_other_provider_secret,
            previous_support_record.as_ref(),
            secure_store,
            support_store,
        )
        .err();

        return ProviderConfigurationSubmissionOutcome::failure(
            ProviderConfigurationFailureClassification::SecureStorageFailure,
            provider_configuration_state_with_components(secure_store, support_store),
            with_restoration_context(
                format!("provider support state could not be persisted: {error}"),
                restoration_error,
            ),
        );
    }

    let retrieval_result = retrieve_provider_secret_with_components(
        request.selected_provider,
        secure_store,
        support_store,
    );
    match retrieval_result {
        Ok(Some(_)) => {
            let configuration_state = provider_configuration_state_with_components(secure_store, support_store);
            if configuration_state.current_status != ProviderConfigurationStatus::Present {
                let restoration_error = restore_previous_provider_state(
                    request.selected_provider,
                    &previous_target_secret,
                    previous_other_provider,
                    &previous_other_provider_secret,
                    previous_support_record.as_ref(),
                    secure_store,
                    support_store,
                )
                .err();

                return ProviderConfigurationSubmissionOutcome::failure(
                    ProviderConfigurationFailureClassification::SecureStorageFailure,
                    provider_configuration_state_with_components(secure_store, support_store),
                    with_restoration_context(
                        "provider configuration did not stabilize into a present local state"
                            .to_string(),
                        restoration_error,
                    ),
                );
            }

            ProviderConfigurationSubmissionOutcome::success(
                ProviderConfigurationSuccessOutcome {
                    selected_provider: request.selected_provider,
                    secure_store_lookup_reference: support_record.secure_store_lookup_reference,
                    support_record_path: support_store.record_path().display().to_string(),
                    configuration_status: configuration_state.current_status,
                    configuration_present: configuration_state.configuration_present,
                },
                configuration_state,
            )
        }
        Ok(None) => {
            let restoration_error = restore_previous_provider_state(
                request.selected_provider,
                &previous_target_secret,
                previous_other_provider,
                &previous_other_provider_secret,
                previous_support_record.as_ref(),
                secure_store,
                support_store,
            )
            .err();

            ProviderConfigurationSubmissionOutcome::failure(
                ProviderConfigurationFailureClassification::SecureStorageFailure,
                provider_configuration_state_with_components(secure_store, support_store),
                with_restoration_context(
                    "provider key could not be retrieved through the shell mediation layer after commit"
                        .to_string(),
                    restoration_error,
                ),
            )
        }
        Err(error) => {
            let restoration_error = restore_previous_provider_state(
                request.selected_provider,
                &previous_target_secret,
                previous_other_provider,
                &previous_other_provider_secret,
                previous_support_record.as_ref(),
                secure_store,
                support_store,
            )
            .err();

            ProviderConfigurationSubmissionOutcome::failure(
                classify_local_storage_error(&error),
                provider_configuration_state_with_components(secure_store, support_store),
                with_restoration_context(
                    format!(
                        "provider key mediation failed after local commit attempt: {error}"
                    ),
                    restoration_error,
                ),
            )
        }
    }
}

pub fn retrieve_provider_secret_with_components<B: ProviderSecureStoreBackend>(
    provider: SupportedProvider,
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
) -> Result<Option<MediatedProviderSecret>, String> {
    if !provider_configuration_flow_materialized() {
        return Ok(None);
    }

    let support_record = support_store.load_record()?;
    let Some(support_record) = support_record else {
        return Ok(None);
    };

    if support_record.selected_provider != provider {
        return Ok(None);
    }

    let expected_lookup_reference = secure_store.lookup_reference(provider);
    if support_record.secure_store_lookup_reference != expected_lookup_reference {
        return Err(
            "provider support state does not match the secure-store lookup reference for the selected provider"
                .to_string(),
        );
    }

    let provider_secret = secure_store.read_secret(provider)?;
    let Some(provider_secret) = provider_secret else {
        return Ok(None);
    };

    if hex_sha256(&provider_secret) != support_record.provider_secret_fingerprint_sha256 {
        return Err(
            "provider support state fingerprint does not match the secure-store provider key"
                .to_string(),
        );
    }

    Ok(Some(MediatedProviderSecret {
        provider,
        api_key: provider_secret,
        secure_store_lookup_reference: support_record.secure_store_lookup_reference,
    }))
}

fn provider_configuration_flow_materialized() -> bool {
    let provider_contract = provider_configuration_contract_basis();
    let provider_storage_boundary = provider_storage_boundary_surface();

    provider_contract.provider_submission_command_materialized
        && provider_storage_boundary.secure_store_commit_implemented
        && provider_storage_boundary.secure_store_retrieval_implemented
        && provider_storage_boundary.support_state_persistence_implemented
}

fn normalize_request(
    request: &ProviderConfigurationSubmissionRequest,
) -> Result<NormalizedProviderConfigurationRequest, String> {
    let submitted_provider = request.selected_provider.trim();
    let selected_provider = match SupportedProvider::parse(submitted_provider) {
        Some(selected_provider) => selected_provider,
        None => return Err(submitted_provider.to_string()),
    };

    Ok(NormalizedProviderConfigurationRequest {
        selected_provider,
        api_key: request.api_key.trim().to_string(),
    })
}

fn restore_previous_provider_state<B: ProviderSecureStoreBackend>(
    selected_provider: SupportedProvider,
    previous_target_secret: &Option<String>,
    previous_other_provider: Option<SupportedProvider>,
    previous_other_provider_secret: &Option<String>,
    previous_support_record: Option<&ProviderSupportStateRecord>,
    secure_store: &ProviderSecureStore<B>,
    support_store: &ProviderSupportStore,
) -> Result<(), String> {
    match previous_target_secret {
        Some(previous_target_secret) => {
            secure_store.commit_secret(selected_provider, previous_target_secret)?;
        }
        None => {
            secure_store.clear_secret(selected_provider)?;
        }
    }

    if let Some(previous_other_provider) = previous_other_provider {
        match previous_other_provider_secret {
            Some(previous_other_provider_secret) => {
                secure_store.commit_secret(previous_other_provider, previous_other_provider_secret)?;
            }
            None => {
                secure_store.clear_secret(previous_other_provider)?;
            }
        }
    }

    match previous_support_record {
        Some(previous_support_record) => support_store.persist_record(previous_support_record),
        None => support_store.clear_record(),
    }
}

fn with_restoration_context(detail_message: String, restoration_error: Option<String>) -> String {
    match restoration_error {
        Some(restoration_error) => {
            format!("{detail_message}; rollback also reported: {restoration_error}")
        }
        None => detail_message,
    }
}

fn classify_local_storage_error(error: &str) -> ProviderConfigurationFailureClassification {
    let normalized_error = error.to_ascii_lowercase();
    if normalized_error.contains("permission")
        || normalized_error.contains("denied")
        || normalized_error.contains("not permitted")
        || normalized_error.contains("not allowed")
        || normalized_error.contains("access is denied")
    {
        ProviderConfigurationFailureClassification::OsPermissionFailure
    } else {
        ProviderConfigurationFailureClassification::SecureStorageFailure
    }
}

fn current_epoch_seconds() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())
        .map(|duration| duration.as_secs())
}

fn hex_sha256(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use tempfile::tempdir;

    use super::*;

    #[derive(Clone, Default)]
    struct InMemorySecureStoreBackend {
        secrets: Arc<Mutex<HashMap<String, String>>>,
        fail_on_set_with: Arc<Mutex<Option<String>>>,
    }

    impl InMemorySecureStoreBackend {
        fn failing_with(message: &str) -> Self {
            Self {
                fail_on_set_with: Arc::new(Mutex::new(Some(message.to_string()))),
                ..Self::default()
            }
        }
    }

    impl ProviderSecureStoreBackend for InMemorySecureStoreBackend {
        fn set_secret(
            &self,
            _service_name: &str,
            lookup_reference: &str,
            secret: &str,
        ) -> Result<(), String> {
            if let Some(message) = self.fail_on_set_with.lock().expect("lock poisoned").clone() {
                return Err(message);
            }

            self.secrets
                .lock()
                .expect("lock poisoned")
                .insert(lookup_reference.to_string(), secret.to_string());
            Ok(())
        }

        fn get_secret(
            &self,
            _service_name: &str,
            lookup_reference: &str,
        ) -> Result<Option<String>, String> {
            Ok(self
                .secrets
                .lock()
                .expect("lock poisoned")
                .get(lookup_reference)
                .cloned())
        }

        fn delete_secret(
            &self,
            _service_name: &str,
            lookup_reference: &str,
        ) -> Result<(), String> {
            self.secrets
                .lock()
                .expect("lock poisoned")
                .remove(lookup_reference);
            Ok(())
        }
    }

    #[test]
    fn provider_configuration_submission_commits_local_state_and_mediates_retrieval() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ProviderSupportStore::from_record_path(
            temp_dir.path().join("provider_support_record.json"),
        );
        let secure_store = ProviderSecureStore::with_backend(InMemorySecureStoreBackend::default());

        let outcome = submit_provider_configuration_with_components(
            &NormalizedProviderConfigurationRequest {
                selected_provider: SupportedProvider::OpenAi,
                api_key: "sk-openai-001".to_string(),
            },
            &secure_store,
            &support_store,
            100,
        );

        assert_eq!(
            outcome.decision_type,
            ProviderConfigurationSubmissionDecisionType::Success
        );
        assert_eq!(
            outcome.configuration_state.current_status,
            ProviderConfigurationStatus::Present
        );
        assert_eq!(
            outcome.configuration_state.current_detail,
            ProviderConfigurationStateDetail::Configured
        );
        assert!(outcome.configuration_state.configuration_present);

        let mediated_secret = retrieve_provider_secret_with_components(
            SupportedProvider::OpenAi,
            &secure_store,
            &support_store,
        )
        .expect("retrieval should succeed")
        .expect("mediated secret should exist");

        assert_eq!(mediated_secret.provider, SupportedProvider::OpenAi);
        assert_eq!(mediated_secret.api_key, "sk-openai-001");
        assert!(support_store.load_record().unwrap().is_some());
    }

    #[test]
    fn provider_configuration_submission_rejects_empty_api_keys_without_mutating_state() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ProviderSupportStore::from_record_path(
            temp_dir.path().join("provider_support_record.json"),
        );
        let secure_store = ProviderSecureStore::with_backend(InMemorySecureStoreBackend::default());

        let outcome = submit_provider_configuration_with_components(
            &NormalizedProviderConfigurationRequest {
                selected_provider: SupportedProvider::Google,
                api_key: "   ".to_string(),
            },
            &secure_store,
            &support_store,
            100,
        );

        assert_eq!(
            outcome.decision_type,
            ProviderConfigurationSubmissionDecisionType::Failure
        );
        assert_eq!(
            outcome.failure_reason,
            Some(ProviderConfigurationFailureClassification::MalformedInput)
        );
        assert_eq!(
            outcome.configuration_state.current_status,
            ProviderConfigurationStatus::Required
        );
        assert!(support_store.load_record().unwrap().is_none());
        assert_eq!(secure_store.read_secret(SupportedProvider::Google).unwrap(), None);
    }

    #[test]
    fn provider_configuration_submission_classifies_permission_failures() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ProviderSupportStore::from_record_path(
            temp_dir.path().join("provider_support_record.json"),
        );
        let secure_store = ProviderSecureStore::with_backend(
            InMemorySecureStoreBackend::failing_with("permission denied by OS secure store"),
        );

        let outcome = submit_provider_configuration_with_components(
            &NormalizedProviderConfigurationRequest {
                selected_provider: SupportedProvider::OpenAi,
                api_key: "sk-openai-001".to_string(),
            },
            &secure_store,
            &support_store,
            100,
        );

        assert_eq!(
            outcome.decision_type,
            ProviderConfigurationSubmissionDecisionType::Failure
        );
        assert_eq!(
            outcome.failure_reason,
            Some(ProviderConfigurationFailureClassification::OsPermissionFailure)
        );
        assert_eq!(
            outcome.configuration_state.current_detail,
            ProviderConfigurationStateDetail::SelectionMissing
        );
    }

    #[test]
    fn provider_configuration_state_reports_fingerprint_mismatch_as_error() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ProviderSupportStore::from_record_path(
            temp_dir.path().join("provider_support_record.json"),
        );
        let secure_store = ProviderSecureStore::with_backend(InMemorySecureStoreBackend::default());

        support_store
            .persist_record(&ProviderSupportStateRecord {
                schema_version: PROVIDER_SUPPORT_STATE_SCHEMA_VERSION,
                selected_provider: SupportedProvider::Google,
                secure_store_lookup_reference: secure_store.lookup_reference(SupportedProvider::Google),
                provider_secret_fingerprint_sha256: "wrong-fingerprint".to_string(),
                configuration_status: ProviderConfigurationStatus::Present,
                last_configuration_outcome: ProviderConfigurationOutcomeMarker::ConfiguredLocally,
                last_updated_timestamp_epoch_seconds: 100,
            })
            .expect("record should persist");
        secure_store
            .commit_secret(SupportedProvider::Google, "google-secret-001")
            .expect("secret should commit");

        let state = provider_configuration_state_with_components(&secure_store, &support_store);

        assert_eq!(state.current_status, ProviderConfigurationStatus::Error);
        assert_eq!(
            state.current_detail,
            ProviderConfigurationStateDetail::SecretFingerprintMismatch
        );
        assert!(!state.configuration_present);
    }

    #[test]
    fn provider_configuration_submission_restores_previous_state_on_support_persist_failure() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let record_path = temp_dir.path().join("provider_support_record.json");
        let support_store = ProviderSupportStore::from_record_path(record_path.clone());
        let secure_store = ProviderSecureStore::with_backend(InMemorySecureStoreBackend::default());

        secure_store
            .commit_secret(SupportedProvider::OpenAi, "sk-openai-previous")
            .expect("previous secret should commit");
        support_store
            .persist_record(&ProviderSupportStateRecord {
                schema_version: PROVIDER_SUPPORT_STATE_SCHEMA_VERSION,
                selected_provider: SupportedProvider::OpenAi,
                secure_store_lookup_reference: secure_store.lookup_reference(SupportedProvider::OpenAi),
                provider_secret_fingerprint_sha256: hex_sha256("sk-openai-previous"),
                configuration_status: ProviderConfigurationStatus::Present,
                last_configuration_outcome: ProviderConfigurationOutcomeMarker::ConfiguredLocally,
                last_updated_timestamp_epoch_seconds: 90,
            })
            .expect("previous record should persist");

        std::fs::write(&record_path, "this is not valid json").expect("invalid json should write");
        let outcome = submit_provider_configuration_with_components(
            &NormalizedProviderConfigurationRequest {
                selected_provider: SupportedProvider::OpenAi,
                api_key: "sk-openai-new".to_string(),
            },
            &secure_store,
            &support_store,
            100,
        );

        assert_eq!(
            outcome.decision_type,
            ProviderConfigurationSubmissionDecisionType::Failure
        );
        assert_eq!(
            secure_store.read_secret(SupportedProvider::OpenAi).unwrap(),
            Some("sk-openai-previous".to_string())
        );
    }

    #[test]
    fn provider_configuration_submission_rejects_unsupported_provider_values_at_the_shell_boundary() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ProviderSupportStore::from_record_path(
            temp_dir.path().join("provider_support_record.json"),
        );
        let secure_store = ProviderSecureStore::with_backend(InMemorySecureStoreBackend::default());

        let outcome = submit_provider_configuration_request_with_components(
            &ProviderConfigurationSubmissionRequest {
            selected_provider: "anthropic".to_string(),
            api_key: "key-001".to_string(),
            },
            &secure_store,
            &support_store,
            100,
        );

        assert_eq!(
            outcome.decision_type,
            ProviderConfigurationSubmissionDecisionType::Failure
        );
        assert_eq!(
            outcome.failure_reason,
            Some(ProviderConfigurationFailureClassification::UnsupportedProvider)
        );
        assert!(support_store.load_record().unwrap().is_none());
        assert_eq!(secure_store.read_secret(SupportedProvider::OpenAi).unwrap(), None);
        assert_eq!(secure_store.read_secret(SupportedProvider::Google).unwrap(), None);
    }

    #[test]
    fn provider_configuration_submission_clears_the_previous_provider_secret_on_switch() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ProviderSupportStore::from_record_path(
            temp_dir.path().join("provider_support_record.json"),
        );
        let secure_store = ProviderSecureStore::with_backend(InMemorySecureStoreBackend::default());

        let openai_outcome = submit_provider_configuration_with_components(
            &NormalizedProviderConfigurationRequest {
                selected_provider: SupportedProvider::OpenAi,
                api_key: "sk-openai-001".to_string(),
            },
            &secure_store,
            &support_store,
            100,
        );
        assert_eq!(
            openai_outcome.decision_type,
            ProviderConfigurationSubmissionDecisionType::Success
        );
        assert_eq!(
            secure_store.read_secret(SupportedProvider::OpenAi).unwrap(),
            Some("sk-openai-001".to_string())
        );

        let google_outcome = submit_provider_configuration_with_components(
            &NormalizedProviderConfigurationRequest {
                selected_provider: SupportedProvider::Google,
                api_key: "google-secret-001".to_string(),
            },
            &secure_store,
            &support_store,
            110,
        );
        assert_eq!(
            google_outcome.decision_type,
            ProviderConfigurationSubmissionDecisionType::Success
        );
        assert_eq!(secure_store.read_secret(SupportedProvider::OpenAi).unwrap(), None);
        assert_eq!(
            secure_store.read_secret(SupportedProvider::Google).unwrap(),
            Some("google-secret-001".to_string())
        );
        assert_eq!(
            support_store.load_record().unwrap().unwrap().selected_provider,
            SupportedProvider::Google
        );
    }
}