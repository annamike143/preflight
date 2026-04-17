use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::run_event_contract::{RunEventRuntimeDomain, RunEventType};
use crate::run_lifecycle_state::project_run_lifecycle_state;
use crate::run_session_contract::RunSessionStateClass;
use crate::run_session_start::{
    load_execution_phase_from_run_session, run_session_state_from_shell,
    run_session_supplemental_events_from_shell, RunSessionStateSnapshot,
};

pub const RUN_EVENT_HISTORY_COMMAND_NAME: &str = "RunEventHistory";
const DEFAULT_MAX_ITEMS: usize = 256;
const MAX_HISTORY_ITEMS: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct RunEventHistoryRequest {
    pub after_sequence_number: Option<u64>,
    pub max_items: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunChronologyEntryKind {
    TranscriptTurn,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunChronologyEntry {
    pub sequence_number: u64,
    pub occurred_at: u64,
    pub entry_kind: RunChronologyEntryKind,
    pub round_number: usize,
    pub turn_index: usize,
    pub speaker_label: String,
    pub speaker_perspective: Option<String>,
    pub turn_text: String,
    pub total_token_units: Option<usize>,
    pub duration_ms: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RunEventEnvelope {
    pub event_id: String,
    pub sequence_number: u64,
    pub occurred_at: u64,
    pub session_id: String,
    pub event_type: RunEventType,
    pub payload: Value,
    pub runtime_domain: RunEventRuntimeDomain,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RunEventHistorySnapshot {
    pub command_name: &'static str,
    pub current_run_id: Option<String>,
    pub current_session_id: Option<String>,
    pub requested_after_sequence_number: Option<u64>,
    pub requested_max_items: usize,
    pub latest_available_sequence_number: Option<u64>,
    pub returned_item_count: usize,
    pub returned_event_count: usize,
    pub returned_chronology_entry_count: usize,
    pub gap_detected: bool,
    pub gap_reason: Option<String>,
    pub stored_history_complete: bool,
    pub event_history_materialized: bool,
    pub transcript_event_chronology_materialized: bool,
    pub live_usage_visibility_materialized: bool,
    pub events: Vec<RunEventEnvelope>,
    pub chronology_entries: Vec<RunChronologyEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunEventHistoryOverview {
    pub latest_available_sequence_number: Option<u64>,
    pub gap_detected: bool,
    pub gap_reason: Option<String>,
    pub event_history_materialized: bool,
    pub transcript_event_chronology_materialized: bool,
    pub live_usage_visibility_materialized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TranscriptArchiveIndexPayload {
    rounds_completed: usize,
    total_turns: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TranscriptTurnPayload {
    round_number: usize,
    turn_index: usize,
    speaker_label: String,
    speaker_perspective: Option<String>,
    turn_text: String,
    total_token_units: Option<usize>,
    duration_ms: Option<usize>,
    occurred_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TokenDurationTrackingPayload {
    total_rounds_completed: usize,
    total_turns_tracked: usize,
    total_input_token_units: usize,
    total_output_token_units: usize,
    total_token_units: usize,
    total_duration_ms: usize,
    average_turn_duration_ms: usize,
    turn_records: Vec<TokenDurationTurnRecordPayload>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TokenDurationTurnRecordPayload {
    round_number: usize,
    turn_index: usize,
    speaker_label: String,
    input_token_units: usize,
    output_token_units: usize,
    total_token_units: usize,
    duration_ms: usize,
    occurred_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PairedTurnRecord {
    transcript: TranscriptTurnPayload,
    tracking: Option<TokenDurationTurnRecordPayload>,
}

#[derive(Debug, Clone, PartialEq)]
enum HistoryItem {
    Event(RunEventEnvelope),
    Chronology(RunChronologyEntry),
}

impl HistoryItem {
    fn sequence_number(&self) -> u64 {
        match self {
            Self::Event(event) => event.sequence_number,
            Self::Chronology(entry) => entry.sequence_number,
        }
    }

    fn occurred_at(&self) -> u64 {
        match self {
            Self::Event(event) => event.occurred_at,
            Self::Chronology(entry) => entry.occurred_at,
        }
    }

    fn assign_sequence(&mut self, session_id: &str, sequence_number: u64) {
        match self {
            Self::Event(event) => {
                event.sequence_number = sequence_number;
                event.event_id = format!("{}-{}", session_id, sequence_number);
            }
            Self::Chronology(entry) => {
                entry.sequence_number = sequence_number;
            }
        }
    }
}

struct SequenceBuilder {
    next_sequence_number: u64,
}

impl SequenceBuilder {
    fn new() -> Self {
        Self {
            next_sequence_number: 1,
        }
    }

    fn next(&mut self) -> u64 {
        let sequence_number = self.next_sequence_number;
        self.next_sequence_number += 1;
        sequence_number
    }
}

struct MaterializedRunEventHistory {
    items: Vec<HistoryItem>,
    gap_reason: Option<String>,
    stored_history_complete: bool,
    event_history_materialized: bool,
    transcript_event_chronology_materialized: bool,
    live_usage_visibility_materialized: bool,
}

pub fn run_event_history_from_shell(request: RunEventHistoryRequest) -> RunEventHistorySnapshot {
    let run_session_state = run_session_state_from_shell();
    let supplemental_events = run_session_supplemental_events_from_shell();
    materialize_run_event_history_with_supplemental(
        &run_session_state,
        &request,
        &supplemental_events,
    )
}

pub fn run_event_history_overview_from_shell() -> RunEventHistoryOverview {
    let run_session_state = run_session_state_from_shell();
    let supplemental_events = run_session_supplemental_events_from_shell();
    run_event_history_overview_with_supplemental(&run_session_state, &supplemental_events)
}

pub fn run_event_history_overview(
    run_session_state: &RunSessionStateSnapshot,
) -> RunEventHistoryOverview {
    run_event_history_overview_with_supplemental(run_session_state, &[])
}

fn run_event_history_overview_with_supplemental(
    run_session_state: &RunSessionStateSnapshot,
    supplemental_events: &[(u64, RunEventType, RunEventRuntimeDomain, Value)],
) -> RunEventHistoryOverview {
    let materialized_history = build_materialized_run_event_history_with_supplemental(
        run_session_state,
        supplemental_events,
    );
    let latest_available_sequence_number = materialized_history
        .items
        .last()
        .map(HistoryItem::sequence_number);

    RunEventHistoryOverview {
        latest_available_sequence_number,
        gap_detected: materialized_history.gap_reason.is_some(),
        gap_reason: materialized_history.gap_reason,
        event_history_materialized: materialized_history.event_history_materialized,
        transcript_event_chronology_materialized: materialized_history
            .transcript_event_chronology_materialized,
        live_usage_visibility_materialized: materialized_history.live_usage_visibility_materialized,
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn materialize_run_event_history(
    run_session_state: &RunSessionStateSnapshot,
    request: &RunEventHistoryRequest,
) -> RunEventHistorySnapshot {
    materialize_run_event_history_with_supplemental(run_session_state, request, &[])
}

fn materialize_run_event_history_with_supplemental(
    run_session_state: &RunSessionStateSnapshot,
    request: &RunEventHistoryRequest,
    supplemental_events: &[(u64, RunEventType, RunEventRuntimeDomain, Value)],
) -> RunEventHistorySnapshot {
    let materialized_history = build_materialized_run_event_history_with_supplemental(
        run_session_state,
        supplemental_events,
    );
    let max_items = sanitize_max_items(request.max_items);
    let after_sequence_number = request.after_sequence_number.unwrap_or(0);
    let latest_available_sequence_number = materialized_history
        .items
        .last()
        .map(HistoryItem::sequence_number);

    let selected_items = materialized_history
        .items
        .into_iter()
        .filter(|item| item.sequence_number() > after_sequence_number)
        .take(max_items)
        .collect::<Vec<_>>();

    let mut events = Vec::new();
    let mut chronology_entries = Vec::new();
    for item in selected_items {
        match item {
            HistoryItem::Event(event) => events.push(event),
            HistoryItem::Chronology(entry) => chronology_entries.push(entry),
        }
    }

    RunEventHistorySnapshot {
        command_name: RUN_EVENT_HISTORY_COMMAND_NAME,
        current_run_id: run_session_state.run_id.clone(),
        current_session_id: run_session_state.session_id.clone(),
        requested_after_sequence_number: request.after_sequence_number,
        requested_max_items: max_items,
        latest_available_sequence_number,
        returned_item_count: events.len() + chronology_entries.len(),
        returned_event_count: events.len(),
        returned_chronology_entry_count: chronology_entries.len(),
        gap_detected: materialized_history.gap_reason.is_some(),
        gap_reason: materialized_history.gap_reason,
        stored_history_complete: materialized_history.stored_history_complete,
        event_history_materialized: materialized_history.event_history_materialized,
        transcript_event_chronology_materialized: materialized_history
            .transcript_event_chronology_materialized,
        live_usage_visibility_materialized: materialized_history.live_usage_visibility_materialized,
        events,
        chronology_entries,
    }
}

fn build_materialized_run_event_history_with_supplemental(
    run_session_state: &RunSessionStateSnapshot,
    supplemental_events: &[(u64, RunEventType, RunEventRuntimeDomain, Value)],
) -> MaterializedRunEventHistory {
    let mut items = Vec::new();
    let mut gap_reason = None;
    let mut stored_history_complete = true;
    let history_pending = ordered_history_still_pending(run_session_state);

    let Some(session_id) = run_session_state.session_id.as_deref() else {
        return MaterializedRunEventHistory {
            items,
            gap_reason,
            stored_history_complete,
            event_history_materialized: true,
            transcript_event_chronology_materialized: true,
            live_usage_visibility_materialized: true,
        };
    };

    let mut sequence_builder = SequenceBuilder::new();
    let lifecycle_state = project_run_lifecycle_state(run_session_state);
    let execution_phase = load_execution_phase_from_run_session(run_session_state);

    if let Some(queued_at) = first_available_timestamp(&[
        run_session_state.workspace_root_path.clone(),
        run_session_state.execution_seed_directory_path.clone(),
        run_session_state.staged_seed_document_path.clone(),
    ]) {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            queued_at,
            RunEventType::RunQueued,
            RunEventRuntimeDomain::Shell,
            json!({
                "run_id": run_session_state.run_id.clone(),
                "workspace_reference": run_session_state.workspace_reference.clone(),
            }),
        )));
    }

    if let Some(ingestion_started_at) = first_available_timestamp(&[
        run_session_state.execution_seed_directory_path.clone(),
        run_session_state.staged_seed_document_path.clone(),
    ]) {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            ingestion_started_at,
            RunEventType::IngestionStarted,
            RunEventRuntimeDomain::Shell,
            json!({
                "workspace_reference": run_session_state.workspace_reference.clone(),
                "seed_fingerprint_sha256": run_session_state.current_seed_fingerprint_sha256.clone(),
            }),
        )));
    }

    if let Some(ingestion_completed_at) = last_available_timestamp(&[
        run_session_state.lineage_manifest_path.clone(),
        run_session_state.staged_seed_document_path.clone(),
    ]) {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            ingestion_completed_at,
            RunEventType::IngestionCompleted,
            RunEventRuntimeDomain::Shell,
            json!({
                "workspace_reference": run_session_state.workspace_reference.clone(),
                "execution_seed_staged": run_session_state.execution_seed_staged,
            }),
        )));
    }

    if let Some(persona_started_at) = first_available_timestamp(&[
        run_session_state.runtime_persona_manifest_path.clone(),
        run_session_state.moderator_basis_path.clone(),
        run_session_state.seed_grounding_memory_path.clone(),
    ]) {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            persona_started_at,
            RunEventType::PersonaGenerationStarted,
            RunEventRuntimeDomain::Engine,
            json!({
                "workspace_reference": run_session_state.workspace_reference,
            }),
        )));
    }

    if let Some(persona_completed_at) = last_available_timestamp(&[
        run_session_state.runtime_persona_manifest_path.clone(),
        run_session_state.moderator_basis_path.clone(),
        run_session_state.seed_grounding_memory_path.clone(),
        run_session_state.round_summary_memory_path.clone(),
        run_session_state.argument_risk_memory_path.clone(),
    ]) {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            persona_completed_at,
            RunEventType::PersonaGenerationCompleted,
            RunEventRuntimeDomain::Engine,
            json!({
                "participant_persona_count": run_session_state.participant_persona_count,
                "moderator_persona_label": run_session_state.moderator_persona_label.clone(),
            }),
        )));
    }

    let transcript_archive_index = match load_json_file::<TranscriptArchiveIndexPayload>(
        run_session_state.transcript_archive_index_path.as_deref(),
    ) {
        Ok(payload) => payload,
        Err(error) => {
            mark_history_incomplete(
                &mut gap_reason,
                &mut stored_history_complete,
                history_pending,
                format!("Transcript archive index could not be loaded. {error}"),
            );
            None
        }
    };

    let transcript_turns = match load_json_lines::<TranscriptTurnPayload>(
        run_session_state.transcript_turns_path.as_deref(),
    ) {
        Ok(turns) => turns,
        Err(error) => {
            mark_history_incomplete(
                &mut gap_reason,
                &mut stored_history_complete,
                history_pending,
                format!("Transcript turn chronology could not be loaded. {error}"),
            );
            Vec::new()
        }
    };

    let token_tracking = match load_json_file::<TokenDurationTrackingPayload>(
        run_session_state.token_duration_tracking_path.as_deref(),
    ) {
        Ok(payload) => payload,
        Err(error) => {
            mark_history_incomplete(
                &mut gap_reason,
                &mut stored_history_complete,
                history_pending,
                format!("Token/duration tracking could not be loaded. {error}"),
            );
            None
        }
    };

    if history_pending
        && (run_session_state.transcript_archive_index_path.is_none()
            || run_session_state.transcript_turns_path.is_none()
            || run_session_state.token_duration_tracking_path.is_none())
    {
        stored_history_complete = false;
    }

    if let Some(index_payload) = transcript_archive_index.as_ref() {
        if index_payload.total_turns != transcript_turns.len() {
            mark_history_incomplete(
                &mut gap_reason,
                &mut stored_history_complete,
                history_pending,
                format!(
                    "Transcript archive index expected {} turns but {} transcript turns were loaded.",
                    index_payload.total_turns,
                    transcript_turns.len()
                ),
            );
        }
        if let Some(completed_round_count) = run_session_state.completed_round_count {
            if index_payload.rounds_completed != completed_round_count {
                mark_history_incomplete(
                    &mut gap_reason,
                    &mut stored_history_complete,
                    history_pending,
                    format!(
                        "Transcript archive index reported {} completed rounds while shell run state reported {}.",
                        index_payload.rounds_completed,
                        completed_round_count
                    ),
                );
            }
        }
    }

    if let Some(tracking_payload) = token_tracking.as_ref() {
        if tracking_payload.total_turns_tracked != tracking_payload.turn_records.len() {
            mark_history_incomplete(
                &mut gap_reason,
                &mut stored_history_complete,
                history_pending,
                format!(
                    "Token/duration tracking reported {} turns while {} tracking records were loaded.",
                    tracking_payload.total_turns_tracked,
                    tracking_payload.turn_records.len()
                ),
            );
        }
        if tracking_payload.total_turns_tracked != transcript_turns.len() {
            mark_history_incomplete(
                &mut gap_reason,
                &mut stored_history_complete,
                history_pending,
                format!(
                    "Transcript turn chronology reported {} turns while token/duration tracking reported {}.",
                    transcript_turns.len(),
                    tracking_payload.total_turns_tracked
                ),
            );
        }
    }

    let paired_turns = pair_turns(
        &transcript_turns,
        token_tracking.as_ref(),
        history_pending,
        &mut gap_reason,
        &mut stored_history_complete,
    );

    let mut grouped_turns = BTreeMap::<usize, Vec<PairedTurnRecord>>::new();
    for paired_turn in paired_turns {
        grouped_turns
            .entry(paired_turn.transcript.round_number)
            .or_default()
            .push(paired_turn);
    }

    let mut cumulative_input_token_units = 0usize;
    let mut cumulative_output_token_units = 0usize;
    let mut cumulative_total_token_units = 0usize;
    let mut cumulative_duration_ms = 0usize;

    for (round_number, round_turns) in grouped_turns {
        let selected_speaker_labels = round_turns
            .iter()
            .map(|turn| turn.transcript.speaker_label.clone())
            .collect::<Vec<_>>();

        let round_started_at = round_turns
            .first()
            .and_then(paired_turn_occurred_at)
            .or_else(|| run_session_state.transcript_turns_path.as_deref().and_then(metadata_timestamp_ms));
        if let Some(round_started_at) = round_started_at {
            items.push(HistoryItem::Event(build_event(
                &mut sequence_builder,
                session_id,
                round_started_at,
                RunEventType::RoundStarted,
                RunEventRuntimeDomain::Debate,
                json!({
                    "round_number": round_number,
                    "speaker_count": selected_speaker_labels.len(),
                    "selected_speaker_labels": selected_speaker_labels,
                }),
            )));
        }

        for paired_turn in &round_turns {
            let Some(occurred_at) = paired_turn_occurred_at(paired_turn) else {
                mark_history_incomplete(
                    &mut gap_reason,
                    &mut stored_history_complete,
                    history_pending,
                    format!(
                        "Transcript turn {}.{} is missing occurred_at evidence in both transcript and tracking artifacts.",
                        paired_turn.transcript.round_number,
                        paired_turn.transcript.turn_index
                    ),
                );
                continue;
            };

            let turn_total_token_units = paired_turn
                .tracking
                .as_ref()
                .map(|record| record.total_token_units)
                .or(paired_turn.transcript.total_token_units);
            let turn_duration_ms = paired_turn
                .tracking
                .as_ref()
                .map(|record| record.duration_ms)
                .or(paired_turn.transcript.duration_ms);

            items.push(HistoryItem::Chronology(RunChronologyEntry {
                sequence_number: sequence_builder.next(),
                occurred_at,
                entry_kind: RunChronologyEntryKind::TranscriptTurn,
                round_number: paired_turn.transcript.round_number,
                turn_index: paired_turn.transcript.turn_index,
                speaker_label: paired_turn.transcript.speaker_label.clone(),
                speaker_perspective: paired_turn.transcript.speaker_perspective.clone(),
                turn_text: paired_turn.transcript.turn_text.clone(),
                total_token_units: turn_total_token_units,
                duration_ms: turn_duration_ms,
            }));

            if let Some(tracking_record) = paired_turn.tracking.as_ref() {
                cumulative_input_token_units += tracking_record.input_token_units;
                cumulative_output_token_units += tracking_record.output_token_units;
                cumulative_total_token_units += tracking_record.total_token_units;
                cumulative_duration_ms += tracking_record.duration_ms;

                items.push(HistoryItem::Event(build_event(
                    &mut sequence_builder,
                    session_id,
                    occurred_at,
                    RunEventType::TokenDurationTracked,
                    RunEventRuntimeDomain::Monitoring,
                    json!({
                        "round_number": tracking_record.round_number,
                        "turn_index": tracking_record.turn_index,
                        "speaker_label": tracking_record.speaker_label,
                        "input_token_units": tracking_record.input_token_units,
                        "output_token_units": tracking_record.output_token_units,
                        "total_token_units": tracking_record.total_token_units,
                        "duration_ms": tracking_record.duration_ms,
                    }),
                )));

                items.push(HistoryItem::Event(build_event(
                    &mut sequence_builder,
                    session_id,
                    occurred_at,
                    RunEventType::BudgetUsageUpdated,
                    RunEventRuntimeDomain::Monitoring,
                    json!({
                        "round_number": tracking_record.round_number,
                        "turn_index": tracking_record.turn_index,
                        "cumulative_input_token_units": cumulative_input_token_units,
                        "cumulative_output_token_units": cumulative_output_token_units,
                        "cumulative_total_token_units": cumulative_total_token_units,
                        "cumulative_duration_ms": cumulative_duration_ms,
                    }),
                )));
            }
        }

        let round_completed_at = round_turns
            .last()
            .and_then(paired_turn_occurred_at)
            .or(round_started_at);
        if let Some(round_completed_at) = round_completed_at {
            let round_summary = if Some(round_number) == run_session_state.completed_round_count {
                run_session_state.latest_round_summary.clone()
            } else {
                None
            };
            items.push(HistoryItem::Event(build_event(
                &mut sequence_builder,
                session_id,
                round_completed_at,
                RunEventType::RoundCompleted,
                RunEventRuntimeDomain::Debate,
                json!({
                    "round_number": round_number,
                    "speaker_count": round_turns.len(),
                        "round_summary": round_summary.clone(),
                }),
            )));
        }
    }

    let runtime_cap_reason = execution_phase
        .as_ref()
        .and_then(|phase| phase.cap_reason)
        .or(run_session_state.runtime_cap_reason);
    if let Some(finalization_started_at) = execution_phase
        .as_ref()
        .and_then(|phase| phase.finalization_started_at)
        .or(run_session_state.finalization_started_at)
    {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            finalization_started_at,
            RunEventType::FinalizationStarted,
            RunEventRuntimeDomain::Engine,
            json!({
                "cap_reason": runtime_cap_reason,
                "detail": execution_phase
                    .as_ref()
                    .map(|phase| phase.detail_message.clone())
                    .unwrap_or_else(|| run_session_state.detail_message.clone()),
            }),
        )));
    }
    if let Some(finalization_completed_at) = execution_phase
        .as_ref()
        .and_then(|phase| phase.finalization_completed_at)
        .or(run_session_state.finalization_completed_at)
    {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            finalization_completed_at,
            RunEventType::FinalizationCompleted,
            RunEventRuntimeDomain::Engine,
            json!({
                "cap_reason": runtime_cap_reason,
                "controlled_finalization_performed": run_session_state.controlled_finalization_performed,
                "detail": "Controlled finalization preserved bounded continuity and completed before report-generation outcome was classified.",
            }),
        )));
    }
    if let Some(report_generation_started_at) = execution_phase
        .as_ref()
        .and_then(|phase| phase.report_generation_started_at)
    {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            report_generation_started_at,
            RunEventType::ReportGenerationStarted,
            RunEventRuntimeDomain::Engine,
            json!({
                "cap_reason": runtime_cap_reason,
                "detail": "Finalization-owned report data assembly started from actual run artifacts.",
            }),
        )));
    }
    if let Some(report_generation_completed_at) = execution_phase
        .as_ref()
        .and_then(|phase| phase.report_generation_completed_at)
    {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            report_generation_completed_at,
            RunEventType::ReportGenerationSucceeded,
            RunEventRuntimeDomain::Engine,
            json!({
                "cap_reason": runtime_cap_reason,
                "detail": execution_phase
                    .as_ref()
                    .map(|phase| phase.detail_message.clone())
                    .unwrap_or_else(|| run_session_state.detail_message.clone()),
            }),
        )));
    }
    if let Some(report_generation_failed_at) = execution_phase
        .as_ref()
        .and_then(|phase| phase.report_generation_failed_at)
    {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            report_generation_failed_at,
            RunEventType::ReportGenerationFailed,
            RunEventRuntimeDomain::Engine,
            json!({
                "cap_reason": runtime_cap_reason,
                "failure_reason": execution_phase
                    .as_ref()
                    .and_then(|phase| phase.report_generation_failure_reason.clone()),
                "detail": execution_phase
                    .as_ref()
                    .map(|phase| phase.detail_message.clone())
                    .unwrap_or_else(|| run_session_state.detail_message.clone()),
            }),
        )));
    }

    let final_occurred_at = items
        .last()
        .map(|item| match item {
            HistoryItem::Event(event) => event.occurred_at,
            HistoryItem::Chronology(entry) => entry.occurred_at,
        })
        .or_else(|| last_available_timestamp(&[
            run_session_state.token_duration_tracking_path.clone(),
            run_session_state.transcript_turns_path.clone(),
            run_session_state.runtime_persona_manifest_path.clone(),
            crate::run_session_start::execution_phase_path_from_run_session(run_session_state),
        ]));

    if let Some(final_occurred_at) = final_occurred_at {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            final_occurred_at,
            RunEventType::RunStateChanged,
            RunEventRuntimeDomain::Shell,
            json!({
                "current_state": lifecycle_state.current_state_name,
                "current_state_terminal": lifecycle_state.current_state_terminal,
                "current_state_cancelable": lifecycle_state.current_state_cancelable,
            }),
        )));
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            final_occurred_at,
            RunEventType::StatusNotePublished,
            RunEventRuntimeDomain::Monitoring,
            json!({
                "status_note": run_session_state.detail_message,
                "current_state": lifecycle_state.current_state_name,
            }),
        )));
    }

    for (occurred_at, event_type, runtime_domain, payload) in supplemental_events.iter().cloned() {
        items.push(HistoryItem::Event(build_event(
            &mut sequence_builder,
            session_id,
            occurred_at,
            event_type,
            runtime_domain,
            payload,
        )));
    }

    sort_and_renumber_history_items(session_id, &mut items);

    MaterializedRunEventHistory {
        items,
        gap_reason,
        stored_history_complete,
        event_history_materialized: true,
        transcript_event_chronology_materialized: true,
        live_usage_visibility_materialized: true,
    }
}

