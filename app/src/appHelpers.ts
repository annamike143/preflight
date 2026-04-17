import type {
  EventPayload,
  PreflightSubmissionState,
  ProviderState,
  ReportAccessExecutionOutcome,
  ReportAccessRequestKind,
  ReportAccessPreviewMode,
  ReportArtifactPresentation,
  ReportAvailabilityStatus,
  ReportPreviewSnapshot,
  RunEvent,
  RunSessionState
} from "./appViewModels";

export function label(value: unknown): string {
  if (value === null || value === undefined || value === "") {
    return "Not Available";
  }

  const normalized = String(value)
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/[_-]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();

  return normalized
    .split(" ")
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1).toLowerCase())
    .join(" ");
}

export function boolText(value: unknown): string {
  return value ? "yes" : "no";
}

export function formatList(values: unknown[] | null | undefined): string {
  if (!values || values.length === 0) {
    return "None";
  }

  return values.map((value) => label(value)).join(", ");
}

export function positiveWholeNumber(value: string): number | null {
  const parsed = Number.parseInt(value, 10);
  return Number.isInteger(parsed) && parsed > 0 ? parsed : null;
}

export function progressPercentage(
  current: number | null | undefined,
  total: number | null | undefined
): number | null {
  if (typeof current !== "number" || typeof total !== "number" || total <= 0) {
    return null;
  }

  return (current / total) * 100;
}

export function payloadText(payload: EventPayload | null | undefined, key: string): string | null {
  const value = payload?.[key];
  return typeof value === "string" && value.length > 0 ? value : null;
}

export function payloadNumber(payload: EventPayload | null | undefined, key: string): number | null {
  const value = payload?.[key];
  return typeof value === "number" ? value : null;
}

export function payloadStringArray(
  payload: EventPayload | null | undefined,
  key: string
): string[] {
  const value = payload?.[key];
  return Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : [];
}

export function providerStateAccessFailure(
  detailMessage = "Provider configuration state unavailable"
): ProviderState {
  return {
    current_status: "error",
    current_detail: detailMessage,
    configuration_present: false,
    selected_provider: "open_ai"
  };
}

export function preflightSubmissionStateAccessFailure(
  detailMessage: string
): PreflightSubmissionState {
  return {
    status: "not_started",
    valid_document_in_pending_configuration: false,
    selected_seed_document: null,
    requested_plan: null,
    effective_plan: null,
    requested_plan_budget_feasible: null,
    detail_message: detailMessage
  };
}

export function runSessionStateAccessFailure(detailMessage: string): RunSessionState {
  return {
    current_state_class: "technical_failure",
    current_state_name: "technical_failure",
    run_id: null,
    session_id: null,
    workspace_allocated: false,
    execution_seed_staged: false,
    participant_persona_count: null,
    participant_persona_labels: [],
    perspective_coverage: [],
    completed_round_count: null,
    speaker_counts_per_round: [],
    tracked_total_token_units: null,
    token_duration_tracking_path: null,
    execution_completion_state: null,
    runtime_cap_reason: null,
    controlled_finalization_performed: false,
    transcript_archive_separated: false,
    cleanup_marker: null,
    detail_message: detailMessage,
    lifecycle_override: null
  };
}

export function describeReportArtifact(
  availabilityStatus: ReportAvailabilityStatus,
  failureReason: string | null | undefined
): ReportArtifactPresentation {
  switch (availabilityStatus) {
    case "available":
      return {
        title: "Report ready",
        detail: "The durable local PDF artifact is currently available to the shell-owned access boundary.",
        actionGuidance: "Open or export the durable PDF artifact."
      };
    case "missing_artifact":
      return {
        title: "Local report PDF",
        detail: "Shell metadata says a report should exist, but the durable artifact is currently missing.",
        actionGuidance: "Regenerate or inspect the durable artifact domain before retrying local access."
      };
    case "inaccessible_artifact":
      return {
        title: "Local report PDF",
        detail: "A durable report artifact exists, but the shell cannot access it from the current boundary.",
        actionGuidance: "Use the shell-owned access path to verify artifact permissions and current availability truth."
      };
    default:
      return {
        title: "Local report PDF",
        detail: failureReason
          ? `No durable local PDF artifact is available yet because report generation failed: ${label(failureReason)}.`
          : "No durable local PDF artifact is available yet.",
        actionGuidance: "Wait for finalization and report-generation success before attempting local access."
      };
  }
}

