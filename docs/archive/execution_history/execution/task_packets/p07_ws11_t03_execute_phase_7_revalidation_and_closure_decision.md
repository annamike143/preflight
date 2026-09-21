# Task Packet

## Task ID
P07-WS11-T03

## Task Title
Execute Phase 7 Revalidation and Closure Decision

## Parent Phase
Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

## Parent Workstream
WS-11 — Validation Recovery and Exit Gates

## Objective
Re-run the bounded Phase 7 exit-gate validation, acceptance mapping, critique pass, and closure decision after `P07-WS11-T02` implemented runtime budget/duration cap enforcement plus truthful capped/finalizing projection so Phase 7 can now either close honestly or remain open for a newly explicit blocking reason.

## Why This Exists
`P07-WS11-T01` denied closure because the repo still treated max token budget and the hard two-hour duration limit as Preflight-only planning constraints while active execution merely tracked usage. `P07-WS11-T02` implemented the missing runtime cap-control path, shell-owned constrained completion classification, finalization chronology, and frontend truth. This packet determines whether that recovered repo truth is now sufficient to close Phase 7 without inventing success.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
- Document A Section 6 — Product Behavior Specification
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 9 — Data Lifecycle and Storage Policy
- Document A Section 10 — API and Interface Contract
- Document A Section 12 — UI/UX Contract
- Document A Section 13 — Non-Functional Requirements
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
- `execution/task_packets/p07_ws11_t02_implement_runtime_budget_duration_cap_enforcement_and_truthful_capped_finalization.md`

## Dependency Preconditions
- `execution/task_packets/p07_ws11_t02_implement_runtime_budget_duration_cap_enforcement_and_truthful_capped_finalization.md` is complete.
- No other packet may remain active while this packet is active.
- The frozen Phase 7 basis remains unchanged: shell-owned lifecycle/status/event authority, real run-local tracking artifacts, bounded one-shot execution, no hidden continuation, and no overclaiming of Phase 8 report truth or Phase 9 cleanup hardening.

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
- `execution/task_packets/p07_ws11_t01_execute_phase_7_validation_critique_and_exit_gate.md`
- `execution/task_packets/p07_ws11_t02_implement_runtime_budget_duration_cap_enforcement_and_truthful_capped_finalization.md`
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
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `app/src/App.tsx`
- `app/src/App.wave7.test.tsx`

## Files / Modules Expected to Change
Create or update only:
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if unresolved handling changes materially after revalidation
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if the closure decision requires a real local governance decision
- `execution/task_packets/p07_ws11_t03_execute_phase_7_revalidation_and_closure_decision.md`
- `shell/src/` files only if the critique pass finds a material in-scope defect in the recovered Phase 7 surfaces
- `engine/src/` or `engine/tests/` files only if the critique pass finds a material in-scope defect in the recovered Phase 7 engine path
- `app/src/App.tsx` or `app/src/App.wave7.test.tsx` only if the critique pass finds a truthful UI contradiction

## Implementation Scope
In scope:
- updated acceptance mapping for `AC-060` through `AC-085`
- rerunning and reviewing the applicable shell, frontend, and engine validation gates for the recovered Phase 7 surfaces
- explicit evidence review for runtime budget/duration warning and cap enforcement, truthful constrained completion, finalization chronology, shell-owned lifecycle/status/event consistency, live monitoring truth, degradation honesty, cancellation behavior, and shutdown coordination
- critique pass over the touched recovery implementation and execution-state files
- tightly bounded in-scope remediation if the critique pass finds a real defect in the recovered Phase 7 surfaces
- explicit Phase 7 closure decision and execution-state updates

