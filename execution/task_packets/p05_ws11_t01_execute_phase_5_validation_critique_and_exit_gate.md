# Task Packet

## Task ID
P05-WS11-T01

## Task Title
Execute Phase 5 Validation, Critique, and Exit Gate

## Parent Phase
Phase 5 — Seed Intake and Preflight

## Parent Workstream
WS-11 — Validation, Critique, and Exit Gates

## Objective
Execute the Phase 5 exit-gate validation pass, acceptance mapping, critique pass, and closure decision so the repo can either close Phase 5 honestly or remain open with an explicit blocking reason if any required seed-intake, Preflight, downgrade-transparency, stale-invalidation, or run-start-gating evidence is still missing.

## Why This Exists
`P05-W05` is the final Phase 5 packet defined by `execution/phase_05_master_execution_plan.md`. Waves 2, 3, and 4 established the contract surface, real single-seed validation, and real bounded Preflight feasibility and effective-plan behavior. This packet now proves whether that implementation actually satisfies the governed acceptance targets and whether Phase 5 may close without inventing approval, downgrade, invalidation, or run-start truth that the repo does not support.

## Source-of-Truth References
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
- `execution/task_packets/p05_ws05_t03_implement_single_seed_intake_document_validation_and_deterministic_rejection_flow.md`
- `execution/task_packets/p05_ws05_t04_implement_provider_model_feasibility_effective_plan_derivation_stale_invalidation_and_run_start_gating.md`

## Dependency Preconditions
- `execution/task_packets/p05_ws05_t03_implement_single_seed_intake_document_validation_and_deterministic_rejection_flow.md` is complete.
- `execution/task_packets/p05_ws05_t04_implement_provider_model_feasibility_effective_plan_derivation_stale_invalidation_and_run_start_gating.md` is complete.
- No other packet may remain active while this packet is active.
- The frozen Phase 5 basis remains unchanged: one seed document only, English-only governed document acceptance, explicit requested-versus-effective plan separation, explicit stale invalidation, explicit downgrade acceptance, and explicit run-start gating.

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/phase_05_master_execution_plan.md`
- `execution/task_packets/p05_ws05_t03_implement_single_seed_intake_document_validation_and_deterministic_rejection_flow.md`
- `execution/task_packets/p05_ws05_t04_implement_provider_model_feasibility_effective_plan_derivation_stale_invalidation_and_run_start_gating.md`
- `shell/Cargo.toml`
- `shell/Cargo.lock`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/preflight_contract.rs`
- `shell/src/preflight_document_validation.rs`
- `shell/src/preflight_effective_plan.rs`
- `shell/src/preflight_feasibility.rs`
- `shell/src/preflight_submission.rs`
- `shell/src/preflight_readiness.rs`
- `app/package.json`
- `app/package-lock.json`
- `app/vite.config.ts`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`
- `app/src/test/setup.ts`

## Files / Modules Expected to Change
Create or update only:
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if unresolved Preflight questions change materially after the exit-gate pass
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if the Phase 5 closure decision requires a real local governance decision
- `execution/task_packets/p05_ws11_t01_execute_phase_5_validation_critique_and_exit_gate.md`
- `shell/Cargo.toml` or `shell/Cargo.lock` only if the critique pass finds an in-scope Phase 5 defect that requires a bounded dependency or lockfile update
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/preflight_contract.rs`
- `shell/src/preflight_document_validation.rs`
- `shell/src/preflight_effective_plan.rs`
- `shell/src/preflight_feasibility.rs`
- `shell/src/preflight_submission.rs`
- `shell/src/preflight_readiness.rs`
- `app/package.json`
- `app/package-lock.json`
- `app/vite.config.ts`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave4.test.tsx`
- `app/src/test/setup.ts`

## Implementation Scope
In scope:
- acceptance mapping for `AC-031` through `AC-040`
- rerunning the applicable shell and frontend validation gates for the implemented Phase 5 seed-intake and Preflight surfaces
- explicit evidence review for English-only rejection, deterministic document rejection, downgrade transparency, stale invalidation, and truthful run-start gating
- critique pass over the touched Phase 5 implementation and execution-state files
- tightly bounded in-scope remediation if the critique pass finds a real defect in the implemented Phase 5 surfaces
- explicit Phase 5 closure decision and execution-state updates

## Required Behavior to Preserve
- the packet must not invent acceptance proof from scaffolding, Wave 2 contract text, or stale pre-Wave-4 statements
- seed selection must remain distinct from seed approval
- provider configuration presence must remain distinct from current eligible Preflight truth
- approved-with-downgrade must remain distinct from full approval, rejection, technical failure, and stale invalidation
- run start must remain blocked unless there is current eligible Preflight truth and any required downgrade acceptance is already recorded
- raw seed documents, raw extracted seed text, and raw provider secrets must remain out of ordinary files, logs, reports, frontend state, and the commercial backend
- if the evidence does not support Phase 5 closure, the packet must leave Phase 5 open honestly rather than forcing closure

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- run/session creation or active run execution behavior
- simulation, reporting, cleanup hardening, updater behavior, or runtime hydration behavior
- live provider/model reachability probing beyond the bounded local Preflight planning-context logic already implemented
- new persistence domains or hidden execution-state surfaces
- broader feature expansion beyond Phase 5 validation and tightly bounded in-scope remediation

## Deliverables
- acceptance mapping exists for the governed Phase 5 criteria
- all applicable Phase 5 shell and frontend gates are rerun truthfully
- critique findings are either fixed in-scope or escalated explicitly
- the repo records a truthful Phase 5 exit-gate decision

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- rerun and report the exact results of:
  - `cargo check`
  - `cargo clippy -- -D warnings`
  - `cargo test`
  - `npm run lint`
  - `npm run typecheck`
  - `npm run test`
  - `npm run build`
- confirm non-English input is rejected truthfully under the frozen 0.80 English-confidence rule
- confirm approved-with-downgrade remains distinct from approval, rejection, technical failure, and stale invalidation
- confirm stale Preflight truth becomes non-current after material input changes
- confirm run start cannot bypass current eligible Preflight truth
- confirm no run/session execution, simulation, reporting, updater, or hidden persistence behavior was added
- confirm whether Phase 5 can close truthfully and record the result explicitly

## Escalation Triggers
Escalate immediately if:
- any governed Phase 5 acceptance target still lacks real evidence after the validation pass
- any applicable gate reports an error or warning after in-scope remediation attempts
- the critique pass reveals that the implemented Phase 5 path still depends on contradictory or non-current Preflight truth
- closing Phase 5 would require treating selected-seed presence, provider configuration presence, or a stale invalidated result as current eligible Preflight truth

## Completion Decision
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 5 Wave 5 packet for acceptance mapping, validation reruns, critique, and the final Phase 5 exit-gate decision over the implemented seed-intake and Preflight surfaces.
- Phase 5 closure is not authorized at activation time and must not be overclaimed until this packet proves `AC-031` through `AC-040` against the live repo truth.

## Completion Notes
- Re-ran the full Phase 5 shell and frontend gate set cleanly during this exit-gate pass: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`.
- Confirmed the live implementation still enforces one-seed intake, PDF/DOCX/TXT/Markdown-only validation, the governed 10 MiB size ceiling, the 1,000 normalized extracted-character minimum, English-only rejection at the 0.80 confidence threshold, explicit approved versus approved-with-downgrade versus rejected outcomes, stale invalidation on material change, and truthful run-start eligibility gating.
- The critique pass reviewed the active Phase 5 shell, frontend, and execution-state surfaces and found no material contradiction or in-scope defect that required remediation before closure.
- No product-code changes were required during the exit-gate pass; only the allowlisted execution-state files and this packet file were updated to record the validation evidence, critique result, and closure decision.
- Phase 5 now closes honestly because `AC-031` through `AC-040` are fully proven against the live seed-intake and Preflight implementation without overclaiming run/session execution, live provider/model reachability probing, reporting, updater behavior, or hidden persistence.

