# Task Packet

## Task ID
P02-WS02-T02

## Task Title
Implement Startup Checks and Runtime Presence Detection Scaffold

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-02 — Desktop Shell and Runtime Management

## Objective
Implement the bounded shell-owned startup-check path and runtime presence detection scaffold without implementing product behavior.

## Why This Exists
Phase 2 requires a truthful startup-check path and runtime presence scaffold before later hydration, integrity, and supervision work can proceed safely.

## Source-of-Truth References
- Document A Section 5 — Installation and First Launch Runtime Hydration journey
- Document A Section 7 — State Model and Lifecycle Rules
- Document A Section 8 — System Architecture and Responsibility Boundaries
- Document A Section 12 — UI truthfulness for hydration and readiness
- Document A Section 13 — Non-Functional Requirements
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 6.4 — Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Dependency Preconditions
- `execution/task_packets/p02_ws02_t01_establish_desktop_shell_startup_scaffold_and_layer_boundary.md` is complete enough that the shell/app bootstrap exists
- Rust/Tauri, React + TypeScript, Python, and the baseline test runners remain the frozen backbone choices
- no conflicting implementation files exist that would silently widen scope

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/current_phase.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`

## Files / Modules Expected to Change
Create or update only:
- `shell/Cargo.toml`
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/runtime_presence.rs`
- `app/src/App.tsx`
- `execution/current_phase.md`
- `execution/current_repo_truth.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md` only if a real local governance decision becomes necessary

## Implementation Scope
In scope:
- shell-owned startup-check function boundaries
- shell-owned runtime presence detection scaffold boundaries
- placeholder shell command exposure consistent with scaffold-only truth
- placeholder frontend text reflecting scaffold truth only
- execution-state updates needed to keep packetization honest

## Required Behavior to Preserve
- shell owns privileged local orchestration
- frontend remains presentation/input only
- Python engine remains unimplemented
- no hidden persistence
- no hidden service surfaces
- no fake readiness or completion claims

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- runtime hydration logic
- runtime download logic
- runtime integrity enforcement
- engine subprocess orchestration
- activation or licensing
- provider configuration
- Preflight
- simulation behavior
- event streaming
- report generation
- transcript handling
- cleanup or janitor logic
- updater/distribution behavior
- database or persistent storage
- OpenAPI or public API surfaces
- Python files

Do not create:
- extra shell files beyond the exact allowed list
- extra frontend files
- extra service modules
- hidden config or persistence helpers

## Deliverables
- startup-check shell scaffold exists
- runtime presence shell scaffold exists
- frontend placeholder text reflects scaffold truth honestly
- execution state reflects T01 closure and T02 assignment/execution honestly

## Validation / Acceptance Proof Required
- show exact files created/updated
- confirm only the allowed implementation files changed
- confirm no hydration, integrity, engine supervision, activation, provider, Preflight, simulation, reporting, cleanup, updater, database, or OpenAPI behavior was implemented
- confirm runtime readiness is not claimed
- confirm frontend remains presentation-only

## Escalation Triggers
Escalate immediately if:
- a new file is required
- runtime presence scaffolding would require real hydration or integrity behavior
- a shell/frontend boundary cannot be kept explicit without extra modules
- any implementation step would imply hidden persistence or hidden service surfaces

## Completion Decision
COMPLETE — 2026-04-14

Completion notes:
- shell-owned startup-check scaffold exists
- shell-owned runtime presence scaffold exists
- frontend placeholder text reflects scaffold truth without implying runtime readiness
- no out-of-scope implementation was added
