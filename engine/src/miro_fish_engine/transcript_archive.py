from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path

ARCHIVE_INDEX_FILE_NAME = "archive_index.json"
SPEAKER_TURNS_FILE_NAME = "speaker_turns.jsonl"


@dataclass(frozen=True)
class TranscriptArchiveSnapshot:
    transcript_archive_dir: str
    transcript_archive_index_path: str
    speaker_turns_path: str
    append_only: bool
    prompt_context_source: bool
    speaker_attribution_required: bool


def initialize_transcript_archive(transcript_archive_dir: Path) -> TranscriptArchiveSnapshot:
    transcript_archive_dir.mkdir(parents=True, exist_ok=True)
    speaker_turns_path = transcript_archive_dir / SPEAKER_TURNS_FILE_NAME
    transcript_archive_index_path = transcript_archive_dir / ARCHIVE_INDEX_FILE_NAME

    speaker_turns_path.touch(exist_ok=True)
    transcript_archive_index_path.write_text(
        json.dumps(
            {
                "archive_version": "wave5_execution_v1",
                "append_only": True,
                "prompt_context_source": False,
                "speaker_attribution_required": True,
                "speaker_turns_path": speaker_turns_path.name,
                "rounds_completed": 0,
                "total_turns": 0,
            },
            indent=2,
        ),
        encoding="utf-8",
    )

    return TranscriptArchiveSnapshot(
        transcript_archive_dir=str(transcript_archive_dir),
        transcript_archive_index_path=str(transcript_archive_index_path),
        speaker_turns_path=str(speaker_turns_path),
        append_only=True,
        prompt_context_source=False,
        speaker_attribution_required=True,
    )


def append_speaker_turn(transcript_archive_dir: Path, turn_payload: dict[str, object]) -> None:
    speaker_turns_path = transcript_archive_dir / SPEAKER_TURNS_FILE_NAME
    with speaker_turns_path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(turn_payload))
        handle.write("\n")


def update_transcript_archive_index(
    transcript_archive_dir: Path,
    *,
    rounds_completed: int,
    total_turns: int,
) -> None:
    transcript_archive_index_path = transcript_archive_dir / ARCHIVE_INDEX_FILE_NAME
    payload: dict[str, object]
    if transcript_archive_index_path.exists():
        payload = json.loads(transcript_archive_index_path.read_text(encoding="utf-8"))
    else:
        payload = {
            "archive_version": "wave5_execution_v1",
            "append_only": True,
            "prompt_context_source": False,
            "speaker_attribution_required": True,
            "speaker_turns_path": SPEAKER_TURNS_FILE_NAME,
        }

    payload["rounds_completed"] = rounds_completed
    payload["total_turns"] = total_turns
    transcript_archive_index_path.write_text(
        json.dumps(payload, indent=2),
        encoding="utf-8",
    )
