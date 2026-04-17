export const sections = [
  "The frontend now exercises three real shell-owned local flows: activation request handling, provider configuration commit handling, and single-seed Preflight submission with bounded feasibility planning outcomes.",
  "The Rust/Tauri shell owns device identity derivation, secure storage, provider-key commit, document parsing, language detection, and the current Preflight result state.",
  "Phase 6 Wave 2 now also freezes shell-owned run/session start contracts, fresh-workspace isolation rules, and Preflight-lineage start truth without starting a run.",
  "Phase 6 Wave 3 now materializes real shell-owned fresh run/session creation, fresh isolated workspace allocation, and execution-seed staging from current eligible Preflight truth without yet starting moderated debate execution.",
  "Phase 6 Wave 4 now materializes engine-owned runtime persona generation, exactly one Moderator basis, bounded memory foundation, and transcript-archive separation from the staged seed workspace without yet starting moderated rounds.",
  "The provider setup flow is bounded to OpenAI and Google only.",
  "Supported MVP seed documents are PDF, DOCX, TXT, and Markdown, with a governed 10 MiB file-size ceiling.",
  "Preflight now distinguishes approved, approved with downgrade, rejected, stale invalidated, and technical failure truth as separate shell-owned outcomes.",
  "Raw provider secrets and raw activation tokens are never retrieved back into the frontend after submission.",
  "Provider configuration success means secure local configuration is present only; it still does not imply provider reachability, model availability, or broader Preflight feasibility.",
  "Preflight now returns requested-plan echoes, effective-plan summaries, bounded token and duration estimates, and provider/model planning context without starting execution.",
  "Run start remains blocked without an eligible current Preflight result, and approved-with-downgrade still requires explicit effective-plan acceptance before execution may begin.",
  "Bounded provider support state persists only non-secret metadata in the app-controlled support domain.",
  "The Python commercial backend remains bounded to entitlement, activation, payment-event, audit, and token-lineage behavior only.",
  "No local HTTP surface, live provider reachability probes, full simulation execution, runtime hydration, or updater behavior is implemented by this screen."
] as const;

export type AppScreen = "setup" | "run" | "reports" | "diagnostics";
export type SupportedProvider = "open_ai" | "google";
export type ReportAvailabilityStatus =
  | "unavailable"
  | "available"
  | "missing_artifact"
  | "inaccessible_artifact";
export type ReportAccessRequestKind = "open_local_pdf" | "export_local_pdf";
export type ReportAccessPreviewMode = "live" | ReportAvailabilityStatus;
export type EventPayload = Record<string, unknown>;

export interface ScreenMeta {
  label: string;
  eyebrow: string;
  title: string;
  detail: string;
}

export const appScreenContent: Record<AppScreen, ScreenMeta> = {
  setup: {
    label: "Setup",
    eyebrow: "Workspace setup",
    title: "Setup and Preflight",
    detail:
      "Activation, provider configuration, Preflight submission, and the current bounded foundation surfaces."
  },
  run: {
    label: "Run",
    eyebrow: "Execution",
    title: "Run Monitoring",
    detail:
      "Fresh run/session start, live monitoring, chronology, and shell-owned execution control surfaces."
  },
  reports: {
    label: "Reports",
    eyebrow: "Outputs",
    title: "Reports and Access",
    detail:
      "Phase 8 finalization, PDF output, storage boundaries, and shell-mediated report access surfaces."
  },
  diagnostics: {
    label: "Diagnostics",
    eyebrow: "Development truth",
    title: "Diagnostics",
    detail:
      "Collapsible technical surfaces for runtime personas, bounded memory, moderated rounds, and repo truth."
  }
};

export interface ActivationResult {
  decision_type: string;
  denial_reason?: string | null;
  technical_failure_reason?: string | null;
  detail_message?: string | null;
}

export interface ProviderState {
  current_status: string;
  current_detail?: string | null;
  configuration_present: boolean;
  selected_provider?: SupportedProvider | null;
}

export interface ProviderOutcome {
  decision_type: string;
  failure_reason?: string | null;
  detail_message?: string | null;
  configuration_state?: ProviderState;
}

export interface SeedDocumentSummary {
  file_name: string;
  detected_kind?: string | null;
}

export interface RequestedPlanSummary {
  requested_agent_count: number;
  requested_round_count: number;
  max_token_budget: number;
}

export interface EffectivePlanSummary {
  effective_agent_count: number;
  effective_round_count: number;
  provider?: SupportedProvider | string | null;
  model?: string | null;
}

