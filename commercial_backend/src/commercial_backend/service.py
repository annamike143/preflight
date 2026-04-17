from __future__ import annotations

import base64
import hashlib
import hmac
import json
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from uuid import NAMESPACE_URL, uuid4, uuid5

from .models import (
    ActivationApproval,
    ActivationAuditAction,
    ActivationDecision,
    ActivationDenialReason,
    ActivationRequest,
    ActivationTechnicalFailureReason,
    DeviceActivationRecord,
    LicenseRecord,
    PaymentEventInput,
    PaymentProcessingStatus,
    RuntimeDeliveryDenialReason,
    RuntimeDeliveryManifestAuthorization,
    RuntimeDeliveryManifestDecision,
    RuntimeDeliveryManifestRequest,
    RuntimeDeliveryTechnicalFailureReason,
    derive_expiration_timestamp,
    is_license_usable,
    normalize_timestamp,
)
from .store import CommercialLedgerStore


@dataclass(frozen=True)
class CommercialBackendConfig:
    signing_key: str
    key_identifier: str
    issuer: str = "miro-fish-commercial-backend"


class CommercialActivationService:
    def __init__(self, store: CommercialLedgerStore, config: CommercialBackendConfig):
        self.store = store
        self.config = config

    def apply_payment_event(self, event: PaymentEventInput):
        license_id = event.license_id or self._derive_license_id(event)
        existing_event = self.store.get_payment_event_by_provider_event(
            event.payment_provider_identifier, event.provider_event_identifier
        )
        if existing_event is not None:
            existing_license = self.store.get_license(
                existing_event.related_license_id or license_id
            )
            return existing_event, existing_license

        payment_event = self.store.record_payment_event(event, license_id)
        license_record = self.store.upsert_license_from_payment_event(event, license_id)
        self.store.mark_payment_event_processed(
            payment_event.internal_event_id, PaymentProcessingStatus.PROCESSED
        )
        processed_event = self.store.get_payment_event_by_provider_event(
            event.payment_provider_identifier, event.provider_event_identifier
        )
        return processed_event, license_record

    def evaluate_activation_request(self, request: ActivationRequest) -> ActivationDecision:
        requested_at = normalize_timestamp(request.requested_timestamp)
        try:
            self.store.record_activation_audit(
                related_license_id=None,
                related_activation_id=request.existing_activation_id,
                action_type=ActivationAuditAction.ACTIVATION_REQUESTED,
                actor_type="commercial_backend",
                reason_classification="activation_requested",
                recorded_at=requested_at,
                metadata_reference=request.license_id,
            )

            license_record = self.store.get_license(request.license_id)
            if license_record is None:
                return self._deny(
                    request=request,
                    reason=ActivationDenialReason.INVALID_LICENSE,
                    recorded_at=requested_at,
                    related_license_id=None,
                    related_activation_id=request.existing_activation_id,
                )

            if not is_license_usable(license_record, requested_at):
                return self._deny(
                    request=request,
                    reason=ActivationDenialReason.UNUSABLE_LICENSE,
                    recorded_at=requested_at,
                    related_license_id=license_record.license_id,
                    related_activation_id=request.existing_activation_id,
                )

            if request.existing_activation_id is not None:
                existing_activation = self.store.get_activation_by_id(
                    request.existing_activation_id
                )
                if (
                    existing_activation is None
                    or existing_activation.device_hash != request.device_hash
                ):
                    return self._deny(
                        request=request,
                        reason=ActivationDenialReason.DEVICE_MISMATCH,
                        recorded_at=requested_at,
                        related_license_id=license_record.license_id,
                        related_activation_id=request.existing_activation_id,
                    )

            active_activation = self.store.get_active_activation_by_device(
                license_record.license_id, request.device_hash
            )
            reused_existing_slot = active_activation is not None
            if active_activation is None:
                active_activations = self.store.list_active_activations(license_record.license_id)
                if len(active_activations) >= license_record.activation_limit:
                    return self._deny(
                        request=request,
                        reason=ActivationDenialReason.ACTIVATION_LIMIT_EXCEEDED,
                        recorded_at=requested_at,
                        related_license_id=license_record.license_id,
                        related_activation_id=None,
                    )

                active_slot_indices = {
                    record.activation_slot_index for record in active_activations
                }
                activation_slot_index = self._lowest_available_slot(
                    license_record.activation_limit, active_slot_indices
                )
                active_activation = self.store.create_activation(
                    license_id=license_record.license_id,
                    activation_slot_index=activation_slot_index,
                    device_hash=request.device_hash,
                    client_platform_identity=request.client_platform_identity,
                    created_at=requested_at,
                )
            else:
                active_activation = self.store.touch_activation_confirmation(
                    active_activation.activation_id, requested_at
                )

            token, issued_at, expiration_timestamp = self._mint_device_bound_token(
                license_record=license_record,
                activation_record=active_activation,
                requested_at=requested_at,
            )
            token_fingerprint = hashlib.sha256(token.encode("utf-8")).hexdigest()
            self.store.supersede_active_token_issuances(active_activation.activation_id)
            self.store.record_token_issuance(
                license_id=license_record.license_id,
                activation_id=active_activation.activation_id,
                issued_at=issued_at,
                expiration_timestamp=expiration_timestamp,
                key_identifier=self.config.key_identifier,
                token_fingerprint=token_fingerprint,
            )
            self.store.record_activation_audit(
                related_license_id=license_record.license_id,
                related_activation_id=active_activation.activation_id,
                action_type=ActivationAuditAction.ACTIVATION_APPROVED,
                actor_type="commercial_backend",
                reason_classification="activation_approved",
                recorded_at=requested_at,
                metadata_reference=request.device_hash,
            )
            self.store.record_activation_audit(
                related_license_id=license_record.license_id,
                related_activation_id=active_activation.activation_id,
                action_type=ActivationAuditAction.TOKEN_ISSUED,
                actor_type="commercial_backend",
                reason_classification="token_issued",
                recorded_at=issued_at,
                metadata_reference=token_fingerprint,
            )
            approval = ActivationApproval(
                activation_id=active_activation.activation_id,
                license_id=license_record.license_id,
                activation_slot_index=active_activation.activation_slot_index,
                plan_tier=license_record.plan_tier,
                device_hash=active_activation.device_hash,
                device_bound_token=token,
                issued_timestamp=issued_at,
                expiration_timestamp=expiration_timestamp,
                key_identifier=self.config.key_identifier,
                reused_existing_slot=reused_existing_slot,
            )
            return ActivationDecision.approved(approval)
        except (OSError, ValueError, json.JSONDecodeError):
            return self._technical_failure(
                related_license_id=request.license_id,
                related_activation_id=request.existing_activation_id,
                reason=ActivationTechnicalFailureReason.TOKEN_ISSUANCE_FAILURE,
                recorded_at=requested_at,
            )
        except Exception:
            return self._technical_failure(
                related_license_id=request.license_id,
                related_activation_id=request.existing_activation_id,
                reason=ActivationTechnicalFailureReason.BACKEND_FAILURE,
                recorded_at=requested_at,
            )

    def resolve_runtime_delivery_manifest(
        self,
        request: RuntimeDeliveryManifestRequest,
        manifest_path: str | None,
    ) -> RuntimeDeliveryManifestDecision:
        requested_at = normalize_timestamp(request.requested_timestamp)

        try:
            license_record = self.store.get_license(request.license_id)
            if license_record is None:
                return RuntimeDeliveryManifestDecision.denied(
                    RuntimeDeliveryDenialReason.INVALID_LICENSE
                )

            if not is_license_usable(license_record, requested_at):
                return RuntimeDeliveryManifestDecision.denied(
                    RuntimeDeliveryDenialReason.UNUSABLE_LICENSE
                )

            normalized_manifest_path = (manifest_path or "").strip()
            if not normalized_manifest_path:
                return RuntimeDeliveryManifestDecision.technical_failure(
                    RuntimeDeliveryTechnicalFailureReason.MANIFEST_NOT_CONFIGURED
                )

            manifest_bytes = Path(normalized_manifest_path).read_bytes()
            return RuntimeDeliveryManifestDecision.authorized(
                RuntimeDeliveryManifestAuthorization(
                    license_id=license_record.license_id,
                    manifest_path=str(Path(normalized_manifest_path).resolve()),
                    manifest_fingerprint_sha256=hashlib.sha256(manifest_bytes).hexdigest(),
                )
            )
        except OSError:
            return RuntimeDeliveryManifestDecision.technical_failure(
                RuntimeDeliveryTechnicalFailureReason.MANIFEST_UNREADABLE
            )
        except Exception:
            return RuntimeDeliveryManifestDecision.technical_failure(
                RuntimeDeliveryTechnicalFailureReason.BACKEND_FAILURE
            )

    def _derive_license_id(self, event: PaymentEventInput) -> str:
        basis = f"{event.purchaser_email}:{event.plan_tier.value}"
        return str(uuid5(NAMESPACE_URL, basis))

    def _lowest_available_slot(self, activation_limit: int, active_slot_indices: set[int]) -> int:
        for slot_index in range(1, activation_limit + 1):
            if slot_index not in active_slot_indices:
                return slot_index

        raise ValueError("no activation slot is available")

    def _mint_device_bound_token(
        self,
        license_record: LicenseRecord,
        activation_record: DeviceActivationRecord,
        requested_at: datetime,
    ) -> tuple[str, datetime, datetime | None]:
        if not self.config.signing_key or not self.config.key_identifier:
            raise ValueError("signing configuration is incomplete")

        issued_at = normalize_timestamp(requested_at)
        expiration_timestamp = derive_expiration_timestamp(
            license_record.plan_tier, issued_at, license_record.expiration_timestamp
        )
        claims = {
            "sub": license_record.purchaser_email,
            "license_id": license_record.license_id,
            "plan_tier": license_record.plan_tier.value,
            "device_hash": activation_record.device_hash,
            "activation_slot": activation_record.activation_slot_index,
            "iat": int(issued_at.timestamp()),
            "iss": self.config.issuer,
            "jti": str(uuid4()),
        }
        if expiration_timestamp is not None:
            claims["exp"] = int(normalize_timestamp(expiration_timestamp).timestamp())

        header = {"alg": "HS256", "kid": self.config.key_identifier, "typ": "JWT"}
        encoded_header = self._base64url_json(header)
        encoded_claims = self._base64url_json(claims)
        signing_input = f"{encoded_header}.{encoded_claims}".encode("utf-8")
        signature = hmac.new(
            self.config.signing_key.encode("utf-8"), signing_input, hashlib.sha256
        ).digest()
        encoded_signature = base64.urlsafe_b64encode(signature).rstrip(b"=").decode("ascii")
        return (
            f"{encoded_header}.{encoded_claims}.{encoded_signature}",
            issued_at,
            expiration_timestamp,
        )

    def _base64url_json(self, payload: dict[str, object]) -> str:
        encoded = json.dumps(payload, separators=(",", ":"), sort_keys=True).encode("utf-8")
        return base64.urlsafe_b64encode(encoded).rstrip(b"=").decode("ascii")

    def _deny(
        self,
        request: ActivationRequest,
        reason: ActivationDenialReason,
        recorded_at: datetime,
        related_license_id: str | None,
        related_activation_id: str | None,
    ) -> ActivationDecision:
        self.store.record_activation_audit(
            related_license_id=related_license_id,
            related_activation_id=related_activation_id,
            action_type=ActivationAuditAction.ACTIVATION_DENIED,
            actor_type="commercial_backend",
            reason_classification=reason.value,
            recorded_at=recorded_at,
            metadata_reference=request.device_hash,
        )
        return ActivationDecision.denied(reason)

    def _technical_failure(
        self,
        related_license_id: str | None,
        related_activation_id: str | None,
        reason: ActivationTechnicalFailureReason,
        recorded_at: datetime,
    ) -> ActivationDecision:
        self.store.record_activation_audit(
            related_license_id=related_license_id,
            related_activation_id=related_activation_id,
            action_type=ActivationAuditAction.ACTIVATION_ERROR,
            actor_type="commercial_backend",
            reason_classification=reason.value,
            recorded_at=recorded_at,
            metadata_reference=None,
        )
        return ActivationDecision.technical_failure(reason)