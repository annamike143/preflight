# Task Packet

## Task ID
P02-WS12-T04

## Task Title
Collapse Repo-Local Execution OS into Lean Dashboard and Live Log Model

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-12 — AI-Agent Task Packaging and Prompt Control

## Objective
Replace the high-overhead mirrored execution-state routine with a lean live model centered on one dashboard and one append-only execution log before Phase 3 packetization begins, without changing product truth or reopening the closed Phase 2 technical outcomes.

## Why This Exists
The closed-loop Phase 2 work proved the repo-local execution OS was accurate enough but heavier than necessary. The approved next governance step is to simplify the live repo execution model before Phase 3 so future autonomous implementation runs against fewer mirrored status surfaces and larger phase-sized waves.

## Source-of-Truth References
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.14 — WS-12 — AI-Agent Task Packaging and Prompt Control
- Document B Section 5.15 — Inter-Phase Ordering Rules
- Document B Section 6.3 — Phase 1 — Governance, Repository Foundation, and Execution Scaffolding
- `execution/README.md`

## Dependency Preconditions
- Phase 2 is already closed honestly.
- No product-feature packet is currently active.
- The user has approved simplifying the repo-local SOP before Phase 3 activation.

## Inputs / Required Context
- live Documents A, B, and C where execution-governance meaning is implicated
- `execution/README.md`
- `execution/current_phase.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/open_questions.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md`

## Files / Modules Expected to Change
Create or update only:
- `execution/README.md`
- `execution/progress.md`
- `execution/live_execution_log.md`
- `execution/current_phase.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/open_questions.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md`
- `execution/task_packets/p02_ws12_t04_collapse_repo_local_execution_os_into_lean_dashboard_and_live_log_model.md`
- one newly authored Phase 3 successor packet file only if that packet is ready to be activated truthfully

## Implementation Scope
In scope:
- define the lean live execution model around `progress.md` and `live_execution_log.md`
- simplify mandatory read order and update rules
- move `current_phase.md`, handoff, and evidence to legacy compatibility/archive status rather than live mandatory state
- preserve packet discipline, zero-warning gating rules, and critique-pass discipline
- author and activate the first truthful Phase 3 packet if Phase 3 entry posture can be represented honestly after simplification

## Required Behavior to Preserve
- Documents A, B, and C remain authoritative above the repo-local execution OS
- one packet remains the only active execution unit at a time
- critique-pass and zero-warning gate discipline remain intact
- the lean model must not weaken truthfulness or authorize feature drift

## Explicit Non-Goals / Forbidden Changes
Do not:
- change product scope or Document A/B/C meaning
- reopen Phase 2 product implementation truth
- implement licensing, activation, provider, Preflight, simulation, reporting, cleanup, or updater behavior
- reintroduce a parallel status document outside `execution/`

## Deliverables
- `execution/progress.md` exists as the live dashboard
- `execution/live_execution_log.md` exists as the append-only live log
- the execution OS read/update rules are simplified honestly
- the legacy mirrored status surfaces are explicitly downgraded to compatibility/archive status
- the first truthful Phase 3 packet is authored and activated if ready

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm the execution OS now uses the lean live model
- confirm the old mirrored surfaces are no longer mandatory live-update files
- confirm no product code or product behavior was changed
- perform a structural consistency review and contradiction check across the execution OS files

## Escalation Triggers
Escalate immediately if:
- the lean model would weaken truth or packet discipline materially
- Phase 3 cannot be activated honestly after simplification because the first packet cannot be bounded coherently
- the simplification would require changing Documents A, B, or C instead of the repo-local execution OS files

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Added `execution/progress.md` and `execution/live_execution_log.md` as the live execution dashboard and append-only log.
- Simplified the execution OS read/update rules in `execution/README.md` so live state now centers on the dashboard/log model rather than the older mirrored admin loop.
- Moved `execution/current_phase.md`, `execution/handoffs/latest_handoff.md`, and `execution/evidence/latest_evidence.md` to legacy compatibility/archive status rather than live mandatory surfaces.
- Authored and activated `execution/task_packets/p03_ws03_t01_establish_phase_3_activation_entry_basis_and_master_execution_blueprint.md` as the first truthful Phase 3 packet.
- Validation for this packet was an execution-doc structural consistency review and contradiction check only; no product code changed.