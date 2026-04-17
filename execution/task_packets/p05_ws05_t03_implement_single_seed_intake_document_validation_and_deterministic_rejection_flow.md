# Task Packet

## Task ID
P05-WS05-T03

## Task Title
Implement Single-Seed Intake, Document Validation, and Deterministic Rejection Flow

## Parent Phase
Phase 5 — Seed Intake and Preflight

## Parent Workstream
WS-05 — Seed Intake and Preflight

## Objective
Implement the real shell-mediated single-seed intake path and deterministic document-validation rejection behavior: exactly one pending seed document, file metadata capture, PDF/DOCX/TXT/Markdown-only enforcement, 10 MiB maximum file-size enforcement, extractable-text validation, 1,000-normalized-character minimum-usable-text enforcement, and English-only rejection below the 0.80 confidence threshold with meaningful known rejection reasons, without yet implementing provider/model feasibility checks, effective-plan derivation, downgrade decisioning, or run start.

## Why This Exists
`P05-WS05-T02` materialized the contract and readiness foundation, but the repo still lacks any real shell-owned Preflight submission path or real seed-document validation behavior. Phase 5 cannot reach truthful run eligibility unless Wave 3 first implements deterministic single-seed intake and governed document-validation rejection behavior while keeping rejection distinct from technical failure and while keeping seed selection distinct from Preflight approval.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
- Document A Section 5 — User Journeys and Interaction Flows
- Document A Section 6 — Product Behavior Specification
- Document A Section 7 — State Machines
- Document A Section 9 — Data Lifecycle and Storage Policy
- Document A Section 10 — API and Interface Contract
- Document A Section 12 — UI/UX Contract
- Document A Section 13 — Non-Functional Requirements
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.8 — WS-05 — Seed Intake and Preflight
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.8 — Phase 5 — Seed Intake and Preflight
- Document B Section 6.7 — Phase 5 — Seed Intake and Preflight
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_05_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p05_ws05_t01_establish_phase_5_seed_intake_and_preflight_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/task_packets/p05_ws05_t02_establish_seed_intake_contracts_preflight_outcome_classes_and_run_eligibility_truth_surfaces.md` is complete.
- `execution/phase_05_master_execution_plan.md` remains the authoritative Phase 5 packetization basis.
- Existing Preflight contract and readiness surfaces remain available as the required Wave 2 foundation.
- Existing runtime, activation, provider, lifecycle, trust-boundary, and local-interface surfaces remain available for read-only integration and truthful gating.
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
- `execution/phase_05_master_execution_plan.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/preflight_contract.rs`
- `shell/src/preflight_readiness.rs`
- `shell/src/provider_readiness.rs`
- `shell/src/activation_readiness.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/Cargo.toml` if bounded document-extraction dependencies are required to implement governed PDF and DOCX validation
- `shell/Cargo.lock` if Cargo dependency resolution changes the lockfile during the same bounded update
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/preflight_contract.rs`
- `shell/src/preflight_readiness.rs`
- `shell/src/preflight_document_validation.rs`
- `shell/src/preflight_submission.rs`
- `app/src/App.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/task_packets/p05_ws05_t03_implement_single_seed_intake_document_validation_and_deterministic_rejection_flow.md`

## Implementation Scope
In scope:
- real shell-owned Preflight submission flow for exactly one selected seed document and bounded requested-plan inputs
- file metadata capture and obvious local file/input failure handling at the shell-owned boundary
- supported-type enforcement for PDF, DOCX, TXT, and Markdown only; 10 MiB maximum file-size enforcement; extractable-text checks; 1,000-normalized-character minimum-usable-text checks; and English-only rejection below the 0.80 confidence threshold for the seed document
- deterministic known rejection reasons for governed document-validation failure paths while preserving technical failure as a separate command-failure class
- readiness, lifecycle, startup, trust-boundary, local-interface, command, and frontend integration needed to expose selected-seed truth and deterministic document-validation rejection truth honestly
- tests for submission behavior, document-validation behavior, known rejection classification, and truth-surface integration

