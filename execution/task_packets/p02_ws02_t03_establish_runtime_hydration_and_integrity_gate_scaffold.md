# Task Packet

## Task ID
P02-WS02-T03

## Task Title
Establish Runtime Hydration and Integrity Gate Scaffold

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-02 — Desktop Shell and Runtime Management

## Objective
Extend the shell-owned startup scaffold so runtime hydration and runtime integrity gate paths are represented truthfully without implementing real runtime acquisition, installation, repair, or verification behavior.

## Why This Exists
Phase 2 requires the app to distinguish runtime-ready from runtime-not-ready conditions and to keep execution blocked until runtime integrity is acceptable. The current repo has startup-check and runtime-presence scaffolds only, so the hydration and integrity gate path must be scaffolded next.

## Source-of-Truth References
- Document A Section 5.4 — Installation and First Launch Runtime Hydration Journey
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 13 — Non-Functional Requirements
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 6.4 — Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Dependency Preconditions
- `execution/task_packets/p02_ws02_t02_implement_startup_checks_and_runtime_presence_detection_scaffold.md` is complete enough that shell-owned startup checks and runtime presence scaffolds already exist
- `execution/task_packets/p02_ws12_t02_establish_frontend_enterprise_gate_baseline.md` is complete enough that frontend lint/typecheck/test/build gates already exist
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
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/runtime_hydration.rs`
- `shell/src/runtime_integrity.rs`
- `app/src/App.tsx`
- `execution/current_phase.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md` only if a real local governance decision becomes necessary
- `execution/task_packets/p02_ws02_t03_establish_runtime_hydration_and_integrity_gate_scaffold.md`

## Implementation Scope
In scope:
- shell-owned runtime hydration scaffold module
- shell-owned runtime integrity gate scaffold module
- startup-check scaffold integration with those new shell-owned structures
- placeholder shell command exposure consistent with scaffold-only truth
- truthful frontend placeholder text reflecting hydration and integrity gate scaffold status only
- execution-state repairs and updates needed to keep packetization honest

## Required Behavior to Preserve
- shell owns privileged local orchestration
- frontend remains presentation/input only
- Python engine remains unimplemented
- no hidden persistence
- no hidden service surfaces
- no fake readiness or completion claims

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- runtime download behavior
- runtime extraction or installation behavior
- runtime repair behavior
- runtime checksum or signature verification behavior
- activation or licensing behavior
- provider configuration behavior
- Preflight, simulation, reporting, cleanup, updater, database, or API behavior
- engine subprocess orchestration
- product-feature UI behavior

## Deliverables
- shell-owned runtime hydration scaffold exists
- shell-owned runtime integrity gate scaffold exists
- startup-check scaffold truthfully references hydration and integrity gate scaffolds
- frontend placeholder text reflects hydration and integrity gate scaffold truth honestly
- execution state records the packet and any execution-state repair honestly

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm frontend gates remain clean if frontend files changed
- confirm no runtime download/install/repair/verification behavior was implemented
- confirm runtime readiness is still not claimed

## Escalation Triggers
Escalate immediately if:
- a new shell file beyond the allowed list becomes necessary
- the scaffold would require real hydration, integrity, or repair behavior to stay coherent
- the shell/frontend boundary cannot be kept explicit without expanding scope
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

Completion notes:
- shell-owned runtime hydration scaffold exists
- shell-owned runtime integrity gate scaffold exists
- startup-check scaffold now includes hydration and integrity gate scaffold truth
- frontend placeholder text reflects hydration and integrity gate scaffold truth without implying runtime readiness
- `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` all completed cleanly