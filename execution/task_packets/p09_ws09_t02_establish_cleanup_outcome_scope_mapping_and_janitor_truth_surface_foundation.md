# Task Packet

## Task ID
P09-WS09-T02

## Task Title
Establish Cleanup Outcome, Scope Mapping, and Janitor Truth Surface Foundation

## Parent Phase
Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening

## Parent Workstream
WS-09 — Cleanup, Shutdown, and Janitor

## Objective
Define the code-level cleanup and janitor contract surfaces, cleanup-result and janitor-result vocabulary, cleanup-scope mapping basis, preserved-report exclusions, and shell-owned truth surfaces needed for later Phase 9 implementation, without yet claiming real cleanup execution, startup janitor purge execution, or shutdown-to-cleanup behavior.

## Why This Exists
`P09-WS09-T01` froze the bounded Phase 9 path and wave order, but the repo still lacks explicit contract modules for cleanup outcome classes, janitor state and result meaning, cleanup-scope mapping across transient versus preserved storage domains, and shell-owned exposure surfaces that keep run outcome truth separate from cleanup and janitor truth. Phase 9 Wave 3 cannot implement real terminal cleanup honestly, and Phase 9 Wave 4 cannot implement app-close cleanup coordination or startup janitor purge honestly, until Wave 2 first establishes the governing vocabulary, scope map, and shell-owned interface surfaces in code while preserving the distinction among run outcome, report availability, cleanup attempt, cleanup outcome, and janitor outcome.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
- Document A Section 6 — Product Behavior Specification
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 9 — Data Lifecycle and Storage Policy
- Document A Section 10 — API and Interface Contract
- Document A Section 12 — UI/UX Contract
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.12 — WS-09 — Cleanup, Shutdown, and Janitor
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.12 — Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening
- Document B Section 6.11 — Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- Document C Annex Family D — Report and Output Pack
- `execution/phase_09_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p09_ws09_t01_establish_phase_9_cleanup_shutdown_and_startup_janitor_hardening_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/phase_09_master_execution_plan.md` remains the authoritative Phase 9 packetization basis.
- Existing Phase 7 lifecycle, status, event, shutdown, and interruption truth surfaces remain available as the nearest repo-local pattern for cleanup/janitor contract foundation work.
- Existing Phase 8 report-output, report-storage, and report-access boundary surfaces remain available as the nearest repo-local pattern for preserved-report exclusions and cleanup-boundary alignment.
- Existing shell-owned workspace boundary and controlled-shutdown surfaces already distinguish no-resume and non-cleaned workspace truth, but they do not yet expose cleanup-result or janitor-result contract meaning.
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
- `execution/phase_09_master_execution_plan.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/report_storage_boundary.rs`
- `app/src/App.tsx`
- `app/src/appViewModels.ts`
- `app/src/hooks/useReportsScreenState.ts`
- `app/src/screens/ReportsScreen.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/run_cleanup_contract.rs`
- `shell/src/cleanup_scope_mapping.rs`
- `shell/src/startup_janitor_contract.rs`
- `app/src/App.tsx`
- `app/src/appViewModels.ts`
- `app/src/App.test.tsx`
- `app/src/App.wave8.test.tsx`
- `app/src/hooks/useReportsScreenState.ts`
- `app/src/screens/ReportsScreen.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved cleanup-scope or janitor-semantics questions change materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p09_ws09_t02_establish_cleanup_outcome_scope_mapping_and_janitor_truth_surface_foundation.md`

## Implementation Scope
In scope:
- shell-owned cleanup contract types for cleanup pending, cleanup running, cleanup succeeded, cleanup partially failed where classified, cleanup failed, and the distinction between cleanup attempt and cleanup outcome without claiming real purge execution yet
- shell-owned startup-janitor contract types for janitor pending next launch, janitor running, janitor purge succeeded, janitor purge failed, and no-resume-preserving janitor semantics without claiming real janitor execution yet
- cleanup-scope mapping surfaces aligned to transient run workspace data, bounded shell-managed cleanup metadata, preserved exported reports outside workspace cleanup scope, and durable support or secure-store domains outside cleanup and janitor scope
- lifecycle, status, event-contract, startup, shutdown, trust-boundary, local-interface, and report-boundary integration needed to expose the new Phase 9 contract and mapping truth honestly
- bounded frontend rendering of shell-owned Phase 9 contract-foundation and cleanup-scope truth without implying real cleanup or janitor execution
- tests for the new cleanup/janitor contract, cleanup-scope mapping, and shell/frontend truth surfaces

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for cleanup triggering truth, cleanup-result truth, janitor-result truth, cleanup-scope mapping, report-preservation exclusions, and filesystem purge decisions
- keep the Python engine as a producer of run-local artifacts and termination-cooperation signals rather than giving it authority over cleanup policy, janitor policy, or cleanup visibility semantics
- keep the frontend as presentation and input only, with no hidden authority over cleanup success, janitor success, scope classification, or filesystem purge decisions
- preserve the distinction between run outcome truth, report availability truth, cleanup attempt truth, cleanup outcome truth, and janitor outcome truth
- preserve the governed distinction between the canonical run lifecycle family and any more detailed cleanup or janitor contract families needed to avoid semantic collapse
- preserve intentionally exported reports outside the transient workspace boundary as cleanup-exempt and janitor-exempt artifacts
- preserve no crash-resume, no hidden continuity, best-effort cleanup limits, and the prohibition on forensic-grade deletion claims
- do not overstate contract or mapping existence as proof that real cleanup execution, app-close cleanup coordination, abnormal-leftover detection, janitor purge execution, or anti-zombie behavior already exists

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- real cleanup triggering or destructive cleanup execution
- real startup janitor scan or purge execution
- real app-close shutdown-to-cleanup coordination
- real abnormal-termination leftover marker handling or janitor retry execution
- anti-zombie execution behavior beyond bounded contract or truth-surface exposure
- forensic deletion guarantees, crash-resume support, hosted archive or recovery behavior, updater or release work, or public API expansion
- product-feature behavior beyond truthful contract or boundary-surface rendering

