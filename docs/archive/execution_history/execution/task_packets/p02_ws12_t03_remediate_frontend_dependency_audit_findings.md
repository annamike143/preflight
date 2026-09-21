# Task Packet

## Task ID
P02-WS12-T03

## Task Title
Remediate Frontend Dependency Audit Findings

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-12 — AI-Agent Task Packaging and Prompt Control

## Objective
Remediate the current frontend dependency audit findings as far as is safely possible without expanding product scope or silently forcing architecture/toolchain drift.

## Why This Exists
The live frontend dependency graph currently reports 5 moderate `npm audit` findings centered in the Vite/Vitest chain, and the hardened SOP now requires this remediation to happen in its own bounded packet before more implementation work proceeds.

## Source-of-Truth References
- Document A Section 13 — Non-Functional Requirements
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.15 — WS-12 — AI-Agent Task Packaging and Prompt Control
- Document B Section 6.4 — Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Dependency Preconditions
- `execution/task_packets/p02_ws12_t02_establish_frontend_enterprise_gate_baseline.md` is complete enough that frontend lint/typecheck/test/build gates already exist
- Node.js and npm are available locally
- the current frontend manifest and lockfile are readable and auditable

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
- `app/package-lock.json`
- `app/eslint.config.js`
- the current `npm audit` results

## Files / Modules Expected to Change
Create or update only:
- `app/package.json`
- `app/package-lock.json`
- `app/eslint.config.js` only if required to preserve the frontend gate baseline after dependency changes
- `app/vite.config.ts` only if required to preserve the frontend gate baseline after dependency changes
- `app/src/main.tsx` only if required to preserve the frontend gate baseline after dependency changes
- `app/src/App.tsx` only if required to preserve the frontend gate baseline after dependency changes
- `execution/current_phase.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md` only if a real local governance decision becomes necessary
- `execution/task_packets/p02_ws12_t03_remediate_frontend_dependency_audit_findings.md`

## Implementation Scope
In scope:
- frontend dependency review and targeted remediation
- safe package updates or pinning changes required to reduce or eliminate the current audit findings
- minimal config/code adjustments required to keep the frontend gate baseline clean after dependency changes
- rerunning the frontend audit, lint, typecheck, test, and build gates
- truthful execution-state updates needed to close or escalate the packet honestly

## Required Behavior to Preserve
- frontend remains presentation/input only
- no product-feature implementation occurs
- no shell-owned privileged behavior moves into the frontend
- no hidden persistence, hidden services, or architecture drift is introduced
- no fake audit pass is claimed

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- new product UI workflows
- router or state-library expansion beyond what is strictly required for safe audit remediation
- shell, engine, activation, provider, Preflight, simulation, reporting, cleanup, updater, database, or API behavior
- silent major-version upgrades that cannot be validated cleanly within this packet
- Document A/B/C changes or generic status-file creation

## Deliverables
- frontend audit findings are reduced or eliminated safely within packet scope, or an explicit escalation explains why not
- frontend lint/typecheck/test/build gates remain clean after any remediation
- execution state records the audit-remediation outcome honestly

## Validation / Acceptance Proof Required
- show exact files created and updated
- show the exact `npm audit` result after remediation work
- show the exact frontend gate commands executed
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` complete with zero warnings and zero errors
- confirm no product-scope behavior was introduced

## Escalation Triggers
Escalate immediately if:
- the available audit fixes require unsafe or unbounded major-version upgrades
- audit remediation would require broader frontend architecture decisions beyond safe dependency hygiene
- any frontend gate reports warnings or errors after in-scope fixes
- the audit findings cannot be reduced or explained honestly within packet scope

## Completion Decision
COMPLETE — 2026-04-15

Completion notes:
- upgraded the audited Vite/Vitest toolchain line to versions that clear the current `npm audit` report
- preserved the clean frontend gate baseline after remediation
- no product-scope behavior was introduced
