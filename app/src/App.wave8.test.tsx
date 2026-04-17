import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import App from "./App";

const { invokeMock } = vi.hoisted(() => ({
  invokeMock: vi.fn()
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: invokeMock
}));

function configureInvokeMock() {
  invokeMock.mockImplementation(async (command: string) => {
    switch (command) {
      case "provider_configuration_state":
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
      case "preflight_readiness_state":
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
          run_start_blocked_without_eligible_preflight: true
        };
      case "run_start_readiness_state":
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
            "Run-start foundation is materialized, but current eligible Preflight lineage does not exist yet"
        };
      case "runtime_persona_contract_state":
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
      case "run_bounded_memory_state":
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
      case "run_moderated_rounds_state":
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
      case "run_token_duration_tracking_state":
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
      case "run_lifecycle_state":
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
            "No shell-owned run/session has been created yet, so the canonical lifecycle remains at RunDraft."
        };
      case "run_status_state":
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
      case "run_event_contract_state":
        return {
          stage: "foundation_materialized",
          contract_name: "RunEventContract",
          shell_command_name: "RunEventContractState",
          event_history_command_name: "RunEventHistory",
          supported_event_types: ["run_state_changed"],
          supported_runtime_domains: ["shell"],
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
          latest_available_sequence_number: null
        };
      case "run_finalization_contract_state":
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
          real_pdf_generation_materialized: true
        };
      case "run_cleanup_contract_state":
        return {
          stage: "foundation_materialized",
          contract_name: "RunCleanupContract",
          shell_command_name: "RunCleanupContractState",
          supported_markers: [
            "cleanup_pending",
            "cleanup_running",
            "cleanup_succeeded",
            "cleanup_partially_failed",
            "cleanup_failed"
          ],
          shell_authority_required: true,
          cleanup_pending_distinct: true,
          cleanup_running_distinct: true,
          cleanup_succeeded_distinct: true,
          cleanup_partially_failed_distinct: true,
          cleanup_failed_distinct: true,
          cleanup_attempt_distinct_from_cleanup_outcome: true,
          cleanup_outcome_distinct_from_run_outcome: true,
          cleanup_partial_failure_requires_visible_reason: true,
          cleanup_outcome_implies_report_deletion: false,
          real_cleanup_execution_materialized: true,
          real_cleanup_result_emission_materialized: true
        };
      case "cleanup_scope_mapping_state":
        return {
          stage: "foundation_materialized",
          mapping_name: "CleanupScopeMapping",
          shell_command_name: "CleanupScopeMappingState",
          cleanup_entries: [
            {
              data_class_name: "TransientRunWorkspaceData",
              storage_domain_name: "PerRunWorkspace",
              cleanup_scope_applies: true,
              startup_janitor_scope_applies: true,
              shell_owned_boundary: true,
              preserved_report_artifact: false
            },
            {
              data_class_name: "RunSupportMetadata",
              storage_domain_name: "RunSupportMetadata",
              cleanup_scope_applies: true,
              startup_janitor_scope_applies: true,
              shell_owned_boundary: true,
              preserved_report_artifact: false
            },
            {
              data_class_name: "DurableReportArtifact",
              storage_domain_name: "DurableReportArtifactDomain",
              cleanup_scope_applies: false,
              startup_janitor_scope_applies: false,
              shell_owned_boundary: true,
              preserved_report_artifact: true
            },
            {
              data_class_name: "ReportMetadataIndexReference",
              storage_domain_name: "ReportMetadataIndexDomain",
              cleanup_scope_applies: false,
              startup_janitor_scope_applies: false,
              shell_owned_boundary: true,
              preserved_report_artifact: false
            },
            {
              data_class_name: "DurableAppLocalState",
              storage_domain_name: "DurableAppLocalState",
              cleanup_scope_applies: false,
              startup_janitor_scope_applies: false,
              shell_owned_boundary: true,
              preserved_report_artifact: false
            }
          ],
          explicit_scope_mapping_materialized: true,
          shell_authority_required: true,
          terminal_cleanup_scope_defined: true,
          startup_janitor_scope_defined: true,
          preserved_report_exception_explicit: true,
          durable_support_state_exclusion_explicit: true,
          crash_resume_continuation_allowed: false,
          report_reclassification_as_workspace_garbage_allowed: false
        };
      case "startup_janitor_contract_state":
        return {
          stage: "foundation_materialized",
          contract_name: "StartupJanitorContract",
          shell_command_name: "StartupJanitorContractState",
          supported_markers: [
            "pending_next_launch",
            "janitor_running",
            "janitor_purge_succeeded",
            "janitor_purge_failed"
          ],
          shell_authority_required: true,
          abnormal_termination_detection_required: true,
          pending_next_launch_distinct: true,
          janitor_running_distinct: true,
          janitor_purge_succeeded_distinct: true,
          janitor_purge_failed_distinct: true,
          janitor_outcome_distinct_from_run_outcome: true,
          janitor_requires_shell_owned_workspace_classification: true,
          janitor_purge_implies_crash_resume: false,
          real_startup_janitor_execution_materialized: true,
          real_abnormal_termination_detection_materialized: true,
          execution_checked_this_launch: true,
          latest_marker: "janitor_purge_succeeded",
          latest_marker_name: "JanitorPurgeSucceeded",
          janitor_running_now: false,
          abnormal_residual_detected_this_launch: true,
          residual_workspace_count: 1,
          residual_marker_count: 1,
          purged_workspace_count: 1,
          failed_workspace_count: 0,
          preserved_report_artifact_count: 1,
          preserved_report_artifact_retained_count: 1,
          started_at: 1713240000000,
          completed_at: 1713240000500,
          timed_out: false,
          latest_status_note:
            "Startup janitor removed 1 abandoned workspace before normal readiness continued. Any preserved durable report artifacts referenced by residual markers remained outside janitor scope."
        };
      case "report_output_contract_state":
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
        };
      case "report_storage_boundary_state":
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
          }
        };
      case "report_access_contract_state":
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
          cleanup_outcome_implies_report_deletion: false
        };
      case "report_access_metadata_state":
        return {
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
        };
      case "open_local_report_pdf":
        return {
          request_kind: "open_local_pdf",
          decision_type: "failed",
          failure_classification: "unavailable",
          availability_status: "unavailable",
          availability_status_name: "ReportUnavailable",
          local_artifact_path: null,
          exported_artifact_path: null,
          detail_message: "No durable local PDF artifact is available yet."
        };
      case "export_local_report_pdf":
        return {
          request_kind: "export_local_pdf",
          decision_type: "failed",
          failure_classification: "unavailable",
          availability_status: "unavailable",
          availability_status_name: "ReportUnavailable",
          local_artifact_path: null,
          exported_artifact_path: null,
          detail_message: "No durable local PDF artifact is available yet."
        };
      case "run_event_history":
        return {
          command_name: "RunEventHistory",
          current_run_id: null,
          current_session_id: null,
          requested_after_sequence_number: null,
          requested_max_items: 256,
          latest_available_sequence_number: null,
          returned_item_count: 0,
          returned_event_count: 0,
          returned_chronology_entry_count: 0,
          gap_detected: false,
          gap_reason: null,
          stored_history_complete: false,
          event_history_materialized: true,
          transcript_event_chronology_materialized: true,
          live_usage_visibility_materialized: true,
          events: [],
          chronology_entries: []
        };
      case "run_session_state":
        return {
          stage: "start_path_materialized",
          current_state_class: "not_started",
          current_state_name: "not_started",
          detail_message: "No run has been started yet.",
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
          participant_persona_count: null,
          participant_persona_labels: [],
          perspective_coverage: [],
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
          runtime_cap_reason: null,
          controlled_finalization_performed: false,
          finalization_started_at: null,
          finalization_completed_at: null,
          runtime_persona_foundation_materialized: true,
          bounded_memory_foundation_materialized: true,
          transcript_archive_separated: true,
          exactly_one_moderator_present: true,
          stable_persona_labels: true,
          moderated_rounds_materialized: true,
          normal_round_speaker_selection_enforced: true,
          speaker_attribution_materialized: true,
          bounded_memory_continuity_mutated: false,
          token_duration_tracking_materialized: true,
          latest_start_lineage_captured: false,
          workspace_allocated: false,
          execution_seed_staged: false,
          pending_execution_ready: false,
          start_blocking_reasons: [],
          technical_failure_reason: null,
          degradation_reason: null,
          retry_attempt_count: 0,
          max_retry_attempts: 0,
          cancellation_request_accepted: false,
          shutdown_requested: false,
          workspace_cleanup_pending: false,
          cleanup_completed: false,
          cleanup_failed: false,
          lifecycle_override: null
        };
      default:
        throw new Error(`Unexpected command: ${command}`);
    }
  });
}

