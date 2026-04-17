# Task Packet

## Task ID
P06-WS06-T03

## Task Title
Implement Fresh Run Creation, Workspace Allocation, and Seed Execution Ingestion

## Parent Phase
Phase 6 — Simulation Engine Core

## Parent Workstream
WS-06 — Simulation Engine and Persona System

## Objective
Implement the real shell-owned fresh run creation path, fresh isolated workspace allocation, current Preflight-lineage capture, and seed execution-ingestion handoff into the engine boundary without yet claiming runtime persona generation, Moderator logic, moderated rounds, bounded memory, token or duration tracking, or active user-visible run monitoring.

## Why This Exists
`P06-WS06-T02` materialized the Wave 2 contract and truth foundation, but the repo still lacks any real run/session creation, any real workspace allocation, and any real execution-seed handoff path derived from current eligible Preflight truth. Phase 6 cannot progress honestly into runtime persona generation or moderated execution until Wave 3 first materializes the shell-owned start path, fresh-workspace isolation behavior, and approved execution-seed staging while keeping execution start blocked when current eligible Preflight truth is missing or stale.

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
- `execution/phase_06_master_execution_plan.md` remains the authoritative Phase 6 packetization basis.
- Existing runtime, activation, provider, Preflight, lifecycle, trust-boundary, local-interface, engine-supervision, run/session-contract, workspace-boundary, start-lineage, and run-start-readiness surfaces remain available as the required Wave 2 foundation.
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
- `execution/phase_06_master_execution_plan.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/preflight_submission.rs`
- `shell/src/preflight_readiness.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/run_start_lineage.rs`
- `shell/src/run_start_readiness.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/Cargo.toml` if a bounded filesystem or identifier dependency is required for truthful run creation or workspace allocation
- `shell/Cargo.lock` if Cargo dependency resolution changes the lockfile during the same bounded update
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/preflight_submission.rs`
- `shell/src/preflight_readiness.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_workspace_boundary.rs`
- `shell/src/run_start_lineage.rs`
- `shell/src/run_start_readiness.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_workspace_allocation.rs`
- `shell/src/run_seed_ingestion.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` if workspace-isolation, workspace-support-metadata, or execution-seed staging questions change materially
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p06_ws06_t03_implement_fresh_run_creation_workspace_allocation_and_seed_execution_ingestion.md`

## Implementation Scope
In scope:
- real shell-owned run/session creation and start-authorization behavior for the bounded Wave 3 start path
- real fresh isolated workspace allocation in the governed local workspace domain, including no-active-reuse enforcement and truthful failure classification when workspace creation fails
- capture of the current approved Preflight lineage at run start, including approved seed fingerprint, requested plan, effective plan, and provider/model planning context
- execution-seed staging or ingestion handoff into the engine boundary from the current approved Preflight lineage and current seed submission state without bypassing Phase 5
- coarse run-state truth for a newly created run/session, including truthful blocked versus technical-failure versus pending-execution distinctions while active moderated execution remains deferred
- lifecycle, startup, trust-boundary, local-interface, engine-supervision, command, and frontend integration needed to expose the new Wave 3 run/session and workspace truth honestly
- tests for fresh run creation, fresh workspace allocation, no-active-reuse behavior, blocked-start behavior, and execution-seed lineage staging

## Required Behavior to Preserve
- keep run start dependent on current eligible Preflight truth rather than bypassing Phase 5
- keep the Rust/Tauri shell as the sole local authority for run/session identity, workspace allocation, run-start authorization, and execution-seed staging into the engine boundary
- keep the frontend presentation and input only, with no hidden authority over run/session identity, workspace allocation, execution-seed contents, or active run truth
- keep fresh-run and fresh-workspace semantics explicit and forbid cross-run active reuse or hidden continuation
- keep execution-seed staging derived from the approved current Preflight lineage rather than from ad hoc file selection or stale historical state
- keep raw provider secrets and raw activation tokens out of run workspaces, ordinary logs, generic config, and report surfaces
- keep transcript/archive visibility distinct from bounded prompt memory and distinct from the shell-owned start-lineage basis
- preserve the closed Phase 2, Phase 3, Phase 4, and Phase 5 surfaces except where truthful integration is required
- do not overstate fresh run creation or seed staging as proof that runtime persona generation, moderated rounds, monitoring, or final reports already exist

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- runtime persona generation
- Moderator logic
- moderated rounds or active debate execution
- bounded memory implementation beyond the already frozen contract basis
- token or duration tracking behavior
- live monitoring fidelity, interruption handling, final report generation, cleanup hardening, updater behavior, or runtime hydration behavior
- commercial-backend expansion, hosted orchestration behavior, or public API behavior
- hidden persistence outside the governed workspace or support-state boundaries
- crash-resume or hidden continuation behavior

