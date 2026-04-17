use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::activation_secure_store::{ActivationSecureStore, ActivationSecureStoreBackend};
use crate::activation_storage_boundary::ActivationValidationStatus;
use crate::activation_support_store::{
    ActivationSupportRecord, ActivationSupportStore, ActivationSupportValidityMarker,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalActivationValidationStage {
    Materialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalActivationValidationDetail {
    ActivatedValid,
    SupportRecordMissing,
    SecureTokenMissing,
    DeviceMismatch,
    TokenExpired,
    TokenFingerprintMismatch,
    TokenLineageMismatch,
    ClaimMismatch,
    MalformedToken,
    Revoked,
    StateAccessFailure,
}

impl LocalActivationValidationDetail {
    pub fn description(self) -> &'static str {
        match self {
            Self::ActivatedValid => "local activation state validates for the current device",
            Self::SupportRecordMissing => "no bounded local activation support record exists",
            Self::SecureTokenMissing => "no device-bound activation token exists in secure storage",
            Self::DeviceMismatch => "the committed token does not bind to the current device",
            Self::TokenExpired => "the committed token is expired under local clock time",
            Self::TokenFingerprintMismatch => {
                "the secure-store token does not match the committed support-record fingerprint"
            }
            Self::TokenLineageMismatch => {
                "the secure-store token does not match the committed signing lineage markers"
            }
            Self::ClaimMismatch => {
                "the secure-store token claims do not match the committed activation support record"
            }
            Self::MalformedToken => "the secure-store token is malformed for local validation",
            Self::Revoked => "the committed activation state is marked revoked",
            Self::StateAccessFailure => "local activation state could not be accessed safely",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalActivationValidationSnapshot {
    pub stage: LocalActivationValidationStage,
    pub validation_capability_materialized: bool,
    pub current_device_hash: String,
    pub local_support_record_present: bool,
    pub local_secure_token_present: bool,
    pub validation_status: ActivationValidationStatus,
    pub validation_detail: LocalActivationValidationDetail,
    pub token_fingerprint_verified: bool,
    pub device_binding_verified: bool,
    pub expiration_check_passed: bool,
    pub signing_lineage_verified: bool,
    pub may_progress_past_activation_gate: bool,
}

impl LocalActivationValidationSnapshot {
    pub fn summary(&self) -> String {
        format!(
            "Activation local-validation surface is materialized. Stage is {:?}; validation capability materialized is {}; current device hash is {}; local support record present is {}; local secure token present is {}; validation status is {:?}; validation detail is {:?}; token fingerprint verified is {}; device binding verified is {}; expiration check passed is {}; signing lineage verified is {}; may progress past activation gate is {}.",
            self.stage,
            self.validation_capability_materialized,
            self.current_device_hash,
            self.local_support_record_present,
            self.local_secure_token_present,
            self.validation_status,
            self.validation_detail,
            self.token_fingerprint_verified,
            self.device_binding_verified,
            self.expiration_check_passed,
            self.signing_lineage_verified,
            self.may_progress_past_activation_gate
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LocalActivationApprovalPayload {
    pub license_id: String,
    pub activation_id: String,
    pub activation_slot_index: u32,
    pub plan_tier: String,
    pub device_hash: String,
    pub device_bound_token: String,
    pub issued_timestamp_epoch_seconds: u64,
    pub expiration_timestamp_epoch_seconds: Option<u64>,
    pub key_identifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalActivationCommitResult {
    pub validation_status: ActivationValidationStatus,
    pub secure_store_lookup_reference: String,
    pub support_record_path: String,
    pub token_fingerprint_sha256: String,
    pub activation_committed: bool,
}

impl LocalActivationCommitResult {
    pub fn summary(&self) -> String {
        format!(
            "Local activation commit completed. Validation status is {:?}; secure-store lookup reference is {}; support record path is {}; token fingerprint is {}; activation committed is {}.",
            self.validation_status,
            self.secure_store_lookup_reference,
            self.support_record_path,
            self.token_fingerprint_sha256,
            self.activation_committed
        )
    }
}

#[derive(Debug, Deserialize)]
struct ParsedTokenHeader {
    alg: String,
    kid: String,
    typ: String,
}

#[derive(Debug, Deserialize)]
struct ParsedTokenClaims {
    sub: String,
    license_id: String,
    plan_tier: String,
    device_hash: String,
    activation_slot: u32,
    iat: u64,
    exp: Option<u64>,
    iss: String,
    jti: String,
}

#[derive(Debug)]
struct ParsedDeviceBoundToken {
    header: ParsedTokenHeader,
    claims: ParsedTokenClaims,
}

pub fn activation_local_validation_surface() -> LocalActivationValidationSnapshot {
    let current_device_hash = current_device_binding_hash().unwrap_or_else(|_| "unavailable".to_string());
    let secure_store = ActivationSecureStore::os_native();
    let support_store = match ActivationSupportStore::default_store() {
        Ok(store) => store,
        Err(_) => {
            return LocalActivationValidationSnapshot {
                stage: LocalActivationValidationStage::Materialized,
                validation_capability_materialized: true,
                current_device_hash,
                local_support_record_present: false,
                local_secure_token_present: false,
                validation_status: ActivationValidationStatus::ActivationError,
                validation_detail: LocalActivationValidationDetail::StateAccessFailure,
                token_fingerprint_verified: false,
                device_binding_verified: false,
                expiration_check_passed: false,
                signing_lineage_verified: false,
                may_progress_past_activation_gate: false,
            };
        }
    };

    match validate_committed_activation(
        &secure_store,
        &support_store,
        &current_device_hash,
        current_epoch_seconds(),
    ) {
        Ok(snapshot) => snapshot,
        Err(_) => LocalActivationValidationSnapshot {
            stage: LocalActivationValidationStage::Materialized,
            validation_capability_materialized: true,
            current_device_hash,
            local_support_record_present: false,
            local_secure_token_present: false,
            validation_status: ActivationValidationStatus::ActivationError,
            validation_detail: LocalActivationValidationDetail::StateAccessFailure,
            token_fingerprint_verified: false,
            device_binding_verified: false,
            expiration_check_passed: false,
            signing_lineage_verified: false,
            may_progress_past_activation_gate: false,
        },
    }
}

pub fn current_device_binding_hash() -> Result<String, String> {
    let host_name = hostname::get().map_err(|error| error.to_string())?;
    let host_name = host_name.to_string_lossy();
    let basis = format!("{}:{}:{}", std::env::consts::OS, std::env::consts::ARCH, host_name);

    Ok(hex_sha256(&basis))
}

pub fn commit_approved_activation<B: ActivationSecureStoreBackend>(
    approval: &LocalActivationApprovalPayload,
    secure_store: &ActivationSecureStore<B>,
    support_store: &ActivationSupportStore,
    current_device_hash: &str,
    current_timestamp_epoch_seconds: u64,
) -> Result<LocalActivationCommitResult, String> {
    validate_approved_activation_payload(approval, current_device_hash)?;

    let token_fingerprint_sha256 = hex_sha256(&approval.device_bound_token);
    let support_record = ActivationSupportRecord {
        schema_version: 1,
        license_reference: approval.license_id.clone(),
        activation_reference: approval.activation_id.clone(),
        activation_slot_index: approval.activation_slot_index,
        plan_tier: approval.plan_tier.clone(),
        device_hash: approval.device_hash.clone(),
        key_identifier: approval.key_identifier.clone(),
        token_lineage_marker: parse_device_bound_token(&approval.device_bound_token)?.claims.jti,
        token_fingerprint_sha256: token_fingerprint_sha256.clone(),
        validity_marker: ActivationSupportValidityMarker::Active,
        secure_store_lookup_reference: secure_store.lookup_reference().to_string(),
        issued_timestamp_epoch_seconds: approval.issued_timestamp_epoch_seconds,
        expiration_timestamp_epoch_seconds: approval.expiration_timestamp_epoch_seconds,
        last_validation_status: ActivationValidationStatus::ActivatedValid,
        last_validation_timestamp_epoch_seconds: current_timestamp_epoch_seconds,
    };

    secure_store.commit_token(&approval.device_bound_token)?;

    if let Err(error) = support_store.persist_record(&support_record) {
        let _ = secure_store.clear_token();
        return Err(error);
    }

    let validation = match validate_committed_activation(
        secure_store,
        support_store,
        current_device_hash,
        current_timestamp_epoch_seconds,
    ) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            let _ = support_store.clear_record();
            let _ = secure_store.clear_token();
            return Err(error);
        }
    };

    if validation.validation_status != ActivationValidationStatus::ActivatedValid {
        let _ = support_store.clear_record();
        let _ = secure_store.clear_token();
        return Err("local activation commit did not produce a valid offline activation state".to_string());
    }

    Ok(LocalActivationCommitResult {
        validation_status: validation.validation_status,
        secure_store_lookup_reference: secure_store.lookup_reference().to_string(),
        support_record_path: support_store.record_path().display().to_string(),
        token_fingerprint_sha256,
        activation_committed: true,
    })
}

pub fn validate_approved_activation_payload(
    approval: &LocalActivationApprovalPayload,
    current_device_hash: &str,
) -> Result<(), String> {
    let parsed_token = parse_device_bound_token(&approval.device_bound_token)?;
    validate_token_for_commit(parsed_token, approval, current_device_hash)
}

pub fn validate_committed_activation<B: ActivationSecureStoreBackend>(
    secure_store: &ActivationSecureStore<B>,
    support_store: &ActivationSupportStore,
    current_device_hash: &str,
    current_timestamp_epoch_seconds: u64,
) -> Result<LocalActivationValidationSnapshot, String> {
    let support_record = support_store.load_record()?;
    let secure_token = secure_store.read_token()?;

    let Some(mut support_record) = support_record else {
        return Ok(LocalActivationValidationSnapshot {
            stage: LocalActivationValidationStage::Materialized,
            validation_capability_materialized: true,
            current_device_hash: current_device_hash.to_string(),
            local_support_record_present: false,
            local_secure_token_present: secure_token.is_some(),
            validation_status: ActivationValidationStatus::ActivationRequired,
            validation_detail: LocalActivationValidationDetail::SupportRecordMissing,
            token_fingerprint_verified: false,
            device_binding_verified: false,
            expiration_check_passed: false,
            signing_lineage_verified: false,
            may_progress_past_activation_gate: false,
        });
    };

    if matches!(support_record.validity_marker, ActivationSupportValidityMarker::Revoked)
        || matches!(
            support_record.last_validation_status,
            ActivationValidationStatus::ActivationRevoked
        )
    {
        support_record.last_validation_status = ActivationValidationStatus::ActivationRevoked;
        support_record.last_validation_timestamp_epoch_seconds = current_timestamp_epoch_seconds;
        support_store.persist_record(&support_record)?;

        return Ok(LocalActivationValidationSnapshot {
            stage: LocalActivationValidationStage::Materialized,
            validation_capability_materialized: true,
            current_device_hash: current_device_hash.to_string(),
            local_support_record_present: true,
            local_secure_token_present: secure_token.is_some(),
            validation_status: ActivationValidationStatus::ActivationRevoked,
            validation_detail: LocalActivationValidationDetail::Revoked,
            token_fingerprint_verified: false,
            device_binding_verified: false,
            expiration_check_passed: false,
            signing_lineage_verified: false,
            may_progress_past_activation_gate: false,
        });
    }

    let Some(secure_token) = secure_token else {
        support_record.last_validation_status = ActivationValidationStatus::ActivationRequired;
        support_record.last_validation_timestamp_epoch_seconds = current_timestamp_epoch_seconds;
        support_store.persist_record(&support_record)?;

        return Ok(LocalActivationValidationSnapshot {
            stage: LocalActivationValidationStage::Materialized,
            validation_capability_materialized: true,
            current_device_hash: current_device_hash.to_string(),
            local_support_record_present: true,
            local_secure_token_present: false,
            validation_status: ActivationValidationStatus::ActivationRequired,
            validation_detail: LocalActivationValidationDetail::SecureTokenMissing,
            token_fingerprint_verified: false,
            device_binding_verified: false,
            expiration_check_passed: false,
            signing_lineage_verified: false,
            may_progress_past_activation_gate: false,
        });
    };

    let parsed_token = match parse_device_bound_token(&secure_token) {
        Ok(parsed_token) => parsed_token,
        Err(_) => {
            support_record.last_validation_status = ActivationValidationStatus::ActivationError;
            support_record.last_validation_timestamp_epoch_seconds = current_timestamp_epoch_seconds;
            support_store.persist_record(&support_record)?;

            return Ok(LocalActivationValidationSnapshot {
                stage: LocalActivationValidationStage::Materialized,
                validation_capability_materialized: true,
                current_device_hash: current_device_hash.to_string(),
                local_support_record_present: true,
                local_secure_token_present: true,
                validation_status: ActivationValidationStatus::ActivationError,
                validation_detail: LocalActivationValidationDetail::MalformedToken,
                token_fingerprint_verified: false,
                device_binding_verified: false,
                expiration_check_passed: false,
                signing_lineage_verified: false,
                may_progress_past_activation_gate: false,
            });
        }
    };

    let token_fingerprint_sha256 = hex_sha256(&secure_token);
    let token_fingerprint_verified = token_fingerprint_sha256 == support_record.token_fingerprint_sha256;
    if !token_fingerprint_verified {
        support_record.last_validation_status = ActivationValidationStatus::ActivationError;
        support_record.last_validation_timestamp_epoch_seconds = current_timestamp_epoch_seconds;
        support_store.persist_record(&support_record)?;

        return Ok(LocalActivationValidationSnapshot {
            stage: LocalActivationValidationStage::Materialized,
            validation_capability_materialized: true,
            current_device_hash: current_device_hash.to_string(),
            local_support_record_present: true,
            local_secure_token_present: true,
            validation_status: ActivationValidationStatus::ActivationError,
            validation_detail: LocalActivationValidationDetail::TokenFingerprintMismatch,
            token_fingerprint_verified,
            device_binding_verified: false,
            expiration_check_passed: false,
            signing_lineage_verified: false,
            may_progress_past_activation_gate: false,
        });
    }

    let signing_lineage_verified = parsed_token.header.kid == support_record.key_identifier
        && parsed_token.claims.jti == support_record.token_lineage_marker
        && !parsed_token.header.kid.is_empty()
        && !parsed_token.claims.jti.is_empty();
    if !signing_lineage_verified {
        support_record.last_validation_status = ActivationValidationStatus::ActivationError;
        support_record.last_validation_timestamp_epoch_seconds = current_timestamp_epoch_seconds;
        support_store.persist_record(&support_record)?;

        return Ok(LocalActivationValidationSnapshot {
            stage: LocalActivationValidationStage::Materialized,
            validation_capability_materialized: true,
            current_device_hash: current_device_hash.to_string(),
            local_support_record_present: true,
            local_secure_token_present: true,
            validation_status: ActivationValidationStatus::ActivationError,
            validation_detail: LocalActivationValidationDetail::TokenLineageMismatch,
            token_fingerprint_verified,
            device_binding_verified: false,
            expiration_check_passed: false,
            signing_lineage_verified,
            may_progress_past_activation_gate: false,
        });
    }

    let support_record_claims_match = parsed_token.claims.license_id == support_record.license_reference
        && parsed_token.claims.plan_tier == support_record.plan_tier
        && parsed_token.claims.activation_slot == support_record.activation_slot_index
        && parsed_token.claims.iat == support_record.issued_timestamp_epoch_seconds
        && parsed_token.claims.exp == support_record.expiration_timestamp_epoch_seconds
        && parsed_token.claims.device_hash == support_record.device_hash;
    if !support_record_claims_match {
        support_record.last_validation_status = ActivationValidationStatus::ActivationError;
        support_record.last_validation_timestamp_epoch_seconds = current_timestamp_epoch_seconds;
        support_store.persist_record(&support_record)?;

        return Ok(LocalActivationValidationSnapshot {
            stage: LocalActivationValidationStage::Materialized,
            validation_capability_materialized: true,
            current_device_hash: current_device_hash.to_string(),
            local_support_record_present: true,
            local_secure_token_present: true,
            validation_status: ActivationValidationStatus::ActivationError,
            validation_detail: LocalActivationValidationDetail::ClaimMismatch,
            token_fingerprint_verified,
            device_binding_verified: false,
            expiration_check_passed: false,
            signing_lineage_verified,
            may_progress_past_activation_gate: false,
        });
    }

    let device_binding_verified = parsed_token.claims.device_hash == current_device_hash;
    if !device_binding_verified {
        support_record.last_validation_status = ActivationValidationStatus::ActivationRequired;
        support_record.last_validation_timestamp_epoch_seconds = current_timestamp_epoch_seconds;
        support_store.persist_record(&support_record)?;

        return Ok(LocalActivationValidationSnapshot {
            stage: LocalActivationValidationStage::Materialized,
            validation_capability_materialized: true,
            current_device_hash: current_device_hash.to_string(),
            local_support_record_present: true,
            local_secure_token_present: true,
            validation_status: ActivationValidationStatus::ActivationRequired,
            validation_detail: LocalActivationValidationDetail::DeviceMismatch,
            token_fingerprint_verified,
            device_binding_verified,
            expiration_check_passed: false,
            signing_lineage_verified,
            may_progress_past_activation_gate: false,
        });
    }

    let expiration_check_passed = parsed_token
        .claims
        .exp
        .map(|expiration| current_timestamp_epoch_seconds <= expiration)
        .unwrap_or(true);
    if !expiration_check_passed {
        support_record.last_validation_status = ActivationValidationStatus::ActivationRequired;
        support_record.last_validation_timestamp_epoch_seconds = current_timestamp_epoch_seconds;
        support_store.persist_record(&support_record)?;

        return Ok(LocalActivationValidationSnapshot {
            stage: LocalActivationValidationStage::Materialized,
            validation_capability_materialized: true,
            current_device_hash: current_device_hash.to_string(),
            local_support_record_present: true,
            local_secure_token_present: true,
            validation_status: ActivationValidationStatus::ActivationRequired,
            validation_detail: LocalActivationValidationDetail::TokenExpired,
            token_fingerprint_verified,
            device_binding_verified,
            expiration_check_passed,
            signing_lineage_verified,
            may_progress_past_activation_gate: false,
        });
    }

    support_record.last_validation_status = ActivationValidationStatus::ActivatedValid;
    support_record.last_validation_timestamp_epoch_seconds = current_timestamp_epoch_seconds;
    support_store.persist_record(&support_record)?;

    Ok(LocalActivationValidationSnapshot {
        stage: LocalActivationValidationStage::Materialized,
        validation_capability_materialized: true,
        current_device_hash: current_device_hash.to_string(),
        local_support_record_present: true,
        local_secure_token_present: true,
        validation_status: ActivationValidationStatus::ActivatedValid,
        validation_detail: LocalActivationValidationDetail::ActivatedValid,
        token_fingerprint_verified,
        device_binding_verified,
        expiration_check_passed,
        signing_lineage_verified,
        may_progress_past_activation_gate: true,
    })
}

fn validate_token_for_commit(
    parsed_token: ParsedDeviceBoundToken,
    approval: &LocalActivationApprovalPayload,
    current_device_hash: &str,
) -> Result<(), String> {
    if parsed_token.header.typ != "JWT" || parsed_token.header.alg.is_empty() {
        return Err("approved activation token does not present a usable JWT header".to_string());
    }

    if parsed_token.header.kid != approval.key_identifier {
        return Err("approved activation token key identifier does not match approval payload".to_string());
    }

    if parsed_token.claims.sub.is_empty() || parsed_token.claims.iss.is_empty() {
        return Err("approved activation token is missing required identity lineage claims".to_string());
    }

    if parsed_token.claims.license_id != approval.license_id
        || parsed_token.claims.plan_tier != approval.plan_tier
        || parsed_token.claims.device_hash != approval.device_hash
        || parsed_token.claims.activation_slot != approval.activation_slot_index
        || parsed_token.claims.iat != approval.issued_timestamp_epoch_seconds
        || parsed_token.claims.exp != approval.expiration_timestamp_epoch_seconds
    {
        return Err("approved activation token claims do not match the approval payload".to_string());
    }

    if parsed_token.claims.device_hash != current_device_hash {
        return Err("approved activation payload does not match the current device binding".to_string());
    }

    if parsed_token.claims.jti.is_empty() {
        return Err("approved activation token is missing signing lineage".to_string());
    }

    Ok(())
}

fn parse_device_bound_token(token: &str) -> Result<ParsedDeviceBoundToken, String> {
    let mut segments = token.split('.');
    let Some(header_segment) = segments.next() else {
        return Err("token is missing a header segment".to_string());
    };
    let Some(claims_segment) = segments.next() else {
        return Err("token is missing a claims segment".to_string());
    };
    let Some(signature_segment) = segments.next() else {
        return Err("token is missing a signature segment".to_string());
    };
    if segments.next().is_some() || signature_segment.is_empty() {
        return Err("token has an invalid segment count".to_string());
    }

    let header_bytes = URL_SAFE_NO_PAD
        .decode(header_segment)
        .map_err(|error| error.to_string())?;
    let claims_bytes = URL_SAFE_NO_PAD
        .decode(claims_segment)
        .map_err(|error| error.to_string())?;
    let header: ParsedTokenHeader = serde_json::from_slice(&header_bytes).map_err(|error| error.to_string())?;
    let claims: ParsedTokenClaims = serde_json::from_slice(&claims_bytes).map_err(|error| error.to_string())?;

    Ok(ParsedDeviceBoundToken { header, claims })
}

fn current_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after the unix epoch")
        .as_secs()
}

fn hex_sha256(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::{Arc, Mutex};

    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    use serde_json::json;
    use tempfile::tempdir;

    use super::*;
    use crate::activation_secure_store::ActivationSecureStoreBackend;

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

        fn delete_secret(&self, _service_name: &str, _lookup_reference: &str) -> Result<(), String> {
            *self.secret.lock().expect("lock poisoned") = None;
            Ok(())
        }
    }

    #[test]
    fn activation_local_validation_surface_reports_materialized_validation_dependencies() {
        let snapshot = activation_local_validation_surface();

        assert!(snapshot.validation_capability_materialized);
        assert_eq!(
            crate::activation_secure_store::activation_secure_store_surface()
                .secure_store_commit_implemented,
            true
        );
        assert_eq!(
            crate::activation_support_store::activation_support_store_surface()
                .persistence_implemented,
            true
        );
    }

    #[test]
    fn commit_and_recover_activation_state_without_writing_raw_token_to_support_record() {
        let current_device_hash = "device-hash-001";
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );
        let payload = LocalActivationApprovalPayload {
            license_id: "license-001".to_string(),
            activation_id: "activation-001".to_string(),
            activation_slot_index: 1,
            plan_tier: "annual".to_string(),
            device_hash: current_device_hash.to_string(),
            device_bound_token: token_for(
                "license-001",
                "annual",
                current_device_hash,
                1,
                100,
                Some(200),
                "kid-001",
                "jti-001",
            ),
            issued_timestamp_epoch_seconds: 100,
            expiration_timestamp_epoch_seconds: Some(200),
            key_identifier: "kid-001".to_string(),
        };

        let commit_result = commit_approved_activation(
            &payload,
            &secure_store,
            &support_store,
            current_device_hash,
            100,
        )
        .expect("commit should succeed");

        assert!(commit_result.activation_committed);
        assert_eq!(
            secure_store.read_token().expect("read should succeed"),
            Some(payload.device_bound_token.clone())
        );

        let validation = validate_committed_activation(
            &secure_store,
            &support_store,
            current_device_hash,
            150,
        )
        .expect("validation should succeed");

        assert_eq!(validation.validation_status, ActivationValidationStatus::ActivatedValid);
        assert_eq!(
            validation.validation_detail,
            LocalActivationValidationDetail::ActivatedValid
        );

        let support_record_content = fs::read_to_string(support_store.record_path())
            .expect("support record should exist");
        assert!(!support_record_content.contains(&payload.device_bound_token));
    }

    #[test]
    fn validated_approval_payload_rejects_token_that_does_not_match_the_current_device() {
        let payload = LocalActivationApprovalPayload {
            license_id: "license-001".to_string(),
            activation_id: "activation-001".to_string(),
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
        };

        let validation_error = validate_approved_activation_payload(&payload, "device-hash-b")
            .expect_err("validation should fail when the approval payload targets a different device");

        assert!(validation_error.contains("current device binding"));
    }

    #[test]
    fn validation_rejects_token_on_different_device() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );
        let payload = LocalActivationApprovalPayload {
            license_id: "license-001".to_string(),
            activation_id: "activation-001".to_string(),
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
        };

        commit_approved_activation(
            &payload,
            &secure_store,
            &support_store,
            "device-hash-a",
            100,
        )
        .expect("commit should succeed");

        let validation = validate_committed_activation(
            &secure_store,
            &support_store,
            "device-hash-b",
            150,
        )
        .expect("validation should succeed");

        assert_eq!(validation.validation_status, ActivationValidationStatus::ActivationRequired);
        assert_eq!(
            validation.validation_detail,
            LocalActivationValidationDetail::DeviceMismatch
        );
        assert!(!validation.may_progress_past_activation_gate);
    }

    #[test]
    fn commit_rolls_back_secure_store_when_support_record_write_fails() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let invalid_record_path = temp_dir.path().join("activation-support-record");
        fs::create_dir_all(&invalid_record_path).expect("directory should exist");

        let support_store = ActivationSupportStore::from_record_path(invalid_record_path);
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );
        let payload = LocalActivationApprovalPayload {
            license_id: "license-001".to_string(),
            activation_id: "activation-001".to_string(),
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
        };

        let commit_result = commit_approved_activation(
            &payload,
            &secure_store,
            &support_store,
            "device-hash-a",
            100,
        );

        assert!(commit_result.is_err());
        assert_eq!(secure_store.read_token().expect("read should succeed"), None);
    }

    #[test]
    fn validation_maps_revoked_state_without_generic_error_collapse() {
        let temp_dir = tempdir().expect("temp dir should exist");
        let support_store = ActivationSupportStore::from_record_path(
            temp_dir.path().join("activation_support_record.json"),
        );
        let secure_store = ActivationSecureStore::with_backend(
            InMemorySecureStoreBackend::default(),
            "device-bound-activation-token",
        );
        let support_record = ActivationSupportRecord {
            schema_version: 1,
            license_reference: "license-001".to_string(),
            activation_reference: "activation-001".to_string(),
            activation_slot_index: 1,
            plan_tier: "annual".to_string(),
            device_hash: "device-hash-a".to_string(),
            key_identifier: "kid-001".to_string(),
            token_lineage_marker: "jti-001".to_string(),
            token_fingerprint_sha256: "fingerprint".to_string(),
            validity_marker: ActivationSupportValidityMarker::Revoked,
            secure_store_lookup_reference: "device-bound-activation-token".to_string(),
            issued_timestamp_epoch_seconds: 100,
            expiration_timestamp_epoch_seconds: Some(200),
            last_validation_status: ActivationValidationStatus::ActivationRevoked,
            last_validation_timestamp_epoch_seconds: 100,
        };
        support_store
            .persist_record(&support_record)
            .expect("persist should succeed");

        let validation = validate_committed_activation(
            &secure_store,
            &support_store,
            "device-hash-a",
            150,
        )
        .expect("validation should succeed");

        assert_eq!(validation.validation_status, ActivationValidationStatus::ActivationRevoked);
        assert_eq!(validation.validation_detail, LocalActivationValidationDetail::Revoked);
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
            "iss": "miro-fish-commercial-backend",
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