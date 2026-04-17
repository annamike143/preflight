import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import App from "./App";

const { invokeMock } = vi.hoisted(() => ({
  invokeMock: vi.fn()
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: invokeMock
}));

function buildProviderConfigurationState(overrides: Record<string, unknown> = {}) {
  return {
    flow_materialized: true,
    support_record_present: true,
    selected_provider: "open_ai",
    secure_store_lookup_reference: "provider-api-key.open_ai",
    last_configuration_outcome: "configured_locally",
    provider_secret_reference_present: true,
    provider_secret_fingerprint_verified: true,
    configuration_present: true,
    current_status: "present",
    current_detail: "configured",
    ...overrides
  };
}

function buildRequestedPlan(overrides: Record<string, unknown> = {}) {
  return {
    requested_agent_count: 7,
    requested_round_count: 8,
    max_token_budget: 12000,
    ...overrides
  };
}

function buildSelectedSeedDocument(overrides: Record<string, unknown> = {}) {
  return {
    file_name: "seed.txt",
    mime_type: "text/plain",
    detected_kind: "txt",
    size_bytes: 36,
    content_fingerprint_sha256: "feedbead1234",
    extracted_text_character_count: 1420,
    english_confidence_percent: 99,
    ...overrides
  };
}

function buildEffectivePlan(overrides: Record<string, unknown> = {}) {
  return {
    effective_agent_count: 5,
    effective_round_count: 3,
    provider: "open_ai",
    model: "gpt-4o-mini",
    max_token_budget: 12000,
    bounded_speaker_model: "moderated_3_to_5_speakers_per_round",
    bounded_concurrency_policy: "single_moderated_speaker_set_per_round",
    ...overrides
  };
}

function buildEstimatedUsage(overrides: Record<string, unknown> = {}) {
  return {
    estimated_total_tokens: 10610,
    estimated_duration_minutes_low: 21,
    estimated_duration_minutes_high: 33,
    ...overrides
  };
}

function buildProviderModelContext(overrides: Record<string, unknown> = {}) {
  return {
    provider: "open_ai",
    model: "gpt-4o-mini",
    local_configuration_supports_planning: true,
    live_reachability_probed: false,
    detail_note:
      "Planning uses the bounded local provider selection, secure-store-backed key presence, and frozen default MVP model mapping only; live provider reachability is not probed at this layer.",
    ...overrides
  };
}

function buildSubmissionState(overrides: Record<string, unknown> = {}) {
  return {
    status: "approved_with_downgrade",
    selected_seed_document: buildSelectedSeedDocument(),
    requested_plan: buildRequestedPlan(),
    valid_document_in_pending_configuration: true,
    effective_plan: buildEffectivePlan(),
    estimated_usage: buildEstimatedUsage(),
    provider_model_context: buildProviderModelContext(),
    requested_plan_budget_feasible: false,
    downgrade_warning:
      "Requested plan was downgraded before run start. Effective agent count is 5; effective round count is 3; reason: the requested agent count exceeded the locked 3-to-5 speaker model; the requested budget could not cover the bounded local estimate.",
    downgrade_acceptance_recorded: false,
    last_rejection_reason: null,
    last_technical_failure_reason: null,
    detail_message:
      "Preflight approved a downgraded effective plan under the bounded local provider/model planning basis",
    ...overrides
  };
}

