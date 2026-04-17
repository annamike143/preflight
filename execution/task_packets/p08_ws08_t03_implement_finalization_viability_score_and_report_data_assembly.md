# Task Packet

## Task ID
P08-WS08-T03

## Task Title
Implement Finalization, Viability Score, and Report Data Assembly

## Parent Phase
Phase 8 — Finalization, Report Generation, and Report Access

## Parent Workstream
WS-08 — Finalization, Report Generation, and Export

## Objective
Implement the engine-owned finalization and report-data assembly path from actual run artifacts, including required-section synthesis, constrained-completion truth, transcript appendix assembly, and shell-owned report-generation success or failure classification inputs, without yet implementing final PDF generation, durable report export, or local report access behavior.

## Why This Exists
`P08-WS08-T02` froze the finalization/report vocabulary, required section inventory, report-boundary meaning, and report availability/access contract surfaces, but the repo still cannot transform actual run artifacts into truthful report data. Phase 8 cannot claim successful completion honestly until successful and constrained-completion paths can perform real finalization, assemble the required report sections from actual bounded-memory, transcript, chronology, and tracked-usage artifacts, and distinguish report-generation success from report-generation failure. This packet establishes that real engine-owned report-data assembly basis before later Wave 4 work may generate a PDF or expose a local report artifact.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
- Document A Section 6 — Product Behavior Specification
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 9 — Data Lifecycle and Storage Policy
- Document A Section 10 — API and Interface Contract
- Document A Section 12 — UI/UX Contract
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.11 — WS-08 — Finalization, Report Generation, and Export
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.11 — Phase 8 — Finalization, Report Generation, and Report Access
- Document B Section 6.10 — Phase 8 — Finalization, Report Generation, and Report Access
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family D — Report and Output Pack
- `execution/phase_08_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p08_ws08_t01_establish_phase_8_finalization_report_generation_and_report_access_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/task_packets/p08_ws08_t02_establish_finalization_and_report_output_contracts_report_boundary_and_availability_truth_surfaces.md` is complete.
- `execution/phase_08_master_execution_plan.md` remains the authoritative Phase 8 packetization basis.
- The current repo already materializes actual run artifacts under shell or engine authority, including bounded memory, transcript archive, ordered chronology, tracked token/duration usage, and finalization timestamps.
- Rust/Cargo/Clippy, frontend validation tooling, and engine Python validation tooling remain available locally.

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
- `shell/src/main.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_finalization_contract.rs`
- `shell/src/report_output_contract.rs`
- `shell/src/report_access_contract.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/startup_checks.rs`
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/bounded_memory.py`
- `engine/src/miro_fish_engine/transcript_archive.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/moderated_execution.py`
- `engine/src/miro_fish_engine/bounded_memory.py`
- `engine/src/miro_fish_engine/token_duration_tracking.py`
- `engine/src/miro_fish_engine/report_data_assembly.py`
- `engine/tests/test_session_execution.py`
- `engine/tests/test_moderated_execution.py`
- `engine/tests/test_report_data_assembly.py`
- `shell/src/main.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_finalization_contract.rs`
- `shell/src/report_output_contract.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/report_access_contract.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/startup_checks.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave7.test.tsx`
- `app/src/App.wave8.test.tsx`
- `app/src/App.wave9.test.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if report-assembly truth reveals a real unresolved governing gap
- `execution/decisions/decision_log.md` if a true local governance decision becomes necessary
- `execution/task_packets/p08_ws08_t03_implement_finalization_viability_score_and_report_data_assembly.md`

## Implementation Scope
In scope:
- engine-owned finalization entry behavior for successful and constrained-completion report-data assembly
- a real report-data assembly module that derives required sections only from actual run artifacts
- Market Viability Score representation generation under finalization authority without rewriting upstream scoring philosophy
- truthful Executive Summary, Ranked Risks, Strongest Pro Arguments, Strongest Anti Arguments, Cost and Token Summary, and Transcript Appendix assembly from actual bounded-memory, transcript, chronology, and tracked-usage artifacts only
- shell integration so report-generation success and report-generation failure become real projected truths rather than contract-only placeholders
- status, lifecycle, event, startup, and bounded frontend integration needed to surface the newly materialized report-generation truth without claiming a generated PDF or local artifact accessibility yet
- tests for engine report-data assembly, shell classification, and frontend truth rendering if touched

