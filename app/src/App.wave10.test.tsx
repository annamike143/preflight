import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import App from "./App";

const { invokeMock } = vi.hoisted(() => ({
  invokeMock: vi.fn()
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: invokeMock
}));

function configureInvokeMock() {
  invokeMock.mockImplementation(async (command: string) => {
    switch (command) {
      case "provider_configuration_state":
        return {
          configuration_present: false,
          current_status: "required",
          current_detail: "selection_missing",
          selected_provider: null
        };
      case "preflight_readiness_state":
        return {
          current_state_name: "PreflightNotStarted",
          run_start_eligible_now: false,
          run_start_blocked_without_eligible_preflight: true,
          current_result_stale: false,
          invalidation_reasons: []
        };
      case "run_start_readiness_state":
        return {
          current_state_name: "RunStartFoundationNotStarted",
          real_run_start_authorized_now: false
        };
      case "runtime_persona_contract_state":
        return {
          contract_name: "RuntimePersonaContract",
          exactly_one_moderator_required: true,
          stable_persona_labels_required: true
        };
      case "run_bounded_memory_state":
        return {
          memory_surface_name: "RunBoundedMemory",
          transcript_archive_separate_from_prompt_memory: true
        };
      case "run_moderated_rounds_state":
        return {
          surface_name: "RunModeratedRounds",
          normal_round_min_speakers: 3
        };
      case "run_token_duration_tracking_state":
        return {
          surface_name: "RunTokenDurationTracking",
          integrated_into_execution_path: true
        };
      case "run_session_state":
        return {
          current_state_name: "RunDraft"
        };
      case "check_native_app_updater":
      case "updater_contract_state":
        return {
          contract_name: "UpdaterLifecycleContract",
          shell_discovery_command_name: "check_native_app_updater",
          shell_prepare_command_name: "prepare_native_app_updater",
          shell_apply_command_name: "apply_native_app_updater",
          current_state_name: "NoUpdateAvailable",
          current_status_note:
            "The shell completed a signed native updater discovery check and no app update is currently available.",
          current_update_available: false,
          current_update_ready: false,
          current_candidate_rejected: false,
          current_error_active: false,
          discovery_configuration_ready: true,
          discovery_endpoints_configured: true,
          discovery_pubkey_configured: true,
          discovery_endpoint_count: 1,
          discovery_configuration_note:
            "Signed native app-updater discovery is configured through MIRO_FISH_APP_UPDATER_ENDPOINTS and MIRO_FISH_APP_UPDATER_PUBKEY.",
          discovery_allowed_during_active_run: true,
          available_state_distinct_from_ready: true,
          rejection_distinct_from_error: true,
          verification_required_before_ready: true,
          active_run_apply_forbidden: true,
          apply_deferred_during_active_run: true,
          apply_allowed_now: false,
          apply_blocked_by_active_run_now: false,
          apply_blocked_because_update_not_ready_now: true,
          restart_coordination_required_after_apply: false,
          apply_status_note:
            "Update apply remains blocked until the shell reaches a verified UpdateReady candidate.",
          active_run_detected_during_last_check: false,
          last_check_attempted: true,
          last_error_detail: null,
          last_discovered_version: null,
          last_discovered_release_notes: null,
          last_discovered_publication_date: null,
          real_update_check_materialized: true,
          real_update_download_materialized: false,
          real_update_verification_materialized: false,
          real_update_apply_materialized: false
        };
      case "prepare_native_app_updater":
        return {
          contract_name: "UpdaterLifecycleContract",
          shell_discovery_command_name: "check_native_app_updater",
          shell_prepare_command_name: "prepare_native_app_updater",
          shell_apply_command_name: "apply_native_app_updater",
          current_state_name: "UpdateReady",
          current_status_note:
            "The shell downloaded and verified app version 1.2.3 (4096 bytes) and promoted it to UpdateReady for explicit shell-owned apply.",
          current_update_available: true,
          current_update_ready: true,
          current_candidate_rejected: false,
          current_error_active: false,
          discovery_configuration_ready: true,
          discovery_endpoints_configured: true,
          discovery_pubkey_configured: true,
          discovery_endpoint_count: 1,
          discovery_configuration_note:
            "Signed native app-updater discovery is configured through MIRO_FISH_APP_UPDATER_ENDPOINTS and MIRO_FISH_APP_UPDATER_PUBKEY.",
          discovery_allowed_during_active_run: true,
          available_state_distinct_from_ready: true,
          rejection_distinct_from_error: true,
          verification_required_before_ready: true,
          active_run_apply_forbidden: true,
          apply_deferred_during_active_run: true,
          apply_allowed_now: true,
          apply_blocked_by_active_run_now: false,
          apply_blocked_because_update_not_ready_now: false,
          restart_coordination_required_after_apply: true,
          apply_status_note:
            "A verified app update is ready for explicit shell-owned apply and restart coordination.",
          active_run_detected_during_last_check: false,
          last_check_attempted: true,
          last_error_detail: null,
          last_discovered_version: "1.2.3",
          last_discovered_release_notes: "Fresh improvements",
          last_discovered_publication_date: "2026-04-17T12:00:00Z",
          real_update_check_materialized: true,
          real_update_download_materialized: true,
          real_update_verification_materialized: true,
          real_update_apply_materialized: false
        };
      case "apply_native_app_updater":
        return {
          contract_name: "UpdaterLifecycleContract",
          shell_discovery_command_name: "check_native_app_updater",
          shell_prepare_command_name: "prepare_native_app_updater",
          shell_apply_command_name: "apply_native_app_updater",
          current_state_name: "UpdateApplied",
          current_status_note:
            "The shell applied verified app version 1.2.3 (4096 bytes) and requested app restart to activate the new build.",
          current_update_available: false,
          current_update_ready: false,
          current_candidate_rejected: false,
          current_error_active: false,
          discovery_configuration_ready: true,
          discovery_endpoints_configured: true,
          discovery_pubkey_configured: true,
          discovery_endpoint_count: 1,
          discovery_configuration_note:
            "Signed native app-updater discovery is configured through MIRO_FISH_APP_UPDATER_ENDPOINTS and MIRO_FISH_APP_UPDATER_PUBKEY.",
          discovery_allowed_during_active_run: true,
          available_state_distinct_from_ready: true,
          rejection_distinct_from_error: true,
          verification_required_before_ready: true,
          active_run_apply_forbidden: true,
          apply_deferred_during_active_run: true,
          apply_allowed_now: false,
          apply_blocked_by_active_run_now: false,
          apply_blocked_because_update_not_ready_now: false,
          restart_coordination_required_after_apply: true,
          apply_status_note:
            "The shell applied the verified app update and requested app restart to activate the new build.",
          active_run_detected_during_last_check: false,
          last_check_attempted: true,
          last_error_detail: null,
          last_discovered_version: "1.2.3",
          last_discovered_release_notes: "Fresh improvements",
          last_discovered_publication_date: "2026-04-17T12:00:00Z",
          real_update_check_materialized: true,
          real_update_download_materialized: true,
          real_update_verification_materialized: true,
          real_update_apply_materialized: true
        };
      case "runtime_delivery_contract_state":
        return {
          contract_name: "ManagedRuntimeDeliveryContract",
          runtime_package_contract_name: "ManagedRuntimePackageContract",
          authorization_basis_defined: true,
          compatibility_basis_defined: true,
          verification_basis_defined: true,
          runtime_use_blocked_until_verified: true,
          runtime_delivery_mutation_during_active_run_forbidden: true,
          real_authorization_exchange_materialized: false,
          real_artifact_acquisition_materialized: false,
          real_verification_execution_materialized: false,
          real_runtime_apply_materialized: false
        };
      default:
        return {};
    }
  });
}

