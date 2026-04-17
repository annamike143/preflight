import type { DiagnosticsScreenViewModel } from "../appViewModels";
import { boolText } from "../appHelpers";
import { FieldText, Panel } from "./shared";
import { detailsStyle, detailsSummaryStyle } from "./sharedStyles";

interface DiagnosticsScreenProps {
  viewModel: DiagnosticsScreenViewModel;
}

export function DiagnosticsScreen({ viewModel }: DiagnosticsScreenProps) {
  return (
    <div style={{ display: "grid", gap: 20 }}>
      <Panel title="Diagnostics surfaces">
        <details style={detailsStyle}>
          <summary style={detailsSummaryStyle}>Runtime persona contract</summary>
          <div style={{ marginTop: 12, display: "grid", gap: 8 }}>
            {viewModel.runtimePersonaContractLabel ? (
              <p style={{ margin: 0, lineHeight: 1.6, fontWeight: 700, color: "#5d4830" }}>
                {viewModel.runtimePersonaContractLabel}
              </p>
            ) : null}
            <FieldText
              labelText="Contract name"
              value={viewModel.runtimePersonaState?.contract_name ?? "Unavailable"}
            />
            <FieldText
              labelText="Exactly one Moderator required"
              value={boolText(viewModel.runtimePersonaState?.exactly_one_moderator_required)}
            />
            <FieldText
              labelText="Stable persona labels required"
              value={boolText(viewModel.runtimePersonaState?.stable_persona_labels_required)}
            />
          </div>
        </details>

        <details style={detailsStyle}>
          <summary style={detailsSummaryStyle}>Bounded memory surface</summary>
          <div style={{ marginTop: 12, display: "grid", gap: 8 }}>
            {viewModel.boundedMemorySurfaceLabel ? (
              <p style={{ margin: 0, lineHeight: 1.6, fontWeight: 700, color: "#5d4830" }}>
                {viewModel.boundedMemorySurfaceLabel}
              </p>
            ) : null}
            <FieldText
              labelText="Memory surface name"
              value={viewModel.runBoundedMemoryState?.memory_surface_name ?? "Unavailable"}
            />
            <FieldText
              labelText="Transcript archive separate from prompt memory"
              value={boolText(viewModel.runBoundedMemoryState?.transcript_archive_separate_from_prompt_memory)}
            />
          </div>
        </details>

        <details style={detailsStyle}>
          <summary style={detailsSummaryStyle}>Moderated rounds and tracking</summary>
          <div style={{ marginTop: 12, display: "grid", gap: 8 }}>
            {viewModel.moderatedRoundsSurfaceLabel ? (
              <p style={{ margin: 0, lineHeight: 1.6, fontWeight: 700, color: "#5d4830" }}>
                {viewModel.moderatedRoundsSurfaceLabel}
              </p>
            ) : null}
            <FieldText
              labelText="Rounds surface"
              value={viewModel.runModeratedRoundsState?.surface_name ?? "Unavailable"}
            />
            {viewModel.trackingSurfaceLabel ? (
              <p style={{ margin: 0, lineHeight: 1.6, color: "#6a5640" }}>
                Tracking surface: {viewModel.trackingSurfaceLabel}
              </p>
            ) : null}
            <FieldText
              labelText="Normal round minimum speakers"
              value={viewModel.runModeratedRoundsState?.normal_round_min_speakers ?? "Unavailable"}
            />
            <FieldText
              labelText="Tracking surface"
              value={viewModel.runTokenDurationTrackingState?.surface_name ?? "Unavailable"}
            />
            <FieldText
              labelText="Integrated into execution path"
              value={boolText(viewModel.runTokenDurationTrackingState?.integrated_into_execution_path)}
            />
          </div>
        </details>

        <details style={detailsStyle}>
          <summary style={detailsSummaryStyle}>Live state projections</summary>
          <div style={{ marginTop: 12, display: "grid", gap: 8 }}>
            {viewModel.lifecycleSurfaceLabel ? (
              <p style={{ margin: 0, lineHeight: 1.6, fontWeight: 700, color: "#5d4830" }}>
                {viewModel.lifecycleSurfaceLabel}
              </p>
            ) : null}
            <FieldText
              labelText="Lifecycle surface"
              value={viewModel.runLifecycleState?.surface_name ?? "Unavailable"}
            />
            <FieldText
              labelText="Current lifecycle name"
              value={viewModel.runLifecycleState?.current_state_name ?? "Unavailable"}
            />
            {viewModel.statusContractLabel ? (
              <p style={{ margin: 0, lineHeight: 1.6, color: "#6a5640" }}>
                Status contract label: {viewModel.statusContractLabel}
              </p>
            ) : null}
            <FieldText
              labelText="Status contract"
              value={viewModel.runStatusState?.contract_name ?? "Unavailable"}
            />
            <FieldText
              labelText="Current run id"
              value={viewModel.runStatusState?.current_run_id ?? "Unavailable"}
            />
          </div>
        </details>
      </Panel>

      <Panel title="Current repo truth">
        <details style={detailsStyle}>
          <summary style={detailsSummaryStyle}>Repo truth notes ({viewModel.sections.length})</summary>
          <ul style={{ margin: "12px 0 0", paddingLeft: 20, display: "grid", gap: 8 }}>
            {viewModel.sections.map((section) => (
              <li key={section}>{section}</li>
            ))}
          </ul>
        </details>
      </Panel>
    </div>
  );
}