# Task Packet

## Task ID
P06-WS06-T04

## Task Title
Implement Runtime Persona Generation, Moderator Basis, and Bounded Memory Foundation

## Parent Phase
Phase 6 — Simulation Engine Core

## Parent Workstream
WS-06 — Simulation Engine and Persona System

## Objective
Implement runtime persona generation from the approved seed-execution context, exactly one Moderator basis, stable persona labeling, required perspective diversity, and bounded seed-grounding plus round-summary plus argument or risk memory foundations inside the local execution path, while keeping full moderated rounds, token or duration tracking, live monitoring fidelity, final-report behavior, and cleanup hardening out of scope.

## Why This Exists
`P06-WS06-T03` materialized the real shell-owned fresh run/session start path, fresh isolated workspace allocation, and approved execution-seed staging, but the repo still cannot truthfully initialize any runtime-generated persona set, Moderator role, or bounded run memory from that approved seed context. Phase 6 Wave 5 cannot claim real moderated round execution honestly until Wave 4 first establishes the engine-side persona-generation and bounded-memory foundations that the frozen Phase 6 plan requires. Because `engine/` remains empty at activation time, this packet also authorizes the bounded engine-package bootstrap needed to materialize those governed behaviors without inventing broader execution, monitoring, or reporting scope.

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
- `execution/phase_06_master_execution_plan.md` remains the authoritative Phase 6 packetization basis.
- Existing runtime, activation, provider, Preflight, lifecycle, trust-boundary, local-interface, engine-supervision, run/session-contract, workspace-boundary, start-lineage, run-start-readiness, fresh-run, workspace-allocation, and execution-seed staging surfaces remain available as the required Wave 2 and Wave 3 foundation.
- `engine/` remains an empty boundary at activation time and may be populated only within this packet's allowlist.
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
- `shell/src/run_start_lineage.rs`
- `shell/src/run_start_readiness.rs`
- `shell/src/run_seed_ingestion.rs`
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
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` if persona-generation or memory-foundation questions change materially
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p06_ws06_t04_implement_runtime_persona_generation_moderator_basis_and_bounded_memory_foundation.md`

## Implementation Scope
In scope:
- bounded engine-package bootstrap for runtime persona generation and bounded-memory foundation only
- runtime-generated persona-set construction from approved seed context already staged by Wave 3 rather than from a static user-authored roster
- exactly one Moderator role, stable persona labels, and favorable plus skeptical plus analytical or neutral perspective coverage basis
- bounded seed-grounding, round-summary continuity, and argument or risk continuity memory structures required before moderated rounds can begin
- transcript-archive versus bounded-memory separation inside the execution workspace so archived turns do not become unlimited default prompt replay
- shell-owned run-session, local-interface, lifecycle, startup, and engine-supervision truth updates needed to expose whether persona or memory foundation initialization now exists without claiming full moderated rounds or live monitoring fidelity
- bounded frontend truth updates only if needed to render the new shell-owned Wave 4 status honestly
- tests for runtime persona invariants, exactly-one-Moderator enforcement, perspective diversity, bounded-memory initialization, transcript-archive separation, and applicable shell plus frontend integration

## Required Behavior to Preserve
- keep run start dependent on current eligible Preflight truth rather than bypassing Phase 5
- keep the Rust/Tauri shell as the sole local authority for run/session identity, workspace allocation, run-start authorization, and coarse supervision truth
- keep the Python engine as the owner of runtime persona generation, Moderator role construction, bounded memory structures, and transcript-archive handling once Wave 4 implementation begins
- keep runtime persona generation derived from approved seed context and current effective-plan basis rather than from any static user-authored persona configuration or prior-run state
- keep exactly one Moderator per run and preserve stable persona labels so later execution remains speaker-attributed within the run
- keep transcript archive structurally separate from bounded prompt memory and forbid naive full-transcript replay as the core memory model
- keep bounded memory grounded in current approved seed context and current-run continuity only, with no cross-run semantic leakage
- keep provider secrets, activation tokens, and hidden continuation state out of engine memory, run transcript artifacts, and ordinary logs
- preserve the completed Wave 3 fresh-run, workspace-allocation, and execution-seed staging behavior while keeping full moderated rounds, token or duration tracking, live monitoring, finalization, export, cleanup hardening, and updater behavior out of scope

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- full moderated round execution or all-agent fanout behavior
- per-round 3-to-5 speaker execution control beyond the bounded foundation needed for later Wave 5 work
- token or duration tracking behavior
- live monitoring fidelity, interruption handling, final report generation, export, cleanup hardening, updater behavior, or runtime hydration behavior
- commercial-backend expansion, hosted orchestration behavior, or public API behavior
- hidden persistence outside the governed run-workspace or support-state boundaries
- crash-resume or hidden continuation behavior
- a static user-authored persona configuration surface
- naive full-transcript replay as a substitute for bounded memory

