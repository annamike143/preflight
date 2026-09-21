# Task Packet

## Task ID
P06-WS06-T05

## Task Title
Implement Moderated Round Execution and Token/Duration Tracking

## Parent Phase
Phase 6 — Simulation Engine Core

## Parent Workstream
WS-06 — Simulation Engine and Persona System

## Objective
Implement the active bounded moderated run path on top of the completed Wave 3 and Wave 4 foundations, including Moderator-controlled multi-round execution, normal-round 3-to-5-speaker selection, speaker-attributed turn capture, round-summary and argument/risk continuity updates, and integrated run-local token and duration tracking, while keeping Phase 7 live monitoring fidelity, Phase 8 final-report behavior, and Phase 9 cleanup hardening out of scope.

## Why This Exists
`P06-WS06-T04` materialized the engine-owned runtime persona foundation, exactly one Moderator basis, bounded memory initialization, transcript-archive separation, and shell-owned engine bootstrap wiring, but the repo still cannot truthfully execute even one moderated round or account for run-local token or duration usage. The current run path stops at a pending-execution foundation state rather than a real active bounded moderated run. Phase 6 cannot reach its validation and exit packet honestly until Wave 5 makes real Moderator-controlled rounds, speaker-attributed execution, continuity updates, and token/duration tracking materially exist.

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
- Document B Section 3.9 — WS-06 — Simulation Engine and Persona System
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.9 — Phase 6 — Simulation Engine Core
- Document B Section 6.8 — Phase 6 — Simulation Engine Core
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_06_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p06_ws06_t01_establish_phase_6_simulation_engine_core_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/task_packets/p06_ws06_t02_establish_run_session_contracts_workspace_isolation_and_start_lineage_foundation.md` is complete.
- `execution/task_packets/p06_ws06_t03_implement_fresh_run_creation_workspace_allocation_and_seed_execution_ingestion.md` is complete.
- `execution/task_packets/p06_ws06_t04_implement_runtime_persona_generation_moderator_basis_and_bounded_memory_foundation.md` is complete.
- `execution/phase_06_master_execution_plan.md` remains the authoritative Phase 6 packetization basis.
- Existing shell-owned run/session, workspace-allocation, start-lineage, run-start-readiness, engine-supervision, trust-boundary, and run-session state surfaces remain available as the required Wave 2 through Wave 4 foundation.
- The bounded `engine/` package now exists and may be extended only within this packet's allowlist.
- Rust/Cargo/Clippy, Node.js/npm, and repo-local Python validation tooling remain available locally.

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
- `execution/phase_06_master_execution_plan.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_start_readiness.rs`
- `shell/src/runtime_persona_contract.rs`
- `shell/src/run_bounded_memory.rs`
- `engine/pyproject.toml`
- `engine/src/miro_fish_engine/__init__.py`
- `engine/src/miro_fish_engine/session_bootstrap.py`
- `engine/src/miro_fish_engine/runtime_personas.py`
- `engine/src/miro_fish_engine/moderator_basis.py`
- `engine/src/miro_fish_engine/bounded_memory.py`
- `engine/src/miro_fish_engine/transcript_archive.py`
- `engine/tests/conftest.py`
- `engine/tests/test_session_bootstrap.py`
- `engine/tests/test_runtime_personas.py`
- `engine/tests/test_moderator_basis.py`
- `engine/tests/test_bounded_memory.py`
- `engine/tests/test_transcript_archive.py`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`
- `engine/`

