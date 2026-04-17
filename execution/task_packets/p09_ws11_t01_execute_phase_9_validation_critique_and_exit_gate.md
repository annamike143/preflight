# Task Packet

## Task ID
P09-WS11-T01

## Task Title
Execute Phase 9 Validation, Critique, and Exit Gate

## Parent Phase
Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening

## Parent Workstream
WS-11 — QA, Acceptance, Red-Team, and Regression

## Objective
Execute the Phase 9 exit-gate validation pass, acceptance mapping, critique pass, and closure decision so the repo can either close Phase 9 honestly or remain open with an explicit blocking reason if any required cleanup, shutdown, janitor, report-preservation, or no-resume evidence is still missing.

## Why This Exists
`P09-W05` is the final Phase 9 packet defined by `execution/phase_09_master_execution_plan.md`. Waves 2 through 4 established the cleanup and janitor contract foundation, real terminal cleanup triggering, cleanup-result truth, app-close cleanup coordination through the bounded controlled-shutdown handoff, abnormal residual detection, startup janitor execution, preserved-report survival across cleanup and janitor behavior, and explicit no-resume preservation. This packet now proves whether that implementation actually satisfies the governed Phase 9 acceptance targets and whether Phase 9 may close without inventing cleanup success, janitor completeness, or crash-resume behavior.

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
- Document B Section 3.14 — WS-11 — QA, Acceptance, Red-Team, and Regression
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.12 — Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening
- Document B Section 6.11.9 — Validation Gate
- Document B Section 6.11.10 — Failure / Reopen Conditions
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- Document C Annex Family D — Report and Output Pack
- `execution/phase_09_master_execution_plan.md`
- `execution/task_packets/p09_ws09_t04_implement_app_close_shutdown_integration_abnormal_termination_residual_detection_and_startup_janitor_execution.md`

