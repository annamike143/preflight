from pathlib import Path

from miro_fish_engine.session_bootstrap import bootstrap_session_from_workspace


def test_session_bootstrap_materializes_personas_memory_and_archive(seeded_workspace: Path) -> None:
    result = bootstrap_session_from_workspace(seeded_workspace)

    assert Path(result.runtime_persona_manifest_path).exists()
    assert Path(result.moderator_basis_path).exists()
    assert Path(result.seed_grounding_memory_path).exists()
    assert Path(result.round_summary_memory_path).exists()
    assert Path(result.argument_risk_memory_path).exists()
    assert Path(result.transcript_archive_index_path).exists()
    assert result.moderator_persona_label == "Moderator"
    assert len(result.participant_persona_labels) == 5
    assert result.exactly_one_moderator_present is True
    assert result.stable_persona_labels is True
    assert result.runtime_persona_foundation_materialized is True
    assert result.bounded_memory_foundation_materialized is True
    assert result.transcript_archive_separated is True
