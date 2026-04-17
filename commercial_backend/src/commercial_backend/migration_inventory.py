from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class MigrationDefinition:
    migration_name: str
    storage_domain: str
    primary_purpose: str
    migration_scope: str
    file_path: Path
    upstream_traceability_note: str


def ordered_migrations(package_root: Path) -> tuple[MigrationDefinition, ...]:
    return (
        MigrationDefinition(
            migration_name="0001_initial_commercial_ledger",
            storage_domain="commercial backend domain",
            primary_purpose=(
                "initial license, activation, payment event, audit, "
                "and token lineage tables"
            ),
            migration_scope="backend-facing",
            file_path=package_root / "migrations" / "0001_initial_commercial_ledger.sql",
            upstream_traceability_note=(
                "DB-005 through DB-070 and CDATA-029 through CDATA-045"
            ),
        ),
    )