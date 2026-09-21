# Task Packet

## Task ID
P02-WS02-T06

## Task Title
Establish Runtime Readiness and Shell Lifecycle Truth Surface

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-02 — Desktop Shell and Runtime Management

## Objective
Extend the shell-owned foundation so the app lifecycle state surface and runtime-readiness truth surface report canonical startup state names and explicit not-ready reasons without implementing janitor behavior, activation, configuration, engine execution, or simulation features.

## Why This Exists
Phase 2 still needs a working shell lifecycle scaffold and a truthful distinction between runtime-ready and runtime-not-ready conditions. The current repo has multiple shell-owned scaffold modules, but it does not yet expose a canonical app-lifecycle truth surface or an explicit runtime-not-ready reason surface.

## Source-of-Truth References
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 10 — Interface and Local Trust Boundary Rules
- Document A Section 13 — Non-Functional Requirements
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 6.4 — Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Dependency Preconditions
- `execution/task_packets/p02_ws02_t05_establish_controlled_shutdown_and_local_interface_contract_scaffold.md` is complete enough that controlled-shutdown and local-interface-contract scaffolds already exist
- `execution/task_packets/p02_ws02_t04_establish_local_app_session_trust_boundary_and_engine_supervision_scaffold.md` is complete enough that trust-boundary and engine-supervision scaffolds already exist
- Rust/Cargo/Clippy tooling is available locally for shell validation

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/current_phase.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/open_questions.md`
- `execution/handoffs/latest_handoff.md`
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/runtime_presence.rs`
- `shell/src/runtime_hydration.rs`
- `shell/src/runtime_integrity.rs`
- `shell/src/engine_supervision.rs`
- `shell/src/controlled_shutdown.rs`
- `shell/src/local_interface_contract.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/runtime_readiness.rs`
- `app/src/App.tsx`
- `execution/current_phase.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md` only if a real local governance decision becomes necessary
- `execution/task_packets/p02_ws02_t06_establish_runtime_readiness_and_shell_lifecycle_truth_surface.md`

## Implementation Scope
In scope:
- shell-owned app-lifecycle truth-surface module using canonical lifecycle state names
- shell-owned runtime-readiness truth-surface module that reports explicit not-ready reasons
- startup-check scaffold integration with those new truth surfaces
- placeholder shell command exposure consistent with scaffold-only truth
- truthful frontend placeholder text reflecting lifecycle and runtime-readiness truth only
- execution-state updates needed to keep packetization honest once this packet is activated and later completed

## Required Behavior to Preserve
- shell owns privileged local orchestration
- frontend remains presentation/input only
- Python engine remains unimplemented
- no hidden persistence
- no hidden service surfaces
- no fake readiness, lifecycle, or completion claims

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- startup janitor behavior
- real activation or licensing behavior
- real provider configuration behavior
- engine spawn, restart, shutdown, or monitoring behavior
- runtime download/install/repair/verification behavior
- simulation execution or run lifecycle behavior
- local HTTP request handling or authenticated request flow behavior
- product-feature UI behavior

## Deliverables
- shell-owned app-lifecycle truth surface exists
- shell-owned runtime-readiness truth surface exists
- startup-check scaffold truthfully references those new surfaces
- frontend placeholder text reflects lifecycle and runtime-not-ready truth honestly
- execution state records the packet honestly once activated and completed

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm frontend gates remain clean if frontend files change
- confirm no janitor, activation, configuration, engine, request-handling, or simulation behavior was implemented
- confirm runtime-ready is not claimed unless the current scaffold state actually supports it

## Escalation Triggers
Escalate immediately if:
- a new shell file beyond the allowed list becomes necessary
- the truth surface would require real activation, configuration, or engine behavior to stay coherent
- the shell/frontend boundary cannot be kept explicit without expanding scope
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Added shell-owned `app_lifecycle` and `runtime_readiness` truth-surface modules.
- Integrated both truth surfaces into `shell/src/startup_checks.rs` and exposed truth-surface note commands through `shell/src/main.rs`.
- Updated `app/src/App.tsx` placeholder text so the frontend truthfully states the current lifecycle surface and the explicit runtime-not-ready posture.
- Verified `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` all complete cleanly.