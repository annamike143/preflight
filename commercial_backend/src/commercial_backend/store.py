from __future__ import annotations

import sqlite3
from datetime import UTC, datetime
from pathlib import Path
from uuid import uuid4

from .migration_inventory import ordered_migrations
from .models import (
    ActivationAuditAction,
    ActivationAuditRecord,
    ActivationStatus,
    DeviceActivationRecord,
    LicenseRecord,
    LicenseStatus,
    PaymentEventInput,
    PaymentEventRecord,
    PaymentProcessingStatus,
    PlanTier,
    TokenIssuanceRecord,
    TokenStatus,
    derive_expiration_timestamp,
    normalize_timestamp,
    resolve_license_status,
)


def _serialize_timestamp(value: datetime | None) -> str | None:
    if value is None:
        return None

    return normalize_timestamp(value).isoformat()


def _parse_timestamp(value: str | None) -> datetime | None:
    if value is None:
        return None

    return normalize_timestamp(datetime.fromisoformat(value))


class CommercialLedgerStore:
    def __init__(self, database_path: str | Path):
        self.database_path = database_path
        self._connection = sqlite3.connect(str(database_path))
        self._connection.row_factory = sqlite3.Row
        self._connection.execute("PRAGMA foreign_keys = ON")
        self._package_root = Path(__file__).resolve().parents[2]
        self._ensure_schema_migrations_table()
        self.apply_migrations()

    @classmethod
    def in_memory(cls) -> "CommercialLedgerStore":
        return cls(":memory:")

    def close(self) -> None:
        self._connection.close()

    def __enter__(self) -> "CommercialLedgerStore":
        return self

    def __exit__(self, exc_type, exc, exc_tb) -> None:
        self.close()

    def _ensure_schema_migrations_table(self) -> None:
        self._connection.execute(
            """
            CREATE TABLE IF NOT EXISTS schema_migrations (
                migration_name TEXT PRIMARY KEY,
                applied_timestamp TEXT NOT NULL
            )
            """
        )
        self._connection.commit()

    def apply_migrations(self) -> None:
        for migration in ordered_migrations(self._package_root):
            already_applied = self._connection.execute(
                "SELECT migration_name FROM schema_migrations WHERE migration_name = ?",
                (migration.migration_name,),
            ).fetchone()
            if already_applied is not None:
                continue

            self._connection.executescript(migration.file_path.read_text(encoding="utf-8"))
            self._connection.execute(
                "INSERT INTO schema_migrations (migration_name, applied_timestamp) VALUES (?, ?)",
                (
                    migration.migration_name,
                    _serialize_timestamp(datetime.now(UTC)),
                ),
            )
            self._connection.commit()

    def count_rows(self, table_name: str) -> int:
        row = self._connection.execute(f"SELECT COUNT(*) AS row_count FROM {table_name}").fetchone()
        return int(row["row_count"])

    def get_payment_event_by_provider_event(
        self, payment_provider_identifier: str, provider_event_identifier: str
    ) -> PaymentEventRecord | None:
        row = self._connection.execute(
            """
            SELECT *
            FROM payment_events
            WHERE payment_provider_identifier = ? AND provider_event_identifier = ?
            """,
            (payment_provider_identifier, provider_event_identifier),
        ).fetchone()
        if row is None:
            return None

        return self._row_to_payment_event(row)

    def record_payment_event(
        self, event: PaymentEventInput, related_license_id: str
    ) -> PaymentEventRecord:
        existing = self.get_payment_event_by_provider_event(
            event.payment_provider_identifier, event.provider_event_identifier
        )
        if existing is not None:
            return existing

        internal_event_id = str(uuid4())
        self._connection.execute(
            """
            INSERT INTO payment_events (
                internal_event_id,
                payment_provider_identifier,
                provider_event_identifier,
                event_type,
                receipt_timestamp,
                processing_status,
                payload_reference,
                related_license_id,
                purchaser_email,
                plan_tier
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                internal_event_id,
                event.payment_provider_identifier,
                event.provider_event_identifier,
                event.event_type,
                _serialize_timestamp(event.receipt_timestamp),
                PaymentProcessingStatus.RECEIVED.value,
                event.payload_reference,
                related_license_id,
                event.purchaser_email,
                event.plan_tier.value,
            ),
        )
        self._connection.commit()
        return self.get_payment_event_by_provider_event(
            event.payment_provider_identifier, event.provider_event_identifier
        )

    def mark_payment_event_processed(
        self, internal_event_id: str, status: PaymentProcessingStatus
    ) -> None:
        self._connection.execute(
            "UPDATE payment_events SET processing_status = ? WHERE internal_event_id = ?",
            (status.value, internal_event_id),
        )
        self._connection.commit()

    def upsert_license_from_payment_event(
        self, event: PaymentEventInput, related_license_id: str
    ) -> LicenseRecord:
        now = normalize_timestamp(event.receipt_timestamp)
        expiration_timestamp = derive_expiration_timestamp(
            event.plan_tier, now, event.expiration_timestamp
        )
        entitlement_status = resolve_license_status(event)
        existing = self.get_license(related_license_id)

        if existing is None:
            self._connection.execute(
                """
                INSERT INTO licenses (
                    license_id,
                    purchaser_email,
                    plan_tier,
                    entitlement_status,
                    issue_timestamp,
                    expiration_timestamp,
                    activation_limit,
                    created_timestamp,
                    updated_timestamp
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                """,
                (
                    related_license_id,
                    event.purchaser_email,
                    event.plan_tier.value,
                    entitlement_status.value,
                    _serialize_timestamp(now),
                    _serialize_timestamp(expiration_timestamp),
                    2,
                    _serialize_timestamp(now),
                    _serialize_timestamp(now),
                ),
            )
        else:
            self._connection.execute(
                """
                UPDATE licenses
                SET purchaser_email = ?,
                    plan_tier = ?,
                    entitlement_status = ?,
                    expiration_timestamp = ?,
                    updated_timestamp = ?
                WHERE license_id = ?
                """,
                (
                    event.purchaser_email,
                    event.plan_tier.value,
                    entitlement_status.value,
                    _serialize_timestamp(expiration_timestamp),
                    _serialize_timestamp(now),
                    related_license_id,
                ),
            )

        self._connection.commit()
        return self.get_license(related_license_id)

    def get_license(self, license_id: str) -> LicenseRecord | None:
        row = self._connection.execute(
            "SELECT * FROM licenses WHERE license_id = ?", (license_id,)
        ).fetchone()
        if row is None:
            return None

        return self._row_to_license(row)

    def get_activation_by_id(self, activation_id: str) -> DeviceActivationRecord | None:
        row = self._connection.execute(
            "SELECT * FROM device_activations WHERE activation_id = ?", (activation_id,)
        ).fetchone()
        if row is None:
            return None

        return self._row_to_device_activation(row)

    def get_active_activation_by_device(
        self, license_id: str, device_hash: str
    ) -> DeviceActivationRecord | None:
        row = self._connection.execute(
            """
            SELECT *
            FROM device_activations
            WHERE parent_license_id = ? AND device_hash = ? AND activation_status = ?
            """,
            (license_id, device_hash, ActivationStatus.ACTIVE.value),
        ).fetchone()
        if row is None:
            return None

        return self._row_to_device_activation(row)

    def list_active_activations(self, license_id: str) -> list[DeviceActivationRecord]:
        rows = self._connection.execute(
            """
            SELECT *
            FROM device_activations
            WHERE parent_license_id = ? AND activation_status = ?
            ORDER BY activation_slot_index ASC
            """,
            (license_id, ActivationStatus.ACTIVE.value),
        ).fetchall()
        return [self._row_to_device_activation(row) for row in rows]

    def create_activation(
        self,
        license_id: str,
        activation_slot_index: int,
        device_hash: str,
        client_platform_identity: str,
        created_at,
    ) -> DeviceActivationRecord:
        activation_id = str(uuid4())
        timestamp = _serialize_timestamp(created_at)
        self._connection.execute(
            """
            INSERT INTO device_activations (
                activation_id,
                parent_license_id,
                activation_slot_index,
                device_hash,
                client_platform_identity,
                activation_status,
                first_activation_timestamp,
                last_confirmed_timestamp,
                created_timestamp,
                updated_timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                activation_id,
                license_id,
                activation_slot_index,
                device_hash,
                client_platform_identity,
                ActivationStatus.ACTIVE.value,
                timestamp,
                timestamp,
                timestamp,
                timestamp,
            ),
        )
        self._connection.commit()
        return self.get_activation_by_id(activation_id)

    def touch_activation_confirmation(
        self, activation_id: str, confirmed_at
    ) -> DeviceActivationRecord:
        timestamp = _serialize_timestamp(confirmed_at)
        self._connection.execute(
            """
            UPDATE device_activations
            SET last_confirmed_timestamp = ?,
                updated_timestamp = ?
            WHERE activation_id = ?
            """,
            (timestamp, timestamp, activation_id),
        )
        self._connection.commit()
        return self.get_activation_by_id(activation_id)

    def record_activation_audit(
        self,
        related_license_id: str | None,
        related_activation_id: str | None,
        action_type: ActivationAuditAction,
        actor_type: str,
        reason_classification: str,
        recorded_at,
        metadata_reference: str | None = None,
    ) -> ActivationAuditRecord:
        audit_id = str(uuid4())
        self._connection.execute(
            """
            INSERT INTO activation_audits (
                audit_id,
                related_license_id,
                related_activation_id,
                action_type,
                actor_type,
                reason_classification,
                metadata_reference,
                recorded_timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                audit_id,
                related_license_id,
                related_activation_id,
                action_type.value,
                actor_type,
                reason_classification,
                metadata_reference,
                _serialize_timestamp(recorded_at),
            ),
        )
        self._connection.commit()
        row = self._connection.execute(
            "SELECT * FROM activation_audits WHERE audit_id = ?", (audit_id,)
        ).fetchone()
        return self._row_to_activation_audit(row)

    def supersede_active_token_issuances(self, activation_id: str) -> None:
        self._connection.execute(
            """
            UPDATE token_issuances
            SET token_status = ?
            WHERE parent_activation_id = ? AND token_status = ?
            """,
            (TokenStatus.SUPERSEDED.value, activation_id, TokenStatus.ACTIVE.value),
        )
        self._connection.commit()

    def record_token_issuance(
        self,
        license_id: str,
        activation_id: str,
        issued_at,
        expiration_timestamp,
        key_identifier: str,
        token_fingerprint: str,
    ) -> TokenIssuanceRecord:
        token_issuance_id = str(uuid4())
        self._connection.execute(
            """
            INSERT INTO token_issuances (
                token_issuance_id,
                parent_license_id,
                parent_activation_id,
                issuance_timestamp,
                expiration_timestamp,
                key_identifier,
                token_fingerprint,
                token_status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                token_issuance_id,
                license_id,
                activation_id,
                _serialize_timestamp(issued_at),
                _serialize_timestamp(expiration_timestamp),
                key_identifier,
                token_fingerprint,
                TokenStatus.ACTIVE.value,
            ),
        )
        self._connection.commit()
        row = self._connection.execute(
            "SELECT * FROM token_issuances WHERE token_issuance_id = ?", (token_issuance_id,)
        ).fetchone()
        return self._row_to_token_issuance(row)

    def list_token_issuances_for_activation(
        self, activation_id: str
    ) -> list[TokenIssuanceRecord]:
        rows = self._connection.execute(
            """
            SELECT *
            FROM token_issuances
            WHERE parent_activation_id = ?
            ORDER BY issuance_timestamp ASC
            """,
            (activation_id,),
        ).fetchall()
        return [self._row_to_token_issuance(row) for row in rows]

    def _row_to_license(self, row: sqlite3.Row) -> LicenseRecord:
        return LicenseRecord(
            license_id=row["license_id"],
            purchaser_email=row["purchaser_email"],
            plan_tier=PlanTier(row["plan_tier"]),
            entitlement_status=LicenseStatus(row["entitlement_status"]),
            issue_timestamp=_parse_timestamp(row["issue_timestamp"]),
            expiration_timestamp=_parse_timestamp(row["expiration_timestamp"]),
            activation_limit=int(row["activation_limit"]),
            created_timestamp=_parse_timestamp(row["created_timestamp"]),
            updated_timestamp=_parse_timestamp(row["updated_timestamp"]),
        )

    def _row_to_payment_event(self, row: sqlite3.Row) -> PaymentEventRecord:
        return PaymentEventRecord(
            internal_event_id=row["internal_event_id"],
            payment_provider_identifier=row["payment_provider_identifier"],
            provider_event_identifier=row["provider_event_identifier"],
            event_type=row["event_type"],
            receipt_timestamp=_parse_timestamp(row["receipt_timestamp"]),
            processing_status=PaymentProcessingStatus(row["processing_status"]),
            payload_reference=row["payload_reference"],
            related_license_id=row["related_license_id"],
            purchaser_email=row["purchaser_email"],
            plan_tier=PlanTier(row["plan_tier"]),
        )

    def _row_to_device_activation(self, row: sqlite3.Row) -> DeviceActivationRecord:
        return DeviceActivationRecord(
            activation_id=row["activation_id"],
            parent_license_id=row["parent_license_id"],
            activation_slot_index=int(row["activation_slot_index"]),
            device_hash=row["device_hash"],
            client_platform_identity=row["client_platform_identity"],
            activation_status=ActivationStatus(row["activation_status"]),
            first_activation_timestamp=_parse_timestamp(row["first_activation_timestamp"]),
            last_confirmed_timestamp=_parse_timestamp(row["last_confirmed_timestamp"]),
            created_timestamp=_parse_timestamp(row["created_timestamp"]),
            updated_timestamp=_parse_timestamp(row["updated_timestamp"]),
        )

    def _row_to_activation_audit(self, row: sqlite3.Row) -> ActivationAuditRecord:
        return ActivationAuditRecord(
            audit_id=row["audit_id"],
            related_license_id=row["related_license_id"],
            related_activation_id=row["related_activation_id"],
            action_type=ActivationAuditAction(row["action_type"]),
            actor_type=row["actor_type"],
            reason_classification=row["reason_classification"],
            metadata_reference=row["metadata_reference"],
            recorded_timestamp=_parse_timestamp(row["recorded_timestamp"]),
        )

    def _row_to_token_issuance(self, row: sqlite3.Row) -> TokenIssuanceRecord:
        return TokenIssuanceRecord(
            token_issuance_id=row["token_issuance_id"],
            parent_license_id=row["parent_license_id"],
            parent_activation_id=row["parent_activation_id"],
            issuance_timestamp=_parse_timestamp(row["issuance_timestamp"]),
            expiration_timestamp=_parse_timestamp(row["expiration_timestamp"]),
            key_identifier=row["key_identifier"],
            token_fingerprint=row["token_fingerprint"],
            token_status=TokenStatus(row["token_status"]),
        )