describe("App Wave 10 diagnostics", () => {
  beforeEach(() => {
    invokeMock.mockReset();
    configureInvokeMock();
  });

  it("renders the shell-owned updater and runtime-delivery contract foundation on the diagnostics screen", async () => {
    render(<App />);

    fireEvent.click(screen.getByRole("button", { name: "Diagnostics" }));

    await waitFor(() => {
      expect(invokeMock).toHaveBeenCalledWith("check_native_app_updater");
      expect(invokeMock).toHaveBeenCalledWith("runtime_delivery_contract_state");
    });

    expect(screen.getByText("Phase 10 Contract Foundation")).toBeTruthy();
    expect(screen.getByText("Updater and runtime-delivery truth")).toBeTruthy();
    expect(screen.getByText("Updater lifecycle contract")).toBeTruthy();
    expect(screen.getByText("Runtime delivery contract")).toBeTruthy();
    expect(screen.getByText("Current state: NoUpdateAvailable")).toBeTruthy();
    expect(screen.getByText("Discovery command: check_native_app_updater")).toBeTruthy();
    expect(screen.getByText("Prepare command: prepare_native_app_updater")).toBeTruthy();
    expect(screen.getByText("Apply command: apply_native_app_updater")).toBeTruthy();
    expect(screen.getByText("Signed config ready: Yes")).toBeTruthy();
    expect(screen.getByText("Runtime package contract: ManagedRuntimePackageContract")).toBeTruthy();
    expect(screen.getByText("Any real updater execution materialized: Yes")).toBeTruthy();
    expect(screen.getByText("Any real runtime delivery execution materialized: No")).toBeTruthy();
  });

  it("lets diagnostics promote an available updater candidate into UpdateReady through the explicit prepare command", async () => {
    render(<App />);

    fireEvent.click(screen.getByRole("button", { name: "Diagnostics" }));

    await waitFor(() => {
      expect(screen.getByText("Prepare verified update")).toBeTruthy();
    });

    fireEvent.click(screen.getByRole("button", { name: "Prepare verified update" }));

    await waitFor(() => {
      expect(invokeMock).toHaveBeenCalledWith("prepare_native_app_updater");
    });

    expect(screen.getByText("Current state: UpdateReady")).toBeTruthy();
    expect(screen.getByText("Update ready now: Yes")).toBeTruthy();
    expect(
      screen.getByText(
        "Apply gate note: A verified app update is ready for explicit shell-owned apply and restart coordination."
      )
    ).toBeTruthy();
  });

  it("lets diagnostics apply a verified updater candidate and surface restart-request truth", async () => {
    render(<App />);

    fireEvent.click(screen.getByRole("button", { name: "Diagnostics" }));

    await waitFor(() => {
      expect(screen.getByText("Prepare verified update")).toBeTruthy();
    });

    fireEvent.click(screen.getByRole("button", { name: "Prepare verified update" }));

    await waitFor(() => {
      expect(invokeMock).toHaveBeenCalledWith("prepare_native_app_updater");
    });

    fireEvent.click(screen.getByRole("button", { name: "Apply verified update and restart" }));

    await waitFor(() => {
      expect(invokeMock).toHaveBeenCalledWith("apply_native_app_updater");
    });

    expect(screen.getByText("Current state: UpdateApplied")).toBeTruthy();
    expect(screen.getByText("Apply allowed now: No")).toBeTruthy();
    expect(
      screen.getByText(
        "Apply gate note: The shell applied the verified app update and requested app restart to activate the new build."
      )
    ).toBeTruthy();
  });
});