function buildPreflightReadinessState(overrides: Record<string, unknown> = {}) {
  return {
    stage: "feasibility_and_invalidation_materialized",
    state_surface_materialized: true,
    preflight_contract_defined: true,
    requested_plan_contract_defined: true,
    effective_plan_contract_defined: true,
    invalidation_semantics_defined: true,
    run_start_gate_truth_surface_materialized: true,
    preflight_execution_materialized: true,
    runtime_readiness_evaluated_separately: true,
    activation_readiness_evaluated_separately: true,
    provider_readiness_evaluated_separately: true,
    runtime_ready_now: true,
    activation_ready_now: true,
    provider_configuration_ready_now: true,
    pending_seed_document_selected: true,
    requested_plan_present: true,
    selected_seed_document: buildSelectedSeedDocument(),
    requested_plan: buildRequestedPlan(),
    effective_plan: buildEffectivePlan(),
    estimated_usage: buildEstimatedUsage(),
    provider_model_context: buildProviderModelContext(),
    requested_plan_budget_feasible: false,
    downgrade_warning:
      "Requested plan was downgraded before run start. Effective agent count is 5; effective round count is 3; reason: the requested agent count exceeded the locked 3-to-5 speaker model; the requested budget could not cover the bounded local estimate.",
    document_validation_completed: true,
    document_validation_passed: true,
    current_state: "approved_with_downgrade",
    current_state_name: "PreflightApprovedWithDowngrade",
    last_rejection_reason: null,
    last_technical_failure_reason: null,
    invalidation_reasons: [],
    current_status_note:
      "Preflight approved a downgraded effective plan under the bounded local provider/model planning basis",
    downgrade_acceptance_required: true,
    downgrade_acceptance_recorded: false,
    current_result_stale: false,
    run_start_eligible_now: false,
    run_start_blocked_without_eligible_preflight: true,
    ...overrides
  };
}

function buildRunStartReadinessState(overrides: Record<string, unknown> = {}) {
  return {
    stage: "foundation_materialized",
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
    runtime_ready_now: false,
    activation_ready_now: false,
    provider_configuration_ready_now: false,
    current_eligible_preflight_now: false,
    downgrade_acceptance_required: false,
    downgrade_acceptance_recorded: false,
    start_lineage_ready_now: false,
    current_state: "not_started",
    current_state_name: "RunStartFoundationNotStarted",
    run_start_execution_materialized: true,
    workspace_allocation_materialized: true,
    runtime_persona_foundation_materialized: true,
    bounded_memory_foundation_materialized: true,
    transcript_archive_separation_materialized: true,
    fresh_workspace_required: true,
    hidden_continuation_allowed: false,
    real_run_start_authorized_now: false,
    would_depend_on_current_eligible_preflight_if_execution_existed: false,
    current_status_note:
      "Run-start foundation is materialized, but current eligible Preflight lineage does not exist yet",
    ...overrides
  };
}

function buildRuntimePersonaContractState(overrides: Record<string, unknown> = {}) {
  return {
    stage: "foundation_materialized",
    contract_name: "RuntimePersonaContract",
    engine_owned_persona_generation: true,
    runtime_generated_from_seed_context: true,
    exactly_one_moderator_required: true,
    stable_persona_labels_required: true,
    favorable_perspective_required: true,
    skeptical_perspective_required: true,
    analytical_or_neutral_perspective_required: true,
    static_user_authored_persona_configuration_allowed: false,
    runtime_persona_generation_materialized: true,
    moderator_basis_materialized: true,
    participant_labeling_materialized: true,
    persona_outputs_attributable: true,
    full_moderated_rounds_implied: true,
    ...overrides
  };
}

function buildRunBoundedMemoryState(overrides: Record<string, unknown> = {}) {
  return {
    stage: "foundation_materialized",
    memory_surface_name: "RunBoundedMemory",
    seed_grounding_memory_required: true,
    round_summary_memory_required: true,
    argument_risk_memory_required: true,
    transcript_archive_separate_from_prompt_memory: true,
    naive_full_transcript_replay_allowed: false,
    cross_run_semantic_leakage_allowed: false,
    bounded_memory_foundation_materialized: true,
    transcript_archive_foundation_materialized: true,
    final_report_implied: false,
    ...overrides
  };
}

