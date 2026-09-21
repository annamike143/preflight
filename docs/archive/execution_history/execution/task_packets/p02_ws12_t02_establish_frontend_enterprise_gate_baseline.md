# Task Packet

## Task ID
P02-WS12-T02

## Task Title
Establish Frontend Enterprise Gate Baseline

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-12 — AI-Agent Task Packaging and Prompt Control

## Objective
Establish the minimum frontend dependency, lint, type-check, build, and test baseline required for future frontend-touching packets to satisfy the zero-warning enterprise gate honestly.

## Why This Exists
The hardened execution SOP now forbids packet closure when applicable validation gates are missing or noisy. The frontend currently lacks a lint gate and initially showed React/JSX resolution diagnostics in the local environment, so future frontend packets would otherwise be blocked.

## Source-of-Truth References
- Document A Section 8 — System Architecture
- Document A Section 13 — Non-Functional Requirements
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.15 — WS-12 — AI-Agent Task Packaging and Prompt Control
- Document B Section 6.4 — Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Dependency Preconditions
- `execution/README.md` defines the zero-warning enterprise gate and mandatory critique pass
- Phase 2 is active
- frontend bootstrap files already exist under `app/`
- Node.js and npm are available locally

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/current_phase.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/open_questions.md`
- `execution/handoffs/latest_handoff.md`
- `app/package.json`
- `app/tsconfig.json`
- `app/vite.config.ts`
- `app/src/main.tsx`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `app/package.json`
- `app/package-lock.json`
- `app/eslint.config.js`
- `app/src/App.tsx` only if required to satisfy the applicable gates without altering product truth
- `app/src/main.tsx` only if required to satisfy the applicable gates without altering product truth
- `app/vite.config.ts` only if required to satisfy the applicable gates without altering product truth
- `execution/current_phase.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md` only if a real local governance decision becomes necessary
- `execution/task_packets/p02_ws12_t02_establish_frontend_enterprise_gate_baseline.md`

## Implementation Scope
In scope:
- frontend dependency installation required for existing React + TypeScript bootstrap truth
- frontend lint configuration and script establishment
- explicit frontend type-check gate establishment
- running the applicable frontend build, lint, type-check, and test gates
- minimal code/config fixes needed to clear those gates without changing product behavior
- execution-state updates required to close the packet honestly

## Required Behavior to Preserve
- frontend remains presentation/input only
- placeholder startup/trust-boundary text remains truthful
- no product-feature implementation occurs
- no hidden persistence, hidden services, or architecture drift is introduced
- no fake gate pass is claimed

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- new product UI workflows
- shell-owned privileged behavior in the frontend
- runtime hydration, integrity, activation, provider, Preflight, simulation, reporting, cleanup, updater, or API behavior
- router, state-library, or architecture expansion beyond what is strictly required for gate establishment
- a generic status file or any Document A/B/C modifications

## Deliverables
- frontend lint script exists
- frontend dependency/type-resolution state is clean enough to run the applicable gates
- frontend build, lint, type-check, and test gates run without errors or warnings
- execution state records the new gate baseline truthfully

## Validation / Acceptance Proof Required
- show exact files created and updated
- show the exact frontend gate commands executed
- confirm the applicable frontend gates complete with zero errors and zero warnings
- confirm no product-scope behavior was introduced
- confirm frontend remains presentation-only

## Escalation Triggers
Escalate immediately if:
- the packet would require feature behavior to clear a gate
- the local environment cannot install or resolve the required frontend dependencies truthfully
- the packet would require broader frontend architecture decisions beyond gate establishment
- any gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

Completion notes:
- frontend dependency baseline is installed locally and `app/package-lock.json` exists
- frontend lint and typecheck scripts now exist in `app/package.json`
- `app/eslint.config.js` establishes the frontend lint gate baseline
- the gate commands `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` all complete cleanly for the current placeholder frontend
- no product-scope behavior was introduced