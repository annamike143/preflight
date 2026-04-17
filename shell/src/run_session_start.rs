use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex, OnceLock,
};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

use crate::abnormal_termination_residual::update_abnormal_residual_marker_from_snapshot;
use crate::run_cleanup_contract::RunCleanupMarker;
use crate::run_cleanup_execution::{execute_terminal_run_cleanup, RunCleanupExecutionOutcome};
use crate::run_seed_ingestion::{
    stage_execution_seed_from_lineage, RunSeedIngestionFailureReason,
    StagedExecutionSeedSnapshot,
};
use crate::run_session_contract::RunSessionStateClass;
use crate::run_start_lineage::{run_start_lineage_surface, RunStartLineageSnapshot};
use crate::run_start_readiness::{run_start_readiness_truth_surface, RunStartReadinessSnapshot};
use crate::run_workspace_allocation::{
    allocate_fresh_run_workspace, RunWorkspaceAllocationFailureReason,
    RunWorkspaceAllocationSnapshot,
};
use crate::report_storage_boundary::materialize_durable_report_artifact;

static RUN_SESSION_STATE: OnceLock<Mutex<StoredRunSessionState>> = OnceLock::new();
static RUN_SESSION_IDENTIFIER_COUNTER: AtomicU64 = AtomicU64::new(1);

