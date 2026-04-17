#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod activation_contract;
mod activation_local_validation;
mod activation_readiness;
mod activation_secure_store;
mod activation_storage_boundary;
mod activation_submission;
mod activation_support_store;
mod abnormal_termination_residual;
mod app_session_trust_boundary;
mod app_lifecycle;
mod cleanup_scope_mapping;
mod controlled_shutdown;
mod engine_supervision;
mod local_interface_contract;
mod native_app_updater;
mod preflight_contract;
mod preflight_document_validation;
mod preflight_effective_plan;
mod preflight_feasibility;
mod preflight_readiness;
mod preflight_submission;
mod provider_configuration_contract;
mod provider_configuration_submission;
mod provider_readiness;
mod provider_secure_store;
mod provider_storage_boundary;
mod provider_support_store;
mod report_access_contract;
mod report_output_contract;
mod report_storage_boundary;
mod run_bounded_memory;
mod run_cleanup_contract;
mod run_cleanup_execution;
mod run_event_contract;
mod run_event_history;
mod run_finalization_contract;
mod run_lifecycle_state;
mod run_moderated_rounds;
mod run_session_contract;
mod run_session_start;
mod run_seed_ingestion;
mod run_status_contract;
mod run_status_projection;
mod run_start_lineage;
mod run_start_readiness;
mod run_token_duration_tracking;
mod run_workspace_allocation;
mod run_workspace_boundary;
mod runtime_persona_contract;
mod runtime_readiness;
mod runtime_package_contract;
mod runtime_delivery_contract;
mod runtime_presence;
mod runtime_hydration;
mod runtime_integrity;
mod startup_checks;
mod startup_janitor_contract;
mod startup_janitor_execution;
mod updater_contract;

