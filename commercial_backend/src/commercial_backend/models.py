from __future__ import annotations

from dataclasses import dataclass
from datetime import UTC, datetime, timedelta
from enum import StrEnum


class LicenseStatus(StrEnum):
    ACTIVE = "active"
    EXPIRED = "expired"
    REVOKED = "revoked"
    PAYMENT_FAILED = "payment_failed"


class PlanTier(StrEnum):
    MONTHLY = "monthly"
    ANNUAL = "annual"
    LIFETIME = "lifetime"


class ActivationStatus(StrEnum):
    ACTIVE = "active"
    REVOKED = "revoked"
    REPLACED = "replaced"
    INACTIVE = "inactive"


class PaymentProcessingStatus(StrEnum):
    RECEIVED = "received"
    PROCESSED = "processed"
    FAILED = "failed"


class TokenStatus(StrEnum):
    ACTIVE = "active"
    REVOKED = "revoked"
    SUPERSEDED = "superseded"


class ActivationAuditAction(StrEnum):
    ACTIVATION_REQUESTED = "activation_requested"
    ACTIVATION_APPROVED = "activation_approved"
    ACTIVATION_DENIED = "activation_denied"
    ACTIVATION_ERROR = "activation_error"
    TOKEN_ISSUED = "token_issued"


class ActivationDecisionType(StrEnum):
    APPROVED = "approved"
    DENIED = "denied"
    TECHNICAL_FAILURE = "technical_failure"


class ActivationDenialReason(StrEnum):
    INVALID_LICENSE = "invalid_license"
    UNUSABLE_LICENSE = "unusable_license"
    ACTIVATION_LIMIT_EXCEEDED = "activation_limit_exceeded"
    DEVICE_MISMATCH = "device_mismatch"


class ActivationTechnicalFailureReason(StrEnum):
    BACKEND_FAILURE = "backend_failure"
    TOKEN_ISSUANCE_FAILURE = "token_issuance_failure"


class RuntimeDeliveryDecisionType(StrEnum):
    AUTHORIZED = "authorized"
    DENIED = "denied"
    TECHNICAL_FAILURE = "technical_failure"


class RuntimeDeliveryDenialReason(StrEnum):
    INVALID_LICENSE = "invalid_license"
    UNUSABLE_LICENSE = "unusable_license"


class RuntimeDeliveryTechnicalFailureReason(StrEnum):
    MANIFEST_NOT_CONFIGURED = "manifest_not_configured"
    MANIFEST_UNREADABLE = "manifest_unreadable"
    BACKEND_FAILURE = "backend_failure"


@dataclass(frozen=True)
class LicenseRecord:
    license_id: str
    purchaser_email: str
    plan_tier: PlanTier
    entitlement_status: LicenseStatus
    issue_timestamp: datetime
    expiration_timestamp: datetime | None
    activation_limit: int
    created_timestamp: datetime
    updated_timestamp: datetime


@dataclass(frozen=True)
class DeviceActivationRecord:
    activation_id: str
    parent_license_id: str
    activation_slot_index: int
    device_hash: str
    client_platform_identity: str
    activation_status: ActivationStatus
    first_activation_timestamp: datetime
    last_confirmed_timestamp: datetime | None
    created_timestamp: datetime
    updated_timestamp: datetime


@dataclass(frozen=True)
class PaymentEventRecord:
    internal_event_id: str
    payment_provider_identifier: str
    provider_event_identifier: str
    event_type: str
    receipt_timestamp: datetime
    processing_status: PaymentProcessingStatus
    payload_reference: str | None
    related_license_id: str | None
    purchaser_email: str
    plan_tier: PlanTier


@dataclass(frozen=True)
class ActivationAuditRecord:
    audit_id: str
    related_license_id: str | None
    related_activation_id: str | None
    action_type: ActivationAuditAction
    actor_type: str
    reason_classification: str
    metadata_reference: str | None
    recorded_timestamp: datetime


@dataclass(frozen=True)
class TokenIssuanceRecord:
    token_issuance_id: str
    parent_license_id: str
    parent_activation_id: str
    issuance_timestamp: datetime
    expiration_timestamp: datetime | None
    key_identifier: str
    token_fingerprint: str
    token_status: TokenStatus


@dataclass(frozen=True)
class PaymentEventInput:
    payment_provider_identifier: str
    provider_event_identifier: str
    event_type: str
    purchaser_email: str
    plan_tier: PlanTier
    receipt_timestamp: datetime
    license_id: str | None = None
    expiration_timestamp: datetime | None = None
    payload_reference: str | None = None
    entitlement_active: bool = True