function buildRunModeratedRoundsState(overrides: Record<string, unknown> = {}) {
  return {
    stage: "execution_materialized",
    surface_name: "RunModeratedRounds",
    engine_owned_moderation: true,
    moderator_controlled_selection_required: true,
    normal_round_min_speakers: 3,
    normal_round_max_speakers: 5,
    all_agent_fanout_allowed: false,
    speaker_attribution_required: true,
    round_summary_continuity_required: true,
    argument_risk_continuity_required: true,
    moderated_round_execution_materialized: true,
    live_monitoring_implied: false,
    final_report_implied: false,
    ...overrides
  };
}

function buildRunTokenDurationTrackingState(overrides: Record<string, unknown> = {}) {
  return {
    stage: "execution_materialized",
    surface_name: "RunTokenDurationTracking",
    engine_owned_tracking: true,
    run_local_tracking_required: true,
    turn_level_tracking_required: true,
    duration_tracking_required: true,
    integrated_into_execution_path: true,
    provider_billing_equivalence_implied: false,
    live_monitoring_implied: false,
    final_report_implied: false,
    token_duration_tracking_materialized: true,
    ...overrides
  };
}

function buildRunSessionState(overrides: Record<string, unknown> = {}) {
  return {
    stage: "start_path_materialized",
    current_state_class: "not_started",
    current_state_name: "not_started",
    run_id: null,
    session_id: null,
    workspace_reference: null,
    workspace_root_path: null,
    execution_seed_directory_path: null,
    transcript_archive_directory_path: null,
    staged_seed_document_path: null,
    lineage_manifest_path: null,
    current_seed_fingerprint_sha256: null,
    runtime_persona_manifest_path: null,
    moderator_basis_path: null,
    bounded_memory_directory_path: null,
    seed_grounding_memory_path: null,
    round_summary_memory_path: null,
    argument_risk_memory_path: null,
    transcript_archive_index_path: null,
    transcript_turns_path: null,
    token_duration_tracking_path: null,
    moderator_persona_label: null,
    participant_persona_labels: [],
    perspective_coverage: [],
    participant_persona_count: null,
    requested_round_count: null,
    completed_round_count: null,
    speaker_counts_per_round: [],
    total_turn_count: null,
    latest_round_summary: null,
    tracked_total_input_token_units: null,
    tracked_total_output_token_units: null,
    tracked_total_token_units: null,
    tracked_total_duration_ms: null,
    average_turn_duration_ms: null,
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
    start_blocking_reasons: [],
    technical_failure_reason: null,
    detail_message:
      "No fresh run/session has been created from current eligible Preflight truth yet",
    ...overrides
  };
}

function buildDowngradeAcceptanceOutcome(overrides: Record<string, unknown> = {}) {
  return {
    acceptance_recorded: true,
    technical_failure_reason: null,
    submission_state: buildSubmissionState({ downgrade_acceptance_recorded: true }),
    detail_message:
      "Downgraded effective plan was explicitly accepted; run start now depends on the result staying current",
    ...overrides
  };
}

function buildProviderConfigurationOutcome(overrides: Record<string, unknown> = {}) {
  return {
    decision_type: "success",
    success: {
      selected_provider: "google",
      secure_store_lookup_reference: "provider-api-key.google",
      support_record_path: "support/provider_configuration_support_state.json",
      configuration_status: "present",
      configuration_present: true
    },
    failure_reason: null,
    configuration_state: buildProviderConfigurationState({
      selected_provider: "google",
      secure_store_lookup_reference: "provider-api-key.google"
    }),
    detail_message: "provider configuration committed locally; Preflight still remains a separate later gate",
    ...overrides
  };
}

