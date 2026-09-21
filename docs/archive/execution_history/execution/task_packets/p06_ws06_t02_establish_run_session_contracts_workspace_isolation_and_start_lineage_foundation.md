# Task Packet

## Task ID
P06-WS06-T02

## Task Title
Establish Run/Session Contracts, Workspace Isolation, and Start-Lineage Foundation

## Parent Phase
Phase 6 — Simulation Engine Core

## Parent Workstream
WS-06 — Simulation Engine and Persona System

## Objective
Define the code-level shell-owned run/session contracts, workspace-isolation boundary surfaces, Preflight-lineage start basis, shell-engine start-boundary truth, and coarse run-state truth surfaces needed for later Phase 6 implementation, without claiming real run/session creation, workspace allocation, seed execution ingestion, persona generation, moderated rounds, or active run execution yet.

## Why This Exists
`P06-WS06-T01` froze the bounded Phase 6 run/session path and wave order, but the repo still lacks explicit run/session contract modules, workspace-isolation boundary surfaces, Preflight-lineage start semantics, and shell-owned run-start truth surfaces. Phase 6 Waves 3 through 5 cannot implement real fresh-run creation, workspace allocation, seed execution ingestion, or moderated execution honestly until Wave 2 first defines the request boundary, run identity vocabulary, workspace-isolation rules, start-lineage snapshot semantics, and shell-owned truth exposure surfaces in code.

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
- `execution/phase_06_master_execution_plan.md` remains the authoritative Phase 6 packetization basis.
- Existing shell lifecycle, startup, trust-boundary, local-interface, engine-supervision, provider, activation, and Preflight surfaces remain available as the nearest repo-local pattern for contract and start-foundation work.
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
- `shell/src/preflight_readiness.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
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
- `app/src/App.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved run-support metadata or workspace-isolation questions change materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p06_ws06_t02_establish_run_session_contracts_workspace_isolation_and_start_lineage_foundation.md`

## Implementation Scope
In scope:
- shell-owned run/session start contract types for start requests, start outcomes, run/session identifiers, coarse run-state classes, and truthful not-started versus start-blocked versus technical-failure distinctions
- shell-owned workspace-isolation boundary surface defining fresh isolated workspace requirements, no-active-reuse semantics, bounded run-support metadata categories, and forbidden hidden continuation or crash-resume behavior without yet materializing real allocation code
- shell-owned Preflight-lineage start contract basis for carrying approved seed fingerprinting and effective-plan context into future run execution without bypassing closed Phase 5 truth
- shell-owned run-start-readiness truth surface that remains separate from runtime readiness, activation readiness, provider readiness, Preflight eligibility, and later active-execution truth while still depending on them
- startup, lifecycle, trust-boundary, local-interface, engine-supervision, command, and frontend integration needed to expose the new Phase 6 contract or boundary or readiness truth surfaces honestly
- tests for the new contract, boundary, readiness, and integration surfaces

## Required Behavior to Preserve
- keep run start dependent on current eligible Preflight truth rather than bypassing Phase 5
- keep the Rust/Tauri shell as the sole local authority for run/session identity, workspace allocation authority, and engine-start authorization surfaces
- keep the frontend presentation and input only, with no hidden authority over run/session identity, workspace allocation, or active run truth
- keep fresh-run and fresh-workspace semantics explicit and forbid cross-run active reuse or hidden continuation
- keep transcript/archive visibility distinct from bounded active memory and distinct from shell-owned start-lineage contracts
- preserve the closed Phase 2, Phase 3, Phase 4, and Phase 5 surfaces except where truthful read-only integration is required
- do not overstate contract or readiness existence as proof that real run/session creation or active execution already exists

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- real run/session creation or active run-start execution behavior
- real workspace allocation, workspace cleanup, or crash-resume behavior
- seed ingestion for execution behavior
- runtime persona generation, Moderator logic, moderated rounds, bounded memory implementation, or token/duration tracking behavior
- live monitoring fidelity, interruption handling, final report generation, cleanup hardening, updater behavior, or runtime hydration behavior
- commercial-backend expansion, hosted orchestration behavior, or public API behavior
- product-feature UI beyond truthful placeholder or boundary-surface rendering

## Deliverables
- `shell/src/run_session_contract.rs` exists and defines the bounded run/session contract surface
- `shell/src/run_workspace_boundary.rs` exists and defines workspace-isolation and no-active-reuse rules without claiming real allocation behavior
- `shell/src/run_start_lineage.rs` exists and defines the Preflight-lineage basis for future execution start
- `shell/src/run_start_readiness.rs` exists and defines shell-owned run-start foundation truth separately from runtime, activation, provider, Preflight, and active execution truth
- shell lifecycle, startup, trust-boundary, engine-supervision, local-interface, and command surfaces expose the new Phase 6 truth honestly
- the frontend, if touched, reports the bounded run/session and start-foundation truth without implying active run execution

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm no real run/session creation, workspace allocation, seed execution ingestion, persona generation, moderated rounds, or active run execution behavior was implemented
- confirm run-start foundation truth still depends on current eligible Preflight truth and does not imply live monitoring fidelity or final-report truth
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful run/session, workspace-isolation, or start-lineage contract surfaces cannot be defined without changing Documents A, B, or C
- the packet would require real run/session creation, workspace allocation, or seed execution ingestion behavior to remain coherent
- lifecycle, startup, or trust-boundary integration would collapse run-start foundation truth into runtime readiness, activation readiness, provider readiness, Preflight truth, or active execution truth
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 6 Wave 2 packet for run/session contracts, workspace-isolation boundary surfaces, Preflight-lineage start basis, and shell-owned start-foundation truth surfaces.
- Repo changes were authorized only within the scope of this packet until closeout or blockage.
- Real run/session creation, workspace allocation, seed execution ingestion, persona generation, moderated execution, and live monitoring remained explicitly out of scope during this packet.

## Completion Notes
- Added `shell/src/run_session_contract.rs`, `shell/src/run_workspace_boundary.rs`, `shell/src/run_start_lineage.rs`, and `shell/src/run_start_readiness.rs` as the bounded Phase 6 Wave 2 shell-owned foundation for run/session contract truth, workspace isolation, Preflight-derived start lineage, and run-start readiness.
- Integrated the new Wave 2 surfaces into `shell/src/main.rs`, `shell/src/app_lifecycle.rs`, `shell/src/app_session_trust_boundary.rs`, `shell/src/startup_checks.rs`, `shell/src/local_interface_contract.rs`, and `shell/src/engine_supervision.rs` so the shell now exposes these truths without claiming real run/session creation, workspace allocation, or active execution.
- Updated `app/src/App.tsx` to surface the bounded Phase 6 Wave 2 run-start foundation truth additively without adding any new frontend authority over run/session identity, workspace allocation, or active execution.
- Product-code changes remained confined to the allowlisted shell and frontend files for this packet; closeout updates remained confined to the allowlisted execution-state files. No git repository is present in the workspace, so this boundary was verified directly from the applied patch set.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`.

## Critique Pass
- Reviewed the touched shell modules, frontend file, and execution-state files against the packet scope, the frozen Phase 6 blueprint, and the upstream Document A and Document B Phase 6 requirements.
- Confirmed the implemented surfaces preserve shell-only run/session identity and workspace authority, preserve current-eligible-Preflight dependency, keep transcript/archive separation from active prompt memory, and avoid overclaiming run/session creation, workspace allocation, seed execution ingestion, persona generation, moderated execution, monitoring, finalization, or cleanup behavior.
- No material contradiction, hidden-scope expansion, or in-scope defect remained after validation and review, so no remediation was required before marking the packet complete.