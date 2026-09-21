# Task Packet

## Task ID
P07-WS07-T06

## Task Title
Implement Degradation, Retry, Controlled Cancellation, and Shutdown Integration

## Parent Phase
Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

## Parent Workstream
WS-07 — Run State, Events, and Live Monitoring

## Objective
Implement the bounded Phase 7 Wave 6 interruption path by materializing truthful paused-rate-limit and paused-network behavior, bounded retry-versus-failure handling, real controlled cancellation execution, and shell-supervised graceful-then-forceful shutdown coordination for active runs, without implying Phase 8 report truth, Phase 9 cleanup hardening, push-streaming supervision, crash-resume behavior, or janitor completion.

## Why This Exists
`P07-WS07-T05` completed the shell-owned in-flight monitoring basis, so the remaining governed Phase 7 gap is no longer live-visibility infrastructure. The repo still lacks real paused/degraded runtime behavior, bounded retry classification, implemented cancellation acceptance, and active-run app-close integration that can stop normal forward execution truthfully. Documents A, B, and C require these behaviors for `AC-075` through `AC-085`, `UI-050` through `UI-057`, `BEHAV-079` through `BEHAV-082`, `API-065` through `API-068`, and the canonical paused/degraded, retryable-failure, canceled, and shutdown-coordination state path. This packet exists to close that bounded interruption/control gap before the Phase 7 exit-gate packet is authored.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
- Document A Section 5 — User Journeys and Interaction Flows
- Document A Section 6 — Product Behavior Specification
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 10 — API and Interface Contract
- Document A Section 12 — UI/UX Contract
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.10 — WS-07 — Run State, Events, and Live Monitoring
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.10 — Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling
- Document B Section 6.9.9 — Validation Gate
- Document B Section 6.9.10 — Failure / Reopen Conditions
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_07_master_execution_plan.md`
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md`
- `execution/task_packets/p02_ws02_t05_establish_controlled_shutdown_and_local_interface_contract_scaffold.md`

## Dependency Preconditions
- `execution/task_packets/p07_ws07_t01_establish_phase_7_run_state_events_live_monitoring_and_interruption_handling_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/task_packets/p07_ws07_t02_establish_canonical_run_lifecycle_exposure_status_contract_and_event_envelope_foundation.md` is complete.
- `execution/task_packets/p07_ws07_t03_implement_shell_status_projection_ordered_event_history_and_monitoring_retrieval_path.md` is complete.
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md` is complete.
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md` is complete and remains the in-flight monitoring prerequisite for this packet.
- `execution/phase_07_master_execution_plan.md` remains the authoritative Phase 7 packetization basis.
- The Phase 2 controlled-shutdown scaffold exists but remains boundary-only and non-operational until this packet materializes active-run shutdown coordination truthfully.
- No other packet may remain active while this packet is active.

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md`
- `execution/phase_07_master_execution_plan.md`
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md`
- `execution/task_packets/p02_ws02_t05_establish_controlled_shutdown_and_local_interface_contract_scaffold.md`
- `shell/src/main.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave7.test.tsx`
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/transcript_archive.py` only if bounded interruption chronology requires new local evidence markers
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `engine/tests/test_token_duration_tracking.py`
- `engine/tests/test_transcript_archive.py` only if interruption chronology evidence changes

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave7.test.tsx`
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/transcript_archive.py` only if bounded interruption chronology requires new local evidence markers
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `engine/tests/test_token_duration_tracking.py`
- `engine/tests/test_transcript_archive.py` only if interruption chronology evidence changes
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if a new unresolved interruption/control question appears during implementation
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` only if a new local governance decision becomes necessary during implementation
- `execution/task_packets/p07_ws07_t06_implement_degradation_retry_controlled_cancellation_and_shutdown_integration.md`

## Implementation Scope
In scope:
- shell-owned implementation of the governed cancellation command and response path, including accepted, rejected, already-terminal, and no-active-run outcomes
- canonical lifecycle, status, and ordered-event integration for `PausedRateLimit`, `PausedNetwork`, `FailedRetryable`, `FailedTerminal`, `Canceled`, and valid shutdown-associated interruption truth where the repo can know those states honestly
- bounded retry behavior for retryable provider/network interruptions, including retry-attempt visibility and retryable-versus-terminal distinction only where the system actually knows it
- no-hidden-post-cancel progression enforcement so accepted cancellation stops normal forward round scheduling and cannot be reported as healthy progress
- shell-supervised active-run app-close coordination that attempts graceful engine shutdown first and uses forceful fallback only when required by governed shutdown policy
- bounded frontend live-monitoring updates that render degraded state truth, retry-vs-terminal status, cancel availability, cancellation acceptance, and shutdown-associated run truth from shell-owned surfaces only
- shell, engine, and frontend tests proving degraded-state honesty, bounded retry visibility, accepted-cancel halt behavior, and graceful-then-forceful shutdown coordination without implying resume or cleanup hardening

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole authority for current lifecycle truth, cancelability, cancellation acceptance, shutdown coordination, and status/event reconciliation
- keep the frontend as a consumer of shell-owned truth only; it must not infer degraded recovery, cancelability, cancellation success, or shutdown completion on its own
- keep the Python engine as the producer of underlying execution activity rather than the authority for external lifecycle or contract meaning
- preserve the frozen canonical run-state family and the governed transition distinctions among paused/degraded, retryable failure, terminal failure, cancellation, finalization, completion, and cleanup-related states
- preserve the bounded local one-shot execution model and do not introduce hosted orchestration, detached background continuation, or hidden daemon supervision
- preserve the no-crash-resume, no-hidden-continuation, and no-frontend-authority rules
- preserve Phase 8 and Phase 9 separation so this packet does not imply report correctness, report availability, cleanup success, janitor behavior, or cleanup hardening
- preserve deterministic chronology and explicit distinction between degraded pause, retryable failure, terminal failure, cancellation, and shutdown initiation rather than collapsing them into generic technical failure

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- Phase 8 report generation, report availability, report export, or output-preservation behavior
- Phase 9 cleanup hardening, startup janitor behavior, cleanup-success guarantees, or cleanup outcome validation beyond the bounded shutdown-control truth required by this packet
- push-streaming supervision, hosted event relays, public API expansion, or hidden provider-health channels
- unbounded retry loops, speculative retry heuristics, or fake recovery claims when retry status is not actually known
- crash-resume behavior, interrupted-run restoration, hidden background continuation, or janitor-as-resume semantics
- updater/runtime hydration behavior, commercial-backend expansion, or unrelated provider-configuration feature work
- frontend-only lifecycle authority, frontend-generated cancellation state, or UI wording that treats cancellation/shutdown acknowledgment as run success

