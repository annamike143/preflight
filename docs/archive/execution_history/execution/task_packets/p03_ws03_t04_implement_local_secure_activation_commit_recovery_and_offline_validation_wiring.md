# Task Packet

## Task ID
P03-WS03-T04

## Task Title
Implement Local Secure Activation Commit, Recovery, and Offline Validation Wiring

## Parent Phase
Phase 3 — Licensing, Activation, and Commercial Backend

## Parent Workstream
WS-03 — Licensing, Activation, and Commercial Backend

## Objective
Implement the shell-mediated local activation commit path, bounded local activation support-record persistence and recovery, current-device token validation, startup activation-readiness determination, and truthful lifecycle wiring for approved offline-capable activation validation behavior.

## Why This Exists
`P03-W04` turns approved commercial activation into truthful local licensed use. `P03-WS03-T02` already froze the shell-side activation contract and storage-boundary split, while `P03-WS03-T03` is the bounded place where real approval or denial or error truth becomes available. This packet consumes those foundations so activation can progress from approved to locally committed and later validate offline in the limited scenarios the product allows.

## Source-of-Truth References
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 9 — Data Lifecycle and Storage Policy
- Document A Section 10 — API and Interface Contract
- Document A Section 11 — Database and Commercial Backend Contract
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.5 — WS-02 — Desktop Shell and Runtime Management
- Document B Section 3.6 — WS-03 — Licensing, Activation, and Commercial Backend
- Document B Section 4.7 — Workstream Dependency Matrix Logic
- Document B Section 6.5 — Phase 3 — Licensing, Activation, and Commercial Backend
- Document C Annex Family B — Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_03_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p03_ws03_t02_establish_activation_contracts_and_shell_owned_local_artifact_boundary_surfaces.md` is complete.
- `execution/task_packets/p03_ws03_t03_implement_commercial_ledger_records_and_activation_decision_path.md` must be complete or otherwise proven to supply real activation approval versus denial versus technical-failure truth and token issuance lineage consistent with the shell contract.
- The bounded activation-artifact split remains frozen: raw token material belongs only in OS-native secure storage and bounded non-secret support metadata belongs only in the local persistent support domain.
- If the final local persistent support storage engine must be narrowed to implement governed support-record persistence, that narrowing may occur only inside this packet and only if it preserves the frozen split.

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/phase_03_master_execution_plan.md`
- `shell/src/activation_contract.rs`
- `shell/src/activation_storage_boundary.rs`
- `shell/src/activation_readiness.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/startup_checks.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/main.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/Cargo.toml`
- `shell/Cargo.lock` if dependency resolution updates it for the bounded Wave 4 shell dependencies
- `shell/src/activation_contract.rs` only if final local commit or validation semantics require bounded contract refinement
- `shell/src/activation_storage_boundary.rs`
- `shell/src/activation_readiness.rs`
- `shell/src/activation_secure_store.rs`
- `shell/src/activation_support_store.rs`
- `shell/src/activation_local_validation.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/startup_checks.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/main.rs`
- `app/src/App.tsx` if truthful user-facing placeholder state changes are required
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if local support-store handling changes materially
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p03_ws03_t04_implement_local_secure_activation_commit_recovery_and_offline_validation_wiring.md`

## Implementation Scope
In scope:
- real shell-side secure-store token commit and retrieval for approved device-bound activation artifacts
- real bounded local activation support-record persistence and recovery in the app-controlled support domain
- current-device token validation using the approved device-binding model
- startup activation-readiness determination and truthful lifecycle transition mapping for `ActivationRequired`, `ConfigurationRequired`, `ReadyIdle`, and `ActivationRevoked`
- bounded offline post-activation validation for approved offline-capable scenarios only
- local recovery of committed activation state across launches
- shell-side tests and validation for commit, recovery, mismatch rejection, and readiness wiring
- truthful frontend placeholder updates if user-facing boundary text must change
- execution-state updates required to keep the packet honest

## Required Behavior to Preserve
- raw token material must remain inside OS-native secure storage and must not be mirrored into ordinary files, workspaces, logs, report artifacts, or generic config
- bounded non-secret support metadata must remain minimal and must not quietly accumulate entitlement history or run data
- activation readiness must remain separate from runtime readiness and provider readiness
- a token minted for one device must fail normal validation on a materially different unauthorized device
- activation success must require successful local commit, not merely backend approval
- offline capability must remain limited to approved post-activation validation and must not expand into offline simulation claims
- this packet must not implement provider configuration, Preflight, simulation, reporting, cleanup hardening, updater behavior, or commercial ledger behavior beyond what it consumes from Wave 3

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- commercial backend ledger or payment-event logic
- public API expansion or generic local-service expansion
- provider configuration, Preflight, simulation, reporting, cleanup, or updater behavior
- hidden persistence outside the frozen secure-store versus local-support split
- broader offline capability than governed post-activation validation

## Deliverables
- real shell-side secure-store activation commit and retrieval behavior exists
- real bounded local support-record persistence and recovery behavior exists
- current-device validation exists and rejects mismatch as normal licensed use
- startup and lifecycle truth can distinguish activation-required, configuration-required, ready-idle, and revoked states honestly
- approved offline post-activation validation works within governed limits

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm frontend gates remain clean if frontend files change
- confirm activation success requires successful local commit
- confirm a token minted for one device fails normal validation on another device
- confirm no raw token material is written into ordinary files, workspaces, or logs
- confirm offline validation remains bounded to approved post-activation scenarios

## Escalation Triggers
Escalate immediately if:
- local secure-store or support-record persistence cannot be implemented without violating the frozen storage-domain split
- the final local support storage engine cannot be narrowed without unauthorized persistence behavior
- offline validation semantics would need to exceed the governed product limits to remain coherent
- lifecycle truth would need to collapse activation readiness into runtime readiness or provider readiness
- any applicable shell or frontend gate reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Updated `shell/Cargo.toml` to add the bounded Wave 4 shell dependencies required for OS-native secure storage, support-record persistence, and local token validation; `shell/Cargo.lock` may also update as a dependency-resolution artifact of that bounded manifest change.
- Added `shell/src/activation_secure_store.rs`, `shell/src/activation_support_store.rs`, and `shell/src/activation_local_validation.rs` as the new Wave 4 local activation modules.
- Implemented shell-side secure-store token commit, retrieval, and rollback behavior using an OS-native secure-store abstraction while keeping raw device-bound token material out of support files and frontend access.
- Implemented bounded JSON support-record persistence and recovery in the app-controlled local support domain with schema version, license reference, activation reference, slot, device hash, signing lineage, fingerprint, validity marker, and last-validation status metadata only.
- Implemented bounded local activation validation that checks committed-token fingerprint consistency, signing lineage, device binding, expiration, and revoked-state handling and rejects normal licensed use on a materially different device.
- Wired the local validation output into activation readiness, startup checks, lifecycle state mapping, trust-boundary truth, local interface truth, and the shell command surface; `ActivationRevoked` is now represented explicitly.
- Refreshed `app/src/App.tsx` so the frontend placeholder no longer falsely claims that secure local activation persistence or local offline validation are absent.
- Preserved packet scope: no app-to-cloud activation request flow, no provider configuration, no Preflight, no simulation, no reporting, no updater behavior, and no commercial backend expansion beyond Wave 3 consumption were implemented.
- Validation passed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`.