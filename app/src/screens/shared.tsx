import type { ReactNode } from "react";

import type {
  PreflightDetailsSource,
  PreflightState,
  RunSessionState
} from "../appViewModels";
import { boolText, formatList, label } from "../appHelpers";

export function FieldText({ labelText, value }: { labelText: string; value: ReactNode }) {
  return <p style={{ margin: 0, lineHeight: 1.55 }}>{labelText}: {value}</p>;
}

export function Panel({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section
      style={{
        border: "1px solid #d9d3c5",
        borderRadius: 22,
        padding: 22,
        background:
          "linear-gradient(180deg, rgba(255, 250, 240, 0.98) 0%, rgba(252, 246, 236, 0.98) 100%)",
        boxShadow: "0 16px 34px rgba(72, 49, 20, 0.08)",
        display: "grid",
        gap: 12
      }}
    >
      <h2
        style={{
          margin: 0,
          fontSize: "1.05rem",
          color: "#4c3a21",
          letterSpacing: "0.01em"
        }}
      >
        {title}
      </h2>
      <div style={{ display: "grid", gap: 10 }}>{children}</div>
    </section>
  );
}

export function CurrentPreflightState({ preflightState }: { preflightState: PreflightState | null }) {
  if (!preflightState) {
    return null;
  }

  return (
    <Panel title="Current Preflight boundary state">
      <FieldText labelText="Canonical state name" value={preflightState.current_state_name ?? "Unavailable"} />
      <FieldText labelText="Run-start eligible now" value={boolText(preflightState.run_start_eligible_now)} />
      <FieldText
        labelText="Run start blocked without eligible Preflight"
        value={boolText(preflightState.run_start_blocked_without_eligible_preflight)}
      />
      <FieldText labelText="Current result stale" value={boolText(preflightState.current_result_stale)} />
      {Array.isArray(preflightState.invalidation_reasons) && preflightState.invalidation_reasons.length > 0 ? (
        <FieldText labelText="Invalidation reasons" value={formatList(preflightState.invalidation_reasons)} />
      ) : null}
      {preflightState.last_rejection_reason ? (
        <FieldText labelText="Last rejection reason" value={label(preflightState.last_rejection_reason)} />
      ) : null}
      {preflightState.last_technical_failure_reason ? (
        <FieldText
          labelText="Last technical failure reason"
          value={label(preflightState.last_technical_failure_reason)}
        />
      ) : null}
      {preflightState.current_status_note ? (
        <p style={{ margin: 0, lineHeight: 1.6 }}>{preflightState.current_status_note}</p>
      ) : null}
    </Panel>
  );
}

export function PreflightDetails({ source }: { source: PreflightDetailsSource | null }) {
  if (!source) {
    return null;
  }

  return (
    <>
      <FieldText labelText="Valid document pending" value={boolText(source.valid_document_in_pending_configuration)} />
      {source.requested_plan_budget_feasible !== null && source.requested_plan_budget_feasible !== undefined ? (
        <FieldText
          labelText="Requested-plan budget feasible"
          value={boolText(source.requested_plan_budget_feasible)}
        />
      ) : null}
      {source.selected_seed_document ? (
        <div style={{ display: "grid", gap: 6 }}>
          <h3 style={{ margin: "4px 0 0", fontSize: "0.98rem" }}>Selected seed document</h3>
          <FieldText labelText="File name" value={source.selected_seed_document.file_name} />
          <FieldText labelText="Detected kind" value={label(source.selected_seed_document.detected_kind)} />
        </div>
      ) : null}
      {source.requested_plan ? (
        <div style={{ display: "grid", gap: 6 }}>
          <h3 style={{ margin: "4px 0 0", fontSize: "0.98rem" }}>Requested plan</h3>
          <FieldText labelText="Requested agent count" value={source.requested_plan.requested_agent_count} />
          <FieldText labelText="Requested round count" value={source.requested_plan.requested_round_count} />
          <FieldText labelText="Max token budget" value={source.requested_plan.max_token_budget} />
        </div>
      ) : null}
      {source.effective_plan ? (
        <div style={{ display: "grid", gap: 6 }}>
          <h3 style={{ margin: "4px 0 0", fontSize: "0.98rem" }}>Effective plan</h3>
          <FieldText labelText="Effective agent count" value={source.effective_plan.effective_agent_count} />
          <FieldText labelText="Effective round count" value={source.effective_plan.effective_round_count} />
          <FieldText labelText="Provider" value={label(source.effective_plan.provider)} />
          <FieldText labelText="Model" value={source.effective_plan.model ?? "Unavailable"} />
        </div>
      ) : null}
    </>
  );
}

