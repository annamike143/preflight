# Task Packet

## Task ID
P02-WS02-T04

## Task Title
Establish Local App-Session Trust Boundary and Engine Supervision Scaffold

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-02 — Desktop Shell and Runtime Management

## Objective
Extend the shell-owned foundation so the local app-session trust boundary and engine supervision path are represented truthfully without implementing real local authorization tokens, backend calls, or engine process orchestration behavior.

## Why This Exists
Phase 2 still requires the local app-session trust boundary concept and implementation path plus local engine process supervision structure. Those are the next meaningful shell-owned foundations still missing after the runtime hydration and integrity scaffolds.

## Source-of-Truth References
- Document A Section 8 — System Architecture
- Document A Section 10 — Interface and Local Trust Boundary Rules
- Document A Section 13 — Non-Functional Requirements
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.5 — WS-02 — Desktop Shell and Runtime Management
- Document B Section 6.4 — Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Dependency Preconditions
- `execution/task_packets/p02_ws02_t03_establish_runtime_hydration_and_integrity_gate_scaffold.md` is complete enough that startup checks, runtime presence, runtime hydration, and runtime integrity scaffolds already exist
- `execution/task_packets/p02_ws12_t03_remediate_frontend_dependency_audit_findings.md` is complete enough that frontend audit remediation no longer blocks further implementation work
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
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/engine_supervision.rs`
- `app/src/App.tsx`
- `execution/current_phase.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md` only if a real local governance decision becomes necessary
- `execution/task_packets/p02_ws02_t04_establish_local_app_session_trust_boundary_and_engine_supervision_scaffold.md`

## Implementation Scope
In scope:
- shell-owned local app-session trust-boundary scaffold module
- shell-owned engine supervision scaffold module
- startup-check scaffold integration with those new structures
- placeholder shell command exposure consistent with scaffold-only truth
- truthful frontend placeholder text reflecting trust-boundary and engine-supervision scaffold status only
- execution-state updates needed to keep packetization honest once this packet is activated and later completed

## Required Behavior to Preserve
- shell owns privileged local orchestration
- frontend remains presentation/input only
- Python engine remains unimplemented
- no hidden persistence
- no hidden service surfaces
- no fake readiness, trust, or supervision claims

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- real local app-session tokens or authorization exchange behavior
- real localhost API calls or backend request handling
- engine spawn, restart, shutdown, or monitoring behavior
- runtime download/install/repair/verification behavior
- activation, provider, Preflight, simulation, reporting, cleanup, updater, database, or API behavior
- product-feature UI behavior

## Deliverables
- shell-owned local app-session trust-boundary scaffold exists
- shell-owned engine supervision scaffold exists
- startup-check scaffold truthfully references those new shell-owned structures
- frontend placeholder text reflects trust-boundary and engine-supervision scaffold truth honestly
- execution state records the packet honestly once activated and completed

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm frontend gates remain clean if frontend files change
- confirm no local token, backend-call, or engine-process behavior was implemented
- confirm runtime readiness is still not claimed

## Escalation Triggers
Escalate immediately if:
- a new shell file beyond the allowed list becomes necessary
- the scaffold would require real local authorization or engine process behavior to stay coherent
- the shell/frontend boundary cannot be kept explicit without expanding scope
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Added shell-owned `app_session_trust_boundary` and `engine_supervision` scaffold modules.
- Integrated both scaffold snapshots into `shell/src/startup_checks.rs` and exposed scaffold-note commands through `shell/src/main.rs`.
- Updated `app/src/App.tsx` placeholder text so the frontend truthfully states that localhost is not inherently trusted and engine supervision remains scaffold-only.
- Verified `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` all complete cleanly.