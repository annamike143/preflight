import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

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

function buildRunFinalizationContractState() {
  return {
    stage: "foundation_materialized",
    contract_name: "RunFinalizationContract",
    shell_command_name: "RunFinalizationContractState",
    supported_markers: [
      "finalization_entered",
      "finalization_succeeded",
      "report_generation_in_progress",
      "report_generation_succeeded",
      "report_generation_failed",
      "report_available",
      "report_unavailable"
    ],
    shell_authority_required: true,
    finalization_entered_distinct: true,
    finalization_succeeded_distinct: true,
    report_generation_in_progress_distinct: true,
    report_generation_succeeded_distinct: true,
    report_generation_failed_distinct: true,
    report_available_distinct: true,
    report_unavailable_distinct: true,
    completion_classification_implies_report_generation_success: true,
    completion_classification_implies_report_available: false,
    cleanup_outcome_implies_report_availability: false,
    real_report_generation_materialized: true,
    real_pdf_generation_materialized: false
  };
}

function buildReportOutputContractState() {
  return {
    stage: "foundation_materialized",
    contract_name: "ReportOutputContract",
    shell_command_name: "ReportOutputContractState",
    required_sections: [
      "executive_summary",
      "market_viability_score",
      "ranked_risks",
      "strongest_pro_arguments",
      "strongest_anti_arguments",
      "cost_and_token_summary",
      "transcript_appendix"
    ],
    output_classes: [
      "report_metadata",
      "report_section_content",
      "transcript_appendix",
      "generated_pdf_artifact",
      "export_access",
      "report_availability"
    ],
    metadata_contract: {
      report_identifier_required_when_generated: true,
      run_identifier_required: true,
      session_identifier_required: true,
      completion_classification_required: true,
      output_class_required: true,
      local_artifact_reference_optional_until_generated: true,
      generation_timestamp_optional_until_generated: true,
      file_size_bytes_optional_until_generated: true,
      secret_material_allowed: false
    },
    section_representation_contract: {
      executive_summary_defined: true,
      market_viability_score_defined: true,
      ranked_risks_defined: true,
      strongest_pro_arguments_defined: true,
      strongest_anti_arguments_defined: true,
      cost_and_token_summary_defined: true,
      transcript_appendix_defined: true,
      scoring_philosophy_rewritten: false,
      invented_section_variants_allowed: false
    },
    pdf_contract: {
      file_format_name: "pdf",
      input_structure_defined: true,
      output_artifact_class_defined: true,
      successful_generation_implied: false,
      alternate_formats_allowed: false
    },
    stable_output_structure_defined: true,
    real_section_synthesis_materialized: true,
    real_viability_score_materialized: true,
    real_cost_token_summary_materialized: true,
    real_transcript_appendix_materialized: true,
  };
}

function buildReportStorageBoundaryState() {
  return {
    stage: "foundation_materialized",
    boundary_name: "ReportStorageBoundary",
    transient_workspace_domain_name: "TransientReportWorkspaceDomain",
    durable_artifact_domain_name: "DurableReportArtifactDomain",
    metadata_index_domain_name: "ReportMetadataIndexDomain",
    transient_generation_intermediates_classified: true,
    durable_export_artifacts_classified: true,
    metadata_index_references_classified: true,
    transient_generation_intermediates_outside_cleanup_scope: false,
    durable_export_artifacts_outside_cleanup_scope: true,
    metadata_index_references_outside_cleanup_scope: true,
    report_generation_intermediates_persistence_materialized: true,
    durable_export_write_materialized: false,
    metadata_index_persistence_materialized: false,
    ordinary_workspace_garbage_classification_allowed_for_durable_artifacts: false,
    metadata_reference_contract: {
      report_identifier_required_when_generated: true,
      artifact_reference_optional_until_generated: true,
      export_timestamp_optional_until_generated: true,
      raw_provider_secret_allowed: false,
      activation_token_material_allowed: false,
      raw_seed_bytes_allowed: false,
      ordinary_log_embedding_allowed: false
    }
  };
}

