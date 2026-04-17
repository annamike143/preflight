use serde::{Deserialize, Serialize};

use crate::run_event_history::{
    run_event_history_overview_from_shell, RUN_EVENT_HISTORY_COMMAND_NAME,
};

pub const RUN_EVENT_CONTRACT_NAME: &str = "RunEventContract";
pub const RUN_EVENT_CONTRACT_COMMAND_NAME: &str = "RunEventContractState";
pub const SUPPORTED_RUN_EVENT_TYPES: [RunEventType; 31] = [
    RunEventType::RunStateChanged,
    RunEventType::StatusNotePublished,
    RunEventType::RunQueued,
    RunEventType::IngestionStarted,
    RunEventType::IngestionCompleted,
    RunEventType::PersonaGenerationStarted,
    RunEventType::PersonaGenerationCompleted,
    RunEventType::RoundStarted,
    RunEventType::RoundCompleted,
    RunEventType::BudgetUsageUpdated,
    RunEventType::TokenDurationTracked,
    RunEventType::PausedRateLimit,
    RunEventType::PausedNetwork,
    RunEventType::CancellationRequested,
    RunEventType::CancellationAccepted,
    RunEventType::CancellationRejected,
    RunEventType::FinalizationStarted,
    RunEventType::FinalizationCompleted,
    RunEventType::ReportGenerationStarted,
    RunEventType::ReportGenerationSucceeded,
    RunEventType::ReportGenerationFailed,
    RunEventType::FailureRecorded,
    RunEventType::CleanupPending,
    RunEventType::CleanupStarted,
    RunEventType::CleanupCompleted,
    RunEventType::CleanupPartiallyFailed,
    RunEventType::CleanupFailed,
    RunEventType::StartupJanitorPendingNextLaunch,
    RunEventType::StartupJanitorStarted,
    RunEventType::StartupJanitorPurgeSucceeded,
    RunEventType::StartupJanitorPurgeFailed,
];
pub const SUPPORTED_RUN_EVENT_RUNTIME_DOMAINS: [RunEventRuntimeDomain; 7] = [
    RunEventRuntimeDomain::Shell,
    RunEventRuntimeDomain::Engine,
    RunEventRuntimeDomain::Debate,
    RunEventRuntimeDomain::Monitoring,
    RunEventRuntimeDomain::Control,
    RunEventRuntimeDomain::Cleanup,
    RunEventRuntimeDomain::Janitor,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunEventContractStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunEventType {
    RunStateChanged,
    StatusNotePublished,
    RunQueued,
    IngestionStarted,
    IngestionCompleted,
    PersonaGenerationStarted,
    PersonaGenerationCompleted,
    RoundStarted,
    RoundCompleted,
    BudgetUsageUpdated,
    TokenDurationTracked,
    PausedRateLimit,
    PausedNetwork,
    CancellationRequested,
    CancellationAccepted,
    CancellationRejected,
    FinalizationStarted,
    FinalizationCompleted,
    ReportGenerationStarted,
    ReportGenerationSucceeded,
    ReportGenerationFailed,
    FailureRecorded,
    CleanupPending,
    CleanupStarted,
    CleanupCompleted,
    CleanupPartiallyFailed,
    CleanupFailed,
    StartupJanitorPendingNextLaunch,
    StartupJanitorStarted,
    StartupJanitorPurgeSucceeded,
    StartupJanitorPurgeFailed,
}

impl RunEventType {
    pub fn name(self) -> &'static str {
        match self {
            Self::RunStateChanged => "RunStateChanged",
            Self::StatusNotePublished => "StatusNotePublished",
            Self::RunQueued => "RunQueued",
            Self::IngestionStarted => "IngestionStarted",
            Self::IngestionCompleted => "IngestionCompleted",
            Self::PersonaGenerationStarted => "PersonaGenerationStarted",
            Self::PersonaGenerationCompleted => "PersonaGenerationCompleted",
            Self::RoundStarted => "RoundStarted",
            Self::RoundCompleted => "RoundCompleted",
            Self::BudgetUsageUpdated => "BudgetUsageUpdated",
            Self::TokenDurationTracked => "TokenDurationTracked",
            Self::PausedRateLimit => "PausedRateLimit",
            Self::PausedNetwork => "PausedNetwork",
            Self::CancellationRequested => "CancellationRequested",
            Self::CancellationAccepted => "CancellationAccepted",
            Self::CancellationRejected => "CancellationRejected",
            Self::FinalizationStarted => "FinalizationStarted",
            Self::FinalizationCompleted => "FinalizationCompleted",
            Self::ReportGenerationStarted => "ReportGenerationStarted",
            Self::ReportGenerationSucceeded => "ReportGenerationSucceeded",
            Self::ReportGenerationFailed => "ReportGenerationFailed",
            Self::FailureRecorded => "FailureRecorded",
            Self::CleanupPending => "CleanupPending",
            Self::CleanupStarted => "CleanupStarted",
            Self::CleanupCompleted => "CleanupCompleted",
            Self::CleanupPartiallyFailed => "CleanupPartiallyFailed",
            Self::CleanupFailed => "CleanupFailed",
            Self::StartupJanitorPendingNextLaunch => "StartupJanitorPendingNextLaunch",
            Self::StartupJanitorStarted => "StartupJanitorStarted",
            Self::StartupJanitorPurgeSucceeded => "StartupJanitorPurgeSucceeded",
            Self::StartupJanitorPurgeFailed => "StartupJanitorPurgeFailed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunEventRuntimeDomain {
    Shell,
    Engine,
    Debate,
    Monitoring,
    Control,
    Cleanup,
    Janitor,
}

impl RunEventRuntimeDomain {
    pub fn name(self) -> &'static str {
        match self {
            Self::Shell => "Shell",
            Self::Engine => "Engine",
            Self::Debate => "Debate",
            Self::Monitoring => "Monitoring",
            Self::Control => "Control",
            Self::Cleanup => "Cleanup",
            Self::Janitor => "Janitor",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunEventGapContract {
    pub cursor_boundary_required: bool,
    pub after_sequence_number_supported: bool,
    pub inclusive_replay_supported: bool,
    pub gap_marker_supported: bool,
    pub missing_history_inference_allowed: bool,
    pub bounded_page_retrieval_required: bool,
    pub latest_sequence_boundary_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunEventContractSnapshot {
    pub stage: RunEventContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub event_history_command_name: &'static str,
    pub supported_event_types: [RunEventType; 31],
    pub supported_runtime_domains: [RunEventRuntimeDomain; 7],
    pub event_id_required: bool,
    pub sequence_number_required: bool,
    pub occurred_at_required: bool,
    pub session_id_required: bool,
    pub event_type_required: bool,
    pub payload_required: bool,
    pub runtime_domain_required: bool,
    pub ordered_retrieval_explicit: bool,
    pub incremental_after_sequence_supported: bool,
    pub latest_sequence_boundary_supported: bool,
    pub history_complete_flag_supported: bool,
    pub event_history_retrieval_implemented: bool,
    pub gap_contract: RunEventGapContract,
    pub event_history_materialized: bool,
    pub live_event_capture_materialized: bool,
    pub finalization_events_distinct_from_report_generation_events: bool,
    pub report_generation_event_vocabulary_materialized: bool,
    pub cleanup_event_vocabulary_materialized: bool,
    pub startup_janitor_event_vocabulary_materialized: bool,
    pub cleanup_events_distinct_from_run_outcome: bool,
    pub startup_janitor_events_distinct_from_run_outcome: bool,
    pub startup_janitor_events_imply_crash_resume: bool,
    pub finalization_events_imply_report_availability: bool,
    pub transcript_event_chronology_materialized: bool,
    pub payload_secret_material_allowed: bool,
    pub frontend_ordering_authority_allowed: bool,
    pub latest_available_sequence_number: Option<u64>,
}

impl RunEventContractSnapshot {
    pub fn summary(self) -> String {
        let supported_event_types = self.supported_event_types.map(RunEventType::name).join(", ");
        let supported_runtime_domains = self
            .supported_runtime_domains
            .map(RunEventRuntimeDomain::name)
            .join(", ");

        format!(
            "Run event contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; event-history command name is {}; supported event types are {}; supported runtime domains are {}; event id required is {}; sequence number required is {}; occurred-at required is {}; session id required is {}; event type required is {}; payload required is {}; runtime domain required is {}; ordered retrieval explicit is {}; incremental after-sequence supported is {}; latest sequence boundary supported is {}; history-complete flag supported is {}; event-history retrieval implemented is {}; cursor boundary required is {}; after-sequence supported is {}; inclusive replay supported is {}; gap marker supported is {}; missing-history inference allowed is {}; bounded page retrieval required is {}; event history materialized is {}; live event capture materialized is {}; finalization events distinct from report generation events is {}; report-generation event vocabulary materialized is {}; cleanup event vocabulary materialized is {}; startup-janitor event vocabulary materialized is {}; cleanup events distinct from run outcome is {}; startup-janitor events distinct from run outcome is {}; startup-janitor events imply crash resume is {}; finalization events imply report availability is {}; transcript-event chronology materialized is {}; payload secret material allowed is {}; frontend ordering authority allowed is {}; latest available sequence number is {:?}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            self.event_history_command_name,
            supported_event_types,
            supported_runtime_domains,
            self.event_id_required,
            self.sequence_number_required,
            self.occurred_at_required,
            self.session_id_required,
            self.event_type_required,
            self.payload_required,
            self.runtime_domain_required,
            self.ordered_retrieval_explicit,
            self.incremental_after_sequence_supported,
            self.latest_sequence_boundary_supported,
            self.history_complete_flag_supported,
            self.event_history_retrieval_implemented,
            self.gap_contract.cursor_boundary_required,
            self.gap_contract.after_sequence_number_supported,
            self.gap_contract.inclusive_replay_supported,
            self.gap_contract.gap_marker_supported,
            self.gap_contract.missing_history_inference_allowed,
            self.gap_contract.bounded_page_retrieval_required,
            self.event_history_materialized,
            self.live_event_capture_materialized,
            self.finalization_events_distinct_from_report_generation_events,
            self.report_generation_event_vocabulary_materialized,
            self.cleanup_event_vocabulary_materialized,
            self.startup_janitor_event_vocabulary_materialized,
            self.cleanup_events_distinct_from_run_outcome,
            self.startup_janitor_events_distinct_from_run_outcome,
            self.startup_janitor_events_imply_crash_resume,
            self.finalization_events_imply_report_availability,
            self.transcript_event_chronology_materialized,
            self.payload_secret_material_allowed,
            self.frontend_ordering_authority_allowed,
            self.latest_available_sequence_number,
        )
    }
}

pub fn run_event_contract_state() -> RunEventContractSnapshot {
    let history_overview = run_event_history_overview_from_shell();

    RunEventContractSnapshot {
        stage: RunEventContractStage::FoundationMaterialized,
        contract_name: RUN_EVENT_CONTRACT_NAME,
        shell_command_name: RUN_EVENT_CONTRACT_COMMAND_NAME,
        event_history_command_name: RUN_EVENT_HISTORY_COMMAND_NAME,
        supported_event_types: SUPPORTED_RUN_EVENT_TYPES,
        supported_runtime_domains: SUPPORTED_RUN_EVENT_RUNTIME_DOMAINS,
        event_id_required: true,
        sequence_number_required: true,
        occurred_at_required: true,
        session_id_required: true,
        event_type_required: true,
        payload_required: true,
        runtime_domain_required: true,
        ordered_retrieval_explicit: true,
        incremental_after_sequence_supported: true,
        latest_sequence_boundary_supported: true,
        history_complete_flag_supported: true,
        event_history_retrieval_implemented: true,
        gap_contract: RunEventGapContract {
            cursor_boundary_required: true,
            after_sequence_number_supported: true,
            inclusive_replay_supported: false,
            gap_marker_supported: true,
            missing_history_inference_allowed: false,
            bounded_page_retrieval_required: true,
            latest_sequence_boundary_supported: true,
        },
        event_history_materialized: history_overview.event_history_materialized,
        live_event_capture_materialized: true,
        finalization_events_distinct_from_report_generation_events: true,
        report_generation_event_vocabulary_materialized: true,
        cleanup_event_vocabulary_materialized: true,
        startup_janitor_event_vocabulary_materialized: true,
        cleanup_events_distinct_from_run_outcome: true,
        startup_janitor_events_distinct_from_run_outcome: true,
        startup_janitor_events_imply_crash_resume: false,
        finalization_events_imply_report_availability: false,
        transcript_event_chronology_materialized: history_overview
            .transcript_event_chronology_materialized,
        payload_secret_material_allowed: false,
        frontend_ordering_authority_allowed: false,
        latest_available_sequence_number: history_overview.latest_available_sequence_number,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_contract_freezes_ordered_envelope_semantics_without_claiming_history() {
        let snapshot = run_event_contract_state();

        assert_eq!(snapshot.contract_name, RUN_EVENT_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, RUN_EVENT_CONTRACT_COMMAND_NAME);
        assert_eq!(snapshot.supported_event_types.len(), 31);
        assert_eq!(snapshot.supported_runtime_domains.len(), 7);
        assert!(snapshot.event_id_required);
        assert!(snapshot.sequence_number_required);
        assert!(snapshot.occurred_at_required);
        assert!(snapshot.session_id_required);
        assert!(snapshot.event_type_required);
        assert!(snapshot.payload_required);
        assert!(snapshot.runtime_domain_required);
        assert!(snapshot.ordered_retrieval_explicit);
        assert!(snapshot.incremental_after_sequence_supported);
        assert!(snapshot.history_complete_flag_supported);
        assert!(snapshot.event_history_retrieval_implemented);
        assert!(snapshot.gap_contract.gap_marker_supported);
        assert!(!snapshot.gap_contract.missing_history_inference_allowed);
        assert!(snapshot.event_history_materialized);
        assert!(snapshot.live_event_capture_materialized);
        assert!(snapshot.finalization_events_distinct_from_report_generation_events);
        assert!(snapshot.report_generation_event_vocabulary_materialized);
        assert!(snapshot.cleanup_event_vocabulary_materialized);
        assert!(snapshot.startup_janitor_event_vocabulary_materialized);
        assert!(snapshot.cleanup_events_distinct_from_run_outcome);
        assert!(snapshot.startup_janitor_events_distinct_from_run_outcome);
        assert!(!snapshot.startup_janitor_events_imply_crash_resume);
        assert!(!snapshot.finalization_events_imply_report_availability);
        assert!(snapshot.transcript_event_chronology_materialized);
        assert!(!snapshot.payload_secret_material_allowed);
        assert!(!snapshot.frontend_ordering_authority_allowed);
        assert_eq!(snapshot.latest_available_sequence_number, None);
    }
}