## Dependency Preconditions
- `execution/task_packets/p09_ws09_t04_implement_app_close_shutdown_integration_abnormal_termination_residual_detection_and_startup_janitor_execution.md` is complete.
- No other packet may remain active while this packet is active.
- The frozen Phase 9 basis remains unchanged: shell-owned cleanup and janitor authority, explicit distinction between run outcome and cleanup or janitor outcome, preserved durable reports outside transient workspace scope, no crash-resume behavior, and no broad automatic app-close interception beyond the bounded controlled-shutdown request surface.

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/phase_09_master_execution_plan.md`
- `execution/task_packets/p09_ws09_t03_implement_terminal_cleanup_triggering_outcome_classification_and_report_preservation_enforcement.md`
- `execution/task_packets/p09_ws09_t04_implement_app_close_shutdown_integration_abnormal_termination_residual_detection_and_startup_janitor_execution.md`
- `shell/src/main.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_cleanup_contract.rs`
- `shell/src/run_cleanup_execution.rs`
- `shell/src/cleanup_scope_mapping.rs`
- `shell/src/startup_janitor_contract.rs`
- `shell/src/startup_janitor_execution.rs`
- `shell/src/abnormal_termination_residual.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/startup_checks.rs`
- `app/src/appViewModels.ts`
- `app/src/screens/ReportsScreen.tsx`
- `app/src/App.wave8.test.tsx`

## Files / Modules Expected to Change
Create or update only:
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if unresolved Phase 9 questions change materially after the exit-gate pass
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if the Phase 9 closure decision requires a real local governance decision
- `execution/phase_09_master_execution_plan.md` only if the critique pass requires truthful exit-gate or phase-closure wording repair
- `execution/task_packets/p09_ws09_t03_implement_terminal_cleanup_triggering_outcome_classification_and_report_preservation_enforcement.md` only if critique evidence requires truthful completion-note correction
- `execution/task_packets/p09_ws09_t04_implement_app_close_shutdown_integration_abnormal_termination_residual_detection_and_startup_janitor_execution.md` only if critique evidence requires truthful completion-note correction
- `execution/task_packets/p09_ws11_t01_execute_phase_9_validation_critique_and_exit_gate.md`
- `shell/src/` files only if the critique pass finds a material in-scope Phase 9 defect in the implemented cleanup, shutdown, janitor, or report-preservation surfaces
- `app/src/` files only if the critique pass finds a truthful UI contradiction in the existing Phase 9 reporting surfaces

## Implementation Scope
In scope:
- acceptance mapping for `AC-092` through `AC-098`
- rerunning and reviewing the applicable shell and frontend validation gates for the implemented Phase 9 surfaces
- explicit evidence review for cleanup triggering after terminal paths, cleanup failure visibility, startup janitor attempts after abnormal termination, no-resume preservation, preserved-report survival outside purge scope, and no cross-run working-data reuse
- critique pass over the touched Phase 9 shell, frontend, and execution-state files
- tightly bounded in-scope remediation if the critique pass finds a real Phase 9 defect in the implemented surfaces
- explicit Phase 9 closure decision and execution-state updates

## Required Behavior to Preserve
- the packet must not invent acceptance proof from contract text alone, stale pre-Wave 4 assumptions, or historical packet intent that the live repo does not satisfy
- the shell must remain the sole authority for cleanup triggering, cleanup-result truth, shutdown coordination, startup janitor execution, filesystem purge decisions, no-resume enforcement, and preserved-report exemptions
- the frontend must remain presentation-only and must not infer hidden cleanup success, janitor success, report survival, or abandoned-workspace reuse semantics on its own
- run outcome, cleanup outcome, janitor outcome, report availability, and report preservation must remain distinct
- the repo must continue to avoid crash-resume, hidden continuity, hosted archive behavior, and forensic deletion guarantees
- if the evidence does not support Phase 9 closure, the packet must leave Phase 9 open honestly rather than forcing closure

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- new product scope beyond exit-gate validation and tightly bounded in-scope remediation
- Phase 10 updater or runtime-delivery hardening, Phase 11 release readiness, hosted archive or recovery behavior, or public API expansion
- crash-resume support, hidden continuity, janitor-as-recovery semantics, or stronger deletion/privacy claims than upstream documents authorize
- report catalog persistence, hosted report library behavior, alternate export formats, or reclassification of preserved reports as transient cleanup targets

## Deliverables
- acceptance mapping exists for the governed Phase 9 criteria
- all applicable Phase 9 shell and frontend gates are rerun truthfully
- critique findings are either fixed in-scope or escalated explicitly
- the repo records a truthful Phase 9 closure decision

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- rerun and report the exact results of:
  - `cargo check`
  - `cargo clippy -- -D warnings`
  - `cargo test`
  - `npm run lint`
  - `npm run typecheck`
  - `npm run test`
  - `npm run build`
- confirm cleanup is attempted after governed terminal paths
- confirm cleanup failure is visible rather than silently implied away
- confirm the next launch attempts startup janitor handling after abnormal termination residue is present
- confirm the repo still does not offer crash resume
- confirm preserved exported reports outside the transient workspace are not deleted as part of cleanup or startup janitor behavior
- confirm run-local working data is not reused as active semantic input for later runs
- confirm whether Phase 9 can close truthfully and record the result explicitly

## Escalation Triggers
Escalate immediately if:
- any governed Phase 9 acceptance target still lacks real evidence after the exit-gate pass
- any applicable gate reports an error or warning after in-scope remediation attempts
- the critique pass reveals contradictory cleanup, shutdown, janitor, report-preservation, or no-resume truth across the implemented surfaces
- closing Phase 9 would require treating crash-resume, hidden continuity, preserved-report deletion, or silent cleanup success as if it were already implemented correctly

## Completion Decision
COMPLETE — 2026-04-17

## Activation Notes
- Activated as the final Phase 9 Wave 5 packet for validation, critique, acceptance mapping, and the closure decision after the completed Wave 4 shutdown-handoff and startup-janitor work.
- Phase 9 closure was not authorized at activation time and was held behind fresh shell plus frontend gate reruns and live evidence for `AC-092` through `AC-098`.

## Completion Notes
- Re-ran the full applicable Phase 9 shell and frontend gate set cleanly during this exit-gate pass: `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` all passed in `shell/` with 120 tests green; `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` all passed in `app/` with 5 Vitest files and 13 tests green.
- Confirmed the live Phase 9 implementation now covers the full bounded cleanup and janitor path: governed terminal runs enter shell-owned cleanup attempt states, cleanup results remain distinct from run outcomes, cleanup failure remains visible, abnormal shutdown residue is recorded for next-launch janitor handling, startup janitor runs before readiness continues, and preserved exported reports remain outside cleanup and janitor scope.
- Confirmed the repo still forbids crash resume and abandoned-workspace reuse as active execution state. The run workspace boundary, bounded-memory rules, residual-marker path, and janitor outcome messaging all continue to preserve no-resume semantics.
- No in-scope product-code remediation was required during this exit-gate pass because the fresh validation reruns and critique pass did not expose a new shell, frontend, cleanup, janitor, or report-preservation defect.
- Phase 9 now closes honestly because `AC-092` through `AC-098` are fully proven against the live cleanup, shutdown, janitor, preserved-report, and no-resume implementation without overclaiming forensic deletion, hidden continuity, hosted archive behavior, or release readiness.

## Acceptance Mapping
- `AC-092` — PROVEN by `maybe_run_terminal_cleanup` and its terminal-path call sites in `shell/src/run_session_start.rs`, which route governed completed, constrained-completion, canceled, shutdown-canceled, and failed-terminal outcomes into shell-owned cleanup attempts, together with `start_run_session_with_components_creates_fresh_run_ids_and_workspaces`, `start_run_session_with_components_preserves_completed_budget_capped_truth`, `start_run_session_with_components_distinguishes_report_generation_failure_from_completion`, and the canceled-path assertions in the same shell test module that all end with emitted cleanup markers.
- `AC-093` — PROVEN by `shell/src/run_cleanup_contract.rs`, `shell/src/run_lifecycle_state.rs`, `shell/src/run_status_projection.rs`, and `shell/src/run_session_start.rs`, which keep cleanup attempt and cleanup outcome distinct from run outcome truth and project `Cleaning`, `Cleaned`, and `CleanupFailed` without rewriting the underlying terminal result.
- `AC-094` — PROVEN by the partial-failure and failure classification path in `shell/src/run_cleanup_execution.rs`, the emitted cleanup failure events and status detail in `shell/src/run_session_start.rs`, the explicit cleanup-failure lifecycle/status projection in `shell/src/run_lifecycle_state.rs` and `shell/src/run_status_projection.rs`, and `run_cleanup_execution::tests::terminal_cleanup_classifies_partial_failure_without_rewriting_preserved_report_exemption`, which verifies visible failure detail rather than silent success implication.
- `AC-095` — PROVEN by `startup_janitor_execution::ensure_startup_janitor_executed` being called at shell launch in `shell/src/main.rs`, by abnormal residual marker creation in `shell/src/abnormal_termination_residual.rs` and the shutdown-handoff test `shutdown_handoff_writes_control_flags_and_residual_marker_for_startup_janitor` in `shell/src/run_session_start.rs`, and by the janitor tests `execute_startup_janitor_under_purges_residual_workspace_and_preserves_report`, `execute_startup_janitor_candidates_with_reports_partial_failure_without_resume_semantics`, and `execute_startup_janitor_candidates_with_marks_safe_timeout_for_unprocessed_residuals` in `shell/src/startup_janitor_execution.rs`.
- `AC-096` — PROVEN by the no-resume flags and assertions in `shell/src/run_workspace_boundary.rs`, the bounded-memory prohibition on cross-run semantic leakage in `shell/src/run_bounded_memory.rs`, the janitor contract rule `janitor_purge_implies_crash_resume: false` in `shell/src/startup_janitor_contract.rs`, and the startup-janitor failure and timeout messaging in `shell/src/startup_janitor_execution.rs`, which explicitly states that abandoned workspaces are not reused as active execution state.
- `AC-097` — PROVEN by `shell/src/report_storage_boundary.rs`, which marks durable export artifacts outside cleanup and startup-janitor scope with an explicit preserved-report exception, by `shell/src/run_cleanup_execution.rs`, which retains the preserved durable report artifact path and reports that it remained outside cleanup scope, and by `execute_startup_janitor_under_purges_residual_workspace_and_preserves_report` in `shell/src/startup_janitor_execution.rs`, which deletes the abandoned workspace while preserving the exported report.
- `AC-098` — PROVEN by the fresh-workspace and no-active-reuse rules in `shell/src/run_workspace_boundary.rs`, the bounded-memory rule `cross_run_semantic_leakage_allowed: false` in `shell/src/run_bounded_memory.rs`, the live run-start path and fresh-workspace test `start_run_session_with_components_creates_fresh_run_ids_and_workspaces` in `shell/src/run_session_start.rs`, and the janitor no-reuse messaging in `shell/src/startup_janitor_execution.rs`, which keeps abandoned working data out of later active semantic execution.

## Critique Pass
- Reviewed the touched shell, frontend, and execution-state files against the packet scope, the frozen Phase 9 blueprint, Document A `AC-092` through `AC-098`, and Document B Phase 9 validation and reopen conditions.
- Confirmed cleanup triggering, cleanup-result classification, cleanup-failure visibility, abnormal residual detection, startup janitor execution, preserved-report survival, no-resume preservation, and bounded frontend cleanup or janitor rendering remain mutually consistent across shell, frontend, and execution-state surfaces.
- Confirmed the frontend remains a shell-truth consumer and does not gain filesystem authority, cleanup authority, janitor authority, or crash-resume semantics.
- Confirmed no updater or release behavior, hosted archive behavior, forensic deletion claim, hidden continuity, or report-catalog expansion was introduced during this exit-gate pass.
- The critique pass completed cleanly and found no fresh in-scope defect that required remediation before Phase 9 closure.

## Structural Consistency and Contradiction Review
- Re-checked the touched live execution surfaces, the active packet, and the governing Phase 9 plan after the exit-gate evidence pass.
- Confirmed the closeout state is consistent with the live repo truth: Wave 3 cleanup execution exists, Wave 4 shutdown-handoff plus startup janitor behavior exists, `P09-WS11-T01` records the acceptance proof truthfully, and no later phase packet is being invented or treated as active.