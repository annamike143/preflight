from pathlib import Path

from miro_fish_engine.token_duration_tracking import (
    TurnTokenDurationRecord,
    estimate_token_units,
    initialize_token_duration_tracking,
    load_token_duration_tracking,
    record_turn_tracking,
    write_token_duration_tracking,
)


def test_token_duration_tracking_accumulates_run_local_usage(tmp_path: Path) -> None:
    tracking_path, tracking_state = initialize_token_duration_tracking(
        tmp_path / "run-local-tracking"
    )
    first_record = TurnTokenDurationRecord(
        round_number=1,
        turn_index=1,
        speaker_label="Advocate",
        input_token_units=estimate_token_units("seed grounding", "bounded continuity"),
        output_token_units=estimate_token_units("bounded turn output"),
        total_token_units=9,
        duration_ms=4,
        occurred_at=1000,
    )
    second_record = TurnTokenDurationRecord(
        round_number=1,
        turn_index=2,
        speaker_label="Skeptic",
        input_token_units=6,
        output_token_units=5,
        total_token_units=11,
        duration_ms=7,
        occurred_at=2000,
    )

    tracking_state = record_turn_tracking(tracking_state, first_record)
    tracking_state = record_turn_tracking(tracking_state, second_record)
    write_token_duration_tracking(tracking_path, tracking_state)
    loaded_tracking_state = load_token_duration_tracking(tracking_path)

    assert Path(tracking_path).exists()
    assert loaded_tracking_state.total_rounds_completed == 1
    assert loaded_tracking_state.total_turns_tracked == 2
    assert loaded_tracking_state.total_token_units == 20
    assert loaded_tracking_state.total_duration_ms == 11
    assert loaded_tracking_state.average_turn_duration_ms == 5
    assert len(loaded_tracking_state.turn_records) == 2
    assert loaded_tracking_state.turn_records[0].occurred_at == 1000
    assert loaded_tracking_state.turn_records[1].occurred_at == 2000