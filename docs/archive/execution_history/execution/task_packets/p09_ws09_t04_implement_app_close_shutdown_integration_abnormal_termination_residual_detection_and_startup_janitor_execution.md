# Task Packet

## Task ID
P09-WS09-T04

## Task Title
Implement App-Close Shutdown Integration, Abnormal-Termination Residual Detection, and Startup Janitor Execution

## Parent Phase
Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening

## Parent Workstream
WS-09 — Cleanup, Shutdown, and Janitor

## Objective
Implement the bounded Phase 9 Wave 4 path for app-close cleanup coordination on active runs, abnormal-termination residual detection, startup janitor scan and purge execution, and anti-zombie supervision alignment where required, without collapsing run outcome truth into cleanup or janitor truth and without introducing crash-resume or hidden continuity behavior.

## Why This Exists
`P09-WS09-T03` completed the normal terminal cleanup path for governed terminal run outcomes, but the repo still lacks truthful app-close cleanup coordination for active runs, bounded abnormal-termination residual detection, real startup janitor execution before readiness checks continue, and the explicit no-resume-preserving janitor behavior that Phase 9 still owes. The frozen Phase 9 plan requires this Wave 4 implementation before the exit-gate packet can assess cleanup and janitor truth honestly. This packet activates that bounded Wave 4 scope while keeping Phase 9 validation and closure work for the later exit-gate packet.

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
- `execution/task_packets/p09_ws09_t03_implement_terminal_cleanup_triggering_outcome_classification_and_report_preservation_enforcement.md` is complete.
- `execution/phase_09_master_execution_plan.md` remains the authoritative Phase 9 packetization basis.
- Existing controlled cancellation and active-run shutdown coordination remain real repo capabilities from completed Phase 7 work.
- Existing terminal cleanup execution, cleanup-result truth, cleanup-scope mapping, preserved-report exclusions, and startup janitor contract surfaces remain available from completed Phase 9 Wave 2 and Wave 3 work.
- The repo still does not implement app-close cleanup coordination through to truthful cleanup or janitor handoff, abnormal-termination residual detection, real startup janitor purge execution, or explicit no-resume-preserving janitor behavior, and those are the exact scope activated by this packet.
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
- `execution/task_packets/p09_ws09_t03_implement_terminal_cleanup_triggering_outcome_classification_and_report_preservation_enforcement.md`
- `execution/task_packets/p07_ws07_t06_implement_degradation_retry_controlled_cancellation_and_shutdown_integration.md`
- `shell/src/main.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/run_cleanup_contract.rs`
- `shell/src/run_cleanup_execution.rs`
- `shell/src/cleanup_scope_mapping.rs`
- `shell/src/startup_janitor_contract.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/startup_checks.rs`
- `app/src/App.tsx`
- `app/src/appHelpers.ts`
- `app/src/appViewModels.ts`
- `app/src/App.wave7.test.tsx`
- `app/src/App.wave8.test.tsx`
- `app/src/hooks/useReportsScreenState.ts`
- `app/src/hooks/useRunScreenState.ts`
- `app/src/screens/ReportsScreen.tsx`
- `app/src/screens/shared.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/run_cleanup_contract.rs`
- `shell/src/run_cleanup_execution.rs`
- `shell/src/cleanup_scope_mapping.rs`
- `shell/src/startup_janitor_contract.rs`
- `shell/src/startup_janitor_execution.rs`
- `shell/src/abnormal_termination_residual.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/startup_checks.rs`
- `app/src/App.tsx`
- `app/src/appHelpers.ts`
- `app/src/appViewModels.ts`
- `app/src/App.wave7.test.tsx`
- `app/src/App.wave8.test.tsx`
- `app/src/hooks/useReportsScreenState.ts`
- `app/src/hooks/useRunScreenState.ts`
- `app/src/screens/ReportsScreen.tsx`
- `app/src/screens/shared.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved shutdown or janitor questions change materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p09_ws09_t04_implement_app_close_shutdown_integration_abnormal_termination_residual_detection_and_startup_janitor_execution.md`