## Files / Modules Expected to Change
Create or update only:
- `shell/Cargo.toml` if bounded shell-engine integration dependencies are required
- `shell/Cargo.lock` if Cargo dependency resolution changes it during the same bounded update
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_start_readiness.rs`
- `shell/src/runtime_persona_contract.rs`
- `shell/src/run_bounded_memory.rs`
- `shell/src/run_moderated_rounds.rs`
- `shell/src/run_token_duration_tracking.rs`
- `engine/pyproject.toml`
- `engine/src/miro_fish_engine/__init__.py`
- `engine/src/miro_fish_engine/session_bootstrap.py`
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/runtime_personas.py`
- `engine/src/miro_fish_engine/moderator_basis.py`
- `engine/src/miro_fish_engine/bounded_memory.py`
- `engine/src/miro_fish_engine/transcript_archive.py`
- `engine/tests/conftest.py`
- `engine/tests/test_session_bootstrap.py`
- `engine/tests/test_runtime_personas.py`
- `engine/tests/test_moderator_basis.py`
- `engine/tests/test_bounded_memory.py`
- `engine/tests/test_transcript_archive.py`
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `engine/tests/test_token_duration_tracking.py`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`
- `app/src/App.wave5.test.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` if moderated-round or tracking questions change materially
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if a real governance decision becomes necessary
- `execution/task_packets/p06_ws06_t05_implement_moderated_round_execution_and_token_duration_tracking.md`

## Implementation Scope
In scope:
- active Moderator-controlled multi-round execution from the already-bootstrapped run workspace
- per-round 3-to-5-speaker selection as the normal round rule
- speaker-attributed turn capture into the transcript archive and bounded continuity layers
- real round-summary and argument/risk continuity mutation across rounds
- run-local token-usage and duration-tracking integration surfaced through shell-owned supervision truth
- shell-owned run-session, lifecycle, supervision, startup, trust-boundary, local-interface, and command-surface updates needed to expose real Wave 5 execution and tracking truth without claiming Phase 7 live monitoring fidelity
- bounded frontend truth updates only if needed to render shell-owned Wave 5 state honestly
- tests for moderated round execution, 3-to-5-speaker selection, speaker attribution, continuity mutation, and token/duration tracking

## Required Behavior to Preserve
- keep run execution dependent on current eligible Preflight truth rather than bypassing Phase 5
- keep the Rust/Tauri shell as the sole local authority for run/session identity, workspace allocation, run-start authorization, and coarse supervision truth
- keep the Python engine as the owner of moderated execution, speaker selection, turn attribution, bounded continuity mutation, and run-local token/duration tracking once Wave 5 implementation begins
- keep runtime personas, exactly one Moderator, stable labels, and bounded-memory foundations derived from the approved seed context and current run only
- keep normal rounds as Moderator-controlled 3-to-5-speaker subsets rather than all-agent full fanout
- keep transcript archive structurally separate from bounded prompt memory and forbid naive full-transcript replay as the core memory model
- keep cross-run semantic leakage forbidden: no prior-run transcript, summary, persona state, or memory content may become active state for a later run
- keep provider secrets, activation tokens, and hidden continuation state out of run workspaces, engine memory artifacts, and ordinary logs
- preserve the completed Wave 3 and Wave 4 fresh-run, workspace-allocation, seed-ingestion, persona-foundation, and bounded-memory-foundation behavior while keeping Phase 7 monitoring fidelity, Phase 8 finalization/report truth, and Phase 9 cleanup hardening out of scope

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- Phase 7 live monitoring fidelity, interruption controls, or rich user-visible streaming telemetry
- Phase 8 final report generation, export, or final-report truth
- Phase 9 cleanup hardening, startup janitor behavior, or end-to-end cleanup closure
- crash-resume, hidden continuation, or any reuse of prior-run semantic state
- static user-authored persona configuration
- all-agent normal-round fanout as a substitute for Moderator-controlled 3-to-5-speaker selection
- naive full-transcript replay as the active prompt-memory model
- commercial-backend expansion, hosted orchestration behavior, or public API behavior
- hidden persistence outside the governed run-workspace or support-state boundaries

## Deliverables
- `shell/src/run_moderated_rounds.rs` exists and defines the shell-visible moderated-round execution surface truthfully
- `shell/src/run_token_duration_tracking.rs` exists and defines the shell-visible token/duration tracking surface truthfully
- `engine/` contains a bounded Python execution path that can run moderated rounds, preserve speaker attribution, update bounded continuity layers, and compute run-local token/duration tracking from the approved run workspace
- shell lifecycle, startup, local-interface, engine-supervision, run-session, and command surfaces expose truthful Wave 5 execution and tracking state without implying Phase 7 live monitoring fidelity or Phase 8 finalization
- the frontend, if touched, renders only shell-owned Wave 5 execution and tracking truth without gaining authority over Moderator decisions, speaker selection, or usage accounting

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowlisted files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors when shell code changes are in scope
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` complete with zero warnings and zero errors if frontend files change
- confirm the applicable engine gate family completes cleanly with Python compile validation for `engine/src` and `engine/tests`, `ruff check engine`, and `pytest engine/tests`
- confirm normal rounds are Moderator-controlled 3-to-5-speaker subsets rather than all-agent full fanout
- confirm speaker-attributed turn capture is real and is not fabricated to conceal missing execution
- confirm round-summary and argument/risk continuity updates mutate bounded memory during real execution rather than report-only reconstruction
- confirm token and duration tracking participate in the real run path rather than appearing only as after-the-fact metadata
- confirm no Phase 7 live monitoring fidelity, Phase 8 final-report behavior, or Phase 9 cleanup hardening was implemented
- perform a critique pass and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful moderated round execution or token/duration tracking cannot be implemented without changing Documents A, B, or C
- the current Wave 4 foundations do not expose enough approved run context to execute real Moderator-controlled rounds truthfully
- 3-to-5-speaker normal-round selection, continuity mutation, or usage tracking would require hidden persistence, hosted orchestration, or public API expansion beyond the frozen Phase 6 basis
- the execution path would collapse into fabricated turns, all-agent fanout, naive transcript replay, or hidden cross-run leakage
- any applicable shell, frontend, or engine gate still reports warnings or errors after in-scope fixes

