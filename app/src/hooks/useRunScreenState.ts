import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

import {
  type AppScreen,
  type RunBoundedMemoryState,
  type RunEventContractState,
  type RunEventHistoryState,
  type RunLifecycleState,
  type RunModeratedRoundsState,
  type RunScreenViewModel,
  type RunSessionOutcome,
  type RunSessionState,
  type RunStartState,
  type RunStatusState,
  type RunTokenDurationTrackingState,
  type RuntimePersonaState
} from "../appViewModels";
import { label, progressPercentage, runSessionStateAccessFailure } from "../appHelpers";
import type { RunScreenActions } from "../screens/RunScreen";
import { refreshTauriState, tauriErrorMessage } from "./tauriCommandHelpers";

interface UseRunScreenStateOptions {
  activeScreen: AppScreen;
  runStartState: RunStartState | null;
  planningTokenBudget: number;
  onNavigateToRun: () => void;
  refreshSetupReadiness: () => Promise<void>;
}

interface UseRunScreenStateResult {
  viewModel: RunScreenViewModel;
  actions: RunScreenActions;
  runtimePersonaState: RuntimePersonaState | null;
  runBoundedMemoryState: RunBoundedMemoryState | null;
  runModeratedRoundsState: RunModeratedRoundsState | null;
  runTokenDurationTrackingState: RunTokenDurationTrackingState | null;
  runLifecycleState: RunLifecycleState | null;
  runStatusState: RunStatusState | null;
  runSessionState: RunSessionState | null;
  startingRunSession: boolean;
  cancelingRunSession: boolean;
  requestingControlledShutdown: boolean;
  clearRunSessionOutcome: () => void;
}

