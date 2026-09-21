# Task Packet

## Task ID
P09-WS09-T01

## Task Title
Establish Phase 9 Cleanup, Shutdown, and Startup Janitor Hardening Entry Basis and Master Execution Blueprint

## Parent Phase
Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening

## Parent Workstream
WS-09 — Cleanup, Shutdown, and Janitor

## Objective
Freeze the bounded cleanup, cleanup-result, app-close shutdown coordination, startup janitor, preserved-report boundary, and no-resume semantics strongly enough to author a detailed Phase 9 execution blueprint before cleanup/janitor feature code is implemented.

## Why This Exists
Phase 9 may not be entered truthfully as raw cleanup implementation yet because the exact boundaries for cleanup triggering, cleanup-result truth, startup janitor behavior, no-resume preservation, anti-zombie integration, and preserved-report exclusions are not concrete enough for autonomous implementation packets. This packet establishes that basis first so later Phase 9 work can proceed without guessing and without collapsing run outcome truth into cleanup success, janitor recovery, or hidden resume behavior.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
- Document A Section 6 — Product Behavior Specification
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 9 — Data Lifecycle and Storage Policy
- Document A Section 12 — UI/UX Contract
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.12 — WS-09 — Cleanup, Shutdown, and Janitor
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.12 — Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening
- Document B Section 6.11 — Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- Document C Annex Family D — Report and Output Pack

## Dependency Preconditions
- Phase 8 is closed honestly.
- The lean repo-local execution OS is in place.
- Terminal run states already exist.
- Shell-level shutdown coordination already exists.
- Workspace boundaries are already real.
- Report export boundaries are already understood.
- The shell, frontend, and engine gate baselines remain in the clean state recorded by `P08-WS11-T01`.
- The repo still does not materialize truthful cleanup triggering, cleanup-result exposure, startup janitor purge behavior, or explicit no-resume-preserving cleanup behavior.

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
- `execution/phase_08_master_execution_plan.md`
- `execution/task_packets/p08_ws11_t01_execute_phase_8_validation_critique_and_exit_gate.md`

## Files / Modules Expected to Change
Create or update only:
- `execution/phase_09_master_execution_plan.md`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p09_ws09_t01_establish_phase_9_cleanup_shutdown_and_startup_janitor_hardening_entry_basis_and_master_execution_blueprint.md`

## Implementation Scope
In scope:
- define the bounded Phase 9 cleanup/shutdown/janitor path strongly enough for later implementation packetization
- define truthful cleanup triggering, cleanup outcome, startup janitor, preserved-report exclusion, no-resume, and anti-zombie semantics needed for later coding packets
- clarify Phase 9 ownership boundaries versus Phase 7 shutdown control, Phase 8 report-preservation truth, and deferred Phase 10/11 release hardening
- author a detailed phase-specific execution blueprint with coherent later waves for Phase 9 implementation
- refine constraints and repo truth only where needed to keep the phase entry truthful

## Required Behavior to Preserve
- keep the shell as the privileged local authority for cleanup triggering, cleanup-result truth, shutdown coordination, janitor execution, and no-resume enforcement
- keep the engine as a producer of run-local artifacts and termination cooperation signals rather than a cleanup-policy authority
- keep the frontend as a consumer of shell-owned cleanup/janitor truth rather than a generator of cleanup success or filesystem purge decisions
- keep Phase 9 separate from Phase 10 updater/distribution hardening and Phase 11 release-readiness work
- keep cleanup attempt, cleanup outcome, run outcome, report availability, and report preservation distinct
- keep no-resume and best-effort cleanup limits explicit rather than softening them into stronger guarantees
- do not invent crash-resume, forensic deletion, hosted archive, or hidden continuity behavior that upstream documents do not authorize

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- cleanup code
- startup janitor code
- shutdown integration code
- anti-zombie code
- updater, release, or hosted archive/recovery behavior
- product feature code beyond execution planning artifacts

## Deliverables
- `execution/phase_09_master_execution_plan.md` exists
- the bounded Phase 9 cleanup/shutdown/janitor path is concrete enough for later coding packets
- the first real Phase 9 implementation waves are packetizable without guessing
- repo truth and constraints reflect the bounded Phase 9 entry basis honestly

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm the packet stayed inside execution-doc and architecture-governance scope only
- perform a structural consistency review and contradiction check across the changed execution/planning files
- confirm no cleanup, janitor, shutdown, report, updater, or other product implementation code was added during this packet

## Escalation Triggers
Escalate immediately if:
- bounded cleanup, janitor, or no-resume semantics cannot be frozen without changing Documents A, B, or C
- Phase 9 ownership boundaries remain too underspecified for a truthful execution blueprint
- the packet would require implementation code to remain coherent

## Completion Decision
COMPLETE — 2026-04-16

## Activation Notes
- Activated as the first Phase 9 packet to establish the Cleanup, Shutdown, and Startup Janitor Hardening entry basis and the Phase 9 master execution blueprint before cleanup/janitor feature implementation begins.
- Phase 9 feature implementation was not authorized at activation time and could not be overclaimed until this packet froze the bounded cleanup, janitor, and preservation basis.

## Completion Notes
- Added `execution/phase_09_master_execution_plan.md` as the phase-specific execution blueprint for Phase 9.
- Froze the bounded Phase 9 path around cleanup triggers, cleanup-result and janitor-result distinctions, report-preservation exclusions, shell-owned shutdown/janitor authority, anti-zombie integration limits, and explicit no-resume preservation.
- Clarified the ownership split among shell-owned cleanup/janitor truth, engine-owned run-local artifact production and termination cooperation, frontend non-authority over cleanup or filesystem purge decisions, and deferred Phase 10/11 hardening work, then synchronized the live execution surfaces to that basis.
- Validation for this packet was a structural consistency review and contradiction check across execution-planning files only; no cleanup, janitor, shutdown, report, updater, or other product implementation code was added.
- No local governance decision-log entry was required because the bounded Phase 9 basis could be frozen using existing Documents A, B, and C without inventing new product law.

## Structural Consistency and Contradiction Review
- Reviewed `execution/phase_09_master_execution_plan.md`, `execution/progress.md`, `execution/current_constraints.md`, `execution/current_repo_truth.md`, `execution/open_questions.md`, `execution/packet_registry.md`, `execution/live_execution_log.md`, and this packet file against the packet scope plus the governing Phase 9 rules in Documents A, B, and C.
- Confirmed the new blueprint preserves shell-owned cleanup and janitor truth, engine non-ownership of cleanup policy, frontend non-authority over cleanup success or purge decisions, preserved-report exclusions, and no-resume behavior.
- Confirmed the frozen Phase 9 plan does not overclaim real cleanup execution, janitor completeness, forensic deletion, crash-resume, hosted archive behavior, updater hardening, or release readiness.
- Confirmed the changed execution/planning files tell one coherent story: `execution/phase_09_master_execution_plan.md` now exists, `P09-WS09-T01` is complete, Phase 9 is active between packets, and later Phase 9 work must cite the frozen Phase 9 master plan.
- No contradiction, hidden-scope expansion, or unresolved blocker remained after review, so the packet is closed honestly.