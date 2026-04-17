import { useState } from "react";

import type { ReportAccessPreviewMode, ReportsScreenViewModel } from "../appViewModels";
import { boolText, label } from "../appHelpers";
import {
  buttonStyle,
  controlStyle,
  detailsStyle,
  detailsSummaryStyle
} from "./sharedStyles";
import {
  FieldText,
  Panel
} from "./shared";

export interface ReportsScreenActions {
  onOpenLocalReport: () => void;
  onExportLocalReport: () => void;
  onReportAccessPreviewModeChange: (value: ReportAccessPreviewMode) => void;
}

interface ReportsScreenProps {
  viewModel: ReportsScreenViewModel;
  actions: ReportsScreenActions;
}

export function ReportsScreen({ viewModel, actions }: ReportsScreenProps) {
  const [showTechnicalAccessContract, setShowTechnicalAccessContract] = useState(false);
  const cleanupExecutionMaterialized = Boolean(
    viewModel.runCleanupContractState?.real_cleanup_execution_materialized
  );
  const startupJanitorExecutionMaterialized = Boolean(
    viewModel.startupJanitorContractState?.real_startup_janitor_execution_materialized
  );
  const cleanupPanelTitle = startupJanitorExecutionMaterialized
    ? "Phase 9 cleanup and janitor execution truth"
    : cleanupExecutionMaterialized
      ? "Phase 9 cleanup truth and janitor contract foundation"
      : "Phase 9 cleanup and janitor contract foundation";
  const cleanupSummary = startupJanitorExecutionMaterialized
    ? "These Phase 9 surfaces now materialize real shell-owned terminal cleanup, abnormal-leftover detection, and startup janitor purge execution while preserving exported reports and durable app-local state outside cleanup and janitor scope."
    : cleanupExecutionMaterialized
      ? "These Phase 9 surfaces now materialize real shell-owned terminal cleanup triggering and cleanup outcome truth while startup janitor execution remains deferred. Preserved exported reports and durable app-local state remain outside cleanup and janitor scope."
      : "These Phase 9 surfaces define cleanup outcome vocabulary, next-launch janitor vocabulary, and cleanup-scope mapping without claiming real terminal cleanup or startup janitor execution. Preserved exported reports and durable app-local state remain outside cleanup and janitor scope.";

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
        <Panel title="Phase 8 finalization and report contract foundation">
          {viewModel.runFinalizationContractState?.contract_name ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.runFinalizationContractState.contract_name}</p>
          ) : null}
          {viewModel.reportOutputContractState?.contract_name ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.reportOutputContractState.contract_name}</p>
          ) : null}
          {viewModel.reportStorageBoundaryState?.boundary_name ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.reportStorageBoundaryState.boundary_name}</p>
          ) : null}
          <h3 style={{ margin: 0, fontSize: "0.98rem" }}>Local report PDF access</h3>
          <FieldText
            labelText="Completion implies report generation success"
            value={boolText(
              viewModel.runFinalizationContractState?.completion_classification_implies_report_generation_success
            )}
          />
          <FieldText
            labelText="Real report generation materialized"
            value={boolText(viewModel.runFinalizationContractState?.real_report_generation_materialized)}
          />
          <FieldText
            labelText="Real PDF generation materialized"
            value={boolText(viewModel.runFinalizationContractState?.real_pdf_generation_materialized)}
          />
          <FieldText
            labelText="PDF format"
            value={String(viewModel.reportOutputContractState?.pdf_contract?.file_format_name ?? "pdf").toUpperCase()}
          />
          <FieldText
            labelText="Successful PDF generation implied"
            value={boolText(viewModel.reportOutputContractState?.pdf_contract?.successful_generation_implied)}
          />
          <p style={{ margin: 0, lineHeight: 1.6 }}>
            These Phase 8 surfaces now materialize governed PDF generation, durable app-local report artifacts,
            shell-owned availability truth, and shell-mediated local open/export actions. The frontend only renders
            report access from current shell state and does not infer file availability from run completion alone.
          </p>
        </Panel>

        <Panel title={cleanupPanelTitle}>
          {viewModel.runCleanupContractState?.contract_name ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.runCleanupContractState.contract_name}</p>
          ) : null}
          {viewModel.cleanupScopeMappingState?.mapping_name ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.cleanupScopeMappingState.mapping_name}</p>
          ) : null}
          {viewModel.startupJanitorContractState?.contract_name ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.startupJanitorContractState.contract_name}</p>
          ) : null}
          <FieldText
            labelText="Cleanup execution materialized"
            value={boolText(viewModel.runCleanupContractState?.real_cleanup_execution_materialized)}
          />
          <FieldText
            labelText="Startup janitor execution materialized"
            value={boolText(viewModel.startupJanitorContractState?.real_startup_janitor_execution_materialized)}
          />
          <FieldText
            labelText="Cleanup outcome distinct from run outcome"
            value={boolText(viewModel.runCleanupContractState?.cleanup_outcome_distinct_from_run_outcome)}
          />
          <FieldText
            labelText="Preserved report cleanup exception"
            value={boolText(viewModel.cleanupScopeMappingState?.preserved_report_exception_explicit)}
          />
          <FieldText
            labelText="Durable support-state exclusion explicit"
            value={boolText(viewModel.cleanupScopeMappingState?.durable_support_state_exclusion_explicit)}
          />
          <FieldText
            labelText="Abnormal termination detection materialized"
            value={boolText(viewModel.startupJanitorContractState?.real_abnormal_termination_detection_materialized)}
          />
          <FieldText
            labelText="Startup janitor checked this launch"
            value={boolText(viewModel.startupJanitorContractState?.execution_checked_this_launch)}
          />
          {viewModel.startupJanitorContractState?.latest_marker_name ? (
            <FieldText
              labelText="Latest janitor marker"
              value={viewModel.startupJanitorContractState.latest_marker_name}
            />
          ) : null}
          {viewModel.startupJanitorContractState?.residual_workspace_count !== null &&
          viewModel.startupJanitorContractState?.residual_workspace_count !== undefined ? (
            <FieldText
              labelText="Residual workspaces detected"
              value={viewModel.startupJanitorContractState.residual_workspace_count}
            />
          ) : null}
          {viewModel.startupJanitorContractState?.purged_workspace_count !== null &&
          viewModel.startupJanitorContractState?.purged_workspace_count !== undefined ? (
            <FieldText
              labelText="Residual workspaces purged"
              value={viewModel.startupJanitorContractState.purged_workspace_count}
            />
          ) : null}
          {viewModel.startupJanitorContractState?.failed_workspace_count !== null &&
          viewModel.startupJanitorContractState?.failed_workspace_count !== undefined ? (
            <FieldText
              labelText="Residual workspaces still blocked"
              value={viewModel.startupJanitorContractState.failed_workspace_count}
            />
          ) : null}
          {viewModel.startupJanitorContractState?.latest_status_note ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>
              {viewModel.startupJanitorContractState.latest_status_note}
            </p>
          ) : null}
          {viewModel.cleanupScopeMappingState?.cleanup_entries?.length ? (
            <div style={{ display: "grid", gap: 10 }}>
              <h3 style={{ margin: 0, fontSize: "0.98rem" }}>Cleanup scope map</h3>
              {viewModel.cleanupScopeMappingState.cleanup_entries.map((entry) => (
                <div
                  key={`${entry.data_class_name ?? "scope"}-${entry.storage_domain_name ?? "domain"}`}
                  style={{
                    display: "grid",
                    gap: 6,
                    padding: 12,
                    borderRadius: 16,
                    background: "rgba(255, 247, 234, 0.7)",
                    border: "1px solid #e3d2b7"
                  }}
                >
                  <div style={{ fontWeight: 700 }}>{entry.data_class_name ?? "Unavailable"}</div>
                  <FieldText labelText="Storage domain" value={entry.storage_domain_name ?? "Unavailable"} />
                  <FieldText
                    labelText="Terminal cleanup applies"
                    value={boolText(entry.cleanup_scope_applies)}
                  />
                  <FieldText
                    labelText="Startup janitor applies"
                    value={boolText(entry.startup_janitor_scope_applies)}
                  />
                  <FieldText
                    labelText="Preserved report artifact"
                    value={boolText(entry.preserved_report_artifact)}
                  />
                </div>
              ))}
            </div>
          ) : null}
          <p style={{ margin: 0, lineHeight: 1.6 }}>{cleanupSummary}</p>
        </Panel>

        <Panel title="Report artifact card">
          {viewModel.developerReportPreviewEnabled ? (
            <label style={{ display: "grid", gap: 4 }}>
              <span>Preview state (dev only)</span>
              <select
                aria-label="Preview state (dev only)"
                value={viewModel.reportAccessPreviewMode}
                onChange={(event) =>
                  actions.onReportAccessPreviewModeChange(event.target.value as ReportAccessPreviewMode)
                }
                style={controlStyle}
              >
                <option value="live">Live</option>
                <option value="available">Available</option>
                <option value="unavailable">Unavailable</option>
                <option value="missing_artifact">Missing Artifact</option>
                <option value="inaccessible_artifact">Inaccessible Artifact</option>
              </select>
            </label>
          ) : null}
          {viewModel.reportAccessPreviewMode !== "live" ? (
            <p style={{ margin: 0, lineHeight: 1.6 }}>
              Showing {label(viewModel.reportAccessPreviewMode)}. This affects the UI preview only; the live shell
              state still refreshes underneath.
            </p>
          ) : null}
          <div
            style={{
              display: "grid",
              gap: 10,
              padding: 16,
              borderRadius: 18,
              background: "rgba(255, 247, 234, 0.82)",
              border: "1px solid #e3d2b7"
            }}
          >
            <div
              style={{
                display: "flex",
                justifyContent: "space-between",
                gap: 12,
                alignItems: "start",
                flexWrap: "wrap"
              }}
            >
              <div style={{ display: "grid", gap: 6 }}>
                <h3 style={{ margin: 0, fontSize: "1.18rem" }}>{viewModel.artifactPresentation.title}</h3>
                <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.artifactPresentation.detail}</p>
              </div>
              <div
                style={{
                  borderRadius: 999,
                  padding: "8px 12px",
                  background: "rgba(76, 58, 33, 0.08)",
                  color: "#4c3a21",
                  fontWeight: 700,
                  whiteSpace: "nowrap"
                }}
              >
                {label(viewModel.effectiveAvailabilityName)}
              </div>
            </div>

            {viewModel.effectiveReportId ? (
              <div style={{ display: "grid", gap: 4 }}>
                <p style={{ margin: 0, fontSize: "0.8rem", letterSpacing: "0.08em", textTransform: "uppercase" }}>
                  Report ID
                </p>
                <p style={{ margin: 0, fontWeight: 700 }}>{viewModel.effectiveReportId}</p>
              </div>
            ) : null}

            {viewModel.effectiveArtifactPath ? (
              <FieldText labelText="Artifact path" value={viewModel.effectiveArtifactPath} />
            ) : null}
            {viewModel.effectiveGeneratedAt ? (
              <FieldText labelText="Generated at" value={new Date(viewModel.effectiveGeneratedAt).toLocaleString()} />
            ) : null}
            {typeof viewModel.effectiveArtifactSizeBytes === "number" ? (
              <FieldText labelText="Artifact size" value={`${viewModel.effectiveArtifactSizeBytes} bytes`} />
            ) : null}

            <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.artifactPresentation.actionGuidance}</p>
            <div style={{ display: "flex", gap: 10, flexWrap: "wrap" }}>
              <button
                type="button"
                onClick={actions.onOpenLocalReport}
                disabled={!viewModel.canOpenLocalReport}
                style={buttonStyle("primary", !viewModel.canOpenLocalReport)}
              >
                Open local PDF
              </button>
              <button
                type="button"
                onClick={actions.onExportLocalReport}
                disabled={!viewModel.canExportLocalReport}
                style={buttonStyle("secondary", !viewModel.canExportLocalReport)}
              >
                Export local PDF
              </button>
            </div>
          </div>

          {viewModel.reportAccessExecutionOutcome ? (
            <div style={{ display: "grid", gap: 8 }}>
              <h3 style={{ margin: 0, fontSize: "0.98rem" }}>Latest action</h3>
              <p style={{ margin: 0, lineHeight: 1.6 }}>
                {label(viewModel.reportAccessExecutionOutcome.request_kind)} {label(viewModel.reportAccessExecutionOutcome.decision_type)}
              </p>
              <p style={{ margin: 0, lineHeight: 1.6 }}>{viewModel.reportAccessExecutionOutcome.detail_message}</p>
              {viewModel.reportAccessExecutionOutcome.exported_artifact_path ? (
                <FieldText
                  labelText="Exported artifact path"
                  value={viewModel.reportAccessExecutionOutcome.exported_artifact_path}
                />
              ) : null}
            </div>
          ) : null}
        </Panel>
      </div>

      <Panel title="Report access contract surface">
        <details open={showTechnicalAccessContract} style={detailsStyle}>
          <summary
            onClick={(event) => {
              event.preventDefault();
              setShowTechnicalAccessContract((value) => !value);
            }}
            style={detailsSummaryStyle}
          >
            Technical access contract
          </summary>
          {showTechnicalAccessContract ? (
            <div style={{ marginTop: 12, display: "grid", gap: 8 }}>
              <FieldText
                labelText="Contract name"
                value={viewModel.reportAccessContractState?.contract_name ?? "Unavailable"}
              />
              <FieldText
                labelText="Shell command name"
                value={viewModel.reportAccessContractState?.shell_command_name ?? "Unavailable"}
              />
              <FieldText
                labelText="Availability state"
                value={label(viewModel.effectiveAvailabilityStatus)}
              />
              <FieldText
                labelText="Report available now"
                value={boolText(viewModel.reportAccessContractState?.report_available_now)}
              />
              <FieldText
                labelText="Local artifact accessible now"
                value={boolText(viewModel.reportAccessContractState?.local_artifact_accessible_now)}
              />
            </div>
          ) : null}
        </details>
      </Panel>
    </div>
  );
}