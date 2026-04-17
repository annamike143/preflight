from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parents[1]
PACKAGE_ROOT = PROJECT_ROOT / "src"


def _run_cli(*args: str, payload: dict[str, object]) -> dict[str, object]:
    environment = os.environ.copy()
    environment["PYTHONPATH"] = str(PACKAGE_ROOT)

    result = subprocess.run(
        [sys.executable, "-m", "commercial_backend", *args],
        cwd=PROJECT_ROOT,
        env=environment,
        input=json.dumps(payload),
        text=True,
        capture_output=True,
        check=True,
    )

    return json.loads(result.stdout)


def test_apply_payment_event_cli_returns_created_license() -> None:
    response = _run_cli(
        "apply-payment-event",
        "--database-path",
        ":memory:",
        payload={
            "payment_provider_identifier": "stripe",
            "provider_event_identifier": "evt-cli-001",
            "event_type": "payment_succeeded",
            "purchaser_email": "owner@example.com",
            "plan_tier": "annual",
            "receipt_timestamp_epoch_seconds": 100,
            "license_id": None,
            "expiration_timestamp_epoch_seconds": None,
            "payload_reference": "evt_ref_001",
            "entitlement_active": True,
        },
    )

    assert response["license_id"]
    assert response["plan_tier"] == "annual"
    assert response["entitlement_status"] == "active"
    assert response["activation_limit"] == 2


def test_evaluate_activation_request_cli_returns_structured_approval(tmp_path: Path) -> None:
    database_path = str(tmp_path / "cli_activation.sqlite3")

    apply_response = _run_cli(
        "apply-payment-event",
        "--database-path",
        database_path,
        payload={
            "payment_provider_identifier": "stripe",
            "provider_event_identifier": "evt-cli-approval-001",
            "event_type": "payment_succeeded",
            "purchaser_email": "owner@example.com",
            "plan_tier": "annual",
            "receipt_timestamp_epoch_seconds": 100,
            "license_id": None,
            "expiration_timestamp_epoch_seconds": None,
            "payload_reference": "evt_ref_001",
            "entitlement_active": True,
        },
    )

    activation_response = _run_cli(
        "evaluate-activation-request",
        "--database-path",
        database_path,
        "--signing-key",
        "signing-key-001",
        "--key-identifier",
        "kid-001",
        payload={
            "license_id": apply_response["license_id"],
            "device_hash": "device-hash-a",
            "client_platform_identity": "windows-x86_64",
            "requested_timestamp_epoch_seconds": 100,
            "existing_activation_id": None,
        },
    )

    assert activation_response["decision_type"] == "approved"
    assert activation_response["approval"]["license_id"] == apply_response["license_id"]
    assert activation_response["approval"]["device_hash"] == "device-hash-a"
    assert activation_response["approval"]["reused_existing_slot"] is False
    assert activation_response["denial_reason"] is None
    assert activation_response["technical_failure_reason"] is None


def test_resolve_runtime_delivery_manifest_cli_returns_authorized_manifest_reference(
    tmp_path: Path,
) -> None:
    database_path = str(tmp_path / "cli_runtime_delivery.sqlite3")
    manifest_path = tmp_path / "runtime_delivery_manifest.json"
    manifest_path.write_text(
        '{"schema_version":1,"runtime_version":"3.14.0","source_root":"runtime","executable_relative_path":"python.exe","compatibility_target_os":"windows","compatibility_target_arch":"x86_64","expected_files":[{"relative_path":"python.exe","sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"},{"relative_path":"support.dll","sha256":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}]}'
    )

    apply_response = _run_cli(
        "apply-payment-event",
        "--database-path",
        database_path,
        payload={
            "payment_provider_identifier": "stripe",
            "provider_event_identifier": "evt-cli-runtime-001",
            "event_type": "payment_succeeded",
            "purchaser_email": "owner@example.com",
            "plan_tier": "annual",
            "receipt_timestamp_epoch_seconds": 100,
            "license_id": None,
            "expiration_timestamp_epoch_seconds": None,
            "payload_reference": "evt_ref_runtime_001",
            "entitlement_active": True,
        },
    )

    runtime_delivery_response = _run_cli(
        "resolve-runtime-delivery-manifest",
        "--database-path",
        database_path,
        "--runtime-delivery-manifest-path",
        str(manifest_path),
        payload={
            "license_id": apply_response["license_id"],
            "requested_timestamp_epoch_seconds": 100,
        },
    )

    assert runtime_delivery_response["decision_type"] == "authorized"
    assert (
        runtime_delivery_response["authorization"]["license_id"]
        == apply_response["license_id"]
    )
    assert (
        runtime_delivery_response["authorization"]["manifest_path"]
        == str(manifest_path.resolve())
    )
    assert (
        len(runtime_delivery_response["authorization"]["manifest_fingerprint_sha256"])
        == 64
    )
    assert runtime_delivery_response["denial_reason"] is None
    assert runtime_delivery_response["technical_failure_reason"] is None