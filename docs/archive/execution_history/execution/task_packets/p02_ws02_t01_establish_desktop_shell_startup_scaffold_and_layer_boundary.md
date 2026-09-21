# Task Packet

## Task ID
P02-WS02-T01

## Task Title
Establish Desktop Shell Startup Scaffold and Layer Boundary

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-02 — Desktop Shell and Runtime Management

## Objective
Create the minimum startup scaffold and explicit layer-boundary foundation for the desktop shell without implementing product behavior.

## Why This Exists
Phase 2 cannot begin safely until the shell startup path, shell/frontend boundary, shell/engine boundary, and shell-owned privileged control surface are scaffolded explicitly enough to prevent architectural drift.

## Source-of-Truth References from Document A
- Section 7 — State Model and Lifecycle Rules
- Section 8 — System Architecture and Responsibility Boundaries
- Section 10 — Interface and Local Trust Boundary Rules
- Section 13 — Non-Functional Requirements
- Section 16 — Implementation Guardrails for AI Coding Agents

## Dependency Preconditions
- `Document_A/document_a_master_project_creative_brief_v2a_final_draft_2.md` exists
- `Document_B/document_b_master_execution_plan_b1_0_final_draft_2.md` exists
- `Document_C/document_c_engineering_annex_pack_c1_0_final_draft_1.md` exists
- `execution/README.md` is the authoritative execution operating-system guide
- Phase 1 is closed
- backbone choices are frozen as Rust/Tauri, React with TypeScript, Python, and the baseline test runners
- `app/`, `shell/`, `engine/`, and `tests/` exist as boundary folders

## Inputs / Required Context
- live Documents A, B, and C
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
- `shell/tauri.conf.json`
- `app/package.json`
- `app/tsconfig.json`
- `app/vite.config.ts`
- `app/src/main.tsx`
- `app/src/App.tsx`
- `execution/current_repo_truth.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md` only if a real local governance decision becomes necessary

## Implementation Scope
In scope:
- shell startup scaffold
- neutral frontend bootstrap inside the shell
- explicit shell/frontend command boundary scaffold
- explicit shell/engine supervision boundary scaffold
- no-feature placeholder UI shell
- no-code or low-code execution-state updates required to close the packet honestly

## Required Behavior to Preserve
- Document A supremacy
- Document B execution-order authority
- Document C technical-annex authority
- shell ownership of privileged local orchestration
- frontend as presentation/input layer only
- Python engine ownership of simulation execution once later packets enable it
- no hidden persistence
- no hidden scope expansion
- no fake completion claims

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- runtime hydration behavior beyond startup-path scaffolding
- activation logic
- provider configuration logic
- Preflight
- simulation behavior
- event streaming
- reporting
- cleanup behavior beyond shell-owned shutdown scaffolding
- updater behavior beyond packet-scoped placeholder surfaces if strictly required
- database schemas
- OpenAPI contracts
- real UI feature behavior
- engine business logic

## Deliverables
- shell startup scaffold files created
- app bootstrap files created
- explicit shell/frontend boundary scaffold created
- explicit shell/engine supervision boundary scaffold created
- updated repo truth
- updated handoff
- updated evidence
- updated decision log only if needed

## Validation / Acceptance Proof Required
- show exact files created/updated
- confirm shell technology, frontend baseline, engine runtime, and test baseline were obeyed
- confirm only packet-scoped shell/app bootstrap and execution-state files changed
- confirm no runtime hydration, activation, provider, Preflight, simulation, reporting, cleanup, updater, database, or OpenAPI behavior was implemented
- confirm the frontend cannot directly own privileged operations in the scaffold design
- confirm no hidden persistence was added

## Risks / Failure Modes
- shell-owned privileged logic bleeds into the frontend
- shell startup scaffold quietly implements feature behavior
- engine boundary is implied but not made explicit
- placeholder UI accidentally implies product truth
- runtime/update surfaces are overbuilt in a startup packet

## Escalation Triggers
Escalate immediately if:
- an existing repo file contradicts the frozen Phase 2 backbone choices
- the packet requires new persistence or hidden service surfaces
- the shell scaffold would require unresolved packaging or updater choices to proceed safely
- a required boundary cannot be represented without contradicting Documents A, B, or C

## Rollback / Rework Trigger
This packet must be reopened or reworked if:
- the frontend can directly perform shell-owned privileged actions
- product behavior is implemented under the label of startup scaffolding
- hidden persistence or hidden service boundaries are introduced
- the shell/engine boundary is still ambiguous after the scaffold is added
- downstream Phase 2 packets cannot build on the resulting startup scaffold safely

## Completion Decision
COMPLETE — 2026-04-14

Completion notes:
- shell startup scaffold files were created
- app bootstrap files were created
- explicit shell/frontend and shell/engine boundary scaffolds were established
- repo truth, handoff, and evidence were updated
- no out-of-scope implementation was added
*** Add File: d:\PROJECTS\miro-fish-desktop-saas\execution\packet_registry.md
# Packet Registry

## Purpose
This file is the canonical historical index of execution packets.
It is not a scope authority source.
Scope authority still comes from Documents A/B/C, the current execution-state files, and the active packet.

