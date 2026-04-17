use serde::{Deserialize, Serialize};

use crate::run_cleanup_contract::RunCleanupMarker;
use crate::run_session_contract::RunSessionStateClass;
use crate::run_session_start::{
    load_execution_phase_from_run_session, run_session_state_from_shell,
    terminal_run_outcome_name_from_snapshot, EngineExecutionPhaseState,
    RunLifecycleOverrideState, RunSessionStateSnapshot,
};

pub const RUN_LIFECYCLE_STATE_NAME: &str = "RunLifecycleState";
pub const SUPPORTED_CANONICAL_RUN_LIFECYCLE_STATES: [CanonicalRunLifecycleState; 16] = [
    CanonicalRunLifecycleState::RunDraft,
    CanonicalRunLifecycleState::RunQueued,
    CanonicalRunLifecycleState::Ingesting,
    CanonicalRunLifecycleState::GeneratingPersonas,
    CanonicalRunLifecycleState::RunningRound,
    CanonicalRunLifecycleState::PausedRateLimit,
    CanonicalRunLifecycleState::PausedNetwork,
    CanonicalRunLifecycleState::Finalizing,
    CanonicalRunLifecycleState::Completed,
    CanonicalRunLifecycleState::CompletedBudgetCapped,
    CanonicalRunLifecycleState::Canceled,
    CanonicalRunLifecycleState::FailedRetryable,
    CanonicalRunLifecycleState::FailedTerminal,
    CanonicalRunLifecycleState::Cleaning,
    CanonicalRunLifecycleState::Cleaned,
    CanonicalRunLifecycleState::CleanupFailed,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunLifecycleStateStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalRunLifecycleState {
    RunDraft,
    RunQueued,
    Ingesting,
    GeneratingPersonas,
    RunningRound,
    PausedRateLimit,
    PausedNetwork,
    Finalizing,
    Completed,
    CompletedBudgetCapped,
    Canceled,
    FailedRetryable,
    FailedTerminal,
    Cleaning,
    Cleaned,
    CleanupFailed,
}

impl CanonicalRunLifecycleState {
    pub fn name(self) -> &'static str {
        match self {
            Self::RunDraft => "RunDraft",
            Self::RunQueued => "RunQueued",
            Self::Ingesting => "Ingesting",
            Self::GeneratingPersonas => "GeneratingPersonas",
            Self::RunningRound => "RunningRound",
            Self::PausedRateLimit => "PausedRateLimit",
            Self::PausedNetwork => "PausedNetwork",
            Self::Finalizing => "Finalizing",
            Self::Completed => "Completed",
            Self::CompletedBudgetCapped => "CompletedBudgetCapped",
            Self::Canceled => "Canceled",
            Self::FailedRetryable => "FailedRetryable",
            Self::FailedTerminal => "FailedTerminal",
            Self::Cleaning => "Cleaning",
            Self::Cleaned => "Cleaned",
            Self::CleanupFailed => "CleanupFailed",
        }
    }

    pub fn terminal(self) -> bool {
        matches!(
            self,
            Self::Cleaning
                | Self::Completed
                | Self::CompletedBudgetCapped
                | Self::Canceled
                | Self::FailedRetryable
                | Self::FailedTerminal
                | Self::Cleaned
                | Self::CleanupFailed
        )
    }

    pub fn cancelable(self) -> bool {
        matches!(
            self,
            Self::RunQueued
                | Self::Ingesting
                | Self::GeneratingPersonas
                | Self::RunningRound
                | Self::PausedRateLimit
                | Self::PausedNetwork
        )
    }

    pub fn retryable(self) -> bool {
        matches!(self, Self::FailedRetryable)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunLifecycleStateSnapshot {
    pub stage: RunLifecycleStateStage,
    pub surface_name: &'static str,
    pub supported_states: [CanonicalRunLifecycleState; 16],
    pub current_state: CanonicalRunLifecycleState,
    pub current_state_name: &'static str,
    pub projected_from_state_class: RunSessionStateClass,
    pub projected_from_state_name: &'static str,
    pub shell_authoritative_projection: bool,
    pub canonical_state_family_frozen: bool,
    pub mapping_basis_explicit: bool,
    pub degraded_states_distinct_in_vocabulary: bool,
    pub cancellation_state_distinct_in_vocabulary: bool,
    pub constrained_completion_state_distinct_in_vocabulary: bool,
    pub retryable_failure_state_distinct_in_vocabulary: bool,
    pub terminal_failure_state_distinct_in_vocabulary: bool,
    pub cleanup_states_distinct_in_vocabulary: bool,
    pub cleanup_pending_state_reserved_for_contract_surface: bool,
    pub startup_janitor_state_outside_canonical_run_lifecycle: bool,
    pub finalization_state_distinct_from_report_generation: bool,
    pub finalization_state_distinct_from_report_availability: bool,
    pub completed_state_implies_report_available: bool,
    pub current_state_terminal: bool,
    pub current_state_cancelable: bool,
    pub current_state_retryable: bool,
    pub live_monitoring_implied: bool,
    pub ordered_event_history_implied: bool,
    pub current_projection_detail: String,
}

impl RunLifecycleStateSnapshot {
    pub fn summary(self) -> String {
        let supported_states = self
            .supported_states
            .map(CanonicalRunLifecycleState::name)
            .join(", ");

        format!(
            "Run lifecycle state surface is materialized. Stage is {:?}; surface name is {}; supported canonical states are {}; current canonical state is {} ({:?}); projected from coarse state class {} ({:?}); shell-authoritative projection is {}; canonical state family frozen is {}; mapping basis explicit is {}; degraded states distinct in vocabulary is {}; cancellation state distinct in vocabulary is {}; constrained completion state distinct in vocabulary is {}; retryable failure state distinct in vocabulary is {}; terminal failure state distinct in vocabulary is {}; cleanup states distinct in vocabulary is {}; cleanup pending state reserved for contract surface is {}; startup janitor state outside canonical run lifecycle is {}; finalization state distinct from report generation is {}; finalization state distinct from report availability is {}; completed state implies report available is {}; current state terminal is {}; current state cancelable is {}; current state retryable is {}; live monitoring implied is {}; ordered event history implied is {}; projection detail is {}.",
            self.stage,
            self.surface_name,
            supported_states,
            self.current_state_name,
            self.current_state,
            self.projected_from_state_name,
            self.projected_from_state_class,
            self.shell_authoritative_projection,
            self.canonical_state_family_frozen,
            self.mapping_basis_explicit,
            self.degraded_states_distinct_in_vocabulary,
            self.cancellation_state_distinct_in_vocabulary,
            self.constrained_completion_state_distinct_in_vocabulary,
            self.retryable_failure_state_distinct_in_vocabulary,
            self.terminal_failure_state_distinct_in_vocabulary,
            self.cleanup_states_distinct_in_vocabulary,
            self.cleanup_pending_state_reserved_for_contract_surface,
            self.startup_janitor_state_outside_canonical_run_lifecycle,
            self.finalization_state_distinct_from_report_generation,
            self.finalization_state_distinct_from_report_availability,
            self.completed_state_implies_report_available,
            self.current_state_terminal,
            self.current_state_cancelable,
            self.current_state_retryable,
            self.live_monitoring_implied,
            self.ordered_event_history_implied,
            self.current_projection_detail,
        )
    }
}

pub fn project_run_lifecycle_state(
    run_session_state: &RunSessionStateSnapshot,
) -> RunLifecycleStateSnapshot {
    let execution_phase = load_execution_phase_from_run_session(run_session_state);

    let (current_state, current_projection_detail) = if let Some(cleanup_marker) = run_session_state.cleanup_marker {
        match cleanup_marker {
            RunCleanupMarker::CleanupPending => (
                CanonicalRunLifecycleState::Cleaning,
                format!(
                    "The shell already resolved the run as {} and has marked terminal cleanup as pending before transient workspace removal begins. {}",
                    terminal_run_outcome_name_from_snapshot(run_session_state),
                    run_session_state.detail_message
                ),
            ),
            RunCleanupMarker::CleanupRunning => (
                CanonicalRunLifecycleState::Cleaning,
                format!(
                    "The shell already resolved the run as {} and terminal cleanup is now active under shell-owned filesystem authority. {}",
                    terminal_run_outcome_name_from_snapshot(run_session_state),
                    run_session_state.detail_message
                ),
            ),
            RunCleanupMarker::CleanupSucceeded => (
                CanonicalRunLifecycleState::Cleaned,
                format!(
                    "The shell resolved the run as {} and terminal cleanup completed without reclassifying preserved report artifacts as transient workspace garbage. {}",
                    terminal_run_outcome_name_from_snapshot(run_session_state),
                    run_session_state.detail_message
                ),
            ),
            RunCleanupMarker::CleanupPartiallyFailed | RunCleanupMarker::CleanupFailed => (
                CanonicalRunLifecycleState::CleanupFailed,
                format!(
                    "The shell resolved the run as {}, but terminal cleanup did not complete cleanly and the cleanup result remains distinct from the run outcome. {}",
                    terminal_run_outcome_name_from_snapshot(run_session_state),
                    run_session_state.detail_message
                ),
            ),
        }
    } else if let Some(lifecycle_override) = run_session_state.lifecycle_override {
        match lifecycle_override {
            RunLifecycleOverrideState::PausedRateLimit => (
                CanonicalRunLifecycleState::PausedRateLimit,
                format!(
                    "The shell has classified the active run as paused for a bounded rate-limit retry path. {}",
                    run_session_state.detail_message
                ),
            ),
            RunLifecycleOverrideState::PausedNetwork => (
                CanonicalRunLifecycleState::PausedNetwork,
                format!(
                    "The shell has classified the active run as paused for a bounded network retry path. {}",
                    run_session_state.detail_message
                ),
            ),
            RunLifecycleOverrideState::CompletedBudgetCapped => (
                CanonicalRunLifecycleState::CompletedBudgetCapped,
                format!(
                    "The shell resolved the active run as CompletedBudgetCapped after a governed runtime cap stopped further normal rounds. {}",
                    run_session_state.detail_message
                ),
            ),
            RunLifecycleOverrideState::Canceled => (
                CanonicalRunLifecycleState::Canceled,
                format!(
                    "The shell accepted interruption control and the run resolved as Canceled rather than healthy completion. {}",
                    run_session_state.detail_message
                ),
            ),
            RunLifecycleOverrideState::FailedRetryable => (
                CanonicalRunLifecycleState::FailedRetryable,
                format!(
                    "The shell exhausted bounded retry handling and now distinguishes this terminal state as FailedRetryable. {}",
                    run_session_state.detail_message
                ),
            ),
            RunLifecycleOverrideState::FailedTerminal => (
                CanonicalRunLifecycleState::FailedTerminal,
                format!(
                    "The shell can classify the current terminal state directly as FailedTerminal. {}",
                    run_session_state.detail_message
                ),
            ),
        }
    } else if matches!(run_session_state.current_state_class, RunSessionStateClass::Running) {
        match execution_phase.as_ref().map(|phase| phase.current_phase) {
            Some(EngineExecutionPhaseState::Finalizing) => (
                CanonicalRunLifecycleState::Finalizing,
                execution_phase
                    .as_ref()
                    .map(|phase| phase.detail_message.clone())
                    .unwrap_or_else(|| {
                        "The shell is projecting constrained finalization from the current run-local execution phase artifact."
                            .to_string()
                    }),
            ),
            Some(EngineExecutionPhaseState::CompletedBudgetCapped) => (
                CanonicalRunLifecycleState::CompletedBudgetCapped,
                execution_phase
                    .as_ref()
                    .map(|phase| phase.detail_message.clone())
                    .unwrap_or_else(|| {
                        "The shell is projecting CompletedBudgetCapped from the current run-local execution phase artifact."
                            .to_string()
                    }),
            ),
            Some(EngineExecutionPhaseState::FailedTerminal) => (
                CanonicalRunLifecycleState::FailedTerminal,
                execution_phase
                    .as_ref()
                    .map(|phase| phase.detail_message.clone())
                    .unwrap_or_else(|| {
                        "The shell is projecting FailedTerminal from the current run-local execution phase artifact."
                            .to_string()
                    }),
            ),
            Some(EngineExecutionPhaseState::Completed) => (
                CanonicalRunLifecycleState::Completed,
                execution_phase
                    .as_ref()
                    .map(|phase| phase.detail_message.clone())
                    .unwrap_or_else(|| {
                        "The shell is projecting Completed from the current run-local execution phase artifact."
                            .to_string()
                    }),
            ),
            _ => (
                CanonicalRunLifecycleState::RunningRound,
                format!(
                    "The shell has handed off one-shot moderated execution under the current run/session identity, so the canonical lifecycle projects to RunningRound. {}",
                    run_session_state.detail_message
                ),
            ),
        }
    } else { match run_session_state.current_state_class {
        RunSessionStateClass::NotStarted => (
            CanonicalRunLifecycleState::RunDraft,
            "No shell-owned run/session has been created yet, so the canonical lifecycle remains at RunDraft."
                .to_string(),
        ),
        RunSessionStateClass::StartBlocked => (
            CanonicalRunLifecycleState::RunDraft,
            format!(
                "Run start is still blocked at the draft boundary. {}",
                run_session_state.detail_message
            ),
        ),
        RunSessionStateClass::TechnicalFailure => (
            CanonicalRunLifecycleState::FailedTerminal,
            format!(
                "Current shell-owned projection conservatively maps the coarse technical-failure class to FailedTerminal until retry-versus-terminal distinction is materialized. {}",
                run_session_state.detail_message
            ),
        ),
        RunSessionStateClass::PendingExecution => (
            if !run_session_state.workspace_allocated {
                CanonicalRunLifecycleState::RunQueued
            } else if !run_session_state.execution_seed_staged {
                CanonicalRunLifecycleState::Ingesting
            } else if !run_session_state.runtime_persona_foundation_materialized {
                CanonicalRunLifecycleState::GeneratingPersonas
            } else {
                CanonicalRunLifecycleState::RunQueued
            },
            if !run_session_state.workspace_allocated {
                format!(
                    "Fresh run/session identity is allocated under shell authority, but workspace allocation is not complete yet. {}",
                    run_session_state.detail_message
                )
            } else if !run_session_state.execution_seed_staged {
                format!(
                    "The shell has allocated the fresh workspace and is still materializing execution-seed artifacts, so the canonical lifecycle is projected as Ingesting. {}",
                    run_session_state.detail_message
                )
            } else if !run_session_state.runtime_persona_foundation_materialized {
                format!(
                    "Execution seed staging is complete while runtime persona and bounded-memory foundation work is still materializing, so the canonical lifecycle is projected as GeneratingPersonas. {}",
                    run_session_state.detail_message
                )
            } else {
                format!(
                    "Fresh run/session preparation is complete and the shell is waiting to hand off one-shot moderated execution, so the canonical lifecycle is projected as RunQueued. {}",
                    run_session_state.detail_message
                )
            },
        ),
        RunSessionStateClass::Finished => (
            CanonicalRunLifecycleState::Completed,
            "The current bounded execution path resolved under shell authority and no non-standard completion override remains active, so the canonical lifecycle projects to Completed."
                .to_string(),
        ),
        RunSessionStateClass::Running => (
            CanonicalRunLifecycleState::RunningRound,
            format!(
                "The shell has handed off one-shot moderated execution under the current run/session identity, so the canonical lifecycle projects to RunningRound. {}",
                run_session_state.detail_message
            ),
        ),
    }};

    let current_state_cancelable = current_state.cancelable() && !run_session_state.cancellation_request_accepted;

    RunLifecycleStateSnapshot {
        stage: RunLifecycleStateStage::FoundationMaterialized,
        surface_name: RUN_LIFECYCLE_STATE_NAME,
        supported_states: SUPPORTED_CANONICAL_RUN_LIFECYCLE_STATES,
        current_state,
        current_state_name: current_state.name(),
        projected_from_state_class: run_session_state.current_state_class,
        projected_from_state_name: run_session_state.current_state_name,
        shell_authoritative_projection: true,
        canonical_state_family_frozen: true,
        mapping_basis_explicit: true,
        degraded_states_distinct_in_vocabulary: true,
        cancellation_state_distinct_in_vocabulary: true,
        constrained_completion_state_distinct_in_vocabulary: true,
        retryable_failure_state_distinct_in_vocabulary: true,
        terminal_failure_state_distinct_in_vocabulary: true,
        cleanup_states_distinct_in_vocabulary: true,
        cleanup_pending_state_reserved_for_contract_surface: true,
        startup_janitor_state_outside_canonical_run_lifecycle: true,
        finalization_state_distinct_from_report_generation: true,
        finalization_state_distinct_from_report_availability: true,
        completed_state_implies_report_available: false,
        current_state_terminal: current_state.terminal(),
        current_state_cancelable,
        current_state_retryable: current_state.retryable(),
        live_monitoring_implied: false,
        ordered_event_history_implied: false,
        current_projection_detail,
    }
}

pub fn run_lifecycle_state_from_shell() -> RunLifecycleStateSnapshot {
    project_run_lifecycle_state(&run_session_state_from_shell())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_cleanup_contract::RunCleanupMarker;
    use crate::run_session_start::RunDegradationReason;

    fn run_session_state_for_class(class: RunSessionStateClass) -> RunSessionStateSnapshot {
        let mut snapshot = RunSessionStateSnapshot::not_started();
        snapshot.current_state_class = class;
        snapshot.current_state_name = class.label();
        snapshot.detail_message = format!("Mock {:?} detail", class);
        snapshot
    }

    #[test]
    fn lifecycle_surface_freezes_all_canonical_states_without_implying_live_history() {
        let snapshot = run_lifecycle_state_from_shell();

        assert_eq!(snapshot.surface_name, RUN_LIFECYCLE_STATE_NAME);
        assert_eq!(snapshot.supported_states.len(), 16);
        assert!(snapshot.canonical_state_family_frozen);
        assert!(snapshot.degraded_states_distinct_in_vocabulary);
        assert!(snapshot.cancellation_state_distinct_in_vocabulary);
        assert!(snapshot.retryable_failure_state_distinct_in_vocabulary);
        assert!(snapshot.cleanup_states_distinct_in_vocabulary);
        assert!(snapshot.cleanup_pending_state_reserved_for_contract_surface);
        assert!(snapshot.startup_janitor_state_outside_canonical_run_lifecycle);
        assert!(snapshot.finalization_state_distinct_from_report_generation);
        assert!(snapshot.finalization_state_distinct_from_report_availability);
        assert!(!snapshot.completed_state_implies_report_available);
        assert!(!snapshot.live_monitoring_implied);
        assert!(!snapshot.ordered_event_history_implied);
    }

    #[test]
    fn running_state_projects_to_running_round_and_remains_cancelable() {
        let snapshot = project_run_lifecycle_state(&run_session_state_for_class(
            RunSessionStateClass::Running,
        ));

        assert_eq!(snapshot.current_state, CanonicalRunLifecycleState::RunningRound);
        assert_eq!(snapshot.current_state_name, "RunningRound");
        assert!(snapshot.current_state_cancelable);
        assert!(!snapshot.current_state_terminal);
    }
    #[test]
    fn lifecycle_override_projects_paused_network_and_stops_redundant_cancelability_after_acceptance() {
        let mut snapshot = run_session_state_for_class(RunSessionStateClass::Running);
        snapshot.lifecycle_override = Some(RunLifecycleOverrideState::PausedNetwork);
        snapshot.degradation_reason = Some(RunDegradationReason::Network);
        snapshot.retry_attempt_count = 1;
        snapshot.cancellation_request_accepted = true;

        let lifecycle = project_run_lifecycle_state(&snapshot);

        assert_eq!(lifecycle.current_state, CanonicalRunLifecycleState::PausedNetwork);
        assert!(!lifecycle.current_state_cancelable);
    }
    #[test]
    fn pending_execution_projects_to_ingesting_when_workspace_exists_but_seed_not_staged() {
        let mut snapshot = run_session_state_for_class(RunSessionStateClass::PendingExecution);
        snapshot.run_id = Some("run-1".to_string());
        snapshot.session_id = Some("session-1".to_string());
        snapshot.workspace_allocated = true;

        let lifecycle = project_run_lifecycle_state(&snapshot);

        assert_eq!(lifecycle.current_state, CanonicalRunLifecycleState::Ingesting);
        assert!(lifecycle.current_projection_detail.contains("Ingesting"));
    }

    #[test]
    fn pending_execution_projects_to_generating_personas_after_seed_staging() {
        let mut snapshot = run_session_state_for_class(RunSessionStateClass::PendingExecution);
        snapshot.run_id = Some("run-1".to_string());
        snapshot.session_id = Some("session-1".to_string());
        snapshot.workspace_allocated = true;
        snapshot.execution_seed_staged = true;

        let lifecycle = project_run_lifecycle_state(&snapshot);

        assert_eq!(lifecycle.current_state, CanonicalRunLifecycleState::GeneratingPersonas);
        assert!(lifecycle.current_projection_detail.contains("GeneratingPersonas"));
    }

    #[test]
    fn finished_state_projects_to_completed_without_claiming_cleanup_variants() {
        let snapshot = project_run_lifecycle_state(&run_session_state_for_class(
            RunSessionStateClass::Finished,
        ));

        assert_eq!(snapshot.current_state, CanonicalRunLifecycleState::Completed);
        assert_eq!(snapshot.current_state_name, "Completed");
        assert!(snapshot.current_state_terminal);
        assert!(!snapshot.current_state_cancelable);
        assert!(snapshot.current_projection_detail.contains("Completed"));
    }

    #[test]
    fn completed_budget_capped_override_projects_constrained_terminal_state() {
        let mut snapshot = run_session_state_for_class(RunSessionStateClass::Finished);
        snapshot.lifecycle_override = Some(RunLifecycleOverrideState::CompletedBudgetCapped);
        snapshot.detail_message =
            "Runtime token budget stopped further normal rounds after bounded finalization."
                .to_string();

        let lifecycle = project_run_lifecycle_state(&snapshot);

        assert_eq!(
            lifecycle.current_state,
            CanonicalRunLifecycleState::CompletedBudgetCapped
        );
        assert!(lifecycle.current_state_terminal);
        assert!(!lifecycle.current_state_cancelable);
    }

    #[test]
    fn cleanup_running_projects_to_cleaning_without_erasing_underlying_run_outcome() {
        let mut snapshot = run_session_state_for_class(RunSessionStateClass::Finished);
        snapshot.cleanup_marker = Some(RunCleanupMarker::CleanupRunning);
        snapshot.detail_message =
            "Run resolved as Completed; shell-owned terminal cleanup is now removing transient workspace targets."
                .to_string();

        let lifecycle = project_run_lifecycle_state(&snapshot);

        assert_eq!(lifecycle.current_state, CanonicalRunLifecycleState::Cleaning);
        assert!(lifecycle.current_projection_detail.contains("resolved the run as Completed"));
        assert!(lifecycle.current_state_terminal);
        assert!(!lifecycle.current_state_cancelable);
    }
}