## Current State
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 6 Wave 5 packet for moderated round execution, per-round 3-to-5-speaker selection, speaker-attributed turn capture, bounded continuity mutation, and run-local token/duration tracking.
- Repo changes are authorized only within the scope of this packet until closeout or blockage.
- Phase 7 live monitoring fidelity, Phase 8 finalization/report behavior, and Phase 9 cleanup hardening remain explicitly out of scope at activation time.

## Completion Notes
- Added the bounded Wave 5 engine execution path under `engine/src/miro_fish_engine/` so approved staged run workspaces now execute real Moderator-controlled multi-round runs with 3-to-5-speaker speaker subsets, speaker-attributed turns, bounded continuity mutation, and run-local token or duration tracking.
- Added `shell/src/run_moderated_rounds.rs` and `shell/src/run_token_duration_tracking.rs`, updated the shell lifecycle, startup, trust-boundary, local-interface, supervision, run-session, and main-command surfaces, and expanded `shell/src/run_session_start.rs` so the shell-owned fresh run path now finishes after real engine execution rather than stopping at a pending-execution foundation state.
- Updated `app/src/App.tsx`, `app/src/App.test.tsx`, `app/src/App.wave4.test.tsx`, and `app/src/App.wave5.test.tsx` so the frontend renders the shell-owned Wave 5 moderated-round and tracking truth plus the expanded finished run-session snapshot without gaining authority over Moderator decisions, speaker selection, or usage accounting.
- Product-code changes remained confined to the allowlisted shell, frontend, and engine files for this packet; closeout updates remained confined to the allowlisted execution-state files. No git repository is present in this workspace, so that boundary was verified directly from the applied patch set and targeted file inspection.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, Python compile validation for `engine/src` and `engine/tests`, `ruff check engine`, and `pytest engine/tests`.

## Critique Pass
- Reviewed the touched shell, frontend, engine, and execution-state files against the packet scope, the frozen Phase 6 blueprint, and the upstream Document A and Document B Phase 6 requirements.
- Confirmed normal rounds are Moderator-controlled 3-to-5-speaker subsets rather than all-agent fanout, speaker attribution is persisted into transcript-archive artifacts, bounded memory continuity mutates during execution, and token or duration tracking participates in the real run path.
- Confirmed the shell remains the sole authority for run/session identity, workspace allocation, start authorization, and coarse truth, while the engine owns moderated execution, speaker selection, continuity mutation, and run-local token or duration tracking.
- Confirmed no Phase 7 live monitoring fidelity, Phase 8 final-report behavior, Phase 9 cleanup hardening, crash-resume, hidden continuation, or naive full-transcript replay was implemented during this packet.
- No material contradiction, hidden-scope expansion, or in-scope defect remained after validation and review, so no remediation was required before marking the packet complete.