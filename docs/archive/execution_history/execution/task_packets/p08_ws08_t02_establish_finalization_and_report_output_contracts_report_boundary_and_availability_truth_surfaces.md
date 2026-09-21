# Task Packet

## Task ID
P08-WS08-T02

## Task Title
Establish Finalization and Report Output Contracts, Report Boundary, and Availability Truth Surfaces

## Parent Phase
Phase 8 — Finalization, Report Generation, and Report Access

## Parent Workstream
WS-08 — Finalization, Report Generation, and Export

## Objective
Define the code-level finalization, report-output, report-availability, and local report-access contract surfaces plus the durable report boundary and required-section schema basis needed for later Phase 8 implementation, without claiming real report synthesis, viability-score generation, PDF generation, or durable report file materialization yet.

## Why This Exists
`P08-WS08-T01` froze the bounded Phase 8 output path and wave order, but the repo still lacks explicit contract modules for finalization/output distinctions, required report sections, report metadata and availability structures, durable report boundary meaning, and local report access/export request or response shapes. Phase 8 Wave 3 cannot implement engine-owned report data assembly honestly, and Phase 8 Wave 4 cannot implement PDF generation and local report access honestly, until Wave 2 first defines the vocabulary, section inventory, boundary classes, and shell-owned exposure surfaces in code while preserving the distinction between completion classification, report-generation success, report availability, export accessibility, and cleanup truth.

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
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- Document C Annex Family D — Report and Output Pack
- `execution/phase_08_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p08_ws08_t01_establish_phase_8_finalization_report_generation_and_report_access_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/phase_08_master_execution_plan.md` remains the authoritative Phase 8 packetization basis.
- Existing Phase 7 lifecycle, status, event, and constrained-finalization truth surfaces remain available as the nearest repo-local pattern for finalization/output contract work.
- Existing shell-owned run/session and monitoring surfaces already expose runtime cap reason, constrained finalization timestamps, and completion classification truth, but they do not yet expose report-generation or report-availability contracts.
- Rust/Cargo/Clippy and frontend validation tooling remain available locally.

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
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_session_contract.rs`
- `shell/src/run_event_history.rs`
- `shell/src/run_session_start.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/run_event_contract.rs`
- `shell/src/run_finalization_contract.rs`
- `shell/src/report_output_contract.rs`
- `shell/src/report_storage_boundary.rs`
- `shell/src/report_access_contract.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave7.test.tsx`
- `app/src/App.wave8.test.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved report metadata, local report catalog/support-state, or output-boundary handling changes materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p08_ws08_t02_establish_finalization_and_report_output_contracts_report_boundary_and_availability_truth_surfaces.md`

## Implementation Scope
In scope:
- shell-owned finalization and output contract types for finalization entered, finalization succeeded, report generation in progress, report generation succeeded, report generation failed, report available, and report unavailable without collapsing those meanings
- report-output contract structures for required report-section inventory, output classes, report metadata shape, PDF input/output contract basis, and the stable representation basis for viability score, ranked risks, pro/anti arguments, cost/token summary, and transcript appendix without real synthesis yet
- report storage-boundary surface defining transient report-generation intermediates, durable exported report artifacts, and report metadata or index references where later governed upstream without claiming real persistence implementation yet
- report access/export contract surface for metadata retrieval, availability representation, local file access/export request shape, and unavailable or missing or inaccessible output failure classifications
- lifecycle, startup, local-interface, status-projection, event-contract, and frontend integration needed to expose the new Phase 8 contract and boundary truth honestly
- tests for the new contract, boundary, availability, and integration surfaces

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for externally exposed completion truth, report-generation classification, report availability/accessibility, and local report access/export contract exposure
- keep the Python engine as the future owner of finalization and report-content synthesis inputs rather than giving it interface authority over report availability or export semantics
- keep the frontend as presentation and input only, with no hidden authority over report existence, report accessibility, completion classification, or cleanup meaning
- preserve the distinction between finalization entered, finalization success, report generation success, report generation failure, report availability, export accessibility, and cleanup outcome
- preserve the governed required report-section names exactly as upstream truth, without inventing alternate report variants, extra sections, or new output modes
- preserve the durable-report versus transient-workspace boundary and forbid secret-bearing fields in report metadata, report intermediates, or exported artifacts
- preserve the completed Phase 7 monitoring and constrained-finalization truth without overstating it as proof that a real report already exists
- do not overstate contract or boundary existence as proof that real report generation, PDF creation, local file access, or report preservation is already implemented

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- real viability-score computation or report-section synthesis
- real transcript appendix assembly or cost/token summary generation
- real PDF generation, durable report write behavior, local open/export execution, or report catalog persistence
- cleanup hardening, janitor behavior, hosted report storage, cloud report sync, alternate output formats, or release-readiness behavior
- commercial-backend expansion, public API behavior, or product-feature UI beyond truthful contract or boundary-surface rendering

## Deliverables
- `shell/src/run_finalization_contract.rs` exists and defines the bounded finalization/output classification surface
- `shell/src/report_output_contract.rs` exists and defines required report-section inventory, output classes, metadata shape, and output-structure basis
- `shell/src/report_storage_boundary.rs` exists and defines the transient-versus-durable report boundary without claiming real persistence implementation
- `shell/src/report_access_contract.rs` exists and defines report metadata retrieval, availability, and access/export request or response shapes
- shell lifecycle, status, local-interface, and command surfaces expose the new Phase 8 contract truth honestly
- the frontend, if touched, reports the bounded Phase 8 report contract truth without implying real report availability, PDF existence, or cleanup success

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm no real report synthesis, PDF generation, local file access/export execution, or cleanup behavior was implemented
- confirm the delivered contracts distinguish completion classification, report-generation success, report availability, and export accessibility without collapsing them into one status
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful report contract, report boundary, or availability surfaces cannot be defined without changing Documents A, B, or C
- the packet would require real report data assembly, viability-score computation, or PDF generation to remain coherent
- lifecycle, status, local-interface, or frontend integration would collapse finalization, completion, report availability, export accessibility, or cleanup truth
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-16

## Activation Notes
- Activated as the current executable Phase 8 Wave 2 packet for finalization/report-output contract foundation after the completed Phase 8 entry-basis freeze.
- Product-code changes are now authorized only within this packet's allowlist and bounded contract-foundation scope.
- Real report data assembly, PDF generation, local report access behavior, and cleanup hardening remain out of scope while this packet is active.

## Completion Notes
- Added `shell/src/run_finalization_contract.rs`, `shell/src/report_output_contract.rs`, `shell/src/report_storage_boundary.rs`, and `shell/src/report_access_contract.rs` as the bounded Phase 8 Wave 2 contract-foundation modules for finalization markers, required report sections, output classes, report boundary meaning, and report availability/access classifications.
- Integrated the new shell-owned Phase 8 surfaces through `shell/src/main.rs`, `shell/src/app_lifecycle.rs`, `shell/src/app_session_trust_boundary.rs`, `shell/src/startup_checks.rs`, `shell/src/local_interface_contract.rs`, `shell/src/run_lifecycle_state.rs`, `shell/src/run_status_contract.rs`, and `shell/src/run_event_contract.rs` so the repo exposes finalization/report truth without collapsing completion classification, report-generation success, report availability, export accessibility, or cleanup meaning.
- Updated `app/src/App.tsx` to render a bounded Phase 8 contract-foundation section and refreshed `app/src/App.test.tsx`, `app/src/App.wave7.test.tsx`, and new `app/src/App.wave8.test.tsx` so the frontend consumes shell-owned report truth without implying a real report, PDF artifact, or local access path already exists.
- Validation reran `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` cleanly.
- No real report synthesis, viability-score computation, transcript appendix assembly, PDF generation, durable report write behavior, local open/export execution, or cleanup hardening was implemented in this packet.

## Structural Consistency and Contradiction Review
- Reviewed `execution/progress.md`, `execution/current_constraints.md`, `execution/current_repo_truth.md`, `execution/packet_registry.md`, `execution/live_execution_log.md`, and this packet file against the completed code changes and the governed Phase 8 ownership split.
- Confirmed the repo now exposes bounded finalization/output vocabulary, report-output schema basis, durable-versus-transient report boundary meaning, and report availability/access contract truth while still keeping real report generation and local access behavior out of scope.
- Confirmed the changed execution surfaces tell one coherent story: `P08-WS08-T02` is complete, Phase 8 is active between packets, the new shell/frontend contract foundation is real, and no later Phase 8 implementation packet is active yet.
- No contradiction, hidden-scope expansion, unresolved blocker, or failed gate remained after review, so the packet is closed honestly.