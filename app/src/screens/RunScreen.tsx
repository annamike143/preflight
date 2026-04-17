import type { RunScreenViewModel } from "../appViewModels";
import { boolText, label, summarizeRunEvent } from "../appHelpers";
import { FieldText, Panel, RunSessionDetails } from "./shared";
import { buttonStyle } from "./sharedStyles";

export interface RunScreenActions {
  onStartRunSession: () => void;
  onCancelRunSession: () => void;
  onRequestControlledShutdown: () => void;
}

interface RunScreenProps {
  viewModel: RunScreenViewModel;
  actions: RunScreenActions;
  startingRunSession: boolean;
  cancelingRunSession: boolean;
  requestingControlledShutdown: boolean;
}

export function RunScreen({
  viewModel,
  actions,
  startingRunSession,
  cancelingRunSession,
  requestingControlledShutdown
}: RunScreenProps) {
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
        <Panel title="Run start control">
          <FieldText
            labelText="Canonical state name"
            value={viewModel.runStartState?.current_state_name ?? "RunStartFoundationNotStarted"}
          />
          <FieldText
            labelText="Real run start authorized now"
            value={boolText(viewModel.runStartState?.real_run_start_authorized_now)}
          />
          <button
            type="button"
            onClick={actions.onStartRunSession}
            disabled={startingRunSession || !viewModel.runStartState?.real_run_start_authorized_now}
            style={buttonStyle(
              "primary",
              startingRunSession || !viewModel.runStartState?.real_run_start_authorized_now
            )}
          >
            {startingRunSession ? "Starting fresh moderated run" : "Start fresh moderated run"}
          </button>
        </Panel>

        <Panel title="Live run monitoring">
          <div style={{ display: "flex", gap: 10, flexWrap: "wrap" }}>
            <strong>{viewModel.lifecycleBadgeText}</strong>
            <strong>{viewModel.statusBadgeText}</strong>
            <strong>{viewModel.resolutionBadgeText}</strong>
          </div>
          <FieldText labelText="Current run id" value={viewModel.currentRunId} />
          <FieldText labelText="Sequence-aligned chronology items" value={viewModel.chronologyItems} />
          {viewModel.trackedTokenUnits !== null ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>
              {viewModel.trackedTokenUnits} token units of {viewModel.monitoringBudgetCap}
            </p>
          ) : null}
          <FieldText
            labelText="Stored history complete"
            value={boolText(viewModel.runEventHistoryState?.stored_history_complete)}
          />
          {typeof viewModel.tokenProgress === "number" && viewModel.tokenProgress >= 80 ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>
              Tracked token usage is at {viewModel.tokenProgress.toFixed(1)}% of the governed budget.
            </p>
          ) : null}
          {viewModel.monitoringAlert ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.monitoringAlert}</p>
          ) : null}
          {Array.isArray(viewModel.runEventHistoryState?.events)
            ? viewModel.runEventHistoryState.events.map((event) => (
                <p key={`event-${event.sequence_number}`} style={{ margin: 0, lineHeight: 1.6 }}>
                  {summarizeRunEvent(event)}
                </p>
              ))
            : null}
          {Array.isArray(viewModel.runEventHistoryState?.chronology_entries)
            ? viewModel.runEventHistoryState.chronology_entries.map((entry) => (
                <div key={`chronology-${entry.sequence_number}`} style={{ display: "grid", gap: 4 }}>
                  <p style={{ margin: 0 }}>Round {entry.round_number} • Turn {entry.turn_index}</p>
                  <p style={{ margin: 0, lineHeight: 1.6 }}>{entry.turn_text}</p>
                </div>
              ))
            : null}
        </Panel>
      </div>

      {viewModel.runSessionOutcome ? (
        <Panel title="Run/session start result">
          <p style={{ margin: 0, lineHeight: 1.6 }}>{label(viewModel.runSessionOutcome.decision_type)}</p>
          {viewModel.runSessionOutcome.technical_failure_reason ? (
            <FieldText
              labelText="Technical failure reason"
              value={label(viewModel.runSessionOutcome.technical_failure_reason)}
            />
          ) : null}
          {viewModel.runSessionOutcome.detail_message ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.runSessionOutcome.detail_message}</p>
          ) : null}
          <RunSessionDetails
            runSessionState={viewModel.runSessionOutcome.run_session_state ?? viewModel.runSessionState}
          />
        </Panel>
      ) : null}

      <div
        style={{
          display: "grid",
          gap: 20,
          gridTemplateColumns: "repeat(auto-fit, minmax(260px, 1fr))",
          alignItems: "start"
        }}
      >
        <Panel title="Shell-owned canonical lifecycle projection">
          <FieldText
            labelText="Canonical state name"
            value={viewModel.runLifecycleState?.current_state_name ?? "Unavailable"}
          />
          {viewModel.runLifecycleState?.current_projection_detail ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.runLifecycleState.current_projection_detail}</p>
          ) : null}
        </Panel>

        <Panel title="Run status contract">
          {viewModel.statusContractLabel ? (
            <p style={{ margin: 0, lineHeight: 1.6, fontWeight: 700, color: "#5d4830" }}>
              {viewModel.statusContractLabel}
            </p>
          ) : null}
          {viewModel.runStatusState?.contract_name ? (
            <p
              style={{
                margin: 0,
                lineHeight: 1.6,
                fontFamily: '"Consolas", "Courier New", monospace',
                fontSize: "0.9rem",
                color: "#6a5640"
              }}
            >
              {viewModel.runStatusState.contract_name}
            </p>
          ) : null}
          <FieldText
            labelText="Shell command name"
            value={viewModel.runStatusState?.shell_command_name ?? "Unavailable"}
          />
          <FieldText
            labelText="Cancellation command implemented"
            value={boolText(viewModel.runStatusState?.cancellation_request_contract?.cancellation_command_implemented)}
          />
          <div style={{ display: "flex", gap: 10, flexWrap: "wrap" }}>
            <button
              type="button"
              onClick={actions.onCancelRunSession}
              disabled={cancelingRunSession || !viewModel.runStatusState?.current_state_cancelable}
              style={buttonStyle(
                "secondary",
                cancelingRunSession || !viewModel.runStatusState?.current_state_cancelable
              )}
            >
              {cancelingRunSession ? "Canceling active run" : "Cancel active run"}
            </button>
            <button
              type="button"
              onClick={actions.onRequestControlledShutdown}
              disabled={requestingControlledShutdown || !!viewModel.runStatusState?.current_state_terminal}
              style={buttonStyle(
                "secondary",
                requestingControlledShutdown || !!viewModel.runStatusState?.current_state_terminal
              )}
            >
              {requestingControlledShutdown ? "Requesting controlled shutdown" : "Request controlled shutdown"}
            </button>
          </div>
        </Panel>

        <Panel title="Run event contract">
          {viewModel.eventContractLabel ? (
            <p style={{ margin: 0, lineHeight: 1.6, fontWeight: 700, color: "#5d4830" }}>
              {viewModel.eventContractLabel}
            </p>
          ) : null}
          {viewModel.runEventContractState?.contract_name ? (
            <p
              style={{
                margin: 0,
                lineHeight: 1.6,
                fontFamily: '"Consolas", "Courier New", monospace',
                fontSize: "0.9rem",
                color: "#6a5640"
              }}
            >
              {viewModel.runEventContractState.contract_name}
            </p>
          ) : null}
          <FieldText
            labelText="Event history command name"
            value={viewModel.runEventContractState?.event_history_command_name ?? "Unavailable"}
          />
          <FieldText
            labelText="Event-history retrieval implemented"
            value={boolText(viewModel.runEventContractState?.event_history_retrieval_implemented)}
          />
          <FieldText
            labelText="Gap marker supported"
            value={boolText(viewModel.runEventContractState?.gap_contract?.gap_marker_supported)}
          />
          <FieldText
            labelText="Ordered event history materialized"
            value={boolText(
              viewModel.runStatusState?.ordered_event_history_materialized ??
                viewModel.runEventContractState?.event_history_materialized
            )}
          />
          {Array.isArray(viewModel.runEventContractState?.supported_runtime_domains) ? (
            <FieldText
              labelText="Supported runtime domains"
              value={viewModel.runEventContractState.supported_runtime_domains.map((domain) => label(domain)).join(", ")}
            />
          ) : null}
        </Panel>

        {!viewModel.runSessionOutcome ? (
          <Panel title="Run/session state snapshot">
            <RunSessionDetails runSessionState={viewModel.runSessionState} />
          </Panel>
        ) : null}
      </div>
    </div>
  );
}