## Deliverables
- `shell/src/run_cleanup_contract.rs` exists and defines the bounded cleanup-attempt and cleanup-outcome contract surface
- `shell/src/cleanup_scope_mapping.rs` exists and defines the cleanup-scope mapping basis for transient workspace material, preserved exported reports, and cleanup-exempt durable domains
- `shell/src/startup_janitor_contract.rs` exists and defines the bounded janitor-state and janitor-result contract surface
- shell lifecycle, startup, shutdown, trust-boundary, local-interface, status, and event-contract surfaces expose the new Phase 9 contract truth honestly
- the frontend, if touched, reports the bounded Phase 9 cleanup/janitor contract and cleanup-scope truth without implying real purge execution or crash-resume support

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm no real cleanup execution, janitor purge execution, shutdown-to-cleanup execution, or anti-zombie execution was implemented
- confirm the delivered contracts distinguish run outcome, report availability, cleanup attempt, cleanup outcome, and janitor outcome without collapsing them into one status family
- confirm the cleanup-scope mapping leaves no ambiguity for transient run-local material, preserved exported reports, and durable support or secure-store domains
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful cleanup/janitor contract or cleanup-scope mapping surfaces cannot be defined without changing Documents A, B, or C
- the packet would require real cleanup or janitor execution to remain coherent
- lifecycle, status, event, startup, shutdown, or report-boundary integration would collapse run outcome truth into cleanup or janitor truth
- cleanup-scope mapping would require hidden persistence, hidden continuity, or reclassification of preserved reports as transient cleanup targets
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-16

## Activation Notes
- Authored and activated immediately as the current executable Phase 9 Wave 2 packet for cleanup outcome contracts, cleanup-scope mapping, janitor truth surfaces, and bounded shell/frontend integration.
- Repo changes are authorized only within this packet's allowlist and bounded contract-foundation scope until closeout or blockage.
- Real cleanup triggering, cleanup execution ordering, app-close cleanup coordination, abnormal-leftover handling, startup janitor purge execution, and anti-zombie execution remain out of scope while this packet is active.

## Completion Notes
- Added `shell/src/run_cleanup_contract.rs`, `shell/src/cleanup_scope_mapping.rs`, and `shell/src/startup_janitor_contract.rs` as the bounded Phase 9 Wave 2 shell-owned contract modules for cleanup outcome vocabulary, cleanup-scope mapping, and startup janitor truth.
- Updated `shell/src/main.rs`, `shell/src/startup_checks.rs`, `shell/src/app_lifecycle.rs`, `shell/src/app_session_trust_boundary.rs`, `shell/src/local_interface_contract.rs`, `shell/src/controlled_shutdown.rs`, `shell/src/engine_supervision.rs`, `shell/src/run_workspace_boundary.rs`, `shell/src/run_lifecycle_state.rs`, `shell/src/run_status_contract.rs`, `shell/src/run_event_contract.rs`, and `shell/src/report_storage_boundary.rs` so the new Phase 9 contract-foundation surfaces are exposed without collapsing run outcome, cleanup outcome, janitor outcome, report availability, or report-preservation truth.
- Updated `app/src/appViewModels.ts`, `app/src/hooks/useReportsScreenState.ts`, `app/src/screens/ReportsScreen.tsx`, and `app/src/App.wave8.test.tsx` so the frontend renders bounded shell-owned Phase 9 cleanup/janitor contract and cleanup-scope truth on the Reports screen without implying real purge execution.
- Validation reran `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` cleanly.
- No real cleanup triggering, destructive cleanup execution, startup janitor scan or purge execution, shutdown-to-cleanup behavior, abnormal-leftover handling, or anti-zombie execution was implemented in this packet.

## Structural Consistency and Contradiction Review
- Reviewed `execution/progress.md`, `execution/current_constraints.md`, `execution/current_repo_truth.md`, `execution/packet_registry.md`, `execution/live_execution_log.md`, and this packet file against the completed code changes and the governed Phase 9 ownership split.
- Confirmed the repo now exposes bounded cleanup outcome vocabulary, cleanup-scope mapping, preserved-report and durable-app-state exclusions, and startup janitor truth without falsely claiming real cleanup or janitor execution.
- Confirmed the changed execution surfaces tell one coherent story: `P09-WS09-T02` is complete, Phase 9 is active between packets, the new shell/frontend contract foundation is real, and no later Phase 9 packet is active yet.
- Confirmed only allowed files changed for this packet.
- No contradiction, hidden-scope expansion, unresolved blocker, or failed gate remained after review, so the packet is closed honestly.