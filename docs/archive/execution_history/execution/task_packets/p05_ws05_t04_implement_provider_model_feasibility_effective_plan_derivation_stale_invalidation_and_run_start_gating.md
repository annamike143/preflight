# Task Packet

## Task ID
P05-WS05-T04

## Task Title
Implement Provider/Model Feasibility, Effective-Plan Derivation, Stale Invalidation, and Run-Start Gating

## Parent Phase
Phase 5 — Seed Intake and Preflight

## Parent Workstream
WS-05 — Seed Intake and Preflight

## Objective
Implement the real Wave 4 Preflight path: shell-owned provider/model feasibility checks, requested-versus-effective plan derivation, explicit approved versus approved-with-downgrade versus rejected outcomes, stale-result invalidation on material configuration change, and truthful run-start eligibility gating, while also replacing the frontend's current `--passWithNoTests` posture with real assertions around the existing Preflight submission flow before broader Wave 4 UI and outcome behavior expands further.

## Why This Exists
`P05-WS05-T03` established truthful single-seed intake and deterministic document-validation rejection behavior, but the repo still cannot truthfully determine provider/model feasibility, derive an effective plan, surface downgrade as a constrained approval path, invalidate stale Preflight truth on material input changes, or allow run start based on current eligible Preflight truth. The frontend gate also still passes under `--passWithNoTests`, which is no longer acceptable now that the Preflight submission surface is real and will be expanded by Wave 4.

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
- `execution/task_packets/p05_ws05_t03_implement_single_seed_intake_document_validation_and_deterministic_rejection_flow.md` is complete.
- Existing provider secure-store, provider support-state, provider submission, provider retrieval-mediation, provider-readiness, activation-readiness, runtime-readiness, lifecycle, trust-boundary, and local-interface surfaces remain available for truthful read-only integration and bounded Wave 4 implementation.
- Node.js, npm, Rust, Cargo, and frontend shell-validation tooling remain available locally.

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
- `shell/Cargo.toml`
- `shell/Cargo.lock`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/preflight_contract.rs`
- `shell/src/preflight_document_validation.rs`
- `shell/src/preflight_submission.rs`
- `shell/src/preflight_readiness.rs`
- `shell/src/provider_readiness.rs`
- `shell/src/provider_secure_store.rs`
- `shell/src/provider_support_store.rs`
- `shell/src/provider_configuration_contract.rs`
- `shell/src/provider_configuration_submission.rs`
- `shell/src/activation_readiness.rs`
- `shell/src/runtime_readiness.rs`
- `app/package.json`
- `app/package-lock.json`
- `app/vite.config.ts`
- `app/src/App.tsx`
- `app/src/main.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/Cargo.toml` if bounded feasibility or plan-derivation dependencies are required
- `shell/Cargo.lock` if Cargo dependency resolution changes it during the same bounded update
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/preflight_contract.rs`
- `shell/src/preflight_document_validation.rs` only if truthful invalidation or effective-plan inputs require limited refinement
- `shell/src/preflight_submission.rs`
- `shell/src/preflight_readiness.rs`
- `shell/src/preflight_feasibility.rs`
- `shell/src/preflight_effective_plan.rs`
- `app/package.json` if frontend DOM-test dependencies are required
- `app/package-lock.json` if npm dependency resolution changes it during the same bounded update
- `app/vite.config.ts` if Vitest environment configuration is required
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/test/setup.ts`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/task_packets/p05_ws05_t04_implement_provider_model_feasibility_effective_plan_derivation_stale_invalidation_and_run_start_gating.md`

## Implementation Scope
In scope:
- shell-owned provider/model feasibility checks during Preflight using the already bounded provider configuration and provider-secret retrieval surfaces
- bounded requested-versus-effective plan derivation and explicit downgrade disclosure basis
- truthful approved versus approved-with-downgrade versus rejected outcome materialization while preserving technical failure as a separate command-failure class
- stale-result invalidation on material changes to seed document, provider selection, provider secret, requested agent count, requested round count, max token budget, runtime validity, or activation validity
- run-start eligibility gating that requires current eligible Preflight truth and recorded downgrade acceptance when required
- frontend presentation changes required to surface effective-plan truth, downgrade disclosure, stale invalidation, and run-start gating honestly
- frontend Preflight submission tests using the repo-standard frontend test stack so `npm run test` exercises the real Preflight submission surface with assertions instead of exiting under `--passWithNoTests`
- tests for feasibility classification, effective-plan derivation, invalidation behavior, and run-start gating truth where applicable inside shell and frontend scope

