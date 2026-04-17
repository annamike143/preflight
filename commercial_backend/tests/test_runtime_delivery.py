from __future__ import annotations

from datetime import UTC, datetime

from commercial_backend.models import (
    PaymentEventInput,
    PlanTier,
    RuntimeDeliveryDecisionType,
    RuntimeDeliveryDenialReason,
    RuntimeDeliveryManifestRequest,
    RuntimeDeliveryTechnicalFailureReason,
)
from commercial_backend.service import CommercialActivationService, CommercialBackendConfig
from commercial_backend.store import CommercialLedgerStore


def _service() -> tuple[CommercialLedgerStore, CommercialActivationService]:
    store = CommercialLedgerStore.in_memory()
    service = CommercialActivationService(
        store=store,
        config=CommercialBackendConfig(
            signing_key="repo-local-commercial-backend-signing-key",
            key_identifier="repo-local-kid-001",
        ),
    )
    return store, service


def test_runtime_delivery_manifest_resolution_denies_invalid_license() -> None:
    store, service = _service()

    try:
        decision = service.resolve_runtime_delivery_manifest(
            request=RuntimeDeliveryManifestRequest(
                license_id="missing-license",
                requested_timestamp=datetime.fromtimestamp(100, tz=UTC),
            ),
            manifest_path="runtime_delivery_manifest.json",
        )
    finally:
        store.close()

    assert decision.decision_type == RuntimeDeliveryDecisionType.DENIED
    assert decision.denial_reason == RuntimeDeliveryDenialReason.INVALID_LICENSE
    assert decision.authorization is None


def test_runtime_delivery_manifest_resolution_reports_missing_manifest_configuration() -> None:
    store, service = _service()

    try:
        _, license_record = service.apply_payment_event(
            PaymentEventInput(
                payment_provider_identifier="stripe",
                provider_event_identifier="evt-runtime-config-001",
                event_type="payment_succeeded",
                purchaser_email="owner@example.com",
                plan_tier=PlanTier.ANNUAL,
                receipt_timestamp=datetime.fromtimestamp(100, tz=UTC),
            )
        )
        decision = service.resolve_runtime_delivery_manifest(
            request=RuntimeDeliveryManifestRequest(
                license_id=license_record.license_id,
                requested_timestamp=datetime.fromtimestamp(100, tz=UTC),
            ),
            manifest_path=None,
        )
    finally:
        store.close()

    assert decision.decision_type == RuntimeDeliveryDecisionType.TECHNICAL_FAILURE
    assert (
        decision.technical_failure_reason
        == RuntimeDeliveryTechnicalFailureReason.MANIFEST_NOT_CONFIGURED
    )
    assert decision.authorization is None
