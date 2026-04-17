use std::env;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use tauri::Runtime;
use tauri_plugin_updater::{Update, UpdaterExt};
use url::Url;

use crate::run_lifecycle_state::{run_lifecycle_state_from_shell, CanonicalRunLifecycleState};
use crate::updater_contract::UpdaterState;

pub const NATIVE_APP_UPDATER_DISCOVERY_COMMAND_NAME: &str = "check_native_app_updater";
pub const NATIVE_APP_UPDATER_PREPARE_COMMAND_NAME: &str = "prepare_native_app_updater";
pub const NATIVE_APP_UPDATER_APPLY_COMMAND_NAME: &str = "apply_native_app_updater";
pub const NATIVE_APP_UPDATER_APPLY_GATE_COMMAND_NAME: &str = "native_app_updater_apply_gate";
const UPDATER_ENDPOINTS_ENV_VAR: &str = "MIRO_FISH_APP_UPDATER_ENDPOINTS";
const UPDATER_PUBKEY_ENV_VAR: &str = "MIRO_FISH_APP_UPDATER_PUBKEY";
const CONFIGURATION_SOURCE_NAME: &str = "EnvironmentVariables";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeAppUpdaterConfigurationStatus {
    pub configuration_source_name: &'static str,
    pub configuration_ready: bool,
    pub endpoints_configured: bool,
    pub pubkey_configured: bool,
    pub endpoint_count: usize,
    pub configuration_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeAppUpdaterRuntimeState {
    pub current_state: UpdaterState,
    pub current_update_available: bool,
    pub current_update_ready: bool,
    pub current_candidate_rejected: bool,
    pub current_error_active: bool,
    pub current_status_note: String,
    pub last_check_attempted: bool,
    pub last_check_started_at_epoch_ms: Option<u64>,
    pub last_check_completed_at_epoch_ms: Option<u64>,
    pub active_run_detected_during_last_check: bool,
    pub real_update_check_materialized: bool,
    pub real_update_download_materialized: bool,
    pub real_update_verification_materialized: bool,
    pub real_update_apply_materialized: bool,
    pub last_error_detail: Option<String>,
    pub last_discovered_version: Option<String>,
    pub last_discovered_release_notes: Option<String>,
    pub last_discovered_publication_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeAppUpdaterApplyGateState {
    pub apply_allowed_now: bool,
    pub apply_blocked_by_active_run: bool,
    pub apply_blocked_because_update_not_ready: bool,
    pub restart_coordination_required: bool,
    pub apply_status_note: String,
}

impl NativeAppUpdaterApplyGateState {
    pub fn summary(&self) -> String {
        format!(
            "Native app-updater apply gate is materialized. Apply allowed now is {}; apply blocked by active run is {}; apply blocked because update is not ready is {}; restart coordination required is {}; apply status note is {}.",
            self.apply_allowed_now,
            self.apply_blocked_by_active_run,
            self.apply_blocked_because_update_not_ready,
            self.restart_coordination_required,
            self.apply_status_note,
        )
    }
}

#[derive(Debug, Clone)]
struct ResolvedNativeAppUpdaterConfiguration {
    pub endpoints: Vec<Url>,
    pub pubkey: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedEndpointValues {
    valid_endpoints: Vec<Url>,
    invalid_endpoint_values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NativeAppUpdaterCheckOutcome {
    NoUpdateAvailable,
    UpdateAvailable(DiscoveredNativeUpdate),
    TechnicalFailure(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NativeAppUpdaterPrepareOutcome {
    NoUpdateAvailable,
    UpdateReady(PreparedNativeUpdate),
    UpdateRejected(RejectedNativeUpdate),
    TechnicalFailure(PrepareTechnicalFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiscoveredNativeUpdate {
    version: String,
    release_notes: Option<String>,
    publication_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreparedNativeUpdate {
    discovered_update: DiscoveredNativeUpdate,
    artifact_byte_count: usize,
}

struct PreparedNativeUpdateCandidate {
    update: Update,
    artifact_bytes: Vec<u8>,
    prepared_update: PreparedNativeUpdate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RejectedNativeUpdate {
    discovered_update: DiscoveredNativeUpdate,
    detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrepareTechnicalFailure {
    discovered_update: Option<DiscoveredNativeUpdate>,
    detail: String,
    download_materialized: bool,
    verification_materialized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeAppUpdaterPreparationFailureKind {
    RejectedCandidate,
    TechnicalFailure,
}

impl DiscoveredNativeUpdate {
    fn from_update(update: &Update) -> Self {
        Self {
            version: update.version.clone(),
            release_notes: update.body.clone().and_then(non_empty_trimmed_option),
            publication_date: update.date.as_ref().map(ToString::to_string),
        }
    }
}

impl Default for NativeAppUpdaterRuntimeState {
    fn default() -> Self {
        Self {
            current_state: UpdaterState::UpdaterIdle,
            current_update_available: false,
            current_update_ready: false,
            current_candidate_rejected: false,
            current_error_active: false,
            current_status_note:
                "Native app-updater discovery is materialized at the shell boundary, but no signed discovery check has run yet."
                    .to_string(),
            last_check_attempted: false,
            last_check_started_at_epoch_ms: None,
            last_check_completed_at_epoch_ms: None,
            active_run_detected_during_last_check: false,
            real_update_check_materialized: false,
            real_update_download_materialized: false,
            real_update_verification_materialized: false,
            real_update_apply_materialized: false,
            last_error_detail: None,
            last_discovered_version: None,
            last_discovered_release_notes: None,
            last_discovered_publication_date: None,
        }
    }
}

static NATIVE_APP_UPDATER_STATE: OnceLock<Mutex<NativeAppUpdaterRuntimeState>> = OnceLock::new();
static PREPARED_NATIVE_UPDATE_CANDIDATE: OnceLock<Mutex<Option<PreparedNativeUpdateCandidate>>> =
    OnceLock::new();

pub fn native_app_updater_configuration_status() -> NativeAppUpdaterConfigurationStatus {
    resolve_native_app_updater_configuration_from_values(
        env::var(UPDATER_ENDPOINTS_ENV_VAR).ok().as_deref(),
        env::var(UPDATER_PUBKEY_ENV_VAR).ok().as_deref(),
    )
    .0
}

pub fn native_app_updater_runtime_state() -> NativeAppUpdaterRuntimeState {
    with_native_app_updater_state(|state| state.clone())
}

pub fn native_app_updater_apply_gate_state() -> NativeAppUpdaterApplyGateState {
    native_app_updater_apply_gate_for_runtime_state(
        &native_app_updater_runtime_state(),
        active_run_detected_for_updater_coordination(),
    )
}

pub fn apply_prepared_native_app_updater_state<R: Runtime>(
    app: &tauri::AppHandle<R>,
) -> Result<(), String> {
    let runtime_state = native_app_updater_runtime_state();
    let active_run_detected = active_run_detected_for_updater_coordination();
    let apply_gate = native_app_updater_apply_gate_for_runtime_state(&runtime_state, active_run_detected);

    if !apply_gate.apply_allowed_now {
        return Err(apply_gate.apply_status_note);
    }

    let Some(prepared_candidate) = take_prepared_native_update_candidate() else {
        let detail = "The shell lost the staged verified app update artifact before apply could begin. Run prepare_native_app_updater again before applying.".to_string();

        with_native_app_updater_state(|state| {
            let next_state = apply_error_runtime_state(state, None, detail.clone(), false);
            *state = next_state;
        });

        return Err(detail);
    };

    with_native_app_updater_state(|state| {
        let next_state = applying_runtime_state(state, &prepared_candidate.prepared_update);
        *state = next_state;
    });

    if let Err(error) = prepared_candidate
        .update
        .install(&prepared_candidate.artifact_bytes)
    {
        let detail = format!(
            "The shell failed to install verified app version {}: {error}",
            prepared_candidate.prepared_update.discovered_update.version,
        );

        with_native_app_updater_state(|state| {
            let next_state = apply_error_runtime_state(
                state,
                Some(&prepared_candidate.prepared_update.discovered_update),
                detail.clone(),
                true,
            );
            *state = next_state;
        });

        return Err(detail);
    }

    with_native_app_updater_state(|state| {
        let next_state = completed_apply_runtime_state(state, &prepared_candidate.prepared_update);
        *state = next_state;
    });

    app.request_restart();

    Ok(())
}

pub async fn refresh_native_app_updater_state<R: Runtime>(app: &tauri::AppHandle<R>) {
    clear_prepared_native_update_candidate();

    let active_run_detected = active_run_detected_for_updater_coordination();
    let (configuration_status, resolved_configuration) = resolve_native_app_updater_configuration_from_values(
        env::var(UPDATER_ENDPOINTS_ENV_VAR).ok().as_deref(),
        env::var(UPDATER_PUBKEY_ENV_VAR).ok().as_deref(),
    );

    if resolved_configuration.is_none() {
        let detail = configuration_status.configuration_note.clone();

        with_native_app_updater_state(|state| {
            state.current_state = UpdaterState::UpdaterError;
            state.current_update_available = false;
            state.current_update_ready = false;
            state.current_candidate_rejected = false;
            state.current_error_active = true;
            state.current_status_note = detail.clone();
            state.active_run_detected_during_last_check = active_run_detected;
            state.last_error_detail = Some(detail.clone());
            state.last_discovered_version = None;
            state.last_discovered_release_notes = None;
            state.last_discovered_publication_date = None;
        });
        return;
    }

    let started_at_epoch_ms = current_epoch_millis();
    let checking_status_note = if active_run_detected {
        "The shell is performing a signed native app-updater discovery check during an active run. Discovery remains allowed, but any later install/apply path stays deferred until the run is terminal."
            .to_string()
    } else {
        "The shell is performing a signed native app-updater discovery check against the configured release endpoint set."
            .to_string()
    };

    with_native_app_updater_state(|state| {
        state.current_state = UpdaterState::CheckingForUpdates;
        state.current_update_available = false;
        state.current_update_ready = false;
        state.current_candidate_rejected = false;
        state.current_error_active = false;
        state.current_status_note = checking_status_note;
        state.last_check_attempted = true;
        state.last_check_started_at_epoch_ms = started_at_epoch_ms;
        state.last_check_completed_at_epoch_ms = None;
        state.active_run_detected_during_last_check = active_run_detected;
        state.last_error_detail = None;
        state.last_discovered_version = None;
        state.last_discovered_release_notes = None;
        state.last_discovered_publication_date = None;
    });

    let resolved_configuration = resolved_configuration.expect("resolved configuration must exist");
    let check_outcome = perform_native_app_updater_check(app, &resolved_configuration).await;
    let completed_at_epoch_ms = current_epoch_millis();

    with_native_app_updater_state(|state| {
        *state = completed_runtime_state(
            active_run_detected,
            started_at_epoch_ms,
            completed_at_epoch_ms,
            check_outcome,
        );
    });
}

pub async fn prepare_native_app_updater_state<R: Runtime>(app: &tauri::AppHandle<R>) {
    clear_prepared_native_update_candidate();

    let active_run_detected = active_run_detected_for_updater_coordination();
    let (configuration_status, resolved_configuration) = resolve_native_app_updater_configuration_from_values(
        env::var(UPDATER_ENDPOINTS_ENV_VAR).ok().as_deref(),
        env::var(UPDATER_PUBKEY_ENV_VAR).ok().as_deref(),
    );

    if resolved_configuration.is_none() {
        let detail = configuration_status.configuration_note.clone();

        with_native_app_updater_state(|state| {
            state.current_state = UpdaterState::UpdaterError;
            state.current_update_available = false;
            state.current_update_ready = false;
            state.current_candidate_rejected = false;
            state.current_error_active = true;
            state.current_status_note = detail.clone();
            state.active_run_detected_during_last_check = active_run_detected;
            state.last_error_detail = Some(detail.clone());
            state.last_discovered_version = None;
            state.last_discovered_release_notes = None;
            state.last_discovered_publication_date = None;
        });
        return;
    }

    let started_at_epoch_ms = current_epoch_millis();
    let preparing_status_note = if active_run_detected {
        "The shell is running signed native updater discovery followed by explicit download and verification during an active run. Preparation is allowed, but apply remains deferred until the run is terminal."
            .to_string()
    } else {
        "The shell is running signed native updater discovery followed by explicit download and verification to determine whether the candidate can reach UpdateReady."
            .to_string()
    };

    with_native_app_updater_state(|state| {
        state.current_state = UpdaterState::CheckingForUpdates;
        state.current_update_available = false;
        state.current_update_ready = false;
        state.current_candidate_rejected = false;
        state.current_error_active = false;
        state.current_status_note = preparing_status_note;
        state.last_check_attempted = true;
        state.last_check_started_at_epoch_ms = started_at_epoch_ms;
        state.last_check_completed_at_epoch_ms = None;
        state.active_run_detected_during_last_check = active_run_detected;
        state.real_update_check_materialized = true;
        state.real_update_download_materialized = false;
        state.real_update_verification_materialized = false;
        state.real_update_apply_materialized = false;
        state.last_error_detail = None;
        state.last_discovered_version = None;
        state.last_discovered_release_notes = None;
        state.last_discovered_publication_date = None;
    });

    let resolved_configuration = resolved_configuration.expect("resolved configuration must exist");
    let prepare_outcome =
        perform_native_app_updater_prepare(app, &resolved_configuration, active_run_detected).await;
    let completed_at_epoch_ms = current_epoch_millis();

    with_native_app_updater_state(|state| {
        *state = completed_preparation_runtime_state(
            active_run_detected,
            started_at_epoch_ms,
            completed_at_epoch_ms,
            prepare_outcome,
        );
    });
}

fn with_native_app_updater_state<T>(
    f: impl FnOnce(&mut NativeAppUpdaterRuntimeState) -> T,
) -> T {
    let state = NATIVE_APP_UPDATER_STATE.get_or_init(|| Mutex::new(NativeAppUpdaterRuntimeState::default()));
    let mut guard = match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    f(&mut guard)
}

fn with_prepared_native_update_candidate<T>(
    f: impl FnOnce(&mut Option<PreparedNativeUpdateCandidate>) -> T,
) -> T {
    let candidate = PREPARED_NATIVE_UPDATE_CANDIDATE.get_or_init(|| Mutex::new(None));
    let mut guard = match candidate.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    f(&mut guard)
}

fn clear_prepared_native_update_candidate() {
    with_prepared_native_update_candidate(|candidate| {
        *candidate = None;
    });
}

fn stage_prepared_native_update_candidate(candidate: PreparedNativeUpdateCandidate) {
    with_prepared_native_update_candidate(|prepared_candidate| {
        *prepared_candidate = Some(candidate);
    });
}

fn take_prepared_native_update_candidate() -> Option<PreparedNativeUpdateCandidate> {
    with_prepared_native_update_candidate(Option::take)
}

fn resolve_native_app_updater_configuration_from_values(
    raw_endpoints: Option<&str>,
    raw_pubkey: Option<&str>,
) -> (
    NativeAppUpdaterConfigurationStatus,
    Option<ResolvedNativeAppUpdaterConfiguration>,
) {
    let parsed_endpoints = parse_endpoint_values(raw_endpoints.unwrap_or_default());
    let trimmed_pubkey = raw_pubkey.unwrap_or_default().trim().to_string();
    let pubkey_configured = !trimmed_pubkey.is_empty();
    let endpoints_configured = !parsed_endpoints.valid_endpoints.is_empty();
    let configuration_ready = endpoints_configured
        && pubkey_configured
        && parsed_endpoints.invalid_endpoint_values.is_empty();

    let configuration_note = if !parsed_endpoints.invalid_endpoint_values.is_empty() {
        format!(
            "{} contains invalid URL values: {}. Native app-updater discovery stays shell-owned but cannot run until every endpoint is valid and signed configuration is complete.",
            UPDATER_ENDPOINTS_ENV_VAR,
            parsed_endpoints.invalid_endpoint_values.join(", ")
        )
    } else if !endpoints_configured && !pubkey_configured {
        format!(
            "Set {} and {} to enable signed native app-updater discovery against the governed release feed.",
            UPDATER_ENDPOINTS_ENV_VAR,
            UPDATER_PUBKEY_ENV_VAR
        )
    } else if !endpoints_configured {
        format!(
            "Set {} to a comma-, semicolon-, or newline-delimited list of release metadata URLs before running a signed native app-updater discovery check.",
            UPDATER_ENDPOINTS_ENV_VAR
        )
    } else if !pubkey_configured {
        format!(
            "Set {} to the updater signing public key before running a signed native app-updater discovery check.",
            UPDATER_PUBKEY_ENV_VAR
        )
    } else {
        format!(
            "Signed native app-updater discovery is configured through {} and {}.",
            UPDATER_ENDPOINTS_ENV_VAR,
            UPDATER_PUBKEY_ENV_VAR
        )
    };

    let configuration_status = NativeAppUpdaterConfigurationStatus {
        configuration_source_name: CONFIGURATION_SOURCE_NAME,
        configuration_ready,
        endpoints_configured,
        pubkey_configured,
        endpoint_count: parsed_endpoints.valid_endpoints.len(),
        configuration_note,
    };

    let resolved_configuration = configuration_ready.then_some(ResolvedNativeAppUpdaterConfiguration {
        endpoints: parsed_endpoints.valid_endpoints,
        pubkey: trimmed_pubkey,
    });

    (configuration_status, resolved_configuration)
}

fn parse_endpoint_values(raw_endpoints: &str) -> ParsedEndpointValues {
    let mut valid_endpoints = Vec::new();
    let mut invalid_endpoint_values = Vec::new();

    for candidate in raw_endpoints
        .split([',', ';', '\n', '\r'])
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        match Url::parse(candidate) {
            Ok(url) => valid_endpoints.push(url),
            Err(_) => invalid_endpoint_values.push(candidate.to_string()),
        }
    }

    ParsedEndpointValues {
        valid_endpoints,
        invalid_endpoint_values,
    }
}

async fn perform_native_app_updater_check<R: Runtime>(
    app: &tauri::AppHandle<R>,
    resolved_configuration: &ResolvedNativeAppUpdaterConfiguration,
) -> NativeAppUpdaterCheckOutcome {
    let updater_builder = match app
        .updater_builder()
        .endpoints(resolved_configuration.endpoints.clone())
    {
        Ok(builder) => builder,
        Err(error) => {
            return NativeAppUpdaterCheckOutcome::TechnicalFailure(format!(
                "The shell could not apply the configured native updater endpoints: {error}"
            ))
        }
    };

    let updater = match updater_builder
        .pubkey(resolved_configuration.pubkey.clone())
        .build()
    {
        Ok(updater) => updater,
        Err(error) => {
            return NativeAppUpdaterCheckOutcome::TechnicalFailure(format!(
                "The shell could not build the signed native updater client: {error}"
            ))
        }
    };

    match updater.check().await {
        Ok(None) => NativeAppUpdaterCheckOutcome::NoUpdateAvailable,
        Ok(Some(update)) => {
            NativeAppUpdaterCheckOutcome::UpdateAvailable(DiscoveredNativeUpdate::from_update(&update))
        }
        Err(error) => NativeAppUpdaterCheckOutcome::TechnicalFailure(format!(
            "The configured native updater discovery check failed: {error}"
        )),
    }
}

async fn perform_native_app_updater_prepare<R: Runtime>(
    app: &tauri::AppHandle<R>,
    resolved_configuration: &ResolvedNativeAppUpdaterConfiguration,
    active_run_detected: bool,
) -> NativeAppUpdaterPrepareOutcome {
    let updater_builder = match app
        .updater_builder()
        .endpoints(resolved_configuration.endpoints.clone())
    {
        Ok(builder) => builder,
        Err(error) => {
            return NativeAppUpdaterPrepareOutcome::TechnicalFailure(PrepareTechnicalFailure {
                discovered_update: None,
                detail: format!(
                    "The shell could not apply the configured native updater endpoints: {error}"
                ),
                download_materialized: false,
                verification_materialized: false,
            })
        }
    };

    let updater = match updater_builder
        .pubkey(resolved_configuration.pubkey.clone())
        .build()
    {
        Ok(updater) => updater,
        Err(error) => {
            return NativeAppUpdaterPrepareOutcome::TechnicalFailure(PrepareTechnicalFailure {
                discovered_update: None,
                detail: format!(
                    "The shell could not build the signed native updater client: {error}"
                ),
                download_materialized: false,
                verification_materialized: false,
            })
        }
    };

    let update = match updater.check().await {
        Ok(None) => return NativeAppUpdaterPrepareOutcome::NoUpdateAvailable,
        Ok(Some(update)) => update,
        Err(error) => {
            return NativeAppUpdaterPrepareOutcome::TechnicalFailure(PrepareTechnicalFailure {
                discovered_update: None,
                detail: format!(
                    "The configured native updater discovery check failed before preparation could begin: {error}"
                ),
                download_materialized: false,
                verification_materialized: false,
            })
        }
    };

    let discovered_update = DiscoveredNativeUpdate::from_update(&update);
    let downloading_status_note = if active_run_detected {
        format!(
            "The shell found signed app version {} and is now downloading the candidate while the current run stays active. Apply remains blocked until the run is terminal.",
            discovered_update.version,
        )
    } else {
        format!(
            "The shell found signed app version {} and is now downloading the candidate for verification.",
            discovered_update.version,
        )
    };

    with_native_app_updater_state(|state| {
        state.current_state = UpdaterState::DownloadingUpdate;
        state.current_update_available = true;
        state.current_update_ready = false;
        state.current_candidate_rejected = false;
        state.current_error_active = false;
        state.current_status_note = downloading_status_note;
        state.active_run_detected_during_last_check = active_run_detected;
        state.real_update_check_materialized = true;
        state.real_update_download_materialized = true;
        state.real_update_verification_materialized = false;
        state.real_update_apply_materialized = false;
        state.last_error_detail = None;
        state.last_discovered_version = Some(discovered_update.version.clone());
        state.last_discovered_release_notes = discovered_update.release_notes.clone();
        state.last_discovered_publication_date = discovered_update.publication_date.clone();
    });

    let artifact_bytes = match update.download(|_, _| {}, || {}).await {
        Ok(artifact_bytes) => artifact_bytes,
        Err(error) => {
            let detail = format!(
                "The shell could not finish native updater download and verification for version {}: {error}",
                discovered_update.version,
            );

            return match preparation_failure_kind_from_error_debug_label(&format!("{error:?}")) {
                NativeAppUpdaterPreparationFailureKind::RejectedCandidate => {
                    NativeAppUpdaterPrepareOutcome::UpdateRejected(RejectedNativeUpdate {
                        discovered_update,
                        detail,
                    })
                }
                NativeAppUpdaterPreparationFailureKind::TechnicalFailure => {
                    NativeAppUpdaterPrepareOutcome::TechnicalFailure(PrepareTechnicalFailure {
                        discovered_update: Some(discovered_update),
                        detail,
                        download_materialized: true,
                        verification_materialized: false,
                    })
                }
            };
        }
    };

    let verifying_status_note = if active_run_detected {
        format!(
            "The shell downloaded app version {} and is now finalizing verification while apply remains blocked by the active run.",
            discovered_update.version,
        )
    } else {
        format!(
            "The shell downloaded app version {} and is now finalizing verification before any UpdateReady classification can be exposed.",
            discovered_update.version,
        )
    };

    with_native_app_updater_state(|state| {
        state.current_state = UpdaterState::VerifyingUpdate;
        state.current_update_available = true;
        state.current_update_ready = false;
        state.current_candidate_rejected = false;
        state.current_error_active = false;
        state.current_status_note = verifying_status_note;
        state.active_run_detected_during_last_check = active_run_detected;
        state.real_update_check_materialized = true;
        state.real_update_download_materialized = true;
        state.real_update_verification_materialized = true;
        state.real_update_apply_materialized = false;
        state.last_error_detail = None;
        state.last_discovered_version = Some(discovered_update.version.clone());
        state.last_discovered_release_notes = discovered_update.release_notes.clone();
        state.last_discovered_publication_date = discovered_update.publication_date.clone();
    });

    match verify_downloaded_native_update_artifact(
        update.version.as_str(),
        update.signature.as_str(),
        update.download_url.as_ref(),
        &artifact_bytes,
    ) {
        Ok(()) => {
            let prepared_update = PreparedNativeUpdate {
                discovered_update: discovered_update.clone(),
                artifact_byte_count: artifact_bytes.len(),
            };

            stage_prepared_native_update_candidate(PreparedNativeUpdateCandidate {
                update,
                artifact_bytes,
                prepared_update: prepared_update.clone(),
            });

            NativeAppUpdaterPrepareOutcome::UpdateReady(prepared_update)
        }
        Err(detail) => NativeAppUpdaterPrepareOutcome::UpdateRejected(RejectedNativeUpdate {
            discovered_update,
            detail,
        }),
    }
}

fn applying_runtime_state(
    previous_state: &NativeAppUpdaterRuntimeState,
    prepared_update: &PreparedNativeUpdate,
) -> NativeAppUpdaterRuntimeState {
    NativeAppUpdaterRuntimeState {
        current_state: UpdaterState::ApplyingUpdate,
        current_update_available: true,
        current_update_ready: false,
        current_candidate_rejected: false,
        current_error_active: false,
        current_status_note: format!(
            "The shell is applying verified app version {} ({} bytes) and will request app restart once installation completes.",
            prepared_update.discovered_update.version,
            prepared_update.artifact_byte_count,
        ),
        last_check_attempted: previous_state.last_check_attempted,
        last_check_started_at_epoch_ms: previous_state.last_check_started_at_epoch_ms,
        last_check_completed_at_epoch_ms: previous_state.last_check_completed_at_epoch_ms,
        active_run_detected_during_last_check: previous_state.active_run_detected_during_last_check,
        real_update_check_materialized: previous_state.real_update_check_materialized,
        real_update_download_materialized: previous_state.real_update_download_materialized,
        real_update_verification_materialized: previous_state.real_update_verification_materialized,
        real_update_apply_materialized: true,
        last_error_detail: None,
        last_discovered_version: Some(prepared_update.discovered_update.version.clone()),
        last_discovered_release_notes: prepared_update.discovered_update.release_notes.clone(),
        last_discovered_publication_date: prepared_update.discovered_update.publication_date.clone(),
    }
}

fn completed_apply_runtime_state(
    previous_state: &NativeAppUpdaterRuntimeState,
    prepared_update: &PreparedNativeUpdate,
) -> NativeAppUpdaterRuntimeState {
    NativeAppUpdaterRuntimeState {
        current_state: UpdaterState::UpdateApplied,
        current_update_available: false,
        current_update_ready: false,
        current_candidate_rejected: false,
        current_error_active: false,
        current_status_note: format!(
            "The shell applied verified app version {} ({} bytes) and requested app restart to activate the new build.",
            prepared_update.discovered_update.version,
            prepared_update.artifact_byte_count,
        ),
        last_check_attempted: previous_state.last_check_attempted,
        last_check_started_at_epoch_ms: previous_state.last_check_started_at_epoch_ms,
        last_check_completed_at_epoch_ms: previous_state.last_check_completed_at_epoch_ms,
        active_run_detected_during_last_check: previous_state.active_run_detected_during_last_check,
        real_update_check_materialized: previous_state.real_update_check_materialized,
        real_update_download_materialized: previous_state.real_update_download_materialized,
        real_update_verification_materialized: previous_state.real_update_verification_materialized,
        real_update_apply_materialized: true,
        last_error_detail: None,
        last_discovered_version: Some(prepared_update.discovered_update.version.clone()),
        last_discovered_release_notes: prepared_update.discovered_update.release_notes.clone(),
        last_discovered_publication_date: prepared_update.discovered_update.publication_date.clone(),
    }
}

fn apply_error_runtime_state(
    previous_state: &NativeAppUpdaterRuntimeState,
    discovered_update: Option<&DiscoveredNativeUpdate>,
    detail: String,
    apply_materialized: bool,
) -> NativeAppUpdaterRuntimeState {
    let last_discovered_version = discovered_update
        .map(|update| update.version.clone())
        .or_else(|| previous_state.last_discovered_version.clone());
    let last_discovered_release_notes = discovered_update
        .and_then(|update| update.release_notes.clone())
        .or_else(|| previous_state.last_discovered_release_notes.clone());
    let last_discovered_publication_date = discovered_update
        .and_then(|update| update.publication_date.clone())
        .or_else(|| previous_state.last_discovered_publication_date.clone());
    let current_status_note = if let Some(update) = discovered_update {
        format!(
            "The shell attempted explicit apply for verified app version {} but installation failed: {}",
            update.version,
            detail,
        )
    } else {
        format!(
            "The shell could not begin explicit apply because no staged verified update artifact is available: {}",
            detail,
        )
    };

    NativeAppUpdaterRuntimeState {
        current_state: UpdaterState::UpdaterError,
        current_update_available: false,
        current_update_ready: false,
        current_candidate_rejected: false,
        current_error_active: true,
        current_status_note,
        last_check_attempted: previous_state.last_check_attempted,
        last_check_started_at_epoch_ms: previous_state.last_check_started_at_epoch_ms,
        last_check_completed_at_epoch_ms: previous_state.last_check_completed_at_epoch_ms,
        active_run_detected_during_last_check: previous_state.active_run_detected_during_last_check,
        real_update_check_materialized: previous_state.real_update_check_materialized,
        real_update_download_materialized: previous_state.real_update_download_materialized,
        real_update_verification_materialized: previous_state.real_update_verification_materialized,
        real_update_apply_materialized: apply_materialized,
        last_error_detail: Some(detail),
        last_discovered_version,
        last_discovered_release_notes,
        last_discovered_publication_date,
    }
}

fn completed_runtime_state(
    active_run_detected: bool,
    started_at_epoch_ms: Option<u64>,
    completed_at_epoch_ms: Option<u64>,
    outcome: NativeAppUpdaterCheckOutcome,
) -> NativeAppUpdaterRuntimeState {
    match outcome {
        NativeAppUpdaterCheckOutcome::NoUpdateAvailable => NativeAppUpdaterRuntimeState {
            current_state: UpdaterState::NoUpdateAvailable,
            current_update_available: false,
            current_update_ready: false,
            current_candidate_rejected: false,
            current_error_active: false,
            current_status_note: if active_run_detected {
                "The shell completed a signed native updater discovery check during an active run and no app update is currently available. Discovery remains separate from run execution, and any later install/apply path stays deferred until the run is terminal."
                    .to_string()
            } else {
                "The shell completed a signed native updater discovery check and no app update is currently available."
                    .to_string()
            },
            last_check_attempted: true,
            last_check_started_at_epoch_ms: started_at_epoch_ms,
            last_check_completed_at_epoch_ms: completed_at_epoch_ms,
            active_run_detected_during_last_check: active_run_detected,
            real_update_check_materialized: true,
            real_update_download_materialized: false,
            real_update_verification_materialized: false,
            real_update_apply_materialized: false,
            last_error_detail: None,
            last_discovered_version: None,
            last_discovered_release_notes: None,
            last_discovered_publication_date: None,
        },
        NativeAppUpdaterCheckOutcome::UpdateAvailable(discovered_update) => {
            let status_note = if active_run_detected {
                format!(
                    "The shell completed a signed native updater discovery check during an active run and found app version {}. The candidate remains UpdateAvailable until the shell runs the explicit preparation command that downloads and verifies the artifact, and any apply path remains blocked while the run is active.",
                    discovered_update.version
                )
            } else {
                format!(
                    "The shell completed a signed native updater discovery check and found app version {}. The candidate remains UpdateAvailable until the shell runs the explicit preparation command that downloads and verifies the artifact.",
                    discovered_update.version
                )
            };

            NativeAppUpdaterRuntimeState {
                current_state: UpdaterState::UpdateAvailable,
                current_update_available: true,
                current_update_ready: false,
                current_candidate_rejected: false,
                current_error_active: false,
                current_status_note: status_note,
                last_check_attempted: true,
                last_check_started_at_epoch_ms: started_at_epoch_ms,
                last_check_completed_at_epoch_ms: completed_at_epoch_ms,
                active_run_detected_during_last_check: active_run_detected,
                real_update_check_materialized: true,
                real_update_download_materialized: false,
                real_update_verification_materialized: false,
                real_update_apply_materialized: false,
                last_error_detail: None,
                last_discovered_version: Some(discovered_update.version),
                last_discovered_release_notes: discovered_update.release_notes,
                last_discovered_publication_date: discovered_update.publication_date,
            }
        }
        NativeAppUpdaterCheckOutcome::TechnicalFailure(detail) => NativeAppUpdaterRuntimeState {
            current_state: UpdaterState::UpdaterError,
            current_update_available: false,
            current_update_ready: false,
            current_candidate_rejected: false,
            current_error_active: true,
            current_status_note: if active_run_detected {
                format!(
                    "The shell attempted a signed native updater discovery check during an active run, but the check failed: {} Discovery remains separate from run execution, and any later install/apply path stays deferred until the run is terminal.",
                    detail
                )
            } else {
                format!(
                    "The shell attempted a signed native updater discovery check, but the check failed: {}",
                    detail
                )
            },
            last_check_attempted: true,
            last_check_started_at_epoch_ms: started_at_epoch_ms,
            last_check_completed_at_epoch_ms: completed_at_epoch_ms,
            active_run_detected_during_last_check: active_run_detected,
            real_update_check_materialized: true,
            real_update_download_materialized: false,
            real_update_verification_materialized: false,
            real_update_apply_materialized: false,
            last_error_detail: Some(detail),
            last_discovered_version: None,
            last_discovered_release_notes: None,
            last_discovered_publication_date: None,
        },
    }
}

fn completed_preparation_runtime_state(
    active_run_detected: bool,
    started_at_epoch_ms: Option<u64>,
    completed_at_epoch_ms: Option<u64>,
    outcome: NativeAppUpdaterPrepareOutcome,
) -> NativeAppUpdaterRuntimeState {
    match outcome {
        NativeAppUpdaterPrepareOutcome::NoUpdateAvailable => NativeAppUpdaterRuntimeState {
            current_state: UpdaterState::NoUpdateAvailable,
            current_update_available: false,
            current_update_ready: false,
            current_candidate_rejected: false,
            current_error_active: false,
            current_status_note: if active_run_detected {
                "The shell ran discovery plus preparation during an active run and no app update is currently available. Apply remains separate from run execution."
                    .to_string()
            } else {
                "The shell ran discovery plus preparation and no app update is currently available."
                    .to_string()
            },
            last_check_attempted: true,
            last_check_started_at_epoch_ms: started_at_epoch_ms,
            last_check_completed_at_epoch_ms: completed_at_epoch_ms,
            active_run_detected_during_last_check: active_run_detected,
            real_update_check_materialized: true,
            real_update_download_materialized: false,
            real_update_verification_materialized: false,
            real_update_apply_materialized: false,
            last_error_detail: None,
            last_discovered_version: None,
            last_discovered_release_notes: None,
            last_discovered_publication_date: None,
        },
        NativeAppUpdaterPrepareOutcome::UpdateReady(prepared_update) => NativeAppUpdaterRuntimeState {
            current_state: UpdaterState::UpdateReady,
            current_update_available: true,
            current_update_ready: true,
            current_candidate_rejected: false,
            current_error_active: false,
            current_status_note: if active_run_detected {
                format!(
                    "The shell downloaded and verified app version {} ({} bytes) and promoted it to UpdateReady. Apply remains blocked until the current run is terminal and restart coordination is safe.",
                    prepared_update.discovered_update.version,
                    prepared_update.artifact_byte_count,
                )
            } else {
                format!(
                    "The shell downloaded and verified app version {} ({} bytes) and promoted it to UpdateReady for explicit shell-owned apply.",
                    prepared_update.discovered_update.version,
                    prepared_update.artifact_byte_count,
                )
            },
            last_check_attempted: true,
            last_check_started_at_epoch_ms: started_at_epoch_ms,
            last_check_completed_at_epoch_ms: completed_at_epoch_ms,
            active_run_detected_during_last_check: active_run_detected,
            real_update_check_materialized: true,
            real_update_download_materialized: true,
            real_update_verification_materialized: true,
            real_update_apply_materialized: false,
            last_error_detail: None,
            last_discovered_version: Some(prepared_update.discovered_update.version),
            last_discovered_release_notes: prepared_update.discovered_update.release_notes,
            last_discovered_publication_date: prepared_update.discovered_update.publication_date,
        },
        NativeAppUpdaterPrepareOutcome::UpdateRejected(rejected_update) => NativeAppUpdaterRuntimeState {
            current_state: UpdaterState::UpdateRejected,
            current_update_available: false,
            current_update_ready: false,
            current_candidate_rejected: true,
            current_error_active: false,
            current_status_note: if active_run_detected {
                format!(
                    "The shell rejected app version {} during explicit download and verification while the run stayed active: {}",
                    rejected_update.discovered_update.version,
                    rejected_update.detail,
                )
            } else {
                format!(
                    "The shell rejected app version {} during explicit download and verification: {}",
                    rejected_update.discovered_update.version,
                    rejected_update.detail,
                )
            },
            last_check_attempted: true,
            last_check_started_at_epoch_ms: started_at_epoch_ms,
            last_check_completed_at_epoch_ms: completed_at_epoch_ms,
            active_run_detected_during_last_check: active_run_detected,
            real_update_check_materialized: true,
            real_update_download_materialized: true,
            real_update_verification_materialized: true,
            real_update_apply_materialized: false,
            last_error_detail: None,
            last_discovered_version: Some(rejected_update.discovered_update.version),
            last_discovered_release_notes: rejected_update.discovered_update.release_notes,
            last_discovered_publication_date: rejected_update.discovered_update.publication_date,
        },
        NativeAppUpdaterPrepareOutcome::TechnicalFailure(failure) => NativeAppUpdaterRuntimeState {
            current_state: UpdaterState::UpdaterError,
            current_update_available: false,
            current_update_ready: false,
            current_candidate_rejected: false,
            current_error_active: true,
            current_status_note: if active_run_detected {
                format!(
                    "The shell attempted native updater preparation during an active run, but the preparation flow failed: {} Apply remains separate from run execution.",
                    failure.detail,
                )
            } else {
                format!(
                    "The shell attempted native updater preparation, but the preparation flow failed: {}",
                    failure.detail,
                )
            },
            last_check_attempted: true,
            last_check_started_at_epoch_ms: started_at_epoch_ms,
            last_check_completed_at_epoch_ms: completed_at_epoch_ms,
            active_run_detected_during_last_check: active_run_detected,
            real_update_check_materialized: true,
            real_update_download_materialized: failure.download_materialized,
            real_update_verification_materialized: failure.verification_materialized,
            real_update_apply_materialized: false,
            last_error_detail: Some(failure.detail),
            last_discovered_version: failure
                .discovered_update
                .as_ref()
                .map(|update| update.version.clone()),
            last_discovered_release_notes: failure
                .discovered_update
                .as_ref()
                .and_then(|update| update.release_notes.clone()),
            last_discovered_publication_date: failure
                .discovered_update
                .as_ref()
                .and_then(|update| update.publication_date.clone()),
        },
    }
}

#[cfg(test)]
pub(crate) fn mock_update_available_runtime_state_for_test(
    active_run_detected: bool,
) -> NativeAppUpdaterRuntimeState {
    completed_runtime_state(
        active_run_detected,
        Some(100),
        Some(200),
        NativeAppUpdaterCheckOutcome::UpdateAvailable(DiscoveredNativeUpdate {
            version: "1.2.3".to_string(),
            release_notes: Some("Fresh improvements".to_string()),
            publication_date: Some("2026-04-17T12:00:00Z".to_string()),
        }),
    )
}

#[cfg(test)]
pub(crate) fn mock_update_ready_runtime_state_for_test(
    active_run_detected_during_last_check: bool,
) -> NativeAppUpdaterRuntimeState {
    NativeAppUpdaterRuntimeState {
        current_state: UpdaterState::UpdateReady,
        current_update_available: true,
        current_update_ready: true,
        current_candidate_rejected: false,
        current_error_active: false,
        current_status_note:
            "The shell verified a native app update candidate and promoted it to UpdateReady."
                .to_string(),
        last_check_attempted: true,
        last_check_started_at_epoch_ms: Some(100),
        last_check_completed_at_epoch_ms: Some(200),
        active_run_detected_during_last_check,
        real_update_check_materialized: true,
        real_update_download_materialized: true,
        real_update_verification_materialized: true,
        real_update_apply_materialized: false,
        last_error_detail: None,
        last_discovered_version: Some("1.2.3".to_string()),
        last_discovered_release_notes: Some("Fresh improvements".to_string()),
        last_discovered_publication_date: Some("2026-04-17T12:00:00Z".to_string()),
    }
}

pub(crate) fn native_app_updater_apply_gate_for_runtime_state(
    runtime_state: &NativeAppUpdaterRuntimeState,
    active_run_detected: bool,
) -> NativeAppUpdaterApplyGateState {
    if runtime_state.current_state == UpdaterState::ApplyingUpdate {
        return NativeAppUpdaterApplyGateState {
            apply_allowed_now: false,
            apply_blocked_by_active_run: false,
            apply_blocked_because_update_not_ready: false,
            restart_coordination_required: true,
            apply_status_note:
                "The shell is applying the verified app update and restart coordination is in progress."
                    .to_string(),
        };
    }

    if runtime_state.current_state == UpdaterState::UpdateApplied {
        return NativeAppUpdaterApplyGateState {
            apply_allowed_now: false,
            apply_blocked_by_active_run: false,
            apply_blocked_because_update_not_ready: false,
            restart_coordination_required: true,
            apply_status_note:
                "The shell applied the verified app update and requested app restart to activate the new build."
                    .to_string(),
        };
    }

    if runtime_state.current_state == UpdaterState::UpdateReady && runtime_state.current_update_ready {
        if active_run_detected {
            return NativeAppUpdaterApplyGateState {
                apply_allowed_now: false,
                apply_blocked_by_active_run: true,
                apply_blocked_because_update_not_ready: false,
                restart_coordination_required: true,
                apply_status_note:
                    "A verified app update is ready, but apply remains blocked until the current run is terminal and restart coordination is safe."
                        .to_string(),
            };
        }

        return NativeAppUpdaterApplyGateState {
            apply_allowed_now: true,
            apply_blocked_by_active_run: false,
            apply_blocked_because_update_not_ready: false,
            restart_coordination_required: true,
            apply_status_note:
                "A verified app update is ready for explicit shell-owned apply and restart coordination."
                    .to_string(),
        };
    }

    NativeAppUpdaterApplyGateState {
        apply_allowed_now: false,
        apply_blocked_by_active_run: false,
        apply_blocked_because_update_not_ready: true,
        restart_coordination_required: false,
        apply_status_note:
            "Update apply remains blocked until the shell reaches a verified UpdateReady candidate."
                .to_string(),
    }
}

pub(crate) fn active_run_detected_for_updater_coordination() -> bool {
    let run_lifecycle_state = run_lifecycle_state_from_shell();

    !run_lifecycle_state.current_state_terminal
        && run_lifecycle_state.current_state != CanonicalRunLifecycleState::RunDraft
}

fn verify_downloaded_native_update_artifact(
    version: &str,
    signature: &str,
    download_url: &str,
    artifact_bytes: &[u8],
) -> Result<(), String> {
    if artifact_bytes.is_empty() {
        return Err(
            "the updater returned an empty artifact after download and verification".to_string(),
        );
    }

    if version.trim().is_empty() {
        return Err(
            "the updater returned a candidate without a stable version identifier".to_string(),
        );
    }

    if signature.trim().is_empty() {
        return Err(
            "the updater returned a candidate without signature metadata".to_string(),
        );
    }

    let download_url = Url::parse(download_url).map_err(|error| {
        format!(
            "the updater returned an invalid download URL for version {}: {error}",
            version,
        )
    })?;

    if download_url.scheme() != "https" {
        return Err(format!(
            "the updater returned a non-HTTPS download URL for version {}",
            version,
        ));
    }

    Ok(())
}

fn preparation_failure_kind_from_error_debug_label(
    error_debug_label: &str,
) -> NativeAppUpdaterPreparationFailureKind {
    let debug_label = error_debug_label.to_ascii_lowercase();

    if [
        "minisign",
        "base64",
        "signatureutf8",
        "invalidupdaterformat",
        "authenticationfailed",
        "unsupportedarch",
        "unsupportedos",
        "targetnotfound",
        "targetsnotfound",
        "binarynotfoundinarchive",
        "insecuretransportprotocol",
    ]
    .iter()
    .any(|marker| debug_label.contains(marker))
    {
        NativeAppUpdaterPreparationFailureKind::RejectedCandidate
    } else {
        NativeAppUpdaterPreparationFailureKind::TechnicalFailure
    }
}

fn non_empty_trimmed_option(value: String) -> Option<String> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn current_epoch_millis() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_requires_endpoints_and_pubkey_without_invalid_urls() {
        let (status, resolved_configuration) = resolve_native_app_updater_configuration_from_values(
            Some("https://updates.example.com/latest.json; not-a-url"),
            Some("pubkey-value"),
        );

        assert!(!status.configuration_ready);
        assert!(status.pubkey_configured);
        assert!(status.endpoints_configured);
        assert_eq!(status.endpoint_count, 1);
        assert!(status
            .configuration_note
            .contains("invalid URL values"));
        assert!(resolved_configuration.is_none());
    }

    #[test]
    fn completed_runtime_state_distinguishes_no_update_available_from_error() {
        let no_update = completed_runtime_state(
            false,
            Some(100),
            Some(200),
            NativeAppUpdaterCheckOutcome::NoUpdateAvailable,
        );
        let failure = completed_runtime_state(
            false,
            Some(100),
            Some(200),
            NativeAppUpdaterCheckOutcome::TechnicalFailure("network timeout".to_string()),
        );

        assert_eq!(no_update.current_state, UpdaterState::NoUpdateAvailable);
        assert!(!no_update.current_error_active);
        assert!(no_update.real_update_check_materialized);
        assert_eq!(failure.current_state, UpdaterState::UpdaterError);
        assert!(failure.current_error_active);
        assert!(failure.real_update_check_materialized);
        assert_eq!(failure.last_error_detail.as_deref(), Some("network timeout"));
    }

    #[test]
    fn completed_runtime_state_distinguishes_update_available_from_ready() {
        let update_available = completed_runtime_state(
            true,
            Some(100),
            Some(200),
            NativeAppUpdaterCheckOutcome::UpdateAvailable(DiscoveredNativeUpdate {
                version: "1.2.3".to_string(),
                release_notes: Some("Fresh improvements".to_string()),
                publication_date: Some("2026-04-17 12:00:00.0 +00:00:00".to_string()),
            }),
        );

        assert_eq!(update_available.current_state, UpdaterState::UpdateAvailable);
        assert!(update_available.current_update_available);
        assert!(!update_available.current_update_ready);
        assert!(!update_available.current_error_active);
        assert_eq!(
            update_available.last_discovered_version.as_deref(),
            Some("1.2.3")
        );
        assert!(update_available.active_run_detected_during_last_check);
        assert!(update_available
            .current_status_note
            .contains("explicit preparation command"));
    }

    #[test]
    fn completed_preparation_runtime_state_promotes_verified_candidate_to_ready() {
        let update_ready = completed_preparation_runtime_state(
            false,
            Some(100),
            Some(200),
            NativeAppUpdaterPrepareOutcome::UpdateReady(PreparedNativeUpdate {
                discovered_update: DiscoveredNativeUpdate {
                    version: "1.2.3".to_string(),
                    release_notes: Some("Fresh improvements".to_string()),
                    publication_date: Some("2026-04-17T12:00:00Z".to_string()),
                },
                artifact_byte_count: 4096,
            }),
        );

        assert_eq!(update_ready.current_state, UpdaterState::UpdateReady);
        assert!(update_ready.current_update_available);
        assert!(update_ready.current_update_ready);
        assert!(!update_ready.current_candidate_rejected);
        assert!(update_ready.real_update_download_materialized);
        assert!(update_ready.real_update_verification_materialized);
        assert!(update_ready.current_status_note.contains("promoted it to UpdateReady"));
    }

    #[test]
    fn completed_apply_runtime_state_marks_update_applied_and_restart_requested() {
        let update_applied = completed_apply_runtime_state(
            &mock_update_ready_runtime_state_for_test(false),
            &PreparedNativeUpdate {
                discovered_update: DiscoveredNativeUpdate {
                    version: "1.2.3".to_string(),
                    release_notes: Some("Fresh improvements".to_string()),
                    publication_date: Some("2026-04-17T12:00:00Z".to_string()),
                },
                artifact_byte_count: 4096,
            },
        );

        assert_eq!(update_applied.current_state, UpdaterState::UpdateApplied);
        assert!(!update_applied.current_update_ready);
        assert!(update_applied.real_update_apply_materialized);
        assert!(update_applied
            .current_status_note
            .contains("requested app restart"));
    }

    #[test]
    fn apply_error_runtime_state_marks_apply_failure_as_updater_error() {
        let apply_error = apply_error_runtime_state(
            &mock_update_ready_runtime_state_for_test(false),
            Some(&DiscoveredNativeUpdate {
                version: "1.2.3".to_string(),
                release_notes: Some("Fresh improvements".to_string()),
                publication_date: Some("2026-04-17T12:00:00Z".to_string()),
            }),
            "installer launch failed".to_string(),
            true,
        );

        assert_eq!(apply_error.current_state, UpdaterState::UpdaterError);
        assert!(apply_error.current_error_active);
        assert!(apply_error.real_update_apply_materialized);
        assert_eq!(
            apply_error.last_error_detail.as_deref(),
            Some("installer launch failed")
        );
    }

    #[test]
    fn completed_preparation_runtime_state_distinguishes_rejection_from_error() {
        let rejected = completed_preparation_runtime_state(
            false,
            Some(100),
            Some(200),
            NativeAppUpdaterPrepareOutcome::UpdateRejected(RejectedNativeUpdate {
                discovered_update: DiscoveredNativeUpdate {
                    version: "1.2.3".to_string(),
                    release_notes: None,
                    publication_date: None,
                },
                detail: "signature verification failed".to_string(),
            }),
        );
        let technical_failure = completed_preparation_runtime_state(
            false,
            Some(100),
            Some(200),
            NativeAppUpdaterPrepareOutcome::TechnicalFailure(PrepareTechnicalFailure {
                discovered_update: Some(DiscoveredNativeUpdate {
                    version: "1.2.3".to_string(),
                    release_notes: None,
                    publication_date: None,
                }),
                detail: "network timeout".to_string(),
                download_materialized: true,
                verification_materialized: false,
            }),
        );

        assert_eq!(rejected.current_state, UpdaterState::UpdateRejected);
        assert!(rejected.current_candidate_rejected);
        assert!(!rejected.current_error_active);
        assert_eq!(technical_failure.current_state, UpdaterState::UpdaterError);
        assert!(!technical_failure.current_candidate_rejected);
        assert!(technical_failure.current_error_active);
        assert_eq!(
            technical_failure.last_error_detail.as_deref(),
            Some("network timeout")
        );
    }

    #[test]
    fn verification_rejects_empty_downloaded_artifact() {
        let verification = verify_downloaded_native_update_artifact(
            "1.2.3",
            "signed-value",
            "https://updates.example.com/miro-fish.zip",
            &[],
        );

        assert_eq!(
            verification,
            Err("the updater returned an empty artifact after download and verification".to_string())
        );
    }

    #[test]
    fn preparation_failure_classifies_signature_and_transport_differently() {
        assert_eq!(
            preparation_failure_kind_from_error_debug_label("Minisign(signature mismatch)"),
            NativeAppUpdaterPreparationFailureKind::RejectedCandidate
        );
        assert_eq!(
            preparation_failure_kind_from_error_debug_label("Reqwest(timeout reached)"),
            NativeAppUpdaterPreparationFailureKind::TechnicalFailure
        );
    }

    #[test]
    fn apply_gate_blocks_ready_update_during_active_run() {
        let gate = native_app_updater_apply_gate_for_runtime_state(
            &mock_update_ready_runtime_state_for_test(true),
            true,
        );

        assert!(!gate.apply_allowed_now);
        assert!(gate.apply_blocked_by_active_run);
        assert!(!gate.apply_blocked_because_update_not_ready);
        assert!(gate.restart_coordination_required);
    }

    #[test]
    fn apply_gate_allows_ready_update_when_idle() {
        let gate = native_app_updater_apply_gate_for_runtime_state(
            &mock_update_ready_runtime_state_for_test(false),
            false,
        );

        assert!(gate.apply_allowed_now);
        assert!(!gate.apply_blocked_by_active_run);
        assert!(!gate.apply_blocked_because_update_not_ready);
        assert!(gate.restart_coordination_required);
    }

    #[test]
    fn apply_gate_rejects_non_ready_state() {
        let gate = native_app_updater_apply_gate_for_runtime_state(
            &mock_update_available_runtime_state_for_test(false),
            false,
        );

        assert!(!gate.apply_allowed_now);
        assert!(!gate.apply_blocked_by_active_run);
        assert!(gate.apply_blocked_because_update_not_ready);
        assert!(!gate.restart_coordination_required);
    }

    #[test]
    fn apply_gate_reports_apply_in_progress() {
        let gate = native_app_updater_apply_gate_for_runtime_state(
            &NativeAppUpdaterRuntimeState {
                current_state: UpdaterState::ApplyingUpdate,
                current_update_available: true,
                current_update_ready: false,
                current_candidate_rejected: false,
                current_error_active: false,
                current_status_note: "Applying".to_string(),
                last_check_attempted: true,
                last_check_started_at_epoch_ms: Some(100),
                last_check_completed_at_epoch_ms: Some(200),
                active_run_detected_during_last_check: false,
                real_update_check_materialized: true,
                real_update_download_materialized: true,
                real_update_verification_materialized: true,
                real_update_apply_materialized: true,
                last_error_detail: None,
                last_discovered_version: Some("1.2.3".to_string()),
                last_discovered_release_notes: Some("Fresh improvements".to_string()),
                last_discovered_publication_date: Some("2026-04-17T12:00:00Z".to_string()),
            },
            false,
        );

        assert!(!gate.apply_allowed_now);
        assert!(!gate.apply_blocked_because_update_not_ready);
        assert!(gate.restart_coordination_required);
        assert!(gate.apply_status_note.contains("restart coordination is in progress"));
    }

    #[test]
    fn apply_gate_reports_applied_update_restart_request() {
        let gate = native_app_updater_apply_gate_for_runtime_state(
            &NativeAppUpdaterRuntimeState {
                current_state: UpdaterState::UpdateApplied,
                current_update_available: false,
                current_update_ready: false,
                current_candidate_rejected: false,
                current_error_active: false,
                current_status_note: "Applied".to_string(),
                last_check_attempted: true,
                last_check_started_at_epoch_ms: Some(100),
                last_check_completed_at_epoch_ms: Some(200),
                active_run_detected_during_last_check: false,
                real_update_check_materialized: true,
                real_update_download_materialized: true,
                real_update_verification_materialized: true,
                real_update_apply_materialized: true,
                last_error_detail: None,
                last_discovered_version: Some("1.2.3".to_string()),
                last_discovered_release_notes: Some("Fresh improvements".to_string()),
                last_discovered_publication_date: Some("2026-04-17T12:00:00Z".to_string()),
            },
            false,
        );

        assert!(!gate.apply_allowed_now);
        assert!(!gate.apply_blocked_because_update_not_ready);
        assert!(gate.restart_coordination_required);
        assert!(gate.apply_status_note.contains("requested app restart"));
    }
}