fn pair_turns(
    transcript_turns: &[TranscriptTurnPayload],
    token_tracking: Option<&TokenDurationTrackingPayload>,
    history_pending: bool,
    gap_reason: &mut Option<String>,
    stored_history_complete: &mut bool,
) -> Vec<PairedTurnRecord> {
    let Some(token_tracking) = token_tracking else {
        return transcript_turns
            .iter()
            .cloned()
            .map(|transcript| PairedTurnRecord {
                transcript,
                tracking: None,
            })
            .collect();
    };

    let mut tracking_by_turn = token_tracking
        .turn_records
        .iter()
        .cloned()
        .map(|record| ((record.round_number, record.turn_index), record))
        .collect::<BTreeMap<_, _>>();

    let mut paired_turns = Vec::new();
    for transcript_turn in transcript_turns.iter().cloned() {
        let key = (transcript_turn.round_number, transcript_turn.turn_index);
        let tracking_record = tracking_by_turn.remove(&key);

        if let Some(tracking_record) = tracking_record.as_ref() {
            if tracking_record.speaker_label != transcript_turn.speaker_label {
                mark_history_incomplete(
                    gap_reason,
                    stored_history_complete,
                    history_pending,
                    format!(
                        "Transcript turn {}.{} belongs to speaker {} while tracking recorded speaker {}.",
                        transcript_turn.round_number,
                        transcript_turn.turn_index,
                        transcript_turn.speaker_label,
                        tracking_record.speaker_label
                    ),
                );
            }
        } else {
            mark_history_incomplete(
                gap_reason,
                stored_history_complete,
                history_pending,
                format!(
                    "Transcript turn {}.{} does not have matching token/duration tracking evidence.",
                    transcript_turn.round_number,
                    transcript_turn.turn_index
                ),
            );
        }

        paired_turns.push(PairedTurnRecord {
            transcript: transcript_turn,
            tracking: tracking_record,
        });
    }

    if !tracking_by_turn.is_empty() {
        mark_history_incomplete(
            gap_reason,
            stored_history_complete,
            history_pending,
            format!(
                "Token/duration tracking contains {} turn records that do not map onto transcript turns.",
                tracking_by_turn.len()
            ),
        );
    }

    paired_turns
}

