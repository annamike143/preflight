import type { ChangeEvent, FormEvent } from "react";

import type { SetupScreenViewModel, SupportedProvider } from "../appViewModels";
import { boolText, label } from "../appHelpers";
import { buttonStyle, controlStyle } from "./sharedStyles";
import {
  CurrentPreflightState,
  FieldText,
  Panel,
  PreflightDetails,
  RunSessionDetails
} from "./shared";

export interface SetupScreenActions {
  onLicenseIdChange: (value: string) => void;
  onExistingActivationIdChange: (value: string) => void;
  onActivationSubmit: (event: FormEvent<HTMLFormElement>) => void;
  onSelectedProviderChange: (value: SupportedProvider) => void;
  onProviderApiKeyChange: (value: string) => void;
  onProviderSubmit: (event: FormEvent<HTMLFormElement>) => void;
  onRequestedAgentCountChange: (value: string) => void;
  onRequestedRoundCountChange: (value: string) => void;
  onMaxTokenBudgetChange: (value: string) => void;
  onSeedDocumentChange: (event: ChangeEvent<HTMLInputElement>) => void;
  onPreflightSubmit: (event: FormEvent<HTMLFormElement>) => void;
  onAcceptDowngrade: () => void;
}

interface SetupScreenProps {
  viewModel: SetupScreenViewModel;
  actions: SetupScreenActions;
}

