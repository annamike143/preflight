from __future__ import annotations

import hashlib
import json
import shutil
import tempfile
from dataclasses import dataclass
from pathlib import Path

from .session_bootstrap import RUN_START_LINEAGE_MANIFEST_FILE_NAME
from .session_execution import (
    EXECUTION_CONTROL_FILE_NAME,
    EngineSessionExecutionResult,
    execute_session_from_workspace,
)


@dataclass(frozen=True)
class HeadlessRunResult:
    report_pdf_path: Path
    report_data_path: Path
    total_tokens: int
    duration_ms: int
    persona_count: int
    round_count: int
    turn_count: int
    personas: tuple[str, ...]
    summary: str


class HeadlessWorkspaceHarness:
    """Ephemeral workspace orchestrator for executing Preflight simulations headlessly
    without the Tauri desktop shell.
    """

    def __init__(
        self,
        seed_path: str | Path,
        agent_count: int = 4,
        round_count: int = 3,
        token_budget: int = 12000,
    ) -> None:
        self.seed_path = Path(seed_path).resolve()
        if not self.seed_path.exists():
            raise FileNotFoundError(f"Seed document not found at: {self.seed_path}")

        self.agent_count = agent_count
        self.round_count = round_count
        self.token_budget = token_budget

    def _compute_sha256(self, file_path: Path) -> str:
        hasher = hashlib.sha256()
        with open(file_path, "rb") as f:
            while chunk := f.read(65536):
                hasher.update(chunk)
        return hasher.hexdigest()

    def run(self, output_pdf_path: str | Path | None = None) -> HeadlessRunResult:
        """Executes the simulation in an ephemeral workspace and exports the final PDF."""
        fingerprint = self._compute_sha256(self.seed_path)
        detected_kind = self.seed_path.suffix.lstrip(".").lower() or "txt"

        with tempfile.TemporaryDirectory(prefix="preflight_headless_") as temp_dir_str:
            workspace_root = Path(temp_dir_str)
            seed_dir = workspace_root / "execution-seed"
            seed_dir.mkdir(parents=True, exist_ok=True)

            # 1. Stage the seed document
            staged_seed = seed_dir / self.seed_path.name
            shutil.copy2(self.seed_path, staged_seed)

            # 2. Generate run_start_lineage.json manifest
            lineage_manifest = {
                "selected_seed_document": {
                    "file_name": self.seed_path.name,
                    "detected_kind": detected_kind,
                    "content_fingerprint_sha256": fingerprint,
                },
                "effective_plan": {
                    "effective_agent_count": self.agent_count,
                    "effective_round_count": self.round_count,
                    "max_token_budget": self.token_budget,
                },
                "requested_plan": {
                    "requested_agent_count": self.agent_count,
                    "requested_round_count": self.round_count,
                    "max_token_budget": self.token_budget,
                },
            }
            manifest_file = seed_dir / RUN_START_LINEAGE_MANIFEST_FILE_NAME
            manifest_file.write_text(json.dumps(lineage_manifest, indent=2), encoding="utf-8")

            # 3. Generate execution_control.json
            control_file = workspace_root / EXECUTION_CONTROL_FILE_NAME
            control_file.write_text(
                json.dumps(
                    {
                        "cancellation_requested": False,
                        "pause_requested": False,
                    }
                ),
                encoding="utf-8",
            )

            # 4. Execute session
            result: EngineSessionExecutionResult = execute_session_from_workspace(workspace_root)

            # 5. Resolve output artifact
            if not result.report_pdf_artifact_path:
                raise RuntimeError(
                    f"Report PDF generation failed: {result.report_generation_failure_reason}"
                )

            src_pdf = Path(result.report_pdf_artifact_path)
            src_data = (
                Path(result.report_data_artifact_path)
                if result.report_data_artifact_path
                else None
            )

            final_pdf = (
                Path(output_pdf_path).resolve()
                if output_pdf_path
                else Path.cwd() / f"preflight_report_{self.seed_path.stem}.pdf"
            )
            shutil.copy2(src_pdf, final_pdf)

            final_data = final_pdf.with_suffix(".json")
            if src_data and src_data.exists():
                shutil.copy2(src_data, final_data)

            return HeadlessRunResult(
                report_pdf_path=final_pdf,
                report_data_path=final_data,
                total_tokens=result.tracked_total_token_units,
                duration_ms=result.tracked_total_duration_ms,
                persona_count=result.participant_persona_count,
                round_count=len(result.speaker_counts_per_round),
                turn_count=result.total_turn_count,
                personas=result.participant_persona_labels,
                summary=result.latest_round_summary or "Simulation completed successfully.",
            )
