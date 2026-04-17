import { render, screen } from "@testing-library/react";
import { beforeEach, describe, it, vi } from "vitest";

import App from "./App";

const { invokeMock } = vi.hoisted(() => ({
  invokeMock: vi.fn()
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: invokeMock
}));

function buildProviderConfigurationState() {
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
    current_detail: "configured"
  };
}

function buildPreflightReadinessState() {
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
    selected_seed_document: {
      file_name: "seed.txt",
      mime_type: "text/plain",
      detected_kind: "txt",
      size_bytes: 36,
      content_fingerprint_sha256: "feedbead1234",
      extracted_text_character_count: 1420,
      english_confidence_percent: 99
    },
    requested_plan: {
      requested_agent_count: 5,
      requested_round_count: 4,
      max_token_budget: 12000
    },
    effective_plan: {
      effective_agent_count: 5,
      effective_round_count: 4,
      provider: "open_ai",
      model: "gpt-4o-mini",
      max_token_budget: 12000,
      bounded_speaker_model: "moderated_3_to_5_speakers_per_round",
      bounded_concurrency_policy: "single_moderated_speaker_set_per_round"
    },
    estimated_usage: {
      estimated_total_tokens: 10980,
      estimated_duration_minutes_low: 28,
      estimated_duration_minutes_high: 44
    },
    provider_model_context: {
      provider: "open_ai",
      model: "gpt-4o-mini",
      local_configuration_supports_planning: true,
      live_reachability_probed: false,
      detail_note: "planning context"
    },
    requested_plan_budget_feasible: true,
    downgrade_warning: null,
    document_validation_completed: true,
    document_validation_passed: true,
    current_state: "approved",
    current_state_name: "PreflightApproved",
    last_rejection_reason: null,
    last_technical_failure_reason: null,
    invalidation_reasons: [],
    current_status_note:
      "Preflight approved the requested plan under the bounded local provider/model planning basis",
    downgrade_acceptance_required: false,
    downgrade_acceptance_recorded: false,
    current_result_stale: false,
    run_start_eligible_now: true,
    run_start_blocked_without_eligible_preflight: false
  };
}

function buildRunStartReadinessState() {
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
    runtime_ready_now: true,
    activation_ready_now: true,
    provider_configuration_ready_now: true,
    current_eligible_preflight_now: true,
    downgrade_acceptance_required: false,
    downgrade_acceptance_recorded: false,
    start_lineage_ready_now: true,
    current_state: "blocked",
    current_state_name: "RunStartFoundationBlocked",
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
      "All run-start prerequisites are satisfied for a fresh run/session start"
  };
}

function buildRuntimePersonaContractState() {
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
    full_moderated_rounds_implied: true
  };
}

function buildRunBoundedMemoryState() {
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
    final_report_implied: false
  };
}

function buildRunModeratedRoundsState() {
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
    final_report_implied: false
  };
}

function buildRunTokenDurationTrackingState() {
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
    token_duration_tracking_materialized: true
  };
}

function buildRunLifecycleState() {
  return {
    stage: "foundation_materialized",
    surface_name: "RunLifecycleState",
    supported_states: [
      "run_draft",
      "run_queued",
      "ingesting",
      "generating_personas",
      "running_round",
      "paused_rate_limit",
      "paused_network",
      "finalizing",
      "completed",
      "completed_budget_capped",
      "canceled",
      "failed_retryable",
      "failed_terminal",
      "cleaning",
      "cleaned",
      "cleanup_failed"
    ],
    current_state: "completed",
    current_state_name: "Completed",
    projected_from_state_class: "finished",
    projected_from_state_name: "finished",
    shell_authoritative_projection: true,
    canonical_state_family_frozen: true,
    mapping_basis_explicit: true,
    degraded_states_distinct_in_vocabulary: true,
    cancellation_state_distinct_in_vocabulary: true,
    constrained_completion_state_distinct_in_vocabulary: true,
    retryable_failure_state_distinct_in_vocabulary: true,
    terminal_failure_state_distinct_in_vocabulary: true,
    cleanup_states_distinct_in_vocabulary: true,
    current_state_terminal: true,
    current_state_cancelable: false,
    current_state_retryable: false,
    live_monitoring_implied: false,
    ordered_event_history_implied: false,
    current_projection_detail:
      "The current bounded execution path only proves successful completion, so the canonical lifecycle projects to Completed rather than constrained-completion, cancellation, or cleanup variants."
  };
}