describe("App Wave 8 report-contract UI", () => {
  beforeEach(() => {
    invokeMock.mockReset();
  });

  it("renders bounded Phase 8 finalization, output, storage, and access contract truth", async () => {
    configureInvokeMock();

    render(<App />);
    fireEvent.click(screen.getByRole("button", { name: "Reports" }));

    await screen.findByText("Phase 8 finalization and report contract foundation");
    await screen.findByText("RunFinalizationContract");
    await screen.findByText("ReportOutputContract");
    await screen.findByText("ReportStorageBoundary");
    await screen.findByText("Local report PDF access");
    await screen.findByText("Phase 9 cleanup and janitor execution truth");
    await screen.findByText("RunCleanupContract");
    await screen.findByText("CleanupScopeMapping");
    await screen.findByText("StartupJanitorContract");
    await screen.findByText("Cleanup execution materialized: yes");
    await screen.findByText("Startup janitor execution materialized: yes");
    await screen.findByText("Preserved report cleanup exception: yes");
    await screen.findByText("Startup janitor checked this launch: yes");
    await screen.findByText("Latest janitor marker: JanitorPurgeSucceeded");
    await screen.findByText("Residual workspaces detected: 1");
    await screen.findByText("Residual workspaces purged: 1");
    await screen.findByText("TransientRunWorkspaceData");
    await screen.findByText("DurableReportArtifact");
    await screen.findByText("Local report PDF");
    await screen.findByText("Completion implies report generation success: yes");
    await screen.findByText("Real report generation materialized: yes");
    await screen.findByText("Real PDF generation materialized: yes");
    await screen.findByText("PDF format: PDF");
    await screen.findByText("Successful PDF generation implied: yes");
    await screen.findByText(
      "These Phase 8 surfaces now materialize governed PDF generation, durable app-local report artifacts, shell-owned availability truth, and shell-mediated local open/export actions. The frontend only renders report access from current shell state and does not infer file availability from run completion alone."
    );
    await screen.findByText(
      "These Phase 9 surfaces now materialize real shell-owned terminal cleanup, abnormal-leftover detection, and startup janitor purge execution while preserving exported reports and durable app-local state outside cleanup and janitor scope."
    );
    await screen.findByText(
      "Startup janitor removed 1 abandoned workspace before normal readiness continued. Any preserved durable report artifacts referenced by residual markers remained outside janitor scope."
    );
    await screen.findByLabelText("Preview state (dev only)");
    fireEvent.click(screen.getByText("Technical access contract"));
    await screen.findByText("Contract name: ReportAccessContract");
    await screen.findByText("Open local PDF");
    await screen.findByText("Export local PDF");
  });

  it("supports dev-only preview states for human review of the report artifact card", async () => {
    configureInvokeMock();

    render(<App />);
    fireEvent.click(screen.getByRole("button", { name: "Reports" }));

    const previewSelector = await screen.findByLabelText("Preview state (dev only)");
    fireEvent.change(previewSelector, { target: { value: "available" } });

    await screen.findByText("Report ready");
    await screen.findByText("Showing Available. This affects the UI preview only; the live shell state still refreshes underneath.");
    await screen.findByText("Report ID");
    await screen.findByText("report-run-042");

    const openButton = screen.getByRole("button", { name: "Open local PDF" }) as HTMLButtonElement;
    const exportButton = screen.getByRole("button", { name: "Export local PDF" }) as HTMLButtonElement;
    expect(openButton.disabled).toBe(false);
    expect(exportButton.disabled).toBe(false);

    fireEvent.click(exportButton);

    await screen.findByText("Latest action");
    await screen.findByText("Export Local Pdf Succeeded");
    await screen.findByText("Preview only: the shell would export a copy of the durable PDF artifact.");
    await screen.findByText(
      "Exported artifact path: C:/Users/mikes/Downloads/Miro Fish Reports/report-run-042.pdf"
    );
  });
});