use activation_contract::activation_contract_basis;
use activation_local_validation::{
    activation_local_validation_surface, commit_approved_activation, current_device_binding_hash,
    LocalActivationApprovalPayload,
};
use activation_readiness::activation_readiness_truth_surface;
use activation_secure_store::{activation_secure_store_surface, ActivationSecureStore};
use activation_storage_boundary::activation_storage_boundary_surface;
use activation_submission::{
    submit_device_activation_from_shell, ActivationSubmissionOutcome,
    ActivationSubmissionRequest,
};
use activation_support_store::{activation_support_store_surface, ActivationSupportStore};
use app_session_trust_boundary::app_session_trust_boundary_scaffold;
use app_lifecycle::app_lifecycle_truth_surface;
use cleanup_scope_mapping::{
    cleanup_scope_mapping_state as cleanup_scope_mapping_truth_surface,
    CleanupScopeMappingSnapshot,
};
use controlled_shutdown::controlled_shutdown_scaffold;
use controlled_shutdown::ControlledShutdownSnapshot;
use engine_supervision::engine_supervision_scaffold;
use local_interface_contract::local_interface_contract_scaffold;
use native_app_updater::{
    apply_prepared_native_app_updater_state, native_app_updater_apply_gate_state,
    prepare_native_app_updater_state, refresh_native_app_updater_state,
};
use preflight_contract::preflight_contract_basis;
use preflight_readiness::{preflight_readiness_truth_surface, PreflightReadinessSnapshot};
use preflight_submission::{
    accept_preflight_downgrade_from_shell, submit_preflight_from_shell,
    PreflightDowngradeAcceptanceOutcome, PreflightSubmissionOutcome,
    PreflightSubmissionRequest,
};
use provider_configuration_contract::provider_configuration_contract_basis;
use provider_configuration_submission::{
    provider_configuration_state_from_shell, submit_provider_configuration_from_shell,
    ProviderConfigurationStateSnapshot, ProviderConfigurationSubmissionOutcome,
    ProviderConfigurationSubmissionRequest,
};
use provider_readiness::provider_readiness_truth_surface;
use provider_storage_boundary::provider_storage_boundary_surface;
use report_access_contract::{
    export_local_report_pdf_from_shell, open_local_report_pdf_from_shell,
    report_access_metadata_state as report_access_metadata_truth_surface,
    ReportAccessExecutionOutcome,
    report_access_contract_state as report_access_contract_truth_surface,
    ReportAccessContractSnapshot, ReportAccessMetadataSnapshot,
};
use report_output_contract::{
    report_output_contract_state as report_output_contract_truth_surface,
    ReportOutputContractSnapshot,
};
use report_storage_boundary::{
    report_storage_boundary_state as report_storage_boundary_truth_surface,
    ReportStorageBoundarySnapshot,
};
use run_bounded_memory::{run_bounded_memory_surface, RunBoundedMemorySnapshot};
use run_cleanup_contract::{
    run_cleanup_contract_state as run_cleanup_contract_truth_surface,
    RunCleanupContractSnapshot,
};
use run_event_contract::{
    run_event_contract_state as run_event_contract_truth_surface, RunEventContractSnapshot,
};
use run_event_history::{
    run_event_history_from_shell, RunEventHistoryRequest, RunEventHistorySnapshot,
};
use run_finalization_contract::{
    run_finalization_contract_state as run_finalization_contract_truth_surface,
    RunFinalizationContractSnapshot,
};
use run_lifecycle_state::{run_lifecycle_state_from_shell, RunLifecycleStateSnapshot};
use run_moderated_rounds::{run_moderated_rounds_surface, RunModeratedRoundsSnapshot};
use run_session_contract::run_session_contract_basis;
use run_session_start::{
    cancel_run_session_from_shell, request_controlled_shutdown_from_shell,
    run_session_state_from_shell, start_run_session_from_shell, ControlledShutdownRequestOutcome,
    RunCancellationOutcome, RunSessionStartOutcome, RunSessionStateSnapshot,
};
use run_status_contract::{run_status_contract_state, RunStatusContractSnapshot};
use run_start_lineage::run_start_lineage_surface;
use run_start_readiness::{run_start_readiness_truth_surface, RunStartReadinessSnapshot};
use run_token_duration_tracking::{
    run_token_duration_tracking_surface, RunTokenDurationTrackingSnapshot,
};
use run_workspace_allocation::default_run_workspace_root;
use run_workspace_boundary::run_workspace_boundary_surface;
use runtime_persona_contract::{
    runtime_persona_contract_surface, RuntimePersonaContractSnapshot,
};
use runtime_delivery_contract::{
    runtime_delivery_contract_state as runtime_delivery_contract_truth_surface,
    RuntimeDeliveryContractSnapshot,
};
use runtime_readiness::runtime_readiness_truth_surface;
use runtime_package_contract::runtime_package_contract_basis;
use runtime_hydration::{hydrate_managed_runtime_from_shell, runtime_hydration_state};
use runtime_integrity::runtime_integrity_state;
use runtime_presence::runtime_presence_state;
use startup_checks::startup_checks_scaffold;
use startup_janitor_contract::{
    startup_janitor_contract_state as startup_janitor_contract_truth_surface,
    StartupJanitorContractSnapshot,
};
use std::time::{SystemTime, UNIX_EPOCH};
use updater_contract::{
    updater_contract_state as updater_contract_truth_surface, UpdaterContractSnapshot,
};

#[tauri::command]
fn placeholder_shell_boundary_note() -> &'static str {
    "Placeholder only: privileged local orchestration belongs to the Rust/Tauri shell, but no privileged product behavior is implemented in this packet."
}

#[tauri::command]
fn placeholder_engine_boundary_note() -> &'static str {
    "Placeholder only: the Python engine is not implemented in this packet and future shell-to-engine supervision remains non-functional scaffolding only."
}

#[tauri::command]
fn activation_contract_basis_note() -> String {
    activation_contract_basis().summary()
}

#[tauri::command]
fn activation_secure_store_surface_note() -> String {
    activation_secure_store_surface().summary()
}

#[tauri::command]
fn activation_support_store_surface_note() -> String {
    activation_support_store_surface().summary()
}

#[tauri::command]
fn activation_local_validation_surface_note() -> String {
    activation_local_validation_surface().summary()
}

