# Task Packet

## Task ID
P07-WS07-T03

## Task Title
Implement Shell Status Projection, Ordered Event History, and Monitoring Retrieval Path

## Parent Phase
Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

## Parent Workstream
WS-07 — Run State, Events, and Live Monitoring

## Objective
Implement the real shell-owned live-status projection and ordered event-history retrieval path over existing run/session and workspace artifacts, including deterministic cursor or sequence retrieval, transcript or event chronology, and live budget or duration visibility, without yet implementing the user-facing live monitoring UI, paused or degraded runtime handling, controlled cancellation behavior, final-report truth, or cleanup hardening.

## Why This Exists
`P07-WS07-T02` established the canonical lifecycle vocabulary, shell-owned status retrieval contract shape, cancellation request or response basis, and ordered event-envelope foundation, but the repo still does not expose real ordered event history, real latest visible sequence truth, or status projection grounded in append-only run artifacts. The current shell status surface still depends primarily on the coarse run-session snapshot, and the current event-contract surface still reports foundation-only semantics with no materialized chronology. At the same time, the Phase 6 execution path already produces transcript-archive and token or duration tracking artifacts inside the per-run workspace. Phase 7 Wave 3 exists to connect those real local artifacts to truthful shell-owned status and event retrieval before any Wave 4 UI feed rendering or Wave 5 degradation or cancellation control work begins.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
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
- Document B Section 6.9 — Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- `execution/phase_06_master_execution_plan.md`
- `execution/phase_07_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p07_ws07_t01_establish_phase_7_run_state_events_live_monitoring_and_interruption_handling_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/task_packets/p07_ws07_t02_establish_canonical_run_lifecycle_exposure_status_contract_and_event_envelope_foundation.md` is complete.
- `execution/phase_07_master_execution_plan.md` remains the authoritative Phase 7 packetization basis.
- Existing Phase 6 run/session execution, transcript-archive, bounded-memory, and token or duration tracking artifacts remain available as live repo capabilities.
- Rust/Cargo/Clippy and Python engine validation tooling remain available locally.

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
- `execution/phase_06_master_execution_plan.md`
- `execution/phase_07_master_execution_plan.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_token_duration_tracking.rs`
- `engine/src/miro_fish_engine/transcript_archive.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/session_execution.py`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_token_duration_tracking.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_history.rs`
- `engine/src/miro_fish_engine/transcript_archive.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/tests/test_transcript_archive.py`
- `engine/tests/test_token_duration_tracking.py`
- `engine/tests/test_moderated_execution.py`
- `engine/tests/test_session_execution.py`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved event-history or chronology truth changes materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p07_ws07_t03_implement_shell_status_projection_ordered_event_history_and_monitoring_retrieval_path.md`

## Implementation Scope
In scope:
- real shell-owned status projection over the current run/session snapshot plus real workspace-local transcript and tracking artifacts
- ordered event-history materialization and retrieval with deterministic sequence numbers, cursor boundaries, latest-available sequence truth, and honest gap signaling
- transcript or event chronology surfaces derived from real lifecycle, debate, and tracking evidence already produced by the current run execution path
- live budget or duration visibility grounded in the existing run-local token or duration tracking artifacts
- narrow local engine-artifact enrichment only where required to preserve truthful `occurred_at`, chronology ordering, or latest-sequence semantics inside the governed event envelope
- shell lifecycle, supervision, trust-boundary, startup, interface, and command integration needed to expose the new status and event retrieval truth honestly
- shell and engine tests for status projection, event ordering, chronology retrieval, gap handling, and live usage visibility

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for lifecycle truth, status projection, event ordering, latest-sequence truth, and status or event reconciliation
- keep the frontend out of scope for user-visible live feed rendering during this packet so Wave 4 retains ownership of monitoring UI behavior
- keep the Python engine as the producer of raw local execution artifacts rather than the authority over external status or event semantics
- preserve the frozen canonical run-state family and avoid exposing the old Phase 6 coarse state classes as a competing lifecycle vocabulary
- preserve the distinction between normal progress, constrained completion, degraded states, cancellation, retryable failure, terminal failure, finalization, and cleanup semantics
- preserve append-only local chronology honesty; if required evidence is missing or malformed, surface that truth explicitly instead of fabricating continuity
- preserve the no-crash-resume, no-hidden-continuation, and no-frontend-authority rules
- keep status snapshots and ordered event retrieval mutually reconcilable so the UI never has to guess which surface is canonical

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- the user-visible live monitoring UI surface, incremental React feed rendering, or other Wave 4 frontend behavior
- paused-rate-limit or paused-network runtime behavior, retry logic, or degraded-state recovery behavior
- controlled cancellation execution, shutdown integration, or no-hidden-post-cancel enforcement behavior
- final-report generation, report availability truth, export behavior, cleanup hardening, janitor behavior, updater behavior, or runtime hydration behavior
- hosted orchestration behavior, public API behavior, commercial-backend expansion, or hidden remote event persistence
- fabricated event reconstruction, implicit recovery inference, or fake `occurred_at` timestamps unsupported by local evidence

