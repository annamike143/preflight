import { invoke } from "@tauri-apps/api/core";
import { type ChangeEvent, type FormEvent, useEffect, useState } from "react";

import {
  type ActivationResult,
  type DowngradeAcceptanceOutcome,
  type PreflightOutcome,
  type PreflightState,
  type ProviderOutcome,
  type ProviderState,
  type RunStartState,
  type SetupScreenViewModel,
  type SupportedProvider
} from "../appViewModels";
import {
  positiveWholeNumber,
  preflightSubmissionStateAccessFailure,
  providerStateAccessFailure
} from "../appHelpers";
import type { SetupScreenActions } from "../screens/SetupScreen";
import { refreshTauriState, tauriErrorMessage } from "./tauriCommandHelpers";

type SetupViewModelSlice = Omit<
  SetupScreenViewModel,
  | "runtimePersonaState"
  | "runBoundedMemoryState"
  | "runModeratedRoundsState"
  | "runTokenDurationTrackingState"
  | "runSessionState"
>;

interface UseSetupScreenStateResult {
  viewModel: SetupViewModelSlice;
  actions: SetupScreenActions;
  preflightState: PreflightState | null;
  runStartState: RunStartState | null;
  refreshPreflightState: () => Promise<void>;
  refreshRunStartState: () => Promise<void>;
}

