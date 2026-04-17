import { fireEvent, render, screen, waitFor } from "@testing-library/react";
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
    support_record_present: false,
    selected_provider: null,
    secure_store_lookup_reference: null,
    last_configuration_outcome: null,
    provider_secret_reference_present: false,
    provider_secret_fingerprint_verified: false,
    configuration_present: false,
    current_status: "required",
    current_detail: "selection_missing"
  };
}

function buildPreflightReadinessState(
  overrides: Record<string, unknown> = {}
) {
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
    runtime_ready_now: false,
    activation_ready_now: false,
    provider_configuration_ready_now: false,
    pending_seed_document_selected: false,
    requested_plan_present: false,
    selected_seed_document: null,
    requested_plan: null,
    effective_plan: null,
    estimated_usage: null,
    provider_model_context: null,
    requested_plan_budget_feasible: null,
    downgrade_warning: null,
    document_validation_completed: false,
    document_validation_passed: false,
    current_state: "not_started",
    current_state_name: "PreflightNotStarted",
    last_rejection_reason: null,
    last_technical_failure_reason: null,
    invalidation_reasons: [],
    current_status_note: "Preflight has not received a seed document submission yet",
    downgrade_acceptance_required: false,
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

function buildRunFinalizationContractState(overrides: Record<string, unknown> = {}) {
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
    real_pdf_generation_materialized: true,
    ...overrides
  };
}

function buildReportOutputContractState(overrides: Record<string, unknown> = {}) {
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
      successful_generation_implied: true,
      alternate_formats_allowed: false
    },
    stable_output_structure_defined: true,
    real_section_synthesis_materialized: true,
    real_viability_score_materialized: true,
    real_cost_token_summary_materialized: true,
    real_transcript_appendix_materialized: true,
    ...overrides
  };
}

function buildReportStorageBoundaryState(overrides: Record<string, unknown> = {}) {
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
    durable_export_write_materialized: true,
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
    },
    ...overrides
  };
}

function buildReportAccessContractState(overrides: Record<string, unknown> = {}) {
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
      local_open_execution_materialized: true,
      local_export_execution_materialized: true
    },
    report_available_now: false,
    local_artifact_accessible_now: false,
    phantom_report_state_allowed: false,
    completion_classification_implies_report_available: false,
    report_generation_success_implies_local_accessibility: false,
    cleanup_outcome_implies_report_deletion: false,
    ...overrides
  };
}

function buildRunLifecycleState(overrides: Record<string, unknown> = {}) {
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
    current_state: "run_draft",
    current_state_name: "RunDraft",
    projected_from_state_class: "not_started",
    projected_from_state_name: "not_started",
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
    current_state_cancelable: false,
    current_state_retryable: false,
    live_monitoring_implied: false,
    ordered_event_history_implied: false,
    current_projection_detail:
      "No shell-owned run/session has been created yet, so the canonical lifecycle remains at RunDraft.",
    ...overrides
  };
}

function buildRunStatusState(overrides: Record<string, unknown> = {}) {
  return {
    stage: "foundation_materialized",
    contract_name: "RunStatusContract",
    shell_command_name: "RunStatusState",
    lifecycle_surface_name: "RunLifecycleState",
    current_lifecycle_state: "run_draft",
    current_lifecycle_state_name: "RunDraft",
    current_run_id: null,
    current_session_id: null,
    current_workspace_reference: null,
    requested_round_count: null,
    completed_round_count: null,
    tracked_total_token_units: null,
    tracked_total_duration_ms: null,
    latest_status_note:
      "No fresh run/session has been created from current eligible Preflight truth yet",
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
    current_state_terminal: false,
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
    },
    ...overrides
  };
}

function buildRunEventContractState(overrides: Record<string, unknown> = {}) {
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
    live_event_capture_materialized: true,
    transcript_event_chronology_materialized: false,
    payload_secret_material_allowed: false,
    frontend_ordering_authority_allowed: false,
    latest_available_sequence_number: null,
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
    execution_completion_state: null,
    runtime_cap_reason: null,
    controlled_finalization_performed: false,
    finalization_started_at: null,
    finalization_completed_at: null,
    report_identifier: null,
    report_generated_at: null,
    durable_report_artifact_path: null,
    report_generation_status: null,
    report_generation_failure_reason: null,
    start_blocking_reasons: [],
    technical_failure_reason: null,
    detail_message:
      "No fresh run/session has been created from current eligible Preflight truth yet",
    ...overrides
  };
}

