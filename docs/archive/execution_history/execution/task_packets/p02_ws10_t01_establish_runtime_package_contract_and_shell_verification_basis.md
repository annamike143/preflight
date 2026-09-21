# Task Packet

## Task ID
P02-WS10-T01

## Task Title
Establish Runtime Package Contract and Shell Verification Basis

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-10 — Updater, Distribution, and Runtime Delivery

## Objective
Freeze the bounded runtime package contract and trust-verification basis the shell needs so runtime presence detection, runtime integrity acceptance, and runtime-ready truth can move beyond scaffold-only states without implicitly deciding the full updater/runtime-delivery toolchain or implementing download/install flow.

## Why This Exists
The T07 closure audit found that Phase 2 cannot close because the repo can truthfully report runtime-not-ready but cannot yet truthfully support a positive runtime-ready path. The missing basis is no longer a generic shell-boundary problem; it is the absence of a concrete runtime package contract and trust-verification basis for shell-owned presence and integrity decisions.

## Source-of-Truth References
- Document A Section 8 — System Architecture
- Document A Section 9.9 — Runtime Package Data
- Document A Section 13 — Non-Functional Requirements
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.5 — WS-02 — Desktop Shell and Runtime Management
- Document B Section 3.13 — WS-10 — Updater, Distribution, and Runtime Delivery
- Document B Section 6.4 — Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Dependency Preconditions
- `execution/task_packets/p02_ws02_t07_execute_phase_2_closure_audit_and_exit_gate_decision.md` is complete enough that the remaining Phase 2 blocker has been identified precisely
- `execution/task_packets/p02_ws02_t06_establish_runtime_readiness_and_shell_lifecycle_truth_surface.md` is complete enough that lifecycle/readiness truth surfaces already exist
- Rust/Cargo/Clippy and frontend validation tooling remain available locally

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/current_phase.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/open_questions.md`
- `execution/handoffs/latest_handoff.md`
- `execution/decisions/decision_log.md`
- `shell/src/main.rs`
- `shell/src/runtime_presence.rs`
- `shell/src/runtime_integrity.rs`
- `shell/src/runtime_readiness.rs`
- `shell/src/startup_checks.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/runtime_package_contract.rs`
- `shell/src/runtime_presence.rs`
- `shell/src/runtime_integrity.rs`
- `shell/src/runtime_readiness.rs`
- `shell/src/startup_checks.rs`
- `app/src/App.tsx`
- `execution/current_phase.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/open_questions.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md` if a real local governance decision is made
- `execution/task_packets/p02_ws10_t01_establish_runtime_package_contract_and_shell_verification_basis.md`

## Implementation Scope
In scope:
- shell-owned runtime package contract module that defines the bounded local basis for presence and integrity decisions
- shell-owned runtime presence surface upgraded from `not-production-detected` to contract-based local determination
- shell-owned runtime integrity surface upgraded from permanently false scaffold values to contract-based local acceptance/rejection basis
- runtime-readiness truth surface updated to consume the upgraded presence/integrity results honestly
- startup-check integration and truthful frontend placeholder updates
- any needed execution-state, open-question, or constraint updates required to keep the packet honest

## Required Behavior to Preserve
- do not implicitly choose the full updater/runtime-delivery toolchain
- do not implement runtime download, install, or updater flow
- keep activation, configuration, and runtime readiness semantically separate
- keep the shell as the explicit local authority
- keep the frontend presentation-only

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- runtime download/distribution flow
- updater behavior
- activation or provider configuration behavior
- engine spawn, simulation execution, or run lifecycle behavior
- public API or local HTTP request handling behavior
- product-feature UI behavior

## Deliverables
- bounded runtime package contract exists for shell presence/integrity decisions
- runtime presence surface is no longer purely decorative
- runtime integrity surface is no longer purely decorative
- runtime-readiness truth can support both runtime-ready and runtime-not-ready outcomes honestly
- execution state records the packet honestly once completed

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm frontend gates remain clean if frontend files change
- confirm no runtime download/install/update behavior was implemented
- confirm the runtime package contract basis does not silently decide the full updater/runtime-delivery toolchain

## Escalation Triggers
Escalate immediately if:
- truthful runtime presence/integrity decisions cannot be bounded without editing Documents A/B/C
- the packet would require implicit choice of updater/runtime-delivery toolchain details that are still open
- the shell/frontend boundary cannot be kept explicit without expanding scope
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Added shell-owned `runtime_package_contract` and integrated it into runtime presence, runtime integrity, runtime readiness, startup checks, and the shell command surface.
- Updated the frontend placeholder truth so it now reports the bounded runtime package contract basis and the current contract-defined runtime-not-ready posture honestly.
- Preserved the shell-owned boundary and did not implement runtime download, install, repair, updater, activation, provider, engine, or simulation behavior.
- Verified `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` all complete cleanly.