from __future__ import annotations

import argparse
import json
import os
import sys
from datetime import UTC, datetime
from pathlib import Path

from .models import (
    ActivationRequest,
    PaymentEventInput,
    PlanTier,
    RuntimeDeliveryManifestRequest,
)
from .service import CommercialActivationService, CommercialBackendConfig
from .store import CommercialLedgerStore

RUNTIME_DELIVERY_MANIFEST_PATH_ENV_VAR = (
    "MIRO_FISH_COMMERCIAL_BACKEND_RUNTIME_DELIVERY_MANIFEST_PATH"
)


def _parse_epoch_seconds(value: object, field_name: str) -> datetime:
    if not isinstance(value, int):
        raise ValueError(f"{field_name} must be an integer epoch-seconds value")

    return datetime.fromtimestamp(value, tz=UTC)


def _parse_plan_tier(value: object) -> PlanTier:
    if not isinstance(value, str):
        raise ValueError("plan_tier must be a string")

    return PlanTier(value)


def _optional_string(value: object) -> str | None:
    if value is None:
        return None
    if not isinstance(value, str):
        raise ValueError("optional string fields must be strings when provided")
    normalized = value.strip()
    return normalized or None


def _optional_epoch_seconds(value: object, field_name: str) -> datetime | None:
    if value is None:
        return None
    return _parse_epoch_seconds(value, field_name)


def _load_stdin_json() -> dict[str, object]:
    try:
        payload = json.load(sys.stdin)
    except json.JSONDecodeError as error:
        raise ValueError(f"stdin did not contain valid JSON: {error}") from error

    if not isinstance(payload, dict):
        raise ValueError("stdin payload must be a JSON object")

    return payload


def _serialize_epoch_seconds(value: datetime | None) -> int | None:
    if value is None:
        return None

    return int(value.astimezone(UTC).timestamp())


def _open_store(database_path: str) -> CommercialLedgerStore:
    if database_path != ":memory:":
        Path(database_path).parent.mkdir(parents=True, exist_ok=True)

    return CommercialLedgerStore(database_path)


def _handle_apply_payment_event(args: argparse.Namespace) -> dict[str, object]:
    payload = _load_stdin_json()

    with _open_store(args.database_path) as store:
        service = CommercialActivationService(
            store=store,
            config=CommercialBackendConfig(
                signing_key=args.signing_key,
                key_identifier=args.key_identifier,
                issuer=args.issuer,
            ),
        )
        _, license_record = service.apply_payment_event(
            PaymentEventInput(
                payment_provider_identifier=str(payload["payment_provider_identifier"]),
                provider_event_identifier=str(payload["provider_event_identifier"]),
                event_type=str(payload["event_type"]),
                purchaser_email=str(payload["purchaser_email"]),
                plan_tier=_parse_plan_tier(payload["plan_tier"]),
                receipt_timestamp=_parse_epoch_seconds(
                    payload["receipt_timestamp_epoch_seconds"],
                    "receipt_timestamp_epoch_seconds",
                ),
                license_id=_optional_string(payload.get("license_id")),
                expiration_timestamp=_optional_epoch_seconds(
                    payload.get("expiration_timestamp_epoch_seconds"),
                    "expiration_timestamp_epoch_seconds",
                ),
                payload_reference=_optional_string(payload.get("payload_reference")),
                entitlement_active=bool(payload.get("entitlement_active", True)),
            )
        )

    return {
        "license_id": license_record.license_id,
        "plan_tier": license_record.plan_tier.value,
        "entitlement_status": license_record.entitlement_status.value,
        "activation_limit": license_record.activation_limit,
        "expiration_timestamp_epoch_seconds": _serialize_epoch_seconds(
            license_record.expiration_timestamp
        ),
    }


def _handle_evaluate_activation_request(args: argparse.Namespace) -> dict[str, object]:
    payload = _load_stdin_json()

    with _open_store(args.database_path) as store:
        service = CommercialActivationService(
            store=store,
            config=CommercialBackendConfig(
                signing_key=args.signing_key,
                key_identifier=args.key_identifier,
                issuer=args.issuer,
            ),
        )
        decision = service.evaluate_activation_request(
            ActivationRequest(
                license_id=str(payload["license_id"]),
                device_hash=str(payload["device_hash"]),
                client_platform_identity=str(payload["client_platform_identity"]),
                requested_timestamp=_parse_epoch_seconds(
                    payload["requested_timestamp_epoch_seconds"],
                    "requested_timestamp_epoch_seconds",
                ),
                existing_activation_id=_optional_string(payload.get("existing_activation_id")),
            )
        )

    response: dict[str, object] = {
        "decision_type": decision.decision_type.value,
        "approval": None,
        "denial_reason": decision.denial_reason.value if decision.denial_reason else None,
        "technical_failure_reason": (
            decision.technical_failure_reason.value
            if decision.technical_failure_reason
            else None
        ),
    }

    if decision.approval is not None:
        response["approval"] = {
            "activation_id": decision.approval.activation_id,
            "license_id": decision.approval.license_id,
            "activation_slot_index": decision.approval.activation_slot_index,
            "plan_tier": decision.approval.plan_tier.value,
            "device_hash": decision.approval.device_hash,
            "device_bound_token": decision.approval.device_bound_token,
            "issued_timestamp_epoch_seconds": _serialize_epoch_seconds(
                decision.approval.issued_timestamp
            ),
            "expiration_timestamp_epoch_seconds": _serialize_epoch_seconds(
                decision.approval.expiration_timestamp
            ),
            "key_identifier": decision.approval.key_identifier,
            "reused_existing_slot": decision.approval.reused_existing_slot,
        }

    return response


