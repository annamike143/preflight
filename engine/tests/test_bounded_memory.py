from miro_fish_engine.bounded_memory import (
    initialize_bounded_memory,
    update_bounded_memory_for_round,
)
from miro_fish_engine.runtime_personas import build_runtime_persona_foundation, derive_seed_context


def test_bounded_memory_stays_seed_grounded_and_separate_from_transcript_replay() -> None:
    seed_context = derive_seed_context(
        source_name="seed.txt",
        source_kind="txt",
        fingerprint_sha256="abc123",
        raw_text="Seed context stays grounded in evidence, risks, and operational tradeoffs.",
        participant_count=5,
    )
    personas = build_runtime_persona_foundation(seed_context)
    memory = initialize_bounded_memory(seed_context, personas)

    assert memory.seed_grounding.seed_fingerprint_sha256 == "abc123"
    assert memory.seed_grounding.transcript_archive_prompt_source_allowed is False
    assert memory.round_summary.completed_rounds == 0
    assert memory.argument_risk.arguments == ()
    assert memory.cross_run_semantic_leakage_allowed is False
    assert memory.transcript_archive_separated is True


def test_bounded_memory_updates_round_continuity_without_transcript_replay() -> None:
    seed_context = derive_seed_context(
        source_name="seed.txt",
        source_kind="txt",
        fingerprint_sha256="abc123",
        raw_text="Seed context keeps continuity bounded across arguments, objections, and risks.",
        participant_count=5,
    )
    personas = build_runtime_persona_foundation(seed_context)
    memory = initialize_bounded_memory(seed_context, personas)

    updated_memory = update_bounded_memory_for_round(
        memory,
        round_summary="Round 1 carried forward bounded continuity.",
        arguments=("Advance the evidence-backed opportunity.",),
        objections=("The operating assumption remains unproven.",),
        risks=("Governance drift could weaken the plan.",),
    )

    assert updated_memory.round_summary.completed_rounds == 1
    assert updated_memory.round_summary.latest_round_summary is not None
    assert updated_memory.round_summary.summary_history == (
        "Round 1 carried forward bounded continuity.",
    )
    assert updated_memory.argument_risk.arguments == (
        "Advance the evidence-backed opportunity.",
    )
    assert updated_memory.argument_risk.objections == (
        "The operating assumption remains unproven.",
    )
    assert updated_memory.argument_risk.risks == (
        "Governance drift could weaken the plan.",
    )
