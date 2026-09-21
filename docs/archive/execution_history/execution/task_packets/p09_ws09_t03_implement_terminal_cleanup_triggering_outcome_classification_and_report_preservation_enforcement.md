# Task Packet

## Task ID
P09-WS09-T03

## Task Title
Implement Terminal Cleanup Triggering, Outcome Classification, and Report Preservation Enforcement

## Parent Phase
Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening

## Parent Workstream
WS-09 — Cleanup, Shutdown, and Janitor

## Objective
Implement the real shell-owned terminal cleanup path for governed terminal run outcomes, including cleanup triggering, cleanup execution ordering, cleanup-result classification, and preserved-report enforcement, without yet claiming app-close cleanup coordination, abnormal-termination residual detection, startup janitor execution, or anti-zombie hardening completeness.

## Why This Exists
`P09-WS09-T02` froze the cleanup and janitor contract vocabulary, cleanup-scope mapping, preserved-report exclusions, and shell-owned truth surfaces needed for Phase 9 implementation, but the repo still does not perform real cleanup after terminal run paths. Phase 9 cannot close honestly until the shell actually attempts best-effort cleanup after completed, constrained-completion, canceled, and terminal-failure runs, classifies the cleanup result without collapsing it into run outcome truth, and preserves exported reports outside the transient workspace boundary. This packet activates that Wave 3 scope while keeping app-close shutdown handoff, abnormal-leftover handling, startup janitor execution, and anti-zombie completion deferred to Wave 4.

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
- `execution/task_packets/p09_ws09_t02_establish_cleanup_outcome_scope_mapping_and_janitor_truth_surface_foundation.md` is complete.
- `execution/phase_09_master_execution_plan.md` remains the authoritative Phase 9 packetization basis.
- Existing terminal run outcomes, finalization/report generation, report availability/access, and durable report-boundary handling remain real repo capabilities from completed Phases 7 and 8.
- Existing cleanup/janitor contract surfaces, cleanup-scope mapping, preserved-report exclusions, and cleanup/janitor truth rendering remain available from `P09-WS09-T02`.
- The repo still does not implement app-close cleanup coordination, abnormal-leftover detection, startup janitor purge execution, or anti-zombie hardening, and those remain out of scope for this packet.
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
- `execution/task_packets/p09_ws09_t02_establish_cleanup_outcome_scope_mapping_and_janitor_truth_surface_foundation.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/run_cleanup_contract.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/startup_checks.rs`
- `app/src/App.tsx`
- `app/src/appViewModels.ts`
- `app/src/App.wave7.test.tsx`
- `app/src/App.wave8.test.tsx`
- `app/src/hooks/useReportsScreenState.ts`
- `app/src/screens/ReportsScreen.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/run_cleanup_contract.rs`
- `shell/src/run_cleanup_execution.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/startup_checks.rs`
- `app/src/App.tsx`
- `app/src/appViewModels.ts`
- `app/src/App.wave7.test.tsx`
- `app/src/App.wave8.test.tsx`
- `app/src/hooks/useReportsScreenState.ts`
- `app/src/screens/ReportsScreen.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved cleanup-path or preserved-report questions change materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p09_ws09_t03_implement_terminal_cleanup_triggering_outcome_classification_and_report_preservation_enforcement.md`