#[tauri::command]
fn commit_activation_approval_locally(
    payload: LocalActivationApprovalPayload,
) -> Result<String, String> {
    let secure_store = ActivationSecureStore::os_native();
    let support_store = ActivationSupportStore::default_store()?;
    let current_device_hash = current_device_binding_hash()?;
    let current_timestamp_epoch_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_secs();

    commit_approved_activation(
        &payload,
        &secure_store,
        &support_store,
        &current_device_hash,
        current_timestamp_epoch_seconds,
    )
    .map(|result| result.summary())
}

#[tauri::command]
fn submit_device_activation(
    request: ActivationSubmissionRequest,
) -> ActivationSubmissionOutcome {
    submit_device_activation_from_shell(&request)
}

#[tauri::command]
fn activation_storage_boundary_surface_note() -> String {
    activation_storage_boundary_surface().summary()
}

#[tauri::command]
fn activation_readiness_truth_surface_note() -> String {
    activation_readiness_truth_surface().summary()
}

#[tauri::command]
fn provider_configuration_contract_note() -> String {
    provider_configuration_contract_basis().summary()
}

#[tauri::command]
fn provider_storage_boundary_note() -> String {
    provider_storage_boundary_surface().summary()
}

#[tauri::command]
fn provider_readiness_truth_surface_note() -> String {
    provider_readiness_truth_surface().summary()
}

#[tauri::command]
fn preflight_contract_note() -> String {
    preflight_contract_basis().summary()
}

#[tauri::command]
fn preflight_readiness_truth_surface_note() -> String {
    preflight_readiness_truth_surface().summary()
}

#[tauri::command]
fn preflight_readiness_state() -> PreflightReadinessSnapshot {
    preflight_readiness_truth_surface()
}

#[tauri::command]
fn run_session_contract_note() -> String {
    run_session_contract_basis().summary()
}

#[tauri::command]
fn run_workspace_boundary_note() -> String {
    run_workspace_boundary_surface().summary()
}

#[tauri::command]
fn run_start_lineage_note() -> String {
    run_start_lineage_surface().summary()
}

#[tauri::command]
fn run_start_readiness_truth_surface_note() -> String {
    run_start_readiness_truth_surface().summary()
}

#[tauri::command]
fn run_start_readiness_state() -> RunStartReadinessSnapshot {
    run_start_readiness_truth_surface()
}

#[tauri::command]
fn runtime_persona_contract_note() -> String {
    runtime_persona_contract_surface().summary()
}

#[tauri::command]
fn runtime_persona_contract_state() -> RuntimePersonaContractSnapshot {
    runtime_persona_contract_surface()
}

#[tauri::command]
fn run_bounded_memory_note() -> String {
    run_bounded_memory_surface().summary()
}

#[tauri::command]
fn run_bounded_memory_state() -> RunBoundedMemorySnapshot {
    run_bounded_memory_surface()
}

#[tauri::command]
fn run_moderated_rounds_note() -> String {
    run_moderated_rounds_surface().summary()
}

#[tauri::command]
fn run_moderated_rounds_state() -> RunModeratedRoundsSnapshot {
    run_moderated_rounds_surface()
}

#[tauri::command]
fn run_token_duration_tracking_note() -> String {
    run_token_duration_tracking_surface().summary()
}

#[tauri::command]
fn run_token_duration_tracking_state() -> RunTokenDurationTrackingSnapshot {
    run_token_duration_tracking_surface()
}

#[tauri::command]
fn run_lifecycle_state() -> RunLifecycleStateSnapshot {
    run_lifecycle_state_from_shell()
}

#[tauri::command]
fn run_status_state() -> RunStatusContractSnapshot {
    run_status_contract_state()
}

#[tauri::command]
fn run_cleanup_contract_state() -> RunCleanupContractSnapshot {
    run_cleanup_contract_truth_surface()
}

#[tauri::command]
fn cleanup_scope_mapping_state() -> CleanupScopeMappingSnapshot {
    cleanup_scope_mapping_truth_surface()
}

#[tauri::command]
fn startup_janitor_contract_state() -> StartupJanitorContractSnapshot {
    startup_janitor_contract_truth_surface()
}

#[tauri::command]
fn run_event_contract_state() -> RunEventContractSnapshot {
    run_event_contract_truth_surface()
}

#[tauri::command]
fn run_finalization_contract_state() -> RunFinalizationContractSnapshot {
    run_finalization_contract_truth_surface()
}

