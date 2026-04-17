import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

import {
  type AppScreen,
  type CleanupScopeMappingState,
  type ReportAccessContractState,
  type ReportAccessExecutionOutcome,
  type ReportAccessMetadataState,
  type ReportAccessPreviewMode,
  type ReportAccessRequestKind,
  type ReportOutputContractState,
  type ReportsScreenViewModel,
  type ReportStorageBoundaryState,
  type RunCleanupContractState,
  type RunFinalizationContractState,
  type RunSessionState,
  type StartupJanitorContractState
} from "../appViewModels";
import {
  buildPreviewReportAccessExecutionOutcome,
  buildReportAccessPreviewSnapshot,
  describeReportArtifact
} from "../appHelpers";
import type { ReportsScreenActions } from "../screens/ReportsScreen";
import { refreshTauriState, tauriErrorMessage } from "./tauriCommandHelpers";

interface UseReportsScreenStateOptions {
  activeScreen: AppScreen;
  runSessionState: RunSessionState | null;
}

interface UseReportsScreenStateResult {
  viewModel: ReportsScreenViewModel;
  actions: ReportsScreenActions;
}

export function useReportsScreenState({
  activeScreen,
  runSessionState
}: UseReportsScreenStateOptions): UseReportsScreenStateResult {
  const [runFinalizationContractState, setRunFinalizationContractState] =
    useState<RunFinalizationContractState | null>(null);
  const [runCleanupContractState, setRunCleanupContractState] =
    useState<RunCleanupContractState | null>(null);
  const [cleanupScopeMappingState, setCleanupScopeMappingState] =
    useState<CleanupScopeMappingState | null>(null);
  const [startupJanitorContractState, setStartupJanitorContractState] =
    useState<StartupJanitorContractState | null>(null);
  const [reportOutputContractState, setReportOutputContractState] =
    useState<ReportOutputContractState | null>(null);
  const [reportStorageBoundaryState, setReportStorageBoundaryState] =
    useState<ReportStorageBoundaryState | null>(null);
  const [reportAccessContractState, setReportAccessContractState] =
    useState<ReportAccessContractState | null>(null);
  const [reportAccessMetadataState, setReportAccessMetadataState] =
    useState<ReportAccessMetadataState | null>(null);
  const [reportAccessExecutionOutcome, setReportAccessExecutionOutcome] =
    useState<ReportAccessExecutionOutcome | null>(null);
  const [reportAccessPreviewMode, setReportAccessPreviewMode] =
    useState<ReportAccessPreviewMode>("live");
  const [executingReportAccessRequest, setExecutingReportAccessRequest] =
    useState<ReportAccessRequestKind | null>(null);

  async function refreshRunFinalizationContractState() {
    await refreshTauriState<RunFinalizationContractState>(
      "run_finalization_contract_state",
      setRunFinalizationContractState
    );
  }

  async function refreshRunCleanupContractState() {
    await refreshTauriState<RunCleanupContractState>(
      "run_cleanup_contract_state",
      setRunCleanupContractState
    );
  }

  async function refreshCleanupScopeMappingState() {
    await refreshTauriState<CleanupScopeMappingState>(
      "cleanup_scope_mapping_state",
      setCleanupScopeMappingState
    );
  }

  async function refreshStartupJanitorContractState() {
    await refreshTauriState<StartupJanitorContractState>(
      "startup_janitor_contract_state",
      setStartupJanitorContractState
    );
  }

  async function refreshReportOutputContractState() {
    await refreshTauriState<ReportOutputContractState>(
      "report_output_contract_state",
      setReportOutputContractState
    );
  }

  async function refreshReportStorageBoundaryState() {
    await refreshTauriState<ReportStorageBoundaryState>(
      "report_storage_boundary_state",
      setReportStorageBoundaryState
    );
  }

  async function refreshReportAccessContractState() {
    await refreshTauriState<ReportAccessContractState>(
      "report_access_contract_state",
      setReportAccessContractState
    );
  }

  async function refreshReportAccessMetadataState() {
    await refreshTauriState<ReportAccessMetadataState>(
      "report_access_metadata_state",
      setReportAccessMetadataState
    );
  }

  useEffect(() => {
    if (activeScreen !== "reports") {
      return;
    }

    void Promise.all([
      refreshRunFinalizationContractState(),
      refreshRunCleanupContractState(),
      refreshCleanupScopeMappingState(),
      refreshStartupJanitorContractState(),
      refreshReportOutputContractState(),
      refreshReportStorageBoundaryState(),
      refreshReportAccessContractState(),
      refreshReportAccessMetadataState()
    ]);
  }, [activeScreen]);

  useEffect(() => {
    if (
      !runSessionState?.report_identifier &&
      !runSessionState?.durable_report_artifact_path &&
      !runSessionState?.report_generated_at &&
      !runSessionState?.report_generation_failure_reason
    ) {
      return;
    }

    void Promise.all([refreshReportAccessContractState(), refreshReportAccessMetadataState()]);
  }, [
    runSessionState?.report_identifier,
    runSessionState?.durable_report_artifact_path,
    runSessionState?.report_generated_at,
    runSessionState?.report_generation_failure_reason
  ]);

  async function openLocalReport() {
    if (reportAccessPreviewMode !== "live") {
      setReportAccessExecutionOutcome(
        buildPreviewReportAccessExecutionOutcome(reportAccessPreviewMode, "open_local_pdf")
      );
      return;
    }

    setExecutingReportAccessRequest("open_local_pdf");
    setReportAccessExecutionOutcome(null);

    try {
      const response = await invoke<ReportAccessExecutionOutcome>("open_local_report_pdf");
      setReportAccessExecutionOutcome(response);
    } catch (error) {
      setReportAccessExecutionOutcome({
        request_kind: "open_local_pdf",
        decision_type: "failed",
        detail_message: tauriErrorMessage(error),
        exported_artifact_path: null
      });
    } finally {
      setExecutingReportAccessRequest(null);
      await Promise.all([refreshReportAccessContractState(), refreshReportAccessMetadataState()]);
    }
  }

  async function exportLocalReport() {
    if (reportAccessPreviewMode !== "live") {
      setReportAccessExecutionOutcome(
        buildPreviewReportAccessExecutionOutcome(reportAccessPreviewMode, "export_local_pdf")
      );
      return;
    }

    setExecutingReportAccessRequest("export_local_pdf");
    setReportAccessExecutionOutcome(null);

    try {
      const response = await invoke<ReportAccessExecutionOutcome>("export_local_report_pdf");
      setReportAccessExecutionOutcome(response);
    } catch (error) {
      setReportAccessExecutionOutcome({
        request_kind: "export_local_pdf",
        decision_type: "failed",
        detail_message: tauriErrorMessage(error),
        exported_artifact_path: null
      });
    } finally {
      setExecutingReportAccessRequest(null);
      await Promise.all([refreshReportAccessContractState(), refreshReportAccessMetadataState()]);
    }
  }

  const developerReportPreviewEnabled = import.meta.env.DEV || import.meta.env.MODE === "test";
  const previewSnapshot =
    reportAccessPreviewMode !== "live" ? buildReportAccessPreviewSnapshot(reportAccessPreviewMode) : null;
  const effectiveAvailabilityStatus =
    previewSnapshot?.availability_status ??
    reportAccessMetadataState?.availability_status ??
    reportAccessContractState?.current_availability_status ??
    "unavailable";
  const effectiveAvailabilityName =
    previewSnapshot?.availability_status_name ??
    reportAccessMetadataState?.availability_status_name ??
    reportAccessContractState?.current_availability_status_name ??
    "ReportUnavailable";
  const effectiveReportId =
    previewSnapshot?.report_identifier ?? reportAccessMetadataState?.report_identifier ?? runSessionState?.report_identifier ?? null;
  const effectiveArtifactPath =
    previewSnapshot?.local_artifact_path ??
    reportAccessMetadataState?.local_artifact_path ??
    runSessionState?.durable_report_artifact_path ??
    null;
  const effectiveGeneratedAt =
    previewSnapshot?.generated_at ?? reportAccessMetadataState?.generated_at ?? runSessionState?.report_generated_at ?? null;
  const effectiveArtifactSizeBytes =
    previewSnapshot?.file_size_bytes ?? reportAccessMetadataState?.file_size_bytes ?? null;
  const effectiveFailureReason =
    previewSnapshot?.report_generation_failure_reason ??
    reportAccessMetadataState?.report_generation_failure_reason ??
    runSessionState?.report_generation_failure_reason ??
    null;
  const localArtifactAccessible =
    previewSnapshot?.local_artifact_accessible_now ??
    reportAccessContractState?.local_artifact_accessible_now ??
    false;

  const viewModel: ReportsScreenViewModel = {
    runFinalizationContractState,
    runCleanupContractState,
    cleanupScopeMappingState,
    startupJanitorContractState,
    reportOutputContractState,
    reportStorageBoundaryState,
    reportAccessContractState,
    reportAccessMetadataState,
    runSessionState,
    reportAccessExecutionOutcome,
    reportAccessPreviewMode,
    developerReportPreviewEnabled,
    effectiveAvailabilityStatus,
    effectiveAvailabilityName,
    effectiveReportId,
    effectiveArtifactPath,
    effectiveGeneratedAt,
    effectiveArtifactSizeBytes,
    artifactPresentation: describeReportArtifact(effectiveAvailabilityStatus, effectiveFailureReason),
    canOpenLocalReport: Boolean(localArtifactAccessible && executingReportAccessRequest === null),
    canExportLocalReport: Boolean(localArtifactAccessible && executingReportAccessRequest === null)
  };

  const actions: ReportsScreenActions = {
    onOpenLocalReport: () => {
      void openLocalReport();
    },
    onExportLocalReport: () => {
      void exportLocalReport();
    },
    onReportAccessPreviewModeChange: setReportAccessPreviewMode
  };

  return {
    viewModel,
    actions
  };
}