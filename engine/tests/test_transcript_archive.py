import json
from pathlib import Path

from miro_fish_engine.transcript_archive import (
    append_speaker_turn,
    initialize_transcript_archive,
    update_transcript_archive_index,
)


def test_transcript_archive_initialization_creates_non_prompt_source_index(tmp_path: Path) -> None:
    snapshot = initialize_transcript_archive(tmp_path / "transcript-archive")

    assert Path(snapshot.transcript_archive_index_path).exists()
    assert Path(snapshot.speaker_turns_path).exists()
    assert snapshot.append_only is True
    assert snapshot.prompt_context_source is False
    assert snapshot.speaker_attribution_required is True


def test_transcript_archive_appends_speaker_turns_and_updates_index(tmp_path: Path) -> None:
    archive_dir = tmp_path / "transcript-archive"
    snapshot = initialize_transcript_archive(archive_dir)

    append_speaker_turn(
        archive_dir,
        {
            "round_number": 1,
            "turn_index": 1,
            "speaker_label": "Advocate",
            "speaker_perspective": "favorable",
            "turn_text": "The seed suggests a bounded upside.",
            "occurred_at": 1000,
        },
    )
    update_transcript_archive_index(archive_dir, rounds_completed=1, total_turns=1)

    speaker_turns = Path(snapshot.speaker_turns_path).read_text(encoding="utf-8").splitlines()
    index_payload = json.loads(
        Path(snapshot.transcript_archive_index_path).read_text(encoding="utf-8")
    )

    assert len(speaker_turns) == 1
    assert json.loads(speaker_turns[0])["speaker_label"] == "Advocate"
    assert json.loads(speaker_turns[0])["occurred_at"] == 1000
    assert index_payload["rounds_completed"] == 1
    assert index_payload["total_turns"] == 1