## Registry
| Packet ID | Title | State | Artifact | Outcome Notes |
| --- | --- | --- | --- | --- |
| P01-WS01-T01 | Establish Repo Execution Operating System and Governance Scaffolding | COMPLETE | `execution/task_packets/p01_ws01_t01_establish_repo_execution_os.md` | execution OS established; neutral repo boundaries created |
| P02-WS02-T01 | Establish Desktop Shell Startup Scaffold and Layer Boundary | COMPLETE | `execution/task_packets/p02_ws02_t01_establish_desktop_shell_startup_scaffold_and_layer_boundary.md` | shell and app startup boundary scaffolds established |
| P02-WS02-T02 | Implement Startup Checks and Runtime Presence Detection Scaffold | COMPLETE | `execution/task_packets/p02_ws02_t02_implement_startup_checks_and_runtime_presence_detection_scaffold.md` | startup-check and runtime-presence scaffolds added truthfully |
| P02-WS12-T01 | Establish Lead Architect Foundational SOP and Execution Gates | COMPLETE | `execution/task_packets/p02_ws12_t01_establish_lead_architect_foundational_sop_and_execution_gates.md` | execution SOP hardened; packet registry and zero-warning closeout rules established |

## Registry Rule
- Update this file whenever a packet is assigned, blocked, completed, cancelled, or otherwise changes state.
- Do not use this file to invent future scope.
- Do not use this file as a substitute for `current_phase.md`, `current_repo_truth.md`, handoff, or evidence.
*** Add File: d:\PROJECTS\miro-fish-desktop-saas\execution\task_packets\p02_ws12_t01_establish_lead_architect_foundational_sop_and_execution_gates.md
# Task Packet

## Task ID
P02-WS12-T01

## Task Title
Establish Lead Architect Foundational SOP and Execution Gates

## Parent Phase
Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Parent Workstream
WS-12 — AI-Agent Task Packaging and Prompt Control

## Objective
Strengthen the canonical execution operating system so future implementation remains context-reconstructable, packet-safe, critique-driven, and enterprise-gated.

## Why This Exists
The project relies on AI-led execution. The repo already has an execution operating system, but it needs explicit packet lifecycle rules, zero-warning closeout gates, a mandatory critique pass, and a canonical packet-history index to prevent drift and stale-state contradictions.

## Source-of-Truth References
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 13 — Non-Functional Requirements
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 1.9 — Task Packet Constitutional Rules
- Document B Section 3.15 — WS-12 — AI-Agent Task Packaging and Prompt Control
- Document B Section 6.4 — Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

## Dependency Preconditions
- `execution/README.md` already exists as the canonical execution operating-system guide
- Phase 2 is active
- execution-state files already exist and can be normalized without changing product scope
- this packet does not authorize product-feature implementation

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/current_phase.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- existing packet files under `execution/task_packets/`

## Files / Modules Expected to Change
Create or update only:
- `execution/README.md`
- `execution/current_constraints.md`
- `execution/current_phase.md`
- `execution/current_repo_truth.md`
- `execution/handoffs/latest_handoff.md`
- `execution/evidence/latest_evidence.md`
- `execution/decisions/decision_log.md`
- `execution/packet_registry.md`
- `execution/task_packets/p02_ws02_t01_establish_desktop_shell_startup_scaffold_and_layer_boundary.md` only to correct a repo-visible stale packet status if evidence supports it
- `execution/task_packets/p02_ws12_t01_establish_lead_architect_foundational_sop_and_execution_gates.md`

## Implementation Scope
In scope:
- canonical SOP hardening inside the execution operating system
- packet lifecycle-state definition
- enterprise zero-warning validation-gate rule
- mandatory critique-pass rule
- no-micro-slice packet rule
- packet-history registry
- execution-state normalization that removes active-versus-complete ambiguity
- correction of repo-visible packet-history contradictions supported by existing execution evidence

## Required Behavior to Preserve
- Documents A/B/C remain the authority stack
- Document B remains a strategic execution plan, not a running status log
- no product-feature implementation occurs
- no hidden persistence, service surfaces, or architecture drift is introduced
- no fabricated history is introduced

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- shell, frontend, engine, database, activation, provider, Preflight, simulation, reporting, cleanup, updater, or API product behavior
- a generic root-level `project_status.md`
- packet completion logging appended into Document B
- retroactive packet history that cannot be supported by repo-visible evidence
- dependency installation or toolchain expansion outside what is necessary to record current governance truth

## Deliverables
- strengthened canonical execution SOP
- explicit packet lifecycle states
- enterprise zero-warning gate policy
- mandatory critique-pass policy
- no-micro-slice packet rule
- packet registry created
- current execution state normalized to a truthful no-active-packet condition
- any repo-visible packet-history contradiction corrected honestly

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm no product implementation files changed
- confirm no generic status file was introduced
- confirm Document B was not turned into a running status log
- confirm critique-pass findings are recorded
- confirm any historical correction is supported by repo-visible evidence

## Escalation Triggers
Escalate immediately if:
- SOP hardening would require changing Documents A/B/C instead of the execution operating system
- a historical inconsistency cannot be corrected with repo-visible evidence
- a required enterprise gate rule would force unsupported toolchain invention during this governance packet
- the requested packet would need product-code changes to justify governance documentation

## Completion Decision
COMPLETE — 2026-04-15

Completion notes:
- canonical execution SOP now requires packet lifecycle states, enterprise zero-warning gates, a critique pass, and no micro-slice packets
- packet history is now indexed in `execution/packet_registry.md`
- current execution state now truthfully reports that no packet is currently active
- the stale `PENDING` status in `P02-WS02-T01` was corrected using repo-visible execution evidence
