from __future__ import annotations

import argparse
import json
import xml.etree.ElementTree as ET
import zipfile
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

from pypdf import PdfReader

from .bounded_memory import initialize_bounded_memory, write_bounded_memory_foundation
from .runtime_personas import (
    build_runtime_persona_foundation,
    derive_seed_context,
    validate_runtime_persona_foundation,
)
from .transcript_archive import initialize_transcript_archive

RUN_START_LINEAGE_MANIFEST_FILE_NAME = "run_start_lineage.json"


@dataclass(frozen=True)
class EngineSessionBootstrapResult:
    runtime_persona_manifest_path: str
    moderator_basis_path: str
    bounded_memory_directory_path: str
    seed_grounding_memory_path: str
    round_summary_memory_path: str
    argument_risk_memory_path: str
    transcript_archive_index_path: str
    moderator_persona_label: str
    participant_persona_labels: tuple[str, ...]
    perspective_coverage: tuple[str, ...]
    participant_persona_count: int
    exactly_one_moderator_present: bool
    stable_persona_labels: bool
    runtime_persona_foundation_materialized: bool
    bounded_memory_foundation_materialized: bool
    transcript_archive_separated: bool
    detail_message: str


def bootstrap_session_from_workspace(workspace_root: str | Path) -> EngineSessionBootstrapResult:
    workspace_root_path = Path(workspace_root)
    execution_seed_dir = workspace_root_path / "execution-seed"
    transcript_archive_dir = workspace_root_path / "transcript-archive"
    manifest_path = execution_seed_dir / RUN_START_LINEAGE_MANIFEST_FILE_NAME
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))

    selected_seed_document = manifest["selected_seed_document"]
    staged_seed_document_path = resolve_staged_seed_document_path(
        execution_seed_dir, selected_seed_document
    )
    seed_text = extract_seed_text(
        staged_seed_document_path, selected_seed_document.get("detected_kind")
    )
    participant_count = resolve_participant_count(manifest)
    seed_context = derive_seed_context(
        source_name=selected_seed_document["file_name"],
        source_kind=selected_seed_document.get("detected_kind")
        or staged_seed_document_path.suffix.lstrip("."),
        fingerprint_sha256=selected_seed_document["content_fingerprint_sha256"],
        raw_text=seed_text,
        participant_count=participant_count,
    )
    personas = build_runtime_persona_foundation(seed_context)
    validate_runtime_persona_foundation(personas)
    bounded_memory = initialize_bounded_memory(seed_context, personas)
    transcript_archive = initialize_transcript_archive(transcript_archive_dir)

    runtime_persona_dir = workspace_root_path / "runtime-personas"
    bounded_memory_dir = workspace_root_path / "bounded-memory"
    runtime_persona_dir.mkdir(parents=True, exist_ok=True)

    runtime_persona_manifest_path = runtime_persona_dir / "persona_set.json"
    moderator_basis_path = runtime_persona_dir / "moderator_basis.json"
    bounded_memory_paths = write_bounded_memory_foundation(
        bounded_memory_dir,
        bounded_memory,
    )

    runtime_persona_manifest_path.write_text(
        json.dumps(
            {
                "moderator": asdict(personas.moderator),
                "participants": [asdict(persona) for persona in personas.participants],
                "stable_labels": personas.stable_labels(),
                "perspective_coverage": personas.perspective_coverage(),
            },
            indent=2,
        ),
        encoding="utf-8",
    )
    moderator_basis_path.write_text(
        json.dumps(asdict(personas.moderator_basis), indent=2),
        encoding="utf-8",
    )

    return EngineSessionBootstrapResult(
        runtime_persona_manifest_path=str(runtime_persona_manifest_path),
        moderator_basis_path=str(moderator_basis_path),
        bounded_memory_directory_path=bounded_memory_paths.bounded_memory_directory_path,
        seed_grounding_memory_path=bounded_memory_paths.seed_grounding_memory_path,
        round_summary_memory_path=bounded_memory_paths.round_summary_memory_path,
        argument_risk_memory_path=bounded_memory_paths.argument_risk_memory_path,
        transcript_archive_index_path=transcript_archive.transcript_archive_index_path,
        moderator_persona_label=personas.moderator.stable_label,
        participant_persona_labels=tuple(persona.stable_label for persona in personas.participants),
        perspective_coverage=personas.perspective_coverage(),
        participant_persona_count=len(personas.participants),
        exactly_one_moderator_present=True,
        stable_persona_labels=True,
        runtime_persona_foundation_materialized=True,
        bounded_memory_foundation_materialized=True,
        transcript_archive_separated=not transcript_archive.prompt_context_source,
        detail_message=(
            "Runtime personas, Moderator basis, bounded memory, and transcript-archive separation "
            "were initialized from the approved staged seed context"
        ),
    )


def resolve_participant_count(manifest: dict[str, Any]) -> int:
    effective_plan = manifest.get("effective_plan") or {}
    requested_plan = manifest.get("requested_plan") or {}
    return int(
        effective_plan.get("effective_agent_count")
        or requested_plan.get("requested_agent_count")
        or 3
    )


def resolve_staged_seed_document_path(
    execution_seed_dir: Path,
    selected_seed_document: dict[str, Any],
) -> Path:
    candidate = execution_seed_dir / selected_seed_document["file_name"]
    if candidate.exists():
        return candidate

    staged_files = [
        path
        for path in execution_seed_dir.iterdir()
        if path.is_file() and path.name != RUN_START_LINEAGE_MANIFEST_FILE_NAME
    ]
    if len(staged_files) != 1:
        raise FileNotFoundError("staged seed document could not be resolved from execution-seed")
    return staged_files[0]


def extract_seed_text(seed_document_path: Path, detected_kind: str | None) -> str:
    normalized_kind = (detected_kind or seed_document_path.suffix.lstrip(".")).lower()
    if normalized_kind in {"txt", "markdown", "md"}:
        return seed_document_path.read_text(encoding="utf-8", errors="ignore")
    if normalized_kind == "docx":
        return extract_docx_text(seed_document_path)
    if normalized_kind == "pdf":
        return extract_pdf_text(seed_document_path)
    raise ValueError(f"unsupported seed-document kind for engine bootstrap: {normalized_kind}")


def extract_docx_text(seed_document_path: Path) -> str:
    with zipfile.ZipFile(seed_document_path) as archive:
        xml_bytes = archive.read("word/document.xml")
    root = ET.fromstring(xml_bytes)
    namespace = {"w": "http://schemas.openxmlformats.org/wordprocessingml/2006/main"}
    text_runs = [node.text or "" for node in root.findall(".//w:t", namespace)]
    return " ".join(text_runs)


def extract_pdf_text(seed_document_path: Path) -> str:
    reader = PdfReader(str(seed_document_path))
    return " ".join((page.extract_text() or "") for page in reader.pages)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--workspace-root", required=True)
    args = parser.parse_args()
    result = bootstrap_session_from_workspace(args.workspace_root)
    print(json.dumps(asdict(result)))


if __name__ == "__main__":
    main()