## Required Behavior to Preserve
- keep Preflight eligibility separate from runtime readiness, activation readiness, provider readiness, and simulation execution
- keep the Rust/Tauri shell as the sole local authority for seed-document processing after boundary submission, Preflight outcome truth, and run-start gating
- keep the frontend presentation and input only, with no hidden authority over Preflight approval or run-start eligibility
- keep exactly one pending seed document per run; replacing the selected file must replace the prior pending seed rather than accumulating multiple seeds
- keep seed selection separate from Preflight approval
- keep document-validation rejection distinct from technical failure
- keep the minimum known document-validation rejection families limited to unsupported file type, file too large, text extraction failed, insufficient extractable text, and non-English seed document unless a later upstream revision explicitly expands them
- keep approved and approved-with-downgrade semantics contract-defined but do not overclaim them as materially implemented if Wave 3 still only establishes rejection behavior
- keep raw seed documents and raw extracted seed content out of generic config, routine logs, the commercial backend, undeclared archives, and ordinary frontend state beyond bounded transient selection state
- preserve the closed Phase 3 activation surfaces and closed Phase 4 provider surfaces except where truthful read-only integration is required
- do not imply run-start eligibility from successful file selection or from deterministic document-validation success alone

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- provider or model readiness checks
- cost or duration estimation behavior
- effective-plan derivation behavior
- downgrade decisioning or downgrade-acceptance workflow behavior
- run/session creation, run start, or active execution behavior
- simulation, reporting, cleanup, updater, or runtime hydration behavior
- commercial-backend expansion, cloud document storage, or public API behavior
- hidden durable persistence of raw seed documents or raw extracted seed content

## Deliverables
- `shell/src/preflight_document_validation.rs` exists and materializes governed PDF/DOCX/TXT/Markdown-only, 10 MiB maximum-file-size, extractable-text, 1,000-normalized-character minimum-usable-text, and 0.80-threshold English-only checks for the seed document
- `shell/src/preflight_submission.rs` exists and materializes the real shell-owned single-seed intake and document-validation submission path
- Preflight contract, readiness, lifecycle, startup, trust-boundary, local-interface, and command surfaces reflect real selected-seed and deterministic document-validation rejection truth without implying approved execution
- the frontend exposes one-document selection and Preflight submission behavior that keeps seed selection distinct from Preflight approval and surfaces deterministic known rejection reasons truthfully

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm PDF/DOCX/TXT/Markdown acceptance boundaries, the 10 MiB size ceiling, the 1,000-normalized-character minimum, and the 0.80 English-confidence threshold are enforced exactly
- confirm unsupported, oversized, unextractable, insufficient-text, and non-English documents are rejected truthfully when those governed conditions are encountered
- confirm known document-validation rejection reasons are surfaced meaningfully and remain distinct from technical failure
- confirm selected seed-document presence does not imply Preflight approval or run-start eligibility
- confirm no provider/model readiness checks, effective-plan derivation, downgrade decisioning, run start, or simulation behavior were implemented
- perform a critique pass and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- Documents A, B, C, the decision log, or this packet disagree about the frozen seed-document constants or minimum document-validation rejection families
- extractable-text or English-only validation would require hidden persistence or ungoverned service dependencies beyond the frozen thresholds
- lifecycle or startup integration would collapse selected-seed truth into approval truth or collapse document-validation rejection into technical failure
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Current State
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 5 Wave 3 packet for real single-seed intake, document validation, and deterministic known-rejection behavior.
- Governance realignment on 2026-04-15 froze the exact seed-intake constants, minimum rejection families, and MVP runtime/update split before Wave 3 implementation resumed.
- Repo changes are authorized only within the scope of this packet until closeout or blockage.
- Provider/model readiness checks, effective-plan derivation, downgrade decisioning, stale-result invalidation execution behavior, and run/session start remain explicitly out of scope during this packet.

## Completion Notes
- Implemented `shell/src/preflight_document_validation.rs` and `shell/src/preflight_submission.rs` to materialize the governed single-seed validation and submission path, including PDF/DOCX/TXT/Markdown support, the 10 MiB size ceiling, extractable-text enforcement, 1,000 normalized extracted-character minimum, English-confidence rejection, SHA-256 fingerprint capture, and one-pending-seed replacement semantics.
- Updated `shell/src/preflight_contract.rs`, `shell/src/preflight_readiness.rs`, `shell/src/local_interface_contract.rs`, and `shell/src/main.rs` so the bounded `submit_preflight` command, selected-seed truth, requested-plan echo, deterministic rejection families, and continued run-start blocking are exposed honestly without overclaiming full Preflight approval.
- Updated `app/src/App.tsx` to submit one seed document through the shell, collect bounded requested-plan inputs, render the Wave 3 non-approval submission posture alongside rejected versus technical-failure outcomes, and display the expanded shell-owned Preflight boundary state.
- Product-code changes remained confined to the allowlisted shell and frontend files for this packet; closeout updates remained confined to the allowlisted execution-state files. No git repository is present in the workspace, so this boundary was verified directly from the applied patch set.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`. The frontend test command exited cleanly under `--passWithNoTests` because no frontend test files existed yet at that closeout.
- Structural consistency review and contradiction check were completed across the touched execution files before closeout. No material contradictions remain within the packet scope.