## Deliverables
- `shell/src/run_status_projection.rs` exists and materializes real shell-owned status projection from the current run/session plus real local execution artifacts
- `shell/src/run_event_history.rs` exists and materializes ordered event-history retrieval with deterministic sequence and cursor semantics consistent with the Wave 2 event contract
- `shell/src/run_status_contract.rs` and `shell/src/run_event_contract.rs` now report materialized status or event-history truth where the repo can support it honestly
- live budget or duration visibility is exposed through the shell-owned status or monitoring retrieval path using existing run-local tracking artifacts
- transcript or event chronology is derived from real local transcript and tracking evidence rather than placeholders or frontend inference
- any required engine artifact enrichment stays bounded to local transcript or tracking chronology metadata and does not create a second unsupported supervision system
- frontend live monitoring rendering remains deferred to Phase 7 Wave 4

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests` complete cleanly if engine files change
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if any frontend files change
- confirm status snapshots, latest visible sequence markers, and ordered event retrieval are grounded in real local run evidence and tell the same story
- confirm chronological retrieval is deterministic enough for later incremental UI synchronization and does not allow silent missing-history inference
- confirm the delivered surfaces do not imply paused or degraded handling, controlled cancellation execution, final-report truth, or cleanup success when that behavior is still not implemented
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful ordered event envelopes cannot be produced from governed local evidence without inventing unsupported states, events, timestamps, or sequence boundaries
- required chronology metadata would force a broader engine supervision or hosted persistence design not already authorized by Documents A, B, or C
- status projection and event retrieval cannot remain mutually reconcilable without frontend inference or hidden reconstruction logic
- narrow engine artifact enrichment would spill into Wave 4 UI behavior, Wave 5 degradation or cancellation behavior, Phase 8 report behavior, or Phase 9 cleanup hardening
- any applicable shell or engine gate still reports warnings or errors after in-scope fixes

## Current State
COMPLETE — 2026-04-15

## Completion Notes
- Completed the bounded Phase 7 Wave 3 implementation by materializing shell-owned status projection in `shell/src/run_status_projection.rs` and ordered event-history retrieval in `shell/src/run_event_history.rs`.
- Integrated the new retrieval and projection truth through `shell/src/main.rs`, `shell/src/run_status_contract.rs`, `shell/src/run_event_contract.rs`, `shell/src/run_session_contract.rs`, `shell/src/local_interface_contract.rs`, `shell/src/engine_supervision.rs`, `shell/src/app_lifecycle.rs`, and `shell/src/startup_checks.rs` without expanding into Wave 4 live-monitoring UI rendering or Wave 5 degradation/cancellation behavior.
- Added bounded engine artifact chronology evidence in `engine/src/miro_fish_engine/moderated_execution.py` and `engine/src/miro_fish_engine/token_duration_tracking.py` so transcript and token/duration records now carry truthful local `occurred_at` timestamps.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests`.

## Structural Consistency and Contradiction Review
- Confirmed the delivered status projection, latest visible sequence, ordered event retrieval, and chronology surfaces all reconcile against the same governed local run artifacts.
- Confirmed no frontend live-monitoring UI behavior, paused/degraded runtime handling, controlled cancellation execution, final-report truth, or cleanup hardening was implied or implemented in this packet.
- Confirmed only allowlisted shell, engine, test, and execution-state files changed for this packet and no contradiction remained between the packet scope and the live execution surfaces at closeout.