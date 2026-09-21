# Execution Operating System

## Purpose
This folder is the canonical execution-control layer for bounded AI-led implementation.
It exists to keep implementation packetized, reviewable, phase-safe, and reconstructable from repository state.
It does not replace Document A, Document B, or Document C.

## Canonical Location
- The authoritative execution operating-system guide is `execution/README.md`.
- Root `README.md` is not present and is not used for execution control.
- Do not create a parallel root-level `project_status.md` for execution tracking.
- Do not append packet-level `Done` notes or runtime status into Document B.

## Lean Live Model
- The live execution model is intentionally lean.
- The primary live status surfaces are:
	- `execution/progress.md`
	- `execution/live_execution_log.md`
- `execution/current_phase.md`, `execution/handoffs/latest_handoff.md`, and `execution/evidence/latest_evidence.md` are retained only as legacy compatibility/archive surfaces unless a recovery packet explicitly reauthorizes them as live state.
- Do not recreate a mirrored multi-file admin loop for every packet when the lean surfaces already express the truth.

## Authority Order
1. `Document_A/document_a_master_project_creative_brief_v2a_final_draft_2.md`
2. `Document_B/document_b_master_execution_plan_b1_0_final_draft_2.md`
3. `Document_C/document_c_engineering_annex_pack_c1_0_final_draft_1.md`
4. `execution/progress.md`
5. `execution/current_constraints.md`
6. `execution/current_repo_truth.md`
7. `execution/open_questions.md`
8. the explicitly assigned task packet under `execution/task_packets/`
9. `execution/packet_registry.md`
10. `execution/live_execution_log.md`
11. legacy compatibility/archive surfaces only when explicitly needed:
	 - `execution/current_phase.md`
	 - `execution/handoffs/latest_handoff.md`
	 - `execution/evidence/latest_evidence.md`

If any lower-level file conflicts with a higher-level file, the higher-level file wins.

## Mandatory Read Order For Any AI Agent
Before making repo changes, read in this order:
1. `execution/progress.md`
2. `execution/current_constraints.md`
3. `execution/current_repo_truth.md`
4. `execution/open_questions.md`
5. the explicitly assigned task packet, if one is assigned
6. the relevant sections of Documents A, B, and C required by that packet
7. `execution/packet_registry.md` and `execution/live_execution_log.md` only when historical context is needed

If no task packet is assigned, stop after reading the execution state files and wait for the next packet.

## Execution Rules
- Do not implement outside the assigned packet.
- Do not invent scope, states, events, fields, persistence classes, endpoints, outputs, or architecture.
- Do not create a new canonical Phase 0.
- Do not treat neutral scaffolding as settled framework or runtime truth.
- Do not create duplicate status surfaces outside `execution/`.
- Do not append implementation status or packet history into Document B.
- Escalate on ambiguity affecting product truth, execution truth, technical-annex truth, or persistence meaning.

## Packet Lifecycle States
Use these states consistently across packet files, `progress.md`, `packet_registry.md`, and `live_execution_log.md`:
- `DRAFTED` = packet concept exists but is not approved for execution.
- `ASSIGNED` = packet is approved as the next executable unit but implementation has not started.
- `ACTIVE` = the packet is the only packet currently authorized for repo changes.
- `BLOCKED` = the packet cannot proceed truthfully because of missing authority, missing tooling, failed gates, missing dependencies, or unresolved contradictions.
- `COMPLETE` = implementation, validation, critique pass, and required execution-state updates are all finished.
- `CANCELLED` = packet was intentionally abandoned and must not be treated as partial completion.

Repository queue state is separate from packet state:
- the repo may have `no active packet assigned` while the most recently completed packet remains historical truth.
- only one packet may be `ACTIVE` at a time.