def _resolve_runtime_delivery_manifest_path(args: argparse.Namespace) -> str | None:
    return args.runtime_delivery_manifest_path or os.environ.get(
        RUNTIME_DELIVERY_MANIFEST_PATH_ENV_VAR
    )


def _handle_resolve_runtime_delivery_manifest(args: argparse.Namespace) -> dict[str, object]:
    payload = _load_stdin_json()

    with _open_store(args.database_path) as store:
        service = CommercialActivationService(
            store=store,
            config=CommercialBackendConfig(
                signing_key=args.signing_key,
                key_identifier=args.key_identifier,
                issuer=args.issuer,
            ),
        )
        decision = service.resolve_runtime_delivery_manifest(
            RuntimeDeliveryManifestRequest(
                license_id=str(payload["license_id"]),
                requested_timestamp=_parse_epoch_seconds(
                    payload["requested_timestamp_epoch_seconds"],
                    "requested_timestamp_epoch_seconds",
                ),
            ),
            manifest_path=_resolve_runtime_delivery_manifest_path(args),
        )

    response: dict[str, object] = {
        "decision_type": decision.decision_type.value,
        "authorization": None,
        "denial_reason": decision.denial_reason.value if decision.denial_reason else None,
        "technical_failure_reason": (
            decision.technical_failure_reason.value
            if decision.technical_failure_reason
            else None
        ),
    }

    if decision.authorization is not None:
        response["authorization"] = {
            "license_id": decision.authorization.license_id,
            "manifest_path": decision.authorization.manifest_path,
            "manifest_fingerprint_sha256": decision.authorization.manifest_fingerprint_sha256,
        }

    return response


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="commercial_backend")
    subparsers = parser.add_subparsers(dest="command", required=True)

    apply_payment_event = subparsers.add_parser("apply-payment-event")
    apply_payment_event.add_argument("--database-path", required=True)
    apply_payment_event.add_argument(
        "--signing-key",
        default="repo-local-commercial-backend-signing-key",
    )
    apply_payment_event.add_argument(
        "--key-identifier",
        default="repo-local-kid-001",
    )
    apply_payment_event.add_argument(
        "--issuer",
        default="miro-fish-commercial-backend",
    )

    evaluate_activation_request = subparsers.add_parser("evaluate-activation-request")
    evaluate_activation_request.add_argument("--database-path", required=True)
    evaluate_activation_request.add_argument("--signing-key", required=True)
    evaluate_activation_request.add_argument("--key-identifier", required=True)
    evaluate_activation_request.add_argument(
        "--issuer",
        default="miro-fish-commercial-backend",
    )

    resolve_runtime_delivery_manifest = subparsers.add_parser(
        "resolve-runtime-delivery-manifest"
    )
    resolve_runtime_delivery_manifest.add_argument("--database-path", required=True)
    resolve_runtime_delivery_manifest.add_argument(
        "--runtime-delivery-manifest-path",
        default=None,
    )
    resolve_runtime_delivery_manifest.add_argument(
        "--signing-key",
        default="repo-local-commercial-backend-signing-key",
    )
    resolve_runtime_delivery_manifest.add_argument(
        "--key-identifier",
        default="repo-local-kid-001",
    )
    resolve_runtime_delivery_manifest.add_argument(
        "--issuer",
        default="miro-fish-commercial-backend",
    )

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    try:
        if args.command == "apply-payment-event":
            response = _handle_apply_payment_event(args)
        elif args.command == "resolve-runtime-delivery-manifest":
            response = _handle_resolve_runtime_delivery_manifest(args)
        else:
            response = _handle_evaluate_activation_request(args)
    except Exception as error:
        print(json.dumps({"error": str(error)}), file=sys.stderr)
        return 1

    print(json.dumps(response, separators=(",", ":")))
    return 0