const MAX_EXECUTION_RETRY_ATTEMPTS: usize = 2;
const NETWORK_RETRY_BACKOFF_MS: u64 = 250;
const RATE_LIMIT_RETRY_BACKOFF_MS: u64 = 400;
const ENGINE_CONTROL_POLL_INTERVAL_MS: u64 = 25;
const GRACEFUL_STOP_TIMEOUT_MS: u64 = 500;
const RUN_LOCAL_TRACKING_DIRECTORY_NAME: &str = "run-local-tracking";
const TOKEN_DURATION_TRACKING_FILE_NAME: &str = "token_duration_tracking.json";
const TRANSCRIPT_ARCHIVE_INDEX_FILE_NAME: &str = "archive_index.json";
const TRANSCRIPT_TURNS_FILE_NAME: &str = "speaker_turns.jsonl";
const EXECUTION_CONTROL_FILE_NAME: &str = "execution_control.json";
const EXECUTION_PHASE_FILE_NAME: &str = "execution_phase.json";
const RETRYABLE_NETWORK_FAILURE_PREFIX: &str = "retryable_network:";
const RETRYABLE_RATE_LIMIT_FAILURE_PREFIX: &str = "retryable_rate_limit:";
const CANCELED_FAILURE_PREFIX: &str = "canceled:";
const SHUTDOWN_CANCELED_FAILURE_PREFIX: &str = "shutdown_canceled:";
const DURABLE_ARTIFACT_WRITE_FAILURE_REASON: &str = "durable_artifact_write_failed";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunSessionStateStage {
    StartPathMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunSessionStartDecisionType {
    ExecutionStarted,
    ExecutionCompleted,
    ExecutionFailed,
    ExecutionCanceled,
    StartBlocked,
    TechnicalFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunSessionTechnicalFailureReason {
    StateAccessFailure,
    IdentifierGenerationFailed,
    WorkspaceAllocationFailed,
    SeedIngestionFailed,
    EngineFoundationBootstrapFailed,
    ModeratedExecutionFailed,
    ReportGenerationFailed,
    RetryableExecutionExhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunDegradationReason {
    RateLimit,
    Network,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunCapReason {
    TokenBudget,
    DurationCap,
}

impl RunCapReason {
    pub fn label(self) -> &'static str {
        match self {
            Self::TokenBudget => "runtime token budget",
            Self::DurationCap => "hard duration cap",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunLifecycleOverrideState {
    PausedRateLimit,
    PausedNetwork,
    CompletedBudgetCapped,
    Canceled,
    FailedRetryable,
    FailedTerminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EngineExecutionCompletionState {
    Completed,
    CompletedBudgetCapped,
    FailedTerminal,
}

impl EngineExecutionCompletionState {
    fn label(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::CompletedBudgetCapped => "completed_budget_capped",
            Self::FailedTerminal => "failed_terminal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineExecutionPhaseState {
    RunningRound,
    Finalizing,
    Completed,
    CompletedBudgetCapped,
    FailedTerminal,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EngineExecutionPhaseSnapshot {
    #[serde(rename = "phase_version")]
    _phase_version: String,
    pub current_phase: EngineExecutionPhaseState,
    pub phase_changed_at: u64,
    pub cap_reason: Option<RunCapReason>,
    pub detail_message: String,
    pub finalization_started_at: Option<u64>,
    pub finalization_completed_at: Option<u64>,
    pub report_generation_started_at: Option<u64>,
    pub report_generation_completed_at: Option<u64>,
    pub report_generation_failed_at: Option<u64>,
    pub report_generation_failure_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunCancellationDecisionType {
    Accepted,
    Rejected,
    AlreadyTerminal,
    NoActiveRun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlledShutdownDecisionType {
    Accepted,
    AlreadyInProgress,
    AlreadyTerminal,
    NoActiveRun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EngineModuleKind {
    FoundationBootstrap,
    ModeratedExecution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EngineFailureClassification {
    Retryable(RunDegradationReason),
    Canceled,
    ShutdownCanceled,
    Terminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RunSessionRuntimeFlags {
    lifecycle_override: Option<RunLifecycleOverrideState>,
    degradation_reason: Option<RunDegradationReason>,
    retry_attempt_count: usize,
    max_retry_attempts: usize,
    cancellation_requested: bool,
    cancellation_request_accepted: bool,
    shutdown_requested: bool,
    graceful_shutdown_started: bool,
    forceful_termination_required: bool,
    forceful_termination_executed: bool,
}

impl Default for RunSessionRuntimeFlags {
    fn default() -> Self {
        Self {
            lifecycle_override: None,
            degradation_reason: None,
            retry_attempt_count: 0,
            max_retry_attempts: MAX_EXECUTION_RETRY_ATTEMPTS,
            cancellation_requested: false,
            cancellation_request_accepted: false,
            shutdown_requested: false,
            graceful_shutdown_started: false,
            forceful_termination_required: false,
            forceful_termination_executed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct SupplementalRunEvent {
    occurred_at: u64,
    event_type: crate::run_event_contract::RunEventType,
    runtime_domain: crate::run_event_contract::RunEventRuntimeDomain,
    payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct EngineExecutionControlPayload {
    cancellation_requested: bool,
    shutdown_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct EngineFoundationBootstrapSnapshot {
    runtime_persona_manifest_path: String,
    moderator_basis_path: String,
    bounded_memory_directory_path: String,
    seed_grounding_memory_path: String,
    round_summary_memory_path: String,
    argument_risk_memory_path: String,
    transcript_archive_index_path: String,
    moderator_persona_label: String,
    participant_persona_labels: Vec<String>,
    perspective_coverage: Vec<String>,
    participant_persona_count: usize,
    exactly_one_moderator_present: bool,
    stable_persona_labels: bool,
    runtime_persona_foundation_materialized: bool,
    bounded_memory_foundation_materialized: bool,
    transcript_archive_separated: bool,
    detail_message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct EngineSessionExecutionSnapshot {
    transcript_turns_path: String,
    token_duration_tracking_path: String,
    requested_round_count: usize,
    completed_round_count: usize,
    speaker_counts_per_round: Vec<usize>,
    total_turn_count: usize,
    latest_round_summary: Option<String>,
    tracked_total_input_token_units: usize,
    tracked_total_output_token_units: usize,
    tracked_total_token_units: usize,
    tracked_total_duration_ms: usize,
    average_turn_duration_ms: usize,
    moderated_rounds_materialized: bool,
    normal_round_speaker_selection_enforced: bool,
    speaker_attribution_materialized: bool,
    bounded_memory_continuity_mutated: bool,
    token_duration_tracking_materialized: bool,
    completion_state: EngineExecutionCompletionState,
    runtime_cap_reason: Option<RunCapReason>,
    controlled_finalization_performed: bool,
    finalization_started_at: Option<u64>,
    finalization_completed_at: Option<u64>,
    report_identifier: Option<String>,
    report_generated_at: Option<u64>,
    report_data_artifact_path: Option<String>,
    report_pdf_artifact_path: Option<String>,
    durable_report_artifact_path: Option<String>,
    report_generation_status: String,
    report_generation_failure_reason: Option<String>,
    detail_message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunSessionStateSnapshot {
    pub stage: RunSessionStateStage,
    pub current_state_class: RunSessionStateClass,
    pub current_state_name: &'static str,
    pub run_id: Option<String>,
    pub session_id: Option<String>,
    pub workspace_reference: Option<String>,
    pub workspace_root_path: Option<String>,
    pub execution_seed_directory_path: Option<String>,
    pub transcript_archive_directory_path: Option<String>,
    pub staged_seed_document_path: Option<String>,
    pub lineage_manifest_path: Option<String>,
    pub current_seed_fingerprint_sha256: Option<String>,
    pub runtime_persona_manifest_path: Option<String>,
    pub moderator_basis_path: Option<String>,
    pub bounded_memory_directory_path: Option<String>,
    pub seed_grounding_memory_path: Option<String>,
    pub round_summary_memory_path: Option<String>,
    pub argument_risk_memory_path: Option<String>,
    pub transcript_archive_index_path: Option<String>,
    pub transcript_turns_path: Option<String>,
    pub token_duration_tracking_path: Option<String>,
    pub moderator_persona_label: Option<String>,
    pub participant_persona_labels: Vec<String>,
    pub perspective_coverage: Vec<String>,
    pub participant_persona_count: Option<usize>,
    pub requested_round_count: Option<usize>,
    pub completed_round_count: Option<usize>,
    pub speaker_counts_per_round: Vec<usize>,
    pub total_turn_count: Option<usize>,
    pub latest_round_summary: Option<String>,
    pub tracked_total_input_token_units: Option<usize>,
    pub tracked_total_output_token_units: Option<usize>,
    pub tracked_total_token_units: Option<usize>,
    pub tracked_total_duration_ms: Option<usize>,
    pub average_turn_duration_ms: Option<usize>,
    pub runtime_persona_foundation_materialized: bool,
    pub bounded_memory_foundation_materialized: bool,
    pub transcript_archive_separated: bool,
    pub exactly_one_moderator_present: bool,
    pub stable_persona_labels: bool,
    pub latest_start_lineage_captured: bool,
    pub workspace_allocated: bool,
    pub execution_seed_staged: bool,
    pub pending_execution_ready: bool,
    pub moderated_rounds_materialized: bool,
    pub normal_round_speaker_selection_enforced: bool,
    pub speaker_attribution_materialized: bool,
    pub bounded_memory_continuity_mutated: bool,
    pub token_duration_tracking_materialized: bool,
    pub execution_completion_state: Option<String>,
    pub runtime_cap_reason: Option<RunCapReason>,
    pub controlled_finalization_performed: bool,
    pub finalization_started_at: Option<u64>,
    pub finalization_completed_at: Option<u64>,
    pub report_identifier: Option<String>,
    pub report_generated_at: Option<u64>,
    pub durable_report_artifact_path: Option<String>,
    pub report_generation_status: Option<String>,
    pub report_generation_failure_reason: Option<String>,
    pub cleanup_marker: Option<RunCleanupMarker>,
    pub cleanup_started_at: Option<u64>,
    pub cleanup_completed_at: Option<u64>,
    pub cleanup_removed_target_count: usize,
    pub cleanup_failed_target_count: usize,
    pub cleanup_failure_reasons: Vec<String>,
    pub start_blocking_reasons: Vec<String>,
    pub technical_failure_reason: Option<RunSessionTechnicalFailureReason>,
    pub lifecycle_override: Option<RunLifecycleOverrideState>,
    pub degradation_reason: Option<RunDegradationReason>,
    pub retry_attempt_count: usize,
    pub max_retry_attempts: usize,
    pub cancellation_requested: bool,
    pub cancellation_request_accepted: bool,
    pub shutdown_requested: bool,
    pub graceful_shutdown_started: bool,
    pub forceful_termination_required: bool,
    pub forceful_termination_executed: bool,
    pub detail_message: String,
}

impl RunSessionStateSnapshot {
    pub fn not_started() -> Self {
        Self {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class: RunSessionStateClass::NotStarted,
            current_state_name: RunSessionStateClass::NotStarted.label(),
            run_id: None,
            session_id: None,
            workspace_reference: None,
            workspace_root_path: None,
            execution_seed_directory_path: None,
            transcript_archive_directory_path: None,
            staged_seed_document_path: None,
            lineage_manifest_path: None,
            current_seed_fingerprint_sha256: None,
            runtime_persona_manifest_path: None,
            moderator_basis_path: None,
            bounded_memory_directory_path: None,
            seed_grounding_memory_path: None,
            round_summary_memory_path: None,
            argument_risk_memory_path: None,
            transcript_archive_index_path: None,
            transcript_turns_path: None,
            token_duration_tracking_path: None,
            moderator_persona_label: None,
            participant_persona_labels: Vec::new(),
            perspective_coverage: Vec::new(),
            participant_persona_count: None,
            requested_round_count: None,
            completed_round_count: None,
            speaker_counts_per_round: Vec::new(),
            total_turn_count: None,
            latest_round_summary: None,
            tracked_total_input_token_units: None,
            tracked_total_output_token_units: None,
            tracked_total_token_units: None,
            tracked_total_duration_ms: None,
            average_turn_duration_ms: None,
            runtime_persona_foundation_materialized: false,
            bounded_memory_foundation_materialized: false,
            transcript_archive_separated: false,
            exactly_one_moderator_present: false,
            stable_persona_labels: false,
            latest_start_lineage_captured: false,
            workspace_allocated: false,
            execution_seed_staged: false,
            pending_execution_ready: false,
            moderated_rounds_materialized: false,
            normal_round_speaker_selection_enforced: false,
            speaker_attribution_materialized: false,
            bounded_memory_continuity_mutated: false,
            token_duration_tracking_materialized: false,
            execution_completion_state: None,
            runtime_cap_reason: None,
            controlled_finalization_performed: false,
            finalization_started_at: None,
            finalization_completed_at: None,
            report_identifier: None,
            report_generated_at: None,
            durable_report_artifact_path: None,
            report_generation_status: None,
            report_generation_failure_reason: None,
            cleanup_marker: None,
            cleanup_started_at: None,
            cleanup_completed_at: None,
            cleanup_removed_target_count: 0,
            cleanup_failed_target_count: 0,
            cleanup_failure_reasons: Vec::new(),
            start_blocking_reasons: Vec::new(),
            technical_failure_reason: None,
            lifecycle_override: None,
            degradation_reason: None,
            retry_attempt_count: 0,
            max_retry_attempts: MAX_EXECUTION_RETRY_ATTEMPTS,
            cancellation_requested: false,
            cancellation_request_accepted: false,
            shutdown_requested: false,
            graceful_shutdown_started: false,
            forceful_termination_required: false,
            forceful_termination_executed: false,
            detail_message:
                "No fresh run/session has been created from current eligible Preflight truth yet"
                    .to_string(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn in_flight(
        current_state_class: RunSessionStateClass,
        run_id: &str,
        session_id: &str,
        run_start_lineage: &RunStartLineageSnapshot,
        workspace: Option<&RunWorkspaceAllocationSnapshot>,
        staged_seed: Option<&StagedExecutionSeedSnapshot>,
        foundation: Option<&EngineFoundationBootstrapSnapshot>,
        detail_message: String,
    ) -> Self {
        let current_seed_fingerprint_sha256 = staged_seed
            .map(|snapshot| snapshot.staged_seed_fingerprint_sha256.clone())
            .or_else(|| {
                run_start_lineage
                    .selected_seed_document
                    .as_ref()
                    .map(|document| document.content_fingerprint_sha256.clone())
            });

        Self {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class,
            current_state_name: current_state_class.label(),
            run_id: Some(run_id.to_string()),
            session_id: Some(session_id.to_string()),
            workspace_reference: workspace.map(|snapshot| snapshot.workspace_reference.clone()),
            workspace_root_path: workspace.map(|snapshot| snapshot.workspace_root_path.clone()),
            execution_seed_directory_path: workspace
                .map(|snapshot| snapshot.execution_seed_directory_path.clone()),
            transcript_archive_directory_path: workspace
                .map(|snapshot| snapshot.transcript_archive_directory_path.clone()),
            staged_seed_document_path: staged_seed
                .map(|snapshot| snapshot.staged_seed_document_path.clone()),
            lineage_manifest_path: staged_seed.map(|snapshot| snapshot.lineage_manifest_path.clone()),
            current_seed_fingerprint_sha256,
            runtime_persona_manifest_path: foundation
                .map(|snapshot| snapshot.runtime_persona_manifest_path.clone()),
            moderator_basis_path: foundation.map(|snapshot| snapshot.moderator_basis_path.clone()),
            bounded_memory_directory_path: foundation
                .map(|snapshot| snapshot.bounded_memory_directory_path.clone()),
            seed_grounding_memory_path: foundation
                .map(|snapshot| snapshot.seed_grounding_memory_path.clone()),
            round_summary_memory_path: foundation
                .map(|snapshot| snapshot.round_summary_memory_path.clone()),
            argument_risk_memory_path: foundation
                .map(|snapshot| snapshot.argument_risk_memory_path.clone()),
            transcript_archive_index_path: foundation
                .map(|snapshot| snapshot.transcript_archive_index_path.clone())
                .or_else(|| workspace.map(derived_transcript_archive_index_path)),
            transcript_turns_path: workspace.map(derived_transcript_turns_path),
            token_duration_tracking_path: workspace.map(derived_token_duration_tracking_path),
            moderator_persona_label: foundation
                .map(|snapshot| snapshot.moderator_persona_label.clone()),
            participant_persona_labels: foundation
                .map(|snapshot| snapshot.participant_persona_labels.clone())
                .unwrap_or_default(),
            perspective_coverage: foundation
                .map(|snapshot| snapshot.perspective_coverage.clone())
                .unwrap_or_default(),
            participant_persona_count: foundation.map(|snapshot| snapshot.participant_persona_count),
            requested_round_count: run_start_lineage
                .effective_plan
                .as_ref()
                .and_then(|plan| usize::try_from(plan.effective_round_count).ok()),
            completed_round_count: None,
            speaker_counts_per_round: Vec::new(),
            total_turn_count: None,
            latest_round_summary: None,
            tracked_total_input_token_units: None,
            tracked_total_output_token_units: None,
            tracked_total_token_units: None,
            tracked_total_duration_ms: None,
            average_turn_duration_ms: None,
            runtime_persona_foundation_materialized: foundation
                .map(|snapshot| snapshot.runtime_persona_foundation_materialized)
                .unwrap_or(false),
            bounded_memory_foundation_materialized: foundation
                .map(|snapshot| snapshot.bounded_memory_foundation_materialized)
                .unwrap_or(false),
            transcript_archive_separated: foundation
                .map(|snapshot| snapshot.transcript_archive_separated)
                .unwrap_or(false),
            exactly_one_moderator_present: foundation
                .map(|snapshot| snapshot.exactly_one_moderator_present)
                .unwrap_or(false),
            stable_persona_labels: foundation
                .map(|snapshot| snapshot.stable_persona_labels)
                .unwrap_or(false),
            latest_start_lineage_captured: run_start_lineage.lineage_derived_from_current_preflight,
            workspace_allocated: workspace.is_some(),
            execution_seed_staged: staged_seed
                .map(|snapshot| snapshot.raw_seed_bytes_staged)
                .unwrap_or(false),
            pending_execution_ready: foundation.is_some(),
            moderated_rounds_materialized: false,
            normal_round_speaker_selection_enforced: false,
            speaker_attribution_materialized: false,
            bounded_memory_continuity_mutated: false,
            token_duration_tracking_materialized: false,
            execution_completion_state: None,
            runtime_cap_reason: None,
            controlled_finalization_performed: false,
            finalization_started_at: None,
            finalization_completed_at: None,
            report_identifier: None,
            report_generated_at: None,
            durable_report_artifact_path: None,
            report_generation_status: None,
            report_generation_failure_reason: None,
            cleanup_marker: None,
            cleanup_started_at: None,
            cleanup_completed_at: None,
            cleanup_removed_target_count: 0,
            cleanup_failed_target_count: 0,
            cleanup_failure_reasons: Vec::new(),
            start_blocking_reasons: Vec::new(),
            technical_failure_reason: None,
            lifecycle_override: None,
            degradation_reason: None,
            retry_attempt_count: 0,
            max_retry_attempts: MAX_EXECUTION_RETRY_ATTEMPTS,
            cancellation_requested: false,
            cancellation_request_accepted: false,
            shutdown_requested: false,
            graceful_shutdown_started: false,
            forceful_termination_required: false,
            forceful_termination_executed: false,
            detail_message,
        }
    }

    fn start_blocked(blocking_reasons: Vec<String>, detail_message: String) -> Self {
        Self {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class: RunSessionStateClass::StartBlocked,
            current_state_name: RunSessionStateClass::StartBlocked.label(),
            run_id: None,
            session_id: None,
            workspace_reference: None,
            workspace_root_path: None,
            execution_seed_directory_path: None,
            transcript_archive_directory_path: None,
            staged_seed_document_path: None,
            lineage_manifest_path: None,
            current_seed_fingerprint_sha256: None,
            runtime_persona_manifest_path: None,
            moderator_basis_path: None,
            bounded_memory_directory_path: None,
            seed_grounding_memory_path: None,
            round_summary_memory_path: None,
            argument_risk_memory_path: None,
            transcript_archive_index_path: None,
            transcript_turns_path: None,
            token_duration_tracking_path: None,
            moderator_persona_label: None,
            participant_persona_labels: Vec::new(),
            perspective_coverage: Vec::new(),
            participant_persona_count: None,
            requested_round_count: None,
            completed_round_count: None,
            speaker_counts_per_round: Vec::new(),
            total_turn_count: None,
            latest_round_summary: None,
            tracked_total_input_token_units: None,
            tracked_total_output_token_units: None,
            tracked_total_token_units: None,
            tracked_total_duration_ms: None,
            average_turn_duration_ms: None,
            runtime_persona_foundation_materialized: false,
            bounded_memory_foundation_materialized: false,
            transcript_archive_separated: false,
            exactly_one_moderator_present: false,
            stable_persona_labels: false,
            latest_start_lineage_captured: false,
            workspace_allocated: false,
            execution_seed_staged: false,
            pending_execution_ready: false,
            moderated_rounds_materialized: false,
            normal_round_speaker_selection_enforced: false,
            speaker_attribution_materialized: false,
            bounded_memory_continuity_mutated: false,
            token_duration_tracking_materialized: false,
            execution_completion_state: None,
            runtime_cap_reason: None,
            controlled_finalization_performed: false,
            finalization_started_at: None,
            finalization_completed_at: None,
            report_identifier: None,
            report_generated_at: None,
            durable_report_artifact_path: None,
            report_generation_status: None,
            report_generation_failure_reason: None,
            cleanup_marker: None,
            cleanup_started_at: None,
            cleanup_completed_at: None,
            cleanup_removed_target_count: 0,
            cleanup_failed_target_count: 0,
            cleanup_failure_reasons: Vec::new(),
            start_blocking_reasons: blocking_reasons,
            technical_failure_reason: None,
            lifecycle_override: None,
            degradation_reason: None,
            retry_attempt_count: 0,
            max_retry_attempts: MAX_EXECUTION_RETRY_ATTEMPTS,
            cancellation_requested: false,
            cancellation_request_accepted: false,
            shutdown_requested: false,
            graceful_shutdown_started: false,
            forceful_termination_required: false,
            forceful_termination_executed: false,
            detail_message,
        }
    }

    fn technical_failure(
        technical_failure_reason: RunSessionTechnicalFailureReason,
        detail_message: String,
    ) -> Self {
        Self {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class: RunSessionStateClass::TechnicalFailure,
            current_state_name: RunSessionStateClass::TechnicalFailure.label(),
            run_id: None,
            session_id: None,
            workspace_reference: None,
            workspace_root_path: None,
            execution_seed_directory_path: None,
            transcript_archive_directory_path: None,
            staged_seed_document_path: None,
            lineage_manifest_path: None,
            current_seed_fingerprint_sha256: None,
            runtime_persona_manifest_path: None,
            moderator_basis_path: None,
            bounded_memory_directory_path: None,
            seed_grounding_memory_path: None,
            round_summary_memory_path: None,
            argument_risk_memory_path: None,
            transcript_archive_index_path: None,
            transcript_turns_path: None,
            token_duration_tracking_path: None,
            moderator_persona_label: None,
            participant_persona_labels: Vec::new(),
            perspective_coverage: Vec::new(),
            participant_persona_count: None,
            requested_round_count: None,
            completed_round_count: None,
            speaker_counts_per_round: Vec::new(),
            total_turn_count: None,
            latest_round_summary: None,
            tracked_total_input_token_units: None,
            tracked_total_output_token_units: None,
            tracked_total_token_units: None,
            tracked_total_duration_ms: None,
            average_turn_duration_ms: None,
            runtime_persona_foundation_materialized: false,
            bounded_memory_foundation_materialized: false,
            transcript_archive_separated: false,
            exactly_one_moderator_present: false,
            stable_persona_labels: false,
            latest_start_lineage_captured: false,
            workspace_allocated: false,
            execution_seed_staged: false,
            pending_execution_ready: false,
            moderated_rounds_materialized: false,
            normal_round_speaker_selection_enforced: false,
            speaker_attribution_materialized: false,
            bounded_memory_continuity_mutated: false,
            token_duration_tracking_materialized: false,
            execution_completion_state: None,
            runtime_cap_reason: None,
            controlled_finalization_performed: false,
            finalization_started_at: None,
            finalization_completed_at: None,
            report_identifier: None,
            report_generated_at: None,
            durable_report_artifact_path: None,
            report_generation_status: None,
            report_generation_failure_reason: None,
            cleanup_marker: None,
            cleanup_started_at: None,
            cleanup_completed_at: None,
            cleanup_removed_target_count: 0,
            cleanup_failed_target_count: 0,
            cleanup_failure_reasons: Vec::new(),
            start_blocking_reasons: Vec::new(),
            technical_failure_reason: Some(technical_failure_reason),
            lifecycle_override: Some(RunLifecycleOverrideState::FailedTerminal),
            degradation_reason: None,
            retry_attempt_count: 0,
            max_retry_attempts: MAX_EXECUTION_RETRY_ATTEMPTS,
            cancellation_requested: false,
            cancellation_request_accepted: false,
            shutdown_requested: false,
            graceful_shutdown_started: false,
            forceful_termination_required: false,
            forceful_termination_executed: false,
            detail_message,
        }
    }

    fn finished(
        run_id: String,
        session_id: String,
        workspace: RunWorkspaceAllocationSnapshot,
        staged_seed: StagedExecutionSeedSnapshot,
        foundation: EngineFoundationBootstrapSnapshot,
        execution: EngineSessionExecutionSnapshot,
    ) -> Self {
        let (current_state_class, technical_failure_reason, lifecycle_override) =
            match execution.completion_state {
                EngineExecutionCompletionState::Completed => {
                    (RunSessionStateClass::Finished, None, None)
                }
                EngineExecutionCompletionState::CompletedBudgetCapped => (
                    RunSessionStateClass::Finished,
                    None,
                    Some(RunLifecycleOverrideState::CompletedBudgetCapped),
                ),
                EngineExecutionCompletionState::FailedTerminal => (
                    RunSessionStateClass::TechnicalFailure,
                    Some(RunSessionTechnicalFailureReason::ReportGenerationFailed),
                    Some(RunLifecycleOverrideState::FailedTerminal),
                ),
            };

        Self {
            stage: RunSessionStateStage::StartPathMaterialized,
            current_state_class,
            current_state_name: current_state_class.label(),
            run_id: Some(run_id),
            session_id: Some(session_id),
            workspace_reference: Some(workspace.workspace_reference),
            workspace_root_path: Some(workspace.workspace_root_path),
            execution_seed_directory_path: Some(workspace.execution_seed_directory_path),
            transcript_archive_directory_path: Some(workspace.transcript_archive_directory_path),
            staged_seed_document_path: Some(staged_seed.staged_seed_document_path),
            lineage_manifest_path: Some(staged_seed.lineage_manifest_path),
            current_seed_fingerprint_sha256: Some(staged_seed.staged_seed_fingerprint_sha256),
            runtime_persona_manifest_path: Some(foundation.runtime_persona_manifest_path),
            moderator_basis_path: Some(foundation.moderator_basis_path),
            bounded_memory_directory_path: Some(foundation.bounded_memory_directory_path),
            seed_grounding_memory_path: Some(foundation.seed_grounding_memory_path),
            round_summary_memory_path: Some(foundation.round_summary_memory_path),
            argument_risk_memory_path: Some(foundation.argument_risk_memory_path),
            transcript_archive_index_path: Some(foundation.transcript_archive_index_path),
            transcript_turns_path: Some(execution.transcript_turns_path),
            token_duration_tracking_path: Some(execution.token_duration_tracking_path),
            moderator_persona_label: Some(foundation.moderator_persona_label),
            participant_persona_labels: foundation.participant_persona_labels,
            perspective_coverage: foundation.perspective_coverage,
            participant_persona_count: Some(foundation.participant_persona_count),
            requested_round_count: Some(execution.requested_round_count),
            completed_round_count: Some(execution.completed_round_count),
            speaker_counts_per_round: execution.speaker_counts_per_round,
            total_turn_count: Some(execution.total_turn_count),
            latest_round_summary: execution.latest_round_summary,
            tracked_total_input_token_units: Some(execution.tracked_total_input_token_units),
            tracked_total_output_token_units: Some(execution.tracked_total_output_token_units),
            tracked_total_token_units: Some(execution.tracked_total_token_units),
            tracked_total_duration_ms: Some(execution.tracked_total_duration_ms),
            average_turn_duration_ms: Some(execution.average_turn_duration_ms),
            runtime_persona_foundation_materialized: foundation
                .runtime_persona_foundation_materialized,
            bounded_memory_foundation_materialized: foundation
                .bounded_memory_foundation_materialized,
            transcript_archive_separated: foundation.transcript_archive_separated,
            exactly_one_moderator_present: foundation.exactly_one_moderator_present,
            stable_persona_labels: foundation.stable_persona_labels,
            latest_start_lineage_captured: true,
            workspace_allocated: true,
            execution_seed_staged: staged_seed.raw_seed_bytes_staged,
            pending_execution_ready: false,
            moderated_rounds_materialized: execution.moderated_rounds_materialized,
            normal_round_speaker_selection_enforced: execution
                .normal_round_speaker_selection_enforced,
            speaker_attribution_materialized: execution.speaker_attribution_materialized,
            bounded_memory_continuity_mutated: execution.bounded_memory_continuity_mutated,
            token_duration_tracking_materialized: execution
                .token_duration_tracking_materialized,
            execution_completion_state: Some(execution.completion_state.label().to_string()),
            runtime_cap_reason: execution.runtime_cap_reason,
            controlled_finalization_performed: execution.controlled_finalization_performed,
            finalization_started_at: execution.finalization_started_at,
            finalization_completed_at: execution.finalization_completed_at,
            report_identifier: execution.report_identifier,
            report_generated_at: execution.report_generated_at,
            durable_report_artifact_path: execution.durable_report_artifact_path,
            report_generation_status: Some(execution.report_generation_status),
            report_generation_failure_reason: execution.report_generation_failure_reason,
            cleanup_marker: None,
            cleanup_started_at: None,
            cleanup_completed_at: None,
            cleanup_removed_target_count: 0,
            cleanup_failed_target_count: 0,
            cleanup_failure_reasons: Vec::new(),
            start_blocking_reasons: Vec::new(),
            technical_failure_reason,
            lifecycle_override,
            degradation_reason: None,
            retry_attempt_count: 0,
            max_retry_attempts: MAX_EXECUTION_RETRY_ATTEMPTS,
            cancellation_requested: false,
            cancellation_request_accepted: false,
            shutdown_requested: false,
            graceful_shutdown_started: false,
            forceful_termination_required: false,
            forceful_termination_executed: false,
            detail_message: execution.detail_message,
        }
    }

    fn transitioned_from(
        base: &Self,
        current_state_class: RunSessionStateClass,
        lifecycle_override: Option<RunLifecycleOverrideState>,
        technical_failure_reason: Option<RunSessionTechnicalFailureReason>,
        detail_message: String,
    ) -> Self {
        let mut snapshot = base.clone();
        snapshot.current_state_class = current_state_class;
        snapshot.current_state_name = current_state_class.label();
        snapshot.pending_execution_ready = false;
        snapshot.lifecycle_override = lifecycle_override;
        snapshot.technical_failure_reason = technical_failure_reason;
        snapshot.detail_message = detail_message;
        snapshot
    }
}

pub fn terminal_run_outcome_name_from_snapshot(
    run_session_state: &RunSessionStateSnapshot,
) -> &'static str {
    match run_session_state.lifecycle_override {
        Some(RunLifecycleOverrideState::CompletedBudgetCapped) => "CompletedBudgetCapped",
        Some(RunLifecycleOverrideState::Canceled) => "Canceled",
        Some(RunLifecycleOverrideState::FailedRetryable) => "FailedRetryable",
        Some(RunLifecycleOverrideState::FailedTerminal) => "FailedTerminal",
        Some(RunLifecycleOverrideState::PausedRateLimit)
        | Some(RunLifecycleOverrideState::PausedNetwork) => run_session_state.current_state_name,
        None => match run_session_state.execution_completion_state.as_deref() {
            Some("completed_budget_capped") => "CompletedBudgetCapped",
            Some("failed_terminal") => "FailedTerminal",
            Some("completed") => "Completed",
            _ => match run_session_state.current_state_class {
                RunSessionStateClass::Finished => "Completed",
                RunSessionStateClass::TechnicalFailure => "FailedTerminal",
                _ => run_session_state.current_state_name,
            },
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunSessionStartOutcome {
    pub decision_type: RunSessionStartDecisionType,
    pub run_session_state: RunSessionStateSnapshot,
    pub technical_failure_reason: Option<RunSessionTechnicalFailureReason>,
    pub detail_message: String,
}

impl RunSessionStartOutcome {
    fn execution_started(run_session_state: RunSessionStateSnapshot) -> Self {
        Self {
            decision_type: RunSessionStartDecisionType::ExecutionStarted,
            technical_failure_reason: None,
            detail_message: run_session_state.detail_message.clone(),
            run_session_state,
        }
    }

    fn execution_completed(run_session_state: RunSessionStateSnapshot) -> Self {
        Self {
            decision_type: RunSessionStartDecisionType::ExecutionCompleted,
            technical_failure_reason: None,
            detail_message: run_session_state.detail_message.clone(),
            run_session_state,
        }
    }

    fn execution_failed(run_session_state: RunSessionStateSnapshot) -> Self {
        Self {
            decision_type: RunSessionStartDecisionType::ExecutionFailed,
            technical_failure_reason: run_session_state.technical_failure_reason,
            detail_message: run_session_state.detail_message.clone(),
            run_session_state,
        }
    }

    fn execution_canceled(run_session_state: RunSessionStateSnapshot) -> Self {
        Self {
            decision_type: RunSessionStartDecisionType::ExecutionCanceled,
            technical_failure_reason: None,
            detail_message: run_session_state.detail_message.clone(),
            run_session_state,
        }
    }

    fn start_blocked(run_session_state: RunSessionStateSnapshot) -> Self {
        Self {
            decision_type: RunSessionStartDecisionType::StartBlocked,
            technical_failure_reason: None,
            detail_message: run_session_state.detail_message.clone(),
            run_session_state,
        }
    }

    fn technical_failure(run_session_state: RunSessionStateSnapshot) -> Self {
        Self {
            decision_type: RunSessionStartDecisionType::TechnicalFailure,
            technical_failure_reason: run_session_state.technical_failure_reason,
            detail_message: run_session_state.detail_message.clone(),
            run_session_state,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunCancellationOutcome {
    pub decision_type: RunCancellationDecisionType,
    pub run_id: Option<String>,
    pub session_id: Option<String>,
    pub current_state_name: String,
    pub detail_message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledShutdownRequestOutcome {
    pub decision_type: ControlledShutdownDecisionType,
    pub run_id: Option<String>,
    pub session_id: Option<String>,
    pub detail_message: String,
}

#[derive(Debug, Clone)]
struct StoredRunSessionState {
    latest_snapshot: RunSessionStateSnapshot,
    start_in_progress: bool,
    runtime_flags: RunSessionRuntimeFlags,
    control_file_path: Option<String>,
    active_engine_module: Option<EngineModuleKind>,
    supplemental_events: Vec<SupplementalRunEvent>,
}

impl Default for StoredRunSessionState {
    fn default() -> Self {
        Self {
            latest_snapshot: RunSessionStateSnapshot::not_started(),
            start_in_progress: false,
            runtime_flags: RunSessionRuntimeFlags::default(),
            control_file_path: None,
            active_engine_module: None,
            supplemental_events: Vec::new(),
        }
    }
}

struct RunSessionStartGuard;

impl Drop for RunSessionStartGuard {
    fn drop(&mut self) {
        clear_run_session_start_in_progress();
    }
}

fn apply_runtime_flags(
    mut snapshot: RunSessionStateSnapshot,
    runtime_flags: RunSessionRuntimeFlags,
) -> RunSessionStateSnapshot {
    snapshot.lifecycle_override = runtime_flags.lifecycle_override;
    snapshot.degradation_reason = runtime_flags.degradation_reason;
    snapshot.retry_attempt_count = runtime_flags.retry_attempt_count;
    snapshot.max_retry_attempts = runtime_flags.max_retry_attempts;
    snapshot.cancellation_requested = runtime_flags.cancellation_requested;
    snapshot.cancellation_request_accepted = runtime_flags.cancellation_request_accepted;
    snapshot.shutdown_requested = runtime_flags.shutdown_requested;
    snapshot.graceful_shutdown_started = runtime_flags.graceful_shutdown_started;
    snapshot.forceful_termination_required = runtime_flags.forceful_termination_required;
    snapshot.forceful_termination_executed = runtime_flags.forceful_termination_executed;
    snapshot
}

fn update_shell_state<F, T>(updater: F) -> Result<T, String>
where
    F: FnOnce(&mut StoredRunSessionState) -> T,
{
    run_session_state_store()
        .lock()
        .map_err(|_| "Run/session state could not be accessed from the shell".to_string())
        .map(|mut state| updater(&mut state))
}

fn reset_shell_runtime_context(snapshot: &RunSessionStateSnapshot) -> Result<(), String> {
    update_shell_state(|state| {
        state.latest_snapshot = snapshot.clone();
        state.runtime_flags = RunSessionRuntimeFlags::default();
        state.control_file_path = None;
        state.active_engine_module = None;
        state.supplemental_events.clear();
    })
}

fn materialize_durable_report_artifact_for_execution(
    mut execution: EngineSessionExecutionSnapshot,
) -> Result<EngineSessionExecutionSnapshot, String> {
    if execution.report_generation_status != "succeeded" {
        return Ok(execution);
    }

    let report_identifier = execution
        .report_identifier
        .clone()
        .ok_or_else(|| "shell report mediation could not resolve the generated report identifier".to_string())?;
    let transient_report_pdf_path = execution
        .report_pdf_artifact_path
        .clone()
        .ok_or_else(|| "shell report mediation could not resolve the transient report PDF artifact".to_string())?;
    let durable_report_artifact_path = materialize_durable_report_artifact(
        Path::new(&transient_report_pdf_path),
        &report_identifier,
    )
    .map(|path| path.display().to_string())?;

    execution.durable_report_artifact_path = Some(durable_report_artifact_path.clone());
    execution.detail_message = format!(
        "{} Durable local PDF now exists at {}.",
        execution.detail_message,
        durable_report_artifact_path
    );

    Ok(execution)
}

fn failed_execution_after_durable_report_write(
    execution: EngineSessionExecutionSnapshot,
    detail_message: String,
) -> EngineSessionExecutionSnapshot {
    EngineSessionExecutionSnapshot {
        transcript_turns_path: execution.transcript_turns_path,
        token_duration_tracking_path: execution.token_duration_tracking_path,
        requested_round_count: execution.requested_round_count,
        completed_round_count: execution.completed_round_count,
        speaker_counts_per_round: execution.speaker_counts_per_round,
        total_turn_count: execution.total_turn_count,
        latest_round_summary: execution.latest_round_summary,
        tracked_total_input_token_units: execution.tracked_total_input_token_units,
        tracked_total_output_token_units: execution.tracked_total_output_token_units,
        tracked_total_token_units: execution.tracked_total_token_units,
        tracked_total_duration_ms: execution.tracked_total_duration_ms,
        average_turn_duration_ms: execution.average_turn_duration_ms,
        moderated_rounds_materialized: execution.moderated_rounds_materialized,
        normal_round_speaker_selection_enforced: execution.normal_round_speaker_selection_enforced,
        speaker_attribution_materialized: execution.speaker_attribution_materialized,
        bounded_memory_continuity_mutated: execution.bounded_memory_continuity_mutated,
        token_duration_tracking_materialized: execution.token_duration_tracking_materialized,
        completion_state: EngineExecutionCompletionState::FailedTerminal,
        runtime_cap_reason: execution.runtime_cap_reason,
        controlled_finalization_performed: execution.controlled_finalization_performed,
        finalization_started_at: execution.finalization_started_at,
        finalization_completed_at: execution.finalization_completed_at,
        report_identifier: execution.report_identifier,
        report_generated_at: execution.report_generated_at,
        report_data_artifact_path: execution.report_data_artifact_path,
        report_pdf_artifact_path: execution.report_pdf_artifact_path,
        durable_report_artifact_path: None,
        report_generation_status: "failed".to_string(),
        report_generation_failure_reason: Some(DURABLE_ARTIFACT_WRITE_FAILURE_REASON.to_string()),
        detail_message,
    }
}

fn set_shell_control_file_path(control_file_path: Option<String>) {
    let _ = update_shell_state(|state| {
        state.control_file_path = control_file_path;
    });
}

fn set_shell_active_engine_module(active_engine_module: Option<EngineModuleKind>) {
    let _ = update_shell_state(|state| {
        state.active_engine_module = active_engine_module;
    });
}

fn current_epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}

fn update_shell_runtime_flags<F>(updater: F)
where
    F: FnOnce(&mut RunSessionRuntimeFlags),
{
    let _ = update_shell_state(|state| {
        updater(&mut state.runtime_flags);
        state.latest_snapshot = apply_runtime_flags(state.latest_snapshot.clone(), state.runtime_flags);
    });
}

fn append_shell_event(
    event_type: crate::run_event_contract::RunEventType,
    runtime_domain: crate::run_event_contract::RunEventRuntimeDomain,
    payload: Value,
) {
    append_shell_event_at(current_epoch_ms(), event_type, runtime_domain, payload);
}

fn append_shell_event_at(
    occurred_at: u64,
    event_type: crate::run_event_contract::RunEventType,
    runtime_domain: crate::run_event_contract::RunEventRuntimeDomain,
    payload: Value,
) {
    let _ = update_shell_state(|state| {
        state.supplemental_events.push(SupplementalRunEvent {
            occurred_at,
            event_type,
            runtime_domain,
            payload,
        });
    });
}

fn current_shell_runtime_flags() -> RunSessionRuntimeFlags {
    update_shell_state(|state| state.runtime_flags).unwrap_or_default()
}

fn current_shell_control_file_path() -> Option<String> {
    update_shell_state(|state| state.control_file_path.clone()).ok().flatten()
}

fn cleanup_canonical_state_name_from_marker(cleanup_marker: RunCleanupMarker) -> &'static str {
    match cleanup_marker {
        RunCleanupMarker::CleanupPending | RunCleanupMarker::CleanupRunning => "Cleaning",
        RunCleanupMarker::CleanupSucceeded => "Cleaned",
        RunCleanupMarker::CleanupPartiallyFailed | RunCleanupMarker::CleanupFailed => "CleanupFailed",
    }
}

fn cleanup_canonical_state_terminal(cleanup_marker: RunCleanupMarker) -> bool {
    let _ = cleanup_marker;
    true
}

fn cleanup_pending_snapshot_from_base(base: &RunSessionStateSnapshot) -> RunSessionStateSnapshot {
    let mut snapshot = base.clone();
    snapshot.cleanup_marker = Some(RunCleanupMarker::CleanupPending);
    snapshot.cleanup_started_at = None;
    snapshot.cleanup_completed_at = None;
    snapshot.cleanup_removed_target_count = 0;
    snapshot.cleanup_failed_target_count = 0;
    snapshot.cleanup_failure_reasons.clear();
    snapshot.detail_message = format!(
        "Run resolved as {}; shell-owned terminal cleanup is pending for transient workspace targets while preserved report artifacts remain outside cleanup scope.",
        terminal_run_outcome_name_from_snapshot(base)
    );
    snapshot
}

fn cleanup_running_snapshot_from_base(
    base: &RunSessionStateSnapshot,
    cleanup_started_at: u64,
) -> RunSessionStateSnapshot {
    let mut snapshot = base.clone();
    snapshot.cleanup_marker = Some(RunCleanupMarker::CleanupRunning);
    snapshot.cleanup_started_at = Some(cleanup_started_at);
    snapshot.cleanup_completed_at = None;
    snapshot.cleanup_removed_target_count = 0;
    snapshot.cleanup_failed_target_count = 0;
    snapshot.cleanup_failure_reasons.clear();
    snapshot.detail_message = format!(
        "Run resolved as {}; shell-owned terminal cleanup is now removing transient workspace targets under shell authority while preserved report artifacts remain outside cleanup scope.",
        terminal_run_outcome_name_from_snapshot(base)
    );
    snapshot
}

fn cleanup_finished_snapshot_from_base(
    base: &RunSessionStateSnapshot,
    cleanup_outcome: &RunCleanupExecutionOutcome,
) -> RunSessionStateSnapshot {
    let mut snapshot = base.clone();
    snapshot.cleanup_marker = Some(cleanup_outcome.cleanup_marker);
    snapshot.cleanup_started_at = Some(cleanup_outcome.started_at);
    snapshot.cleanup_completed_at = Some(cleanup_outcome.completed_at);
    snapshot.cleanup_removed_target_count = cleanup_outcome.removed_targets.len();
    snapshot.cleanup_failed_target_count = cleanup_outcome.failed_targets.len();
    snapshot.cleanup_failure_reasons = cleanup_outcome
        .failed_targets
        .iter()
        .map(|failure| format!("{} ({})", failure.target_path, failure.reason))
        .collect::<Vec<_>>();
    snapshot.detail_message = format!(
        "Run resolved as {}; {}",
        terminal_run_outcome_name_from_snapshot(base),
        cleanup_outcome.detail_message
    );
    if matches!(
        cleanup_outcome.cleanup_marker,
        RunCleanupMarker::CleanupPartiallyFailed | RunCleanupMarker::CleanupFailed
    ) {
        snapshot.detail_message = format!(
            "{} Any residual workspace material will be retried by the next-launch startup janitor and will not be reused as active execution state.",
            snapshot.detail_message
        );
    }
    snapshot
}

fn sync_abnormal_residual_marker(
    run_session_state: &RunSessionStateSnapshot,
    cleanup_handoff_required: bool,
    detail_message: &str,
) {
    let _ = update_abnormal_residual_marker_from_snapshot(
        run_session_state,
        run_session_state.shutdown_requested,
        cleanup_handoff_required,
        detail_message.to_string(),
    );
}

fn append_startup_janitor_pending_event(snapshot: &RunSessionStateSnapshot, occurred_at: u64) {
    append_shell_event_at(
        occurred_at,
        crate::run_event_contract::RunEventType::StartupJanitorPendingNextLaunch,
        crate::run_event_contract::RunEventRuntimeDomain::Janitor,
        json!({
            "run_id": snapshot.run_id,
            "session_id": snapshot.session_id,
            "workspace_reference": snapshot.workspace_reference,
            "workspace_root_path": snapshot.workspace_root_path,
            "cleanup_marker": snapshot.cleanup_marker.map(RunCleanupMarker::name),
            "shutdown_requested": snapshot.shutdown_requested,
            "detail": snapshot.detail_message,
        }),
    );
}

fn append_cleanup_projection_events(snapshot: &RunSessionStateSnapshot, occurred_at: u64) {
    let Some(cleanup_marker) = snapshot.cleanup_marker else {
        return;
    };
    append_shell_event_at(
        occurred_at,
        crate::run_event_contract::RunEventType::RunStateChanged,
        crate::run_event_contract::RunEventRuntimeDomain::Shell,
        json!({
            "current_state": cleanup_canonical_state_name_from_marker(cleanup_marker),
            "current_state_terminal": cleanup_canonical_state_terminal(cleanup_marker),
            "current_state_cancelable": false,
            "run_outcome": terminal_run_outcome_name_from_snapshot(snapshot),
            "cleanup_marker": cleanup_marker.name(),
        }),
    );
    append_shell_event_at(
        occurred_at,
        crate::run_event_contract::RunEventType::StatusNotePublished,
        crate::run_event_contract::RunEventRuntimeDomain::Monitoring,
        json!({
            "status_note": snapshot.detail_message,
            "current_state": cleanup_canonical_state_name_from_marker(cleanup_marker),
            "run_outcome": terminal_run_outcome_name_from_snapshot(snapshot),
            "cleanup_marker": cleanup_marker.name(),
        }),
    );
}

fn append_cleanup_marker_event(
    snapshot: &RunSessionStateSnapshot,
    cleanup_outcome: Option<&RunCleanupExecutionOutcome>,
    occurred_at: u64,
) {
    let Some(cleanup_marker) = snapshot.cleanup_marker else {
        return;
    };
    let event_type = match cleanup_marker {
        RunCleanupMarker::CleanupPending => crate::run_event_contract::RunEventType::CleanupPending,
        RunCleanupMarker::CleanupRunning => crate::run_event_contract::RunEventType::CleanupStarted,
        RunCleanupMarker::CleanupSucceeded => crate::run_event_contract::RunEventType::CleanupCompleted,
        RunCleanupMarker::CleanupPartiallyFailed => {
            crate::run_event_contract::RunEventType::CleanupPartiallyFailed
        }
        RunCleanupMarker::CleanupFailed => crate::run_event_contract::RunEventType::CleanupFailed,
    };

    append_shell_event_at(
        occurred_at,
        event_type,
        crate::run_event_contract::RunEventRuntimeDomain::Cleanup,
        json!({
            "run_id": snapshot.run_id,
            "session_id": snapshot.session_id,
            "workspace_reference": snapshot.workspace_reference,
            "workspace_root_path": snapshot.workspace_root_path,
            "run_outcome": terminal_run_outcome_name_from_snapshot(snapshot),
            "cleanup_marker": cleanup_marker.name(),
            "removed_target_count": cleanup_outcome
                .map(|outcome| outcome.removed_targets.len())
                .unwrap_or(0),
            "failed_target_count": cleanup_outcome
                .map(|outcome| outcome.failed_targets.len())
                .unwrap_or(0),
            "failed_targets": cleanup_outcome
                .map(|outcome| outcome.failed_targets.iter().map(|failure| {
                    json!({
                        "target_path": failure.target_path,
                        "reason": failure.reason,
                    })
                }).collect::<Vec<_>>())
                .unwrap_or_default(),
            "preserved_report_artifact_path": snapshot.durable_report_artifact_path,
            "detail": snapshot.detail_message,
        }),
    );
}

#[allow(clippy::result_large_err)]
fn maybe_run_terminal_cleanup<SnapshotObserver>(
    state: &mut StoredRunSessionState,
    terminal_snapshot: RunSessionStateSnapshot,
    snapshot_observer: &mut SnapshotObserver,
) -> Result<RunSessionStateSnapshot, RunSessionStartOutcome>
where
    SnapshotObserver: FnMut(&RunSessionStateSnapshot) -> Result<(), RunSessionStartOutcome>,
{
    if terminal_snapshot.workspace_root_path.is_none() || terminal_snapshot.cleanup_marker.is_some() {
        return Ok(terminal_snapshot);
    }

    let pending_occurred_at = current_epoch_ms();
    let pending_snapshot = store_snapshot(
        state,
        cleanup_pending_snapshot_from_base(&terminal_snapshot),
        snapshot_observer,
    )?;
    append_cleanup_marker_event(&pending_snapshot, None, pending_occurred_at);
    append_cleanup_projection_events(&pending_snapshot, pending_occurred_at);

    let cleanup_started_at = current_epoch_ms();
    let running_snapshot = store_snapshot(
        state,
        cleanup_running_snapshot_from_base(&pending_snapshot, cleanup_started_at),
        snapshot_observer,
    )?;
    append_cleanup_marker_event(&running_snapshot, None, cleanup_started_at);
    append_cleanup_projection_events(&running_snapshot, cleanup_started_at);

    let cleanup_outcome = execute_terminal_run_cleanup(&running_snapshot);
    let completed_snapshot = cleanup_finished_snapshot_from_base(&running_snapshot, &cleanup_outcome);
    if matches!(
        cleanup_outcome.cleanup_marker,
        RunCleanupMarker::CleanupPartiallyFailed | RunCleanupMarker::CleanupFailed
    ) {
        sync_abnormal_residual_marker(
            &completed_snapshot,
            true,
            "Terminal cleanup left residual workspace material that requires next-launch startup janitor purge rather than active-workspace reuse.",
        );
    }
    let completed_snapshot = store_snapshot(
        state,
        completed_snapshot,
        snapshot_observer,
    )?;
    append_cleanup_marker_event(
        &completed_snapshot,
        Some(&cleanup_outcome),
        cleanup_outcome.completed_at,
    );
    append_cleanup_projection_events(&completed_snapshot, cleanup_outcome.completed_at);
    if matches!(
        cleanup_outcome.cleanup_marker,
        RunCleanupMarker::CleanupPartiallyFailed | RunCleanupMarker::CleanupFailed
    ) {
        append_startup_janitor_pending_event(&completed_snapshot, cleanup_outcome.completed_at);
    }

    Ok(completed_snapshot)
}

pub fn run_session_supplemental_events_from_shell() -> Vec<(u64, crate::run_event_contract::RunEventType, crate::run_event_contract::RunEventRuntimeDomain, Value)> {
    update_shell_state(|state| {
        state
            .supplemental_events
            .iter()
            .cloned()
            .map(|event| (event.occurred_at, event.event_type, event.runtime_domain, event.payload))
            .collect::<Vec<_>>()
    })
    .unwrap_or_default()
}

pub fn run_session_state_from_shell() -> RunSessionStateSnapshot {
    match run_session_state_store().lock() {
        Ok(state) => apply_runtime_flags(state.latest_snapshot.clone(), state.runtime_flags),
        Err(_) => RunSessionStateSnapshot::technical_failure(
            RunSessionTechnicalFailureReason::StateAccessFailure,
            "Run/session state could not be read from the shell".to_string(),
        ),
    }
}

pub fn start_run_session_from_shell() -> RunSessionStartOutcome {
    let start_guard = match begin_run_session_start() {
        Ok(guard) => guard,
        Err(outcome) => return outcome,
    };

    let run_start_readiness = run_start_readiness_truth_surface();
    let run_start_lineage = run_start_lineage_surface();
    if !run_start_readiness.real_run_start_authorized_now {
        let blocking_reasons = run_start_readiness
            .blocking_reasons()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let snapshot = RunSessionStateSnapshot::start_blocked(
            blocking_reasons,
            format!(
                "Run start remained blocked. {}",
                run_start_readiness.current_status_note
            ),
        );
        let _ = reset_shell_runtime_context(&snapshot);
        return RunSessionStartOutcome::start_blocked(snapshot);
    }

    let run_id = match next_unique_identifier("run") {
        Ok(run_id) => run_id,
        Err(reason) => {
            let snapshot = RunSessionStateSnapshot::technical_failure(
                reason,
                "Run/session identifier generation failed inside the shell".to_string(),
            );
            let _ = reset_shell_runtime_context(&snapshot);
            return RunSessionStartOutcome::technical_failure(snapshot);
        }
    };

    let session_id = match next_unique_identifier("session") {
        Ok(session_id) => session_id,
        Err(reason) => {
            let snapshot = RunSessionStateSnapshot::technical_failure(
                reason,
                "Session identifier generation failed inside the shell".to_string(),
            );
            let _ = reset_shell_runtime_context(&snapshot);
            return RunSessionStartOutcome::technical_failure(snapshot);
        }
    };

    let initial_snapshot = RunSessionStateSnapshot::in_flight(
        RunSessionStateClass::PendingExecution,
        &run_id,
        &session_id,
        &run_start_lineage,
        None,
        None,
        None,
        "Fresh shell-owned run/session identity has been allocated; workspace allocation has not completed yet."
            .to_string(),
    );

    if let Err(detail_message) = reset_shell_runtime_context(&initial_snapshot) {
        return RunSessionStartOutcome::technical_failure(RunSessionStateSnapshot::technical_failure(
            RunSessionTechnicalFailureReason::StateAccessFailure,
            detail_message,
        ));
    }

    let worker_readiness = run_start_readiness.clone();
    let worker_lineage = run_start_lineage.clone();
    let worker_run_id = run_id.clone();
    let worker_session_id = session_id.clone();

    thread::spawn(move || {
        let _start_guard = start_guard;
        let mut state = StoredRunSessionState {
            latest_snapshot: initial_snapshot.clone(),
            ..StoredRunSessionState::default()
        };

        let _ = start_run_session_with_components(
            &mut state,
            worker_readiness,
            worker_lineage,
            publish_snapshot_to_shell,
            || Ok(worker_run_id.clone()),
            || Ok(worker_session_id.clone()),
            allocate_fresh_run_workspace,
            stage_execution_seed_from_lineage,
            bootstrap_runtime_foundation_from_workspace,
            execute_moderated_session_from_workspace,
        );
    });

    RunSessionStartOutcome::execution_started(run_session_state_from_shell())
}

#[allow(clippy::too_many_arguments)]
fn start_run_session_with_components<
    SnapshotObserver,
    RunIdGenerator,
    SessionIdGenerator,
    WorkspaceAllocator,
    SeedStager,
    FoundationBootstrap,
    ModeratedExecution,
>(
    state: &mut StoredRunSessionState,
    run_start_readiness: RunStartReadinessSnapshot,
    run_start_lineage: RunStartLineageSnapshot,
    mut snapshot_observer: SnapshotObserver,
    mut run_id_generator: RunIdGenerator,
    mut session_id_generator: SessionIdGenerator,
    workspace_allocator: WorkspaceAllocator,
    seed_stager: SeedStager,
    foundation_bootstrap: FoundationBootstrap,
    mut moderated_execution: ModeratedExecution,
) -> RunSessionStartOutcome
where
    SnapshotObserver: FnMut(&RunSessionStateSnapshot) -> Result<(), RunSessionStartOutcome>,
    RunIdGenerator: FnMut() -> Result<String, RunSessionTechnicalFailureReason>,
    SessionIdGenerator: FnMut() -> Result<String, RunSessionTechnicalFailureReason>,
    WorkspaceAllocator:
        FnOnce(&str) -> Result<RunWorkspaceAllocationSnapshot, RunWorkspaceAllocationFailureReason>,
    SeedStager: FnOnce(
        &RunWorkspaceAllocationSnapshot,
        &RunStartLineageSnapshot,
    ) -> Result<StagedExecutionSeedSnapshot, RunSeedIngestionFailureReason>,
    FoundationBootstrap:
        FnOnce(&RunWorkspaceAllocationSnapshot) -> Result<EngineFoundationBootstrapSnapshot, String>,
    ModeratedExecution:
        FnMut(&RunWorkspaceAllocationSnapshot) -> Result<EngineSessionExecutionSnapshot, String>,
{
    if !run_start_readiness.real_run_start_authorized_now {
        let blocking_reasons = run_start_readiness
            .blocking_reasons()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let snapshot = match store_snapshot(
            state,
            RunSessionStateSnapshot::start_blocked(
                blocking_reasons,
                format!(
                    "Run start remained blocked. {}",
                    run_start_readiness.current_status_note
                ),
            ),
            &mut snapshot_observer,
        ) {
            Ok(snapshot) => snapshot,
            Err(outcome) => return outcome,
        };
        return RunSessionStartOutcome::start_blocked(snapshot);
    }

    let run_id = match run_id_generator() {
        Ok(run_id) => run_id,
        Err(reason) => {
            let snapshot = match store_snapshot(
                state,
                RunSessionStateSnapshot::technical_failure(
                    reason,
                    "Run/session identifier generation failed inside the shell".to_string(),
                ),
                &mut snapshot_observer,
            ) {
                Ok(snapshot) => snapshot,
                Err(outcome) => return outcome,
            };
            return RunSessionStartOutcome::technical_failure(snapshot);
        }
    };

    let session_id = match session_id_generator() {
        Ok(session_id) => session_id,
        Err(reason) => {
            let snapshot = match store_snapshot(
                state,
                RunSessionStateSnapshot::technical_failure(
                    reason,
                    "Session identifier generation failed inside the shell".to_string(),
                ),
                &mut snapshot_observer,
            ) {
                Ok(snapshot) => snapshot,
                Err(outcome) => return outcome,
            };
            return RunSessionStartOutcome::technical_failure(snapshot);
        }
    };

    if let Err(outcome) = store_snapshot(
        state,
        RunSessionStateSnapshot::in_flight(
            RunSessionStateClass::PendingExecution,
            &run_id,
            &session_id,
            &run_start_lineage,
            None,
            None,
            None,
            "Fresh shell-owned run/session identity has been allocated; workspace allocation has not completed yet."
                .to_string(),
        ),
        &mut snapshot_observer,
    ) {
        return outcome;
    }

    let workspace = match workspace_allocator(&run_id) {
        Ok(workspace) => workspace,
        Err(reason) => {
            let snapshot = match store_snapshot(
                state,
                RunSessionStateSnapshot::technical_failure(
                    RunSessionTechnicalFailureReason::WorkspaceAllocationFailed,
                    format!(
                        "Fresh workspace allocation failed inside the shell with {}",
                        reason.label()
                    ),
                ),
                &mut snapshot_observer,
            ) {
                Ok(snapshot) => snapshot,
                Err(outcome) => return outcome,
            };
            return RunSessionStartOutcome::technical_failure(snapshot);
        }
    };

    match initialize_execution_control_file(&workspace.workspace_root_path) {
        Ok(control_file_path) => set_shell_control_file_path(Some(control_file_path)),
        Err(detail_message) => {
            let cleanup_note = cleanup_workspace(&workspace.workspace_root_path);
            let snapshot = match store_snapshot(
                state,
                RunSessionStateSnapshot::technical_failure(
                    RunSessionTechnicalFailureReason::StateAccessFailure,
                    format!(
                        "Execution control coordination could not be initialized. {}. {}",
                        detail_message, cleanup_note
                    ),
                ),
                &mut snapshot_observer,
            ) {
                Ok(snapshot) => snapshot,
                Err(outcome) => return outcome,
            };
            return RunSessionStartOutcome::technical_failure(snapshot);
        }
    }

    if let Err(outcome) = store_snapshot(
        state,
        RunSessionStateSnapshot::in_flight(
            RunSessionStateClass::PendingExecution,
            &run_id,
            &session_id,
            &run_start_lineage,
            Some(&workspace),
            None,
            None,
            "Fresh shell-owned workspace allocation completed; execution-seed staging is now the next bounded checkpoint."
                .to_string(),
        ),
        &mut snapshot_observer,
    ) {
        let cleanup_note = cleanup_workspace(&workspace.workspace_root_path);
        let snapshot = match store_snapshot(
            state,
            RunSessionStateSnapshot::technical_failure(
                RunSessionTechnicalFailureReason::StateAccessFailure,
                format!(
                    "In-flight run/session snapshot publication failed after workspace allocation. {}",
                    cleanup_note
                ),
            ),
            &mut snapshot_observer,
        ) {
            Ok(snapshot) => snapshot,
            Err(_) => return outcome,
        };
        return RunSessionStartOutcome::technical_failure(snapshot);
    }

    sync_abnormal_residual_marker(
        &state.latest_snapshot,
        true,
        "A fresh transient run workspace now exists under shell authority. If the process exits before terminal cleanup completes, startup janitor will classify any leftover workspace as abandoned residue rather than resume it.",
    );

    match stop_requested_outcome(state, &mut snapshot_observer, if current_shell_runtime_flags().shutdown_requested {
        "Shell-supervised shutdown was accepted before execution-seed staging completed, so normal forward run progression stopped before engine bootstrap began."
    } else {
        "Shell-owned cancellation was accepted before execution-seed staging completed, so normal forward run progression stopped before engine bootstrap began."
    }) {
        Ok(Some(outcome)) => return outcome,
        Err(outcome) => return outcome,
        Ok(None) => {}
    }

    let staged_seed = match seed_stager(&workspace, &run_start_lineage) {
        Ok(staged_seed) => staged_seed,
        Err(reason) => {
            let cleanup_note = cleanup_workspace(&workspace.workspace_root_path);
            let snapshot = match store_snapshot(
                state,
                RunSessionStateSnapshot::technical_failure(
                    RunSessionTechnicalFailureReason::SeedIngestionFailed,
                    format!(
                        "Execution seed staging failed inside the shell with {}. {}",
                        reason.label(), cleanup_note
                    ),
                ),
                &mut snapshot_observer,
            ) {
                Ok(snapshot) => snapshot,
                Err(outcome) => return outcome,
            };
            return RunSessionStartOutcome::technical_failure(snapshot);
        }
    };

    if let Err(outcome) = store_snapshot(
        state,
        RunSessionStateSnapshot::in_flight(
            RunSessionStateClass::PendingExecution,
            &run_id,
            &session_id,
            &run_start_lineage,
            Some(&workspace),
            Some(&staged_seed),
            None,
            "Execution seed staging completed under shell authority; runtime persona and bounded-memory foundation bootstrap is the next checkpoint."
                .to_string(),
        ),
        &mut snapshot_observer,
    ) {
        let cleanup_note = cleanup_workspace(&workspace.workspace_root_path);
        let snapshot = match store_snapshot(
            state,
            RunSessionStateSnapshot::technical_failure(
                RunSessionTechnicalFailureReason::StateAccessFailure,
                format!(
                    "In-flight run/session snapshot publication failed after execution-seed staging. {}",
                    cleanup_note
                ),
            ),
            &mut snapshot_observer,
        ) {
            Ok(snapshot) => snapshot,
            Err(_) => return outcome,
        };
        return RunSessionStartOutcome::technical_failure(snapshot);
    }

    match stop_requested_outcome(state, &mut snapshot_observer, if current_shell_runtime_flags().shutdown_requested {
        "Shell-supervised shutdown was accepted after execution-seed staging, so normal forward progression stopped before engine foundation bootstrap completed."
    } else {
        "Shell-owned cancellation was accepted after execution-seed staging, so normal forward progression stopped before engine foundation bootstrap completed."
    }) {
        Ok(Some(outcome)) => return outcome,
        Err(outcome) => return outcome,
        Ok(None) => {}
    }

    let foundation = match foundation_bootstrap(&workspace) {
        Ok(foundation) => foundation,
        Err(detail_message) => {
            match classify_engine_failure(&detail_message) {
                EngineFailureClassification::Canceled
                | EngineFailureClassification::ShutdownCanceled => {
                    let detail_message = if matches!(
                        classify_engine_failure(&detail_message),
                        EngineFailureClassification::ShutdownCanceled
                    ) {
                        "Shell-supervised shutdown stopped engine foundation bootstrap before moderated execution began."
                            .to_string()
                    } else {
                        "Shell-owned cancellation stopped engine foundation bootstrap before moderated execution began."
                            .to_string()
                    };
                    let snapshot = match store_snapshot(
                        state,
                        canceled_snapshot_from_base(&state.latest_snapshot, detail_message),
                        &mut snapshot_observer,
                    ) {
                        Ok(snapshot) => snapshot,
                        Err(outcome) => return outcome,
                    };
                                let snapshot = match maybe_run_terminal_cleanup(
                                    state,
                                    snapshot,
                                    &mut snapshot_observer,
                                ) {
                                    Ok(snapshot) => snapshot,
                                    Err(outcome) => return outcome,
                                };
                                return RunSessionStartOutcome::execution_canceled(snapshot);
                }
                _ => {}
            }
            let cleanup_note = cleanup_workspace(&workspace.workspace_root_path);
            let snapshot = match store_snapshot(
                state,
                RunSessionStateSnapshot::technical_failure(
                    RunSessionTechnicalFailureReason::EngineFoundationBootstrapFailed,
                    format!(
                        "Runtime persona and bounded-memory foundation bootstrap failed inside the engine boundary. {}. {}",
                        detail_message, cleanup_note
                    ),
                ),
                &mut snapshot_observer,
            ) {
                Ok(snapshot) => snapshot,
                Err(outcome) => return outcome,
            };
            return RunSessionStartOutcome::technical_failure(snapshot);
        }
    };

    if let Err(outcome) = store_snapshot(
        state,
        RunSessionStateSnapshot::in_flight(
            RunSessionStateClass::PendingExecution,
            &run_id,
            &session_id,
            &run_start_lineage,
            Some(&workspace),
            Some(&staged_seed),
            Some(&foundation),
            "Runtime persona generation and bounded-memory foundation are ready; one-shot moderated execution handoff is pending."
                .to_string(),
        ),
        &mut snapshot_observer,
    ) {
        let cleanup_note = cleanup_workspace(&workspace.workspace_root_path);
        let snapshot = match store_snapshot(
            state,
            RunSessionStateSnapshot::technical_failure(
                RunSessionTechnicalFailureReason::StateAccessFailure,
                format!(
                    "In-flight run/session snapshot publication failed after engine foundation bootstrap. {}",
                    cleanup_note
                ),
            ),
            &mut snapshot_observer,
        ) {
            Ok(snapshot) => snapshot,
            Err(_) => return outcome,
        };
        return RunSessionStartOutcome::technical_failure(snapshot);
    }

    match stop_requested_outcome(state, &mut snapshot_observer, if current_shell_runtime_flags().shutdown_requested {
        "Shell-supervised shutdown was accepted after runtime persona foundation bootstrap, so moderated execution did not continue."
    } else {
        "Shell-owned cancellation was accepted after runtime persona foundation bootstrap, so moderated execution did not continue."
    }) {
        Ok(Some(outcome)) => return outcome,
        Err(outcome) => return outcome,
        Ok(None) => {}
    }

    let running_snapshot = RunSessionStateSnapshot::in_flight(
        RunSessionStateClass::Running,
        &run_id,
        &session_id,
        &run_start_lineage,
        Some(&workspace),
        Some(&staged_seed),
        Some(&foundation),
        "One-shot moderated execution is active under the current shell-owned run/session identity; ordered history may still be pending while transcript and tracking artifacts materialize."
            .to_string(),
    );

    if let Err(outcome) = store_snapshot(state, running_snapshot.clone(), &mut snapshot_observer) {
        let cleanup_note = cleanup_workspace(&workspace.workspace_root_path);
        let snapshot = match store_snapshot(
            state,
            RunSessionStateSnapshot::technical_failure(
                RunSessionTechnicalFailureReason::StateAccessFailure,
                format!(
                    "In-flight run/session snapshot publication failed before moderated execution could begin. {}",
                    cleanup_note
                ),
            ),
            &mut snapshot_observer,
        ) {
            Ok(snapshot) => snapshot,
            Err(_) => return outcome,
        };
        return RunSessionStartOutcome::technical_failure(snapshot);
    }

    let execution = {
        let mut execution_result: Option<EngineSessionExecutionSnapshot> = None;

        for attempt in 0..=MAX_EXECUTION_RETRY_ATTEMPTS {
            update_shell_runtime_flags(|flags| {
                flags.lifecycle_override = None;
                flags.degradation_reason = None;
                flags.retry_attempt_count = attempt;
            });

            if attempt > 0 {
                let _ = store_snapshot(
                    state,
                    running_snapshot.clone(),
                    &mut snapshot_observer,
                );
            }

            match moderated_execution(&workspace) {
                Ok(execution) => {
                    execution_result = Some(execution);
                    break;
                }
                Err(detail_message) => {
                    match classify_engine_failure(&detail_message) {
                        EngineFailureClassification::Canceled
                        | EngineFailureClassification::ShutdownCanceled => {
                            let detail_message = if matches!(
                                classify_engine_failure(&detail_message),
                                EngineFailureClassification::ShutdownCanceled
                            ) {
                                "Shell-supervised shutdown stopped moderated execution before further debate progression could continue."
                                    .to_string()
                            } else {
                                "Shell-owned cancellation stopped moderated execution before further debate progression could continue."
                                    .to_string()
                            };
                            let snapshot = match store_snapshot(
                                state,
                                canceled_snapshot_from_base(&state.latest_snapshot, detail_message),
                                &mut snapshot_observer,
                            ) {
                                Ok(snapshot) => snapshot,
                                Err(outcome) => return outcome,
                            };
                                                let snapshot = match maybe_run_terminal_cleanup(
                                                    state,
                                                    snapshot,
                                                    &mut snapshot_observer,
                                                ) {
                                                    Ok(snapshot) => snapshot,
                                                    Err(outcome) => return outcome,
                                                };
                                                return RunSessionStartOutcome::execution_canceled(snapshot);
                        }
                        EngineFailureClassification::Retryable(reason)
                            if attempt < MAX_EXECUTION_RETRY_ATTEMPTS =>
                        {
                            let backoff_ms = retry_backoff_ms(reason);
                            update_shell_runtime_flags(|flags| {
                                flags.lifecycle_override = Some(match reason {
                                    RunDegradationReason::RateLimit => {
                                        RunLifecycleOverrideState::PausedRateLimit
                                    }
                                    RunDegradationReason::Network => {
                                        RunLifecycleOverrideState::PausedNetwork
                                    }
                                });
                                flags.degradation_reason = Some(reason);
                                flags.retry_attempt_count = attempt + 1;
                            });
                            append_shell_event(
                                match reason {
                                    RunDegradationReason::RateLimit => {
                                        crate::run_event_contract::RunEventType::PausedRateLimit
                                    }
                                    RunDegradationReason::Network => {
                                        crate::run_event_contract::RunEventType::PausedNetwork
                                    }
                                },
                                crate::run_event_contract::RunEventRuntimeDomain::Control,
                                json!({
                                    "retry_attempt": attempt + 1,
                                    "max_retry_attempts": MAX_EXECUTION_RETRY_ATTEMPTS,
                                    "detail": detail_message,
                                }),
                            );
                            let paused_snapshot = paused_snapshot_from_base(
                                &state.latest_snapshot,
                                reason,
                                attempt + 1,
                                format!(
                                    "Moderated execution hit a {:?} interruption and is paused before bounded retry attempt {} of {}. {}",
                                    reason,
                                    attempt + 1,
                                    MAX_EXECUTION_RETRY_ATTEMPTS,
                                    detail_message
                                ),
                            );
                            if let Err(outcome) =
                                store_snapshot(state, paused_snapshot, &mut snapshot_observer)
                            {
                                return outcome;
                            }

                            match wait_for_retry_backoff(
                                state,
                                &mut snapshot_observer,
                                backoff_ms,
                            ) {
                                Ok(Some(outcome)) => return outcome,
                                Err(outcome) => return outcome,
                                Ok(None) => {}
                            }

                            continue;
                        }
                        EngineFailureClassification::Retryable(reason) => {
                            update_shell_runtime_flags(|flags| {
                                flags.lifecycle_override = Some(RunLifecycleOverrideState::FailedRetryable);
                                flags.degradation_reason = Some(reason);
                                flags.retry_attempt_count = MAX_EXECUTION_RETRY_ATTEMPTS;
                            });
                            append_shell_event(
                                crate::run_event_contract::RunEventType::FailureRecorded,
                                crate::run_event_contract::RunEventRuntimeDomain::Control,
                                json!({
                                    "failure_classification": "failed_retryable",
                                    "degradation_reason": format!("{:?}", reason),
                                    "detail": detail_message,
                                }),
                            );
                            let snapshot = match store_snapshot(
                                state,
                                failed_snapshot_from_base(
                                    &state.latest_snapshot,
                                    RunLifecycleOverrideState::FailedRetryable,
                                    RunSessionTechnicalFailureReason::RetryableExecutionExhausted,
                                    format!(
                                        "Bounded retry attempts were exhausted after a {:?} interruption. {}",
                                        reason, detail_message
                                    ),
                                ),
                                &mut snapshot_observer,
                            ) {
                                Ok(snapshot) => snapshot,
                                Err(outcome) => return outcome,
                            };
                            return RunSessionStartOutcome::technical_failure(snapshot);
                        }
                        EngineFailureClassification::Terminal => {
                            let snapshot = match store_snapshot(
                                state,
                                failed_snapshot_from_base(
                                    &state.latest_snapshot,
                                    RunLifecycleOverrideState::FailedTerminal,
                                    RunSessionTechnicalFailureReason::ModeratedExecutionFailed,
                                    format!(
                                            "Moderated execution failed inside the engine boundary. {}",
                                            detail_message
                                    ),
                                ),
                                &mut snapshot_observer,
                            ) {
                                Ok(snapshot) => snapshot,
                                Err(outcome) => return outcome,
                            };
                                let snapshot = match maybe_run_terminal_cleanup(
                                    state,
                                    snapshot,
                                    &mut snapshot_observer,
                                ) {
                                    Ok(snapshot) => snapshot,
                                    Err(outcome) => return outcome,
                                };
                                return RunSessionStartOutcome::technical_failure(snapshot);
                        }
                    }
                }
            }
        }

        match execution_result.take() {
            Some(execution) => {
                let fallback_execution = execution.clone();
                match materialize_durable_report_artifact_for_execution(execution) {
                    Ok(execution) => execution,
                    Err(error) => failed_execution_after_durable_report_write(
                        fallback_execution,
                        format!(
                            "Moderator-controlled execution generated transient report artifacts, but shell-owned durable report materialization failed. {}",
                            error
                        ),
                    ),
                }
            }
            _ => unreachable!("execution result should be populated before leaving retry loop"),
        }
    };

    let snapshot = match store_snapshot(
        state,
        RunSessionStateSnapshot::finished(
            run_id,
            session_id,
            workspace,
            staged_seed,
            foundation,
            execution,
        ),
        &mut snapshot_observer,
    ) {
        Ok(snapshot) => snapshot,
        Err(outcome) => return outcome,
    };
    sync_abnormal_residual_marker(
        &snapshot,
        true,
        "Terminal run outcome is recorded, but transient workspace cleanup remains best-effort until shell-owned cleanup or next-launch janitor purge removes the abandoned workspace.",
    );
        let snapshot = match maybe_run_terminal_cleanup(state, snapshot, &mut snapshot_observer) {
            Ok(snapshot) => snapshot,
            Err(outcome) => return outcome,
        };
    if matches!(snapshot.current_state_class, RunSessionStateClass::TechnicalFailure) {
        RunSessionStartOutcome::execution_failed(snapshot)
    } else {
        RunSessionStartOutcome::execution_completed(snapshot)
    }
}

fn run_session_state_store() -> &'static Mutex<StoredRunSessionState> {
    RUN_SESSION_STATE.get_or_init(|| Mutex::new(StoredRunSessionState::default()))
}

#[allow(clippy::result_large_err)]
fn begin_run_session_start() -> Result<RunSessionStartGuard, RunSessionStartOutcome> {
    match run_session_state_store().lock() {
        Ok(mut state) => {
            if state.start_in_progress
                || matches!(
                    state.latest_snapshot.current_state_class,
                    RunSessionStateClass::PendingExecution | RunSessionStateClass::Running
                )
            {
                return Err(RunSessionStartOutcome::start_blocked(
                    RunSessionStateSnapshot::start_blocked(
                        vec![
                            "another shell-owned run/session start is already active".to_string(),
                        ],
                        "A shell-owned run/session start is already active; another one-shot start cannot begin until the current run resolves."
                            .to_string(),
                    ),
                ));
            }

            state.start_in_progress = true;
            Ok(RunSessionStartGuard)
        }
        Err(_) => Err(RunSessionStartOutcome::technical_failure(
            RunSessionStateSnapshot::technical_failure(
                RunSessionTechnicalFailureReason::StateAccessFailure,
                "Run/session start could not acquire shell-owned state coordination"
                    .to_string(),
            ),
        )),
    }
}

fn clear_run_session_start_in_progress() {
    if let Ok(mut state) = run_session_state_store().lock() {
        state.start_in_progress = false;
    }
}

#[allow(clippy::result_large_err)]
fn publish_snapshot_to_shell(snapshot: &RunSessionStateSnapshot) -> Result<(), RunSessionStartOutcome> {
    match run_session_state_store().lock() {
        Ok(mut state) => {
            state.latest_snapshot = apply_runtime_flags(snapshot.clone(), state.runtime_flags);
            Ok(())
        }
        Err(_) => Err(RunSessionStartOutcome::technical_failure(
            RunSessionStateSnapshot::technical_failure(
                RunSessionTechnicalFailureReason::StateAccessFailure,
                "Run/session state could not be published to the shell during the active start path"
                    .to_string(),
            ),
        )),
    }
}

#[allow(clippy::result_large_err)]
fn store_snapshot<SnapshotObserver>(
    state: &mut StoredRunSessionState,
    snapshot: RunSessionStateSnapshot,
    snapshot_observer: &mut SnapshotObserver,
) -> Result<RunSessionStateSnapshot, RunSessionStartOutcome>
where
    SnapshotObserver: FnMut(&RunSessionStateSnapshot) -> Result<(), RunSessionStartOutcome>,
{
    state.latest_snapshot = snapshot.clone();
    snapshot_observer(&snapshot)?;
    Ok(snapshot)
}

fn next_unique_identifier(prefix: &str) -> Result<String, RunSessionTechnicalFailureReason> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| RunSessionTechnicalFailureReason::IdentifierGenerationFailed)?
        .as_millis();
    let ordinal = RUN_SESSION_IDENTIFIER_COUNTER.fetch_add(1, Ordering::SeqCst);

    Ok(format!("{prefix}-{timestamp}-{ordinal}"))
}

fn derived_transcript_archive_index_path(workspace: &RunWorkspaceAllocationSnapshot) -> String {
    PathBuf::from(&workspace.transcript_archive_directory_path)
        .join(TRANSCRIPT_ARCHIVE_INDEX_FILE_NAME)
        .to_string_lossy()
        .to_string()
}

fn derived_transcript_turns_path(workspace: &RunWorkspaceAllocationSnapshot) -> String {
    PathBuf::from(&workspace.transcript_archive_directory_path)
        .join(TRANSCRIPT_TURNS_FILE_NAME)
        .to_string_lossy()
        .to_string()
}

fn derived_token_duration_tracking_path(workspace: &RunWorkspaceAllocationSnapshot) -> String {
    PathBuf::from(&workspace.workspace_root_path)
        .join(RUN_LOCAL_TRACKING_DIRECTORY_NAME)
        .join(TOKEN_DURATION_TRACKING_FILE_NAME)
        .to_string_lossy()
        .to_string()
}

pub fn execution_phase_path_from_run_session(
    run_session_state: &RunSessionStateSnapshot,
) -> Option<String> {
    if let Some(token_duration_tracking_path) = run_session_state.token_duration_tracking_path.as_deref() {
        let tracking_path = PathBuf::from(token_duration_tracking_path);
        if let Some(parent) = tracking_path.parent() {
            return Some(
                parent
                    .join(EXECUTION_PHASE_FILE_NAME)
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }

    run_session_state.workspace_root_path.as_deref().map(|workspace_root_path| {
        PathBuf::from(workspace_root_path)
            .join(RUN_LOCAL_TRACKING_DIRECTORY_NAME)
            .join(EXECUTION_PHASE_FILE_NAME)
            .to_string_lossy()
            .to_string()
    })
}

pub fn load_execution_phase_from_run_session(
    run_session_state: &RunSessionStateSnapshot,
) -> Option<EngineExecutionPhaseSnapshot> {
    let phase_path = execution_phase_path_from_run_session(run_session_state)?;
    let payload = fs::read_to_string(&phase_path).ok()?;
    serde_json::from_str::<EngineExecutionPhaseSnapshot>(&payload).ok()
}

fn retry_backoff_ms(reason: RunDegradationReason) -> u64 {
    match reason {
        RunDegradationReason::RateLimit => RATE_LIMIT_RETRY_BACKOFF_MS,
        RunDegradationReason::Network => NETWORK_RETRY_BACKOFF_MS,
    }
}

fn classify_engine_failure(detail_message: &str) -> EngineFailureClassification {
    if detail_message.contains(RETRYABLE_RATE_LIMIT_FAILURE_PREFIX) {
        EngineFailureClassification::Retryable(RunDegradationReason::RateLimit)
    } else if detail_message.contains(RETRYABLE_NETWORK_FAILURE_PREFIX) {
        EngineFailureClassification::Retryable(RunDegradationReason::Network)
    } else if detail_message.contains(SHUTDOWN_CANCELED_FAILURE_PREFIX) {
        EngineFailureClassification::ShutdownCanceled
    } else if detail_message.contains(CANCELED_FAILURE_PREFIX) {
        EngineFailureClassification::Canceled
    } else {
        EngineFailureClassification::Terminal
    }
}

fn initialize_execution_control_file(workspace_root_path: &str) -> Result<String, String> {
    let control_directory_path = PathBuf::from(workspace_root_path).join(RUN_LOCAL_TRACKING_DIRECTORY_NAME);
    fs::create_dir_all(&control_directory_path)
        .map_err(|error| format!("execution control directory could not be created: {error}"))?;
    let control_file_path = control_directory_path.join(EXECUTION_CONTROL_FILE_NAME);
    let payload = EngineExecutionControlPayload {
        cancellation_requested: false,
        shutdown_requested: false,
    };
    fs::write(
        &control_file_path,
        serde_json::to_string_pretty(&payload)
            .map_err(|error| format!("execution control payload could not be encoded: {error}"))?,
    )
    .map_err(|error| format!("execution control file could not be written: {error}"))?;

    Ok(control_file_path.to_string_lossy().to_string())
}

fn write_execution_control_file(
    control_file_path: &str,
    runtime_flags: RunSessionRuntimeFlags,
) -> Result<(), String> {
    let payload = EngineExecutionControlPayload {
        cancellation_requested: runtime_flags.cancellation_request_accepted,
        shutdown_requested: runtime_flags.shutdown_requested,
    };
    fs::write(
        control_file_path,
        serde_json::to_string_pretty(&payload)
            .map_err(|error| format!("execution control payload could not be encoded: {error}"))?,
    )
    .map_err(|error| format!("execution control file could not be updated: {error}"))
}

fn run_session_snapshot_terminal(run_session_state: &RunSessionStateSnapshot) -> bool {
    if matches!(
        load_execution_phase_from_run_session(run_session_state)
            .as_ref()
            .map(|phase| phase.current_phase),
        Some(EngineExecutionPhaseState::Completed)
            | Some(EngineExecutionPhaseState::CompletedBudgetCapped)
            | Some(EngineExecutionPhaseState::FailedTerminal)
    ) {
        return true;
    }

    matches!(
        run_session_state.lifecycle_override,
        Some(
            RunLifecycleOverrideState::CompletedBudgetCapped
                | RunLifecycleOverrideState::Canceled
                | RunLifecycleOverrideState::FailedRetryable
                | RunLifecycleOverrideState::FailedTerminal
        )
    ) || matches!(
        run_session_state.current_state_class,
        RunSessionStateClass::TechnicalFailure | RunSessionStateClass::Finished
    )
}

fn run_session_snapshot_cancelable(run_session_state: &RunSessionStateSnapshot) -> bool {
    if run_session_state.cancellation_request_accepted || run_session_snapshot_terminal(run_session_state) {
        return false;
    }

    if matches!(
        load_execution_phase_from_run_session(run_session_state)
            .as_ref()
            .map(|phase| phase.current_phase),
        Some(EngineExecutionPhaseState::Finalizing)
    ) {
        return false;
    }

    match run_session_state.lifecycle_override {
        Some(RunLifecycleOverrideState::PausedRateLimit)
        | Some(RunLifecycleOverrideState::PausedNetwork) => true,
        Some(_) => false,
        None => matches!(
            run_session_state.current_state_class,
            RunSessionStateClass::PendingExecution | RunSessionStateClass::Running
        ),
    }
}

fn paused_snapshot_from_base(
    base: &RunSessionStateSnapshot,
    reason: RunDegradationReason,
    retry_attempt_count: usize,
    detail_message: String,
) -> RunSessionStateSnapshot {
    let mut snapshot = RunSessionStateSnapshot::transitioned_from(
        base,
        RunSessionStateClass::Running,
        Some(match reason {
            RunDegradationReason::RateLimit => RunLifecycleOverrideState::PausedRateLimit,
            RunDegradationReason::Network => RunLifecycleOverrideState::PausedNetwork,
        }),
        None,
        detail_message,
    );
    snapshot.degradation_reason = Some(reason);
    snapshot.retry_attempt_count = retry_attempt_count;
    snapshot.max_retry_attempts = MAX_EXECUTION_RETRY_ATTEMPTS;
    snapshot
}

fn canceled_snapshot_from_base(
    base: &RunSessionStateSnapshot,
    detail_message: String,
) -> RunSessionStateSnapshot {
    RunSessionStateSnapshot::transitioned_from(
        base,
        RunSessionStateClass::Finished,
        Some(RunLifecycleOverrideState::Canceled),
        None,
        detail_message,
    )
}

fn failed_snapshot_from_base(
    base: &RunSessionStateSnapshot,
    lifecycle_override: RunLifecycleOverrideState,
    technical_failure_reason: RunSessionTechnicalFailureReason,
    detail_message: String,
) -> RunSessionStateSnapshot {
    RunSessionStateSnapshot::transitioned_from(
        base,
        RunSessionStateClass::TechnicalFailure,
        Some(lifecycle_override),
        Some(technical_failure_reason),
        detail_message,
    )
}

#[allow(clippy::result_large_err)]
fn stop_requested_outcome<SnapshotObserver>(
    state: &mut StoredRunSessionState,
    snapshot_observer: &mut SnapshotObserver,
    detail_message: &str,
) -> Result<Option<RunSessionStartOutcome>, RunSessionStartOutcome>
where
    SnapshotObserver: FnMut(&RunSessionStateSnapshot) -> Result<(), RunSessionStartOutcome>,
{
    let runtime_flags = current_shell_runtime_flags();
    if !runtime_flags.cancellation_request_accepted && !runtime_flags.shutdown_requested {
        return Ok(None);
    }

    let snapshot = store_snapshot(
        state,
        canceled_snapshot_from_base(&state.latest_snapshot, detail_message.to_string()),
        snapshot_observer,
    )?;
    let snapshot = maybe_run_terminal_cleanup(state, snapshot, snapshot_observer)?;
    Ok(Some(RunSessionStartOutcome::execution_canceled(snapshot)))
}

#[allow(clippy::result_large_err)]
fn wait_for_retry_backoff<SnapshotObserver>(
    state: &mut StoredRunSessionState,
    snapshot_observer: &mut SnapshotObserver,
    backoff_ms: u64,
) -> Result<Option<RunSessionStartOutcome>, RunSessionStartOutcome>
where
    SnapshotObserver: FnMut(&RunSessionStateSnapshot) -> Result<(), RunSessionStartOutcome>,
{
    let deadline = Instant::now() + Duration::from_millis(backoff_ms);
    while Instant::now() < deadline {
        if let Some(outcome) = stop_requested_outcome(
            state,
            snapshot_observer,
            if current_shell_runtime_flags().shutdown_requested {
                "Shell-supervised shutdown was accepted while the run was paused for bounded retry, so no further retry attempt was started."
            } else {
                "Shell-owned cancellation was accepted while the run was paused for bounded retry, so no further retry attempt was started."
            },
        )? {
            return Ok(Some(outcome));
        }

        thread::sleep(Duration::from_millis(ENGINE_CONTROL_POLL_INTERVAL_MS));
    }

    Ok(None)
}

pub fn cancel_run_session_from_shell() -> RunCancellationOutcome {
    let run_session_state = run_session_state_from_shell();

    if run_session_state.run_id.is_none() || run_session_state.session_id.is_none() {
        return RunCancellationOutcome {
            decision_type: RunCancellationDecisionType::NoActiveRun,
            run_id: None,
            session_id: None,
            current_state_name: run_session_state.current_state_name.to_string(),
            detail_message: "No shell-owned run/session is currently active, so cancellation could not be requested."
                .to_string(),
        };
    }

    if run_session_snapshot_terminal(&run_session_state) {
        return RunCancellationOutcome {
            decision_type: RunCancellationDecisionType::AlreadyTerminal,
            run_id: run_session_state.run_id,
            session_id: run_session_state.session_id,
            current_state_name: run_session_state.current_state_name.to_string(),
            detail_message: "The current shell-owned run/session is already terminal, so cancellation could not be accepted."
                .to_string(),
        };
    }

    if !run_session_snapshot_cancelable(&run_session_state) {
        return RunCancellationOutcome {
            decision_type: RunCancellationDecisionType::Rejected,
            run_id: run_session_state.run_id,
            session_id: run_session_state.session_id,
            current_state_name: run_session_state.current_state_name.to_string(),
            detail_message: "The current shell-owned run/session is not in a cancelable lifecycle state, so cancellation was rejected."
                .to_string(),
        };
    }

    update_shell_runtime_flags(|flags| {
        flags.cancellation_requested = true;
        flags.cancellation_request_accepted = true;
    });
    let _ = update_shell_state(|state| {
        state.latest_snapshot.detail_message = "Shell-owned cancellation was accepted; the active run is stopping normal forward progression and awaiting engine acknowledgement."
            .to_string();
    });
    if let Some(control_file_path) = current_shell_control_file_path() {
        let _ = write_execution_control_file(&control_file_path, current_shell_runtime_flags());
    }
    append_shell_event(
        crate::run_event_contract::RunEventType::CancellationRequested,
        crate::run_event_contract::RunEventRuntimeDomain::Control,
        json!({
            "run_id": run_session_state.run_id,
            "session_id": run_session_state.session_id,
        }),
    );
    append_shell_event(
        crate::run_event_contract::RunEventType::CancellationAccepted,
        crate::run_event_contract::RunEventRuntimeDomain::Control,
        json!({
            "run_id": run_session_state.run_id,
            "session_id": run_session_state.session_id,
        }),
    );

    let updated_state = run_session_state_from_shell();
    RunCancellationOutcome {
        decision_type: RunCancellationDecisionType::Accepted,
        run_id: updated_state.run_id,
        session_id: updated_state.session_id,
        current_state_name: updated_state.current_state_name.to_string(),
        detail_message: updated_state.detail_message,
    }
}

pub fn request_controlled_shutdown_from_shell() -> ControlledShutdownRequestOutcome {
    let run_session_state = run_session_state_from_shell();

    if run_session_state.run_id.is_none() || run_session_state.session_id.is_none() {
        return ControlledShutdownRequestOutcome {
            decision_type: ControlledShutdownDecisionType::NoActiveRun,
            run_id: None,
            session_id: None,
            detail_message: "No shell-owned run/session is currently active, so controlled shutdown did not need to coordinate an active run."
                .to_string(),
        };
    }

    if run_session_snapshot_terminal(&run_session_state) {
        return ControlledShutdownRequestOutcome {
            decision_type: ControlledShutdownDecisionType::AlreadyTerminal,
            run_id: run_session_state.run_id,
            session_id: run_session_state.session_id,
            detail_message: "The current shell-owned run/session is already terminal, so controlled shutdown did not need to interrupt it."
                .to_string(),
        };
    }

    if current_shell_runtime_flags().shutdown_requested {
        return ControlledShutdownRequestOutcome {
            decision_type: ControlledShutdownDecisionType::AlreadyInProgress,
            run_id: run_session_state.run_id,
            session_id: run_session_state.session_id,
            detail_message: "Shell-supervised shutdown coordination is already in progress for the current run/session."
                .to_string(),
        };
    }

    update_shell_runtime_flags(|flags| {
        flags.shutdown_requested = true;
        flags.graceful_shutdown_started = true;
        flags.cancellation_requested = true;
        flags.cancellation_request_accepted = true;
    });
    let _ = update_shell_state(|state| {
        state.latest_snapshot.detail_message = "Shell-supervised shutdown was accepted; graceful stop was requested first and forceful termination will be used only if the active engine process does not exit in time."
            .to_string();
    });
    if let Some(control_file_path) = current_shell_control_file_path() {
        let _ = write_execution_control_file(&control_file_path, current_shell_runtime_flags());
    }
    append_shell_event(
        crate::run_event_contract::RunEventType::CancellationRequested,
        crate::run_event_contract::RunEventRuntimeDomain::Control,
        json!({
            "run_id": run_session_state.run_id,
            "session_id": run_session_state.session_id,
            "shutdown_requested": true,
        }),
    );
    append_shell_event(
        crate::run_event_contract::RunEventType::CancellationAccepted,
        crate::run_event_contract::RunEventRuntimeDomain::Control,
        json!({
            "run_id": run_session_state.run_id,
            "session_id": run_session_state.session_id,
            "shutdown_requested": true,
        }),
    );

    let updated_state = run_session_state_from_shell();
    sync_abnormal_residual_marker(
        &updated_state,
        true,
        "Shell-supervised shutdown coordination is active for the current workspace. If terminal cleanup cannot complete before the process exits, startup janitor will retry purge on the next launch without resuming the abandoned run.",
    );
    ControlledShutdownRequestOutcome {
        decision_type: ControlledShutdownDecisionType::Accepted,
        run_id: updated_state.run_id,
        session_id: updated_state.session_id,
        detail_message: updated_state.detail_message,
    }
}

pub fn shell_engine_process_active() -> bool {
    update_shell_state(|state| state.active_engine_module.is_some()).unwrap_or(false)
}

pub fn shell_shutdown_coordination_flags() -> (bool, bool, bool, bool) {
    let flags = current_shell_runtime_flags();
    (
        flags.shutdown_requested,
        flags.graceful_shutdown_started,
        flags.forceful_termination_required,
        flags.forceful_termination_executed,
    )
}

fn cleanup_workspace(workspace_root_path: &str) -> String {
    match fs::remove_dir_all(workspace_root_path) {
        Ok(()) => "Any partially allocated workspace was removed".to_string(),
        Err(_) => "A partially allocated workspace could not be removed automatically".to_string(),
    }
}

fn bootstrap_runtime_foundation_from_workspace(
    workspace: &RunWorkspaceAllocationSnapshot,
) -> Result<EngineFoundationBootstrapSnapshot, String> {
    run_engine_module(
        "miro_fish_engine.session_bootstrap",
        &workspace.workspace_root_path,
        "engine bootstrap command",
        EngineModuleKind::FoundationBootstrap,
    )
}

fn execute_moderated_session_from_workspace(
    workspace: &RunWorkspaceAllocationSnapshot,
) -> Result<EngineSessionExecutionSnapshot, String> {
    run_engine_module(
        "miro_fish_engine.session_execution",
        &workspace.workspace_root_path,
        "engine execution command",
        EngineModuleKind::ModeratedExecution,
    )
}

fn run_engine_module<T: DeserializeOwned>(
    module_name: &str,
    workspace_root_path: &str,
    command_name: &str,
    engine_module_kind: EngineModuleKind,
) -> Result<T, String> {
    let engine_python = env::var("MIRO_FISH_ENGINE_PYTHON").unwrap_or_else(|_| "python".to_string());
    let pythonpath = build_engine_pythonpath()?;

    let mut child = Command::new(engine_python)
        .arg("-m")
        .arg(module_name)
        .arg("--workspace-root")
        .arg(workspace_root_path)
        .env("PYTHONPATH", pythonpath)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("{command_name} could not be invoked: {error}"))?;

    set_shell_active_engine_module(Some(engine_module_kind));

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| format!("{command_name} stdout pipe could not be captured"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| format!("{command_name} stderr pipe could not be captured"))?;

    let stdout_reader = thread::spawn(move || {
        let mut reader = stdout;
        let mut buffer = Vec::new();
        let _ = reader.read_to_end(&mut buffer);
        buffer
    });
    let stderr_reader = thread::spawn(move || {
        let mut reader = stderr;
        let mut buffer = Vec::new();
        let _ = reader.read_to_end(&mut buffer);
        buffer
    });

    let mut graceful_deadline = None::<Instant>;
    let exit_status = loop {
        let runtime_flags = current_shell_runtime_flags();
        if runtime_flags.cancellation_request_accepted || runtime_flags.shutdown_requested {
            if let Some(control_file_path) = current_shell_control_file_path() {
                write_execution_control_file(&control_file_path, runtime_flags)?;
            }

            if graceful_deadline.is_none() {
                graceful_deadline = Some(Instant::now() + Duration::from_millis(GRACEFUL_STOP_TIMEOUT_MS));
            } else if graceful_deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                update_shell_runtime_flags(|flags| {
                    flags.forceful_termination_required = true;
                });
                let _ = child.kill();
                update_shell_runtime_flags(|flags| {
                    flags.forceful_termination_executed = true;
                });
            }
        }

        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                thread::sleep(Duration::from_millis(ENGINE_CONTROL_POLL_INTERVAL_MS));
            }
            Err(error) => {
                set_shell_active_engine_module(None);
                return Err(format!("{command_name} could not be supervised: {error}"));
            }
        }
    };

    set_shell_active_engine_module(None);

    let stdout = stdout_reader
        .join()
        .map_err(|_| format!("{command_name} stdout reader panicked"))?;
    let stderr = stderr_reader
        .join()
        .map_err(|_| format!("{command_name} stderr reader panicked"))?;

    if !exit_status.success() {
        let stderr = String::from_utf8_lossy(&stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&stdout).trim().to_string();
        let detail = if !stderr.is_empty() { stderr } else { stdout };
        return Err(format!(
            "{command_name} exited unsuccessfully with {}{}",
            exit_status,
            if detail.is_empty() {
                String::new()
            } else {
                format!(": {detail}")
            }
        ));
    }

    serde_json::from_slice::<T>(&stdout)
        .map_err(|error| format!("{command_name} output could not be decoded: {error}"))
}

fn build_engine_pythonpath() -> Result<OsString, String> {
    let engine_src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("engine")
        .join("src");
    let mut paths = vec![engine_src];

    if let Some(existing_pythonpath) = env::var_os("PYTHONPATH") {
        paths.extend(env::split_paths(&existing_pythonpath));
    }

    env::join_paths(paths)
        .map_err(|error| format!("engine PYTHONPATH could not be constructed: {error}"))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::Builder;

    use super::*;
    use crate::abnormal_termination_residual::{
        abnormal_residual_marker_path, AbnormalTerminationResidualRecord,
    };
    use crate::preflight_contract::{
        PreflightEffectivePlanSnapshot, PreflightProviderModelContextSnapshot,
        PreflightRequestedPlanSnapshot, SupportedSeedDocumentKind,
    };
    use crate::preflight_document_validation::SelectedSeedDocumentSnapshot;
    use crate::provider_configuration_contract::SupportedProvider;
    use crate::run_seed_ingestion::stage_execution_seed_from_lineage_with_bytes;
    use crate::run_start_lineage::{RunStartLineageSnapshot, RunStartLineageStage};
    use crate::run_start_readiness::{
        RunStartFoundationState, RunStartReadinessSnapshot, RunStartReadinessStage,
    };
    use crate::run_workspace_allocation::allocate_fresh_run_workspace_under;

    fn ready_engine_foundation() -> EngineFoundationBootstrapSnapshot {
        EngineFoundationBootstrapSnapshot {
            runtime_persona_manifest_path: "C:/temp/run-1/runtime-personas/persona_set.json"
                .to_string(),
            moderator_basis_path: "C:/temp/run-1/runtime-personas/moderator_basis.json"
                .to_string(),
            bounded_memory_directory_path: "C:/temp/run-1/bounded-memory".to_string(),
            seed_grounding_memory_path: "C:/temp/run-1/bounded-memory/seed_grounding.json"
                .to_string(),
            round_summary_memory_path: "C:/temp/run-1/bounded-memory/round_summary.json"
                .to_string(),
            argument_risk_memory_path:
                "C:/temp/run-1/bounded-memory/argument_risk_memory.json".to_string(),
            transcript_archive_index_path:
                "C:/temp/run-1/transcript-archive/archive_index.json".to_string(),
            moderator_persona_label: "Moderator".to_string(),
            participant_persona_labels: vec![
                "Advocate".to_string(),
                "Skeptic".to_string(),
                "Analyst".to_string(),
                "Builder".to_string(),
                "Auditor".to_string(),
            ],
            perspective_coverage: vec![
                "favorable".to_string(),
                "skeptical".to_string(),
                "analytical".to_string(),
                "neutral".to_string(),
            ],
            participant_persona_count: 5,
            exactly_one_moderator_present: true,
            stable_persona_labels: true,
            runtime_persona_foundation_materialized: true,
            bounded_memory_foundation_materialized: true,
            transcript_archive_separated: true,
            detail_message: "Fresh run/session created, fresh workspace allocated, execution seed staged locally, runtime personas generated, and bounded memory foundation initialized; full moderated execution remains deferred".to_string(),
        }
    }

    fn write_mock_pdf_artifact(file_stem: &str) -> String {
        let path = Builder::new()
            .prefix(file_stem)
            .suffix(".pdf")
            .tempfile()
            .expect("mock pdf tempfile should be created")
            .into_temp_path()
            .keep()
            .expect("mock pdf tempfile should persist");
        fs::write(&path, b"%PDF-1.4\n%%EOF").expect("mock pdf should be written");
        path.display().to_string()
    }

    fn ready_engine_execution() -> EngineSessionExecutionSnapshot {
        EngineSessionExecutionSnapshot {
            transcript_turns_path: "C:/temp/run-1/transcript-archive/speaker_turns.jsonl"
                .to_string(),
            token_duration_tracking_path:
                "C:/temp/run-1/run-local-tracking/token_duration_tracking.json".to_string(),
            requested_round_count: 4,
            completed_round_count: 4,
            speaker_counts_per_round: vec![3, 4, 5, 3],
            total_turn_count: 15,
            latest_round_summary: Some(
                "Round 4 kept bounded continuity grounded in the approved seed context."
                    .to_string(),
            ),
            tracked_total_input_token_units: 420,
            tracked_total_output_token_units: 190,
            tracked_total_token_units: 610,
            tracked_total_duration_ms: 48,
            average_turn_duration_ms: 3,
            moderated_rounds_materialized: true,
            normal_round_speaker_selection_enforced: true,
            speaker_attribution_materialized: true,
            bounded_memory_continuity_mutated: true,
            token_duration_tracking_materialized: true,
            completion_state: EngineExecutionCompletionState::Completed,
            runtime_cap_reason: None,
            controlled_finalization_performed: false,
            finalization_started_at: None,
            finalization_completed_at: None,
            report_identifier: Some("report-data-run-1-1713200000000".to_string()),
            report_generated_at: Some(1713200000000),
            report_data_artifact_path: Some(
                "C:/temp/run-1/report-generation/assembled_report_data.json".to_string(),
            ),
            report_pdf_artifact_path: Some(write_mock_pdf_artifact(
                "report-data-run-1-1713200000000",
            )),
            durable_report_artifact_path: None,
            report_generation_status: "succeeded".to_string(),
            report_generation_failure_reason: None,
            detail_message: "Moderator-controlled execution completed bounded rounds with speaker attribution, continuity mutation, and run-local token/duration tracking".to_string(),
        }
    }

    fn ready_engine_execution_capped() -> EngineSessionExecutionSnapshot {
        EngineSessionExecutionSnapshot {
            transcript_turns_path: "C:/temp/run-1/transcript-archive/speaker_turns.jsonl"
                .to_string(),
            token_duration_tracking_path:
                "C:/temp/run-1/run-local-tracking/token_duration_tracking.json".to_string(),
            requested_round_count: 4,
            completed_round_count: 2,
            speaker_counts_per_round: vec![3, 4],
            total_turn_count: 8,
            latest_round_summary: Some(
                "Controlled finalization preserved bounded continuity after the runtime token budget halted further normal rounds."
                    .to_string(),
            ),
            tracked_total_input_token_units: 380,
            tracked_total_output_token_units: 210,
            tracked_total_token_units: 590,
            tracked_total_duration_ms: 40,
            average_turn_duration_ms: 5,
            moderated_rounds_materialized: false,
            normal_round_speaker_selection_enforced: true,
            speaker_attribution_materialized: true,
            bounded_memory_continuity_mutated: true,
            token_duration_tracking_materialized: true,
            completion_state: EngineExecutionCompletionState::CompletedBudgetCapped,
            runtime_cap_reason: Some(RunCapReason::TokenBudget),
            controlled_finalization_performed: true,
            finalization_started_at: Some(1713200004000),
            finalization_completed_at: Some(1713200004500),
            report_identifier: Some("report-data-run-1-1713200004000".to_string()),
            report_generated_at: Some(1713200004000),
            report_data_artifact_path: Some(
                "C:/temp/run-1/report-generation/assembled_report_data.json".to_string(),
            ),
            report_pdf_artifact_path: Some(write_mock_pdf_artifact(
                "report-data-run-1-1713200004000",
            )),
            durable_report_artifact_path: None,
            report_generation_status: "succeeded".to_string(),
            report_generation_failure_reason: None,
            detail_message: "Moderator-controlled execution hit a governed runtime cap, stopped scheduling further normal rounds, and completed bounded constrained finalization from available material".to_string(),
        }
    }

    fn ready_engine_execution_failed() -> EngineSessionExecutionSnapshot {
        EngineSessionExecutionSnapshot {
            transcript_turns_path: "C:/temp/run-1/transcript-archive/speaker_turns.jsonl"
                .to_string(),
            token_duration_tracking_path:
                "C:/temp/run-1/run-local-tracking/token_duration_tracking.json".to_string(),
            requested_round_count: 4,
            completed_round_count: 4,
            speaker_counts_per_round: vec![3, 4, 5, 3],
            total_turn_count: 15,
            latest_round_summary: Some(
                "Bounded continuity completed normal rounds, but report data assembly later failed."
                    .to_string(),
            ),
            tracked_total_input_token_units: 410,
            tracked_total_output_token_units: 240,
            tracked_total_token_units: 650,
            tracked_total_duration_ms: 48,
            average_turn_duration_ms: 3,
            moderated_rounds_materialized: true,
            normal_round_speaker_selection_enforced: true,
            speaker_attribution_materialized: true,
            bounded_memory_continuity_mutated: true,
            token_duration_tracking_materialized: true,
            completion_state: EngineExecutionCompletionState::FailedTerminal,
            runtime_cap_reason: None,
            controlled_finalization_performed: true,
            finalization_started_at: Some(1713200005000),
            finalization_completed_at: Some(1713200005200),
            report_identifier: Some("report-data-run-1-1713200005000".to_string()),
            report_generated_at: Some(1713200005000),
            report_data_artifact_path: Some(
                "C:/temp/run-1/report-generation/assembled_report_data.json".to_string(),
            ),
            report_pdf_artifact_path: None,
            durable_report_artifact_path: None,
            report_generation_status: "failed".to_string(),
            report_generation_failure_reason: Some("report_data_assembly_failed".to_string()),
            detail_message: "Moderator-controlled execution completed debate material, but report data assembly failed from the actual run artifacts.".to_string(),
        }
    }

    fn ready_run_start_readiness() -> RunStartReadinessSnapshot {
        RunStartReadinessSnapshot {
            stage: RunStartReadinessStage::FoundationMaterialized,
            state_surface_materialized: true,
            run_session_contract_defined: true,
            workspace_boundary_defined: true,
            start_lineage_basis_materialized: true,
            runtime_persona_contract_defined: true,
            bounded_memory_contract_defined: true,
            run_start_command_explicit: true,
            shell_owned_run_session_identity_authority: true,
            shell_owned_workspace_allocation_authority: true,
            shell_owned_engine_start_authorization: true,
            runtime_readiness_evaluated_separately: true,
            activation_readiness_evaluated_separately: true,
            provider_readiness_evaluated_separately: true,
            preflight_readiness_evaluated_separately: true,
            runtime_ready_now: true,
            activation_ready_now: true,
            provider_configuration_ready_now: true,
            current_eligible_preflight_now: true,
            downgrade_acceptance_required: false,
            downgrade_acceptance_recorded: false,
            start_lineage_ready_now: true,
            current_state: RunStartFoundationState::Blocked,
            current_state_name: crate::run_start_readiness::STATE_RUN_START_FOUNDATION_BLOCKED,
            run_start_execution_materialized: true,
            workspace_allocation_materialized: true,
            runtime_persona_foundation_materialized: true,
            bounded_memory_foundation_materialized: true,
            transcript_archive_separation_materialized: true,
            fresh_workspace_required: true,
            hidden_continuation_allowed: false,
            real_run_start_authorized_now: true,
            would_depend_on_current_eligible_preflight_if_execution_existed: true,
            current_status_note:
                "All run-start prerequisites are satisfied for a fresh run/session start".to_string(),
        }
    }

    fn blocked_run_start_readiness() -> RunStartReadinessSnapshot {
        let mut snapshot = ready_run_start_readiness();
        snapshot.runtime_ready_now = false;
        snapshot.current_eligible_preflight_now = false;
        snapshot.start_lineage_ready_now = false;
        snapshot.real_run_start_authorized_now = false;
        snapshot.current_state = RunStartFoundationState::NotStarted;
        snapshot.current_state_name = crate::run_start_readiness::STATE_RUN_START_FOUNDATION_NOT_STARTED;
        snapshot.current_status_note =
            "Run-start foundation is materialized, but current eligible Preflight lineage does not exist yet"
                .to_string();
        snapshot
    }

    fn ready_run_start_lineage() -> RunStartLineageSnapshot {
        RunStartLineageSnapshot {
            stage: RunStartLineageStage::FoundationMaterialized,
            lineage_name: crate::run_start_lineage::RUN_START_LINEAGE_NAME,
            lineage_basis_materialized: true,
            current_eligible_preflight_required: true,
            preflight_state_name: "PreflightApproved",
            preflight_run_start_eligible_now: true,
            preflight_result_current: true,
            selected_seed_document: Some(SelectedSeedDocumentSnapshot {
                file_name: "seed.txt".to_string(),
                mime_type: Some("text/plain".to_string()),
                detected_kind: Some(SupportedSeedDocumentKind::Txt),
                size_bytes: 12,
                content_fingerprint_sha256: "abc123".to_string(),
                extracted_text_character_count: Some(1200),
                english_confidence_percent: Some(99),
            }),
            requested_plan: Some(PreflightRequestedPlanSnapshot {
                requested_agent_count: 5,
                requested_round_count: 4,
                max_token_budget: 12000,
            }),
            effective_plan: Some(PreflightEffectivePlanSnapshot {
                effective_agent_count: 5,
                effective_round_count: 4,
                provider: SupportedProvider::OpenAi,
                model: "gpt-4o-mini".to_string(),
                max_token_budget: 12000,
                bounded_speaker_model: "moderated_3_to_5_speakers_per_round".to_string(),
                bounded_concurrency_policy: "single_moderated_speaker_set_per_round".to_string(),
            }),
            provider_model_context: Some(PreflightProviderModelContextSnapshot {
                provider: SupportedProvider::OpenAi,
                model: "gpt-4o-mini".to_string(),
                local_configuration_supports_planning: true,
                live_reachability_probed: false,
                detail_note: "planning context".to_string(),
            }),
            downgrade_acceptance_required: false,
            downgrade_acceptance_recorded: false,
            lineage_derived_from_current_preflight: true,
            raw_seed_bytes_embedded: false,
            transcript_archive_embedded: false,
            prior_run_memory_embedded: false,
            future_execution_lineage_ready: true,
            detail_note: "Current eligible Preflight truth already contains the bounded seed fingerprint required for a future run start".to_string(),
        }
    }

    fn running_snapshot_with_workspace(root: &Path) -> RunSessionStateSnapshot {
        let workspace_root = root.join("run-shutdown");
        fs::create_dir_all(&workspace_root).expect("workspace root should exist");

        let mut snapshot = RunSessionStateSnapshot::not_started();
        snapshot.current_state_class = RunSessionStateClass::Running;
        snapshot.current_state_name = RunSessionStateClass::Running.label();
        snapshot.run_id = Some("run-shutdown".to_string());
        snapshot.session_id = Some("session-shutdown".to_string());
        snapshot.workspace_reference = Some("workspace::run-shutdown".to_string());
        snapshot.workspace_root_path = Some(workspace_root.to_string_lossy().to_string());
        snapshot.workspace_allocated = true;
        snapshot.pending_execution_ready = true;
        snapshot.detail_message = "One-shot moderated execution is active under shell supervision."
            .to_string();
        snapshot
    }

    #[test]
    fn start_run_session_with_components_returns_blocked_when_start_is_not_authorized() {
        let mut state = StoredRunSessionState::default();
        let outcome = start_run_session_with_components(
            &mut state,
            blocked_run_start_readiness(),
            ready_run_start_lineage(),
            |_| Ok(()),
            || Ok("run-id".to_string()),
            || Ok("session-id".to_string()),
            |_| panic!("workspace allocation should not run when start is blocked"),
            |_, _| panic!("seed staging should not run when start is blocked"),
            |_| panic!("engine foundation bootstrap should not run when start is blocked"),
            |_| panic!("moderated execution should not run when start is blocked"),
        );

        assert_eq!(outcome.decision_type, RunSessionStartDecisionType::StartBlocked);
        assert_eq!(
            outcome.run_session_state.current_state_class,
            RunSessionStateClass::StartBlocked
        );
        assert!(!outcome.run_session_state.pending_execution_ready);
        assert!(!outcome.run_session_state.start_blocking_reasons.is_empty());
        assert!(!outcome.run_session_state.moderated_rounds_materialized);
    }

    #[test]
    fn start_run_session_with_components_creates_fresh_run_ids_and_workspaces() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let seed_bytes = b"hello world!".to_vec();
        let first_outcome = start_run_session_with_components(
            &mut StoredRunSessionState::default(),
            ready_run_start_readiness(),
            ready_run_start_lineage(),
            |_| Ok(()),
            || Ok("run-1".to_string()),
            || Ok("session-1".to_string()),
            |run_id| allocate_fresh_run_workspace_under(temp_dir.path(), run_id),
            |workspace, lineage| {
                stage_execution_seed_from_lineage_with_bytes(
                    workspace,
                    lineage,
                    lineage.selected_seed_document.as_ref().expect("seed document exists"),
                    &seed_bytes,
                )
            },
            |_| Ok(ready_engine_foundation()),
            |_| Ok(ready_engine_execution()),
        );
        let second_outcome = start_run_session_with_components(
            &mut StoredRunSessionState::default(),
            ready_run_start_readiness(),
            ready_run_start_lineage(),
            |_| Ok(()),
            || Ok("run-2".to_string()),
            || Ok("session-2".to_string()),
            |run_id| allocate_fresh_run_workspace_under(temp_dir.path(), run_id),
            |workspace, lineage| {
                stage_execution_seed_from_lineage_with_bytes(
                    workspace,
                    lineage,
                    lineage.selected_seed_document.as_ref().expect("seed document exists"),
                    &seed_bytes,
                )
            },
            |_| Ok(ready_engine_foundation()),
            |_| Ok(ready_engine_execution()),
        );

        assert_eq!(
            first_outcome.decision_type,
            RunSessionStartDecisionType::ExecutionCompleted
        );
        assert_eq!(
            second_outcome.decision_type,
            RunSessionStartDecisionType::ExecutionCompleted
        );
        assert_ne!(first_outcome.run_session_state.run_id, second_outcome.run_session_state.run_id);
        assert_ne!(
            first_outcome.run_session_state.workspace_reference,
            second_outcome.run_session_state.workspace_reference
        );
        assert!(first_outcome.run_session_state.workspace_allocated);
        assert!(first_outcome.run_session_state.execution_seed_staged);
        assert!(first_outcome.run_session_state.runtime_persona_foundation_materialized);
        assert!(first_outcome.run_session_state.bounded_memory_foundation_materialized);
        assert!(first_outcome.run_session_state.transcript_archive_separated);
        assert!(first_outcome.run_session_state.exactly_one_moderator_present);
        assert_eq!(
            first_outcome.run_session_state.moderator_persona_label.as_deref(),
            Some("Moderator")
        );
        assert_eq!(
            first_outcome.run_session_state.current_state_class,
            RunSessionStateClass::Finished
        );
        assert_eq!(first_outcome.run_session_state.requested_round_count, Some(4));
        assert_eq!(first_outcome.run_session_state.completed_round_count, Some(4));
        assert_eq!(first_outcome.run_session_state.speaker_counts_per_round, vec![3, 4, 5, 3]);
        assert_eq!(first_outcome.run_session_state.total_turn_count, Some(15));
        assert_eq!(
            first_outcome.run_session_state.tracked_total_token_units,
            Some(610)
        );
        assert!(first_outcome.run_session_state.moderated_rounds_materialized);
        assert!(first_outcome.run_session_state.normal_round_speaker_selection_enforced);
        assert!(first_outcome.run_session_state.speaker_attribution_materialized);
        assert!(first_outcome.run_session_state.bounded_memory_continuity_mutated);
        assert!(first_outcome.run_session_state.token_duration_tracking_materialized);
        assert!(!first_outcome.run_session_state.pending_execution_ready);
        assert_eq!(
            first_outcome.run_session_state.cleanup_marker,
            Some(RunCleanupMarker::CleanupSucceeded)
        );
        assert!(Path::new(
            first_outcome
                .run_session_state
                .durable_report_artifact_path
                .as_deref()
                .expect("durable report artifact path should exist after completion")
        )
        .exists());
        assert!(!Path::new(
            first_outcome
                .run_session_state
                .workspace_root_path
                .as_deref()
                .expect("workspace root path should be captured")
        )
        .exists());
    }

    #[test]
    fn start_run_session_with_components_publishes_in_flight_snapshots_before_completion() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let seed_bytes = b"hello world!".to_vec();
        let mut state = StoredRunSessionState::default();
        let mut published_snapshots = Vec::<RunSessionStateSnapshot>::new();

        let outcome = start_run_session_with_components(
            &mut state,
            ready_run_start_readiness(),
            ready_run_start_lineage(),
            |snapshot| {
                published_snapshots.push(snapshot.clone());
                Ok(())
            },
            || Ok("run-1".to_string()),
            || Ok("session-1".to_string()),
            |run_id| allocate_fresh_run_workspace_under(temp_dir.path(), run_id),
            |workspace, lineage| {
                stage_execution_seed_from_lineage_with_bytes(
                    workspace,
                    lineage,
                    lineage.selected_seed_document.as_ref().expect("seed document exists"),
                    &seed_bytes,
                )
            },
            |_| Ok(ready_engine_foundation()),
            |_| Ok(ready_engine_execution()),
        );

        assert_eq!(outcome.decision_type, RunSessionStartDecisionType::ExecutionCompleted);
        assert_eq!(published_snapshots.len(), 9);
        assert_eq!(published_snapshots[0].current_state_class, RunSessionStateClass::PendingExecution);
        assert!(published_snapshots[0].run_id.is_some());
        assert!(!published_snapshots[0].workspace_allocated);
        assert_eq!(published_snapshots[1].current_state_class, RunSessionStateClass::PendingExecution);
        assert!(published_snapshots[1].workspace_allocated);
        assert!(!published_snapshots[1].execution_seed_staged);
        assert_eq!(published_snapshots[2].current_state_class, RunSessionStateClass::PendingExecution);
        assert!(published_snapshots[2].execution_seed_staged);
        assert!(!published_snapshots[2].pending_execution_ready);
        assert_eq!(published_snapshots[3].current_state_class, RunSessionStateClass::PendingExecution);
        assert!(published_snapshots[3].pending_execution_ready);
        assert!(published_snapshots[3].runtime_persona_foundation_materialized);
        assert_eq!(published_snapshots[4].current_state_class, RunSessionStateClass::Running);
        assert_eq!(published_snapshots[4].current_state_name, RunSessionStateClass::Running.label());
        assert_eq!(published_snapshots[5].current_state_class, RunSessionStateClass::Finished);
        assert_eq!(published_snapshots[6].cleanup_marker, Some(RunCleanupMarker::CleanupPending));
        assert_eq!(published_snapshots[7].cleanup_marker, Some(RunCleanupMarker::CleanupRunning));
        assert_eq!(published_snapshots[8].cleanup_marker, Some(RunCleanupMarker::CleanupSucceeded));
        assert_eq!(state.latest_snapshot.current_state_class, RunSessionStateClass::Finished);
        assert_eq!(state.latest_snapshot.cleanup_marker, Some(RunCleanupMarker::CleanupSucceeded));
    }

    #[test]
    fn start_run_session_with_components_preserves_completed_budget_capped_truth() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let seed_bytes = b"hello world!".to_vec();

        let outcome = start_run_session_with_components(
            &mut StoredRunSessionState::default(),
            ready_run_start_readiness(),
            ready_run_start_lineage(),
            |_| Ok(()),
            || Ok("run-1".to_string()),
            || Ok("session-1".to_string()),
            |run_id| allocate_fresh_run_workspace_under(temp_dir.path(), run_id),
            |workspace, lineage| {
                stage_execution_seed_from_lineage_with_bytes(
                    workspace,
                    lineage,
                    lineage.selected_seed_document.as_ref().expect("seed document exists"),
                    &seed_bytes,
                )
            },
            |_| Ok(ready_engine_foundation()),
            |_| Ok(ready_engine_execution_capped()),
        );

        assert_eq!(
            outcome.decision_type,
            RunSessionStartDecisionType::ExecutionCompleted
        );
        assert_eq!(
            outcome.run_session_state.lifecycle_override,
            Some(RunLifecycleOverrideState::CompletedBudgetCapped)
        );
        assert_eq!(
            outcome.run_session_state.runtime_cap_reason,
            Some(RunCapReason::TokenBudget)
        );
        assert!(outcome.run_session_state.controlled_finalization_performed);
        assert_eq!(outcome.run_session_state.completed_round_count, Some(2));
        assert!(!outcome.run_session_state.moderated_rounds_materialized);
        assert_eq!(
            outcome.run_session_state.cleanup_marker,
            Some(RunCleanupMarker::CleanupSucceeded)
        );
    }

    #[test]
    fn start_run_session_with_components_distinguishes_report_generation_failure_from_completion() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let seed_bytes = b"hello world!".to_vec();

        let outcome = start_run_session_with_components(
            &mut StoredRunSessionState::default(),
            ready_run_start_readiness(),
            ready_run_start_lineage(),
            |_| Ok(()),
            || Ok("run-1".to_string()),
            || Ok("session-1".to_string()),
            |run_id| allocate_fresh_run_workspace_under(temp_dir.path(), run_id),
            |workspace, lineage| {
                stage_execution_seed_from_lineage_with_bytes(
                    workspace,
                    lineage,
                    lineage.selected_seed_document.as_ref().expect("seed document exists"),
                    &seed_bytes,
                )
            },
            |_| Ok(ready_engine_foundation()),
            |_| Ok(ready_engine_execution_failed()),
        );

        assert_eq!(outcome.decision_type, RunSessionStartDecisionType::ExecutionFailed);
        assert_eq!(
            outcome.run_session_state.current_state_class,
            RunSessionStateClass::TechnicalFailure
        );
        assert_eq!(
            outcome.run_session_state.technical_failure_reason,
            Some(RunSessionTechnicalFailureReason::ReportGenerationFailed)
        );
        assert_eq!(
            outcome.run_session_state.lifecycle_override,
            Some(RunLifecycleOverrideState::FailedTerminal)
        );
        assert_eq!(
            outcome.run_session_state.cleanup_marker,
            Some(RunCleanupMarker::CleanupSucceeded)
        );
    }

    #[test]
    fn start_run_session_with_components_pauses_and_retries_retryable_execution_failures() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let seed_bytes = b"hello world!".to_vec();
        let mut state = StoredRunSessionState::default();
        let mut published_snapshots = Vec::<RunSessionStateSnapshot>::new();
        let mut execution_attempts = 0usize;

        let outcome = start_run_session_with_components(
            &mut state,
            ready_run_start_readiness(),
            ready_run_start_lineage(),
            |snapshot| {
                published_snapshots.push(snapshot.clone());
                Ok(())
            },
            || Ok("run-1".to_string()),
            || Ok("session-1".to_string()),
            |run_id| allocate_fresh_run_workspace_under(temp_dir.path(), run_id),
            |workspace, lineage| {
                stage_execution_seed_from_lineage_with_bytes(
                    workspace,
                    lineage,
                    lineage.selected_seed_document.as_ref().expect("seed document exists"),
                    &seed_bytes,
                )
            },
            |_| Ok(ready_engine_foundation()),
            |_| {
                execution_attempts += 1;
                if execution_attempts == 1 {
                    Err(format!(
                        "{} provider asked for bounded retry",
                        RETRYABLE_NETWORK_FAILURE_PREFIX
                    ))
                } else {
                    Ok(ready_engine_execution())
                }
            },
        );

        assert_eq!(outcome.decision_type, RunSessionStartDecisionType::ExecutionCompleted);
        assert_eq!(execution_attempts, 2);
        assert!(published_snapshots.iter().any(|snapshot| {
            snapshot.lifecycle_override == Some(RunLifecycleOverrideState::PausedNetwork)
                && snapshot.retry_attempt_count == 1
        }));
        assert_eq!(state.latest_snapshot.current_state_class, RunSessionStateClass::Finished);
        assert_eq!(state.latest_snapshot.cleanup_marker, Some(RunCleanupMarker::CleanupSucceeded));
    }

    #[test]
    fn shutdown_handoff_writes_control_flags_and_residual_marker_for_startup_janitor() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");
        let mut snapshot = running_snapshot_with_workspace(temp_dir.path());
        snapshot.shutdown_requested = true;
        snapshot.cancellation_request_accepted = true;
        let workspace_root_path = snapshot
            .workspace_root_path
            .clone()
            .expect("workspace root path should exist");
        let control_file_path = initialize_execution_control_file(&workspace_root_path)
            .expect("execution control file should be created");
        let runtime_flags = RunSessionRuntimeFlags {
            cancellation_requested: true,
            cancellation_request_accepted: true,
            shutdown_requested: true,
            graceful_shutdown_started: true,
            ..RunSessionRuntimeFlags::default()
        };

        write_execution_control_file(&control_file_path, runtime_flags)
            .expect("execution control file should be updated");
        sync_abnormal_residual_marker(
            &snapshot,
            true,
            "Shell-supervised shutdown coordination is active for the current workspace. If terminal cleanup cannot complete before the process exits, startup janitor will retry purge on the next launch without resuming the abandoned run.",
        );

        let control_payload = fs::read_to_string(&control_file_path)
            .expect("execution control file should remain readable");
        let control_payload = serde_json::from_str::<EngineExecutionControlPayload>(&control_payload)
            .expect("execution control payload should decode");
        let marker_path = abnormal_residual_marker_path(&workspace_root_path);
        let marker_payload = fs::read_to_string(&marker_path)
            .expect("abnormal residual marker should exist after shutdown acceptance");
        let marker = serde_json::from_str::<AbnormalTerminationResidualRecord>(&marker_payload)
            .expect("abnormal residual marker should decode");

        assert!(control_payload.cancellation_requested);
        assert!(control_payload.shutdown_requested);
        assert_eq!(marker.run_id.as_deref(), Some("run-shutdown"));
        assert_eq!(marker.session_id.as_deref(), Some("session-shutdown"));
        assert!(marker.shutdown_requested);
        assert!(marker.cleanup_handoff_required);
        assert!(marker.detail_message.contains("startup janitor"));
    }
}