fn paired_turn_occurred_at(paired_turn: &PairedTurnRecord) -> Option<u64> {
    paired_turn
        .transcript
        .occurred_at
        .or_else(|| paired_turn.tracking.as_ref().and_then(|record| record.occurred_at))
}

fn build_event(
    sequence_builder: &mut SequenceBuilder,
    session_id: &str,
    occurred_at: u64,
    event_type: RunEventType,
    runtime_domain: RunEventRuntimeDomain,
    payload: Value,
) -> RunEventEnvelope {
    let sequence_number = sequence_builder.next();

    RunEventEnvelope {
        event_id: format!("{}-{}", session_id, sequence_number),
        sequence_number,
        occurred_at,
        session_id: session_id.to_string(),
        event_type,
        payload,
        runtime_domain,
    }
}

fn sanitize_max_items(max_items: Option<usize>) -> usize {
    max_items
        .unwrap_or(DEFAULT_MAX_ITEMS)
        .clamp(1, MAX_HISTORY_ITEMS)
}

fn sort_and_renumber_history_items(session_id: &str, items: &mut [HistoryItem]) {
    items.sort_by_key(|item| (item.occurred_at(), item.sequence_number()));
    for (index, item) in items.iter_mut().enumerate() {
        item.assign_sequence(session_id, u64::try_from(index + 1).unwrap_or(u64::MAX));
    }
}

