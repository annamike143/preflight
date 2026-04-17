use serde::{Deserialize, Serialize};

use crate::activation_readiness::activation_readiness_truth_surface;
use crate::preflight_readiness::{preflight_readiness_truth_surface, PreflightState};
use crate::provider_readiness::provider_readiness_truth_surface;
use crate::run_bounded_memory::run_bounded_memory_surface;
use crate::run_session_contract::run_session_contract_basis;
use crate::runtime_persona_contract::runtime_persona_contract_surface;
use crate::run_start_lineage::run_start_lineage_surface;
use crate::run_workspace_boundary::run_workspace_boundary_surface;

pub const STATE_RUN_START_FOUNDATION_NOT_STARTED: &str = "RunStartFoundationNotStarted";
pub const STATE_RUN_START_FOUNDATION_BLOCKED: &str = "RunStartFoundationBlocked";
pub const STATE_RUN_START_FOUNDATION_ERROR: &str = "RunStartFoundationError";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStartReadinessStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStartFoundationState {
    NotStarted,
    Blocked,
    Error,
}

impl RunStartFoundationState {
    pub fn state_name(self) -> &'static str {
        match self {
            Self::NotStarted => STATE_RUN_START_FOUNDATION_NOT_STARTED,
            Self::Blocked => STATE_RUN_START_FOUNDATION_BLOCKED,
            Self::Error => STATE_RUN_START_FOUNDATION_ERROR,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunStartReadinessSnapshot {
    pub stage: RunStartReadinessStage,
    pub state_surface_materialized: bool,
    pub run_session_contract_defined: bool,
    pub workspace_boundary_defined: bool,
    pub start_lineage_basis_materialized: bool,
    pub runtime_persona_contract_defined: bool,
    pub bounded_memory_contract_defined: bool,
    pub run_start_command_explicit: bool,
    pub shell_owned_run_session_identity_authority: bool,
    pub shell_owned_workspace_allocation_authority: bool,
    pub shell_owned_engine_start_authorization: bool,
    pub runtime_readiness_evaluated_separately: bool,
    pub activation_readiness_evaluated_separately: bool,
    pub provider_readiness_evaluated_separately: bool,
    pub preflight_readiness_evaluated_separately: bool,
    pub runtime_ready_now: bool,
    pub activation_ready_now: bool,
    pub provider_configuration_ready_now: bool,
    pub current_eligible_preflight_now: bool,
    pub downgrade_acceptance_required: bool,
    pub downgrade_acceptance_recorded: bool,
    pub start_lineage_ready_now: bool,
    pub current_state: RunStartFoundationState,
    pub current_state_name: &'static str,
    pub run_start_execution_materialized: bool,
    pub workspace_allocation_materialized: bool,
    pub runtime_persona_foundation_materialized: bool,
    pub bounded_memory_foundation_materialized: bool,
    pub transcript_archive_separation_materialized: bool,
    pub fresh_workspace_required: bool,
    pub hidden_continuation_allowed: bool,
    pub real_run_start_authorized_now: bool,
    pub would_depend_on_current_eligible_preflight_if_execution_existed: bool,
    pub current_status_note: String,
}

impl RunStartReadinessSnapshot {
    pub fn supported_states(&self) -> [RunStartFoundationState; 3] {
        [
            RunStartFoundationState::NotStarted,
            RunStartFoundationState::Blocked,
            RunStartFoundationState::Error,
        ]
    }

    pub fn blocking_reasons(&self) -> Vec<&'static str> {
        let mut reasons = Vec::new();

        if !self.run_session_contract_defined {
            reasons.push("run/session contract is not defined");
        }

        if !self.workspace_boundary_defined {
            reasons.push("workspace boundary is not defined");
        }

        if !self.start_lineage_basis_materialized {
            reasons.push("run-start lineage basis is not materialized");
        }

        if !self.runtime_persona_contract_defined {
            reasons.push("runtime persona contract is not defined");
        }

        if !self.bounded_memory_contract_defined {
            reasons.push("bounded memory contract is not defined");
        }

        if !self.runtime_ready_now {
            reasons.push("runtime readiness is not satisfied");
        }

        if !self.activation_ready_now {
            reasons.push("activation readiness is not satisfied");
        }

        if !self.provider_configuration_ready_now {
            reasons.push("provider configuration readiness is not satisfied");
        }

        if !self.current_eligible_preflight_now {
            reasons.push("current eligible Preflight truth does not exist");
        }

        if !self.start_lineage_ready_now {
            reasons.push("run-start lineage is not ready from current Preflight truth");
        }

        if !self.workspace_allocation_materialized {
            reasons.push("real workspace allocation is not materialized yet");
        }

        if !self.runtime_persona_foundation_materialized {
            reasons.push("runtime persona foundation is not materialized yet");
        }

        if !self.bounded_memory_foundation_materialized {
            reasons.push("bounded memory foundation is not materialized yet");
        }

        if !self.transcript_archive_separation_materialized {
            reasons.push("transcript archive separation is not materialized yet");
        }

        reasons
    }

    pub fn summary(&self) -> String {
        let blocking_reasons = self.blocking_reasons().join(", ");
        let supported_state_count = self.supported_states().len();

        format!(
            "Run-start foundation truth surface is materialized. Stage is {:?}; state surface materialized is {}; run/session contract defined is {}; workspace boundary defined is {}; start-lineage basis materialized is {}; runtime persona contract defined is {}; bounded memory contract defined is {}; run-start command explicit is {}; shell-owned run/session identity authority is {}; shell-owned workspace allocation authority is {}; shell-owned engine-start authorization is {}; runtime readiness evaluated separately is {}; activation readiness evaluated separately is {}; provider readiness evaluated separately is {}; Preflight readiness evaluated separately is {}; runtime ready now is {}; activation ready now is {}; provider configuration ready now is {}; current eligible Preflight now is {}; downgrade acceptance required is {}; downgrade acceptance recorded is {}; start lineage ready now is {}; current state is {:?}; current state name is {}; supported state count is {}; run-start execution materialized is {}; workspace allocation materialized is {}; runtime persona foundation materialized is {}; bounded-memory foundation materialized is {}; transcript-archive separation materialized is {}; fresh workspace required is {}; hidden continuation allowed is {}; real run start authorized now is {}; current eligible Preflight would still be required if execution existed is {}; current status note is {}; blocking reasons are {}.",
            self.stage,
            self.state_surface_materialized,
            self.run_session_contract_defined,
            self.workspace_boundary_defined,
            self.start_lineage_basis_materialized,
            self.runtime_persona_contract_defined,
            self.bounded_memory_contract_defined,
            self.run_start_command_explicit,
            self.shell_owned_run_session_identity_authority,
            self.shell_owned_workspace_allocation_authority,
            self.shell_owned_engine_start_authorization,
            self.runtime_readiness_evaluated_separately,
            self.activation_readiness_evaluated_separately,
            self.provider_readiness_evaluated_separately,
            self.preflight_readiness_evaluated_separately,
            self.runtime_ready_now,
            self.activation_ready_now,
            self.provider_configuration_ready_now,
            self.current_eligible_preflight_now,
            self.downgrade_acceptance_required,
            self.downgrade_acceptance_recorded,
            self.start_lineage_ready_now,
            self.current_state,
            self.current_state_name,
            supported_state_count,
            self.run_start_execution_materialized,
            self.workspace_allocation_materialized,
            self.runtime_persona_foundation_materialized,
            self.bounded_memory_foundation_materialized,
            self.transcript_archive_separation_materialized,
            self.fresh_workspace_required,
            self.hidden_continuation_allowed,
            self.real_run_start_authorized_now,
            self.would_depend_on_current_eligible_preflight_if_execution_existed,
            self.current_status_note,
            blocking_reasons,
        )
    }
}

pub fn run_start_readiness_truth_surface() -> RunStartReadinessSnapshot {
    let run_session_contract = run_session_contract_basis();
    let workspace_boundary = run_workspace_boundary_surface();
    let start_lineage = run_start_lineage_surface();
    let runtime_persona_contract = runtime_persona_contract_surface();
    let bounded_memory = run_bounded_memory_surface();
    let activation_readiness = activation_readiness_truth_surface();
    let provider_readiness = provider_readiness_truth_surface();
    let preflight_readiness = preflight_readiness_truth_surface();
    let current_state = resolve_run_start_foundation_state(&preflight_readiness);
    let current_status_note = resolve_run_start_status_note(current_state, &preflight_readiness);

    RunStartReadinessSnapshot {
        stage: RunStartReadinessStage::FoundationMaterialized,
        state_surface_materialized: true,
        run_session_contract_defined: run_session_contract.request_contract.current_eligible_preflight_required
            && run_session_contract.identifier_contract.run_id_required
            && run_session_contract.identifier_contract.session_id_required,
        workspace_boundary_defined: workspace_boundary.fresh_workspace_required
            && workspace_boundary.no_active_workspace_reuse,
        start_lineage_basis_materialized: start_lineage.lineage_basis_materialized,
        runtime_persona_contract_defined: runtime_persona_contract.runtime_generated_from_seed_context
            && runtime_persona_contract.exactly_one_moderator_required,
        bounded_memory_contract_defined: bounded_memory.seed_grounding_memory_required
            && bounded_memory.transcript_archive_separate_from_prompt_memory,
        run_start_command_explicit: run_session_contract.run_start_command_explicit,
        shell_owned_run_session_identity_authority: run_session_contract
            .shell_owned_run_session_identity_required,
        shell_owned_workspace_allocation_authority: workspace_boundary.shell_owned_workspace_allocation,
        shell_owned_engine_start_authorization: true,
        runtime_readiness_evaluated_separately: true,
        activation_readiness_evaluated_separately: true,
        provider_readiness_evaluated_separately: true,
        preflight_readiness_evaluated_separately: true,
        runtime_ready_now: activation_readiness.runtime_ready_now,
        activation_ready_now: activation_readiness.activation_ready_for_licensed_use,
        provider_configuration_ready_now: provider_readiness.provider_configuration_ready,
        current_eligible_preflight_now: preflight_readiness.run_start_eligible_now,
        downgrade_acceptance_required: preflight_readiness.downgrade_acceptance_required,
        downgrade_acceptance_recorded: preflight_readiness.downgrade_acceptance_recorded,
        start_lineage_ready_now: start_lineage.future_execution_lineage_ready,
        current_state,
        current_state_name: current_state.state_name(),
        run_start_execution_materialized: run_session_contract.run_start_execution_materialized,
        workspace_allocation_materialized: workspace_boundary.workspace_allocation_implemented,
        runtime_persona_foundation_materialized: runtime_persona_contract
            .runtime_persona_generation_materialized,
        bounded_memory_foundation_materialized: bounded_memory.bounded_memory_foundation_materialized,
        transcript_archive_separation_materialized: bounded_memory
            .transcript_archive_foundation_materialized,
        fresh_workspace_required: workspace_boundary.fresh_workspace_required,
        hidden_continuation_allowed: workspace_boundary.hidden_continuation_allowed,
        real_run_start_authorized_now: run_session_contract.run_start_execution_materialized
            && workspace_boundary.workspace_allocation_implemented
            && runtime_persona_contract.runtime_persona_generation_materialized
            && bounded_memory.bounded_memory_foundation_materialized
            && bounded_memory.transcript_archive_foundation_materialized
            && activation_readiness.runtime_ready_now
            && activation_readiness.activation_ready_for_licensed_use
            && provider_readiness.provider_configuration_ready
            && preflight_readiness.run_start_eligible_now
            && start_lineage.future_execution_lineage_ready,
        would_depend_on_current_eligible_preflight_if_execution_existed: preflight_readiness.run_start_eligible_now,
        current_status_note,
    }
}

fn resolve_run_start_foundation_state(
    preflight_readiness: &crate::preflight_readiness::PreflightReadinessSnapshot,
) -> RunStartFoundationState {
    if preflight_readiness.current_state == PreflightState::Error
        || preflight_readiness.last_technical_failure_reason.is_some()
    {
        return RunStartFoundationState::Error;
    }

    if !preflight_readiness.pending_seed_document_selected && !preflight_readiness.requested_plan_present {
        return RunStartFoundationState::NotStarted;
    }

    RunStartFoundationState::Blocked
}

fn resolve_run_start_status_note(
    current_state: RunStartFoundationState,
    preflight_readiness: &crate::preflight_readiness::PreflightReadinessSnapshot,
) -> String {
    match current_state {
        RunStartFoundationState::NotStarted => {
            "Run-start foundation is materialized, but current eligible Preflight lineage does not exist yet"
                .to_string()
        }
        RunStartFoundationState::Blocked => format!(
            "Run-start foundation is materialized, but current readiness prerequisites still block a fresh run/session start. {}",
            preflight_readiness.current_status_note
        ),
        RunStartFoundationState::Error => format!(
            "Run-start foundation cannot be evaluated cleanly because the upstream Preflight surface is reporting a technical failure. {}",
            preflight_readiness.current_status_note
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_start_readiness_truth_surface_stays_boundary_only_and_separates_domains() {
        let snapshot = run_start_readiness_truth_surface();

        assert!(snapshot.state_surface_materialized);
        assert!(snapshot.run_session_contract_defined);
        assert!(snapshot.workspace_boundary_defined);
        assert!(snapshot.start_lineage_basis_materialized);
        assert!(snapshot.runtime_persona_contract_defined);
        assert!(snapshot.bounded_memory_contract_defined);
        assert!(snapshot.run_start_command_explicit);
        assert!(snapshot.shell_owned_run_session_identity_authority);
        assert!(snapshot.shell_owned_workspace_allocation_authority);
        assert!(snapshot.shell_owned_engine_start_authorization);
        assert!(snapshot.runtime_readiness_evaluated_separately);
        assert!(snapshot.activation_readiness_evaluated_separately);
        assert!(snapshot.provider_readiness_evaluated_separately);
        assert!(snapshot.preflight_readiness_evaluated_separately);
        assert_eq!(snapshot.supported_states().len(), 3);
        assert!(snapshot.run_start_execution_materialized);
        assert!(snapshot.workspace_allocation_materialized);
        assert!(snapshot.runtime_persona_foundation_materialized);
        assert!(snapshot.bounded_memory_foundation_materialized);
        assert!(snapshot.transcript_archive_separation_materialized);
        assert!(snapshot.fresh_workspace_required);
        assert!(!snapshot.hidden_continuation_allowed);
        assert!(!snapshot.real_run_start_authorized_now);
        assert!(!snapshot.current_state_name.is_empty());
        assert!(!snapshot.blocking_reasons().is_empty());
    }
}