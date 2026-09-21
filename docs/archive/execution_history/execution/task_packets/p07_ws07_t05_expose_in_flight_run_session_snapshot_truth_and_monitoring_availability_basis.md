# Task Packet

## Task ID
P07-WS07-T05

## Task Title
Expose In-Flight Run/Session Snapshot Truth and Monitoring Availability Basis

## Parent Phase
Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

## Parent Workstream
WS-07 — Run State, Events, and Live Monitoring

## Objective
Implement the bounded shell-owned in-flight run/session snapshot publication path needed for honest live monitoring during active execution, so current run/session identity, workspace reference, and monitoring-availability truth become visible before the one-shot execution returns, without introducing push streaming, degraded-state logic, controlled cancellation behavior, final-report truth, or cleanup hardening.

## Why This Exists
`P07-WS07-T04` implemented the Wave 4 monitoring UI surface and passed the frontend gate family cleanly, but the packet cannot close honestly because `shell/src/run_session_start.rs` currently persists `state.latest_snapshot` only after one-shot moderated execution returns. That leaves `run_session_state`, `run_status_state`, and `run_event_history` without shell-owned in-flight run/session identity during active execution, which contradicts the live-visibility intent of `AC-070` through `AC-074`. This packet exists to close that exact shell-owned contradiction before later degradation/cancellation work begins.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
- Document A Section 6 — Product Behavior Specification
- Document A Section 7 — State Machines
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
- `execution/phase_07_master_execution_plan.md`
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md`

## Dependency Preconditions
- `execution/task_packets/p07_ws07_t01_establish_phase_7_run_state_events_live_monitoring_and_interruption_handling_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/task_packets/p07_ws07_t02_establish_canonical_run_lifecycle_exposure_status_contract_and_event_envelope_foundation.md` is complete.
- `execution/task_packets/p07_ws07_t03_implement_shell_status_projection_ordered_event_history_and_monitoring_retrieval_path.md` is complete.
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md` is BLOCKED and remains the authoritative reason this prerequisite packet exists.
- `execution/phase_07_master_execution_plan.md` is reopened locally to insert this bounded prerequisite before degradation/cancellation work.
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
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md`
- `shell/src/main.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/startup_checks.rs`
- `shell/src/engine_supervision.rs`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_lifecycle_state.rs` only if bounded lifecycle/status wording must reflect the new in-flight publication basis truthfully
- `shell/src/app_lifecycle.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/startup_checks.rs`
- `shell/src/engine_supervision.rs` only if bounded shell supervision truth must reflect the new in-flight publication basis
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if a new unresolved shell-truth question appears during implementation
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` only if a new local governance decision becomes necessary during implementation
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md`

## Implementation Scope
In scope:
- shell-owned publication of a nonterminal current run/session snapshot before one-shot execution completes
- bounded in-flight snapshot updates at truthful shell checkpoints such as identity allocation, workspace allocation, seed staging, foundation readiness, and active execution handoff where those checkpoints are already knowable inside the shell
- current run/session identity, workspace reference, and monitoring-availability truth sufficient for `run_session_state`, `run_status_state`, and `run_event_history` to respond honestly during active execution
- explicit shell-owned pending-history or not-yet-materialized messaging where ordered chronology is still unavailable rather than fabricated sequence progress
- narrow shell lifecycle, startup, interface, supervision, and contract truth-surface refinements needed to reflect the new in-flight publication basis
- shell tests proving in-flight snapshot publication, truthful status/history availability messaging, and no contradiction with the existing one-shot execution model

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole authority for current run/session identity, lifecycle truth, monitoring availability, event ordering, and cancelability meaning
- preserve the existing one-shot execution model unless a strictly bounded shell snapshot publication change is enough to expose truthful in-flight visibility
- preserve the frozen canonical run-state family and never treat coarse run-session classes as a second competing lifecycle vocabulary
- preserve explicit distinction between `history not yet materialized` and `history complete` rather than inventing synthetic sequence progress
- preserve the no-crash-resume, no-hidden-continuation, and no-frontend-authority rules
- preserve Phase 8 and Phase 9 separation so this packet does not imply report availability, report correctness, cleanup success, or janitor completion
- preserve the existing prohibition on push-streaming claims or hidden engine event/status channels when those surfaces do not yet exist

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- frontend monitoring UI changes beyond what `P07-WS07-T04` already delivered
- engine-side incremental event emission, hosted relays, or a new live push/status channel
- paused-rate-limit or paused-network runtime behavior, retry logic, or degraded-state recovery behavior
- controlled cancellation execution, shutdown integration, or no-hidden-post-cancel enforcement behavior
- new engine artifact formats, hosted persistence, hidden reconstruction logic, or crash-resume semantics
- final-report generation, report availability truth, cleanup hardening, janitor behavior, updater behavior, or runtime hydration behavior