function buildReportAccessContractState() {
  return {
    stage: "foundation_materialized",
    contract_name: "ReportAccessContract",
    shell_command_name: "ReportAccessContractState",
    supported_request_kinds: ["retrieve_metadata", "open_local_pdf", "export_local_pdf"],
    supported_failure_classifications: [
      "unavailable",
      "missing_artifact",
      "inaccessible_artifact",
      "malformed_request"
    ],
    current_availability_status: "unavailable",
    current_availability_status_name: "ReportUnavailable",
    shell_authority_required: true,
    metadata_retrieval_contract: {
      report_identifier_required_when_generated: true,
      run_identifier_required: true,
      session_identifier_required: true,
      completion_classification_required: true,
      availability_status_required: true,
      local_artifact_path_optional_until_accessible: true,
      file_size_bytes_optional_until_accessible: true,
      secret_material_allowed: false
    },
    access_request_contract: {
      shell_command_boundary_required: true,
      frontend_direct_filesystem_access_allowed: false,
      local_artifact_reference_required_for_open_or_export: true,
      offline_access_possible_when_file_exists: true,
      open_request_shape_defined: true,
      export_request_shape_defined: true
    },
    response_contract: {
      metadata_retrieval_supported: true,
      availability_representation_supported: true,
      open_request_supported: true,
      export_request_supported: true,
      unavailable_response_defined: true,
      missing_artifact_response_defined: true,
      inaccessible_artifact_response_defined: true,
      local_open_execution_materialized: false,
      local_export_execution_materialized: false
    },
    report_available_now: false,
    local_artifact_accessible_now: false,
    phantom_report_state_allowed: false,
    completion_classification_implies_report_available: false,
    report_generation_success_implies_local_accessibility: false,
    cleanup_outcome_implies_report_deletion: false
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
    current_state: "running_round",
    current_state_name: "RunningRound",
    projected_from_state_class: "running",
    projected_from_state_name: "running",
    shell_authoritative_projection: true,
    canonical_state_family_frozen: true,
    mapping_basis_explicit: true,
    degraded_states_distinct_in_vocabulary: true,
    cancellation_state_distinct_in_vocabulary: true,
    constrained_completion_state_distinct_in_vocabulary: true,
    retryable_failure_state_distinct_in_vocabulary: true,
    terminal_failure_state_distinct_in_vocabulary: true,
    cleanup_states_distinct_in_vocabulary: true,
    current_state_terminal: false,
    current_state_cancelable: true,
    current_state_retryable: false,
    live_monitoring_implied: false,
    ordered_event_history_implied: false,
    current_projection_detail:
      "The coarse running class projects to RunningRound until finer-grained ingest, persona-generation, and finalization live states are materialized."
  };
}

function buildRunStatusState() {
  return {
    stage: "foundation_materialized",
    contract_name: "RunStatusContract",
    shell_command_name: "RunStatusState",
    lifecycle_surface_name: "RunLifecycleState",
    current_lifecycle_state: "running_round",
    current_lifecycle_state_name: "RunningRound",
    current_run_id: "run-1",
    current_session_id: "session-1",
    current_workspace_reference: "workspace::run-1",
    requested_round_count: 4,
    completed_round_count: 2,
    tracked_total_token_units: 320,
    tracked_total_duration_ms: 25,
    latest_status_note:
      "Shell status projection confirms that interruption control, ordered history visibility, and cancelability truth remain shell-owned during active execution.",
    latest_visible_sequence_number: 5,
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
    current_state_cancelable: true,
    current_state_terminal: false,
    current_state_retryable: false,
    current_state_degraded: false,
    ordered_event_history_materialized: true,
    live_status_push_materialized: false,
    cancellation_request_contract: {
      shell_command_boundary_required: true,
      shell_owned_cancelability_authority_required: true,
      active_run_identifier_required_when_cancelable: true,
      frontend_direct_engine_signal_allowed: false,
      cancelability_derived_from_canonical_state: true,
      cancellation_request_shape_materialized: true,
      cancellation_command_explicit: true,
      cancellation_command_implemented: true
    },
    cancellation_response_contract: {
      accepted_response_defined: true,
      rejected_response_defined: true,
      already_terminal_response_defined: true,
      no_active_run_response_defined: true,
      hidden_post_cancel_progress_allowed: false,
      forceful_termination_implied_by_acceptance: false,
      controlled_cancellation_behavior_materialized: true
    }
  };
}

function buildRunEventContractState() {
  return {
    stage: "foundation_materialized",
    contract_name: "RunEventContract",
    shell_command_name: "RunEventContractState",
    event_history_command_name: "RunEventHistory",
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
    event_history_retrieval_implemented: true,
    gap_contract: {
      cursor_boundary_required: true,
      after_sequence_number_supported: true,
      inclusive_replay_supported: false,
      gap_marker_supported: true,
      missing_history_inference_allowed: false,
      bounded_page_retrieval_required: true,
      latest_sequence_boundary_supported: true
    },
    event_history_materialized: true,
    live_event_capture_materialized: true,
    transcript_event_chronology_materialized: true,
    payload_secret_material_allowed: false,
    frontend_ordering_authority_allowed: false,
    latest_available_sequence_number: 5
  };
}