#[tauri::command]
fn report_output_contract_state() -> ReportOutputContractSnapshot {
    report_output_contract_truth_surface()
}

#[tauri::command]
fn report_storage_boundary_state() -> ReportStorageBoundarySnapshot {
    report_storage_boundary_truth_surface()
}

#[tauri::command]
fn report_access_contract_state() -> ReportAccessContractSnapshot {
    report_access_contract_truth_surface()
}

#[tauri::command]
fn report_access_metadata_state() -> ReportAccessMetadataSnapshot {
    report_access_metadata_truth_surface()
}

#[tauri::command]
fn open_local_report_pdf() -> ReportAccessExecutionOutcome {
    open_local_report_pdf_from_shell()
}

#[tauri::command]
fn export_local_report_pdf() -> ReportAccessExecutionOutcome {
    export_local_report_pdf_from_shell()
}

#[tauri::command]
fn run_event_history(request: Option<RunEventHistoryRequest>) -> RunEventHistorySnapshot {
    run_event_history_from_shell(request.unwrap_or_default())
}

#[tauri::command]
fn run_session_state() -> RunSessionStateSnapshot {
    run_session_state_from_shell()
}

#[tauri::command]
fn run_session_state_note() -> String {
    run_session_state_from_shell().detail_message
}

#[tauri::command]
fn start_run_session() -> RunSessionStartOutcome {
    start_run_session_from_shell()
}

#[tauri::command]
fn cancel_run_session() -> RunCancellationOutcome {
    cancel_run_session_from_shell()
}

#[tauri::command]
fn controlled_shutdown_state() -> ControlledShutdownSnapshot {
    controlled_shutdown_scaffold()
}

#[tauri::command]
fn request_controlled_shutdown() -> ControlledShutdownRequestOutcome {
    request_controlled_shutdown_from_shell()
}

#[tauri::command]
fn run_workspace_root_note() -> String {
    format!(
        "Fresh run workspaces allocate beneath {} until later cleanup hardening is implemented",
        default_run_workspace_root().to_string_lossy()
    )
}

#[tauri::command]
fn submit_preflight(request: PreflightSubmissionRequest) -> PreflightSubmissionOutcome {
    submit_preflight_from_shell(&request)
}

#[tauri::command]
fn accept_preflight_downgrade() -> PreflightDowngradeAcceptanceOutcome {
    accept_preflight_downgrade_from_shell()
}

#[tauri::command]
fn provider_configuration_state() -> ProviderConfigurationStateSnapshot {
    provider_configuration_state_from_shell()
}

#[tauri::command]
fn submit_provider_configuration(
    request: ProviderConfigurationSubmissionRequest,
) -> ProviderConfigurationSubmissionOutcome {
    submit_provider_configuration_from_shell(&request)
}

#[tauri::command]
fn startup_checks_scaffold_note() -> String {
    startup_checks_scaffold().summary()
}

#[tauri::command]
fn runtime_presence_scaffold_note() -> String {
    runtime_presence_state().summary()
}

#[tauri::command]
fn runtime_hydration_scaffold_note() -> String {
    runtime_hydration_state().summary()
}

#[tauri::command]
fn runtime_integrity_scaffold_note() -> String {
    runtime_integrity_state().summary()
}

#[tauri::command]
fn hydrate_managed_runtime() -> Result<String, String> {
    hydrate_managed_runtime_from_shell().map(|outcome| outcome.summary())
}

#[tauri::command]
fn app_session_trust_boundary_scaffold_note() -> String {
    app_session_trust_boundary_scaffold().summary()
}

#[tauri::command]
fn app_lifecycle_truth_surface_note() -> String {
    app_lifecycle_truth_surface().summary()
}

#[tauri::command]
fn engine_supervision_scaffold_note() -> String {
    engine_supervision_scaffold().summary()
}

#[tauri::command]
fn runtime_readiness_truth_surface_note() -> String {
    runtime_readiness_truth_surface().summary()
}

#[tauri::command]
fn runtime_package_contract_basis_note() -> String {
    runtime_package_contract_basis().summary()
}