## Deliverables
- `shell/src/runtime_persona_contract.rs` exists and defines the shell-visible runtime persona and Moderator basis surface truthfully
- `shell/src/run_bounded_memory.rs` exists and defines the bounded memory and transcript-separation surface truthfully
- `engine/` contains a bounded Python engine package that can initialize runtime personas, exactly one Moderator, and bounded memory from approved staged seed context
- shell lifecycle, startup, local-interface, engine-supervision, run-session, and command surfaces now expose truthful Wave 4 foundation state without implying full moderated rounds, token or duration tracking, or live monitoring
- the frontend, if touched, renders only shell-owned Wave 4 foundation truth without gaining authority over persona construction, Moderator decisions, or bounded memory contents

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowlisted files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors when shell code changes are in scope
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` complete with zero warnings and zero errors if frontend files change
- confirm the applicable engine gate family completes cleanly with Python compile validation for `engine/src` and `engine/tests`, `ruff check engine`, and `pytest engine/tests`
- confirm personas are runtime-generated from approved seed context rather than from static user-authored configuration
- confirm exactly one Moderator exists and the required stable-label plus perspective-diversity invariants are enforced
- confirm bounded memory initialization is real, remains grounded in approved seed context, and stays structurally separate from transcript archive replay
- confirm no full moderated rounds, token or duration tracking, live monitoring, final-report, cleanup, updater, or crash-resume behavior was implemented
- perform a critique pass and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful runtime persona generation or bounded-memory foundation cannot be implemented without changing Documents A, B, or C
- engine-package bootstrap would require hidden persistence, hosted orchestration, or public API expansion beyond the frozen Phase 6 basis
- exactly one Moderator or the required perspective-diversity basis cannot be enforced deterministically from approved seed context
- bounded memory would collapse into unlimited transcript replay, prior-run reuse, or hidden semantic leakage
- any applicable shell, frontend, or engine gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 6 Wave 4 packet for runtime persona generation, exactly one Moderator basis, stable persona labeling, perspective diversity, and bounded memory foundation.
- Repo changes are authorized only within the scope of this packet until closeout or blockage.
- Full moderated rounds, token or duration tracking, live monitoring, finalization, export, and cleanup hardening remained explicitly out of scope at activation time.

## Completion Notes
- Added the bounded `engine/` package under `engine/src/miro_fish_engine/` plus `engine/tests/` so runtime personas, exactly one Moderator basis, bounded memory foundation, transcript-archive separation, and workspace bootstrap can now be materialized from the approved staged seed workspace.
- Added `shell/src/runtime_persona_contract.rs` and `shell/src/run_bounded_memory.rs`, and updated the shell lifecycle, trust-boundary, startup, local-interface, supervision, run-start-readiness, run-session, and main-command surfaces so Wave 4 foundation truth is exposed without implying moderated rounds, token or duration tracking, or live monitoring.
- Updated `shell/src/run_session_start.rs` so the shell-owned fresh run/session start path now invokes the engine bootstrap after seed staging and persists runtime persona, Moderator, bounded-memory, and transcript-archive artifact truth inside the run/session state snapshot.
- Updated `app/src/App.tsx`, `app/src/App.test.tsx`, and `app/src/App.wave4.test.tsx` so the frontend now renders the shell-owned Wave 4 foundation section and the expanded run/session truth surface without gaining any authority over persona generation, Moderator decisions, or bounded memory contents.
- Product-code changes remained confined to the allowlisted shell, frontend, and engine files for this packet; closeout updates remained confined to the allowlisted execution-state files. No git repository is present in this workspace, so that boundary was verified directly from the applied patch set and targeted file inspection.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, Python compile validation for `engine/src` and `engine/tests`, `ruff check engine`, and `pytest engine/tests`.

## Critique Pass
- Reviewed the touched shell, frontend, engine, and execution-state files against the packet scope, the frozen Phase 6 blueprint, and the upstream Document A and Document B Phase 6 requirements.
- Confirmed personas are runtime-generated from approved staged seed context rather than from any static user-authored roster, exactly one Moderator is enforced, bounded memory is initialized separately from transcript archive replay, and the shell remains the sole authority for run/session identity, workspace allocation, and start authorization.
- Confirmed no full moderated rounds, per-round speaker execution, token or duration tracking, live monitoring, final-report behavior, cleanup hardening, or hidden continuation behavior was implemented during this packet.
- No material contradiction, hidden-scope expansion, or in-scope defect remained after validation and review, so no remediation was required before marking the packet complete.