## Required Behavior to Preserve
- the packet must not treat stale Wave 7 denial truth as if it still governs after the recovery packet changed the repo
- the packet must not invent closure if the recovered runtime cap-control evidence is still incomplete or contradictory
- the shell must remain the sole authority for canonical lifecycle truth, status reconciliation, ordered event history, cancelability, cancellation acceptance, and shutdown coordination
- the frontend must remain a consumer of shell-owned truth and must not infer hidden cap hits, finalization, or constrained completion on its own
- budget/duration visibility and cap control must remain grounded in real run-local tracking artifacts and engine execution behavior
- Phase 8 report truth, Phase 9 cleanup hardening, hosted orchestration, crash-resume, and hidden continuation must remain out of scope

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- new product scope beyond revalidation and tightly bounded remediation
- Phase 8 final-report generation, export, or report-availability truth
- Phase 9 cleanup hardening, startup janitor behavior, or cleanup-success claims
- hosted orchestration, hidden continuation, crash-resume behavior, or synthetic chronology reconstruction
- unrelated provider/commercial-backend expansion, updater work, or broader UI feature work

## Deliverables
- updated acceptance mapping exists for the governed Phase 7 criteria
- all applicable Phase 7 shell, frontend, and engine gates are rerun truthfully after the recovery packet
- critique findings are either fixed in-scope or escalated explicitly
- the repo records a truthful Phase 7 closure decision

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
  - `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`
  - `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`
  - `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests`
- confirm runtime budget and duration caps now stop further normal rounds in the real engine path
- confirm live budget/duration visibility and warning state remain grounded in real run-local tracking artifacts
- confirm `Finalizing` and `CompletedBudgetCapped` are shell-owned and remain mutually consistent across lifecycle, status, event history, and frontend monitoring
- confirm in-flight run/session identity, ordered chronology, degraded-state honesty, cancellation handling, and shutdown coordination remain truthful after the recovery packet
- confirm no Phase 8 report truth, Phase 9 cleanup hardening, hosted orchestration, or crash-resume behavior was added
- confirm whether Phase 7 can close truthfully and record the result explicitly

## Escalation Triggers
Escalate immediately if:
- any governed Phase 7 acceptance target still lacks real evidence after the recovery packet
- any applicable gate reports an error or warning after in-scope remediation attempts
- the critique pass reveals contradictory lifecycle, status, event, cancellation, or shutdown truth across the recovered Phase 7 surfaces
- closing Phase 7 would require treating report availability, cleanup success, hidden continuation, or synthetic chronology as if it were already implemented correctly

## Completion Decision
COMPLETE — 2026-04-16

## Completion Notes
- Re-ran the full Phase 7 shell, frontend, and engine gate sets cleanly after the recovery packet and final shell Clippy remediation: `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` all passed in `shell/` with 98 tests green; `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` all passed in `app/` with 4 Vitest files and 10 tests green; `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests` all passed with 14 engine tests green.
- Confirmed the recovered Phase 7 implementation now enforces runtime token-budget and hard duration caps inside the real engine loop, stops scheduling new normal rounds when those caps are reached, preserves truthful constrained completion versus healthy completion, and writes the run-local `execution_phase.json` artifact that the shell consumes for live state projection.
- Confirmed shell-owned lifecycle, status, and ordered event-history surfaces now project `Finalizing` and `CompletedBudgetCapped` truthfully from runtime control evidence, including finalization-started/finalization-completed chronology and near-cap warning notes, without giving the frontend lifecycle authority.
- Confirmed the frontend monitoring surface remains a shell-truth consumer and now renders cap warnings, constrained-finalization alerts, runtime cap reason, and capped terminal truth without inventing hidden execution state.
- The critique pass found one remaining in-scope defect during the fresh rerun: `shell/src/run_status_projection.rs` used a manual range comparison that now fails the repo Clippy baseline. That issue was fixed in-scope and the full shell gate set was rerun cleanly before closure.
- Phase 7 now closes honestly because `AC-060` through `AC-085` are fully proven against the recovered live repo truth without overclaiming Phase 8 report behavior, Phase 9 cleanup hardening, hosted orchestration, crash-resume, or hidden continuation.