function buildRunEventHistoryState() {
  return {
    command_name: "RunEventHistory",
    current_run_id: "run-1",
    current_session_id: "session-1",
    requested_after_sequence_number: null,
    requested_max_items: 256,
    latest_available_sequence_number: 5,
    returned_item_count: 5,
    returned_event_count: 3,
    returned_chronology_entry_count: 2,
    gap_detected: false,
    gap_reason: null,
    stored_history_complete: true,
    event_history_materialized: true,
    transcript_event_chronology_materialized: true,
    live_usage_visibility_materialized: true,
    events: [
      {
        event_id: "event-1",
        sequence_number: 1,
        occurred_at: 1713200000000,
        session_id: "session-1",
        event_type: "run_queued",
        runtime_domain: "shell",
        payload: {
          workspace_reference: "workspace::run-1"
        }
      },
      {
        event_id: "event-2",
        sequence_number: 2,
        occurred_at: 1713200001000,
        session_id: "session-1",
        event_type: "round_started",
        runtime_domain: "debate",
        payload: {
          round_number: 1,
          selected_speaker_labels: ["Advocate", "Skeptic", "Analyst"]
        }
      },
      {
        event_id: "event-5",
        sequence_number: 5,
        occurred_at: 1713200003000,
        session_id: "session-1",
        event_type: "budget_usage_updated",
        runtime_domain: "monitoring",
        payload: {
          cumulative_total_token_units: 320,
          cumulative_duration_ms: 25
        }
      }
    ],
    chronology_entries: [
      {
        sequence_number: 3,
        occurred_at: 1713200001500,
        entry_kind: "transcript_turn",
        round_number: 1,
        turn_index: 1,
        speaker_label: "Advocate",
        speaker_perspective: "favorable",
        turn_text: "The favorable case keeps the plan grounded in the seed evidence.",
        total_token_units: 120,
        duration_ms: 9
      },
      {
        sequence_number: 4,
        occurred_at: 1713200002200,
        entry_kind: "transcript_turn",
        round_number: 1,
        turn_index: 2,
        speaker_label: "Skeptic",
        speaker_perspective: "skeptical",
        turn_text: "The skeptical case highlights where the seed leaves uncertainty unresolved.",
        total_token_units: 200,
        duration_ms: 16
      }
    ]
  };
}

function buildRunSessionState() {
  return {
    stage: "start_path_materialized",
    current_state_class: "running",
    current_state_name: "running",
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
    completed_round_count: 2,
    speaker_counts_per_round: [3, 4],
    total_turn_count: 7,
    latest_round_summary: "Round 2 preserved bounded continuity.",
    tracked_total_input_token_units: 210,
    tracked_total_output_token_units: 110,
    tracked_total_token_units: 320,
    tracked_total_duration_ms: 25,
    average_turn_duration_ms: 4,
    runtime_cap_reason: null,
    controlled_finalization_performed: false,
    finalization_started_at: null,
    finalization_completed_at: null,
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
      "Moderator-controlled execution is currently projected through the Phase 7 running-round contract basis."
  };
}

function configureInvokeMock(overrides?: {
  runLifecycleState?: Record<string, unknown>;
  runStatusState?: Record<string, unknown>;
  runEventHistoryState?: Record<string, unknown>;
  runSessionState?: Record<string, unknown>;
}) {
  const runLifecycleState = { ...buildRunLifecycleState(), ...(overrides?.runLifecycleState ?? {}) };
  const runStatusState = { ...buildRunStatusState(), ...(overrides?.runStatusState ?? {}) };
  const runEventHistoryState = {
    ...buildRunEventHistoryState(),
    ...(overrides?.runEventHistoryState ?? {})
  };
  const runSessionState = { ...buildRunSessionState(), ...(overrides?.runSessionState ?? {}) };

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
        return runLifecycleState;
      case "run_status_state":
        return runStatusState;
      case "run_event_contract_state":
        return buildRunEventContractState();
      case "run_finalization_contract_state":
        return buildRunFinalizationContractState();
      case "report_output_contract_state":
        return buildReportOutputContractState();
      case "report_storage_boundary_state":
        return buildReportStorageBoundaryState();
      case "report_access_contract_state":
        return buildReportAccessContractState();
      case "run_event_history":
        return runEventHistoryState;
      case "run_session_state":
        return runSessionState;
      default:
        throw new Error(`Unexpected command: ${command}`);
    }
  });
}