export interface PreflightSubmissionState {
  status?: string;
  valid_document_in_pending_configuration: boolean;
  requested_plan_budget_feasible?: boolean | null;
  selected_seed_document?: SeedDocumentSummary | null;
  requested_plan?: RequestedPlanSummary | null;
  effective_plan?: EffectivePlanSummary | null;
  detail_message?: string | null;
}

export type PreflightDetailsSource = Pick<
  PreflightSubmissionState,
  | "valid_document_in_pending_configuration"
  | "requested_plan_budget_feasible"
  | "selected_seed_document"
  | "requested_plan"
  | "effective_plan"
>;

export interface PreflightOutcome {
  decision_type: string;
  rejection_reason?: string | null;
  technical_failure_reason?: string | null;
  detail_message?: string | null;
  submission_state?: PreflightSubmissionState | null;
}

export interface PreflightState {
  current_state_name: string;
  run_start_eligible_now: boolean;
  run_start_blocked_without_eligible_preflight: boolean;
  current_result_stale: boolean;
  current_status_note?: string | null;
  invalidation_reasons?: string[];
  last_rejection_reason?: string | null;
  last_technical_failure_reason?: string | null;
  downgrade_acceptance_required?: boolean;
  downgrade_warning?: string | null;
  valid_document_in_pending_configuration?: boolean;
  requested_plan_budget_feasible?: boolean | null;
  selected_seed_document?: SeedDocumentSummary | null;
  requested_plan?: RequestedPlanSummary | null;
  effective_plan?: EffectivePlanSummary | null;
}

export interface DowngradeAcceptanceOutcome {
  acceptance_recorded: boolean;
  detail_message?: string | null;
}

export interface RuntimePersonaState {
  contract_name?: string | null;
  exactly_one_moderator_required?: boolean;
  stable_persona_labels_required?: boolean;
}

export interface RunBoundedMemoryState {
  memory_surface_name?: string | null;
  transcript_archive_separate_from_prompt_memory?: boolean;
}

export interface RunModeratedRoundsState {
  surface_name?: string | null;
  normal_round_min_speakers?: number | null;
}

export interface RunTokenDurationTrackingState {
  surface_name?: string | null;
  integrated_into_execution_path?: boolean;
}

export interface RunStartState {
  current_state_name: string;
  real_run_start_authorized_now: boolean;
}

export interface RunSessionState {
  current_state_class?: string | null;
  current_state_name?: string | null;
  run_id?: string | null;
  session_id?: string | null;
  workspace_allocated?: boolean;
  execution_seed_staged?: boolean;
  moderator_persona_label?: string | null;
  participant_persona_count?: number | null;
  participant_persona_labels?: string[];
  perspective_coverage?: string[];
  transcript_archive_separated?: boolean;
  completed_round_count?: number | null;
  speaker_counts_per_round?: number[];
  tracked_total_token_units?: number | null;
  token_duration_tracking_path?: string | null;
  execution_completion_state?: string | null;
  runtime_cap_reason?: string | null;
  controlled_finalization_performed?: boolean;
  cleanup_marker?: string | null;
  detail_message?: string | null;
  lifecycle_override?: string | null;
  report_identifier?: string | null;
  report_generation_failure_reason?: string | null;
  report_generated_at?: number | null;
  durable_report_artifact_path?: string | null;
}

export interface RunSessionOutcome {
  decision_type: string;
  technical_failure_reason?: string | null;
  detail_message?: string | null;
  run_session_state?: RunSessionState | null;
}

export interface RunLifecycleState {
  surface_name?: string | null;
  current_state?: string | null;
  current_state_name?: string | null;
  current_projection_detail?: string | null;
}

export interface RunCancellationRequestContract {
  cancellation_command_implemented?: boolean;
}

export interface RunStatusState {
  contract_name?: string | null;
  shell_command_name?: string | null;
  current_state_cancelable?: boolean;
  current_state_terminal?: boolean;
  current_run_id?: string | null;
  current_lifecycle_state?: string | null;
  current_lifecycle_state_name?: string | null;
  ordered_event_history_materialized?: boolean;
  tracked_total_token_units?: number | null;
  cancellation_request_contract?: RunCancellationRequestContract;
}

export interface RunEventContractState {
  contract_name?: string | null;
  event_history_command_name?: string | null;
  event_history_retrieval_implemented?: boolean;
  gap_contract?: {
    gap_marker_supported?: boolean;
  };
  supported_runtime_domains?: string[];
  event_history_materialized?: boolean;
}

export interface RunEvent {
  sequence_number: number;
  event_type?: string | null;
  payload?: EventPayload | null;
}

