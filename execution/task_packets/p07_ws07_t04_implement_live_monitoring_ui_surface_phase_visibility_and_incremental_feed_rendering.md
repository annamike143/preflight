# Task Packet

## Task ID
P07-WS07-T04

## Task Title
Implement Live Monitoring UI Surface, Phase Visibility, and Incremental Feed Rendering

## Parent Phase
Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

## Parent Workstream
WS-07 — Run State, Events, and Live Monitoring

## Objective
Implement the user-visible live monitoring UI surface that consumes the shell-owned lifecycle, status, and ordered event-history truth already materialized in Wave 3, including meaningful phase visibility, incremental event or transcript chronology rendering, and visible budget or duration progress, without giving the frontend lifecycle authority and without yet implementing paused or degraded runtime behavior, controlled cancellation behavior, final-report truth, or cleanup hardening.

## Why This Exists
`P07-WS07-T03` completed the shell-owned status projection and ordered event-history retrieval path, but the current frontend still renders mostly static contract or foundation panels plus finished run/session details. The repo does not yet provide the active run-monitoring surface required by `AC-070` through `AC-074`, and budget or duration visibility is still not presented as a user-followable live monitoring experience. Phase 7 Wave 4 exists to consume the already-governed shell truth in the UI before Wave 5 adds paused/degraded runtime handling, controlled cancellation behavior, and shutdown-integrity wiring.

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
- Document B Section 6.9 — Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- `execution/phase_07_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p07_ws07_t01_establish_phase_7_run_state_events_live_monitoring_and_interruption_handling_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/task_packets/p07_ws07_t02_establish_canonical_run_lifecycle_exposure_status_contract_and_event_envelope_foundation.md` is complete.
- `execution/task_packets/p07_ws07_t03_implement_shell_status_projection_ordered_event_history_and_monitoring_retrieval_path.md` is complete.
- `execution/phase_07_master_execution_plan.md` remains the authoritative Phase 7 packetization basis.
- The shell-owned `run_lifecycle_state`, `run_status_state`, `run_event_contract_state`, `run_session_state`, and `run_event_history` surfaces remain available as live repo capabilities.
- The current frontend still renders shell-owned Phase 7 truth through `app/src/App.tsx` and existing Phase 7 test coverage remains available for extension.

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/phase_07_master_execution_plan.md`
- `execution/task_packets/p07_ws07_t03_implement_shell_status_projection_ordered_event_history_and_monitoring_retrieval_path.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/startup_checks.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_status_projection.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave7.test.tsx`

## Files / Modules Expected to Change
Create or update only:
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave7.test.tsx`
- `shell/src/app_lifecycle.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/startup_checks.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/main.rs`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if a real UI-truth ambiguity becomes newly unresolved
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md`

## Implementation Scope
In scope:
- a user-visible live monitoring surface distinct from the static activation, configuration, and foundational truth sections
- frontend consumption of shell-owned current lifecycle, status, run/session identity, latest visible sequence, and ordered event-history truth through governed Tauri commands only
- visible phase or state presentation for ingestion, persona generation, active debate, paused or degraded states, and later finalizing or cleaning states where the shell truthfully exposes those states
- incremental ordered event or transcript chronology rendering based on `run_event_history` retrieval, including honest handling of incomplete history or gap markers
- visible token or duration progress, round progress, and bounded warning affordances when shell-owned status or event truth provides them
- bounded refresh or polling behavior needed for the frontend to render shell-owned live truth without becoming the authority for lifecycle state, ordering, or cancelability
- focused shell-interface or lifecycle truth-surface adjustments only where required to expose already-governed command visibility or UI materialization truth without inventing new event semantics
- frontend regression tests proving truthful phase visibility, chronology rendering, and no-static-black-box behavior when meaningful events exist

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for lifecycle truth, status projection, event ordering, latest-sequence truth, and cancelability meaning
- keep the frontend as a consumer of shell-owned truth only; it must not infer hidden states, recover missing history, or decide degraded versus healthy classification on its own
- preserve the frozen canonical run-state family and never collapse it into generic loading, done, or error labels
- preserve chronological ordering from shell-owned event retrieval and surface explicit gap or incompleteness truth instead of silently smoothing it away
- preserve transcript visibility as a user-facing chronology surface rather than turning it into unlimited prompt-memory authority
- preserve the distinction between normal progress, constrained completion, degraded states, cancellation, terminal failure, and cleanup-related semantics
- preserve the no-crash-resume, no-hidden-continuation, and no-frontend-authority rules
- preserve Phase 8 and Phase 9 separation so monitoring UI does not imply report availability, report correctness, cleanup success, or janitor completion

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- paused-rate-limit or paused-network runtime behavior, retry logic, or degraded-state recovery behavior
- controlled cancellation execution, cancel-command acceptance wiring, shutdown integration, or no-hidden-post-cancel enforcement behavior
- new shell-owned event meaning, new persistence, hosted event relays, or hidden reconstruction logic beyond the governed Wave 3 retrieval surfaces
- engine-side event production changes, engine supervision expansion, or new raw artifact formats
- final-report generation, report availability truth, export behavior, cleanup hardening, janitor behavior, updater behavior, or runtime hydration behavior
- frontend-only lifecycle authority, frontend-generated sequence numbers, or silent omission of meaningful active-run events when shell truth exists

## Deliverables
- `app/src/App.tsx` renders a distinct live monitoring surface for the active run rather than reducing the experience to static configuration plus contract panels
- the frontend consumes `run_event_history` and renders user-followable ordered chronology for meaningful lifecycle or transcript activity
- current phase or state visibility is rendered from shell-owned canonical lifecycle truth without inventing unsupported state transitions
- budget or duration visibility plus round progress are visible from shell-owned status or event truth, including honest warnings where already supported by the status surface
- the frontend keeps status, event chronology, and latest visible sequence mutually reconcilable instead of presenting contradictory stories
- frontend regression coverage proves truthful rendering of live monitoring state and chronology when shell-owned events exist
- any limited shell-surface updates remain bounded to reflecting already-governed UI-facing truth and do not create new runtime or interruption behavior

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` complete with zero warnings and zero errors
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete cleanly if any shell files change
- confirm the live monitoring surface, lifecycle/status view, and chronology feed all consume shell-owned truth and do not infer hidden state, missing history, or cancelability on their own
- confirm active debate is not reduced to a static black-box/loading-only experience when meaningful events exist
- confirm the delivered UI does not imply paused/degraded behavior, controlled cancellation execution, final-report truth, or cleanup success when those behaviors are still not implemented
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- the current Wave 3 status or event retrieval surfaces cannot support a truthful monitoring UI without inventing unsupported state, chronology, gap, or budget-warning semantics
- the frontend cannot render active monitoring truth coherently without introducing a second authority for lifecycle, ordering, or cancelability
- implementing the live monitoring surface would require new engine artifacts, hosted orchestration, or new persistence not already authorized by Documents A, B, or C
- meaningful phase visibility for the governed canonical states would require reopening Wave 3 status/event semantics rather than consuming the existing shell-owned truth
- any applicable frontend or shell gate reports warnings or errors after in-scope fixes

## Current State
COMPLETE — 2026-04-16 (frontend monitoring UI already implemented; honest closeout satisfied after T05 completed the shell-owned in-flight snapshot prerequisite)

## Activation Notes
- Activated as the current Phase 7 Wave 4 packet for frontend live-monitoring UI rendering, current phase visibility, and incremental chronology feed rendering over the completed Wave 3 shell-owned retrieval surfaces.
- Repo changes are now authorized only within the scope of this packet until closeout or blockage.
- Wave 5 degradation, retry, controlled cancellation, and shutdown behavior remain explicitly deferred while this packet focuses on shell-truth consumption and UI rendering only.

## Completion Notes
- `app/src/App.tsx` now renders the distinct Wave 4 monitoring surface, incremental event-history merge path, budget/duration progress, and honest pending-history or gap messaging, and `app/src/App.wave7.test.tsx` now proves the new monitoring surface against shell-owned fixtures.
- `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` completed cleanly for the frontend implementation.
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md` completed the shell-owned in-flight snapshot publication path, so this packet's previously recorded closeout blocker is now resolved.
- The frontend closeout remains grounded in prior clean Wave 4 gate evidence; no additional frontend file changes were required during T05 to make the already-delivered monitoring UI honest.