describe("App Wave 7 monitoring-contract UI", () => {
  beforeEach(() => {
    invokeMock.mockReset();
  });

  it("renders shell-owned monitoring, chronology, and event contract truth", async () => {
    configureInvokeMock();

    render(<App />);
      fireEvent.click(screen.getByRole("button", { name: "Run" }));

    await screen.findByText("Live run monitoring");
    await screen.findAllByText("Current run id: run-1");
    await screen.findByText("Sequence-aligned chronology items: 5");
    await screen.findByText("320 token units of 12000");
    await screen.findByText("Stored history complete: yes");
    await screen.findByText("Fresh run workspace is allocated at workspace::run-1.");
    await screen.findByText("Round 1 started. Speakers: Advocate, Skeptic, Analyst.");
    await screen.findByText("Round 1 • Turn 1");
    await screen.findByText("The favorable case keeps the plan grounded in the seed evidence.");
    await screen.findByText("Shell-owned canonical lifecycle projection");
    await screen.findByText("Canonical state name: RunningRound");
    await screen.findByText("RunStatusContract");
    await screen.findByText("Shell command name: RunStatusState");
    await screen.findByText("Cancellation command implemented: yes");
    await screen.findByText("Cancel active run");
    await screen.findByText("Request controlled shutdown");
    await screen.findByText("RunEventContract");
    await screen.findByText("Event history command name: RunEventHistory");
    await screen.findByText("Event-history retrieval implemented: yes");
    await screen.findByText("Gap marker supported: yes");
    await screen.findAllByText("Ordered event history materialized: yes");
    await screen.findByText(
      "Supported runtime domains: Shell, Engine, Debate, Monitoring, Control, Cleanup"
    );
  });

  it("renders capped finalization truth from shell-owned monitoring surfaces", async () => {
    configureInvokeMock({
      runLifecycleState: {
        current_state: "completed_budget_capped",
        current_state_name: "CompletedBudgetCapped",
        current_state_terminal: true,
        current_state_cancelable: false,
        current_projection_detail:
          "The shell resolved the run as CompletedBudgetCapped after a governed runtime cap stopped further normal rounds."
      },
      runStatusState: {
        current_lifecycle_state: "completed_budget_capped",
        current_lifecycle_state_name: "CompletedBudgetCapped",
        current_state_terminal: true,
        current_state_cancelable: false,
        latest_status_note:
          "Shell status projection confirms that the run resolved as CompletedBudgetCapped after the runtime token budget stopped further normal rounds.",
        completed_round_count: 2,
        tracked_total_token_units: 12000
      },
      runEventHistoryState: {
        latest_available_sequence_number: 7,
        returned_item_count: 7,
        returned_event_count: 5,
        events: [
          {
            event_id: "event-1",
            sequence_number: 1,
            occurred_at: 1713200000000,
            session_id: "session-1",
            event_type: "run_queued",
            runtime_domain: "shell",
            payload: {
              workspace_reference: "workspace::run-1"
            }
          },
          {
            event_id: "event-2",
            sequence_number: 2,
            occurred_at: 1713200001000,
            session_id: "session-1",
            event_type: "budget_usage_updated",
            runtime_domain: "monitoring",
            payload: {
              cumulative_total_token_units: 12000,
              cumulative_duration_ms: 25
            }
          },
          {
            event_id: "event-3",
            sequence_number: 3,
            occurred_at: 1713200001500,
            session_id: "session-1",
            event_type: "finalization_started",
            runtime_domain: "engine",
            payload: {
              cap_reason: "token_budget"
            }
          },
          {
            event_id: "event-4",
            sequence_number: 4,
            occurred_at: 1713200001700,
            session_id: "session-1",
            event_type: "finalization_completed",
            runtime_domain: "engine",
            payload: {
              cap_reason: "token_budget"
            }
          },
          {
            event_id: "event-7",
            sequence_number: 7,
            occurred_at: 1713200002000,
            session_id: "session-1",
            event_type: "run_state_changed",
            runtime_domain: "shell",
            payload: {
              current_state: "CompletedBudgetCapped"
            }
          }
        ]
      },
      runSessionState: {
        completed_round_count: 2,
        tracked_total_token_units: 12000,
        runtime_cap_reason: "token_budget",
        controlled_finalization_performed: true,
        finalization_started_at: 1713200001500,
        finalization_completed_at: 1713200001700,
        latest_round_summary:
          "Controlled finalization preserved bounded continuity after the runtime token budget halted further normal rounds.",
        detail_message:
          "Moderator-controlled execution hit a governed runtime cap, stopped scheduling further normal rounds, and completed bounded constrained finalization from available material"
      }
    });

    render(<App />);
  fireEvent.click(screen.getByRole("button", { name: "Run" }));

    expect(await screen.findAllByText("Completed Budget Capped")).toHaveLength(3);
    await screen.findByText(
      "This run resolved as CompletedBudgetCapped after the Token Budget truncated the planned round schedule."
    );
    await screen.findByText("Runtime cap reason: Token Budget");
    await screen.findByText("Controlled finalization performed: yes");
    await screen.findByText(
      "Constrained finalization completed after the Token Budget truncated the planned run."
    );
  });
});