export function SetupScreen({ viewModel, actions }: SetupScreenProps) {
  return (
    <div style={{ display: "grid", gap: 20 }}>
      <div
        style={{
          display: "grid",
          gap: 20,
          gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))",
          alignItems: "start"
        }}
      >
        <Panel title="Activation request">
          <form onSubmit={actions.onActivationSubmit} style={{ display: "grid", gap: 10 }}>
            <label style={{ display: "grid", gap: 4 }}>
              <span>License ID</span>
              <input
                value={viewModel.licenseId}
                onChange={(event) => actions.onLicenseIdChange(event.target.value)}
                style={controlStyle}
              />
            </label>
            <label style={{ display: "grid", gap: 4 }}>
              <span>Existing activation ID</span>
              <input
                value={viewModel.existingActivationId}
                onChange={(event) => actions.onExistingActivationIdChange(event.target.value)}
                style={controlStyle}
              />
            </label>
            <button
              type="submit"
              disabled={viewModel.submittingActivation}
              style={{ ...buttonStyle("primary", viewModel.submittingActivation), justifySelf: "start" }}
            >
              {viewModel.submittingActivation ? "Submitting activation request" : "Submit activation request"}
            </button>
          </form>
          {viewModel.activationResult ? (
            <div style={{ display: "grid", gap: 8 }}>
              <h3 style={{ margin: 0, fontSize: "0.98rem" }}>Activation result</h3>
              <FieldText labelText="Decision type" value={label(viewModel.activationResult.decision_type)} />
              {viewModel.activationResult.denial_reason ? (
                <FieldText labelText="Denial reason" value={label(viewModel.activationResult.denial_reason)} />
              ) : null}
              {viewModel.activationResult.technical_failure_reason ? (
                <FieldText
                  labelText="Technical failure reason"
                  value={label(viewModel.activationResult.technical_failure_reason)}
                />
              ) : null}
              {viewModel.activationResult.detail_message ? (
                <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.activationResult.detail_message}</p>
              ) : null}
            </div>
          ) : null}
        </Panel>

        <Panel title="Provider configuration">
          <form onSubmit={actions.onProviderSubmit} style={{ display: "grid", gap: 10 }}>
            <label style={{ display: "grid", gap: 4 }}>
              <span>Provider</span>
              <select
                aria-label="Provider"
                value={viewModel.selectedProvider}
                onChange={(event) => actions.onSelectedProviderChange(event.target.value as SupportedProvider)}
                style={controlStyle}
              >
                <option value="open_ai">OpenAI</option>
                <option value="google">Google</option>
              </select>
            </label>
            <label style={{ display: "grid", gap: 4 }}>
              <span>Provider API key</span>
              <input
                aria-label="Provider API key"
                value={viewModel.providerApiKey}
                onChange={(event) => actions.onProviderApiKeyChange(event.target.value)}
                style={controlStyle}
              />
            </label>
            <button
              type="submit"
              disabled={viewModel.submittingProvider || viewModel.providerApiKey.trim().length === 0}
              style={{
                ...buttonStyle(
                  "primary",
                  viewModel.submittingProvider || viewModel.providerApiKey.trim().length === 0
                ),
                justifySelf: "start"
              }}
            >
              {viewModel.submittingProvider ? "Committing provider configuration" : "Commit provider configuration"}
            </button>
          </form>
          {viewModel.providerOutcome ? (
            <div style={{ display: "grid", gap: 8 }}>
              <h3 style={{ margin: 0, fontSize: "0.98rem" }}>Provider commit result</h3>
              <FieldText labelText="Decision type" value={label(viewModel.providerOutcome.decision_type)} />
              {viewModel.providerOutcome.failure_reason ? (
                <FieldText labelText="Failure reason" value={label(viewModel.providerOutcome.failure_reason)} />
              ) : null}
              {viewModel.providerOutcome.detail_message ? (
                <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.providerOutcome.detail_message}</p>
              ) : null}
            </div>
          ) : null}
          {viewModel.providerState ? (
            <div style={{ display: "grid", gap: 6 }}>
              <FieldText labelText="Current status" value={label(viewModel.providerState.current_status)} />
              <FieldText labelText="Configuration present" value={boolText(viewModel.providerState.configuration_present)} />
            </div>
          ) : null}
        </Panel>
      </div>

      <CurrentPreflightState preflightState={viewModel.preflightState} />

      <Panel title="Preflight submission">
        <form
          onSubmit={actions.onPreflightSubmit}
          style={{ display: "grid", gap: 14, gridTemplateColumns: "repeat(auto-fit, minmax(190px, 1fr))" }}
        >
          <label style={{ display: "grid", gap: 4, gridColumn: "1 / -1" }}>
            <span>Seed document</span>
            <input aria-label="Seed document" type="file" onChange={actions.onSeedDocumentChange} style={controlStyle} />
          </label>
          <label style={{ display: "grid", gap: 4 }}>
            <span>Requested agent count</span>
            <input
              value={viewModel.requestedAgentCount}
              onChange={(event) => actions.onRequestedAgentCountChange(event.target.value)}
              style={controlStyle}
            />
          </label>
          <label style={{ display: "grid", gap: 4 }}>
            <span>Requested round count</span>
            <input
              value={viewModel.requestedRoundCount}
              onChange={(event) => actions.onRequestedRoundCountChange(event.target.value)}
              style={controlStyle}
            />
          </label>
          <label style={{ display: "grid", gap: 4 }}>
            <span>Max token budget</span>
            <input
              value={viewModel.maxTokenBudget}
              onChange={(event) => actions.onMaxTokenBudgetChange(event.target.value)}
              style={controlStyle}
            />
          </label>
          <div style={{ display: "grid", gap: 8, alignContent: "end" }}>
            <button
              type="submit"
              disabled={viewModel.preflightSubmitDisabled}
              style={{ ...buttonStyle("primary", viewModel.preflightSubmitDisabled), justifySelf: "start" }}
            >
              {viewModel.submittingPreflight ? "Submitting seed document" : "Submit seed document"}
            </button>
          </div>
        </form>

        {viewModel.selectedSeedDocument ? (
          <p style={{ margin: 0, lineHeight: 1.6 }}>Selected file: {viewModel.selectedSeedDocument.name}</p>
        ) : null}

        {viewModel.preflightOutcome ? (
          <div style={{ display: "grid", gap: 8 }}>
            <h3 style={{ margin: 0, fontSize: "0.98rem" }}>Preflight submission result</h3>
            <FieldText labelText="Decision type" value={label(viewModel.preflightOutcome.decision_type)} />
            {viewModel.preflightOutcome.rejection_reason ? (
              <FieldText labelText="Rejection reason" value={label(viewModel.preflightOutcome.rejection_reason)} />
            ) : null}
            {viewModel.preflightOutcome.technical_failure_reason ? (
              <FieldText
                labelText="Technical failure reason"
                value={label(viewModel.preflightOutcome.technical_failure_reason)}
              />
            ) : null}
            <PreflightDetails source={viewModel.preflightOutcome.submission_state ?? null} />
            {viewModel.preflightOutcome.detail_message ? (
              <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.preflightOutcome.detail_message}</p>
            ) : null}
          </div>
        ) : (
          <PreflightDetails source={viewModel.preflightSource} />
        )}
      </Panel>

      {viewModel.showDowngradeDisclosure ? (
        <Panel title="Downgrade disclosure">
          <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.preflightState?.downgrade_warning}</p>
          <button
            type="button"
            onClick={actions.onAcceptDowngrade}
            disabled={viewModel.acceptingDowngrade}
            style={buttonStyle("secondary", viewModel.acceptingDowngrade)}
          >
            {viewModel.acceptingDowngrade ? "Accepting downgraded effective plan" : "Accept downgraded effective plan"}
          </button>
          {viewModel.downgradeAcceptanceOutcome ? (
            <div style={{ display: "grid", gap: 6 }}>
              <p style={{ margin: 0, lineHeight: 1.6 }}>
                Downgrade acceptance detail: {viewModel.downgradeAcceptanceOutcome.detail_message}
              </p>
              <FieldText
                labelText="Downgrade acceptance recorded"
                value={boolText(viewModel.downgradeAcceptanceOutcome.acceptance_recorded)}
              />
            </div>
          ) : null}
        </Panel>
      ) : null}

      <div
        style={{
          display: "grid",
          gap: 20,
          gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))",
          alignItems: "start"
        }}
      >
        <Panel title="Runtime persona and bounded memory foundation">
          <div style={{ display: "grid", gap: 8 }}>
            <h3 style={{ margin: 0, fontSize: "0.98rem" }}>Runtime persona contract</h3>
            {viewModel.runtimePersonaState?.contract_name ? (
              <p style={{ margin: 0, lineHeight: 1.6 }}>{label(viewModel.runtimePersonaState.contract_name)}</p>
            ) : null}
            <FieldText
              labelText="Exactly one Moderator required"
              value={boolText(viewModel.runtimePersonaState?.exactly_one_moderator_required)}
            />
          </div>
          <div style={{ display: "grid", gap: 8 }}>
            <h3 style={{ margin: 0, fontSize: "0.98rem" }}>Run bounded memory</h3>
            {viewModel.runBoundedMemoryState?.memory_surface_name ? (
              <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.runBoundedMemoryState.memory_surface_name}</p>
            ) : null}
            <FieldText
              labelText="Transcript archive separate from prompt memory"
              value={boolText(viewModel.runBoundedMemoryState?.transcript_archive_separate_from_prompt_memory)}
            />
          </div>
        </Panel>

        <Panel title="Moderated rounds and run-local tracking">
          <div style={{ display: "grid", gap: 8 }}>
            {viewModel.runModeratedRoundsState?.surface_name ? (
              <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.runModeratedRoundsState.surface_name}</p>
            ) : null}
            <FieldText
              labelText="Normal round minimum speakers"
              value={viewModel.runModeratedRoundsState?.normal_round_min_speakers ?? "Unavailable"}
            />
          </div>
          <div style={{ display: "grid", gap: 8 }}>
            {viewModel.runTokenDurationTrackingState?.surface_name ? (
              <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.runTokenDurationTrackingState.surface_name}</p>
            ) : null}
            <FieldText
              labelText="Integrated into execution path"
              value={boolText(viewModel.runTokenDurationTrackingState?.integrated_into_execution_path)}
            />
          </div>
          <RunSessionDetails runSessionState={viewModel.runSessionState} />
        </Panel>
      </div>
    </div>
  );
}