function buildRunSessionStartOutcome(overrides: Record<string, unknown> = {}) {
  return {
    decision_type: "execution_completed",
    technical_failure_reason: null,
    detail_message:
      "Moderator-controlled execution completed bounded rounds with speaker attribution, continuity mutation, and run-local token/duration tracking",
    run_session_state: buildRunSessionState({
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
      detail_message:
        "Moderator-controlled execution completed bounded rounds with speaker attribution, continuity mutation, and run-local token/duration tracking"
    }),
    ...overrides
  };
}

function buildSubmissionState(overrides: Record<string, unknown> = {}) {
  return {
    status: "not_started",
    selected_seed_document: null,
    requested_plan: null,
    valid_document_in_pending_configuration: false,
    effective_plan: null,
    estimated_usage: null,
    provider_model_context: null,
    requested_plan_budget_feasible: null,
    downgrade_warning: null,
    downgrade_acceptance_recorded: false,
    last_rejection_reason: null,
    last_technical_failure_reason: null,
    detail_message: "Preflight has not received a seed document submission yet",
    ...overrides
  };
}

function buildSelectedSeedDocument(overrides: Record<string, unknown> = {}) {
  return {
    file_name: "seed.txt",
    mime_type: "text/plain",
    detected_kind: "txt",
    size_bytes: 27,
    content_fingerprint_sha256: "abc123def456",
    extracted_text_character_count: 1420,
    english_confidence_percent: 99,
    ...overrides
  };
}

function buildRequestedPlan(overrides: Record<string, unknown> = {}) {
  return {
    requested_agent_count: 5,
    requested_round_count: 4,
    max_token_budget: 12000,
    ...overrides
  };
}