function buildRunStatusState() {
  return {
    stage: "foundation_materialized",
    contract_name: "RunStatusContract",
    shell_command_name: "RunStatusState",
    lifecycle_surface_name: "RunLifecycleState",
    current_lifecycle_state: "completed",
    current_lifecycle_state_name: "Completed",
    current_run_id: "run-1",
    current_session_id: "session-1",
    current_workspace_reference: "workspace::run-1",
    requested_round_count: 4,
    completed_round_count: 4,
    tracked_total_token_units: 610,
    tracked_total_duration_ms: 48,
    latest_status_note:
      "Moderator-controlled execution completed bounded rounds with speaker attribution, continuity mutation, and run-local token/duration tracking",
    latest_visible_sequence_number: null,
    shell_authority_required: true,
    current_status_retrieval_explicit: true,
    current_status_retrieval_implemented: true,
    canonical_state_included: true,
    run_session_identity_included: true,
    workspace_reference_included: true,
    round_count_visibility_included: true,
    token_duration_visibility_included: true,
    degradation_flags_included: true,
    cancelability_flags_included: true,
    latest_status_note_included: true,
    latest_visible_sequence_marker_included: true,
    event_cursor_alignment_defined: true,
    frontend_state_inference_allowed: false,
    current_state_cancelable: false,
    current_state_terminal: true,
    current_state_retryable: false,
    current_state_degraded: false,
    ordered_event_history_materialized: false,
    live_status_push_materialized: false,
    cancellation_request_contract: {
      shell_command_boundary_required: true,
      shell_owned_cancelability_authority_required: true,
      active_run_identifier_required_when_cancelable: true,
      frontend_direct_engine_signal_allowed: false,
      cancelability_derived_from_canonical_state: true,
      cancellation_request_shape_materialized: true,
      cancellation_command_explicit: true,
      cancellation_command_implemented: false
    },
    cancellation_response_contract: {
      accepted_response_defined: true,
      rejected_response_defined: true,
      already_terminal_response_defined: true,
      no_active_run_response_defined: true,
      hidden_post_cancel_progress_allowed: false,
      forceful_termination_implied_by_acceptance: false,
      controlled_cancellation_behavior_materialized: false
    }
  };
}

function buildRunEventContractState() {
  return {
    stage: "foundation_materialized",
    contract_name: "RunEventContract",
    shell_command_name: "RunEventContractState",
    supported_event_types: [
      "run_state_changed",
      "status_note_published",
      "run_queued",
      "ingestion_started",
      "ingestion_completed",
      "persona_generation_started",
      "persona_generation_completed",
      "round_started",
      "round_completed",
      "budget_usage_updated",
      "token_duration_tracked",
      "paused_rate_limit",
      "paused_network",
      "cancellation_requested",
      "cancellation_accepted",
      "cancellation_rejected",
      "finalization_started",
      "finalization_completed",
      "failure_recorded",
      "cleanup_started",
      "cleanup_completed"
    ],
    supported_runtime_domains: ["shell", "engine", "debate", "monitoring", "control", "cleanup"],
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
    gap_contract: {
      cursor_boundary_required: true,
      after_sequence_number_supported: true,
      inclusive_replay_supported: false,
      gap_marker_supported: true,
      missing_history_inference_allowed: false,
      bounded_page_retrieval_required: true,
      latest_sequence_boundary_supported: true
    },
    event_history_materialized: false,
    live_event_capture_materialized: false,
    transcript_event_chronology_materialized: false,
    payload_secret_material_allowed: false,
    frontend_ordering_authority_allowed: false,
    latest_available_sequence_number: null
  };
}