export function RunSessionDetails({ runSessionState }: { runSessionState: RunSessionState | null }) {
  if (!runSessionState) {
    return null;
  }

  return (
    <div style={{ display: "grid", gap: 6 }}>
      {runSessionState.run_id ? <FieldText labelText="Run id" value={runSessionState.run_id} /> : null}
      {runSessionState.session_id ? <FieldText labelText="Session id" value={runSessionState.session_id} /> : null}
      <FieldText labelText="Workspace allocated" value={boolText(runSessionState.workspace_allocated)} />
      <FieldText labelText="Execution seed staged" value={boolText(runSessionState.execution_seed_staged)} />
      {runSessionState.moderator_persona_label ? (
        <FieldText labelText="Moderator label" value={runSessionState.moderator_persona_label} />
      ) : null}
      {runSessionState.participant_persona_count !== null && runSessionState.participant_persona_count !== undefined ? (
        <FieldText labelText="Participant persona count" value={runSessionState.participant_persona_count} />
      ) : null}
      {Array.isArray(runSessionState.participant_persona_labels) && runSessionState.participant_persona_labels.length > 0 ? (
        <FieldText
          labelText="Participant persona labels"
          value={runSessionState.participant_persona_labels.join(", ")}
        />
      ) : null}
      {Array.isArray(runSessionState.perspective_coverage) && runSessionState.perspective_coverage.length > 0 ? (
        <FieldText labelText="Perspective coverage" value={formatList(runSessionState.perspective_coverage)} />
      ) : null}
      <FieldText
        labelText="Transcript archive separated"
        value={boolText(runSessionState.transcript_archive_separated)}
      />
      {runSessionState.completed_round_count !== null && runSessionState.completed_round_count !== undefined ? (
        <FieldText labelText="Completed round count" value={runSessionState.completed_round_count} />
      ) : null}
      {Array.isArray(runSessionState.speaker_counts_per_round) && runSessionState.speaker_counts_per_round.length > 0 ? (
        <FieldText
          labelText="Speaker counts per round"
          value={runSessionState.speaker_counts_per_round.join(", ")}
        />
      ) : null}
      {runSessionState.tracked_total_token_units !== null && runSessionState.tracked_total_token_units !== undefined ? (
        <FieldText labelText="Tracked total token units" value={runSessionState.tracked_total_token_units} />
      ) : null}
      {runSessionState.token_duration_tracking_path ? (
        <FieldText labelText="Token/duration tracking path" value={runSessionState.token_duration_tracking_path} />
      ) : null}
      {runSessionState.execution_completion_state ? (
        <FieldText
          labelText="Execution completion state"
          value={label(runSessionState.execution_completion_state)}
        />
      ) : null}
      {runSessionState.runtime_cap_reason ? (
        <FieldText labelText="Runtime cap reason" value={label(runSessionState.runtime_cap_reason)} />
      ) : null}
      {runSessionState.controlled_finalization_performed ? (
        <FieldText labelText="Controlled finalization performed" value="yes" />
      ) : null}
      {runSessionState.cleanup_marker ? (
        <FieldText labelText="Cleanup marker" value={label(runSessionState.cleanup_marker)} />
      ) : null}
      {runSessionState.detail_message ? (
        <p style={{ margin: 0, lineHeight: 1.6 }}>{runSessionState.detail_message}</p>
      ) : null}
    </div>
  );
}