function buildEffectivePlan(overrides: Record<string, unknown> = {}) {
  return {
    effective_agent_count: 5,
    effective_round_count: 4,
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
    estimated_total_tokens: 10980,
    estimated_duration_minutes_low: 28,
    estimated_duration_minutes_high: 44,
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

function buildPreflightSubmissionOutcome(overrides: Record<string, unknown> = {}) {
  return {
    decision_type: "approved",
    rejection_reason: null,
    technical_failure_reason: null,
    submission_state: buildSubmissionState({
      status: "approved",
      selected_seed_document: buildSelectedSeedDocument(),
      requested_plan: buildRequestedPlan(),
      valid_document_in_pending_configuration: true,
      effective_plan: buildEffectivePlan(),
      estimated_usage: buildEstimatedUsage(),
      provider_model_context: buildProviderModelContext(),
      requested_plan_budget_feasible: true,
      detail_message:
        "Preflight approved the requested plan under the bounded local provider/model planning basis"
    }),
    detail_message:
      "Preflight approved the requested plan under the bounded local provider/model planning basis",
    ...overrides
  };
}

function configureInvokeMock(options: {
  preflightStates: Array<Record<string, unknown>>;
  runStartStates?: Array<Record<string, unknown>>;
  runtimePersonaStates?: Array<Record<string, unknown>>;
  runBoundedMemoryStates?: Array<Record<string, unknown>>;
  runModeratedRoundsStates?: Array<Record<string, unknown>>;
  runTokenDurationTrackingStates?: Array<Record<string, unknown>>;
  runLifecycleStates?: Array<Record<string, unknown>>;
  runStatusStates?: Array<Record<string, unknown>>;
  runEventContractStates?: Array<Record<string, unknown>>;
  runSessionStates?: Array<Record<string, unknown>>;
  reportAccessContractStates?: Array<Record<string, unknown>>;
  reportAccessMetadataStates?: Array<Record<string, unknown>>;
  startRunOutcome?: Record<string, unknown>;
  submitOutcome?: Record<string, unknown>;
  submitError?: Error;
  openReportOutcome?: Record<string, unknown>;
  exportReportOutcome?: Record<string, unknown>;
}) {
  let preflightStateIndex = 0;
  let runStartStateIndex = 0;
  let runtimePersonaStateIndex = 0;
  let runBoundedMemoryStateIndex = 0;
  let runModeratedRoundsStateIndex = 0;
  let runTokenDurationTrackingStateIndex = 0;
  let runLifecycleStateIndex = 0;
  let runStatusStateIndex = 0;
  let runEventContractStateIndex = 0;
  let runSessionStateIndex = 0;
  let reportAccessContractStateIndex = 0;
  let reportAccessMetadataStateIndex = 0;

  invokeMock.mockImplementation(async (command: string) => {
    switch (command) {
      case "provider_configuration_state":
        return buildProviderConfigurationState();
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
      case "run_lifecycle_state": {
        const runLifecycleStates = options.runLifecycleStates ?? [buildRunLifecycleState()];
        const nextState = runLifecycleStates[
          Math.min(runLifecycleStateIndex, runLifecycleStates.length - 1)
        ];
        runLifecycleStateIndex += 1;
        return nextState;
      }
      case "run_status_state": {
        const runStatusStates = options.runStatusStates ?? [buildRunStatusState()];
        const nextState = runStatusStates[
          Math.min(runStatusStateIndex, runStatusStates.length - 1)
        ];
        runStatusStateIndex += 1;
        return nextState;
      }
      case "run_event_contract_state": {
        const runEventContractStates = options.runEventContractStates ?? [buildRunEventContractState()];
        const nextState = runEventContractStates[
          Math.min(runEventContractStateIndex, runEventContractStates.length - 1)
        ];
        runEventContractStateIndex += 1;
        return nextState;
      }
      case "run_finalization_contract_state":
        return buildRunFinalizationContractState();
      case "report_output_contract_state":
        return buildReportOutputContractState();
      case "report_storage_boundary_state":
        return buildReportStorageBoundaryState();
      case "report_access_contract_state":
      {
        const reportAccessContractStates =
          options.reportAccessContractStates ?? [buildReportAccessContractState()];
        const nextState = reportAccessContractStates[
          Math.min(reportAccessContractStateIndex, reportAccessContractStates.length - 1)
        ];
        reportAccessContractStateIndex += 1;
        return nextState;
      }
      case "report_access_metadata_state": {
        const reportAccessMetadataStates = options.reportAccessMetadataStates ?? [
          {
            report_identifier: null,
            run_identifier: null,
            session_identifier: null,
            completion_classification: null,
            report_generation_status: null,
            report_generation_failure_reason: null,
            availability_status: "unavailable",
            availability_status_name: "ReportUnavailable",
            local_artifact_path: null,
            file_size_bytes: null,
            generated_at: null
          }
        ];
        const nextState = reportAccessMetadataStates[
          Math.min(reportAccessMetadataStateIndex, reportAccessMetadataStates.length - 1)
        ];
        reportAccessMetadataStateIndex += 1;
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
      case "open_local_report_pdf":
        return (
          options.openReportOutcome ?? {
            request_kind: "open_local_pdf",
            decision_type: "succeeded",
            failure_classification: null,
            availability_status: "available",
            availability_status_name: "ReportAvailable",
            local_artifact_path: "C:/reports/report-001.pdf",
            exported_artifact_path: null,
            detail_message: "Shell opened the durable local PDF artifact."
          }
        );
      case "export_local_report_pdf":
        return (
          options.exportReportOutcome ?? {
            request_kind: "export_local_pdf",
            decision_type: "succeeded",
            failure_classification: null,
            availability_status: "available",
            availability_status_name: "ReportAvailable",
            local_artifact_path: "C:/reports/report-001.pdf",
            exported_artifact_path: "C:/Users/test/Downloads/Miro Fish Reports/report-001.pdf",
            detail_message: "Shell exported the local PDF artifact."
          }
        );
      case "start_run_session":
        if (options.startRunOutcome) {
          return options.startRunOutcome;
        }

        throw new Error("Unexpected start_run_session invocation");
      case "submit_preflight":
        if (options.submitError) {
          throw options.submitError;
        }

        if (options.submitOutcome) {
          return options.submitOutcome;
        }

        throw new Error("Unexpected submit_preflight invocation");
      default:
        throw new Error(`Unexpected command: ${command}`);
    }
  });
}

function findInvokeCall(commandName: string) {
  return invokeMock.mock.calls.find(([command]) => command === commandName);
}

async function renderAppAndWaitForInitialPreflightState() {
  render(<App />);
  await screen.findByText("Canonical state name: PreflightNotStarted");
}

function selectSeedDocument(file: File) {
  fireEvent.change(screen.getByLabelText("Seed document"), {
    target: { files: [file] }
  });
}

function openRunScreen() {
  fireEvent.click(screen.getByRole("button", { name: "Run" }));
}

describe("App Preflight submission flow", () => {
  beforeEach(() => {
    invokeMock.mockReset();
  });

  it("submits a selected seed document, sends the expected shell payload, and renders the approved state", async () => {
    const initialState = buildPreflightReadinessState();
    const acceptedState = buildPreflightReadinessState({
      runtime_ready_now: true,
      activation_ready_now: true,
      provider_configuration_ready_now: true,
      pending_seed_document_selected: true,
      requested_plan_present: true,
      selected_seed_document: buildSelectedSeedDocument({
        size_bytes: 36,
        content_fingerprint_sha256: "feedbead1234"
      }),
      requested_plan: buildRequestedPlan(),
      effective_plan: buildEffectivePlan(),
      estimated_usage: buildEstimatedUsage(),
      provider_model_context: buildProviderModelContext(),
      requested_plan_budget_feasible: true,
      document_validation_completed: true,
      document_validation_passed: true,
      current_status_note:
        "Preflight approved the requested plan under the bounded local provider/model planning basis",
      current_state: "approved",
      current_state_name: "PreflightApproved",
      run_start_eligible_now: true,
      run_start_blocked_without_eligible_preflight: false
    });

    configureInvokeMock({
      preflightStates: [initialState, acceptedState],
      submitOutcome: buildPreflightSubmissionOutcome({
        submission_state: buildSubmissionState({
          status: "approved",
          selected_seed_document: buildSelectedSeedDocument({
            size_bytes: 36,
            content_fingerprint_sha256: "feedbead1234"
          }),
          requested_plan: buildRequestedPlan(),
          valid_document_in_pending_configuration: true,
          effective_plan: buildEffectivePlan(),
          estimated_usage: buildEstimatedUsage(),
          provider_model_context: buildProviderModelContext(),
          requested_plan_budget_feasible: true,
          detail_message:
            "Preflight approved the requested plan under the bounded local provider/model planning basis"
        }),
        detail_message:
          "Preflight approved the requested plan under the bounded local provider/model planning basis"
      })
    });

    await renderAppAndWaitForInitialPreflightState();

    const submitButton = screen.getByRole("button", { name: "Submit seed document" });
    expect((submitButton as HTMLButtonElement).disabled).toBe(true);

    const fileContent = "Seed input for accepted preflight flow";
    const file = new File([fileContent], "seed.txt", { type: "text/plain" });
    selectSeedDocument(file);

    await screen.findByText("Selected file: seed.txt");
    expect((submitButton as HTMLButtonElement).disabled).toBe(false);

    fireEvent.click(submitButton);

    await screen.findByText("Preflight submission result");
    await screen.findByText("Valid document pending: yes");
    await screen.findAllByText("Requested-plan budget feasible: yes");
    await screen.findByText("Selected seed document");
    await screen.findByText("File name: seed.txt");
    await screen.findByText("Effective plan");
    await screen.findAllByText("Model: gpt-4o-mini");
    await screen.findByText("Requested agent count: 5");
    await screen.findByText("Run start blocked without eligible Preflight: no");

    await waitFor(() => {
      expect(findInvokeCall("submit_preflight")).toBeDefined();
    });

    const submitCall = findInvokeCall("submit_preflight");
    expect(submitCall?.[1]).toEqual({
      request: {
        seed_document: {
          filename: "seed.txt",
          mime_type: "text/plain",
          size_bytes: file.size,
          bytes: Array.from(new TextEncoder().encode(fileContent))
        },
        requested_agent_count: 5,
        requested_round_count: 4,
        max_token_budget: 12000
      }
    });
  });

  it("renders deterministic rejection details when the shell rejects the submitted seed document", async () => {
    const initialState = buildPreflightReadinessState();
    const rejectedSeedDocument = buildSelectedSeedDocument({
      file_name: "seed.pdf",
      mime_type: "application/pdf",
      detected_kind: "pdf",
      size_bytes: 11485760,
      extracted_text_character_count: null,
      english_confidence_percent: null
    });
    const rejectedState = buildPreflightReadinessState({
      pending_seed_document_selected: true,
      requested_plan_present: true,
      selected_seed_document: rejectedSeedDocument,
      requested_plan: buildRequestedPlan(),
      document_validation_completed: true,
      document_validation_passed: false,
      current_state: "rejected",
      current_state_name: "PreflightRejected",
      last_rejection_reason: "file_too_large",
      current_status_note:
        "Seed document was rejected because it exceeds the governed 10 MiB file-size ceiling"
    });

    configureInvokeMock({
      preflightStates: [initialState, rejectedState],
      submitOutcome: buildPreflightSubmissionOutcome({
        decision_type: "rejected",
        rejection_reason: "file_too_large",
        submission_state: buildSubmissionState({
          status: "rejected",
          selected_seed_document: rejectedSeedDocument,
          requested_plan: buildRequestedPlan(),
          valid_document_in_pending_configuration: false,
          last_rejection_reason: "file_too_large",
          detail_message:
            "Seed document was rejected because it exceeds the governed 10 MiB file-size ceiling"
        }),
        detail_message:
          "Seed document was rejected because it exceeds the governed 10 MiB file-size ceiling"
      })
    });

    await renderAppAndWaitForInitialPreflightState();

    const file = new File(["rejected content"], "seed.pdf", {
      type: "application/pdf"
    });
    selectSeedDocument(file);

    fireEvent.click(screen.getByRole("button", { name: "Submit seed document" }));

    await screen.findByText("Rejection reason: File Too Large");
    await screen.findByText("Valid document pending: no");
    await screen.findByText("Canonical state name: PreflightRejected");
    await screen.findByText("Last rejection reason: File Too Large");
  });

  it("renders a technical failure outcome when the shell command fails", async () => {
    const initialState = buildPreflightReadinessState();

    configureInvokeMock({
      preflightStates: [initialState, initialState],
      submitError: new Error("shell boundary unavailable")
    });

    await renderAppAndWaitForInitialPreflightState();

    const file = new File(["technical failure content"], "seed.txt", {
      type: "text/plain"
    });
    selectSeedDocument(file);

    fireEvent.click(screen.getByRole("button", { name: "Submit seed document" }));

    await screen.findByText("Technical failure reason: State Access Failure");
    await screen.findByText("shell boundary unavailable");
    await screen.findByText("Canonical state name: PreflightNotStarted");

    await waitFor(() => {
      expect(findInvokeCall("submit_preflight")).toBeDefined();
    });
  });

  it("starts a fresh moderated run and renders finished execution truth when run start is authorized", async () => {
    const approvedPreflightState = buildPreflightReadinessState({
      runtime_ready_now: true,
      activation_ready_now: true,
      provider_configuration_ready_now: true,
      pending_seed_document_selected: true,
      requested_plan_present: true,
      selected_seed_document: buildSelectedSeedDocument({
        size_bytes: 36,
        content_fingerprint_sha256: "feedbead1234"
      }),
      requested_plan: buildRequestedPlan(),
      effective_plan: buildEffectivePlan(),
      estimated_usage: buildEstimatedUsage(),
      provider_model_context: buildProviderModelContext(),
      requested_plan_budget_feasible: true,
      document_validation_completed: true,
      document_validation_passed: true,
      current_state: "approved",
      current_state_name: "PreflightApproved",
      run_start_eligible_now: true,
      run_start_blocked_without_eligible_preflight: false,
      current_status_note:
        "Preflight approved the requested plan under the bounded local provider/model planning basis"
    });
    const runStartReadyState = buildRunStartReadinessState({
      runtime_ready_now: true,
      activation_ready_now: true,
      provider_configuration_ready_now: true,
      current_eligible_preflight_now: true,
      start_lineage_ready_now: true,
      current_state: "blocked",
      current_state_name: "RunStartFoundationBlocked",
      real_run_start_authorized_now: true,
      current_status_note:
        "All run-start prerequisites are satisfied for a fresh run/session start"
    });
    const pendingExecutionOutcome = buildRunSessionStartOutcome();

    configureInvokeMock({
      preflightStates: [approvedPreflightState, approvedPreflightState],
      runStartStates: [runStartReadyState, runStartReadyState],
      runSessionStates: [buildRunSessionState(), pendingExecutionOutcome.run_session_state],
      startRunOutcome: pendingExecutionOutcome
    });

    render(<App />);
    await screen.findByText("Canonical state name: PreflightApproved");

    openRunScreen();

    fireEvent.click(screen.getByRole("button", { name: "Start fresh moderated run" }));

    await screen.findByText("Run/session start result");
    await screen.findByText("Execution Completed");
    await screen.findByText("Run id: run-1");
    await screen.findByText("Workspace allocated: yes");
    await screen.findByText("Execution seed staged: yes");
    await screen.findByText("Moderator label: Moderator");
    await screen.findByText("Participant persona count: 5");
    await screen.findByText("Participant persona labels: Advocate, Skeptic, Analyst, Builder, Auditor");
    await screen.findByText("Perspective coverage: Favorable, Skeptical, Analytical, Neutral");
    await screen.findByText("Transcript archive separated: yes");
    await screen.findByText("Completed round count: 4");
    await screen.findByText("Speaker counts per round: 3, 4, 5, 3");
    await screen.findByText("Tracked total token units: 610");
    await screen.findByText(
      "Token/duration tracking path: C:/temp/run-1/run-local-tracking/token_duration_tracking.json"
    );

    await waitFor(() => {
      expect(findInvokeCall("start_run_session")).toBeDefined();
    });
  });

  it("renders execution failure truth when report generation fails after debate execution", async () => {
    const approvedPreflightState = buildPreflightReadinessState({
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
      requested_plan_budget_feasible: true,
      document_validation_completed: true,
      document_validation_passed: true,
      current_state: "approved",
      current_state_name: "PreflightApproved",
      run_start_eligible_now: true,
      run_start_blocked_without_eligible_preflight: false,
      current_status_note:
        "Preflight approved the requested plan under the bounded local provider/model planning basis"
    });
    const runStartReadyState = buildRunStartReadinessState({
      runtime_ready_now: true,
      activation_ready_now: true,
      provider_configuration_ready_now: true,
      current_eligible_preflight_now: true,
      start_lineage_ready_now: true,
      current_state: "blocked",
      current_state_name: "RunStartFoundationBlocked",
      real_run_start_authorized_now: true,
      current_status_note:
        "All run-start prerequisites are satisfied for a fresh run/session start"
    });
    const failedOutcome = buildRunSessionStartOutcome({
      decision_type: "execution_failed",
      technical_failure_reason: "report_generation_failed",
      detail_message:
        "Moderator-controlled execution completed debate material, but report data assembly failed from the actual run artifacts.",
      run_session_state: buildRunSessionState({
        current_state_class: "technical_failure",
        current_state_name: "technical_failure",
        technical_failure_reason: "report_generation_failed",
        lifecycle_override: "failed_terminal",
        moderated_rounds_materialized: true,
        token_duration_tracking_materialized: true,
        runtime_persona_foundation_materialized: true,
        bounded_memory_foundation_materialized: true,
        transcript_archive_separated: true,
        detail_message:
          "Moderator-controlled execution completed debate material, but report data assembly failed from the actual run artifacts."
      })
    });

    configureInvokeMock({
      preflightStates: [approvedPreflightState, approvedPreflightState],
      runStartStates: [runStartReadyState, runStartReadyState],
      runSessionStates: [buildRunSessionState(), failedOutcome.run_session_state],
      startRunOutcome: failedOutcome
    });

    render(<App />);
    await screen.findByText("Canonical state name: PreflightApproved");

    openRunScreen();

    fireEvent.click(screen.getByRole("button", { name: "Start fresh moderated run" }));

    await screen.findByText("Run/session start result");
    await screen.findByText("Execution Failed");
    await screen.findByText("Technical failure reason: Report Generation Failed");
    expect(
      await screen.findAllByText(
        "Moderator-controlled execution completed debate material, but report data assembly failed from the actual run artifacts."
      )
    ).not.toHaveLength(0);
  });
});