function configureInvokeMock(options: {
  providerStates?: Array<Record<string, unknown>>;
  preflightStates: Array<Record<string, unknown>>;
  runStartStates?: Array<Record<string, unknown>>;
  runtimePersonaStates?: Array<Record<string, unknown>>;
  runBoundedMemoryStates?: Array<Record<string, unknown>>;
  runModeratedRoundsStates?: Array<Record<string, unknown>>;
  runTokenDurationTrackingStates?: Array<Record<string, unknown>>;
  runSessionStates?: Array<Record<string, unknown>>;
  acceptDowngradeOutcome?: Record<string, unknown>;
  providerSubmitOutcome?: Record<string, unknown>;
}) {
  let providerStateIndex = 0;
  let preflightStateIndex = 0;
  let runStartStateIndex = 0;
  let runtimePersonaStateIndex = 0;
  let runBoundedMemoryStateIndex = 0;
  let runModeratedRoundsStateIndex = 0;
  let runTokenDurationTrackingStateIndex = 0;
  let runSessionStateIndex = 0;

  invokeMock.mockImplementation(async (command: string) => {
    switch (command) {
      case "provider_configuration_state": {
        const providerStates = options.providerStates ?? [buildProviderConfigurationState()];
        const nextState = providerStates[
          Math.min(providerStateIndex, providerStates.length - 1)
        ];
        providerStateIndex += 1;
        return nextState;
      }
      case "preflight_readiness_state": {
        const nextState = options.preflightStates[
          Math.min(preflightStateIndex, options.preflightStates.length - 1)
        ];
        preflightStateIndex += 1;
        return nextState;
      }
      case "run_start_readiness_state": {
        const runStartStates = options.runStartStates ?? [buildRunStartReadinessState()];
        const nextState = runStartStates[
          Math.min(runStartStateIndex, runStartStates.length - 1)
        ];
        runStartStateIndex += 1;
        return nextState;
      }
      case "runtime_persona_contract_state": {
        const runtimePersonaStates = options.runtimePersonaStates ?? [buildRuntimePersonaContractState()];
        const nextState = runtimePersonaStates[
          Math.min(runtimePersonaStateIndex, runtimePersonaStates.length - 1)
        ];
        runtimePersonaStateIndex += 1;
        return nextState;
      }
      case "run_bounded_memory_state": {
        const runBoundedMemoryStates = options.runBoundedMemoryStates ?? [buildRunBoundedMemoryState()];
        const nextState = runBoundedMemoryStates[
          Math.min(runBoundedMemoryStateIndex, runBoundedMemoryStates.length - 1)
        ];
        runBoundedMemoryStateIndex += 1;
        return nextState;
      }
      case "run_moderated_rounds_state": {
        const runModeratedRoundsStates =
          options.runModeratedRoundsStates ?? [buildRunModeratedRoundsState()];
        const nextState = runModeratedRoundsStates[
          Math.min(runModeratedRoundsStateIndex, runModeratedRoundsStates.length - 1)
        ];
        runModeratedRoundsStateIndex += 1;
        return nextState;
      }
      case "run_token_duration_tracking_state": {
        const runTokenDurationTrackingStates =
          options.runTokenDurationTrackingStates ?? [buildRunTokenDurationTrackingState()];
        const nextState = runTokenDurationTrackingStates[
          Math.min(
            runTokenDurationTrackingStateIndex,
            runTokenDurationTrackingStates.length - 1
          )
        ];
        runTokenDurationTrackingStateIndex += 1;
        return nextState;
      }
      case "run_session_state": {
        const runSessionStates = options.runSessionStates ?? [buildRunSessionState()];
        const nextState = runSessionStates[
          Math.min(runSessionStateIndex, runSessionStates.length - 1)
        ];
        runSessionStateIndex += 1;
        return nextState;
      }
      case "accept_preflight_downgrade":
        return options.acceptDowngradeOutcome ?? buildDowngradeAcceptanceOutcome();
      case "submit_provider_configuration":
        return options.providerSubmitOutcome ?? buildProviderConfigurationOutcome();
      default:
        throw new Error(`Unexpected command: ${command}`);
    }
  });
}

function findInvokeCall(commandName: string) {
  return invokeMock.mock.calls.find(([command]) => command === commandName);
}

async function renderAppAndWaitForPreflightState(stateName: string) {
  render(<App />);
  await screen.findByText(`Canonical state name: ${stateName}`);
}

