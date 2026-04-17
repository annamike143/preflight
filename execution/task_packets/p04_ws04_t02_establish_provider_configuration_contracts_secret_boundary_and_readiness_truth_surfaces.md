# Task Packet

## Task ID
P04-WS04-T02

## Task Title
Establish Provider Configuration Contracts, Secret Boundary, and Readiness Truth Surfaces

## Parent Phase
Phase 4 — Provider Configuration and Secure Local Setup

## Parent Workstream
WS-04 — Provider Configuration and Secret Handling

## Objective
Define the code-level shell-owned provider-configuration contracts, provider-secret boundary surfaces, provider-readiness truth surfaces, and lifecycle or startup exposure needed for Phase 4 implementation, without claiming real provider-key storage, retrieval, rotation, deletion, or live provider behavior yet.

## Why This Exists
`P04-WS04-T01` froze the bounded provider-secret path and the Phase 4 wave plan, but the repo still lacks explicit provider-configuration contract modules and provider-readiness truth surfaces. Phase 4 Wave 3 cannot implement secure local provider commit and retrieval honestly until Wave 2 first defines the supported-provider contract, secret-boundary rules, readiness semantics, and shell-owned exposure surfaces in code.

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
- Document B Section 5.7 — Phase 4 — Provider Configuration and Secure Local Setup
- Document B Section 6.6 — Phase 4 — Provider Configuration and Secure Local Setup
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_04_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p04_ws04_t01_establish_phase_4_provider_configuration_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/phase_04_master_execution_plan.md` remains the authoritative Phase 4 packetization basis.
- Existing shell activation, trust-boundary, lifecycle, and startup surfaces remain available as the nearest repo-local pattern for contract, boundary, and readiness work.
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
- `shell/src/activation_contract.rs`
- `shell/src/activation_storage_boundary.rs`
- `shell/src/activation_readiness.rs`
- `shell/src/activation_secure_store.rs`
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
- `app/src/App.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved provider-support-state handling changes materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p04_ws04_t02_establish_provider_configuration_contracts_secret_boundary_and_readiness_truth_surfaces.md`

## Implementation Scope
In scope:
- shell-owned provider-configuration contract types for supported provider families, bounded provider-selection input, key-entry request shape, and truthful response or classification shape
- shell-owned provider storage-boundary surface defining OS-native secure-store authority, bounded non-secret provider-support-state contract, and forbidden ordinary persistence paths without yet materializing real provider-key commit behavior
- provider-readiness truth surface that remains separate from runtime readiness, activation readiness, and Preflight eligibility and that maps truthfully onto `ConfigurationRequired` versus `ReadyIdle`
- startup, lifecycle, and trust-boundary integration needed to expose the new provider contract or boundary or readiness truth surfaces honestly
- truthful frontend placeholder updates only if needed to surface the new shell-owned Phase 4 truth
- tests for the new contract, boundary, readiness, and integration surfaces

## Required Behavior to Preserve
- keep provider readiness separate from runtime readiness, activation readiness, and Preflight eligibility
- keep the Rust/Tauri shell as the sole local authority for provider-secret handling surfaces
- keep the frontend presentation and input only, with no direct secure-store access or raw provider-secret access
- keep supported MVP provider scope bounded to OpenAI and Google only
- preserve the frozen Phase 3 activation-artifact split and the existing activation and commercial-backend behavior untouched except where read-only integration is required
- do not overstate contract or boundary existence as proof that real provider configuration is already stored or usable

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- real provider-key commit, retrieval, rotation, deletion, or secure-store cleanup behavior
- provider-support-state persistence or recovery behavior beyond contract or boundary definition
- live provider validation, model discovery, reachability checks, or Preflight logic
- simulation execution, reporting, cleanup, updater, or runtime hydration behavior
- commercial-backend expansion, cloud provider vault behavior, or public API behavior
- product-feature UI beyond truthful placeholder or boundary-surface rendering

## Deliverables
- `shell/src/provider_configuration_contract.rs` exists and defines the bounded provider-configuration contract surface
- `shell/src/provider_storage_boundary.rs` exists and defines the shell-owned provider-secret boundary plus bounded support-state contract surface without claiming real secure-store behavior
- `shell/src/provider_readiness.rs` exists and defines provider-readiness truth separately from runtime, activation, and Preflight truth
- shell startup, lifecycle, trust-boundary, and command surfaces expose the new provider truth honestly
- the frontend, if touched, reports the bounded provider-configuration truth without implying real provider setup completion

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm no real provider-key storage, retrieval, rotation, deletion, or Preflight behavior was implemented
- confirm provider-readiness truth does not imply provider or model reachability or run-start eligibility
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful provider contract or readiness surfaces cannot be defined without changing Documents A, B, or C
- the packet would require real provider-secret persistence or retrieval behavior to remain coherent
- lifecycle or startup integration would collapse provider readiness into runtime readiness, activation readiness, or Preflight truth
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Added `shell/src/provider_configuration_contract.rs` to define the shell-owned provider-configuration contract surface for OpenAI and Google only, including bounded request expectations and failure classifications without materializing the real provider submission flow.
- Added `shell/src/provider_storage_boundary.rs` to freeze the provider-secret boundary in code: raw provider secrets remain secure-store-only by contract, bounded support-state metadata remains non-secret-only by contract, and no real secure-store commit or retrieval behavior is claimed yet.
- Added `shell/src/provider_readiness.rs` and integrated it into `shell/src/app_lifecycle.rs`, `shell/src/startup_checks.rs`, `shell/src/app_session_trust_boundary.rs`, and `shell/src/main.rs` so provider readiness now exists as a truthful shell-owned surface separate from runtime readiness, activation readiness, and Preflight eligibility.
- Updated `app/src/App.tsx` so the frontend truth now reports the new provider contract, boundary, and readiness surfaces without pretending that provider setup or secure provider storage is already implemented.
- Validation ran cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`.
- Critique pass outcome: the initial implementation introduced readiness-surface recursion and a Clippy enum-naming violation; both were fixed in-scope before closeout, and no real provider-key storage, retrieval, or Preflight behavior was added.