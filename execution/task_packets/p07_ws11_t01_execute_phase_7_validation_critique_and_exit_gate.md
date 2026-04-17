# Task Packet

## Task ID
P07-WS11-T01

## Task Title
Execute Phase 7 Validation, Critique, and Exit Gate

## Parent Phase
Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

## Parent Workstream
WS-11 — Validation, Critique, and Exit Gates

## Objective
Execute the Phase 7 exit-gate validation pass, acceptance mapping, critique pass, and closure decision so the repo can either close Phase 7 honestly or remain open with an explicit blocking reason if any required budget/duration visibility, canonical state exposure, deterministic chronology, in-flight monitoring truth, degraded-state honesty, cancellation behavior, or shutdown coordination evidence is still missing.

## Why This Exists
`P07-W07` is the final Phase 7 packet defined by `execution/phase_07_master_execution_plan.md`. Waves 2 through 6 established the canonical lifecycle/status/event surfaces, real shell-owned status and event retrieval, live monitoring UI, in-flight run/session snapshot publication, and implemented degradation/cancellation/shutdown integration. This packet now proves whether that implementation actually satisfies the governed Phase 7 acceptance targets and whether Phase 7 may close without inventing live-visibility, interruption, or closure truth that the repo does not support.

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
- Document B Section 6.9 — Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling
- Document B Section 6.9.9 — Validation Gate
- Document B Section 6.9.10 — Failure / Reopen Conditions
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_07_master_execution_plan.md`
- `execution/task_packets/p07_ws07_t02_establish_canonical_run_lifecycle_exposure_status_contract_and_event_envelope_foundation.md`
- `execution/task_packets/p07_ws07_t03_implement_shell_status_projection_ordered_event_history_and_monitoring_retrieval_path.md`
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md`
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md`
- `execution/task_packets/p07_ws07_t06_implement_degradation_retry_controlled_cancellation_and_shutdown_integration.md`

## Dependency Preconditions
- `execution/task_packets/p07_ws07_t02_establish_canonical_run_lifecycle_exposure_status_contract_and_event_envelope_foundation.md` is complete.
- `execution/task_packets/p07_ws07_t03_implement_shell_status_projection_ordered_event_history_and_monitoring_retrieval_path.md` is complete.
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md` is complete.
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md` is complete.
- `execution/task_packets/p07_ws07_t06_implement_degradation_retry_controlled_cancellation_and_shutdown_integration.md` is complete.
- No other packet may remain active while this packet is active.
- The frozen Phase 7 basis remains unchanged: canonical lifecycle exposure, deterministic ordered event retrieval, shell-owned live status/event truth, in-flight run/session identity during active execution, paused/degraded truth, bounded retry visibility, controlled cancellation handling, and active-run shutdown coordination without Phase 8 report truth or Phase 9 cleanup hardening.

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
- `execution/task_packets/p07_ws07_t02_establish_canonical_run_lifecycle_exposure_status_contract_and_event_envelope_foundation.md`
- `execution/task_packets/p07_ws07_t03_implement_shell_status_projection_ordered_event_history_and_monitoring_retrieval_path.md`
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md`
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md`
- `execution/task_packets/p07_ws07_t06_implement_degradation_retry_controlled_cancellation_and_shutdown_integration.md`
- `shell/Cargo.toml`
- `shell/Cargo.lock`
- `shell/src/main.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_moderated_rounds.rs`
- `shell/src/run_token_duration_tracking.rs`
- `engine/pyproject.toml`
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/transcript_archive.py`
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `engine/tests/test_token_duration_tracking.py`
- `engine/tests/test_transcript_archive.py`
- `app/package.json`
- `app/package-lock.json`
- `app/vite.config.ts`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`
- `app/src/App.wave5.test.tsx`
- `app/src/App.wave7.test.tsx`

## Files / Modules Expected to Change
Create or update only:
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if unresolved Phase 7 questions change materially after the exit-gate pass
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if the Phase 7 closure decision requires a real local governance decision
- `execution/task_packets/p07_ws11_t01_execute_phase_7_validation_critique_and_exit_gate.md`
- `shell/Cargo.toml` or `shell/Cargo.lock` only if the critique pass finds an in-scope Phase 7 defect that requires a bounded dependency or lockfile update
- `shell/src/main.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_moderated_rounds.rs`
- `shell/src/run_token_duration_tracking.rs`
- `engine/pyproject.toml`
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/transcript_archive.py`
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `engine/tests/test_token_duration_tracking.py`
- `engine/tests/test_transcript_archive.py`
- `app/package.json`
- `app/package-lock.json`
- `app/vite.config.ts`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`
- `app/src/App.wave5.test.tsx`
- `app/src/App.wave7.test.tsx`

## Implementation Scope
In scope:
- acceptance mapping for `AC-060` through `AC-085`
- rerunning the applicable shell, frontend, and engine validation gates for the implemented Phase 7 surfaces
- explicit evidence review for live budget/duration visibility, canonical state/phase visibility, deterministic chronology, in-flight run/session identity during active execution, paused/degraded truth, retryable-versus-terminal distinction where known, accepted-cancel halt behavior, and graceful-then-forceful shutdown handling
- critique pass over the touched Phase 7 shell, frontend, engine, and execution-state files
- tightly bounded in-scope remediation if the critique pass finds a real Phase 7 defect in the implemented monitoring/interruption surfaces
- explicit Phase 7 closure decision and execution-state updates

## Required Behavior to Preserve
- the packet must not invent acceptance proof from earlier contract text, stale pre-Wave-6 statements, or execution-scaffold posture that no longer matches the live repo
- the shell must remain the sole authority for canonical lifecycle truth, status/event reconciliation, cancelability, cancellation acceptance, and shutdown coordination
- the frontend must remain a consumer of shell-owned truth and must not infer hidden states, chronology, or cancelability on its own
- budget/duration visibility must remain grounded in real run-local tracking artifacts rather than synthetic counters
- status and event history must continue to tell a mutually consistent story across active, degraded, canceled, failed, and terminal paths
- the canonical run-state family must remain singular; no second competing vocabulary may be introduced during exit assessment
- if the evidence does not support Phase 7 closure, the packet must leave Phase 7 open honestly rather than forcing closure

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- Phase 8 final report generation, export, or report-availability truth
- Phase 9 cleanup hardening, startup janitor behavior, cleanup guarantees, or cleanup-success claims
- hosted orchestration, push-streaming supervision, crash-resume behavior, hidden continuation, or synthetic chronology reconstruction
- updater/runtime-delivery work, public API expansion, or unrelated provider/commercial-backend scope
- broader product-feature expansion beyond the validation pass and tightly bounded in-scope remediation

## Deliverables
- acceptance mapping exists for the governed Phase 7 criteria
- all applicable Phase 7 shell, frontend, and engine gates are rerun truthfully
- critique findings are either fixed in-scope or escalated explicitly
- the repo records a truthful Phase 7 exit-gate decision

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
- confirm live budget/duration visibility remains grounded in real run-local tracking artifacts
- confirm canonical lifecycle and phase visibility remain shell-owned and mutually consistent with status and event history
- confirm in-flight run/session identity and ordered chronology remain truthful during active execution
- confirm degraded states are not mislabeled as healthy progress and retryable-versus-terminal distinction appears only where the repo knows it truthfully
- confirm accepted cancellation stops normal forward progression and is reflected consistently in lifecycle, status, event history, and frontend monitoring
- confirm controlled shutdown still attempts graceful engine stop first and escalates to forceful termination only when required
- confirm no Phase 8 report truth, Phase 9 cleanup hardening, hosted orchestration, or crash-resume behavior was added
- confirm whether Phase 7 can close truthfully and record the result explicitly

## Escalation Triggers
Escalate immediately if:
- any governed Phase 7 acceptance target still lacks real evidence after the validation pass
- any applicable gate reports an error or warning after in-scope remediation attempts
- the critique pass reveals contradictory lifecycle, status, event, cancellation, or shutdown truth across the implemented Phase 7 surfaces
- closing Phase 7 would require treating report availability, cleanup success, hidden continuation, or synthetic chronology as if it were already implemented correctly

## Completion Decision
COMPLETE — 2026-04-16

## Activation Notes
- Activated as the current Phase 7 Wave 7 packet for acceptance mapping, validation reruns, critique, tightly bounded remediation if needed, and the final exit-gate decision over the implemented monitoring and interruption surfaces.
- Phase 7 closure is not authorized at activation time and must not be overclaimed until this packet proves `AC-060` through `AC-085` against the live repo truth.

## Completion Notes
- Re-ran the full Phase 7 shell, frontend, and engine gate sets cleanly during this exit-gate pass: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests`.
- Confirmed live monitoring remains grounded in real run-local tracking artifacts and shell-owned status/event history, with incremental chronology, in-flight run/session identity, paused/degraded truth, accepted cancellation handling, and graceful-then-forceful shutdown coordination still materialized end to end.
- The critique pass found no gate failures or behavioural regressions inside the implemented monitoring and interruption surfaces, but it did confirm a material Phase 7 closure blocker: the repo still enforces max token budget and the hard two-hour limit only at Preflight estimation time while active execution merely tracks and displays live token/duration usage.
- Because the live run path does not yet stop scheduling new rounds on runtime budget or duration cap reach, does not yet transition into truthful controlled finalization or `CompletedBudgetCapped`, and does not yet project live `Finalizing` or `Cleaning` phases, the governed Phase 7 acceptance set is not fully proven.
- No product-code changes were required during this exit-gate pass; only the allowlisted execution-state files and this packet file were updated to record the validation evidence, critique result, and non-closure decision.
- Phase 7 therefore does not close honestly at this time and remains open between packets pending a later bounded packet for runtime budget/duration cap enforcement and truthful capped/finalization state projection.