## Deliverables
- `run_session_state` exposes current nonterminal run/session identity and workspace truth during active execution instead of only after completion
- `run_status_state` can report current run/session identity and truthful monitoring availability while ordered history is still pending or partial
- `run_event_history` responds truthfully against the current in-flight snapshot and explicitly reports pending-history conditions where no ordered chronology is yet available
- shell lifecycle, interface, startup, and supervision truth surfaces acknowledge the in-flight monitoring basis without implying push streaming, degraded-state handling, or controlled cancellation behavior
- shell regression coverage proves the new publication path without contradicting existing Phase 6 one-shot execution truth

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if any frontend files are touched for bounded compatibility reasons
- confirm in-flight run/session identity becomes visible before one-shot execution returns
- confirm status, run-session, and event-history surfaces tell a mutually consistent story when chronology is pending, partial, or complete
- confirm no push-streaming claim, hidden event channel, crash-resume implication, or controlled cancellation behavior is introduced
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful in-flight snapshot publication would require a new engine event/status channel, hosted orchestration, or persistence surface not already authorized by Documents A, B, or C
- the one-shot shell execution model cannot expose current run/session identity early enough without violating the no-hidden-continuation rule
- status and event-history retrieval cannot remain mutually truthful once in-flight publication is added
- the bounded shell prerequisite would spill into Wave 6 degradation/cancellation behavior, Phase 8 final-report behavior, or Phase 9 cleanup hardening
- any applicable shell gate reports warnings or errors after in-scope remediation

## Current State
COMPLETE — 2026-04-16

## Assignment Notes
- Authored after `P07-WS07-T04` implemented the frontend monitoring UI but remained blocked on honest closeout because the shell one-shot path withholds the stored run/session snapshot until execution returns.
- This packet must be activated before any further shell or frontend implementation aimed at closing Phase 7 Wave 4 or beginning later Phase 7 degradation/cancellation work.
- The later degradation/cancellation packet is reindexed to Phase 7 Wave 6 so this prerequisite can stay narrowly focused on shell-owned in-flight snapshot truth.

## Completion Notes
- `shell/src/run_session_start.rs` now publishes truthful in-flight run/session snapshots at shell-known checkpoints for identity allocation, workspace allocation, execution-seed staging, foundation readiness, active execution handoff, and final completion without holding the stored snapshot mutex across the entire one-shot execution path.
- `shell/src/run_lifecycle_state.rs` now projects `Ingesting` and `GeneratingPersonas` truthfully from the new in-flight checkpoint fields, and `shell/src/run_event_history.rs` now distinguishes pending active history from true post-run gap conditions.
- `shell/src/run_session_contract.rs`, `shell/src/local_interface_contract.rs`, `shell/src/app_lifecycle.rs`, and `shell/src/engine_supervision.rs` now acknowledge the in-flight monitoring basis without implying push streaming, degraded-state handling, or controlled cancellation behavior.
- `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` completed cleanly for the shell crate after the implementation and critique pass.