## Required Behavior to Preserve
- keep Preflight eligibility separate from runtime readiness, activation readiness, provider readiness, and simulation execution
- keep the Rust/Tauri shell as the sole authority for Preflight outcome truth, stale invalidation, and run-start gating
- keep exactly one pending seed document per run and do not allow selection accumulation
- keep seed selection separate from approval
- keep document-validation rejection separate from technical failure
- keep approved-with-downgrade distinct from full approval and from rejection
- keep requested plan separate from effective plan and forbid silent requested-plan mutation
- keep raw provider secrets, raw seed documents, and raw extracted seed content out of routine logs, generic config, undeclared archives, the commercial backend, and ordinary frontend state beyond bounded transient selection state
- keep actual run/session creation, execution, monitoring, reporting, cleanup, updater behavior, and runtime hydration outside this packet

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- run/session creation or active execution behavior
- simulation, persona generation, report generation, cleanup hardening, or updater implementation
- commercial-backend expansion, cloud document storage, or public API behavior
- hidden durable persistence of raw seed documents, extracted seed text, or provider secrets
- silent fallback from provider/model infeasibility into fabricated approval truth
- arbitrary frontend architecture changes unrelated to bounded Preflight testing or truthful Wave 4 UI/state needs

## Deliverables
- shell-owned feasibility and effective-plan modules exist and are wired into the Preflight command path truthfully
- Preflight can now distinguish approved, approved-with-downgrade, rejected, and technical-failure outcomes without collapsing downgrade or stale invalidation semantics
- run-start eligibility truth requires a current eligible Preflight result and downgrade acceptance when applicable
- the frontend renders effective-plan, downgrade, and invalidation truth honestly
- the frontend test gate now contains real assertions around the Preflight submission flow and no longer relies on `--passWithNoTests` as the only passing condition for this feature surface

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowlisted files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors when shell code changes are in scope
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` complete with zero warnings and zero errors
- confirm the frontend test suite now includes real assertions around the Preflight submission flow
- confirm approved-with-downgrade remains distinct from approval, rejection, technical failure, and stale invalidation
- confirm stale results become non-current after material changes
- confirm run start remains blocked without current eligible Preflight truth
- confirm no simulation, run execution, reporting, cleanup, updater, or hidden persistence behavior was implemented
- perform a critique pass and contradiction check across the touched execution files before any closeout decision

## Escalation Triggers
Escalate immediately if:
- provider/model feasibility truth cannot be implemented honestly without ungoverned network assumptions, hidden persistence, or unsupported contract expansion
- effective-plan derivation would require silent mutation or misleading approval semantics
- stale invalidation or run-start gating would collapse downgrade, rejection, or technical-failure distinctions
- frontend test coverage would require unbounded framework replacement instead of bounded Vitest-based additions
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Current State
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 5 Wave 4 packet for provider/model feasibility, effective-plan derivation, stale invalidation, and run-start gating.
- Frontend Preflight submission tests are explicitly authorized inside this packet because the Wave 3 submission boundary is now real and must have assertion-backed regression coverage before Wave 4 expands the same UI and outcome surface further.
- Provider/model feasibility, effective-plan derivation, downgrade disclosure, stale invalidation, and run-start eligibility were incomplete at activation time and were not overclaimed before validation closed this packet.

## Completion Notes
- Implemented `shell/src/preflight_effective_plan.rs` and `shell/src/preflight_feasibility.rs` to materialize bounded provider/model planning context, effective-plan derivation, estimated-usage heuristics, rejection and invalidation messaging, and evaluation-basis comparison without claiming live provider/model reachability probing.
- Updated `shell/src/preflight_contract.rs`, `shell/src/preflight_submission.rs`, `shell/src/preflight_readiness.rs`, and `shell/src/main.rs` so the shell now distinguishes approved versus approved-with-downgrade versus rejected versus technical-failure outcomes, records downgrade acceptance, exposes current invalidation reasons, and computes truthful run-start eligibility.
- Updated `app/src/App.tsx`, `app/src/App.test.tsx`, and `app/src/App.wave4.test.tsx` to render effective plan, estimated usage, provider/model context, downgrade disclosure, invalidation truth, and assertion-backed frontend coverage for downgrade acceptance and stale invalidation.
- Product-code changes remained confined to the allowlisted shell and frontend files for this packet; closeout updates remained confined to the allowlisted execution-state files. No git repository is present in the workspace, so this boundary was verified directly from the applied patch set and targeted contradiction searches.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`. The frontend test suite now includes real assertions across `app/src/App.test.tsx` and `app/src/App.wave4.test.tsx`.
- The critique pass and contradiction check were completed across the touched execution files before closeout. Stale pre-Wave-4 execution-truth claims were corrected during closeout and no material contradictions remain within the packet scope.