fn ordered_history_still_pending(run_session_state: &RunSessionStateSnapshot) -> bool {
    matches!(
        run_session_state.current_state_class,
        RunSessionStateClass::PendingExecution | RunSessionStateClass::Running
    )
}

fn mark_history_incomplete(
    gap_reason: &mut Option<String>,
    stored_history_complete: &mut bool,
    history_pending: bool,
    note: String,
) {
    *stored_history_complete = false;
    if !history_pending {
        record_gap(gap_reason, note);
    }
}

fn record_gap(gap_reason: &mut Option<String>, note: String) {
    if gap_reason.is_none() {
        *gap_reason = Some(note);
    }
}

fn load_json_file<T>(path: Option<&str>) -> Result<Option<T>, String>
where
    T: for<'de> Deserialize<'de>,
{
    let Some(path) = path else {
        return Ok(None);
    };
    let payload = fs::read_to_string(path)
        .map_err(|error| format!("{} could not be read: {}", path, error))?;
    let value = serde_json::from_str::<T>(&payload)
        .map_err(|error| format!("{} could not be parsed: {}", path, error))?;
    Ok(Some(value))
}

fn load_json_lines<T>(path: Option<&str>) -> Result<Vec<T>, String>
where
    T: for<'de> Deserialize<'de>,
{
    let Some(path) = path else {
        return Ok(Vec::new());
    };
    let payload = fs::read_to_string(path)
        .map_err(|error| format!("{} could not be read: {}", path, error))?;
    payload
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<T>(line)
                .map_err(|error| format!("{} contains invalid JSONL: {}", path, error))
        })
        .collect()
}

