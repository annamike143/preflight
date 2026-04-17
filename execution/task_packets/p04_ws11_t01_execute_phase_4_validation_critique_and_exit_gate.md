# Task Packet

## Task ID
P04-WS11-T01

## Task Title
Execute Phase 4 Validation, Critique, and Exit Gate

## Parent Phase
Phase 4 — Provider Configuration and Secure Local Setup

## Parent Workstream
WS-11 — Validation, Critique, and Exit Gates

## Objective
Execute the Phase 4 exit-gate validation pass, acceptance mapping, critique pass, and closure decision so the repo can either close Phase 4 honestly or remain open with explicit blocking truth if any required provider-configuration, secure-secret-handling, or configuration-gating evidence is still missing.

## Why This Exists
`P04-W04` is the final Phase 4 packet defined by `execution/phase_04_master_execution_plan.md`. Waves 2 and 3 established the provider contract and secret boundary foundation, then implemented the real shell-owned secure provider setup path. This packet now proves whether that implementation actually satisfies the governed acceptance targets and whether Phase 4 may close without inventing provider or Preflight success that the repo does not support.

## Source-of-Truth References
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
- `execution/task_packets/p04_ws04_t02_establish_provider_configuration_contracts_secret_boundary_and_readiness_truth_surfaces.md`
- `execution/task_packets/p04_ws04_t03_implement_provider_secure_local_commit_retrieval_mediation_and_configuration_gating.md`

## Dependency Preconditions
- `execution/task_packets/p04_ws04_t02_establish_provider_configuration_contracts_secret_boundary_and_readiness_truth_surfaces.md` is complete.
- `execution/task_packets/p04_ws04_t03_implement_provider_secure_local_commit_retrieval_mediation_and_configuration_gating.md` is complete.
- No other packet is active.
- The bounded provider-secret split remains frozen: raw provider secrets belong only in OS-native secure storage and bounded non-secret provider-configuration support state belongs only in the app-controlled local persistent support domain.

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/phase_04_master_execution_plan.md`
- `execution/task_packets/p04_ws04_t02_establish_provider_configuration_contracts_secret_boundary_and_readiness_truth_surfaces.md`
- `execution/task_packets/p04_ws04_t03_implement_provider_secure_local_commit_retrieval_mediation_and_configuration_gating.md`
- `shell/src/provider_configuration_contract.rs`
- `shell/src/provider_secure_store.rs`
- `shell/src/provider_support_store.rs`
- `shell/src/provider_configuration_submission.rs`
- `shell/src/provider_storage_boundary.rs`
- `shell/src/provider_readiness.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/startup_checks.rs`
- `shell/src/main.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if unresolved provider-handling questions change materially after the exit-gate pass
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if the Phase 4 closure decision requires a real local governance decision
- `execution/task_packets/p04_ws11_t01_execute_phase_4_validation_critique_and_exit_gate.md`
- `shell/src/` files only if the critique pass finds a material in-scope defect in the already implemented Phase 4 provider surfaces
- `app/src/App.tsx` only if the critique pass finds a truthful UI contradiction in the bounded provider-configuration surface

## Implementation Scope
In scope:
- acceptance mapping for `AC-025` through `AC-030`
- rerunning the applicable shell and frontend validation gates for the implemented Phase 4 provider surfaces
- explicit evidence review for supported-provider scope, secure-store-only secret handling, bounded support-state persistence, shell-only retrieval mediation, truthful `ConfigurationRequired` gating, and no overclaiming of provider/model readiness before Preflight
- critique pass over the touched Phase 4 implementation and execution-state files
- tightly bounded in-scope remediation if the critique pass finds a real defect in the implemented Phase 4 provider surfaces
- explicit Phase 4 exit-gate decision and execution-state updates