export function buildReportAccessPreviewSnapshot(
  mode: Exclude<ReportAccessPreviewMode, "live">
): ReportPreviewSnapshot {
  switch (mode) {
    case "available":
      return {
        availability_status: "available",
        availability_status_name: "ReportAvailable",
        report_identifier: "report-run-042",
        completion_classification: "completed",
        report_generation_status: "succeeded",
        report_generation_failure_reason: null,
        generated_at: 1713200002000,
        local_artifact_path: "C:/reports/report-run-042.pdf",
        file_size_bytes: 248320,
        report_available_now: true,
        local_artifact_accessible_now: true
      };
    case "missing_artifact":
      return {
        availability_status: "missing_artifact",
        availability_status_name: "ReportMissingArtifact",
        report_identifier: "report-run-042",
        completion_classification: "completed",
        report_generation_status: "succeeded",
        report_generation_failure_reason: null,
        generated_at: 1713200002000,
        local_artifact_path: null,
        file_size_bytes: null,
        report_available_now: false,
        local_artifact_accessible_now: false
      };
    case "inaccessible_artifact":
      return {
        availability_status: "inaccessible_artifact",
        availability_status_name: "ReportInaccessibleArtifact",
        report_identifier: "report-run-042",
        completion_classification: "completed",
        report_generation_status: "succeeded",
        report_generation_failure_reason: null,
        generated_at: 1713200002000,
        local_artifact_path: "C:/reports/report-run-042.pdf",
        file_size_bytes: 248320,
        report_available_now: true,
        local_artifact_accessible_now: false
      };
    default:
      return {
        availability_status: "unavailable",
        availability_status_name: "ReportUnavailable",
        report_identifier: null,
        completion_classification: null,
        report_generation_status: null,
        report_generation_failure_reason: null,
        generated_at: null,
        local_artifact_path: null,
        file_size_bytes: null,
        report_available_now: false,
        local_artifact_accessible_now: false
      };
  }
}

export function buildPreviewReportAccessExecutionOutcome(
  mode: Exclude<ReportAccessPreviewMode, "live">,
  requestKind: ReportAccessRequestKind
): ReportAccessExecutionOutcome {
  if (mode === "available") {
    return {
      request_kind: requestKind,
      decision_type: "succeeded",
      detail_message:
        requestKind === "export_local_pdf"
          ? "Preview only: the shell would export a copy of the durable PDF artifact."
          : "Preview only: the shell would open the durable PDF artifact.",
      exported_artifact_path:
        requestKind === "export_local_pdf"
          ? "C:/Users/mikes/Downloads/Miro Fish Reports/report-run-042.pdf"
          : null
    };
  }

  return {
    request_kind: requestKind,
    decision_type: "failed",
    detail_message: `Preview only: ${label(mode)} would block this request from the shell-owned access boundary.`,
    exported_artifact_path: null
  };
}

export function summarizeRunEvent(event: RunEvent): string {
  const payload = event.payload ?? null;

  switch (event.event_type) {
    case "run_queued": {
      const workspaceReference = payloadText(payload, "workspace_reference");
      return workspaceReference
        ? `Fresh run workspace is allocated at ${workspaceReference}.`
        : "Fresh run workspace allocation was recorded by the shell.";
    }
    case "round_started": {
      const roundNumber = payloadNumber(payload, "round_number");
      const speakerLabels = payloadStringArray(payload, "selected_speaker_labels");
      return roundNumber !== null && speakerLabels.length > 0
        ? `Round ${roundNumber} started. Speakers: ${speakerLabels.join(", ")}.`
        : "A moderated round started under shell-owned chronology alignment.";
    }
    case "budget_usage_updated": {
      const tokenUnits = payloadNumber(payload, "cumulative_total_token_units");
      const durationMs = payloadNumber(payload, "cumulative_duration_ms");
      if (tokenUnits !== null && durationMs !== null) {
        return `${tokenUnits} token units tracked over ${durationMs} ms.`;
      }

      return "Run-local token and duration tracking was updated.";
    }
    case "finalization_started": {
      const capReason = payloadText(payload, "cap_reason");
      return capReason
        ? `Constrained finalization started after the ${label(capReason)} truncated the planned run.`
        : "Constrained finalization started from currently available material.";
    }
    case "finalization_completed": {
      const capReason = payloadText(payload, "cap_reason");
      return capReason
        ? `Constrained finalization completed after the ${label(capReason)} truncated the planned run.`
        : "Constrained finalization completed from currently available material.";
    }
    case "run_state_changed": {
      const currentState = payloadText(payload, "current_state");
      return currentState
        ? `Shell-owned lifecycle now projects ${currentState}.`
        : "The shell published a new canonical lifecycle state.";
    }
    case "failure_recorded": {
      const failureReason = payloadText(payload, "failure_reason");
      return failureReason
        ? `A runtime failure was recorded: ${label(failureReason)}.`
        : "A runtime failure was recorded.";
    }
    default:
      return event.event_type ? `${label(event.event_type)} was recorded.` : "A run event was recorded.";
  }
}