import { invoke } from "@tauri-apps/api/core";
import { type ChangeEvent, type ReactNode, useEffect, useRef, useState } from "react";

import { appScreenContent, type AppScreen } from "./appViewModels";
import { label } from "./appHelpers";
import { useDiagnosticsScreenViewModel } from "./hooks/useDiagnosticsScreenViewModel";
import { useReportsScreenState } from "./hooks/useReportsScreenState";
import { useRunScreenState } from "./hooks/useRunScreenState";
import { useSetupScreenState } from "./hooks/useSetupScreenState";
import { DiagnosticsScreen } from "./screens/DiagnosticsScreen";
import { ReportsScreen } from "./screens/ReportsScreen";
import { buttonStyle } from "./screens/sharedStyles";
import { RunScreen } from "./screens/RunScreen";
import { SetupScreen } from "./screens/SetupScreen";

interface UpdaterContractState {
  contract_name?: string | null;
  shell_discovery_command_name?: string | null;
  shell_prepare_command_name?: string | null;
  shell_apply_command_name?: string | null;
  current_state_name?: string | null;
  current_status_note?: string | null;
  current_update_available?: boolean;
  current_update_ready?: boolean;
  current_candidate_rejected?: boolean;
  current_error_active?: boolean;
  discovery_configuration_ready?: boolean;
  discovery_endpoints_configured?: boolean;
  discovery_pubkey_configured?: boolean;
  discovery_endpoint_count?: number | null;
  discovery_configuration_note?: string | null;
  discovery_allowed_during_active_run?: boolean;
  available_state_distinct_from_ready?: boolean;
  rejection_distinct_from_error?: boolean;
  verification_required_before_ready?: boolean;
  active_run_apply_forbidden?: boolean;
  apply_deferred_during_active_run?: boolean;
  apply_allowed_now?: boolean;
  apply_blocked_by_active_run_now?: boolean;
  apply_blocked_because_update_not_ready_now?: boolean;
  restart_coordination_required_after_apply?: boolean;
  apply_status_note?: string | null;
  active_run_detected_during_last_check?: boolean;
  last_check_attempted?: boolean;
  last_error_detail?: string | null;
  last_discovered_version?: string | null;
  last_discovered_release_notes?: string | null;
  last_discovered_publication_date?: string | null;
  real_update_check_materialized?: boolean;
  real_update_download_materialized?: boolean;
  real_update_verification_materialized?: boolean;
  real_update_apply_materialized?: boolean;
}

interface RuntimeDeliveryContractState {
  contract_name?: string | null;
  runtime_package_contract_name?: string | null;
  authorization_basis_defined?: boolean;
  compatibility_basis_defined?: boolean;
  verification_basis_defined?: boolean;
  runtime_use_blocked_until_verified?: boolean;
  runtime_delivery_mutation_during_active_run_forbidden?: boolean;
  real_authorization_exchange_materialized?: boolean;
  real_artifact_acquisition_materialized?: boolean;
  real_verification_execution_materialized?: boolean;
  real_runtime_apply_materialized?: boolean;
}

function truthText(value?: boolean) {
  if (value === true) {
    return "Yes";
  }

  if (value === false) {
    return "No";
  }

  return "Unknown";
}