## Acceptance Mapping
- `AC-060` — PROVEN by `shell/src/preflight_contract.rs`, which requires `max_token_budget_required`, and `shell/src/preflight_submission.rs`, which rejects `max_token_budget == 0` before a run can begin.
- `AC-061` — PROVEN by the cumulative turn-tracking state in `engine/src/miro_fish_engine/token_duration_tracking.py` and the shell-owned `TokenDurationTracked` plus `BudgetUsageUpdated` chronology events in `shell/src/run_event_history.rs`.
- `AC-062` — PROVEN by the Phase 7 monitoring surface in `app/src/App.tsx` and the Wave 7 UI test in `app/src/App.wave7.test.tsx`, which renders tracked token usage against the governed budget.
- `AC-063` — PROVEN by `CAP_WARNING_THRESHOLD_PERCENT` and `build_cap_warning_note` in `shell/src/run_status_projection.rs`, which emit token and duration warnings once tracked usage reaches 80% but remains below the cap, together with the shell-owned warning banner logic in `app/src/App.tsx`.
- `AC-064` — PROVEN by `execute_moderated_rounds` in `engine/src/miro_fish_engine/moderated_execution.py`, which now resolves runtime token-budget exhaustion before further normal round scheduling, together with `test_moderated_execution_stops_new_normal_rounds_after_runtime_cap` in `engine/tests/test_moderated_execution.py`.
- `AC-065` — PROVEN by the recovered `completed_budget_capped` engine result shape in `engine/src/miro_fish_engine/session_execution.py`, the shell-owned constrained terminal classification in `shell/src/run_session_start.rs` and `shell/src/run_lifecycle_state.rs`, and the UI rendering asserted in `app/src/App.wave7.test.tsx`.
- `AC-066` — PROVEN by `engine/tests/test_moderated_execution.py` and `engine/tests/test_session_execution.py`, which show capped runs end with `completed_round_count < requested_round_count`, append speaker counts only for fully completed rounds, and do not schedule further normal rounds after the cap.
- `AC-067` — PROVEN by `RUN_DURATION_CAP_MS = 7_200_000` in `engine/src/miro_fish_engine/session_execution.py` and `test_session_execution_resolves_completed_budget_capped_on_hard_duration_cap` in `engine/tests/test_session_execution.py`.
- `AC-068` — PROVEN by the hard-duration-cap path in `engine/src/miro_fish_engine/moderated_execution.py` and `engine/src/miro_fish_engine/session_execution.py`, which stop further normal rounds and resolve the run as constrained completion with truthful finalization metadata when completed material exists.
- `AC-069` — PROVEN by the capped session result assertions in `engine/tests/test_session_execution.py`, which keep `moderated_rounds_materialized` false for truncated runs and preserve `CompletedBudgetCapped` rather than mislabeling incomplete planned work as healthy full completion.
- `AC-070` — PROVEN by the live monitoring surface in `app/src/App.tsx` and the Wave 7 UI coverage in `app/src/App.wave7.test.tsx`, which render shell-owned lifecycle, status, chronology, and tracking truth during active execution.
- `AC-071` — PROVEN by `shell/src/run_lifecycle_state.rs`, `shell/src/run_status_projection.rs`, and `shell/src/run_event_history.rs`, which now project `Ingesting`, `GeneratingPersonas`, `RunningRound`, `Finalizing`, `CompletedBudgetCapped`, and `Completed` from shell-known snapshots plus the run-local execution-phase artifact, together with `app/src/App.tsx`, which renders those shell-owned phase names without inventing cleanup truth. `Cleaning`, `Cleaned`, and `CleanupFailed` remain canonical vocabulary but are not falsely emitted because Phase 9 cleanup hardening is still out of scope.
- `AC-072` — PROVEN by the in-flight snapshot publication path in `shell/src/run_session_start.rs`, the ordered retrieval path in `shell/src/run_event_history.rs`, and the polling monitoring UI in `app/src/App.tsx`, which surface meaningful progress before the run ends.
- `AC-073` — PROVEN by the deterministic sequence construction in `shell/src/run_event_history.rs`, including `event_history_materializes_finalization_events_for_capped_completion`, and the Wave 7 frontend chronology assertions in `app/src/App.wave7.test.tsx`.
- `AC-074` — PROVEN by `app/src/App.tsx` and `app/src/App.wave7.test.tsx`, which show live shell-owned chronology items, phase/state truth, and usage progress rather than reducing active debate to an undifferentiated loading indicator when meaningful events exist.
- `AC-075` — PROVEN by the paused rate-limit classification and lifecycle exposure in `shell/src/run_session_start.rs`, `shell/src/run_lifecycle_state.rs`, and the Wave 7 frontend vocabulary/test coverage.
- `AC-076` — PROVEN by the paused network classification and lifecycle exposure in `shell/src/run_session_start.rs`, `shell/src/run_lifecycle_state.rs`, and the same Wave 7 monitoring truth surfaces.
- `AC-077` — PROVEN by the bounded retry handling in `shell/src/run_session_start.rs`, including the retry pause window and the `start_run_session_with_components_pauses_and_retries_retryable_execution_failures` test.
- `AC-078` — PROVEN by the explicit retryable-versus-terminal classification in `shell/src/run_session_start.rs`, which distinguishes paused retryable conditions and `FailedRetryable` from terminal failure where the repo knows that difference truthfully.
- `AC-079` — PROVEN by the recovered cap-control path in `engine/src/miro_fish_engine/moderated_execution.py`, the finalization chronology materialization in `shell/src/run_event_history.rs`, the constrained terminal classification in `shell/src/run_session_start.rs` and `shell/src/run_lifecycle_state.rs`, and the capped-monitoring UI assertions in `app/src/App.wave7.test.tsx`.
- `AC-080` — PROVEN by the shell-owned supervision path in `shell/src/run_session_start.rs`, which still classifies ordinary provider/network degradation into retryable paused/failure outcomes and bounded retry behavior instead of crashing the app shell.
- `AC-081` — PROVEN by the cancelability-gated monitoring UI in `app/src/App.tsx` and the Wave 7 frontend assertion that renders `Cancel active run` only when the shell says the run is cancelable.
- `AC-082` — PROVEN by `cancel_run_session_from_shell` in `shell/src/run_session_start.rs`, which marks cancellation accepted, writes the execution-control file, appends control events, and halts further normal progression once the engine acknowledges the request.
- `AC-083` — PROVEN by the distinct canceled classification and snapshot construction in `shell/src/run_session_start.rs`, which transitions into cancellation truth rather than successful completion.
- `AC-084` — PROVEN by `request_controlled_shutdown_from_shell` and `run_engine_module` in `shell/src/run_session_start.rs`, which request graceful engine stop first and call `child.kill()` only after the bounded graceful-stop timeout expires.
- `AC-085` — PROVEN by the shell execution-control file writes in `shell/src/run_session_start.rs`, the safe-boundary polling in `engine/src/miro_fish_engine/moderated_execution.py`, and the stop-requested early-return paths that prevent further normal rounds or retries after cancellation is accepted.

## Critique Pass
- Reviewed the touched shell, frontend, engine, and execution-state files against the packet scope, the frozen Phase 7 blueprint, Document A `AC-060` through `AC-085`, and Document B Phase 7 validation and reopen conditions.
- Confirmed runtime cap enforcement, constrained finalization, capped terminal classification, live monitoring, ordered chronology, paused/degraded truth, bounded retry, cancellation handling, and graceful-then-forceful shutdown coordination remain grounded in shell-owned runtime truth and real engine control behavior.
- Confirmed no Phase 8 report truth, Phase 9 cleanup hardening, hosted orchestration, crash-resume behavior, hidden continuation, or synthetic chronology reconstruction was introduced.
- The only fresh critique finding was the shell Clippy `manual_range_contains` warning in `shell/src/run_status_projection.rs`; it was fixed in-scope and the shell gate set was rerun cleanly before phase closure.