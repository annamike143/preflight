use serde::{Deserialize, Serialize};

pub const RUN_SESSION_CONTRACT_NAME: &str = "RunSessionContract";
pub const RUN_START_COMMAND_NAME: &str = "StartRunSession";
pub const RUN_STATUS_STATE_COMMAND_NAME: &str = "RunStatusState";
pub const RUN_EVENT_CONTRACT_COMMAND_NAME: &str = "RunEventContractState";
pub const RUN_EVENT_HISTORY_COMMAND_NAME: &str = "RunEventHistory";
pub const RUN_CANCELLATION_COMMAND_NAME: &str = "CancelRunSession";
pub const SUPPORTED_RUN_SESSION_STATE_CLASSES: [RunSessionStateClass; 6] = [
    RunSessionStateClass::NotStarted,
    RunSessionStateClass::StartBlocked,
    RunSessionStateClass::TechnicalFailure,
    RunSessionStateClass::PendingExecution,
    RunSessionStateClass::Running,
    RunSessionStateClass::Finished,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunSessionContractStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunSessionStateClass {
    NotStarted,
    StartBlocked,
    TechnicalFailure,
    PendingExecution,
    Running,
    Finished,
}

impl RunSessionStateClass {
    pub fn label(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::StartBlocked => "start_blocked",
            Self::TechnicalFailure => "technical_failure",
            Self::PendingExecution => "pending_execution",
            Self::Running => "running",
            Self::Finished => "finished",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunStartRequestContract {
    pub current_eligible_preflight_required: bool,
    pub downgrade_acceptance_required_when_present: bool,
    pub runtime_readiness_required: bool,
    pub activation_readiness_required: bool,
    pub provider_configuration_required: bool,
    pub shell_command_boundary_required: bool,
    pub frontend_supplied_run_session_identifier_allowed: bool,
    pub frontend_direct_workspace_authority_allowed: bool,
    pub direct_engine_start_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunSessionIdentifierContract {
    pub run_id_required: bool,
    pub session_id_required: bool,
    pub workspace_reference_required: bool,
    pub shell_owned_generation_required: bool,
    pub fresh_identity_per_run_required: bool,
    pub hidden_continuation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunStartResponseContract {
    pub start_blocked_supported: bool,
    pub technical_failure_supported: bool,
    pub coarse_run_state_class_return_supported: bool,
    pub execution_started_supported: bool,
    pub final_report_implied_by_response: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunCancellationRequestContract {
    pub shell_command_boundary_required: bool,
    pub shell_owned_cancelability_authority_required: bool,
    pub active_run_identifier_required_when_cancelable: bool,
    pub frontend_direct_engine_signal_allowed: bool,
    pub cancelability_derived_from_canonical_state: bool,
    pub cancellation_command_explicit: bool,
    pub cancellation_command_implemented: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunCancellationResponseContract {
    pub accepted_response_defined: bool,
    pub rejected_response_defined: bool,
    pub already_terminal_response_defined: bool,
    pub no_active_run_response_defined: bool,
    pub hidden_post_cancel_progress_allowed: bool,
    pub forceful_termination_implied_by_acceptance: bool,
    pub controlled_cancellation_behavior_materialized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunSessionContractSnapshot {
    pub stage: RunSessionContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub run_status_command_name: &'static str,
    pub run_event_contract_command_name: &'static str,
    pub run_event_history_command_name: &'static str,
    pub run_cancellation_command_name: &'static str,
    pub request_contract: RunStartRequestContract,
    pub identifier_contract: RunSessionIdentifierContract,
    pub response_contract: RunStartResponseContract,
    pub cancellation_request_contract: RunCancellationRequestContract,
    pub cancellation_response_contract: RunCancellationResponseContract,
    pub shell_owned_run_session_identity_required: bool,
    pub supported_state_classes_materialized: bool,
    pub canonical_run_lifecycle_surface_defined: bool,
    pub run_status_contract_defined: bool,
    pub run_event_contract_defined: bool,
    pub run_start_command_explicit: bool,
    pub run_start_execution_materialized: bool,
    pub run_session_identity_allocation_materialized: bool,
    pub workspace_reference_allocation_materialized: bool,
    pub in_flight_snapshot_publication_materialized: bool,
    pub monitoring_availability_basis_materialized: bool,
    pub runtime_persona_foundation_materialized: bool,
    pub bounded_memory_foundation_materialized: bool,
    pub transcript_archive_separation_materialized: bool,
    pub moderated_round_execution_materialized: bool,
    pub run_local_token_duration_tracking_materialized: bool,
    pub live_monitoring_implied: bool,
}

impl RunSessionContractSnapshot {
    pub fn supported_state_classes(self) -> [RunSessionStateClass; 6] {
        SUPPORTED_RUN_SESSION_STATE_CLASSES
    }

    pub fn summary(self) -> String {
        let supported_state_classes = self
            .supported_state_classes()
            .map(RunSessionStateClass::label)
            .join(", ");

        format!(
            "Run/session contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; run-status command name is {}; run-event contract command name is {}; run-event history command name is {}; run-cancellation command name is {}; supported state classes are {}; current eligible Preflight required is {}; shell-owned generation required is {}; execution-started supported is {}; cancellation shell-command boundary required is {}; cancellation cancelability derived from canonical state is {}; cancellation command explicit is {}; cancellation command implemented is {}; cancellation accepted response defined is {}; cancellation rejected response defined is {}; hidden post-cancel progress allowed is {}; controlled cancellation behavior materialized is {}; shell-owned run/session identity required is {}; supported state classes materialized is {}; canonical run-lifecycle surface defined is {}; run-status contract defined is {}; run-event contract defined is {}; run-start execution materialized is {}; in-flight snapshot publication materialized is {}; monitoring-availability basis materialized is {}; runtime persona foundation materialized is {}; bounded-memory foundation materialized is {}; transcript-archive separation materialized is {}; moderated-round execution materialized is {}; run-local token-and-duration tracking materialized is {}; live monitoring implied is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            self.run_status_command_name,
            self.run_event_contract_command_name,
            self.run_event_history_command_name,
            self.run_cancellation_command_name,
            supported_state_classes,
            self.request_contract.current_eligible_preflight_required,
            self.identifier_contract.shell_owned_generation_required,
            self.response_contract.execution_started_supported,
            self.cancellation_request_contract.shell_command_boundary_required,
            self.cancellation_request_contract.cancelability_derived_from_canonical_state,
            self.cancellation_request_contract.cancellation_command_explicit,
            self.cancellation_request_contract.cancellation_command_implemented,
            self.cancellation_response_contract.accepted_response_defined,
            self.cancellation_response_contract.rejected_response_defined,
            self.cancellation_response_contract.hidden_post_cancel_progress_allowed,
            self.cancellation_response_contract.controlled_cancellation_behavior_materialized,
            self.shell_owned_run_session_identity_required,
            self.supported_state_classes_materialized,
            self.canonical_run_lifecycle_surface_defined,
            self.run_status_contract_defined,
            self.run_event_contract_defined,
            self.run_start_execution_materialized,
            self.in_flight_snapshot_publication_materialized,
            self.monitoring_availability_basis_materialized,
            self.runtime_persona_foundation_materialized,
            self.bounded_memory_foundation_materialized,
            self.transcript_archive_separation_materialized,
            self.moderated_round_execution_materialized,
            self.run_local_token_duration_tracking_materialized,
            self.live_monitoring_implied
        )
    }
}

pub fn run_session_contract_basis() -> RunSessionContractSnapshot {
    RunSessionContractSnapshot {
        stage: RunSessionContractStage::FoundationMaterialized,
        contract_name: RUN_SESSION_CONTRACT_NAME,
        shell_command_name: RUN_START_COMMAND_NAME,
        run_status_command_name: RUN_STATUS_STATE_COMMAND_NAME,
        run_event_contract_command_name: RUN_EVENT_CONTRACT_COMMAND_NAME,
        run_event_history_command_name: RUN_EVENT_HISTORY_COMMAND_NAME,
        run_cancellation_command_name: RUN_CANCELLATION_COMMAND_NAME,
        request_contract: RunStartRequestContract {
            current_eligible_preflight_required: true,
            downgrade_acceptance_required_when_present: true,
            runtime_readiness_required: true,
            activation_readiness_required: true,
            provider_configuration_required: true,
            shell_command_boundary_required: true,
            frontend_supplied_run_session_identifier_allowed: false,
            frontend_direct_workspace_authority_allowed: false,
            direct_engine_start_allowed: false,
        },
        identifier_contract: RunSessionIdentifierContract {
            run_id_required: true,
            session_id_required: true,
            workspace_reference_required: true,
            shell_owned_generation_required: true,
            fresh_identity_per_run_required: true,
            hidden_continuation_allowed: false,
        },
        response_contract: RunStartResponseContract {
            start_blocked_supported: true,
            technical_failure_supported: true,
            coarse_run_state_class_return_supported: true,
            execution_started_supported: true,
            final_report_implied_by_response: false,
        },
        cancellation_request_contract: RunCancellationRequestContract {
            shell_command_boundary_required: true,
            shell_owned_cancelability_authority_required: true,
            active_run_identifier_required_when_cancelable: true,
            frontend_direct_engine_signal_allowed: false,
            cancelability_derived_from_canonical_state: true,
            cancellation_command_explicit: true,
            cancellation_command_implemented: true,
        },
        cancellation_response_contract: RunCancellationResponseContract {
            accepted_response_defined: true,
            rejected_response_defined: true,
            already_terminal_response_defined: true,
            no_active_run_response_defined: true,
            hidden_post_cancel_progress_allowed: false,
            forceful_termination_implied_by_acceptance: false,
            controlled_cancellation_behavior_materialized: true,
        },
        shell_owned_run_session_identity_required: true,
        supported_state_classes_materialized: true,
        canonical_run_lifecycle_surface_defined: true,
        run_status_contract_defined: true,
        run_event_contract_defined: true,
        run_start_command_explicit: true,
        run_start_execution_materialized: true,
        run_session_identity_allocation_materialized: true,
        workspace_reference_allocation_materialized: true,
        in_flight_snapshot_publication_materialized: true,
        monitoring_availability_basis_materialized: true,
        runtime_persona_foundation_materialized: true,
        bounded_memory_foundation_materialized: true,
        transcript_archive_separation_materialized: true,
        moderated_round_execution_materialized: true,
        run_local_token_duration_tracking_materialized: true,
        live_monitoring_implied: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_session_contract_basis_stays_boundary_only_and_shell_owned() {
        let snapshot = run_session_contract_basis();

        assert_eq!(snapshot.contract_name, RUN_SESSION_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, RUN_START_COMMAND_NAME);
        assert_eq!(snapshot.run_status_command_name, RUN_STATUS_STATE_COMMAND_NAME);
        assert_eq!(
            snapshot.run_event_contract_command_name,
            RUN_EVENT_CONTRACT_COMMAND_NAME
        );
        assert_eq!(
            snapshot.run_cancellation_command_name,
            RUN_CANCELLATION_COMMAND_NAME
        );
        assert_eq!(snapshot.supported_state_classes().len(), 6);
        assert!(snapshot.request_contract.current_eligible_preflight_required);
        assert!(snapshot.request_contract.downgrade_acceptance_required_when_present);
        assert!(snapshot.request_contract.runtime_readiness_required);
        assert!(snapshot.request_contract.activation_readiness_required);
        assert!(snapshot.request_contract.provider_configuration_required);
        assert!(snapshot.request_contract.shell_command_boundary_required);
        assert!(!snapshot.request_contract.frontend_supplied_run_session_identifier_allowed);
        assert!(!snapshot.request_contract.frontend_direct_workspace_authority_allowed);
        assert!(!snapshot.request_contract.direct_engine_start_allowed);
        assert!(snapshot.identifier_contract.run_id_required);
        assert!(snapshot.identifier_contract.session_id_required);
        assert!(snapshot.identifier_contract.workspace_reference_required);
        assert!(snapshot.identifier_contract.shell_owned_generation_required);
        assert!(snapshot.identifier_contract.fresh_identity_per_run_required);
        assert!(!snapshot.identifier_contract.hidden_continuation_allowed);
        assert!(snapshot.response_contract.start_blocked_supported);
        assert!(snapshot.response_contract.technical_failure_supported);
        assert!(snapshot.response_contract.coarse_run_state_class_return_supported);
        assert!(snapshot.response_contract.execution_started_supported);
        assert!(!snapshot.response_contract.final_report_implied_by_response);
        assert!(snapshot
            .cancellation_request_contract
            .shell_command_boundary_required);
        assert!(snapshot
            .cancellation_request_contract
            .shell_owned_cancelability_authority_required);
        assert!(snapshot
            .cancellation_request_contract
            .active_run_identifier_required_when_cancelable);
        assert!(!snapshot
            .cancellation_request_contract
            .frontend_direct_engine_signal_allowed);
        assert!(snapshot
            .cancellation_request_contract
            .cancelability_derived_from_canonical_state);
        assert!(snapshot
            .cancellation_request_contract
            .cancellation_command_explicit);
        assert!(snapshot
            .cancellation_request_contract
            .cancellation_command_implemented);
        assert!(snapshot
            .cancellation_response_contract
            .accepted_response_defined);
        assert!(snapshot
            .cancellation_response_contract
            .rejected_response_defined);
        assert!(snapshot
            .cancellation_response_contract
            .already_terminal_response_defined);
        assert!(snapshot
            .cancellation_response_contract
            .no_active_run_response_defined);
        assert!(!snapshot
            .cancellation_response_contract
            .hidden_post_cancel_progress_allowed);
        assert!(!snapshot
            .cancellation_response_contract
            .forceful_termination_implied_by_acceptance);
        assert!(snapshot
            .cancellation_response_contract
            .controlled_cancellation_behavior_materialized);
        assert!(snapshot.shell_owned_run_session_identity_required);
        assert!(snapshot.supported_state_classes_materialized);
        assert!(snapshot.canonical_run_lifecycle_surface_defined);
        assert!(snapshot.run_status_contract_defined);
        assert!(snapshot.run_event_contract_defined);
        assert!(snapshot.run_start_command_explicit);
        assert!(snapshot.run_start_execution_materialized);
        assert!(snapshot.run_session_identity_allocation_materialized);
        assert!(snapshot.workspace_reference_allocation_materialized);
        assert!(snapshot.in_flight_snapshot_publication_materialized);
        assert!(snapshot.monitoring_availability_basis_materialized);
        assert!(snapshot.runtime_persona_foundation_materialized);
        assert!(snapshot.bounded_memory_foundation_materialized);
        assert!(snapshot.transcript_archive_separation_materialized);
        assert!(snapshot.moderated_round_execution_materialized);
        assert!(snapshot.run_local_token_duration_tracking_materialized);
        assert!(!snapshot.live_monitoring_implied);
    }
}