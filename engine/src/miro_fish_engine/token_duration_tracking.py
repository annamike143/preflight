from __future__ import annotations

import json
import math
from dataclasses import asdict, dataclass
from pathlib import Path

RUN_LOCAL_TRACKING_DIRECTORY_NAME = "run-local-tracking"
TOKEN_DURATION_TRACKING_FILE_NAME = "token_duration_tracking.json"
EXECUTION_PHASE_FILE_NAME = "execution_phase.json"


@dataclass(frozen=True)
class TurnTokenDurationRecord:
    round_number: int
    turn_index: int
    speaker_label: str
    input_token_units: int
    output_token_units: int
    total_token_units: int
    duration_ms: int
    occurred_at: int


@dataclass(frozen=True)
class TokenDurationTrackingState:
    tracking_version: str
    token_count_basis: str
    duration_measurement_basis: str
    total_rounds_completed: int
    total_turns_tracked: int
    total_input_token_units: int
    total_output_token_units: int
    total_token_units: int
    total_duration_ms: int
    average_turn_duration_ms: int
    turn_records: tuple[TurnTokenDurationRecord, ...]


@dataclass(frozen=True)
class ExecutionPhaseState:
    phase_version: str
    current_phase: str
    phase_changed_at: int
    cap_reason: str | None
    detail_message: str
    finalization_started_at: int | None
    finalization_completed_at: int | None
    report_generation_started_at: int | None
    report_generation_completed_at: int | None
    report_generation_failed_at: int | None
    report_generation_failure_reason: str | None


def estimate_token_units(*text_segments: str) -> int:
    combined = " ".join(segment.strip() for segment in text_segments if segment and segment.strip())
    if not combined:
        return 0
    return max(1, math.ceil(len(combined) / 4))


def initialize_token_duration_tracking(
    tracking_dir: Path,
) -> tuple[Path, TokenDurationTrackingState]:
    tracking_dir.mkdir(parents=True, exist_ok=True)
    tracking_path = tracking_dir / TOKEN_DURATION_TRACKING_FILE_NAME
    state = TokenDurationTrackingState(
        tracking_version="wave5_run_local_tracking_v1",
        token_count_basis="local_char_window_estimate",
        duration_measurement_basis="wall_clock_elapsed_ms",
        total_rounds_completed=0,
        total_turns_tracked=0,
        total_input_token_units=0,
        total_output_token_units=0,
        total_token_units=0,
        total_duration_ms=0,
        average_turn_duration_ms=0,
        turn_records=(),
    )
    write_token_duration_tracking(tracking_path, state)
    return tracking_path, state


def initialize_execution_phase(
    tracking_dir: Path,
    *,
    phase_changed_at: int,
    detail_message: str,
) -> tuple[Path, ExecutionPhaseState]:
    tracking_dir.mkdir(parents=True, exist_ok=True)
    phase_path = tracking_dir / EXECUTION_PHASE_FILE_NAME
    state = ExecutionPhaseState(
        phase_version="wave8_execution_phase_v1",
        current_phase="running_round",
        phase_changed_at=phase_changed_at,
        cap_reason=None,
        detail_message=detail_message,
        finalization_started_at=None,
        finalization_completed_at=None,
        report_generation_started_at=None,
        report_generation_completed_at=None,
        report_generation_failed_at=None,
        report_generation_failure_reason=None,
    )
    write_execution_phase(phase_path, state)
    return phase_path, state


def record_turn_tracking(
    state: TokenDurationTrackingState,
    turn_record: TurnTokenDurationRecord,
) -> TokenDurationTrackingState:
    next_turn_records = (*state.turn_records, turn_record)
    total_duration_ms = state.total_duration_ms + turn_record.duration_ms
    total_turns_tracked = state.total_turns_tracked + 1
    average_turn_duration_ms = total_duration_ms // total_turns_tracked
    return TokenDurationTrackingState(
        tracking_version=state.tracking_version,
        token_count_basis=state.token_count_basis,
        duration_measurement_basis=state.duration_measurement_basis,
        total_rounds_completed=max(state.total_rounds_completed, turn_record.round_number),
        total_turns_tracked=total_turns_tracked,
        total_input_token_units=state.total_input_token_units + turn_record.input_token_units,
        total_output_token_units=state.total_output_token_units + turn_record.output_token_units,
        total_token_units=state.total_token_units + turn_record.total_token_units,
        total_duration_ms=total_duration_ms,
        average_turn_duration_ms=average_turn_duration_ms,
        turn_records=next_turn_records,
    )


def write_token_duration_tracking(
    tracking_path: Path,
    state: TokenDurationTrackingState,
) -> None:
    tracking_path.write_text(json.dumps(asdict(state), indent=2), encoding="utf-8")


def write_execution_phase(
    phase_path: Path,
    state: ExecutionPhaseState,
) -> None:
    phase_path.write_text(json.dumps(asdict(state), indent=2), encoding="utf-8")


def load_token_duration_tracking(tracking_path: Path) -> TokenDurationTrackingState:
    payload = json.loads(tracking_path.read_text(encoding="utf-8"))
    turn_records = tuple(
        TurnTokenDurationRecord(**record) for record in payload.get("turn_records", [])
    )
    return TokenDurationTrackingState(
        tracking_version=payload["tracking_version"],
        token_count_basis=payload["token_count_basis"],
        duration_measurement_basis=payload["duration_measurement_basis"],
        total_rounds_completed=payload["total_rounds_completed"],
        total_turns_tracked=payload["total_turns_tracked"],
        total_input_token_units=payload["total_input_token_units"],
        total_output_token_units=payload["total_output_token_units"],
        total_token_units=payload["total_token_units"],
        total_duration_ms=payload["total_duration_ms"],
        average_turn_duration_ms=payload["average_turn_duration_ms"],
        turn_records=turn_records,
    )


def load_execution_phase(phase_path: Path) -> ExecutionPhaseState:
    payload = json.loads(phase_path.read_text(encoding="utf-8"))
    return ExecutionPhaseState(
        phase_version=payload["phase_version"],
        current_phase=payload["current_phase"],
        phase_changed_at=payload["phase_changed_at"],
        cap_reason=payload.get("cap_reason"),
        detail_message=payload["detail_message"],
        finalization_started_at=payload.get("finalization_started_at"),
        finalization_completed_at=payload.get("finalization_completed_at"),
        report_generation_started_at=payload.get("report_generation_started_at"),
        report_generation_completed_at=payload.get("report_generation_completed_at"),
        report_generation_failed_at=payload.get("report_generation_failed_at"),
        report_generation_failure_reason=payload.get("report_generation_failure_reason"),
    )