## Deliverables
- `shell/src/run_session_start.rs` exists and materializes the shell-owned fresh run/session creation and start-authorization path
- `shell/src/run_workspace_allocation.rs` exists and materializes fresh isolated workspace allocation and no-active-reuse enforcement
- `shell/src/run_seed_ingestion.rs` exists and materializes execution-seed staging from the current approved Preflight lineage into the run workspace or engine boundary handoff path
- run/session-contract, workspace-boundary, start-lineage, run-start-readiness, lifecycle, startup, trust-boundary, local-interface, and engine-supervision surfaces now derive truthful Wave 3 state from actual local run-start behavior instead of contract-only assumptions
- the frontend exposes truthful bounded run/session and workspace-start truth without implying runtime persona generation or active moderated execution

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm each approved Wave 3 start path produces a fresh run/session identity and a fresh isolated workspace
- confirm run start is blocked truthfully when current eligible Preflight truth is absent, stale, or otherwise not start-eligible
- confirm execution-seed staging comes from the approved current Preflight lineage rather than bypassing Phase 5
- confirm no runtime persona generation, moderated rounds, bounded-memory execution, monitoring, final-report, cleanup, or updater behavior was implemented
- perform a critique pass and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful fresh-run, workspace-allocation, or execution-seed staging behavior cannot be implemented without changing Documents A, B, or C
- the current Preflight surfaces do not expose enough information to stage execution seed truthfully at run start
- workspace allocation or execution-seed staging would require hidden persistence, crash-resume support, or cross-run reuse that the frozen Phase 6 basis forbids
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 6 Wave 3 packet for fresh run creation, fresh isolated workspace allocation, current Preflight-lineage capture, and execution-seed staging.
- Repo changes were authorized only within the scope of this packet until closeout or blockage.
- Runtime persona generation, Moderator logic, moderated rounds, bounded memory execution, token or duration tracking, and live monitoring remained explicitly out of scope during this packet.

## Completion Notes
- Added `shell/src/run_session_start.rs`, `shell/src/run_workspace_allocation.rs`, and `shell/src/run_seed_ingestion.rs` as the bounded Phase 6 Wave 3 shell-owned foundation for fresh run/session creation, fresh isolated workspace allocation, current Preflight-lineage capture, and execution-seed staging.
- Updated `shell/src/run_session_contract.rs`, `shell/src/run_workspace_boundary.rs`, `shell/src/run_start_readiness.rs`, `shell/src/local_interface_contract.rs`, `shell/src/engine_supervision.rs`, `shell/src/main.rs`, `shell/src/app_lifecycle.rs`, `shell/src/startup_checks.rs`, and `shell/src/preflight_submission.rs` so the Wave 2 contract surfaces now reflect the real Wave 3 fresh-run path and the shell exposes `start_run_session` plus `run_session_state` truthfully.
- Updated `app/src/App.tsx`, `app/src/App.test.tsx`, and `app/src/App.wave4.test.tsx` so the frontend now renders shell-owned run-start readiness, latest run/session truth, and pending-execution creation outcomes without implying runtime persona generation or active moderated execution.
- Product-code changes remained confined to the allowlisted shell and frontend files for this packet; closeout updates remained confined to the allowlisted execution-state files. No git repository is present in the workspace, so this boundary was verified directly from the applied patch set.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`.

## Critique Pass
- Reviewed the touched shell modules, frontend files, and execution-state files against the packet scope, the frozen Phase 6 blueprint, and the upstream Document A and Document B Phase 6 requirements.
- Confirmed the implemented surfaces preserve shell-only run/session identity and workspace authority, preserve current-eligible-Preflight dependency, keep execution-seed staging derived from the approved current Preflight lineage, and avoid overclaiming runtime persona generation, Moderator behavior, moderated execution, bounded-memory execution, monitoring, finalization, or cleanup behavior.
- No material contradiction, hidden-scope expansion, or in-scope defect remained after validation and review, so no remediation was required before marking the packet complete.