## Packet Sizing Standard
- Do not create micro-slice packets.
- A packet must be the smallest coherent execution unit that can be implemented, validated, critiqued, and handed off end-to-end without requiring another token-round merely to prove the first one was meaningful.
- If a packet is too small to support a truthful implementation-plus-validation cycle, redesign it before execution.
- If a packet becomes too broad to validate or critique coherently, split it before execution rather than during closeout.

## Enterprise Validation Gate
Every packet must define and execute the relevant validation gates for the surfaces it changes.

Rules:
- No packet may be marked `COMPLETE` if any applicable build, lint, type-check, or test gate reports an error.
- No packet may be marked `COMPLETE` if any applicable build, lint, type-check, or test gate reports a warning.
- Missing dependencies, missing scripts, missing config, or missing tooling count as gate failures, not as implied passes.
- If a required gate cannot run truthfully, the packet must remain `BLOCKED` or be redesigned before implementation continues.
- Each packet must record the exact gate commands or validation checks used to close it.

Minimum gate expectations by changed surface:
- shell code: build/check, lint, and test gates appropriate to Rust/Tauri code
- frontend code: build, lint, type-check, and test gates appropriate to React + TypeScript code
- engine code: build or import/syntax gate, lint, type-check where governed, and test gates appropriate to Python code
- execution-doc-only packets: structural consistency review and contradiction check across the execution OS files

## Mandatory Critique Pass
After implementation and before packet closeout, perform a critique pass over:
- the touched files
- adjacent execution-state files
- gate results
- known repo realities that could make the packet’s claims misleading

The critique pass must:
- identify remaining inconsistencies, boundary drift, unfinished gate work, weak assumptions, and improvement opportunities
- fix material in-scope findings before closeout
- escalate or explicitly log out-of-scope findings rather than silently ignoring them
- update the live dashboard/log surfaces so the next agent inherits the critique result, not just the implementation diff

## Historical Integrity Rule
- Do not fabricate retroactive history.
- If a historical inconsistency is discovered in the execution OS, correct it only when repo-visible evidence supports the correction.
- If evidence is insufficient, record the gap honestly and create a dedicated recovery packet instead of inventing a clean history.

## Folder Roles
- `progress.md` = live dashboard for active phase, active packet, wave posture, and next-step truth
- `live_execution_log.md` = append-only execution log for activations, completions, closures, and governance transitions
- `current_constraints.md` = active hard constraints and escalation triggers
- `current_repo_truth.md` = known repository state versus unresolved unknowns
- `packet_registry.md` = historical index of packet state and outcomes; not a scope authority source
- `open_questions.md` = unresolved items that do not authorize implementation drift
- `decisions/decision_log.md` = real local governance decisions only
- `task_packets/` = bounded implementation packets
- legacy compatibility/archive surfaces only:
	- `current_phase.md`
	- `handoffs/latest_handoff.md`
	- `evidence/latest_evidence.md`

## Update Rule
Every completed packet must update:
- its own packet file state and completion notes
- `progress.md`
- `current_repo_truth.md`
- `packet_registry.md`
- `live_execution_log.md`
- `decisions/decision_log.md` only when a real local execution decision was made
- `current_constraints.md` only when the active execution rules or gating constraints changed
- `open_questions.md` only when unresolved questions or their default handling materially change
- legacy compatibility/archive surfaces only when a recovery or governance-maintenance packet explicitly requires them

## Phase 1 Boundary
Phase 1 authorizes governance scaffolding, traceability scaffolding, execution logging, and empty neutral repo boundaries only.
It does not authorize feature code, runtime logic, persistence code, API surfaces, or framework-locking architecture decisions.

## Phase 2 Entry Baseline
Phase 2 may begin only on top of explicitly frozen backbone choices.
The current frozen baseline is:
- desktop shell technology: Rust/Tauri
- frontend framework baseline: React with TypeScript
- engine runtime/language: Python
- repo test runner baseline: `cargo test` for shell code, `vitest` for frontend code, and `pytest` for engine code