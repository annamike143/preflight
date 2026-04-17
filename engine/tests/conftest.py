from __future__ import annotations

import json
import sys
from pathlib import Path

import pytest

ENGINE_SRC = Path(__file__).resolve().parents[1] / "src"
if str(ENGINE_SRC) not in sys.path:
    sys.path.insert(0, str(ENGINE_SRC))


@pytest.fixture
def seeded_workspace(tmp_path: Path) -> Path:
    workspace_root = tmp_path / "run-1"
    execution_seed_dir = workspace_root / "execution-seed"
    transcript_archive_dir = workspace_root / "transcript-archive"
    execution_seed_dir.mkdir(parents=True)
    transcript_archive_dir.mkdir(parents=True)

    seed_path = execution_seed_dir / "seed.txt"
    seed_path.write_text(
        (
            "Miro Fish explores product positioning, risk management, operational tradeoffs, "
            "and evidence-backed decision quality for a bounded moderated simulation run."
        ),
        encoding="utf-8",
    )

    manifest_path = execution_seed_dir / "run_start_lineage.json"
    manifest_path.write_text(
        json.dumps(
            {
                "lineage_name": "RunStartLineage",
                "preflight_state_name": "PreflightApproved",
                "preflight_run_start_eligible_now": True,
                "preflight_result_current": True,
                "downgrade_acceptance_required": False,
                "downgrade_acceptance_recorded": False,
                "selected_seed_document": {
                    "file_name": "seed.txt",
                    "mime_type": "text/plain",
                    "detected_kind": "txt",
                    "size_bytes": seed_path.stat().st_size,
                    "content_fingerprint_sha256": (
                        "feedbead1234feedbead1234feedbead1234"
                        "feedbead1234feedbead1234"
                    ),
                    "extracted_text_character_count": 1400,
                    "english_confidence_percent": 99,
                },
                "requested_plan": {
                    "requested_agent_count": 5,
                    "requested_round_count": 4,
                    "max_token_budget": 12000,
                },
                "effective_plan": {
                    "effective_agent_count": 5,
                    "effective_round_count": 4,
                    "provider": "open_ai",
                    "model": "gpt-4o-mini",
                    "max_token_budget": 12000,
                    "bounded_speaker_model": "moderated_3_to_5_speakers_per_round",
                    "bounded_concurrency_policy": "single_moderated_speaker_set_per_round",
                },
                "provider_model_context": {
                    "provider": "open_ai",
                    "model": "gpt-4o-mini",
                    "local_configuration_supports_planning": True,
                    "live_reachability_probed": False,
                    "detail_note": "planning context",
                },
                "detail_note": (
                    "Current eligible Preflight truth already contains the bounded seed "
                    "fingerprint required for a future run start"
                ),
            },
            indent=2,
        ),
        encoding="utf-8",
    )

    return workspace_root
