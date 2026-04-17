from __future__ import annotations

from datetime import UTC, datetime

from commercial_backend.models import (
    ActivationDecisionType,
    ActivationDenialReason,
    ActivationRequest,
    ActivationTechnicalFailureReason,
    PaymentEventInput,
    PlanTier,
    TokenStatus,
)
from commercial_backend.service import CommercialActivationService, CommercialBackendConfig
from commercial_backend.store import CommercialLedgerStore


def _service(
    signing_key: str = "super-secret-key",
) -> tuple[CommercialLedgerStore, CommercialActivationService]:
    store = CommercialLedgerStore.in_memory()
    service = CommercialActivationService(
        store=store,
        config=CommercialBackendConfig(signing_key=signing_key, key_identifier="kid-001"),
    )
    return store, service


def _payment_event(
    provider_event_identifier: str,
    purchaser_email: str = "owner@example.com",
    receipt_timestamp: datetime | None = None,
) -> PaymentEventInput:
    return PaymentEventInput(
        payment_provider_identifier="stripe",
        provider_event_identifier=provider_event_identifier,
        event_type="payment_succeeded",
        purchaser_email=purchaser_email,
        plan_tier=PlanTier.ANNUAL,
        receipt_timestamp=receipt_timestamp or datetime(2026, 4, 15, 12, 0, tzinfo=UTC),
        payload_reference="evt_ref_001",
    )


def test_payment_event_translation_is_idempotent_and_creates_one_license() -> None:
    store, service = _service()
    event = _payment_event("evt_001")

    first_payment, first_license = service.apply_payment_event(event)
    second_payment, second_license = service.apply_payment_event(event)

    assert first_payment.internal_event_id == second_payment.internal_event_id
    assert first_license.license_id == second_license.license_id
    assert store.count_rows("payment_events") == 1
    assert store.count_rows("licenses") == 1


def test_activation_approval_reuses_same_device_without_consuming_extra_slot() -> None:
    store, service = _service()
    _, license_record = service.apply_payment_event(_payment_event("evt_002"))
    request = ActivationRequest(
        license_id=license_record.license_id,
        device_hash="device-hash-a",
        client_platform_identity="windows",
        requested_timestamp=datetime(2026, 4, 15, 13, 0, tzinfo=UTC),
    )

    first_decision = service.evaluate_activation_request(request)
    second_decision = service.evaluate_activation_request(request)

    assert first_decision.decision_type == ActivationDecisionType.APPROVED
    assert second_decision.decision_type == ActivationDecisionType.APPROVED
    assert first_decision.approval is not None
    assert second_decision.approval is not None
    assert first_decision.approval.activation_slot_index == 1
    assert second_decision.approval.activation_slot_index == 1
    assert second_decision.approval.reused_existing_slot is True
    assert len(store.list_active_activations(license_record.license_id)) == 1

    token_issuances = store.list_token_issuances_for_activation(
        first_decision.approval.activation_id
    )
    assert len(token_issuances) == 2
    assert token_issuances[0].token_status == TokenStatus.SUPERSEDED
    assert token_issuances[1].token_status == TokenStatus.ACTIVE
    assert token_issuances[0].token_fingerprint != token_issuances[1].token_fingerprint


def test_third_device_activation_is_denied_truthfully() -> None:
    store, service = _service()
    _, license_record = service.apply_payment_event(_payment_event("evt_003"))

    for device_hash in ("device-a", "device-b"):
        decision = service.evaluate_activation_request(
            ActivationRequest(
                license_id=license_record.license_id,
                device_hash=device_hash,
                client_platform_identity="windows",
                requested_timestamp=datetime(2026, 4, 15, 14, 0, tzinfo=UTC),
            )
        )
        assert decision.decision_type == ActivationDecisionType.APPROVED

    denied_decision = service.evaluate_activation_request(
        ActivationRequest(
            license_id=license_record.license_id,
            device_hash="device-c",
            client_platform_identity="windows",
            requested_timestamp=datetime(2026, 4, 15, 14, 5, tzinfo=UTC),
        )
    )

    assert denied_decision.decision_type == ActivationDecisionType.DENIED
    assert denied_decision.denial_reason == ActivationDenialReason.ACTIVATION_LIMIT_EXCEEDED
    assert len(store.list_active_activations(license_record.license_id)) == 2


def test_device_mismatch_is_denied_when_existing_activation_reference_conflicts() -> None:
    _, service = _service()
    _, license_record = service.apply_payment_event(_payment_event("evt_004"))
    first_decision = service.evaluate_activation_request(
        ActivationRequest(
            license_id=license_record.license_id,
            device_hash="device-a",
            client_platform_identity="windows",
            requested_timestamp=datetime(2026, 4, 15, 15, 0, tzinfo=UTC),
        )
    )

    assert first_decision.approval is not None

    mismatch_decision = service.evaluate_activation_request(
        ActivationRequest(
            license_id=license_record.license_id,
            device_hash="device-b",
            client_platform_identity="windows",
            requested_timestamp=datetime(2026, 4, 15, 15, 10, tzinfo=UTC),
            existing_activation_id=first_decision.approval.activation_id,
        )
    )

    assert mismatch_decision.decision_type == ActivationDecisionType.DENIED
    assert mismatch_decision.denial_reason == ActivationDenialReason.DEVICE_MISMATCH


def test_invalid_license_and_technical_failure_stay_distinct() -> None:
    _, service = _service(signing_key="")

    invalid_license_decision = service.evaluate_activation_request(
        ActivationRequest(
            license_id="missing-license",
            device_hash="device-a",
            client_platform_identity="windows",
            requested_timestamp=datetime(2026, 4, 15, 16, 0, tzinfo=UTC),
        )
    )
    assert invalid_license_decision.decision_type == ActivationDecisionType.DENIED
    assert invalid_license_decision.denial_reason == ActivationDenialReason.INVALID_LICENSE

    _, license_record = service.apply_payment_event(_payment_event("evt_005"))
    technical_failure_decision = service.evaluate_activation_request(
        ActivationRequest(
            license_id=license_record.license_id,
            device_hash="device-a",
            client_platform_identity="windows",
            requested_timestamp=datetime(2026, 4, 15, 16, 5, tzinfo=UTC),
        )
    )
    assert technical_failure_decision.decision_type == ActivationDecisionType.TECHNICAL_FAILURE
    assert (
        technical_failure_decision.technical_failure_reason
        == ActivationTechnicalFailureReason.TOKEN_ISSUANCE_FAILURE
    )


def test_token_issuance_tracks_fingerprint_without_persisting_raw_token() -> None:
    store, service = _service()
    _, license_record = service.apply_payment_event(_payment_event("evt_006"))
    decision = service.evaluate_activation_request(
        ActivationRequest(
            license_id=license_record.license_id,
            device_hash="device-a",
            client_platform_identity="windows",
            requested_timestamp=datetime(2026, 4, 15, 17, 0, tzinfo=UTC),
        )
    )

    assert decision.approval is not None
    token_issuances = store.list_token_issuances_for_activation(decision.approval.activation_id)
    assert len(token_issuances) == 1
    assert token_issuances[0].token_status == TokenStatus.ACTIVE
    assert token_issuances[0].token_fingerprint
    assert token_issuances[0].token_fingerprint != decision.approval.device_bound_token