fn first_available_timestamp(paths: &[Option<String>]) -> Option<u64> {
    paths
        .iter()
        .filter_map(|path| path.as_deref().and_then(metadata_timestamp_ms))
        .min()
}

fn last_available_timestamp(paths: &[Option<String>]) -> Option<u64> {
    paths
        .iter()
        .filter_map(|path| path.as_deref().and_then(metadata_timestamp_ms))
        .max()
}

fn metadata_timestamp_ms(path: &str) -> Option<u64> {
    let metadata = Path::new(path).metadata().ok()?;
    let system_time = metadata.modified().or_else(|_| metadata.created()).ok()?;
    system_time_to_epoch_ms(system_time)
}

fn system_time_to_epoch_ms(system_time: SystemTime) -> Option<u64> {
    system_time
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    use tempfile::TempDir;

    use crate::run_session_contract::RunSessionStateClass;
    use crate::run_session_start::{
        RunCapReason, RunLifecycleOverrideState, RunSessionStateSnapshot,
        RunSessionStateStage,
    };

    fn current_epoch_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
            .unwrap_or(0)
    }

    fn finished_snapshot_with_artifacts(temp_dir: &TempDir) -> RunSessionStateSnapshot {
        let workspace_root = temp_dir.path().join("workspace");
        let execution_seed_directory = workspace_root.join("execution-seed");
        let transcript_archive_directory = workspace_root.join("transcript-archive");
        let tracking_directory = workspace_root.join("run-local-tracking");
        let runtime_persona_manifest_path = workspace_root.join("runtime_personas.json");
        let moderator_basis_path = workspace_root.join("moderator_basis.json");
        let seed_grounding_memory_path = workspace_root.join("seed_grounding.json");
        let round_summary_memory_path = workspace_root.join("round_summary.json");
        let argument_risk_memory_path = workspace_root.join("argument_risk.json");
        let staged_seed_document_path = execution_seed_directory.join("seed.txt");
        let lineage_manifest_path = execution_seed_directory.join("run_start_lineage.json");
        let transcript_archive_index_path = transcript_archive_directory.join("archive_index.json");
        let transcript_turns_path = transcript_archive_directory.join("speaker_turns.jsonl");
        let token_duration_tracking_path = tracking_directory.join("token_duration_tracking.json");

        fs::create_dir_all(&execution_seed_directory).unwrap();
        fs::create_dir_all(&transcript_archive_directory).unwrap();
        fs::create_dir_all(&tracking_directory).unwrap();
        fs::write(&staged_seed_document_path, "seed\n").unwrap();
        fs::write(&lineage_manifest_path, "{}\n").unwrap();
        fs::write(&runtime_persona_manifest_path, "{}\n").unwrap();
        fs::write(&moderator_basis_path, "{}\n").unwrap();
        fs::write(&seed_grounding_memory_path, "{}\n").unwrap();
        fs::write(&round_summary_memory_path, "{}\n").unwrap();
        fs::write(&argument_risk_memory_path, "{}\n").unwrap();
        fs::write(
            &transcript_archive_index_path,
            serde_json::to_string_pretty(&json!({
                "rounds_completed": 1,
                "total_turns": 2,
            }))
            .unwrap(),
        )
        .unwrap();

        let transcript_base_timestamp = current_epoch_ms().saturating_add(1_000);

        let mut transcript_file = fs::File::create(&transcript_turns_path).unwrap();
        writeln!(
            transcript_file,
            "{}",
            serde_json::to_string(&json!({
                "round_number": 1,
                "turn_index": 1,
                "speaker_label": "Advocate",
                "speaker_perspective": "favorable",
                "turn_text": "First bounded turn.",
                "total_token_units": 10,
                "duration_ms": 5,
                "occurred_at": transcript_base_timestamp,
            }))
            .unwrap()
        )
        .unwrap();
        writeln!(
            transcript_file,
            "{}",
            serde_json::to_string(&json!({
                "round_number": 1,
                "turn_index": 2,
                "speaker_label": "Skeptic",
                "speaker_perspective": "skeptical",
                "turn_text": "Second bounded turn.",
                "total_token_units": 12,
                "duration_ms": 7,
                "occurred_at": transcript_base_timestamp + 1_000,
            }))
            .unwrap()
        )
        .unwrap();

        fs::write(
            &token_duration_tracking_path,
            serde_json::to_string_pretty(&json!({
                "total_rounds_completed": 1,
                "total_turns_tracked": 2,
                "total_input_token_units": 11,
                "total_output_token_units": 11,
                "total_token_units": 22,
                "total_duration_ms": 12,
                "average_turn_duration_ms": 6,
                "turn_records": [
                    {
                        "round_number": 1,
                        "turn_index": 1,
                        "speaker_label": "Advocate",
                        "input_token_units": 5,
                        "output_token_units": 5,
                        "total_token_units": 10,
                        "duration_ms": 5,
                        "occurred_at": transcript_base_timestamp,
                    },
                    {
                        "round_number": 1,
                        "turn_index": 2,
                        "speaker_label": "Skeptic",
                        "input_token_units": 6,
                        "output_token_units": 6,
                        "total_token_units": 12,
                        "duration_ms": 7,
                        "occurred_at": transcript_base_timestamp + 1_000,
                    }
                ]
            }))
            .unwrap(),
        )
        .unwrap();

        RunSessionStateSnapshot {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class: RunSessionStateClass::Finished,
            current_state_name: RunSessionStateClass::Finished.label(),
            run_id: Some("run-1".to_string()),
            session_id: Some("session-1".to_string()),
            workspace_reference: Some("workspace-run-1".to_string()),
            workspace_root_path: Some(workspace_root.to_string_lossy().to_string()),
            execution_seed_directory_path: Some(
                execution_seed_directory.to_string_lossy().to_string(),
            ),
            transcript_archive_directory_path: Some(
                transcript_archive_directory.to_string_lossy().to_string(),
            ),
            staged_seed_document_path: Some(staged_seed_document_path.to_string_lossy().to_string()),
            lineage_manifest_path: Some(lineage_manifest_path.to_string_lossy().to_string()),
            current_seed_fingerprint_sha256: Some("seed-hash".to_string()),
            runtime_persona_manifest_path: Some(
                runtime_persona_manifest_path.to_string_lossy().to_string(),
            ),
            moderator_basis_path: Some(moderator_basis_path.to_string_lossy().to_string()),
            bounded_memory_directory_path: Some(workspace_root.join("bounded-memory").to_string_lossy().to_string()),
            seed_grounding_memory_path: Some(seed_grounding_memory_path.to_string_lossy().to_string()),
            round_summary_memory_path: Some(round_summary_memory_path.to_string_lossy().to_string()),
            argument_risk_memory_path: Some(argument_risk_memory_path.to_string_lossy().to_string()),
            transcript_archive_index_path: Some(transcript_archive_index_path.to_string_lossy().to_string()),
            transcript_turns_path: Some(transcript_turns_path.to_string_lossy().to_string()),
            token_duration_tracking_path: Some(token_duration_tracking_path.to_string_lossy().to_string()),
            moderator_persona_label: Some("Moderator".to_string()),
            participant_persona_labels: vec!["Advocate".to_string(), "Skeptic".to_string()],
            perspective_coverage: vec!["favorable".to_string(), "skeptical".to_string()],
            participant_persona_count: Some(2),
            requested_round_count: Some(1),
            completed_round_count: Some(1),
            speaker_counts_per_round: vec![2],
            total_turn_count: Some(2),
            latest_round_summary: Some("Round summary".to_string()),
            tracked_total_input_token_units: Some(11),
            tracked_total_output_token_units: Some(11),
            tracked_total_token_units: Some(22),
            tracked_total_duration_ms: Some(12),
            average_turn_duration_ms: Some(6),
            runtime_persona_foundation_materialized: true,
            bounded_memory_foundation_materialized: true,
            transcript_archive_separated: true,
            exactly_one_moderator_present: true,
            stable_persona_labels: true,
            latest_start_lineage_captured: true,
            workspace_allocated: true,
            execution_seed_staged: true,
            pending_execution_ready: false,
            moderated_rounds_materialized: true,
            normal_round_speaker_selection_enforced: false,
            speaker_attribution_materialized: true,
            bounded_memory_continuity_mutated: true,
            token_duration_tracking_materialized: true,
            start_blocking_reasons: Vec::new(),
            technical_failure_reason: None,
            detail_message: "Execution completed".to_string(),
            ..RunSessionStateSnapshot::not_started()
        }
    }

    #[test]
    fn event_history_materializes_ordered_events_and_transcript_chronology() {
        let temp_dir = tempfile::tempdir().unwrap();
        let snapshot = finished_snapshot_with_artifacts(&temp_dir);

        let history = materialize_run_event_history(&snapshot, &RunEventHistoryRequest::default());

        assert!(history.event_history_materialized);
        assert!(history.transcript_event_chronology_materialized);
        assert!(history.live_usage_visibility_materialized);
        assert!(!history.gap_detected);
        assert!(history.stored_history_complete);
        assert_eq!(history.returned_event_count, 13);
        assert_eq!(history.returned_chronology_entry_count, 2);
        assert_eq!(history.latest_available_sequence_number, Some(15));
        assert_eq!(history.events.first().unwrap().event_type, RunEventType::RunQueued);
        assert_eq!(history.events.last().unwrap().event_type, RunEventType::StatusNotePublished);
        assert_eq!(history.chronology_entries[0].sequence_number, 7);
        assert_eq!(history.chronology_entries[1].sequence_number, 10);
    }

    #[test]
    fn event_history_reports_gap_when_tracking_is_missing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let snapshot = finished_snapshot_with_artifacts(&temp_dir);
        fs::write(
            snapshot.token_duration_tracking_path.as_ref().unwrap(),
            serde_json::to_string_pretty(&json!({
                "total_rounds_completed": 1,
                "total_turns_tracked": 1,
                "total_input_token_units": 5,
                "total_output_token_units": 5,
                "total_token_units": 10,
                "total_duration_ms": 5,
                "average_turn_duration_ms": 5,
                "turn_records": [
                    {
                        "round_number": 1,
                        "turn_index": 1,
                        "speaker_label": "Advocate",
                        "input_token_units": 5,
                        "output_token_units": 5,
                        "total_token_units": 10,
                        "duration_ms": 5,
                        "occurred_at": 1000,
                    }
                ]
            }))
            .unwrap(),
        )
        .unwrap();

        let history = materialize_run_event_history(&snapshot, &RunEventHistoryRequest::default());

        assert!(history.gap_detected);
        assert!(!history.stored_history_complete);
        assert!(history.gap_reason.unwrap().contains("tracking"));
    }

    #[test]
    fn event_history_materializes_finalization_events_for_capped_completion() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut snapshot = finished_snapshot_with_artifacts(&temp_dir);
        snapshot.lifecycle_override = Some(RunLifecycleOverrideState::CompletedBudgetCapped);
        snapshot.runtime_cap_reason = Some(RunCapReason::TokenBudget);
        snapshot.controlled_finalization_performed = true;
        snapshot.finalization_started_at = Some(current_epoch_ms().saturating_add(2_500));
        snapshot.finalization_completed_at = Some(current_epoch_ms().saturating_add(3_000));
        snapshot.detail_message =
            "Runtime token budget stopped further normal rounds after constrained finalization."
                .to_string();

        let phase_path = std::path::Path::new(snapshot.token_duration_tracking_path.as_ref().unwrap())
            .with_file_name("execution_phase.json");
        fs::write(
            &phase_path,
            serde_json::to_string_pretty(&json!({
                "phase_version": "wave7_execution_phase_v1",
                "current_phase": "completed_budget_capped",
                "phase_changed_at": snapshot.finalization_completed_at,
                "cap_reason": "token_budget",
                "detail_message": snapshot.detail_message,
                "finalization_started_at": snapshot.finalization_started_at,
                "finalization_completed_at": snapshot.finalization_completed_at,
            }))
            .unwrap(),
        )
        .unwrap();

        let history = materialize_run_event_history(&snapshot, &RunEventHistoryRequest::default());

        assert!(history
            .events
            .iter()
            .any(|event| event.event_type == RunEventType::FinalizationStarted));
        assert!(history
            .events
            .iter()
            .any(|event| event.event_type == RunEventType::FinalizationCompleted));
        assert!(history.events.iter().any(|event| {
            event.event_type == RunEventType::RunStateChanged
                && event
                    .payload
                    .get("current_state")
                    .and_then(Value::as_str)
                    == Some("CompletedBudgetCapped")
        }));
    }

    #[test]
    fn event_history_keeps_active_pending_history_distinct_from_gap() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace_root = temp_dir.path().join("workspace");
        let execution_seed_directory = workspace_root.join("execution-seed");
        let transcript_archive_directory = workspace_root.join("transcript-archive");
        let runtime_persona_manifest_path = workspace_root.join("runtime_personas.json");
        let moderator_basis_path = workspace_root.join("moderator_basis.json");
        let seed_grounding_memory_path = workspace_root.join("seed_grounding.json");
        let round_summary_memory_path = workspace_root.join("round_summary.json");
        let argument_risk_memory_path = workspace_root.join("argument_risk.json");
        let staged_seed_document_path = execution_seed_directory.join("seed.txt");
        let lineage_manifest_path = execution_seed_directory.join("run_start_lineage.json");
        let transcript_archive_index_path = transcript_archive_directory.join("archive_index.json");
        let transcript_turns_path = transcript_archive_directory.join("speaker_turns.jsonl");
        let token_duration_tracking_path = workspace_root
            .join("run-local-tracking")
            .join("token_duration_tracking.json");

        fs::create_dir_all(&execution_seed_directory).unwrap();
        fs::create_dir_all(&transcript_archive_directory).unwrap();
        fs::write(&staged_seed_document_path, "seed\n").unwrap();
        fs::write(&lineage_manifest_path, "{}\n").unwrap();
        fs::write(&runtime_persona_manifest_path, "{}\n").unwrap();
        fs::write(&moderator_basis_path, "{}\n").unwrap();
        fs::write(&seed_grounding_memory_path, "{}\n").unwrap();
        fs::write(&round_summary_memory_path, "{}\n").unwrap();
        fs::write(&argument_risk_memory_path, "{}\n").unwrap();

        let snapshot = RunSessionStateSnapshot {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class: RunSessionStateClass::Running,
            current_state_name: RunSessionStateClass::Running.label(),
            run_id: Some("run-1".to_string()),
            session_id: Some("session-1".to_string()),
            workspace_reference: Some("workspace-run-1".to_string()),
            workspace_root_path: Some(workspace_root.to_string_lossy().to_string()),
            execution_seed_directory_path: Some(
                execution_seed_directory.to_string_lossy().to_string(),
            ),
            transcript_archive_directory_path: Some(
                transcript_archive_directory.to_string_lossy().to_string(),
            ),
            staged_seed_document_path: Some(staged_seed_document_path.to_string_lossy().to_string()),
            lineage_manifest_path: Some(lineage_manifest_path.to_string_lossy().to_string()),
            current_seed_fingerprint_sha256: Some("seed-hash".to_string()),
            runtime_persona_manifest_path: Some(
                runtime_persona_manifest_path.to_string_lossy().to_string(),
            ),
            moderator_basis_path: Some(moderator_basis_path.to_string_lossy().to_string()),
            bounded_memory_directory_path: Some(workspace_root.join("bounded-memory").to_string_lossy().to_string()),
            seed_grounding_memory_path: Some(seed_grounding_memory_path.to_string_lossy().to_string()),
            round_summary_memory_path: Some(round_summary_memory_path.to_string_lossy().to_string()),
            argument_risk_memory_path: Some(argument_risk_memory_path.to_string_lossy().to_string()),
            transcript_archive_index_path: Some(transcript_archive_index_path.to_string_lossy().to_string()),
            transcript_turns_path: Some(transcript_turns_path.to_string_lossy().to_string()),
            token_duration_tracking_path: Some(token_duration_tracking_path.to_string_lossy().to_string()),
            moderator_persona_label: Some("Moderator".to_string()),
            participant_persona_labels: vec!["Advocate".to_string(), "Skeptic".to_string()],
            perspective_coverage: vec!["favorable".to_string(), "skeptical".to_string()],
            participant_persona_count: Some(2),
            requested_round_count: Some(4),
            completed_round_count: None,
            speaker_counts_per_round: Vec::new(),
            total_turn_count: None,
            latest_round_summary: None,
            tracked_total_input_token_units: None,
            tracked_total_output_token_units: None,
            tracked_total_token_units: None,
            tracked_total_duration_ms: None,
            average_turn_duration_ms: None,
            runtime_persona_foundation_materialized: true,
            bounded_memory_foundation_materialized: true,
            transcript_archive_separated: true,
            exactly_one_moderator_present: true,
            stable_persona_labels: true,
            latest_start_lineage_captured: true,
            workspace_allocated: true,
            execution_seed_staged: true,
            pending_execution_ready: true,
            moderated_rounds_materialized: false,
            normal_round_speaker_selection_enforced: false,
            speaker_attribution_materialized: false,
            bounded_memory_continuity_mutated: false,
            token_duration_tracking_materialized: false,
            start_blocking_reasons: Vec::new(),
            technical_failure_reason: None,
            detail_message: "One-shot moderated execution is active; ordered history may still be pending while transcript artifacts materialize.".to_string(),
            ..RunSessionStateSnapshot::not_started()
        };

        let history = materialize_run_event_history(&snapshot, &RunEventHistoryRequest::default());

        assert!(!history.gap_detected);
        assert!(!history.stored_history_complete);
        assert_eq!(history.returned_chronology_entry_count, 0);
        assert!(history.returned_event_count >= 7);
        assert_eq!(history.events.first().unwrap().event_type, RunEventType::RunQueued);
        assert_eq!(history.events.last().unwrap().event_type, RunEventType::StatusNotePublished);
    }

    #[test]
    fn event_history_keeps_cleanup_events_visible_after_workspace_cleanup() {
        let occurred_at = current_epoch_ms().saturating_add(5_000);
        let snapshot = RunSessionStateSnapshot {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class: RunSessionStateClass::Finished,
            current_state_name: RunSessionStateClass::Finished.label(),
            run_id: Some("run-1".to_string()),
            session_id: Some("session-1".to_string()),
            cleanup_marker: Some(crate::run_cleanup_contract::RunCleanupMarker::CleanupSucceeded),
            detail_message: "Run resolved as Completed; best-effort terminal cleanup removed transient workspace targets while preserving the durable report artifact."
                .to_string(),
            ..RunSessionStateSnapshot::not_started()
        };

        let history = materialize_run_event_history_with_supplemental(
            &snapshot,
            &RunEventHistoryRequest::default(),
            &[
                (
                    occurred_at,
                    RunEventType::CleanupStarted,
                    RunEventRuntimeDomain::Cleanup,
                    json!({ "cleanup_marker": "CleanupRunning" }),
                ),
                (
                    occurred_at + 1,
                    RunEventType::CleanupCompleted,
                    RunEventRuntimeDomain::Cleanup,
                    json!({ "cleanup_marker": "CleanupSucceeded" }),
                ),
                (
                    occurred_at + 1,
                    RunEventType::RunStateChanged,
                    RunEventRuntimeDomain::Shell,
                    json!({ "current_state": "Cleaned" }),
                ),
                (
                    occurred_at + 1,
                    RunEventType::StatusNotePublished,
                    RunEventRuntimeDomain::Monitoring,
                    json!({
                        "status_note": snapshot.detail_message,
                        "current_state": "Cleaned",
                    }),
                ),
            ],
        );

        assert!(!history.gap_detected);
        assert_eq!(history.returned_event_count, 4);
        assert!(history.events.iter().any(|event| event.event_type == RunEventType::CleanupStarted));
        assert!(history.events.iter().any(|event| event.event_type == RunEventType::CleanupCompleted));
        assert_eq!(history.events.last().unwrap().event_type, RunEventType::StatusNotePublished);
    }
}