# Task Packet

## Task ID
P07-WS11-T02

## Task Title
Implement Runtime Budget/Duration Cap Enforcement and Truthful Capped Finalization

## Parent Phase
Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

## Parent Workstream
WS-11 — Validation Recovery and Exit-Gate Remediation

## Objective
Implement the missing runtime budget-hit and duration-cap control path so the active run stops scheduling new normal rounds once a governed cap is reached, preserves truthful constrained-completion versus terminal-failure distinction, and projects capped/finalizing truth consistently through the shell lifecycle, status, ordered event history, engine result shape, and frontend monitoring UI.

## Why This Exists
`P07-WS11-T01` reran the full Phase 7 gate set cleanly but denied closure because the repo still enforced max token budget and the hard two-hour limit only at Preflight estimation time while active execution merely tracked and displayed usage. The canonical vocabulary already includes `Finalizing` and `CompletedBudgetCapped`, but the live shell and engine path could not enter those states truthfully. This packet exists to close that exact contradiction without inventing report generation, cleanup hardening, hosted orchestration, crash-resume behavior, or broader product scope.

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
- Document B Section 3.12 — WS-11 — Validation, Critique, and Exit Gates
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.10 — Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling
- Document B Section 6.9.9 — Validation Gate
- Document B Section 6.9.10 — Failure / Reopen Conditions
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_07_master_execution_plan.md`
- `execution/task_packets/p07_ws11_t01_execute_phase_7_validation_critique_and_exit_gate.md`

## Dependency Preconditions
- `execution/task_packets/p07_ws07_t02_establish_canonical_run_lifecycle_exposure_status_contract_and_event_envelope_foundation.md` is complete.
- `execution/task_packets/p07_ws07_t03_implement_shell_status_projection_ordered_event_history_and_monitoring_retrieval_path.md` is complete.
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md` is complete.
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md` is complete.
- `execution/task_packets/p07_ws07_t06_implement_degradation_retry_controlled_cancellation_and_shutdown_integration.md` is complete.
- `execution/task_packets/p07_ws11_t01_execute_phase_7_validation_critique_and_exit_gate.md` is complete and remains the authoritative reason this recovery packet exists.
- No other packet may remain active while this packet is active.
- The frozen Phase 7 basis remains unchanged: shell-owned lifecycle/status/event authority, real run-local tracking artifacts, bounded one-shot execution, no hidden continuation, and no Phase 8 or Phase 9 overclaiming.

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
- `execution/task_packets/p07_ws11_t01_execute_phase_7_validation_critique_and_exit_gate.md`
- `shell/src/main.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/preflight_contract.rs`
- `shell/src/preflight_submission.rs`
- `shell/src/preflight_effective_plan.rs`
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/bounded_memory.py`
- `engine/src/miro_fish_engine/transcript_archive.py`
- `engine/tests/conftest.py`
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `app/src/App.tsx`
- `app/src/App.wave7.test.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/controlled_shutdown.rs` only if truthful capped/finalization control changes shell lifecycle wording there
- `shell/src/app_lifecycle.rs` only if truthful capped/finalization state exposure requires bounded lifecycle updates
- `shell/src/local_interface_contract.rs` only if truthful command-note wording must reflect the new cap-control path
- `shell/src/engine_supervision.rs` only if truthful supervision wording must reflect cap-driven finalization semantics
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/bounded_memory.py` only if controlled finalization needs bounded-summary persistence updates
- `engine/src/miro_fish_engine/transcript_archive.py` only if capped/finalization chronology needs bounded local evidence markers
- `engine/tests/conftest.py`
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `app/src/App.tsx`
- `app/src/App.wave7.test.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if a new unresolved cap-control question appears during implementation
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` only if a real local governance decision becomes necessary
- `execution/task_packets/p07_ws11_t02_implement_runtime_budget_duration_cap_enforcement_and_truthful_capped_finalization.md`

## Implementation Scope
In scope:
- runtime enforcement of max token budget and hard two-hour duration cap inside the real engine execution path
- stopping new normal round scheduling once a runtime budget or duration cap is reached
- truthful controlled finalization from completed material when constrained completion remains meaningful
- truthful terminal classification for capped completion versus failure when constrained finalization is not meaningful
- shell-owned terminal classification and lifecycle/status projection for `Finalizing`, `CompletedBudgetCapped`, and any still-truthful failure path reached by runtime cap logic
- ordered event history integration for cap-hit, finalization-started, and finalization-completed truth where the repo can know those events honestly
- bounded frontend monitoring updates that surface cap warnings, constrained-completion messaging, capped terminal truth, and any real finalization-phase visibility from shell-owned surfaces only
- shell, engine, and frontend regression coverage proving runtime cap enforcement, no hidden post-cap round scheduling, truthful constrained completion, and no false normal completion after truncation

## Required Behavior to Preserve
- keep the shell as the sole authority for canonical lifecycle truth, terminal classification, status reconciliation, and event-history truth exposed to the frontend
- keep the frontend as a consumer of shell-owned truth only; it must not invent hidden cap hits, finalization, or constrained completion on its own
- keep the engine as the producer of underlying execution activity and run-local tracking rather than the authority for external UI semantics
- preserve the frozen canonical state family and the distinction among healthy completion, constrained completion, cancellation, retryable failure, terminal failure, and any later cleanup semantics
- preserve the bounded one-shot execution model, no-hidden-continuation rule, and no crash-resume behavior
- preserve Phase 8 and Phase 9 separation so this packet does not imply report generation, report availability, cleanup hardening, janitor completion, or cleanup success
- preserve deterministic chronology and truthful round-count reporting so truncated runs are never presented as if all planned rounds completed

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- Phase 8 report generation, report availability, report export, or final-report correctness behavior
- Phase 9 cleanup hardening, startup janitor behavior, cleanup-success guarantees, or cleanup outcome validation
- hosted orchestration, push-streaming supervision, public API expansion, or hidden live-session coordination
- crash-resume, interrupted-run restoration, hidden background continuation, or synthetic chronology reconstruction
- broader provider/commercial-backend feature work, runtime delivery work, updater behavior, or unrelated UI expansion
- frontend-owned lifecycle authority, synthetic capped completion, or messaging that treats truncated work as full completion

## Deliverables
- the real engine execution path enforces runtime token-budget and duration-cap boundaries rather than treating them as Preflight-only estimates
- the shell-owned run lifecycle, status, and ordered event-history surfaces can express truthful constrained finalization and `CompletedBudgetCapped` where the repo actually reaches that path
- the frontend monitoring UI reflects shell-owned capped/finalizing truth and warning state without inventing hidden transitions
- regression coverage proves the runtime cap path stops new normal rounds, preserves truthful constrained completion versus failure, and avoids false full-completion claims
- execution state reflects the packet lifecycle honestly

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` complete with zero warnings and zero errors if frontend files change
- confirm `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests` complete cleanly if engine files change
- prove one runtime budget-cap path and one runtime duration-cap path
- prove capped runs stop normal round scheduling and do not present incomplete planned work as full completion
- prove `CompletedBudgetCapped` or truthful terminal-failure classification is shell-owned and consistent across lifecycle, status, event history, and UI rendering
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful runtime cap enforcement would require hidden hosted supervision, undeclared persistence, or live provider-health channels not already authorized by Documents A, B, or C
- constrained finalization cannot be implemented without overclaiming Phase 8 report behavior or Phase 9 cleanup behavior
- shell-owned terminal classification cannot remain singular and authoritative once runtime cap logic is materialized
- any applicable shell, frontend, or engine gate reports warnings or errors after in-scope remediation attempts