export default function App() {
  const [activeScreen, setActiveScreen] = useState<AppScreen>("setup");
  const [updaterContractState, setUpdaterContractState] = useState<UpdaterContractState | null>(null);
  const [runtimeDeliveryContractState, setRuntimeDeliveryContractState] =
    useState<RuntimeDeliveryContractState | null>(null);
  const [phaseTenContractsLoading, setPhaseTenContractsLoading] = useState(false);
  const [phaseTenContractsError, setPhaseTenContractsError] = useState<string | null>(null);
  const [phaseTenUpdaterActionLoading, setPhaseTenUpdaterActionLoading] = useState<string | null>(null);
  const [phaseTenUpdaterActionError, setPhaseTenUpdaterActionError] = useState<string | null>(null);
  const phaseTenContractsLoadedRef = useRef(false);

  const setupState = useSetupScreenState();

  async function refreshPhaseTenContractsSurface(refreshUpdaterDiscovery: boolean) {
    setPhaseTenContractsLoading(true);
    setPhaseTenContractsError(null);

    try {
      const updaterCommand = refreshUpdaterDiscovery ? "check_native_app_updater" : "updater_contract_state";
      const [updaterState, runtimeDeliveryState] = await Promise.all([
        invoke<UpdaterContractState>(updaterCommand),
        invoke<RuntimeDeliveryContractState>("runtime_delivery_contract_state")
      ]);

      setUpdaterContractState(updaterState);
      setRuntimeDeliveryContractState(runtimeDeliveryState);
      phaseTenContractsLoadedRef.current = true;
    } catch (error) {
      setPhaseTenContractsError(error instanceof Error ? error.message : String(error));
    } finally {
      setPhaseTenContractsLoading(false);
    }
  }

  async function handleRefreshUpdaterDiscovery() {
    setPhaseTenUpdaterActionLoading("Refreshing signed discovery");
    setPhaseTenUpdaterActionError(null);

    try {
      const updaterState = await invoke<UpdaterContractState>(
        updaterContractState?.shell_discovery_command_name ?? "check_native_app_updater"
      );

      setUpdaterContractState(updaterState);
      phaseTenContractsLoadedRef.current = true;
    } catch (error) {
      setPhaseTenUpdaterActionError(error instanceof Error ? error.message : String(error));
    } finally {
      setPhaseTenUpdaterActionLoading(null);
    }
  }

  async function handlePrepareUpdater() {
    setPhaseTenUpdaterActionLoading("Preparing verified update");
    setPhaseTenUpdaterActionError(null);

    try {
      const updaterState = await invoke<UpdaterContractState>(
        updaterContractState?.shell_prepare_command_name ?? "prepare_native_app_updater"
      );

      setUpdaterContractState(updaterState);
      phaseTenContractsLoadedRef.current = true;
    } catch (error) {
      setPhaseTenUpdaterActionError(error instanceof Error ? error.message : String(error));
    } finally {
      setPhaseTenUpdaterActionLoading(null);
    }
  }

  async function handleApplyUpdater() {
    setPhaseTenUpdaterActionLoading("Applying verified update and requesting restart");
    setPhaseTenUpdaterActionError(null);

    try {
      const updaterState = await invoke<UpdaterContractState>(
        updaterContractState?.shell_apply_command_name ?? "apply_native_app_updater"
      );

      setUpdaterContractState(updaterState);
      phaseTenContractsLoadedRef.current = true;
    } catch (error) {
      setPhaseTenUpdaterActionError(error instanceof Error ? error.message : String(error));
    } finally {
      setPhaseTenUpdaterActionLoading(null);
    }
  }

  useEffect(() => {
    if (activeScreen !== "diagnostics") {
      return;
    }

    let cancelled = false;

    async function loadPhaseTenContractsEffect() {
      if (cancelled) {
        return;
      }

      await refreshPhaseTenContractsSurface(!phaseTenContractsLoadedRef.current);
    }

    void loadPhaseTenContractsEffect();

    return () => {
      cancelled = true;
    };
  }, [activeScreen]);

  const runState = useRunScreenState({
    activeScreen,
    runStartState: setupState.runStartState,
    planningTokenBudget: setupState.preflightState?.requested_plan?.max_token_budget ?? 12000,
    onNavigateToRun: () => setActiveScreen("run"),
    refreshSetupReadiness: async () => {
      await Promise.all([setupState.refreshPreflightState(), setupState.refreshRunStartState()]);
    }
  });

  const reportsState = useReportsScreenState({
    activeScreen,
    runSessionState: runState.runSessionState
  });

  const diagnosticsViewModel = useDiagnosticsScreenViewModel({
    runtimePersonaState: runState.runtimePersonaState,
    runBoundedMemoryState: runState.runBoundedMemoryState,
    runModeratedRoundsState: runState.runModeratedRoundsState,
    runTokenDurationTrackingState: runState.runTokenDurationTrackingState,
    runLifecycleState: runState.runLifecycleState,
    runStatusState: runState.runStatusState
  });

  const setupViewModel = {
    ...setupState.viewModel,
    runtimePersonaState: runState.runtimePersonaState,
    runBoundedMemoryState: runState.runBoundedMemoryState,
    runModeratedRoundsState: runState.runModeratedRoundsState,
    runTokenDurationTrackingState: runState.runTokenDurationTrackingState,
    runSessionState: runState.runSessionState
  };

  const setupActions: typeof setupState.actions = {
    ...setupState.actions,
    onSeedDocumentChange: (event: ChangeEvent<HTMLInputElement>) => {
      runState.clearRunSessionOutcome();
      setupState.actions.onSeedDocumentChange(event);
    },
    onSelectedProviderChange: (value) => {
      runState.clearRunSessionOutcome();
      setupState.actions.onSelectedProviderChange(value);
    }
  };

  let activeScreenView: ReactNode = null;

  switch (activeScreen) {
    case "setup":
      activeScreenView = <SetupScreen viewModel={setupViewModel} actions={setupActions} />;
      break;
    case "run":
      activeScreenView = (
        <RunScreen
          viewModel={runState.viewModel}
          actions={runState.actions}
          startingRunSession={runState.startingRunSession}
          cancelingRunSession={runState.cancelingRunSession}
          requestingControlledShutdown={runState.requestingControlledShutdown}
        />
      );
      break;
    case "reports":
      activeScreenView = <ReportsScreen viewModel={reportsState.viewModel} actions={reportsState.actions} />;
      break;
    case "diagnostics":
      activeScreenView = <DiagnosticsScreen viewModel={diagnosticsViewModel} />;
      break;
  }

  const screenContent = appScreenContent[activeScreen];
  const heroStats = [
    {
      label: "Preflight",
      value: setupState.preflightState?.run_start_eligible_now
        ? "Eligible for run start"
        : label(setupState.preflightState?.current_state_name ?? "not_started")
    },
    {
      label: "Run",
      value: runState.runStatusState?.current_run_id ?? runState.runSessionState?.run_id ?? "No active run"
    },
    {
      label: "Report",
      value:
        reportsState.viewModel.effectiveAvailabilityStatus === "available"
          ? "Artifact ready"
          : label(reportsState.viewModel.effectiveAvailabilityName)
    }
  ];
  const updaterExecutionMaterialized = updaterContractState
    ? [
        updaterContractState.real_update_check_materialized,
        updaterContractState.real_update_download_materialized,
        updaterContractState.real_update_verification_materialized,
        updaterContractState.real_update_apply_materialized
      ].some(Boolean)
    : undefined;
  const runtimeDeliveryExecutionMaterialized = runtimeDeliveryContractState
    ? [
        runtimeDeliveryContractState.real_authorization_exchange_materialized,
        runtimeDeliveryContractState.real_artifact_acquisition_materialized,
        runtimeDeliveryContractState.real_verification_execution_materialized,
        runtimeDeliveryContractState.real_runtime_apply_materialized
      ].some(Boolean)
    : undefined;

  return (
    <>
      <style>{`
        html, body, #root {
          margin: 0;
          min-height: 100%;
        }

        *, *::before, *::after {
          box-sizing: border-box;
        }
      `}</style>
      <main
        style={{
          minHeight: "100vh",
          padding: "clamp(18px, 3vw, 28px)",
          background:
            "radial-gradient(circle at top left, rgba(246, 219, 173, 0.52), transparent 34%), linear-gradient(180deg, #f8f3ea 0%, #efe5d3 100%)",
          color: "#2f271e",
          fontFamily: '"Trebuchet MS", "Segoe UI", sans-serif'
        }}
      >
        <div style={{ maxWidth: 1220, margin: "0 auto", display: "grid", gap: 22, width: "100%" }}>
          <header
            style={{
              padding: 28,
              borderRadius: 28,
              background:
                "linear-gradient(135deg, rgba(71, 49, 22, 0.96) 0%, rgba(105, 74, 36, 0.94) 100%)",
              color: "#fff8ec",
              boxShadow: "0 24px 54px rgba(43, 30, 11, 0.22)",
              display: "grid",
              gap: 22
            }}
          >
            <div
              style={{
                display: "grid",
                gap: 18,
                gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))",
                alignItems: "start"
              }}
            >
              <div style={{ display: "grid", gap: 12 }}>
                <p
                  style={{
                    margin: 0,
                    letterSpacing: "0.18em",
                    textTransform: "uppercase",
                    fontSize: 12,
                    opacity: 0.9
                  }}
                >
                  {screenContent.eyebrow}
                </p>
                <h1
                  style={{
                    margin: 0,
                    fontSize: "clamp(2.35rem, 6vw, 3.8rem)",
                    lineHeight: 1.02,
                    fontFamily: '"Palatino Linotype", "Book Antiqua", Georgia, serif'
                  }}
                >
                  {screenContent.title}
                </h1>
                <p style={{ margin: 0, maxWidth: 760, lineHeight: 1.65, fontSize: "1.02rem", opacity: 0.94 }}>
                  {screenContent.detail}
                </p>
              </div>

              <div
                style={{
                  display: "grid",
                  gap: 10,
                  padding: 18,
                  borderRadius: 20,
                  background: "rgba(255, 248, 236, 0.1)",
                  border: "1px solid rgba(255, 241, 220, 0.16)"
                }}
              >
                <div style={{ fontSize: 12, letterSpacing: "0.12em", textTransform: "uppercase", opacity: 0.85 }}>
                  Current desktop focus
                </div>
                <div style={{ fontSize: "1.35rem", fontWeight: 700 }}>{screenContent.label}</div>
                <div style={{ lineHeight: 1.6, opacity: 0.92 }}>
                  The desktop shell now separates setup, execution, report access, and diagnostics into dedicated
                  surfaces so the product flow reads as a workflow instead of a contract dump.
                </div>
              </div>
            </div>

            <section
              style={{
                display: "grid",
                gap: 12,
                gridTemplateColumns: "repeat(auto-fit, minmax(180px, 1fr))"
              }}
            >
              {heroStats.map((stat) => (
                <div
                  key={stat.label}
                  style={{
                    padding: 16,
                    borderRadius: 18,
                    background: "rgba(255, 248, 236, 0.08)",
                    border: "1px solid rgba(255, 241, 220, 0.16)",
                    display: "grid",
                    gap: 6
                  }}
                >
                  <div style={{ fontSize: 12, letterSpacing: "0.1em", textTransform: "uppercase", opacity: 0.82 }}>
                    {stat.label}
                  </div>
                  <div style={{ fontSize: "1.08rem", fontWeight: 700 }}>{stat.value}</div>
                </div>
              ))}
            </section>
          </header>

          <nav
            aria-label="Primary"
            style={{
              display: "flex",
              gap: 10,
              flexWrap: "wrap",
              padding: 14,
              borderRadius: 20,
              background: "rgba(255, 251, 244, 0.88)",
              border: "1px solid #dfd3bf",
              boxShadow: "0 10px 26px rgba(82, 58, 25, 0.06)"
            }}
          >
            {(Object.keys(appScreenContent) as AppScreen[]).map((screen) => {
              const active = activeScreen === screen;

              return (
                <button
                  key={screen}
                  type="button"
                  onClick={() => setActiveScreen(screen)}
                  aria-pressed={active}
                  style={
                    active
                      ? {
                          ...buttonStyle("primary"),
                          padding: "10px 18px"
                        }
                      : {
                          ...buttonStyle("secondary"),
                          padding: "10px 18px"
                        }
                  }
                >
                  {appScreenContent[screen].label}
                </button>
              );
            })}
          </nav>

          {activeScreen === "diagnostics" ? (
            <section
              aria-labelledby="phase-10-contract-foundation"
              style={{
                display: "grid",
                gap: 18,
                padding: 24,
                borderRadius: 26,
                background: "rgba(255, 251, 244, 0.92)",
                border: "1px solid #ddcfb7",
                boxShadow: "0 12px 28px rgba(82, 58, 25, 0.08)"
              }}
            >
              <div style={{ display: "grid", gap: 8 }}>
                <div
                  style={{
                    fontSize: 12,
                    letterSpacing: "0.12em",
                    textTransform: "uppercase",
                    color: "#74552b"
                  }}
                >
                  Phase 10 Contract Foundation
                </div>
                <h2
                  id="phase-10-contract-foundation"
                  style={{ margin: 0, fontSize: "clamp(1.35rem, 3vw, 1.9rem)", lineHeight: 1.12 }}
                >
                  Updater and runtime-delivery truth
                </h2>
                <p style={{ margin: 0, lineHeight: 1.6, maxWidth: 820, color: "#57452a" }}>
                  These diagnostics come from shell-owned Phase 10 contract surfaces only. They now include
                  real signed updater discovery, explicit download-and-verification preparation truth, and
                  explicit shell-owned update apply plus restart-request execution.
                </p>
              </div>

              <div style={{ display: "flex", flexWrap: "wrap", gap: 12 }}>
                <button
                  type="button"
                  style={buttonStyle("secondary", phaseTenContractsLoading || phaseTenUpdaterActionLoading !== null)}
                  onClick={() => {
                    void handleRefreshUpdaterDiscovery();
                  }}
                  disabled={phaseTenContractsLoading || phaseTenUpdaterActionLoading !== null}
                >
                  Refresh updater discovery
                </button>
                <button
                  type="button"
                  style={buttonStyle(
                    "primary",
                    phaseTenContractsLoading ||
                      phaseTenUpdaterActionLoading !== null ||
                      updaterContractState?.discovery_configuration_ready === false
                  )}
                  onClick={() => {
                    void handlePrepareUpdater();
                  }}
                  disabled={
                    phaseTenContractsLoading ||
                    phaseTenUpdaterActionLoading !== null ||
                    updaterContractState?.discovery_configuration_ready === false
                  }
                >
                  Prepare verified update
                </button>
                <button
                  type="button"
                  style={buttonStyle(
                    "primary",
                    phaseTenContractsLoading ||
                      phaseTenUpdaterActionLoading !== null ||
                      updaterContractState?.apply_allowed_now !== true
                  )}
                  onClick={() => {
                    void handleApplyUpdater();
                  }}
                  disabled={
                    phaseTenContractsLoading ||
                    phaseTenUpdaterActionLoading !== null ||
                    updaterContractState?.apply_allowed_now !== true
                  }
                >
                  Apply verified update and restart
                </button>
              </div>

              {phaseTenContractsLoading ? (
                <div
                  role="status"
                  style={{
                    padding: 16,
                    borderRadius: 18,
                    background: "#f6ecda",
                    border: "1px solid #e0cfb1",
                    color: "#5d4825"
                  }}
                >
                  Loading shell-owned updater and runtime-delivery contract truth.
                </div>
              ) : null}

              {phaseTenUpdaterActionLoading ? (
                <div
                  role="status"
                  style={{
                    padding: 16,
                    borderRadius: 18,
                    background: "#f6ecda",
                    border: "1px solid #e0cfb1",
                    color: "#5d4825"
                  }}
                >
                  {phaseTenUpdaterActionLoading}.
                </div>
              ) : null}

              {phaseTenContractsError ? (
                <div
                  role="status"
                  style={{
                    padding: 16,
                    borderRadius: 18,
                    background: "#f8ddd0",
                    border: "1px solid #e6b8a5",
                    color: "#6b2d17"
                  }}
                >
                  Unable to load Phase 10 contract truth: {phaseTenContractsError}
                </div>
              ) : null}

              {phaseTenUpdaterActionError ? (
                <div
                  role="status"
                  style={{
                    padding: 16,
                    borderRadius: 18,
                    background: "#f8ddd0",
                    border: "1px solid #e6b8a5",
                    color: "#6b2d17"
                  }}
                >
                  Unable to complete updater action: {phaseTenUpdaterActionError}
                </div>
              ) : null}

              {updaterContractState && runtimeDeliveryContractState ? (
                <div
                  style={{
                    display: "grid",
                    gap: 16,
                    gridTemplateColumns: "repeat(auto-fit, minmax(260px, 1fr))"
                  }}
                >
                  <article
                    style={{
                      display: "grid",
                      gap: 12,
                      padding: 18,
                      borderRadius: 22,
                      background: "linear-gradient(180deg, #fff8ee 0%, #f4e8d2 100%)",
                      border: "1px solid #dcc8a6"
                    }}
                  >
                    <div style={{ display: "grid", gap: 4 }}>
                      <div style={{ fontSize: 12, letterSpacing: "0.1em", textTransform: "uppercase" }}>
                        {updaterContractState.contract_name ?? "UpdaterLifecycleContract"}
                      </div>
                      <div style={{ fontSize: "1.2rem", fontWeight: 700 }}>Updater lifecycle contract</div>
                    </div>
                    <div style={{ display: "grid", gap: 8, lineHeight: 1.55 }}>
                      <div>Current state: {updaterContractState.current_state_name ?? "Unknown"}</div>
                      <div>
                        Discovery command: {updaterContractState.shell_discovery_command_name ?? "Unknown"}
                      </div>
                      <div>
                        Prepare command: {updaterContractState.shell_prepare_command_name ?? "Unknown"}
                      </div>
                      <div>Apply command: {updaterContractState.shell_apply_command_name ?? "Unknown"}</div>
                      <div>
                        Signed config ready: {truthText(updaterContractState.discovery_configuration_ready)}
                      </div>
                      <div>
                        Configured endpoints: {updaterContractState.discovery_endpoint_count ?? "Unknown"}
                      </div>
                      <div>
                        Signing public key configured: {truthText(updaterContractState.discovery_pubkey_configured)}
                      </div>
                      <div>
                        Discovery allowed during active run: {truthText(
                          updaterContractState.discovery_allowed_during_active_run
                        )}
                      </div>
                      <div>
                        Availability remains distinct from ready: {truthText(updaterContractState.available_state_distinct_from_ready)}
                      </div>
                      <div>
                        Rejection remains distinct from error: {truthText(updaterContractState.rejection_distinct_from_error)}
                      </div>
                      <div>
                        Verification required before ready: {truthText(updaterContractState.verification_required_before_ready)}
                      </div>
                      <div>
                        Active-run apply forbidden: {truthText(updaterContractState.active_run_apply_forbidden)}
                      </div>
                      <div>
                        Apply deferred during active run: {truthText(
                          updaterContractState.apply_deferred_during_active_run
                        )}
                      </div>
                      <div>Apply allowed now: {truthText(updaterContractState.apply_allowed_now)}</div>
                      <div>
                        Apply blocked by active run now: {truthText(
                          updaterContractState.apply_blocked_by_active_run_now
                        )}
                      </div>
                      <div>
                        Apply blocked because update not ready now: {truthText(
                          updaterContractState.apply_blocked_because_update_not_ready_now
                        )}
                      </div>
                      <div>
                        Restart coordination required after apply: {truthText(
                          updaterContractState.restart_coordination_required_after_apply
                        )}
                      </div>
                      <div>
                        Last check attempted: {truthText(updaterContractState.last_check_attempted)}
                      </div>
                      <div>
                        Update available now: {truthText(updaterContractState.current_update_available)}
                      </div>
                      <div>
                        Update ready now: {truthText(updaterContractState.current_update_ready)}
                      </div>
                      <div>
                        Candidate rejected now: {truthText(updaterContractState.current_candidate_rejected)}
                      </div>
                      <div>
                        Error active now: {truthText(updaterContractState.current_error_active)}
                      </div>
                      <div>
                        Active run detected during last check: {truthText(
                          updaterContractState.active_run_detected_during_last_check
                        )}
                      </div>
                      {updaterContractState.last_discovered_version ? (
                        <div>Available version: {updaterContractState.last_discovered_version}</div>
                      ) : null}
                      <div>Any real updater execution materialized: {truthText(updaterExecutionMaterialized)}</div>
                      <div>
                        Current status note: {updaterContractState.current_status_note ?? "Unavailable"}
                      </div>
                      <div>
                        Discovery config note: {updaterContractState.discovery_configuration_note ?? "Unavailable"}
                      </div>
                      <div>Apply gate note: {updaterContractState.apply_status_note ?? "Unavailable"}</div>
                      {updaterContractState.last_error_detail ? (
                        <div>Last error detail: {updaterContractState.last_error_detail}</div>
                      ) : null}
                      {updaterContractState.last_discovered_release_notes ? (
                        <div>Release notes: {updaterContractState.last_discovered_release_notes}</div>
                      ) : null}
                      {updaterContractState.last_discovered_publication_date ? (
                        <div>Publication date: {updaterContractState.last_discovered_publication_date}</div>
                      ) : null}
                    </div>
                  </article>

                  <article
                    style={{
                      display: "grid",
                      gap: 12,
                      padding: 18,
                      borderRadius: 22,
                      background: "linear-gradient(180deg, #fff9f2 0%, #ece1c8 100%)",
                      border: "1px solid #d8c5a2"
                    }}
                  >
                    <div style={{ display: "grid", gap: 4 }}>
                      <div style={{ fontSize: 12, letterSpacing: "0.1em", textTransform: "uppercase" }}>
                        {runtimeDeliveryContractState.contract_name ?? "ManagedRuntimeDeliveryContract"}
                      </div>
                      <div style={{ fontSize: "1.2rem", fontWeight: 700 }}>Runtime delivery contract</div>
                    </div>
                    <div style={{ display: "grid", gap: 8, lineHeight: 1.55 }}>
                      <div>
                        Runtime package contract: {runtimeDeliveryContractState.runtime_package_contract_name ?? "Unknown"}
                      </div>
                      <div>
                        Authorization basis defined: {truthText(runtimeDeliveryContractState.authorization_basis_defined)}
                      </div>
                      <div>
                        Compatibility basis defined: {truthText(runtimeDeliveryContractState.compatibility_basis_defined)}
                      </div>
                      <div>
                        Verification basis defined: {truthText(runtimeDeliveryContractState.verification_basis_defined)}
                      </div>
                      <div>
                        Runtime use blocked until verified: {truthText(runtimeDeliveryContractState.runtime_use_blocked_until_verified)}
                      </div>
                      <div>
                        Active-run mutation forbidden: {truthText(
                          runtimeDeliveryContractState.runtime_delivery_mutation_during_active_run_forbidden
                        )}
                      </div>
                      <div>
                        Any real runtime delivery execution materialized: {truthText(runtimeDeliveryExecutionMaterialized)}
                      </div>
                    </div>
                  </article>
                </div>
              ) : null}
            </section>
          ) : null}

          {activeScreenView}
        </div>
      </main>
    </>
  );
}