## Implementation Scope
In scope:
- shell-owned app-close integration from `AppShuttingDown` into truthful active-run termination handling plus immediate cleanup or next-launch janitor handoff behavior
- abnormal-termination residual detection and bounded marker or manifest handling for later startup-janitor use under shell authority only
- startup janitor scan and purge attempts before readiness checks continue, including safe timeout, partial-failure, and failure visibility where the repo can classify those outcomes truthfully
- anti-zombie supervision, process-lock, and cleanup-ordering alignment needed so app-close and next-launch janitor behavior remain truthful under the existing bounded execution model
- lifecycle, status, event-history, startup-summary, trust-surface, and report-boundary integration needed to expose janitor and abnormal-termination truth without collapsing run outcome or cleanup outcome truth
- preserved exported report enforcement across app-close cleanup handoff and startup janitor behavior so durable report artifacts outside cleanup scope survive both paths
- bounded frontend rendering of shell-owned janitor and abnormal-termination truth where already-governed surfaces make that truth user-visible
- tests covering active-run app close, abnormal residual detection, next-launch janitor purge attempts, preserved-report survival, and explicit no-resume preservation where the repo can prove those outcomes deterministically

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for app-close cleanup coordination, residual detection, janitor triggering, janitor outcome truth, cleanup ordering, filesystem purge decisions, and no-resume preservation
- keep the Python engine as a producer of run-local artifacts and termination-cooperation signals only, without giving it janitor-policy, residual-policy, or externally exposed cleanup or janitor authority
- keep the frontend as presentation-only, with no hidden authority over janitor success, residual classification, report-preservation decisions, or filesystem inspection
- preserve the distinction among run outcome truth, cleanup outcome truth, janitor outcome truth, report-generation truth, and report availability or access truth
- preserve intentionally exported reports outside the transient workspace boundary as cleanup-exempt and janitor-exempt artifacts
- preserve durable app support data and OS-native secure-store domains outside cleanup and janitor scope
- preserve no crash-resume, no hidden continuity, and best-effort cleanup or janitor limits without overstating forensic deletion guarantees
- preserve the completed Wave 3 terminal cleanup path without rewriting its run-outcome distinction or reclassifying cleanup success as app-close success

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- Phase 9 exit-gate acceptance mapping, closure decisioning, or packet-closeout work beyond what this packet needs for truthful implementation
- crash-resume support, session restoration, hidden abandoned-workspace continuity, or janitor-as-recovery behavior
- forensic deletion guarantees, hosted archive or recovery behavior, updater or release work, or public API expansion
- report catalog persistence, hosted report library behavior, or any reclassification of preserved exported reports as transient cleanup or janitor targets
- frontend-owned janitor authority, direct filesystem scanning, or any engine-owned cleanup or janitor policy behavior
- unrelated provider, activation, Preflight, Phase 8 report-generation, or Phase 10 update-distribution feature work

## Deliverables
- `shell/src/startup_janitor_execution.rs` exists and implements the bounded next-launch janitor path under shell authority
- abnormal-termination residual detection exists under shell authority without becoming a hidden continuity or crash-resume store
- active app close now coordinates truthful run termination with cleanup execution or janitor handoff behavior rather than leaving silent leftovers
- startup checks and adjacent shell lifecycle surfaces expose real janitor execution truth consistently before readiness continues
- preserved exported reports outside cleanup scope survive both app-close cleanup handoff and startup janitor behavior
- the frontend, if touched, renders shell-owned janitor and abnormal-termination truth without gaining lifecycle or filesystem authority

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm active app close coordinates truthful termination handling plus cleanup execution or janitor handoff rather than leaving silent leftovers
- confirm the next launch attempts stale-workspace purge after abnormal termination where residual markers are present
- confirm janitor behavior does not imply run restoration, hidden continuity, or crash resume
- confirm preserved exported reports outside the transient workspace are not deleted as part of app-close cleanup or startup janitor purge behavior
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful abnormal-termination residual detection would require hidden persistence, crash-resume behavior, or undeclared hosted coordination not already authorized by Documents A, B, or C
- app-close cleanup coordination cannot remain shell-owned or cannot stay distinct from underlying run outcome truth
- startup janitor purge behavior would require reclassifying durable report artifacts, support-state data, or secure-store domains as janitor targets
- anti-zombie supervision or janitor execution would require reopening unrelated Phase 7 monitoring scope or introducing hidden long-lived daemon behavior
- any applicable shell or frontend gate reports warnings or errors after in-scope remediation

## Completion Decision
COMPLETE — 2026-04-16

## Activation Notes
- Authored and activated immediately as the current executable Phase 9 Wave 4 packet for app-close cleanup coordination, abnormal-termination residual detection, startup janitor purge execution, anti-zombie supervision alignment where required, and explicit no-resume-preserving janitor behavior.
- Repo changes were authorized only within this packet's allowlist and bounded Wave 4 scope until closeout.
- The later Phase 9 exit-gate packet remained inactive while this packet was implementing and validating Wave 4 behavior.

## Completion Notes
- Completed after materializing `shell/src/abnormal_termination_residual.rs` and `shell/src/startup_janitor_execution.rs`, wiring shell-owned controlled-shutdown handoff to residual markers and next-launch janitor behavior through `shell/src/run_session_start.rs` and `shell/src/main.rs`, updating the adjacent startup, lifecycle, trust, and Reports-screen truth surfaces, and preserving the ban on crash-resume or broad OS-level close interception.
- Added tighter Wave 4 shell coverage for shutdown-to-janitor handoff and startup-janitor timeout or partial-failure behavior in `shell/src/run_session_start.rs` and `shell/src/startup_janitor_execution.rs`.
- Validation reran `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` cleanly.
- The completed Wave 4 implementation remains bounded to shell-owned lifecycle control, explicit no-resume preservation, and preserved-report survival outside cleanup or janitor scope; the Phase 9 exit gate remains deferred to the later packet.

## Structural Consistency and Contradiction Review
- Reviewed `execution/progress.md`, `execution/current_constraints.md`, `execution/current_repo_truth.md`, `execution/packet_registry.md`, `execution/live_execution_log.md`, and this packet file against the completed Wave 4 code changes and the frozen Phase 9 ownership split.
- Confirmed the changed execution surfaces tell one coherent story: `P09-WS09-T04` is complete, Phase 9 is active between packets, and `P09-WS11-T01` remains the prepared but inactive successor.
- Confirmed the closeout preserves shell-only authority over cleanup and janitor truth, report-preservation exclusions, the ban on crash-resume or hidden continuity, and the restriction against broad automatic app-close interception beyond the bounded controlled-shutdown request surface.
- Confirmed only allowed files changed for this packet.
- No contradiction, hidden-scope expansion, unresolved blocker, or failed gate remained after review, so the packet is closed honestly.