## Required Behavior to Preserve
- keep the Python engine as the owner of finalization and report-content synthesis inputs
- keep the Rust/Tauri shell as the sole authority for externally exposed completion truth, report-generation classification, report availability/accessibility, and local access mediation
- keep the frontend as presentation only, with no hidden authority over report existence, report accessibility, or completion classification
- preserve the governed required report sections exactly as upstream truth
- preserve the distinction between successful completion, constrained completion, report-generation failure, report availability, export accessibility, and cleanup outcome
- preserve the rule that report data derives only from actual completed run material and must not invent nonexistent rounds, turns, arguments, usage totals, or transcript dialogue
- preserve the distinction between report-generation success and later PDF or local artifact accessibility, which remains a later packet concern
- preserve the durable-report versus transient-workspace boundary without implementing durable writes yet

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- final PDF generation or any placeholder PDF output
- durable report write behavior, report catalog persistence, or local open/export execution
- hosted report storage, cloud report sync, alternate export formats, or release-readiness behavior
- cleanup hardening, janitor behavior, or cleanup outcome validation
- frontend-owned report inference or direct filesystem access
- commercial-backend or public API expansion

## Deliverables
- `engine/src/miro_fish_engine/report_data_assembly.py` exists and assembles truthful required report sections from actual run artifacts
- engine session execution returns real report-generation success or failure data and assembled section content basis
- shell lifecycle, status, and report contract surfaces consume the new report-generation result truthfully without claiming PDF or local artifact availability
- the frontend, if touched, renders report-generation truth as distinct from report availability and local access

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm `python -m compileall engine/src engine/tests`, `python -m ruff check engine`, and `python -m pytest engine/tests` complete with zero warnings and zero errors
- confirm assembled report data derives only from actual run artifacts and does not fabricate nonexistent debate material
- confirm report-generation failure remains distinct from successful completion and from report availability or access
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful required-section assembly cannot be implemented without changing Documents A, B, or C
- Market Viability Score generation would require rewriting the upstream scoring philosophy instead of formalizing a bounded representation
- shell integration would collapse report-generation success into report availability or cleanup truth
- any applicable shell, frontend, or engine gate still reports warnings or errors after in-scope fixes

## Completion Notes
- Engine-owned transient report-data assembly now exists in `engine/src/miro_fish_engine/report_data_assembly.py` and is invoked from finalization for successful and constrained-completion runs.
- `engine/src/miro_fish_engine/session_execution.py`, `engine/src/miro_fish_engine/moderated_execution.py`, and `engine/src/miro_fish_engine/token_duration_tracking.py` now preserve truthful finalization versus report-generation sequencing, materialize viability-score and required-section synthesis from actual run artifacts only, and classify report-data assembly failure as terminal rather than successful completion.
- `shell/src/run_session_start.rs`, `shell/src/run_lifecycle_state.rs`, `shell/src/run_event_contract.rs`, `shell/src/run_event_history.rs`, `shell/src/run_finalization_contract.rs`, `shell/src/report_output_contract.rs`, `shell/src/report_storage_boundary.rs`, `shell/src/run_status_contract.rs`, and `shell/src/startup_checks.rs` now surface real report-generation success versus failure truth, report-generation events, and Wave 3 materialization status without claiming PDF generation or local report accessibility.
- `app/src/App.tsx`, `app/src/App.test.tsx`, `app/src/App.wave7.test.tsx`, and `app/src/App.wave8.test.tsx` now render the new `execution_failed` outcome and the updated Phase 8 Wave 3 contract truth.
- Validation completed cleanly with `cargo test`, `cargo check`, `cargo clippy -- -D warnings`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `python -m compileall engine/src engine/tests`, `python -m ruff check engine`, and `python -m pytest engine/tests`.
- Critique pass result: clean for in-scope work; no material contradiction or in-scope remediation remained before closeout.

## Current State
COMPLETE — 2026-04-16

## Activation Notes
- Authored and activated immediately as the current executable Phase 8 Wave 3 packet for engine-owned finalization, viability-score generation, and required report-data assembly.
- Product-code changes are now authorized only within this packet's allowlist and bounded report-data assembly scope.
- Final PDF generation, durable report export, local report access behavior, and cleanup hardening remain out of scope while this packet is active.
- Final PDF generation, durable report export, local report access behavior, and cleanup hardening remain deferred to later packets after this packet's truthful completion.