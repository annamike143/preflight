# Task Packet

## Task ID
P05-WS05-T02

## Task Title
Establish Seed Intake Contracts, Preflight Outcome Classes, and Run-Eligibility Truth Surfaces

## Parent Phase
Phase 5 — Seed Intake and Preflight

## Parent Workstream
WS-05 — Seed Intake and Preflight

## Objective
Define the code-level shell-owned seed-intake and Preflight contracts, Preflight outcome and invalidation truth surfaces, requested-versus-effective plan basis, and run-start-eligibility truth surfaces needed for later Phase 5 implementation, without claiming real document validation, language detection, provider reachability, model availability, or estimation behavior yet.

## Why This Exists
`P05-WS05-T01` froze the bounded seed-intake and Preflight path plus the Phase 5 wave plan, but the repo still lacks explicit Preflight contract modules, outcome classes, invalidation semantics, and run-start-gating truth surfaces. Phase 5 Waves 3 and 4 cannot implement deterministic document validation and run-eligibility behavior honestly until Wave 2 first defines the seed-intake request boundary, Preflight result vocabulary, requested-versus-effective plan structure, invalidation rules, and shell-owned truth exposure surfaces in code.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
- Document A Section 5 — User Journeys and Interaction Flows
- Document A Section 6 — Product Behavior Specification
- Document A Section 7 — State Machines
- Document A Section 10 — API and Interface Contract
- Document A Section 12 — UI/UX Contract
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.8 — WS-05 — Seed Intake and Preflight
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.8 — Phase 5 — Seed Intake and Preflight
- Document B Section 6.7 — Phase 5 — Seed Intake and Preflight
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- `execution/phase_05_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p05_ws05_t01_establish_phase_5_seed_intake_and_preflight_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/phase_05_master_execution_plan.md` remains the authoritative Phase 5 packetization basis.
- Existing shell lifecycle, startup, trust-boundary, local-interface, activation, and provider surfaces remain available as the nearest repo-local pattern for contract and readiness work.
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
- `execution/phase_05_master_execution_plan.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/activation_readiness.rs`
- `shell/src/provider_readiness.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/preflight_contract.rs`
- `shell/src/preflight_readiness.rs`
- `app/src/App.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved Preflight result support-state narrowing changes materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p05_ws05_t02_establish_seed_intake_contracts_preflight_outcome_classes_and_run_eligibility_truth_surfaces.md`

## Implementation Scope
In scope:
- shell-owned seed-intake and Preflight contract types for one seed document, bounded requested run inputs, Preflight outcome classes, rejection versus technical-failure distinction, and stale invalidation semantics
- requested-plan versus effective-plan contract basis strong enough for later truthful downgrade disclosure
- Preflight readiness and run-start-eligibility truth surface that remains separate from runtime readiness, activation readiness, provider readiness, and later run execution
- startup, lifecycle, trust-boundary, local-interface, command, and frontend integration needed to expose the new seed-intake and Preflight truth surfaces honestly
- tests for the new contract, readiness, and integration surfaces

## Required Behavior to Preserve
- keep Preflight eligibility separate from runtime readiness, activation readiness, provider readiness, and simulation execution
- keep the Rust/Tauri shell as the sole local authority for run-start gating and Preflight truth surfaces
- keep the frontend presentation and input only, with no hidden authority over run-start eligibility
- keep seed selection separate from Preflight approval
- keep approved, approved-with-downgrade, rejected, technical failure, and stale invalidation distinct
- preserve the closed Phase 3 activation surfaces and closed Phase 4 provider surfaces except where read-only integration is required
- do not overstate contract or readiness existence as proof that real document validation, provider reachability, or estimation already exists

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- real document parsing or extractable-text validation behavior
- language-detection behavior
- provider reachability or model availability checks
- cost or duration estimation behavior
- real Preflight execution or persistence behavior
- run/session creation or active execution behavior
- simulation, reporting, cleanup, updater, or runtime hydration behavior
- commercial-backend expansion, cloud document storage, or public API behavior

## Deliverables
- `shell/src/preflight_contract.rs` exists and defines the bounded seed-intake and Preflight contract surface
- `shell/src/preflight_readiness.rs` exists and defines Preflight state, invalidation, and run-start-eligibility truth separately from runtime, activation, and provider readiness
- shell lifecycle, startup, trust-boundary, local-interface, and command surfaces expose the new Preflight truth honestly
- the frontend, if touched, reports the bounded seed-intake and Preflight truth without implying real document validation or run-start implementation

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm no real seed-intake validation, Preflight execution, run start, or simulation behavior was implemented
- confirm run-start-gating truth does not bypass Preflight and does not collapse rejection into technical failure
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful Preflight contract or readiness surfaces cannot be defined without changing Documents A, B, or C
- the packet would require real document validation, estimation, or provider reachability behavior to remain coherent
- lifecycle or startup integration would collapse Preflight eligibility into runtime readiness, activation readiness, provider readiness, or active execution truth
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Current State
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 5 Wave 2 packet for seed-intake contracts, Preflight outcome classes, invalidation semantics, and run-start-eligibility truth surfaces.
- Repo changes are authorized only within the scope of this packet until closeout or blockage.
- Real seed-intake validation, Preflight execution, run/session creation, and simulation behavior remain explicitly out of scope during this packet.

## Completion Notes
- Implemented `shell/src/preflight_contract.rs` and `shell/src/preflight_readiness.rs` as the bounded Phase 5 Wave 2 foundation for one-seed intake contract truth, canonical Preflight outcome states, stale invalidation, requested-versus-effective plan basis, and run-start eligibility truth.
- Integrated the new Preflight truth surfaces into `shell/src/main.rs`, `shell/src/app_lifecycle.rs`, `shell/src/app_session_trust_boundary.rs`, `shell/src/startup_checks.rs`, and `shell/src/local_interface_contract.rs` without collapsing Preflight into runtime readiness, activation readiness, provider readiness, or run execution.
- Updated `app/src/App.tsx` to render a bounded shell-owned Preflight readiness panel without implying real document validation, language detection, provider/model reachability checks, or run-start implementation.
- Product-code changes remained confined to the allowlisted shell and frontend files for this packet; closeout updates remained confined to the allowlisted execution-state files. No git repository is present in the workspace, so this boundary was verified directly from the applied patch set.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`. The frontend test command exited cleanly under `--passWithNoTests` because no frontend test files existed yet at that closeout.
- Structural consistency review and contradiction check were completed across the touched execution files before closeout. No material contradictions remain within the packet scope.