#[tauri::command]
async fn check_native_app_updater(app: tauri::AppHandle) -> UpdaterContractSnapshot {
    refresh_native_app_updater_state(&app).await;
    updater_contract_truth_surface()
}

#[tauri::command]
async fn prepare_native_app_updater(app: tauri::AppHandle) -> UpdaterContractSnapshot {
    prepare_native_app_updater_state(&app).await;
    updater_contract_truth_surface()
}

#[tauri::command]
fn apply_native_app_updater(app: tauri::AppHandle) -> Result<UpdaterContractSnapshot, String> {
    apply_prepared_native_app_updater_state(&app)?;
    Ok(updater_contract_truth_surface())
}

#[tauri::command]
fn native_app_updater_apply_gate_note() -> String {
    native_app_updater_apply_gate_state().summary()
}

#[tauri::command]
fn updater_contract_state() -> UpdaterContractSnapshot {
    updater_contract_truth_surface()
}

#[tauri::command]
fn runtime_delivery_contract_state() -> RuntimeDeliveryContractSnapshot {
    runtime_delivery_contract_truth_surface()
}

#[tauri::command]
fn local_interface_contract_scaffold_note() -> String {
    local_interface_contract_scaffold().summary()
}

#[tauri::command]
fn controlled_shutdown_scaffold_note() -> String {
    controlled_shutdown_scaffold().summary()
}

fn build_shell_scaffold() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default()
    .plugin(tauri_plugin_updater::Builder::new().build())
    .invoke_handler(tauri::generate_handler![
        placeholder_shell_boundary_note,
        placeholder_engine_boundary_note,
        activation_contract_basis_note,
        activation_secure_store_surface_note,
        activation_support_store_surface_note,
        activation_local_validation_surface_note,
        commit_activation_approval_locally,
        submit_device_activation,
        activation_storage_boundary_surface_note,
        activation_readiness_truth_surface_note,
        preflight_contract_note,
        preflight_readiness_truth_surface_note,
        preflight_readiness_state,
        run_session_contract_note,
        run_workspace_boundary_note,
        run_start_lineage_note,
        run_start_readiness_truth_surface_note,
        run_start_readiness_state,
        runtime_persona_contract_note,
        runtime_persona_contract_state,
        run_bounded_memory_note,
        run_bounded_memory_state,
        run_moderated_rounds_note,
        run_moderated_rounds_state,
        run_token_duration_tracking_note,
        run_token_duration_tracking_state,
        run_lifecycle_state,
        run_status_state,
        run_cleanup_contract_state,
        cleanup_scope_mapping_state,
        startup_janitor_contract_state,
        run_event_contract_state,
        run_finalization_contract_state,
        report_output_contract_state,
        report_storage_boundary_state,
        report_access_contract_state,
        report_access_metadata_state,
        open_local_report_pdf,
        export_local_report_pdf,
        run_event_history,
        run_session_state,
        run_session_state_note,
        start_run_session,
        cancel_run_session,
        controlled_shutdown_state,
        request_controlled_shutdown,
        run_workspace_root_note,
        submit_preflight,
        accept_preflight_downgrade,
        provider_configuration_contract_note,
        provider_storage_boundary_note,
        provider_readiness_truth_surface_note,
        provider_configuration_state,
        submit_provider_configuration,
        startup_checks_scaffold_note,
        runtime_presence_scaffold_note,
        runtime_hydration_scaffold_note,
        runtime_integrity_scaffold_note,
        hydrate_managed_runtime,
        app_session_trust_boundary_scaffold_note,
        app_lifecycle_truth_surface_note,
        engine_supervision_scaffold_note,
        runtime_readiness_truth_surface_note,
        runtime_package_contract_basis_note,
        check_native_app_updater,
        prepare_native_app_updater,
        apply_native_app_updater,
        native_app_updater_apply_gate_note,
        updater_contract_state,
        runtime_delivery_contract_state,
        local_interface_contract_scaffold_note,
        controlled_shutdown_scaffold_note
    ])
}

fn main() {
    let _ = startup_janitor_execution::ensure_startup_janitor_executed();

    build_shell_scaffold()
        .run(tauri::generate_context!())
        .expect("failed to run miro-fish shell");
}

