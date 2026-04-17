from miro_fish_engine.moderator_basis import build_moderator_basis
from miro_fish_engine.runtime_personas import derive_seed_context


def test_moderator_basis_carries_stable_label_and_bounded_selection_rule() -> None:
    seed_context = derive_seed_context(
        source_name="strategy.md",
        source_kind="markdown",
        fingerprint_sha256="abc123",
        raw_text="Strategy evidence, governance concerns, and execution tradeoffs shape the topic.",
        participant_count=5,
    )

    moderator = build_moderator_basis(seed_context)

    assert moderator.stable_label == "Moderator"
    assert moderator.selection_policy_name == "moderated_3_to_5_speakers_per_round"
    assert moderator.normal_round_min_speakers == 3
    assert moderator.normal_round_max_speakers == 5
