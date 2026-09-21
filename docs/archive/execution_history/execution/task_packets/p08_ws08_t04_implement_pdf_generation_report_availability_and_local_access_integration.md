# Task Packet

## Task ID
P08-WS08-T04

## Task Title
Implement PDF Generation, Report Availability, and Local Access Integration

## Parent Phase
Phase 8 — Finalization, Report Generation, and Report Access

## Parent Workstream
WS-08 — Finalization, Report Generation, and Export

## Objective
Materialize the real PDF generation path from the engine-owned assembled report data, shell-owned durable report artifact handling, truthful report availability and local access behavior, and the bounded frontend access affordances without adding report catalog persistence, hosted report storage, or cleanup hardening.

## Why This Exists
`P08-WS08-T03` established truthful finalization sequencing, engine-owned report-data assembly, viability-score generation, required-section synthesis, and shell-owned report-generation success versus failure classification. The repo still cannot produce the governed PDF artifact, write that artifact to a durable local path outside the transient run workspace, or expose truthful shell-owned report availability, metadata retrieval, and local open or export behavior. Phase 8 cannot reach its exit gate honestly until those Wave 4 capabilities are real.

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
- `execution/task_packets/p08_ws08_t03_implement_finalization_viability_score_and_report_data_assembly.md` is complete.
- `execution/phase_08_master_execution_plan.md` remains the authoritative Phase 8 packetization basis.
- The current repo already materializes actual report-data assembly outputs and report-generation success or failure classification inputs.
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
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/report_data_assembly.py`
- `shell/src/main.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_finalization_contract.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/report_output_contract.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/report_access_contract.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/startup_checks.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave8.test.tsx`

## Files / Modules Expected to Change
Create or update only:
- `engine/src/miro_fish_engine/session_execution.py`
- `engine/src/miro_fish_engine/report_data_assembly.py`
- `engine/src/miro_fish_engine/report_pdf_generation.py`
- `engine/tests/test_session_execution.py`
- `engine/tests/test_report_pdf_generation.py`
- `shell/src/main.rs`
- `shell/src/run_session_start.rs`
- `shell/src/run_finalization_contract.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/report_output_contract.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/report_access_contract.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/startup_checks.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave8.test.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if PDF or access truth reveals a real unresolved governing gap
- `execution/decisions/decision_log.md` if a true local governance decision becomes necessary
- `execution/phase_08_master_execution_plan.md`
- `execution/task_packets/p08_ws08_t04_implement_pdf_generation_report_availability_and_local_access_integration.md`

## Implementation Scope
In scope:
- real PDF generation from the already assembled report data and only the governed required report sections
- durable local report artifact creation outside the transient run workspace when report generation succeeds
- shell-owned report metadata retrieval and truthful report availability or unavailability determination from actual file-state evidence
- shell-owned local open and export behavior for the generated PDF without frontend filesystem authority
- shell, lifecycle, startup, interface, and bounded frontend integration needed to expose report availability and local access truth without conflating those truths with cleanup outcome
- tests for engine PDF generation, shell report availability or access behavior, and frontend report-access rendering if touched

## Required Behavior to Preserve
- keep the Python engine as the owner of report-content synthesis inputs and the source of PDF-generation input material
- keep the Rust/Tauri shell as the sole authority for externally exposed completion truth, report availability or accessibility truth, local path mediation, and local open or export execution
- keep the frontend as presentation only, with no hidden authority over report existence, report accessibility, or completion classification
- preserve the governed required report sections exactly as upstream truth
- preserve the distinction between successful completion, constrained completion, report-generation success, report availability, local accessibility, export execution, and cleanup outcome
- preserve the durable-report versus transient-workspace boundary without implying report catalog persistence or cleanup hardening
- preserve the rule that successful completion does not imply report accessibility unless the local artifact actually exists and is accessible
- preserve the rule that report content derives only from actual completed run material and must not invent nonexistent rounds, turns, arguments, usage totals, or transcript dialogue

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- report catalog persistence or metadata-index persistence beyond what the existing contracts already define as a later possibility
- hosted report storage, cloud report sync, alternate export formats, browser report portals, or release-readiness behavior
- cleanup hardening, janitor behavior, or cleanup outcome validation
- frontend-owned report inference or direct filesystem access
- commercial-backend or public API expansion
- any new phase-exit or acceptance-mapping work that belongs to `P08-WS11-T01`

## Deliverables
- `engine/src/miro_fish_engine/report_pdf_generation.py` exists and materializes a real PDF artifact from assembled report data
- successful and constrained-completion runs can produce a durable local PDF artifact outside the transient run workspace when generation succeeds
- shell-owned report availability and local access behavior are real and remain distinct from report-generation success and cleanup outcome
- the frontend, if touched, renders report availability and local access affordances only from shell-owned truth

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm `python -m compileall engine/src engine/tests`, `python -m ruff check engine`, and `python -m pytest engine/tests` complete with zero warnings and zero errors
- confirm successful and constrained-completion runs now yield a real PDF artifact when generation succeeds
- confirm report availability and local access remain distinct from report-generation success and from cleanup outcome
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful PDF generation cannot be implemented without changing Documents A, B, or C
- durable report output would require violating the shell-owned storage-boundary and access-authority split
- shell integration would collapse report-generation success into report availability, export accessibility, or cleanup truth
- local open or export behavior would require frontend filesystem authority or hidden hosted infrastructure
- any applicable shell, frontend, or engine gate still reports warnings or errors after in-scope fixes

## Current State
COMPLETE — 2026-04-16

## Activation Notes
- Authored and activated immediately as the current executable Phase 8 Wave 4 packet for PDF generation, shell-owned report availability, durable local report output, and local access integration.
- Product-code changes are now authorized only within this packet's allowlist and bounded Wave 4 scope.
- Phase 8 exit-gate assessment remains out of scope until `P08-WS11-T01` is authored and activated.

## Completion Notes
- `engine/src/miro_fish_engine/report_pdf_generation.py` now materializes a real governed PDF artifact from the assembled report data, and `engine/src/miro_fish_engine/session_execution.py` now completes finalization by generating that PDF for successful and constrained-completion runs.
- `shell/src/run_session_start.rs`, `shell/src/report_storage_boundary.rs`, `shell/src/report_access_contract.rs`, `shell/src/app_lifecycle.rs`, `shell/src/startup_checks.rs`, and `shell/src/main.rs` now materialize shell-owned durable report artifact handling, truthful report availability and metadata retrieval, and local open/export execution without giving the frontend filesystem authority.
- The frontend now renders the report artifact card and shell-owned report-access affordances through the completed reports surface while keeping preview-only review modes development-scoped and non-authoritative.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `python -m compileall engine/src engine/tests`, `python -m ruff check engine`, and `python -m pytest engine/tests`.
- The critique pass found no material contradiction in the completed Wave 4 implementation, and Phase 8 exit-gate work remained deferred to `P08-WS11-T01`.