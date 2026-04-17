# Task Packet

## Task ID
P07-WS07-T02

## Task Title
Establish Canonical Run Lifecycle Exposure, Status Contract, and Event Envelope Foundation

## Parent Phase
Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

## Parent Workstream
WS-07 — Run State, Events, and Live Monitoring

## Objective
Define the code-level canonical run lifecycle vocabulary, shell-owned live-status contract surfaces, cancellation request and response basis, and ordered event-envelope and retrieval contract surfaces needed for later Phase 7 implementation, without yet claiming real ordered event history materialization, incremental live UI synchronization, paused/degraded runtime handling, or controlled cancellation behavior.

## Why This Exists
`P07-WS07-T01` froze the bounded Phase 7 wave order and the canonical state/event basis, but the repo still exposes only coarse Phase 6 run-session classes and lacks explicit canonical run lifecycle modules, live-status retrieval contracts, cancellation request/response contracts, and ordered event-envelope or retrieval semantics. Phase 7 Waves 3 through 5 cannot implement truthful status projection, event history retrieval, live monitoring UI, or interruption handling honestly until Wave 2 first establishes the canonical vocabulary, interface shapes, sequence semantics, and shell-owned monitoring authority in code.

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
- `execution/phase_07_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p07_ws07_t01_establish_phase_7_run_state_events_live_monitoring_and_interruption_handling_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/phase_07_master_execution_plan.md` remains the authoritative Phase 7 packetization basis.
- Existing Phase 6 shell-owned run/session, workspace, lifecycle, supervision, moderation, and token/duration tracking surfaces remain available as the nearest repo-local pattern for canonical state exposure and monitoring-contract foundation work.
- Rust/Cargo/Clippy and frontend validation tooling remain available locally.

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
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_start_readiness.rs`
- `app/src/App.tsx`

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
- `shell/src/run_start_readiness.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_event_contract.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave7.test.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved canonical state or event-semantics questions change materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p07_ws07_t02_establish_canonical_run_lifecycle_exposure_status_contract_and_event_envelope_foundation.md`

## Implementation Scope
In scope:
- canonical run lifecycle state family for shell-owned exposure, including the upstream Phase 7 state vocabulary and a truthful mapping basis from the current coarse Phase 6 run-session classes
- shell-owned live-status contract types for current state, status retrieval shape, state label exposure, status-note exposure, identity/reference fields, and latest visible sequence marker semantics
- shell-owned cancellation request/response contract basis with cancelability derived from governed states rather than frontend heuristics
- ordered event retrieval contract surfaces covering request shape, ordered retrieval semantics, sequence boundaries, gap detection meaning, and canonical event-envelope structure
- shell lifecycle, startup, trust-boundary, local-interface, engine-supervision, command, and frontend integration needed to expose the new Phase 7 contract and vocabulary truth honestly
- tests for the new lifecycle, status-contract, event-envelope, and integration surfaces

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for lifecycle truth, status retrieval, cancelability, and event/status mediation
- keep the frontend as a consumer of shell-owned monitoring truth with no hidden authority over lifecycle states, event ordering, or cancelability
- keep the Python engine as the producer of underlying execution activity rather than giving it authority over external status/event contract meaning
- preserve the frozen canonical state family and avoid exposing the old coarse Phase 6 run-state classes as a competing user-facing lifecycle vocabulary
- preserve the distinction between degraded/interrupted, cancellation, constrained completion, failure, finalization, and cleanup-related states
- preserve the no-crash-resume and no-hidden-continuation rules
- do not overstate contract or vocabulary existence as proof that real ordered event history, live UI sync, interruption handling, or controlled cancellation behavior already exists

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- real ordered event history capture, append-only event persistence, or incremental event retrieval over live run artifacts
- live monitoring UI feed rendering beyond truthful contract or placeholder or boundary-surface exposure
- real paused/degraded handling, bounded retry behavior, or interruption recovery behavior
- real controlled cancellation handling, shutdown integration, or no-hidden-post-cancel enforcement behavior
- final report generation, report availability behavior, cleanup hardening, janitor behavior, updater behavior, or runtime hydration behavior
- hosted orchestration behavior, public API behavior, or commercial-backend expansion
- product-feature behavior beyond truthful contract and boundary-surface rendering

## Deliverables
- `shell/src/run_lifecycle_state.rs` exists and defines the bounded canonical Phase 7 run lifecycle vocabulary and mapping basis
- `shell/src/run_status_contract.rs` exists and defines the shell-owned live-status retrieval contract surface
- `shell/src/run_event_contract.rs` exists and defines the ordered retrieval and event-envelope contract basis
- `shell/src/run_session_contract.rs` includes cancellation request/response contract semantics consistent with the frozen Phase 7 basis
- shell lifecycle, startup, trust-boundary, supervision, local-interface, and command surfaces expose the new Phase 7 truth honestly
- the frontend, if touched, reports the bounded lifecycle/status/event contract truth without implying that live monitoring or interruption handling is fully implemented

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm the canonical state family aligns to the frozen upstream Phase 7 vocabulary and does not invent unsupported states or transitions
- confirm status and event contracts define interface shape only and do not imply real event history materialization, live UI synchronization, interruption handling, controlled cancellation, final-report truth, or cleanup hardening
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful canonical state exposure cannot be defined without inventing additional states or transitions beyond Documents A, B, or C
- truthful status or event contract surfaces would require hidden persistence, hosted orchestration, or real event-history implementation to remain coherent
- cancellation request/response semantics cannot remain shell-owned without implying unsupported cancellation behavior or crash-resume semantics
- lifecycle, startup, supervision, or trust-boundary integration would collapse status/event truth into frontend inference or engine-owned interface authority
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Current State
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 7 Wave 2 packet for canonical run lifecycle exposure, live-status contract surfaces, cancellation request/response basis, and ordered event-envelope foundation.
- Repo changes are authorized only within the scope of this packet until closeout or blockage.
- Real ordered event history materialization, live monitoring UI synchronization, paused/degraded handling, controlled cancellation behavior, report truth, and cleanup hardening remain explicitly out of scope during this packet.

## Completion Notes
- Added `shell/src/run_lifecycle_state.rs`, `shell/src/run_status_contract.rs`, and `shell/src/run_event_contract.rs` to define the frozen canonical Phase 7 lifecycle vocabulary, shell-owned current-status retrieval shape, cancellation request/response basis, and ordered event-envelope or retrieval contract semantics without claiming live event capture or real interruption handling.
- Updated `shell/src/run_session_contract.rs`, `shell/src/app_lifecycle.rs`, `shell/src/app_session_trust_boundary.rs`, `shell/src/startup_checks.rs`, `shell/src/local_interface_contract.rs`, `shell/src/engine_supervision.rs`, and `shell/src/main.rs` so the shell now exposes the new Phase 7 lifecycle/status/event contract truth through the allowlisted boundary surfaces while preserving shell authority and keeping real event history and cancellation execution out of scope.
- Updated `app/src/App.tsx`, `app/src/App.test.tsx`, `app/src/App.wave5.test.tsx`, and added `app/src/App.wave7.test.tsx` so the frontend renders the bounded Phase 7 lifecycle/status/event contract truth as shell-owned foundation surfaces without inferring hidden states, event ordering, or cancelability.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`.
- Updated `execution/progress.md`, `execution/current_constraints.md`, `execution/current_repo_truth.md`, `execution/packet_registry.md`, and `execution/live_execution_log.md` so the execution OS now records `P07-WS07-T02` as complete and returns Phase 7 to a truthful between-packets posture.
- No decision-log update was required because the canonical lifecycle, status, cancellation-contract, and event-envelope basis were all derivable from the existing Documents A, B, and C plus the frozen Phase 7 master plan.

## Structural Consistency and Contradiction Review
- Reviewed the new shell lifecycle, status-contract, and event-contract modules plus the touched lifecycle, trust-boundary, supervision, startup, interface, command, and frontend files against the frozen Phase 7 master plan and the active packet scope.
- Confirmed the canonical exposed state family remains exactly `RunDraft`, `RunQueued`, `Ingesting`, `GeneratingPersonas`, `RunningRound`, `PausedRateLimit`, `PausedNetwork`, `Finalizing`, `Completed`, `CompletedBudgetCapped`, `Canceled`, `FailedRetryable`, `FailedTerminal`, `Cleaning`, `Cleaned`, and `CleanupFailed` without inventing extra states or collapsing degraded/canceled/failure/cleanup meanings.
- Confirmed the new status and event surfaces define interface shape only: no ordered event history materialization, no live event feed synchronization, no paused/degraded runtime behavior, no controlled cancellation execution, no report truth, and no cleanup hardening are implied by the delivered code.
- Confirmed the shell remains the sole local authority for lifecycle truth, status retrieval, cancelability, and event-order mediation while the frontend remains a bounded consumer of shell-owned truth only.
- No contradiction, hidden-scope expansion, unresolved blocker, or failed gate remained after review, so the packet is closed honestly.