## Deliverables
- the shell-owned cancellation command is implemented and exposed through the allowlisted local interface with truthful accepted/rejected/already-terminal/no-active outcomes
- the shell lifecycle, status, and ordered-event surfaces can expose paused rate-limit/network truth, bounded retryable-versus-terminal distinction where known, and cancellation/shutdown-associated run truth without contradictory stories
- accepted cancellation and active-run app-close handling stop normal forward run progression and do not allow hidden post-cancel debate continuation
- `shell/src/controlled_shutdown.rs` and adjacent lifecycle/supervision surfaces now reflect real graceful-then-forceful shutdown coordination truth instead of scaffold-only posture
- the frontend renders cancel controls only when shell-owned cancelability truth allows them and surfaces degraded/canceled/shutdown truth without implying report or cleanup success
- regression coverage proves degraded-state honesty, retry-vs-failure distinction, implemented cancel behavior, and active-run shutdown coordination

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` complete cleanly if frontend files change
- confirm `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests` complete cleanly if engine files change
- confirm degraded states are not mislabeled as healthy progress and retryable-versus-terminal distinction appears only where the repo knows it truthfully
- confirm accepted cancellation stops normal forward run progression and no hidden post-cancel round scheduling remains
- confirm normal app close during an active run attempts graceful shutdown first and uses forceful fallback only when required, without implying run success, cleanup success, or report availability
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful paused/degraded detection would require undeclared provider-health channels, hidden persistence, or new hosted supervision surfaces not already authorized by Documents A, B, or C
- bounded retry/cancellation/shutdown behavior cannot be enforced without violating the no-hidden-continuation or no-resume rules
- shutdown integration would require Phase 9 cleanup hardening, startup janitor behavior, or cleanup-outcome guarantees to remain coherent
- cancellation acceptance cannot remain shell-owned or accepted cancellation cannot halt normal round progression truthfully in the existing bounded execution model
- any applicable shell, frontend, or engine gate reports warnings or errors after in-scope remediation

## Current State
COMPLETE — 2026-04-16

## Assignment Notes
- Activated as the current Phase 7 Wave 6 packet for paused/degraded truth, bounded retry handling, controlled cancellation execution, and active-run shutdown integration over the completed T04/T05 monitoring basis.
- Repo changes are now authorized only within the scope of this packet until closeout or blockage.
- Phase 8 report truth and Phase 9 cleanup hardening remain explicitly out of scope while this packet focuses on interruption/control truth during active execution.

## Completion Notes
- `shell/src/run_session_start.rs` now starts runs asynchronously under shell-owned worker supervision, supervises the active Python child process, publishes runtime-control flags and supplemental control events during retry or cancellation or shutdown transitions, uses a workspace-local `execution_control.json` file for graceful interruption signaling, and applies graceful-then-forceful termination fallback when required.
- `shell/src/run_lifecycle_state.rs`, `shell/src/run_status_contract.rs`, `shell/src/run_status_projection.rs`, `shell/src/run_event_contract.rs`, `shell/src/run_event_history.rs`, `shell/src/controlled_shutdown.rs`, `shell/src/app_lifecycle.rs`, `shell/src/local_interface_contract.rs`, and `shell/src/engine_supervision.rs` now expose paused/degraded truth, bounded retry visibility, implemented cancellation behavior, live interruption chronology, and active-run shutdown coordination without implying report or cleanup success.
- `engine/src/miro_fish_engine/moderated_execution.py` and `engine/src/miro_fish_engine/session_execution.py` now honor shell-authored execution-control signals at safe round and turn boundaries, while `app/src/App.tsx` plus the updated Wave 7 frontend coverage now surface shell-owned cancel and controlled-shutdown controls together with retry or degradation feedback.
- `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests` completed cleanly, and the structural consistency review found no material contradiction that required in-scope remediation.