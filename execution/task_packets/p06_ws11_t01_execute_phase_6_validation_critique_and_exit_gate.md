# Task Packet

## Task ID
P06-WS11-T01

## Task Title
Execute Phase 6 Validation, Critique, and Exit Gate

## Parent Phase
Phase 6 — Simulation Engine Core

## Parent Workstream
WS-11 — Validation, Critique, and Exit Gates

## Objective
Execute the Phase 6 exit-gate validation pass, acceptance mapping, critique pass, and closure decision so the repo can either close Phase 6 honestly or remain open with an explicit blocking reason if any required fresh-run, workspace-isolation, runtime-persona, Moderator, bounded-memory, moderated-round, or token or duration tracking evidence is still missing.

## Why This Exists
`P06-W06` is the final Phase 6 packet defined by `execution/phase_06_master_execution_plan.md`. Waves 2 through 5 established the run/session foundation, real fresh-run workspace allocation and seed execution ingestion, runtime persona generation and bounded-memory foundation, and the bounded one-shot moderated execution path with speaker attribution and token or duration tracking. This packet now proves whether that implementation actually satisfies the governed Phase 6 acceptance targets and whether Phase 6 may close without inventing fresh-run, Moderator, bounded-memory, or execution truth that the repo does not support.

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
- Document B Section 3.9 — WS-06 — Simulation Engine and Persona System
- Document B Section 3.12 — WS-11 — Validation, Critique, and Exit Gates
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.9 — Phase 6 — Simulation Engine Core
- Document B Section 6.8 — Phase 6 — Simulation Engine Core
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_06_master_execution_plan.md`
- `execution/task_packets/p06_ws06_t02_establish_run_session_contracts_workspace_isolation_and_start_lineage_foundation.md`
- `execution/task_packets/p06_ws06_t03_implement_fresh_run_creation_workspace_allocation_and_seed_execution_ingestion.md`
- `execution/task_packets/p06_ws06_t04_implement_runtime_persona_generation_moderator_basis_and_bounded_memory_foundation.md`
- `execution/task_packets/p06_ws06_t05_implement_moderated_round_execution_and_token_duration_tracking.md`

## Dependency Preconditions
- `execution/task_packets/p06_ws06_t02_establish_run_session_contracts_workspace_isolation_and_start_lineage_foundation.md` is complete.
- `execution/task_packets/p06_ws06_t03_implement_fresh_run_creation_workspace_allocation_and_seed_execution_ingestion.md` is complete.
- `execution/task_packets/p06_ws06_t04_implement_runtime_persona_generation_moderator_basis_and_bounded_memory_foundation.md` is complete.
- `execution/task_packets/p06_ws06_t05_implement_moderated_round_execution_and_token_duration_tracking.md` is complete.
- No other packet may remain active while this packet is active.
- The frozen Phase 6 basis remains unchanged: fresh-run plus fresh-workspace execution, current eligible Preflight lineage at run start, runtime-generated personas with exactly one Moderator, Moderator-controlled 3-to-5-speaker normal rounds, bounded memory, transcript-archive separation, and token or duration tracking integrated into real run behavior.

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
- `execution/task_packets/p06_ws06_t02_establish_run_session_contracts_workspace_isolation_and_start_lineage_foundation.md`
- `execution/task_packets/p06_ws06_t03_implement_fresh_run_creation_workspace_allocation_and_seed_execution_ingestion.md`
- `execution/task_packets/p06_ws06_t04_implement_runtime_persona_generation_moderator_basis_and_bounded_memory_foundation.md`
- `execution/task_packets/p06_ws06_t05_implement_moderated_round_execution_and_token_duration_tracking.md`
- `shell/Cargo.toml`
- `shell/Cargo.lock`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/run_start_lineage.rs`
- `shell/src/run_start_readiness.rs`
- `shell/src/run_session_start.rs`
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
- `app/package.json`
- `app/package-lock.json`
- `app/vite.config.ts`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`
- `app/src/App.wave5.test.tsx`

## Files / Modules Expected to Change
Create or update only:
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if unresolved Phase 6 questions change materially after the exit-gate pass
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if the Phase 6 closure decision requires a real local governance decision
- `execution/task_packets/p06_ws11_t01_execute_phase_6_validation_critique_and_exit_gate.md`
- `shell/Cargo.toml` or `shell/Cargo.lock` only if the critique pass finds an in-scope Phase 6 defect that requires a bounded dependency or lockfile update
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/run_start_lineage.rs`
- `shell/src/run_start_readiness.rs`
- `shell/src/run_session_start.rs`
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
- `app/package.json`
- `app/package-lock.json`
- `app/vite.config.ts`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`
- `app/src/App.wave5.test.tsx`

## Implementation Scope
In scope:
- acceptance mapping for `AC-041` through `AC-059`
- rerunning the applicable shell, frontend, and engine validation gates for the implemented Phase 6 surfaces
- explicit evidence review for fresh-run behavior, workspace isolation, current eligible Preflight lineage at run start, runtime persona generation, exactly one Moderator, 3-to-5-speaker moderated rounds, bounded memory, transcript-archive separation, and token or duration tracking
- critique pass over the touched Phase 6 shell, frontend, engine, and execution-state files
- tightly bounded in-scope remediation if the critique pass finds a real Phase 6 defect in the implemented surfaces
- explicit Phase 6 closure decision and execution-state updates

## Required Behavior to Preserve
- the packet must not invent acceptance proof from earlier contract text, scaffolding, or stale pre-Wave-5 statements
- run start must remain dependent on current eligible Preflight truth and must not bypass the closed Phase 5 lineage basis
- each run must remain a fresh run with a fresh isolated workspace and no hidden continuation or crash-resume path
- runtime personas must remain runtime-generated from approved seed context with exactly one Moderator and stable persona labeling
- normal moderated rounds must remain Moderator-controlled 3-to-5-speaker subsets rather than all-agent full fanout
- bounded memory must remain distinct from transcript-archive visibility and must not collapse into naive full-transcript replay
- token and duration tracking must remain integrated into the real run path rather than being treated as report-only metadata
- if the evidence does not support Phase 6 closure, the packet must leave Phase 6 open honestly rather than forcing closure

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- Phase 7 live monitoring fidelity, interruption handling, or rich user-visible streaming telemetry
- Phase 8 final report generation, export, or final-report truth
- Phase 9 cleanup hardening, startup janitor behavior, or end-to-end cleanup closure
- crash-resume, hidden continuation, or reuse of prior-run semantic state
- static user-authored persona configuration
- all-agent normal-round fanout as a substitute for Moderator-controlled 3-to-5-speaker selection
- naive full-transcript replay as the active prompt-memory model
- commercial-backend expansion, hosted orchestration behavior, or public API behavior
- hidden persistence outside the governed run-workspace or support-state boundaries

## Deliverables
- acceptance mapping exists for the governed Phase 6 criteria
- all applicable Phase 6 shell, frontend, and engine gates are rerun truthfully
- critique findings are either fixed in-scope or escalated explicitly
- the repo records a truthful Phase 6 exit-gate decision

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
- confirm run execution still depends on current eligible Preflight truth and fresh isolated workspace allocation
- confirm runtime personas remain runtime-generated from approved seed context and still include exactly one Moderator
- confirm normal rounds remain Moderator-controlled 3-to-5-speaker subsets rather than all-agent fanout
- confirm bounded memory remains structurally distinct from transcript-archive visibility and mutates during real execution
- confirm token and duration tracking still participate in the real run path rather than appearing only after the fact
- confirm no Phase 7 live monitoring fidelity, Phase 8 final-report behavior, or Phase 9 cleanup hardening was added
- confirm whether Phase 6 can close truthfully and record the result explicitly

## Escalation Triggers
Escalate immediately if:
- any governed Phase 6 acceptance target still lacks real evidence after the validation pass
- any applicable gate reports an error or warning after in-scope remediation attempts
- the critique pass reveals that the implemented Phase 6 path still depends on stale, hidden, or contradictory execution truth
- closing Phase 6 would require treating all-agent fanout, naive transcript replay, hidden continuation, or report-only tracking as if they were implemented correctly

## Completion Decision
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 6 Wave 6 packet for acceptance mapping, validation reruns, critique, tightly bounded remediation if needed, and the final Phase 6 exit-gate decision over the implemented run/session, persona, moderated-round, bounded-memory, and token or duration tracking surfaces.
- Phase 6 closure is not authorized at activation time and must not be overclaimed until this packet proves `AC-041` through `AC-059` against the live repo truth.

## Completion Notes
- Re-ran the full Phase 6 shell, frontend, and engine gate sets cleanly during this exit-gate pass: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall engine/src engine/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check engine`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest engine/tests`.
- Confirmed the live implementation still requires current eligible Preflight truth at run start, allocates a fresh isolated workspace for each run, generates runtime personas from approved seed context with exactly one Moderator, executes Moderator-controlled 3-to-5-speaker rounds, mutates bounded continuity during real execution, and tracks token or duration usage inside the real run path.
- The critique pass reviewed the touched shell, frontend, engine, and execution-state surfaces and found no material contradiction or in-scope defect that required remediation before closure.
- No product-code changes were required during the exit-gate pass; only the allowlisted execution-state files and this packet file were updated to record the validation evidence, critique result, and closure decision.
- Phase 6 now closes honestly because `AC-041` through `AC-059` are fully proven against the live Phase 6 implementation without overclaiming Phase 7 live monitoring fidelity, Phase 8 final-report behavior, Phase 9 cleanup hardening, or hidden continuation behavior.

## Acceptance Mapping
- `AC-041` — PROVEN by the real shell-owned run creation path in `shell/src/run_session_start.rs`, which generates fresh run and session identifiers before execution, together with `run_session_start::tests::start_run_session_with_components_creates_fresh_run_ids_and_workspaces`.
- `AC-042` — PROVEN by `shell/src/run_workspace_allocation.rs`, which allocates a new per-run workspace directory structure for every run, together with `run_workspace_allocation::tests::allocate_fresh_run_workspace_under_creates_isolated_workspace_paths`.
- `AC-043` — PROVEN by the no-reuse rules in `shell/src/run_workspace_boundary.rs`, the `AlreadyExists` rejection in `shell/src/run_workspace_allocation.rs`, and `run_workspace_boundary::tests::run_workspace_boundary_surface_keeps_run_workspaces_isolated_and_non_reusable` plus `run_workspace_allocation::tests::allocate_fresh_run_workspace_under_rejects_reuse_of_existing_workspace_path`.
- `AC-044` — PROVEN by `engine/src/miro_fish_engine/runtime_personas.py`, where `derive_seed_context` and `build_runtime_persona_foundation` construct personas from approved seed context at runtime, together with `engine/tests/test_runtime_personas.py`.
- `AC-045` — PROVEN by `engine/src/miro_fish_engine/moderator_basis.py`, which materializes the Moderator basis with stable label `Moderator`, and by `validate_runtime_persona_foundation` in `engine/src/miro_fish_engine/runtime_personas.py`, together with `engine/tests/test_moderator_basis.py` and `engine/tests/test_runtime_personas.py`.
- `AC-046` — PROVEN by the required favorable, skeptical, and analytical or neutral perspective coverage checks in `engine/src/miro_fish_engine/runtime_personas.py`, together with `engine/tests/test_runtime_personas.py` and the shell truth surface in `shell/src/runtime_persona_contract.rs`.
- `AC-047` — PROVEN by stable persona labels in `engine/src/miro_fish_engine/runtime_personas.py`, speaker-labeled turn archival in `engine/src/miro_fish_engine/transcript_archive.py`, and the finished run-session truth surfaced through `shell/src/run_session_start.rs` and rendered in `app/src/App.tsx`.
- `AC-048` — PROVEN by the active moderated execution loop in `engine/src/miro_fish_engine/moderated_execution.py` and the shell truth surface in `shell/src/run_moderated_rounds.rs`, which declares engine-owned moderation and forbids all-agent fanout.
- `AC-049` — PROVEN by `select_round_speakers` in `engine/src/miro_fish_engine/moderated_execution.py`, which selects only a 3-to-5-speaker subset each round, together with `engine/tests/test_moderated_execution.py` and `engine/tests/test_session_execution.py`, which prove the speaker counts `(3, 4, 5, 3)`.
- `AC-050` — PROVEN by `shell/src/run_moderated_rounds.rs`, which sets `all_agent_fanout_allowed` to false, and by `engine/src/miro_fish_engine/moderated_execution.py`, which never uses all-agent full fanout as the normal round pattern.
- `AC-051` — PROVEN by `build_round_summary` in `engine/src/miro_fish_engine/moderated_execution.py` and `update_bounded_memory_for_round` in `engine/src/miro_fish_engine/bounded_memory.py`, together with `engine/tests/test_bounded_memory.py` and `engine/tests/test_session_execution.py`, which confirm completed rounds produce and retain a latest round summary.
- `AC-052` — PROVEN by `append_speaker_turn` in `engine/src/miro_fish_engine/transcript_archive.py`, which writes speaker-labeled archived turns, together with `engine/tests/test_moderated_execution.py`, which confirms every archived turn has a `speaker_label` and `speaker_perspective`.
- `AC-053` — PROVEN by the real turn-generation loop in `engine/src/miro_fish_engine/moderated_execution.py`, which records one archived turn per selected speaker, together with `engine/tests/test_moderated_execution.py` and `engine/tests/test_session_execution.py`, which verify that actual turn counts, speaker counts, and execution outputs are materialized rather than skipped or faked.
- `AC-054` — PROVEN by the bounded-memory structures in `engine/src/miro_fish_engine/bounded_memory.py`, the transcript-archive separation in `engine/src/miro_fish_engine/transcript_archive.py`, and `engine/tests/test_bounded_memory.py`, which confirms transcript replay is not allowed as prompt context.
- `AC-055` — PROVEN by the seed-grounding memory in `engine/src/miro_fish_engine/bounded_memory.py` and the turn construction in `engine/src/miro_fish_engine/moderated_execution.py`, which use the approved seed excerpt and topic keywords as active grounding during execution.
- `AC-056` — PROVEN by `RoundSummaryMemory` and `update_bounded_memory_for_round` in `engine/src/miro_fish_engine/bounded_memory.py`, together with `engine/tests/test_bounded_memory.py`, which confirms bounded round-summary continuity is maintained across rounds.
- `AC-057` — PROVEN by `ArgumentRiskMemory` and `update_bounded_memory_for_round` in `engine/src/miro_fish_engine/bounded_memory.py`, plus `classify_turn_contributions` in `engine/src/miro_fish_engine/moderated_execution.py`, together with `engine/tests/test_bounded_memory.py`, which confirms bounded argument, objection, and risk continuity is preserved.
- `AC-058` — PROVEN by `engine/src/miro_fish_engine/transcript_archive.py`, which marks the archive as append-only and `prompt_context_source: false`, by `shell/src/run_bounded_memory.rs`, which forbids naive full-transcript replay, and by `engine/tests/test_transcript_archive.py`, which confirms the archive remains a distinct visibility artifact.
- `AC-059` — PROVEN by the fresh-run plus fresh-workspace rules in `shell/src/run_session_start.rs`, `shell/src/run_workspace_boundary.rs`, and `shell/src/run_workspace_allocation.rs`, together with the explicit `cross_run_semantic_leakage_allowed: false` setting in `engine/src/miro_fish_engine/bounded_memory.py` and `engine/tests/test_bounded_memory.py`.

## Critique Pass
- Reviewed the touched shell, frontend, engine, and execution-state files against the packet scope, the frozen Phase 6 blueprint, Document A `AC-041` through `AC-059`, and Document B Phase 6 validation and reopen conditions.
- Confirmed run execution still depends on current eligible Preflight truth and fresh isolated workspace allocation, personas remain runtime-generated with exactly one Moderator, normal rounds remain 3-to-5-speaker subsets, bounded memory remains real and distinct from transcript replay, and token or duration tracking remains structurally integrated into real execution.
- Confirmed no all-agent fanout normal rounds, naive full-transcript replay, hidden continuation, Phase 7 live monitoring fidelity, Phase 8 final-report behavior, or Phase 9 cleanup hardening was introduced.
- No material contradiction, hidden-scope expansion, behavioural regression, or in-scope defect remained after validation and review, so no remediation was required before closing the phase.