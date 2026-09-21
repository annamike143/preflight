from __future__ import annotations

from pathlib import Path

from miro_fish_engine.cli import main
from miro_fish_engine.workspace_harness import HeadlessWorkspaceHarness


def test_cli_missing_seed(tmp_path: Path):
    non_existent = tmp_path / "missing.md"
    exit_code = main(["--seed", str(non_existent)])
    assert exit_code == 1


def test_cli_dry_run():
    seed_path = Path(__file__).parent.parent / "evals" / "test_seeds" / "api_rfc.md"
    assert seed_path.exists()

    exit_code = main(["--seed", str(seed_path), "--dry-run"])
    assert exit_code == 0


def test_headless_workspace_harness(tmp_path: Path):
    seed_path = Path(__file__).parent.parent / "evals" / "test_seeds" / "api_rfc.md"
    assert seed_path.exists()

    output_pdf = tmp_path / "custom_report.pdf"
    harness = HeadlessWorkspaceHarness(
        seed_path=seed_path,
        agent_count=3,
        round_count=1,
        token_budget=6000,
    )

    result = harness.run(output_pdf_path=output_pdf)

    assert result.report_pdf_path.exists()
    assert result.report_data_path.exists()
    assert result.persona_count == 3
    assert result.round_count == 1
    assert result.turn_count >= 1
    assert len(result.personas) == 3