export interface RunChronologyEntry {
  sequence_number: number;
  round_number?: number | null;
  turn_index?: number | null;
  turn_text?: string | null;
}

export interface RunEventHistoryState {
  returned_item_count?: number;
  stored_history_complete?: boolean;
  events?: RunEvent[];
  chronology_entries?: RunChronologyEntry[];
}

export interface RunFinalizationContractState {
  contract_name?: string | null;
  completion_classification_implies_report_generation_success?: boolean;
  real_report_generation_materialized?: boolean;
  real_pdf_generation_materialized?: boolean;
}

export interface RunCleanupContractState {
  contract_name?: string | null;
  shell_command_name?: string | null;
  supported_markers?: string[];
  shell_authority_required?: boolean;
  cleanup_pending_distinct?: boolean;
  cleanup_running_distinct?: boolean;
  cleanup_succeeded_distinct?: boolean;
  cleanup_partially_failed_distinct?: boolean;
  cleanup_failed_distinct?: boolean;
  cleanup_attempt_distinct_from_cleanup_outcome?: boolean;
  cleanup_outcome_distinct_from_run_outcome?: boolean;
  cleanup_partial_failure_requires_visible_reason?: boolean;
  cleanup_outcome_implies_report_deletion?: boolean;
  real_cleanup_execution_materialized?: boolean;
  real_cleanup_result_emission_materialized?: boolean;
}

export interface CleanupScopeEntryState {
  data_class_name?: string | null;
  storage_domain_name?: string | null;
  cleanup_scope_applies?: boolean;
  startup_janitor_scope_applies?: boolean;
  shell_owned_boundary?: boolean;
  preserved_report_artifact?: boolean;
}

export interface CleanupScopeMappingState {
  mapping_name?: string | null;
  shell_command_name?: string | null;
  cleanup_entries?: CleanupScopeEntryState[];
  explicit_scope_mapping_materialized?: boolean;
  shell_authority_required?: boolean;
  terminal_cleanup_scope_defined?: boolean;
  startup_janitor_scope_defined?: boolean;
  preserved_report_exception_explicit?: boolean;
  durable_support_state_exclusion_explicit?: boolean;
  crash_resume_continuation_allowed?: boolean;
  report_reclassification_as_workspace_garbage_allowed?: boolean;
}

export interface StartupJanitorContractState {
  contract_name?: string | null;
  shell_command_name?: string | null;
  supported_markers?: string[];
  shell_authority_required?: boolean;
  abnormal_termination_detection_required?: boolean;
  pending_next_launch_distinct?: boolean;
  janitor_running_distinct?: boolean;
  janitor_purge_succeeded_distinct?: boolean;
  janitor_purge_failed_distinct?: boolean;
  janitor_outcome_distinct_from_run_outcome?: boolean;
  janitor_requires_shell_owned_workspace_classification?: boolean;
  janitor_purge_implies_crash_resume?: boolean;
  real_startup_janitor_execution_materialized?: boolean;
  real_abnormal_termination_detection_materialized?: boolean;
  execution_checked_this_launch?: boolean;
  latest_marker?: string | null;
  latest_marker_name?: string | null;
  janitor_running_now?: boolean;
  abnormal_residual_detected_this_launch?: boolean;
  residual_workspace_count?: number | null;
  residual_marker_count?: number | null;
  purged_workspace_count?: number | null;
  failed_workspace_count?: number | null;
  preserved_report_artifact_count?: number | null;
  preserved_report_artifact_retained_count?: number | null;
  started_at?: number | null;
  completed_at?: number | null;
  timed_out?: boolean;
  latest_status_note?: string | null;
}

export interface ReportOutputContractState {
  contract_name?: string | null;
  pdf_contract?: {
    file_format_name?: string | null;
    successful_generation_implied?: boolean;
  };
}

export interface ReportStorageBoundaryState {
  boundary_name?: string | null;
}

export interface ReportAccessContractState {
  contract_name?: string | null;
  shell_command_name?: string | null;
  report_available_now?: boolean;
  local_artifact_accessible_now?: boolean;
  current_availability_status?: ReportAvailabilityStatus;
  current_availability_status_name?: string | null;
}

export interface ReportAccessMetadataState {
  availability_status?: ReportAvailabilityStatus;
  availability_status_name?: string | null;
  report_identifier?: string | null;
  report_generation_failure_reason?: string | null;
  local_artifact_path?: string | null;
  generated_at?: number | null;
  file_size_bytes?: number | null;
}

