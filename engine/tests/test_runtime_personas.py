from miro_fish_engine.runtime_personas import build_runtime_persona_foundation, derive_seed_context


def test_runtime_personas_enforce_exactly_one_moderator_and_required_diversity() -> None:
    seed_context = derive_seed_context(
        source_name="seed.txt",
        source_kind="txt",
        fingerprint_sha256="abc123",
        raw_text=(
            "This seed discusses product opportunity, downside risk, evidence quality, and "
            "implementation tradeoffs for a bounded moderated simulation run."
        ),
        participant_count=5,
    )

    foundation = build_runtime_persona_foundation(seed_context)

    assert foundation.moderator.stable_label == "Moderator"
    assert len(foundation.participants) == 5
    assert len(set(foundation.stable_labels())) == len(foundation.stable_labels())
    assert {"favorable", "skeptical", "analytical"}.issubset(set(foundation.perspective_coverage()))
    assert (
        "product" in foundation.moderator.focus_note.lower()
        or "opportunity" in foundation.moderator.focus_note.lower()
    )
