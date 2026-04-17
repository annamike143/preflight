# Task Packet

## Task ID
P04-WS04-T03

## Task Title
Implement Provider Secure Local Commit, Retrieval Mediation, and Configuration Gating

## Parent Phase
Phase 4 — Provider Configuration and Secure Local Setup

## Parent Workstream
WS-04 — Provider Configuration and Secret Handling

## Objective
Implement the real shell-mediated provider configuration path: provider selection input, provider-key submission, OS-native secure-store commit, bounded non-secret provider support-state persistence, shell-only retrieval mediation, truthful provider configuration outcomes, and truthful `ConfigurationRequired` versus `ReadyIdle` gating without claiming Preflight, provider reachability, or live inference behavior.

## Why This Exists
`P04-WS04-T02` established the contract, boundary, and readiness foundation, but the repo still lacks the real local provider setup path that stores a provider key, persists bounded non-secret support state, mediates secret retrieval through shell-owned layers, and updates provider-readiness truth from actual local state. Phase 4 cannot close honestly until this concrete provider configuration behavior exists.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 9 — Data Lifecycle and Storage Policy
- Document A Section 10 — API and Interface Contract
- Document A Section 12 — UI/UX Contract
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.7 — WS-04 — Provider Configuration and Secret Handling
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.7 — Phase 4 — Provider Configuration and Secure Local Setup
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_04_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p04_ws04_t01_establish_phase_4_provider_configuration_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/task_packets/p04_ws04_t02_establish_provider_configuration_contracts_secret_boundary_and_readiness_truth_surfaces.md` is complete.
- Existing activation secure-store, support-store, and local-validation implementation patterns remain available as the nearest repo-local reference for shell-owned local state work.
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
- `execution/phase_04_master_execution_plan.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/provider_configuration_contract.rs`
- `shell/src/provider_storage_boundary.rs`
- `shell/src/provider_readiness.rs`
- `shell/src/activation_secure_store.rs`
- `shell/src/activation_support_store.rs`
- `shell/src/activation_local_validation.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/provider_configuration_contract.rs`
- `shell/src/provider_storage_boundary.rs`
- `shell/src/provider_readiness.rs`
- `shell/src/provider_secure_store.rs`
- `shell/src/provider_support_store.rs`
- `shell/src/provider_configuration_submission.rs`
- `app/src/App.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` if provider support-store narrowing changes materially
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p04_ws04_t03_implement_provider_secure_local_commit_retrieval_mediation_and_configuration_gating.md`

## Implementation Scope
In scope:
- real shell-owned provider secure-store abstraction and OS-native commit or retrieval or delete behavior for supported providers
- bounded provider support-state persistence and recovery in the app-controlled local support domain without storing raw provider secrets
- shell-mediated provider configuration submission path from frontend input to secure local commit and support-state update
- shell-only provider secret retrieval mediation for later allowed local execution layers without exposing raw provider secrets to the frontend
- truthful provider configuration success or failure outcomes, including malformed input, secure-store failure, OS permission issues, configuration-present truth, and configuration-missing truth
- lifecycle, startup, trust-boundary, and frontend integration needed to make `ConfigurationRequired` versus `ReadyIdle` gating truthful from actual local provider configuration state
- tests for secure-store behavior, support-state persistence, provider submission outcomes, retrieval mediation, and gating truth

## Required Behavior to Preserve
- keep provider readiness separate from runtime readiness, activation readiness, and Preflight eligibility
- keep the Rust/Tauri shell as the sole local authority for provider-secret handling and retrieval mediation
- keep supported MVP provider scope bounded to OpenAI and Google only
- keep raw provider secrets out of plaintext config, normal workspace files, routine logs, reports, and transcript archives
- keep the frontend presentation and input only; no raw provider-secret retrieval or direct secure-store access may cross into the frontend
- do not claim provider or model reachability, provider feasibility, or run-start eligibility from successful key storage alone
- preserve the closed Phase 3 activation and commercial-backend behavior except where startup or lifecycle truth consumes the new provider readiness state

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- Preflight feasibility logic
- provider/model reachability checks
- live provider inference behavior
- simulation execution, reporting, cleanup, updater, or runtime hydration behavior
- commercial-backend expansion, cloud provider vault behavior, or public API behavior
- hidden persistence outside the governed support-store and secure-store domains

## Deliverables
- `shell/src/provider_secure_store.rs` exists and materializes the shell-owned provider secure-store abstraction and OS-native commit or retrieval or delete path
- `shell/src/provider_support_store.rs` exists and materializes bounded non-secret provider support-state persistence and recovery
- `shell/src/provider_configuration_submission.rs` exists and materializes the shell-owned provider configuration commit path and retrieval mediation
- provider-readiness truth now derives from actual local provider state instead of scaffold-only assumptions
- the frontend exposes a truthful provider configuration flow that exercises the real shell-owned path without receiving raw provider secrets

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm no provider secrets are intentionally persisted in plaintext app files, ordinary config, workspaces, reports, or logs
- confirm the frontend cannot directly retrieve raw provider secrets
- confirm successful provider-key storage does not imply provider or model readiness before Preflight
- perform a critique pass and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful provider support-state persistence requires overriding the frozen storage-domain split
- a real provider setup path cannot be implemented without changing Documents A, B, or C
- lifecycle gating would require collapsing provider readiness into runtime readiness, activation readiness, or Preflight truth
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Current State
COMPLETE — 2026-04-15

## Activation Notes
- Activated as the current Phase 4 Wave 3 implementation packet.
- Repo changes are authorized only within the scope of this packet until closeout or blockage.
- Preflight, provider reachability, and live inference behavior remain explicitly out of scope during this packet.
- Completed by materializing `shell/src/provider_secure_store.rs`, `shell/src/provider_support_store.rs`, and `shell/src/provider_configuration_submission.rs`, wiring provider readiness to actual local state, exposing bounded provider submission and provider-state shell commands, and updating the frontend to exercise the real provider path without retrieving raw provider secrets.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`.
- Critique pass outcome: no contradiction was found between the touched code surfaces and the updated execution-state files; provider configuration remains separate from runtime readiness, activation readiness, and Preflight truth.