function buildRunSessionState() {
  return {
    stage: "start_path_materialized",
    current_state_class: "finished",
    current_state_name: "finished",
    run_id: "run-1",
    session_id: "session-1",
    workspace_reference: "workspace::run-1",
    workspace_root_path: "C:/temp/run-1",
    execution_seed_directory_path: "C:/temp/run-1/execution-seed",
    transcript_archive_directory_path: "C:/temp/run-1/transcript-archive",
    staged_seed_document_path: "C:/temp/run-1/execution-seed/seed.txt",
    lineage_manifest_path: "C:/temp/run-1/execution-seed/run_start_lineage.json",
    current_seed_fingerprint_sha256: "feedbead1234",
    runtime_persona_manifest_path: "C:/temp/run-1/runtime-personas/persona_set.json",
    moderator_basis_path: "C:/temp/run-1/runtime-personas/moderator_basis.json",
    bounded_memory_directory_path: "C:/temp/run-1/bounded-memory",
    seed_grounding_memory_path: "C:/temp/run-1/bounded-memory/seed_grounding.json",
    round_summary_memory_path: "C:/temp/run-1/bounded-memory/round_summary.json",
    argument_risk_memory_path: "C:/temp/run-1/bounded-memory/argument_risk_memory.json",
    transcript_archive_index_path: "C:/temp/run-1/transcript-archive/archive_index.json",
    transcript_turns_path: "C:/temp/run-1/transcript-archive/speaker_turns.jsonl",
    token_duration_tracking_path:
      "C:/temp/run-1/run-local-tracking/token_duration_tracking.json",
    moderator_persona_label: "Moderator",
    participant_persona_labels: ["Advocate", "Skeptic", "Analyst", "Builder", "Auditor"],
    perspective_coverage: ["favorable", "skeptical", "analytical", "neutral"],
    participant_persona_count: 5,
    requested_round_count: 4,
    completed_round_count: 4,
    speaker_counts_per_round: [3, 4, 5, 3],
    total_turn_count: 15,
    latest_round_summary:
      "Round 4 kept bounded continuity grounded in the approved seed context.",
    tracked_total_input_token_units: 420,
    tracked_total_output_token_units: 190,
    tracked_total_token_units: 610,
    tracked_total_duration_ms: 48,
    average_turn_duration_ms: 3,
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
    normal_round_speaker_selection_enforced: true,
    speaker_attribution_materialized: true,
    bounded_memory_continuity_mutated: true,
    token_duration_tracking_materialized: true,
    start_blocking_reasons: [],
    technical_failure_reason: null,
    detail_message:
      "Moderator-controlled execution completed bounded rounds with speaker attribution, continuity mutation, and run-local token/duration tracking"
  };
}

function configureInvokeMock() {
  invokeMock.mockImplementation(async (command: string) => {
    switch (command) {
      case "provider_configuration_state":
        return buildProviderConfigurationState();
      case "preflight_readiness_state":
        return buildPreflightReadinessState();
      case "run_start_readiness_state":
        return buildRunStartReadinessState();
      case "runtime_persona_contract_state":
        return buildRuntimePersonaContractState();
      case "run_bounded_memory_state":
        return buildRunBoundedMemoryState();
      case "run_moderated_rounds_state":
        return buildRunModeratedRoundsState();
      case "run_token_duration_tracking_state":
        return buildRunTokenDurationTrackingState();
      case "run_lifecycle_state":
        return buildRunLifecycleState();
      case "run_status_state":
        return buildRunStatusState();
      case "run_event_contract_state":
        return buildRunEventContractState();
      case "run_session_state":
        return buildRunSessionState();
      default:
        throw new Error(`Unexpected command: ${command}`);
    }
  });
}

describe("App Wave 5 execution UI", () => {
  beforeEach(() => {
    invokeMock.mockReset();
  });

  it("renders moderated-round and run-local tracking truth alongside finished run state", async () => {
    configureInvokeMock();

    render(<App />);

    await screen.findByText("Canonical state name: PreflightApproved");
    await screen.findByText("Moderated rounds and run-local tracking");
    await screen.findByText("RunModeratedRounds");
    await screen.findByText("Normal round minimum speakers: 3");
    await screen.findByText("RunTokenDurationTracking");
    await screen.findByText("Integrated into execution path: yes");
    await screen.findAllByText("Completed round count: 4");
    await screen.findByText("Speaker counts per round: 3, 4, 5, 3");
    await screen.findAllByText("Tracked total token units: 610");
    await screen.findByText(
      "Token/duration tracking path: C:/temp/run-1/run-local-tracking/token_duration_tracking.json"
    );
  });
});