## Required Behavior to Preserve
- the packet must not invent acceptance proof from scaffolding, old Wave 2 statements, or stale Phase 4 claims
- provider readiness must remain distinct from runtime readiness, activation readiness, and Preflight eligibility
- raw provider secrets must remain only in approved OS-native secure storage
- bounded provider support state must remain minimal and non-secret
- successful local provider configuration must not be treated as proof that a provider or model is reachable or valid for run execution
- if the evidence does not support Phase 4 closure, the packet must leave Phase 4 open honestly rather than forcing closure

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- Preflight feasibility logic
- provider/model reachability checks
- live provider inference behavior
- simulation, reporting, cleanup hardening, updater, or runtime hydration behavior
- new persistence domains or hidden execution-state surfaces
- broader feature expansion beyond Phase 4 validation and tightly bounded in-scope remediation

## Deliverables
- acceptance mapping exists for the governed Phase 4 criteria
- all applicable Phase 4 gates are rerun truthfully
- critique findings are either fixed in-scope or escalated explicitly
- the repo records a truthful Phase 4 exit-gate decision

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
- confirm supported provider scope remains OpenAI and Google only
- confirm provider secrets are intentionally stored only in approved OS-native secure storage and not in plaintext app files, ordinary config, workspaces, reports, or logs
- confirm the frontend cannot directly retrieve raw provider secrets
- confirm missing required provider configuration still truthfully blocks `ReadyIdle`
- confirm configuration-present is not misrepresented as provider/model readiness or Preflight-ready status
- confirm whether Phase 4 can close truthfully; if not, record the blocking reason explicitly

## Escalation Triggers
Escalate immediately if:
- any governed Phase 4 acceptance target still lacks real evidence after the validation pass
- any applicable gate reports an error or warning after in-scope remediation attempts
- the critique pass reveals that the implemented provider path still depends on a mocked or contradictory configuration state
- closing Phase 4 would require treating stored provider configuration as proof of provider or model reachability before Preflight

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Re-ran the full Phase 4 shell and frontend gate set cleanly after critique remediation: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`.
- Confirmed the supported provider scope remains OpenAI and Google only, provider secrets remain confined to OS-native secure storage under shell authority, bounded provider support state remains non-secret, and the frontend still cannot retrieve raw provider secrets after submission.
- The critique pass found two material Phase 4 defects and fixed them in scope: provider switches previously left the deselected provider secret behind in secure storage, and the declared `unsupported_provider` failure classification was not reachable at the shell command boundary because unsupported values were rejected before shell-side classification.
- Phase 4 now closes honestly because `AC-025` through `AC-030` are fully proven against the live provider-configuration implementation without overclaiming provider reachability, model readiness, or Preflight success.

## Acceptance Mapping
- `AC-025` — PROVEN by the bounded provider selection contract and frontend workflow, which expose only OpenAI and Google as approved MVP choices and now reject unsupported provider values explicitly at the shell boundary.
- `AC-026` — PROVEN by the materialized shell-owned provider submission path and the frontend provider-configuration form, which provide a real way to store a provider API key required for later run execution.
- `AC-027` — PROVEN by the OS-native `keyring`-backed provider secure-store implementation and the provider secure-store and storage-boundary tests, which keep raw provider secrets under shell-owned secure storage only.
- `AC-028` — PROVEN by the bounded provider support-state store and provider-storage-boundary behavior, which persist only non-secret metadata in the app-controlled support domain, forbid ordinary plaintext secret persistence, and now clear stale deselected-provider secrets during provider switches.
- `AC-029` — PROVEN by the provider-readiness and app-lifecycle truth surfaces plus the frontend state rendering, which keep provider configuration missing or broken in a `required` or `error` posture, expose the missing-detail truth, and preserve `ConfigurationRequired` rather than `ReadyIdle` when required provider configuration is absent.
- `AC-030` — PROVEN by the submission outcome contract, provider-readiness semantics, and frontend messaging, which state that successful provider-key storage establishes local configuration presence only and does not imply provider reachability, model availability, or Preflight approval.