describe("App Wave 4 Preflight UI", () => {
  beforeEach(() => {
    invokeMock.mockReset();
  });

  it("renders the runtime persona and bounded-memory foundation surfaces", async () => {
    configureInvokeMock({
      preflightStates: [buildPreflightReadinessState()]
    });

    await renderAppAndWaitForPreflightState("PreflightApprovedWithDowngrade");

    await screen.findByText("Runtime persona and bounded memory foundation");
    await screen.findByText("Runtime persona contract");
    await screen.findByText("Exactly one Moderator required: yes");
    await screen.findByText("Run bounded memory");
    await screen.findByText("Transcript archive separate from prompt memory: yes");
  });

  it("renders downgrade disclosure and records explicit acceptance before run-start eligibility turns true", async () => {
    const initialState = buildPreflightReadinessState();
    const acceptedState = buildPreflightReadinessState({
      downgrade_acceptance_recorded: true,
      current_status_note:
        "Downgraded effective plan was explicitly accepted; run start now depends on the result staying current",
      run_start_eligible_now: true,
      run_start_blocked_without_eligible_preflight: false
    });

    configureInvokeMock({
      preflightStates: [initialState, acceptedState],
      acceptDowngradeOutcome: buildDowngradeAcceptanceOutcome()
    });

    await renderAppAndWaitForPreflightState("PreflightApprovedWithDowngrade");

    await screen.findByText("Downgrade disclosure");
    await screen.findByText(/Requested plan was downgraded before run start/);
    fireEvent.click(screen.getByRole("button", { name: "Accept downgraded effective plan" }));

    await screen.findByText(/Downgrade acceptance detail:/);
    await screen.findByText("Downgrade acceptance recorded: yes");
    await screen.findByText("Run-start eligible now: yes");

    await waitFor(() => {
      expect(findInvokeCall("accept_preflight_downgrade")).toBeDefined();
    });
  });

  it("marks an existing Preflight result stale after a material provider change", async () => {
    const initialState = buildPreflightReadinessState({
      current_state: "approved",
      current_state_name: "PreflightApproved",
      downgrade_acceptance_required: false,
      requested_plan_budget_feasible: true,
      downgrade_warning: null,
      run_start_eligible_now: true,
      run_start_blocked_without_eligible_preflight: false,
      requested_plan: buildRequestedPlan({ requested_agent_count: 5, requested_round_count: 4 }),
      effective_plan: buildEffectivePlan({ effective_round_count: 4 }),
      current_status_note:
        "Preflight approved the requested plan under the bounded local provider/model planning basis"
    });
    const invalidatedState = buildPreflightReadinessState({
      current_state: "invalidated",
      current_state_name: "PreflightInvalidated",
      current_result_stale: true,
      invalidation_reasons: ["provider_changed"],
      current_status_note:
        "Current Preflight result is stale because material inputs changed after evaluation: provider_changed.",
      downgrade_acceptance_required: false,
      run_start_eligible_now: false,
      run_start_blocked_without_eligible_preflight: true
    });

    configureInvokeMock({
      providerStates: [buildProviderConfigurationState()],
      preflightStates: [initialState, invalidatedState],
      providerSubmitOutcome: buildProviderConfigurationOutcome()
    });

    await renderAppAndWaitForPreflightState("PreflightApproved");

    fireEvent.change(screen.getByLabelText("Provider"), {
      target: { value: "google" }
    });
    fireEvent.change(screen.getByLabelText("Provider API key"), {
      target: { value: "google-secret-001" }
    });
    fireEvent.click(screen.getByRole("button", { name: "Commit provider configuration" }));

    await screen.findByText("Provider commit result");
    await screen.findByText("Canonical state name: PreflightInvalidated");
    await screen.findByText("Current result stale: yes");
    await screen.findByText("Invalidation reasons: Provider Changed");

    await waitFor(() => {
      expect(findInvokeCall("submit_provider_configuration")).toBeDefined();
    });
  });
});