## Implementation Scope
In scope:
- real shell-owned cleanup triggering after governed terminal run paths for `Completed`, `CompletedBudgetCapped`, `Canceled`, and `FailedTerminal`
- cleanup execution ordering aligned to shell-known process termination, file-lock, and report-preservation truth
- best-effort purge of transient run workspace material and cleanup-targeted transient coordination metadata using the completed cleanup-scope mapping basis
- shell-owned cleanup outcome classification, including truthful distinction among cleanup attempt started, cleanup succeeded, cleanup partially failed where classified, and cleanup failed
- lifecycle, status, event-history, startup-summary, report-boundary, and trust-surface integration needed to expose real cleanup truth without collapsing run outcome or report-availability truth
- preserved exported report enforcement so durable report artifacts outside the transient workspace survive cleanup attempts
- bounded frontend rendering of shell-owned cleanup truth where already-governed surfaces make that truth user-visible
- tests covering successful cleanup, preserved-report survival, and visible cleanup failure classification where the repo can prove it deterministically

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for cleanup triggering, cleanup ordering, cleanup-result truth, cleanup-scope interpretation, report-preservation enforcement, and filesystem purge decisions
- keep the Python engine as a producer of run-local artifacts and termination-cooperation signals only, without giving it cleanup-policy or cleanup-result authority
- keep the frontend as presentation-only, with no hidden authority over cleanup success, report-preservation decisions, or filesystem inspection
- preserve the distinction among run outcome truth, report-generation truth, report availability/access truth, cleanup attempt truth, and cleanup outcome truth
- preserve intentionally exported reports outside the transient workspace boundary as cleanup-exempt and janitor-exempt artifacts
- preserve durable app support data and OS-native secure-store domains outside cleanup scope
- preserve no crash-resume, no hidden continuity, and best-effort cleanup limits without overstating forensic deletion guarantees
- preserve the startup janitor contract foundation from Wave 2 without claiming startup janitor execution, abnormal-leftover detection, or app-close cleanup coordination already exists

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- app-close cleanup coordination for active runs
- abnormal-termination leftover marker creation, detection, or janitor retry execution
- real startup janitor scan or purge execution
- new anti-zombie supervision behavior beyond whatever later Wave 4 needs to packetize explicitly
- forensic deletion guarantees, crash-resume support, hosted archive or recovery behavior, updater or release work, or public API expansion
- report catalog persistence, hosted report library behavior, or any reclassification of preserved exported reports as transient cleanup targets
- frontend-owned cleanup authority, direct filesystem scanning, or any engine-owned cleanup-policy behavior

## Deliverables
- `shell/src/run_cleanup_execution.rs` exists and implements the bounded terminal cleanup path and cleanup-result classification basis
- governed terminal run paths now attempt real cleanup under shell authority without deleting preserved report artifacts outside cleanup scope
- shell lifecycle, status, event, startup, report-boundary, and trust surfaces expose real cleanup truth consistently
- the frontend, if touched, renders shell-owned cleanup truth without gaining lifecycle or filesystem authority

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm cleanup is attempted after governed terminal run paths and that preserved exported reports outside the workspace survive cleanup
- confirm cleanup success is not treated as run success and cleanup failure does not rewrite an otherwise distinct run outcome
- confirm no app-close cleanup coordination, abnormal-leftover detection, startup janitor purge execution, or anti-zombie hardening was implemented under this packet
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- terminal cleanup triggering cannot be implemented truthfully without pulling in app-close cleanup coordination, abnormal-leftover handling, startup janitor execution, or anti-zombie scope from Wave 4
- preserved-report enforcement would require reclassifying durable exported reports, app support data, or secure-store domains as cleanup targets
- lifecycle, status, event, or report-boundary integration would collapse run outcome truth into cleanup outcome truth
- real cleanup execution would require new hidden persistence, hidden continuity, or crash-resume behavior
- any applicable shell or frontend gate reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-16

## Activation Notes
- Authored and activated immediately as the current executable Phase 9 Wave 3 packet for real terminal cleanup triggering, cleanup execution ordering, cleanup-result classification, and preserved-report enforcement across governed terminal run paths.
- Repo changes are authorized only within this packet's allowlist and bounded Wave 3 scope until closeout or blockage.
- App-close cleanup coordination, abnormal-leftover detection, startup janitor purge execution, and anti-zombie hardening remained out of scope while this packet was active.

## Completion Notes
- Completed after materializing `shell/src/run_cleanup_execution.rs`, wiring real terminal cleanup into governed `Completed`, `CompletedBudgetCapped`, `Canceled`, and `FailedTerminal` paths, projecting cleanup markers through lifecycle/status/event truth, preserving durable report artifacts outside cleanup scope, and updating bounded frontend cleanup rendering.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm test`, and `npm run build`.
- A structural consistency review and contradiction check across the touched execution files completed cleanly before closeout.
- App-close cleanup coordination, abnormal-leftover detection, startup janitor purge execution, and anti-zombie hardening remain deferred to later Phase 9 scope.