## Acceptance Mapping
- `AC-060` — PROVEN by `shell/src/preflight_contract.rs`, which requires `max_token_budget_required`, `shell/src/preflight_submission.rs`, which rejects `max_token_budget == 0`, and the run-start path, which still depends on current eligible Preflight truth before a run can begin.
- `AC-061` — PROVEN by the cumulative turn-tracking state in `engine/src/miro_fish_engine/token_duration_tracking.py` and the shell-owned `TokenDurationTracked` plus `BudgetUsageUpdated` chronology events in `shell/src/run_event_history.rs`.
- `AC-062` — PROVEN by the Phase 7 monitoring surface in `app/src/App.tsx`, which renders tracked token usage against the budget cap, together with `app/src/App.wave7.test.tsx`, which asserts `320 token units of 12000`.
- `AC-063` — NOT YET PROVEN in full. The monitoring UI shows percentage progress and exhaustion coloring, but the repo does not implement a meaningful pre-exhaustion budget-warning threshold before the cap is actually reached.
- `AC-064` — NOT YET PROVEN. The current repo requires a budget at Preflight time and tracks usage during execution, but no live shell or engine control path stops scheduling new normal rounds and enters controlled finalization when the runtime budget is hit.
- `AC-065` — NOT YET PROVEN. `CompletedBudgetCapped` exists in `shell/src/run_lifecycle_state.rs` and the frontend vocabulary, but the live execution path never transitions into that terminal classification.
- `AC-066` — NOT YET PROVEN. Because runtime budget-hit logic is absent, the repo does not yet prove that normal rounds stop once budget-cap control becomes active.
- `AC-067` — NOT YET PROVEN. `shell/src/preflight_effective_plan.rs` enforces the hard two-hour limit only against estimated requested/effective plans, but the active execution path does not apply a live hard two-hour stop while the run is in progress.
- `AC-068` — NOT YET PROVEN. No live shell or engine control path currently stops normal round scheduling at a runtime duration-cap boundary and then chooses controlled finalization or truthful terminal failure.
- `AC-069` — NOT YET PROVEN. The repo has no implemented budget- or duration-capped truncation path yet, so it cannot truthfully classify an incomplete planned run as constrained rather than fully completed.
- `AC-070` — PROVEN by the live monitoring surface in `app/src/App.tsx` and `app/src/App.wave7.test.tsx`, which render shell-owned run status, chronology, and monitoring truth during active execution.
- `AC-071` — NOT YET PROVEN in full. The live repo projects `Ingesting`, `GeneratingPersonas`, `RunningRound`, and paused/degraded states truthfully, but `Finalizing` and `Cleaning` currently remain canonical vocabulary and frontend styling rather than live projected runtime phases.
- `AC-072` — PROVEN by the in-flight snapshot publication in `shell/src/run_session_start.rs`, the ordered retrieval path in `shell/src/run_event_history.rs`, and the polling monitoring UI in `app/src/App.tsx`, which surface meaningful progress before the run ends.
- `AC-073` — PROVEN by the ordered sequence construction in `shell/src/run_event_history.rs`, the chronology materialization test coverage in `cargo test`, and the Phase 7 frontend assertions in `app/src/App.wave7.test.tsx`.
- `AC-074` — PROVEN by `app/src/App.tsx` and `app/src/App.wave7.test.tsx`, which show live shell-owned chronology items, phase/state truth, and usage progress rather than only an undifferentiated loading indicator when meaningful progress events exist.
- `AC-075` — PROVEN by the paused rate-limit classification and lifecycle exposure in `shell/src/run_session_start.rs`, `shell/src/run_lifecycle_state.rs`, and the Wave 7 frontend vocabulary/test coverage.
- `AC-076` — PROVEN by the paused network classification and lifecycle exposure in `shell/src/run_session_start.rs`, `shell/src/run_lifecycle_state.rs`, and the same Wave 7 monitoring truth surfaces.
- `AC-077` — PROVEN by the bounded retry handling in `shell/src/run_session_start.rs`, including the retry pause window and the `start_run_session_with_components_pauses_and_retries_retryable_execution_failures` test.
- `AC-078` — PROVEN by the explicit retryable-versus-terminal classification in `shell/src/run_session_start.rs`, which distinguishes paused retryable conditions and `FailedRetryable` from terminal failure where the repo knows that difference truthfully.
- `AC-079` — NOT YET PROVEN. The repo truthfully exposes paused/degraded, canceled, retryable-failure, and terminal-failure states, but it does not yet implement or surface a real interruption-to-controlled-finalization path that would present constrained completion rather than normal full completion.
- `AC-080` — PROVEN by the shell-owned supervision path in `shell/src/run_session_start.rs`, which classifies ordinary provider/network degradation into retryable paused/failure outcomes and bounded retry behavior instead of crashing the app shell.
- `AC-081` — PROVEN by the cancelability-gated monitoring UI in `app/src/App.tsx` and the Wave 7 frontend assertion that renders `Cancel active run` when the run is cancelable.
- `AC-082` — PROVEN by `cancel_run_session_from_shell` in `shell/src/run_session_start.rs`, which marks cancellation accepted, writes the execution-control file, appends control events, and halts further normal progression once the engine acknowledges the request.
- `AC-083` — PROVEN by the distinct canceled classification and snapshot construction in `shell/src/run_session_start.rs`, which transitions into cancellation truth rather than successful completion.
- `AC-084` — PROVEN by `request_controlled_shutdown_from_shell` and `run_engine_module` in `shell/src/run_session_start.rs`, which request graceful engine stop first and call `child.kill()` only after the bounded graceful-stop timeout expires.
- `AC-085` — PROVEN by the shell execution-control file writes in `shell/src/run_session_start.rs`, the safe-boundary polling in `engine/src/miro_fish_engine/moderated_execution.py`, and the stop-requested early-return paths that prevent further normal rounds or retries after cancellation is accepted.

## Critique Pass
- Reviewed the touched shell, frontend, engine, and execution-state files against the packet scope, the frozen Phase 7 blueprint, Document A `AC-060` through `AC-085`, and Document B Phase 7 validation and reopen conditions.
- Confirmed live monitoring, ordered chronology, paused/degraded truth, bounded retry, cancellation handling, and graceful-then-forceful shutdown coordination remain grounded in shell-owned runtime truth and real engine interruption signals.
- Confirmed no Phase 8 report truth, Phase 9 cleanup hardening, hosted orchestration, crash-resume behavior, or synthetic chronology reconstruction was introduced.
- The material remaining contradiction is runtime cap control: the repo still treats max token budget and the hard duration cap as Preflight planning constraints plus live visibility, not as active execution stop/finalization control.