## Completion Decision
COMPLETE — 2026-04-16

## Activation Notes
- Activated as the current bounded Phase 7 recovery packet after `P07-WS11-T01` denied closure on the runtime cap-control gap.
- Repo changes were authorized only within the scope of this packet until closeout.
- The next step after this packet completed was a fresh Phase 7 exit-gate rerun against live repo truth.

## Completion Notes
- Implemented runtime token-budget and duration-cap enforcement in `engine/src/miro_fish_engine/moderated_execution.py` and `engine/src/miro_fish_engine/session_execution.py`, including constrained finalization, truthful capped result metadata, and the run-local `execution_phase.json` artifact.
- Updated shell-owned lifecycle, status, and ordered event-history projection in `shell/src/run_session_start.rs`, `shell/src/run_lifecycle_state.rs`, `shell/src/run_status_projection.rs`, and `shell/src/run_event_history.rs` so `Finalizing` and `CompletedBudgetCapped` now project from runtime control evidence rather than frontend inference.
- Updated `app/src/App.tsx` and `app/src/App.wave7.test.tsx` so live monitoring renders shell-owned cap warnings, constrained-finalization chronology, runtime cap reason, and capped terminal truth.
- Added regression coverage for runtime token-budget and duration-cap control paths in `engine/tests/test_moderated_execution.py`, `engine/tests/test_session_execution.py`, `shell/src/run_session_start.rs`, `shell/src/run_lifecycle_state.rs`, `shell/src/run_event_history.rs`, and `app/src/App.wave7.test.tsx`.
- Re-ran the full recovery validation matrix cleanly after implementation and final shell Clippy remediation: `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` in `shell/` all passed with 98 tests green; `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` in `app/` all passed with 4 Vitest files and 10 tests green; `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests` all passed with 14 engine tests green.
- The critique pass found no remaining contradiction across engine, shell, frontend, and execution-state surfaces after the runtime cap-control path was materialized.