export function useRunScreenState({
  activeScreen,
  runStartState,
  planningTokenBudget,
  onNavigateToRun,
  refreshSetupReadiness
}: UseRunScreenStateOptions): UseRunScreenStateResult {
  const [runtimePersonaState, setRuntimePersonaState] = useState<RuntimePersonaState | null>(null);
  const [runBoundedMemoryState, setRunBoundedMemoryState] =
    useState<RunBoundedMemoryState | null>(null);
  const [runModeratedRoundsState, setRunModeratedRoundsState] =
    useState<RunModeratedRoundsState | null>(null);
  const [runTokenDurationTrackingState, setRunTokenDurationTrackingState] =
    useState<RunTokenDurationTrackingState | null>(null);
  const [runLifecycleState, setRunLifecycleState] = useState<RunLifecycleState | null>(null);
  const [runStatusState, setRunStatusState] = useState<RunStatusState | null>(null);
  const [runEventContractState, setRunEventContractState] =
    useState<RunEventContractState | null>(null);
  const [runEventHistoryState, setRunEventHistoryState] =
    useState<RunEventHistoryState | null>(null);
  const [runSessionState, setRunSessionState] = useState<RunSessionState | null>(null);
  const [runSessionOutcome, setRunSessionOutcome] = useState<RunSessionOutcome | null>(null);
  const [startingRunSession, setStartingRunSession] = useState(false);
  const [cancelingRunSession, setCancelingRunSession] = useState(false);
  const [requestingControlledShutdown, setRequestingControlledShutdown] = useState(false);

  async function refreshRuntimePersonaState() {
    await refreshTauriState<RuntimePersonaState>(
      "runtime_persona_contract_state",
      setRuntimePersonaState
    );
  }

  async function refreshRunBoundedMemoryState() {
    await refreshTauriState<RunBoundedMemoryState>("run_bounded_memory_state", setRunBoundedMemoryState);
  }

  async function refreshRunModeratedRoundsState() {
    await refreshTauriState<RunModeratedRoundsState>(
      "run_moderated_rounds_state",
      setRunModeratedRoundsState
    );
  }

  async function refreshRunTokenDurationTrackingState() {
    await refreshTauriState<RunTokenDurationTrackingState>(
      "run_token_duration_tracking_state",
      setRunTokenDurationTrackingState
    );
  }

  async function refreshRunLifecycleState() {
    await refreshTauriState<RunLifecycleState>("run_lifecycle_state", setRunLifecycleState);
  }

  async function refreshRunStatusState() {
    await refreshTauriState<RunStatusState>("run_status_state", setRunStatusState);
  }

  async function refreshRunEventContractState() {
    await refreshTauriState<RunEventContractState>("run_event_contract_state", setRunEventContractState);
  }

  async function refreshRunEventHistory() {
    await refreshTauriState<RunEventHistoryState>("run_event_history", setRunEventHistoryState, {
      request: {
        max_items: 256
      }
    });
  }

  async function refreshRunSessionState() {
    await refreshTauriState<RunSessionState>("run_session_state", setRunSessionState);
  }

  useEffect(() => {
    void Promise.all([
      refreshRuntimePersonaState(),
      refreshRunBoundedMemoryState(),
      refreshRunModeratedRoundsState(),
      refreshRunTokenDurationTrackingState(),
      refreshRunSessionState()
    ]);
  }, []);

  useEffect(() => {
    if (activeScreen !== "run") {
      return;
    }

    void Promise.all([
      refreshRunLifecycleState(),
      refreshRunStatusState(),
      refreshRunEventContractState(),
      refreshRunEventHistory(),
      refreshRunSessionState()
    ]);
  }, [activeScreen]);

  async function startRunSession() {
    onNavigateToRun();
    setStartingRunSession(true);
    setRunSessionOutcome(null);

    try {
      const response = await invoke<RunSessionOutcome>("start_run_session");
      setRunSessionOutcome(response);
      setRunSessionState(
        response.run_session_state ??
          runSessionStateAccessFailure(response.detail_message ?? "Run session state unavailable")
      );
    } catch (error) {
      const detailMessage = tauriErrorMessage(error);
      const fallbackState = runSessionStateAccessFailure(detailMessage);

      setRunSessionOutcome({
        decision_type: "technical_failure",
        run_session_state: fallbackState,
        technical_failure_reason: "state_access_failure",
        detail_message: detailMessage
      });
      setRunSessionState(fallbackState);
    } finally {
      setStartingRunSession(false);
      await Promise.all([
        refreshRunLifecycleState(),
        refreshRunStatusState(),
        refreshRunEventContractState(),
        refreshRunEventHistory(),
        refreshRunSessionState(),
        refreshSetupReadiness()
      ]);
    }
  }

  async function cancelRunSession() {
    setCancelingRunSession(true);

    try {
      await invoke("cancel_run_session");
    } catch {
      // The refreshed shell state remains authoritative.
    } finally {
      setCancelingRunSession(false);
      await Promise.all([
        refreshRunLifecycleState(),
        refreshRunStatusState(),
        refreshRunEventHistory(),
        refreshRunSessionState()
      ]);
    }
  }

  async function requestControlledShutdown() {
    setRequestingControlledShutdown(true);

    try {
      await invoke("request_controlled_shutdown");
    } catch {
      // The refreshed shell state remains authoritative.
    } finally {
      setRequestingControlledShutdown(false);
      await Promise.all([
        refreshRunLifecycleState(),
        refreshRunStatusState(),
        refreshRunEventHistory(),
        refreshRunSessionState()
      ]);
    }
  }

  const monitoringBudgetCap = planningTokenBudget > 0 ? planningTokenBudget : 12000;
  const trackedTokenUnits =
    runStatusState?.tracked_total_token_units ?? runSessionState?.tracked_total_token_units ?? null;

  const viewModel: RunScreenViewModel = {
    runStartState,
    runSessionOutcome,
    runSessionState,
    runLifecycleState,
    runStatusState,
    runEventContractState,
    runEventHistoryState,
    lifecycleBadgeText: label(
      runLifecycleState?.current_state_name ?? runLifecycleState?.current_state ?? "Unknown"
    ),
    statusBadgeText: label(
      runStatusState?.current_lifecycle_state_name ?? runStatusState?.current_lifecycle_state ?? "Unknown"
    ),
    resolutionBadgeText: label(
      runSessionState?.execution_completion_state ??
        runSessionState?.lifecycle_override ??
        runStatusState?.current_lifecycle_state_name ??
        runLifecycleState?.current_state_name ??
        "Unknown"
    ),
    statusContractLabel: runStatusState?.contract_name ? label(runStatusState.contract_name) : null,
    eventContractLabel: runEventContractState?.contract_name ? label(runEventContractState.contract_name) : null,
    monitoringBudgetCap,
    trackedTokenUnits,
    tokenProgress: progressPercentage(trackedTokenUnits, monitoringBudgetCap),
    currentRunId: runStatusState?.current_run_id ?? runSessionState?.run_id ?? "Unavailable",
    chronologyItems: runEventHistoryState?.returned_item_count ?? 0,
    monitoringAlert:
      runStatusState?.current_lifecycle_state === "completed_budget_capped" ||
      runLifecycleState?.current_state === "completed_budget_capped"
        ? `This run resolved as CompletedBudgetCapped after the ${label(
            runSessionState?.runtime_cap_reason ?? "token_budget"
          )} truncated the planned round schedule.`
        : null
  };

  const actions: RunScreenActions = {
    onStartRunSession: () => {
      void startRunSession();
    },
    onCancelRunSession: () => {
      void cancelRunSession();
    },
    onRequestControlledShutdown: () => {
      void requestControlledShutdown();
    }
  };

  return {
    viewModel,
    actions,
    runtimePersonaState,
    runBoundedMemoryState,
    runModeratedRoundsState,
    runTokenDurationTrackingState,
    runLifecycleState,
    runStatusState,
    runSessionState,
    startingRunSession,
    cancelingRunSession,
    requestingControlledShutdown,
    clearRunSessionOutcome: () => {
      setRunSessionOutcome(null);
    }
  };
}