export function useSetupScreenState(): UseSetupScreenStateResult {
  const [licenseId, setLicenseId] = useState("");
  const [existingActivationId, setExistingActivationId] = useState("");
  const [submittingActivation, setSubmittingActivation] = useState(false);
  const [activationResult, setActivationResult] = useState<ActivationResult | null>(null);

  const [selectedProvider, setSelectedProvider] = useState<SupportedProvider>("open_ai");
  const [providerApiKey, setProviderApiKey] = useState("");
  const [submittingProvider, setSubmittingProvider] = useState(false);
  const [providerState, setProviderState] = useState<ProviderState | null>(null);
  const [providerOutcome, setProviderOutcome] = useState<ProviderOutcome | null>(null);

  const [selectedSeedDocument, setSelectedSeedDocument] = useState<File | null>(null);
  const [requestedAgentCount, setRequestedAgentCount] = useState("5");
  const [requestedRoundCount, setRequestedRoundCount] = useState("4");
  const [maxTokenBudget, setMaxTokenBudget] = useState("12000");
  const [submittingPreflight, setSubmittingPreflight] = useState(false);
  const [preflightOutcome, setPreflightOutcome] = useState<PreflightOutcome | null>(null);
  const [preflightState, setPreflightState] = useState<PreflightState | null>(null);
  const [runStartState, setRunStartState] = useState<RunStartState | null>(null);
  const [downgradeAcceptanceOutcome, setDowngradeAcceptanceOutcome] =
    useState<DowngradeAcceptanceOutcome | null>(null);
  const [acceptingDowngrade, setAcceptingDowngrade] = useState(false);

  const parsedRequestedAgentCount = positiveWholeNumber(requestedAgentCount);
  const parsedRequestedRoundCount = positiveWholeNumber(requestedRoundCount);
  const parsedMaxTokenBudget = positiveWholeNumber(maxTokenBudget);

  const preflightSubmitDisabled =
    submittingPreflight ||
    selectedSeedDocument === null ||
    parsedRequestedAgentCount === null ||
    parsedRequestedRoundCount === null ||
    parsedMaxTokenBudget === null;

  async function refreshProviderState() {
    const response = await refreshTauriState<ProviderState>(
      "provider_configuration_state",
      setProviderState
    );

    if (response?.selected_provider === "open_ai" || response?.selected_provider === "google") {
      setSelectedProvider(response.selected_provider);
    }
  }

  async function refreshPreflightState() {
    await refreshTauriState<PreflightState>("preflight_readiness_state", setPreflightState);
  }

  async function refreshRunStartState() {
    await refreshTauriState<RunStartState>("run_start_readiness_state", setRunStartState);
  }

  useEffect(() => {
    void Promise.all([refreshProviderState(), refreshPreflightState(), refreshRunStartState()]);
  }, []);

  async function handleActivationSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmittingActivation(true);
    setActivationResult(null);

    try {
      const response = await invoke<ActivationResult>("submit_device_activation", {
        request: {
          license_id: licenseId.trim(),
          existing_activation_id: existingActivationId.trim() || null
        }
      });

      setActivationResult(response);
    } catch (error) {
      setActivationResult({
        decision_type: "technical_failure",
        denial_reason: null,
        technical_failure_reason: "backend_failure",
        detail_message: tauriErrorMessage(error)
      });
    } finally {
      setSubmittingActivation(false);
      await Promise.all([refreshPreflightState(), refreshRunStartState()]);
    }
  }

  async function handleProviderSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmittingProvider(true);
    setProviderOutcome(null);

    try {
      const response = await invoke<ProviderOutcome>("submit_provider_configuration", {
        request: {
          selected_provider: selectedProvider,
          api_key: providerApiKey.trim()
        }
      });

      setProviderOutcome(response);
      setProviderState(response.configuration_state ?? providerStateAccessFailure(response.detail_message ?? undefined));
    } catch (error) {
      const detailMessage = tauriErrorMessage(error);
      const fallbackState = providerStateAccessFailure(detailMessage);

      setProviderOutcome({
        decision_type: "failure",
        failure_reason: "secure_storage_failure",
        configuration_state: fallbackState,
        detail_message: detailMessage
      });
      setProviderState(fallbackState);
    } finally {
      setProviderApiKey("");
      setSubmittingProvider(false);
      await Promise.all([refreshPreflightState(), refreshRunStartState()]);
    }
  }

  function handleSeedDocumentChange(event: ChangeEvent<HTMLInputElement>) {
    const nextFile = event.target.files?.[0] ?? null;
    setSelectedSeedDocument(nextFile);
    setPreflightOutcome(null);
    setDowngradeAcceptanceOutcome(null);
  }

  async function handlePreflightSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (
      !selectedSeedDocument ||
      parsedRequestedAgentCount === null ||
      parsedRequestedRoundCount === null ||
      parsedMaxTokenBudget === null
    ) {
      const detailMessage = "A single seed document plus positive requested-plan values are required";

      setPreflightOutcome({
        decision_type: "technical_failure",
        rejection_reason: null,
        technical_failure_reason: "malformed_input",
        submission_state: preflightSubmissionStateAccessFailure(detailMessage),
        detail_message: detailMessage
      });
      return;
    }

    setSubmittingPreflight(true);
    setPreflightOutcome(null);

    try {
      const bytes = Array.from(new Uint8Array(await selectedSeedDocument.arrayBuffer()));
      const response = await invoke<PreflightOutcome>("submit_preflight", {
        request: {
          seed_document: {
            filename: selectedSeedDocument.name,
            mime_type: selectedSeedDocument.type || null,
            size_bytes: selectedSeedDocument.size,
            bytes
          },
          requested_agent_count: parsedRequestedAgentCount,
          requested_round_count: parsedRequestedRoundCount,
          max_token_budget: parsedMaxTokenBudget
        }
      });

      setPreflightOutcome(response);
    } catch (error) {
      const detailMessage = tauriErrorMessage(error);

      setPreflightOutcome({
        decision_type: "technical_failure",
        rejection_reason: null,
        technical_failure_reason: "state_access_failure",
        submission_state: preflightSubmissionStateAccessFailure(detailMessage),
        detail_message: detailMessage
      });
    } finally {
      setSubmittingPreflight(false);
      await Promise.all([refreshPreflightState(), refreshRunStartState()]);
    }
  }

  async function handleAcceptDowngrade() {
    setAcceptingDowngrade(true);
    setDowngradeAcceptanceOutcome(null);

    try {
      const response = await invoke<DowngradeAcceptanceOutcome>("accept_preflight_downgrade");
      setDowngradeAcceptanceOutcome(response);
    } catch (error) {
      setDowngradeAcceptanceOutcome({
        acceptance_recorded: false,
        detail_message: tauriErrorMessage(error)
      });
    } finally {
      setAcceptingDowngrade(false);
      await Promise.all([refreshPreflightState(), refreshRunStartState()]);
    }
  }

  const livePreflightDetailsSource = preflightState
    ? {
        valid_document_in_pending_configuration:
          preflightState.valid_document_in_pending_configuration ?? false,
        requested_plan_budget_feasible: preflightState.requested_plan_budget_feasible ?? null,
        selected_seed_document: preflightState.selected_seed_document ?? null,
        requested_plan: preflightState.requested_plan ?? null,
        effective_plan: preflightState.effective_plan ?? null
      }
    : null;

  const viewModel: SetupViewModelSlice = {
    licenseId,
    existingActivationId,
    submittingActivation,
    activationResult,
    selectedProvider,
    providerApiKey,
    submittingProvider,
    providerOutcome,
    providerState,
    selectedSeedDocument,
    requestedAgentCount,
    requestedRoundCount,
    maxTokenBudget,
    preflightSubmitDisabled,
    submittingPreflight,
    preflightOutcome,
    preflightState,
    preflightSource: preflightOutcome?.submission_state ?? livePreflightDetailsSource,
    downgradeAcceptanceOutcome,
    showDowngradeDisclosure: Boolean(
      preflightState?.downgrade_acceptance_required && preflightState?.downgrade_warning
    ),
    acceptingDowngrade
  };

  const actions: SetupScreenActions = {
    onLicenseIdChange: setLicenseId,
    onExistingActivationIdChange: setExistingActivationId,
    onActivationSubmit: handleActivationSubmit,
    onSelectedProviderChange: setSelectedProvider,
    onProviderApiKeyChange: setProviderApiKey,
    onProviderSubmit: handleProviderSubmit,
    onRequestedAgentCountChange: setRequestedAgentCount,
    onRequestedRoundCountChange: setRequestedRoundCount,
    onMaxTokenBudgetChange: setMaxTokenBudget,
    onSeedDocumentChange: handleSeedDocumentChange,
    onPreflightSubmit: handlePreflightSubmit,
    onAcceptDowngrade: () => {
      void handleAcceptDowngrade();
    }
  };

  return {
    viewModel,
    actions,
    preflightState,
    runStartState,
    refreshPreflightState,
    refreshRunStartState
  };
}