export interface ReportAccessExecutionOutcome {
  request_kind: string;
  decision_type: string;
  detail_message?: string | null;
  exported_artifact_path?: string | null;
}

export interface ReportPreviewSnapshot {
  availability_status: ReportAvailabilityStatus;
  availability_status_name: string;
  report_identifier: string | null;
  completion_classification: string | null;
  report_generation_status: string | null;
  report_generation_failure_reason: string | null;
  generated_at: number | null;
  local_artifact_path: string | null;
  file_size_bytes: number | null;
  report_available_now: boolean;
  local_artifact_accessible_now: boolean;
}

export interface ReportArtifactPresentation {
  title: string;
  detail: string;
  actionGuidance: string;
}

export interface SetupScreenViewModel {
  licenseId: string;
  existingActivationId: string;
  submittingActivation: boolean;
  activationResult: ActivationResult | null;
  selectedProvider: SupportedProvider;
  providerApiKey: string;
  submittingProvider: boolean;
  providerOutcome: ProviderOutcome | null;
  providerState: ProviderState | null;
  selectedSeedDocument: File | null;
  requestedAgentCount: string;
  requestedRoundCount: string;
  maxTokenBudget: string;
  preflightSubmitDisabled: boolean;
  submittingPreflight: boolean;
  preflightOutcome: PreflightOutcome | null;
  preflightState: PreflightState | null;
  preflightSource: PreflightDetailsSource | null;
  downgradeAcceptanceOutcome: DowngradeAcceptanceOutcome | null;
  showDowngradeDisclosure: boolean;
  acceptingDowngrade: boolean;
  runtimePersonaState: RuntimePersonaState | null;
  runBoundedMemoryState: RunBoundedMemoryState | null;
  runModeratedRoundsState: RunModeratedRoundsState | null;
  runTokenDurationTrackingState: RunTokenDurationTrackingState | null;
  runSessionState: RunSessionState | null;
}

export interface RunScreenViewModel {
  runStartState: RunStartState | null;
  runSessionOutcome: RunSessionOutcome | null;
  runSessionState: RunSessionState | null;
  runLifecycleState: RunLifecycleState | null;
  runStatusState: RunStatusState | null;
  runEventContractState: RunEventContractState | null;
  runEventHistoryState: RunEventHistoryState | null;
  lifecycleBadgeText: string;
  statusBadgeText: string;
  resolutionBadgeText: string;
  statusContractLabel: string | null;
  eventContractLabel: string | null;
  monitoringBudgetCap: number;
  trackedTokenUnits: number | null;
  tokenProgress: number | null;
  currentRunId: string;
  chronologyItems: number;
  monitoringAlert: string | null;
}

export interface ReportsScreenViewModel {
  runFinalizationContractState: RunFinalizationContractState | null;
  runCleanupContractState: RunCleanupContractState | null;
  cleanupScopeMappingState: CleanupScopeMappingState | null;
  startupJanitorContractState: StartupJanitorContractState | null;
  reportOutputContractState: ReportOutputContractState | null;
  reportStorageBoundaryState: ReportStorageBoundaryState | null;
  reportAccessContractState: ReportAccessContractState | null;
  reportAccessMetadataState: ReportAccessMetadataState | null;
  runSessionState: RunSessionState | null;
  reportAccessExecutionOutcome: ReportAccessExecutionOutcome | null;
  reportAccessPreviewMode: ReportAccessPreviewMode;
  developerReportPreviewEnabled: boolean;
  effectiveAvailabilityStatus: ReportAvailabilityStatus;
  effectiveAvailabilityName: string;
  effectiveReportId: string | null;
  effectiveArtifactPath: string | null;
  effectiveGeneratedAt: number | null;
  effectiveArtifactSizeBytes: number | null;
  artifactPresentation: ReportArtifactPresentation;
  canOpenLocalReport: boolean;
  canExportLocalReport: boolean;
}

export interface DiagnosticsScreenViewModel {
  runtimePersonaState: RuntimePersonaState | null;
  runBoundedMemoryState: RunBoundedMemoryState | null;
  runModeratedRoundsState: RunModeratedRoundsState | null;
  runTokenDurationTrackingState: RunTokenDurationTrackingState | null;
  runLifecycleState: RunLifecycleState | null;
  runStatusState: RunStatusState | null;
  runtimePersonaContractLabel: string | null;
  boundedMemorySurfaceLabel: string | null;
  moderatedRoundsSurfaceLabel: string | null;
  trackingSurfaceLabel: string | null;
  lifecycleSurfaceLabel: string | null;
  statusContractLabel: string | null;
  sections: readonly string[];
}