@dataclass(frozen=True)
class ActivationRequest:
    license_id: str
    device_hash: str
    client_platform_identity: str
    requested_timestamp: datetime
    existing_activation_id: str | None = None


@dataclass(frozen=True)
class ActivationApproval:
    activation_id: str
    license_id: str
    activation_slot_index: int
    plan_tier: PlanTier
    device_hash: str
    device_bound_token: str
    issued_timestamp: datetime
    expiration_timestamp: datetime | None
    key_identifier: str
    reused_existing_slot: bool


@dataclass(frozen=True)
class ActivationDecision:
    decision_type: ActivationDecisionType
    approval: ActivationApproval | None = None
    denial_reason: ActivationDenialReason | None = None
    technical_failure_reason: ActivationTechnicalFailureReason | None = None

    @classmethod
    def approved(cls, approval: ActivationApproval) -> "ActivationDecision":
        return cls(decision_type=ActivationDecisionType.APPROVED, approval=approval)

    @classmethod
    def denied(cls, reason: ActivationDenialReason) -> "ActivationDecision":
        return cls(decision_type=ActivationDecisionType.DENIED, denial_reason=reason)

    @classmethod
    def technical_failure(
        cls, reason: ActivationTechnicalFailureReason
    ) -> "ActivationDecision":
        return cls(
            decision_type=ActivationDecisionType.TECHNICAL_FAILURE,
            technical_failure_reason=reason,
        )


@dataclass(frozen=True)
class RuntimeDeliveryManifestRequest:
    license_id: str
    requested_timestamp: datetime


@dataclass(frozen=True)
class RuntimeDeliveryManifestAuthorization:
    license_id: str
    manifest_path: str
    manifest_fingerprint_sha256: str


@dataclass(frozen=True)
class RuntimeDeliveryManifestDecision:
    decision_type: RuntimeDeliveryDecisionType
    authorization: RuntimeDeliveryManifestAuthorization | None = None
    denial_reason: RuntimeDeliveryDenialReason | None = None
    technical_failure_reason: RuntimeDeliveryTechnicalFailureReason | None = None

    @classmethod
    def authorized(
        cls, authorization: RuntimeDeliveryManifestAuthorization
    ) -> "RuntimeDeliveryManifestDecision":
        return cls(
            decision_type=RuntimeDeliveryDecisionType.AUTHORIZED,
            authorization=authorization,
        )

    @classmethod
    def denied(
        cls, reason: RuntimeDeliveryDenialReason
    ) -> "RuntimeDeliveryManifestDecision":
        return cls(decision_type=RuntimeDeliveryDecisionType.DENIED, denial_reason=reason)

    @classmethod
    def technical_failure(
        cls, reason: RuntimeDeliveryTechnicalFailureReason
    ) -> "RuntimeDeliveryManifestDecision":
        return cls(
            decision_type=RuntimeDeliveryDecisionType.TECHNICAL_FAILURE,
            technical_failure_reason=reason,
        )


def derive_expiration_timestamp(
    plan_tier: PlanTier, receipt_timestamp: datetime, explicit_expiration: datetime | None
) -> datetime | None:
    if explicit_expiration is not None:
        return explicit_expiration

    if plan_tier == PlanTier.LIFETIME:
        return None

    if plan_tier == PlanTier.MONTHLY:
        return receipt_timestamp + timedelta(days=30)

    return receipt_timestamp + timedelta(days=365)


def resolve_license_status(event: PaymentEventInput) -> LicenseStatus:
    normalized_type = event.event_type.lower()

    if normalized_type in {"license_revoked", "subscription_revoked"}:
        return LicenseStatus.REVOKED

    if normalized_type in {"payment_failed", "invoice.payment_failed"}:
        return LicenseStatus.PAYMENT_FAILED

    expiration_timestamp = derive_expiration_timestamp(
        event.plan_tier, event.receipt_timestamp, event.expiration_timestamp
    )
    if expiration_timestamp is not None and expiration_timestamp <= event.receipt_timestamp:
        return LicenseStatus.EXPIRED

    if not event.entitlement_active:
        return LicenseStatus.PAYMENT_FAILED

    return LicenseStatus.ACTIVE


def is_license_usable(license_record: LicenseRecord, as_of: datetime) -> bool:
    if license_record.entitlement_status != LicenseStatus.ACTIVE:
        return False

    if license_record.expiration_timestamp is None:
        return True

    return license_record.expiration_timestamp > as_of


def normalize_timestamp(value: datetime) -> datetime:
    if value.tzinfo is None:
        return value.replace(tzinfo=UTC)

    return value.astimezone(UTC)