## Acceptance Mapping
- `AC-031` — PROVEN by the single `seed_document` request shape in `shell/src/preflight_submission.rs`, the single stored pending-seed slot in the shell submission state, and `preflight_submission::tests::rejected_document_submission_replaces_prior_result_without_accumulating`, which verifies replacement rather than accumulation.
- `AC-032` — PROVEN by the supported-kind boundary in `shell/src/preflight_document_validation.rs`, which accepts only PDF, DOCX, TXT, and Markdown via extension or MIME classification, together with `preflight_document_validation::tests::unsupported_extension_is_rejected` and the positive TXT or DOCX validation tests.
- `AC-033` — PROVEN by the governed `MAX_SEED_DOCUMENT_SIZE_BYTES` limit in `shell/src/preflight_contract.rs` and `preflight_document_validation::tests::oversized_document_is_rejected`.
- `AC-034` — PROVEN by the shell-owned readiness surface in `shell/src/preflight_readiness.rs`, which keeps `run_start_eligible_now` false and `run_start_blocked_without_eligible_preflight` true until current eligible Preflight truth exists, together with `preflight_readiness::tests::preflight_readiness_truth_surface_keeps_run_start_blocked_without_preflight`.
- `AC-035` — PROVEN by the explicit approved, approved-with-downgrade, and rejected decision classes in `shell/src/preflight_submission.rs` and `shell/src/preflight_feasibility.rs`, while technical failure remains separate, together with `preflight_submission::tests::approved_submission_persists_effective_plan_and_requested_plan`, `preflight_submission::tests::downgraded_submission_requires_and_records_explicit_acceptance`, and `preflight_submission::tests::rejected_document_submission_replaces_prior_result_without_accumulating`.
- `AC-036` — PROVEN by the frozen `MINIMUM_ENGLISH_CONFIDENCE_PERCENT` threshold in `shell/src/preflight_contract.rs`, the English-confidence check in `shell/src/preflight_document_validation.rs`, and `preflight_document_validation::tests::non_english_text_is_rejected`.
- `AC-037` — PROVEN by the meaningful known-rejection messaging in `shell/src/preflight_feasibility.rs` and the frontend rendering path in `app/src/App.tsx`, together with `App.test.tsx` coverage that renders deterministic rejection details when the shell rejects a governed seed document.
- `AC-038` — PROVEN by the requested-plan versus effective-plan rendering, downgrade warning, and provider/model planning-context display in `app/src/App.tsx`, together with `App.wave4.test.tsx`, which verifies that downgrade disclosure is shown before explicit acceptance is recorded.
- `AC-039` — PROVEN by the run-start eligibility logic in `shell/src/preflight_readiness.rs`, which permits eligibility only for current approved results or for current approved-with-downgrade results after explicit acceptance, together with `preflight_readiness::tests::approved_with_downgrade_requires_acceptance_before_run_start` and the Wave 4 frontend downgrade-acceptance test.
- `AC-040` — PROVEN by the material-change comparison logic in `shell/src/preflight_feasibility.rs`, the invalidated-state resolution in `shell/src/preflight_readiness.rs`, `preflight_feasibility::tests::invalidation_reasons_detect_requested_plan_and_seed_changes`, `preflight_readiness::tests::materially_changed_requested_plan_invalidates_prior_result`, and `App.wave4.test.tsx`, which verifies stale invalidation after a provider change.