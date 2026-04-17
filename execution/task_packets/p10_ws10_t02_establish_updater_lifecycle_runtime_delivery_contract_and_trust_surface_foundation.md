# Task Packet

## Task ID
P10-WS10-T02

## Task Title
Establish Updater Lifecycle, Runtime Delivery Contract, and Trust Surface Foundation

## Parent Phase
Phase 10 — Updater, Runtime Delivery, and Distribution Hardening

## Parent Workstream
WS-10 — Updater, Distribution, and Runtime Delivery

## Objective
Define the code-level updater lifecycle contract surfaces, runtime-delivery contract surfaces, verification-before-apply or use semantics, and shell-owned truth surfaces needed for later Phase 10 implementation, without yet claiming real update discovery, artifact download, verification execution, or apply or use behavior.

## Why This Exists
`P10-WS10-T01` froze the bounded Phase 10 path and wave order, but the repo still lacks explicit contract modules for canonical updater states, update-availability and update-readiness distinctions, trust-based rejection versus technical-failure meaning, runtime-delivery authorization and verification basis, and shell-owned truth surfaces that preserve the boundary between app-shell updates and managed runtime delivery. Phase 10 Wave 3 cannot implement real native app-updater discovery honestly, and Phase 10 Wave 4 cannot implement verified runtime delivery and apply or use gating honestly, until Wave 2 first establishes the governing vocabulary and trust-surface basis in code while preserving the distinction between updater state, runtime readiness, run lifecycle truth, report truth, and cleanup truth.

## Source-of-Truth References
- Document A Section 1 — Project Constitution
- Document A Section 3 — Core Terms and Definitions
- Document A Section 7.10 — Updater Lifecycle
- Document A Section 8 — Runtime Distribution Architecture
- Document A Section 10 — Commercial/Update Interface Rules
- Document A Section 11 — Runtime Distribution/Update Support Contract
- Document A Section 12 — UI/UX Contract
- Document A Section 13 — Runtime/Update Integrity Requirements
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.13 — WS-10 — Updater, Distribution, and Runtime Delivery
- Document B Section 4.6 — Dependency Chains
- Document B Section 5.13 — Phase 10 — Updater, Runtime Delivery, and Distribution Hardening
- Document B Section 6.12 — Phase 10 — Updater, Runtime Delivery, and Distribution Hardening
- Document C Annex Family A — Interface and API Contract Pack
- Document C Annex Family B — Event, State, and Runtime Contract Pack
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_10_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p10_ws10_t01_establish_phase_10_updater_runtime_delivery_and_distribution_hardening_entry_basis_and_master_execution_blueprint.md` is COMPLETE.
- `execution/phase_10_master_execution_plan.md` remains the authoritative Phase 10 packetization basis.
- Existing runtime package contract, runtime presence, runtime integrity, runtime readiness, and startup-check surfaces remain available as the nearest repo-local pattern for runtime-delivery contract foundation work.
- Existing shell-owned local-interface and frontend truth surfaces remain available as the nearest repo-local pattern for bounded updater/runtime-delivery exposure without frontend authority.
- Existing closed Phase 7 through Phase 9 lifecycle, report, cleanup, shutdown, and no-resume surfaces remain stable and must not be redefined by Phase 10 contract work.
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
- `execution/phase_10_master_execution_plan.md`
- `execution/task_packets/p02_ws10_t01_establish_runtime_package_contract_and_shell_verification_basis.md`
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/runtime_package_contract.rs`
- `shell/src/runtime_presence.rs`
- `shell/src/runtime_integrity.rs`
- `shell/src/runtime_readiness.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/runtime_package_contract.rs`
- `shell/src/runtime_presence.rs`
- `shell/src/runtime_integrity.rs`
- `shell/src/runtime_readiness.rs`
- `shell/src/updater_contract.rs`
- `shell/src/runtime_delivery_contract.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave10.test.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved updater/runtime contract or trust-surface blockers change materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p10_ws10_t02_establish_updater_lifecycle_runtime_delivery_contract_and_trust_surface_foundation.md`

## Implementation Scope
In scope:
- shell-owned updater contract types for canonical updater states, allowed transition meaning, update availability, update readiness, trust-based rejection, and technical failure without claiming real update-check, download, verification, or apply execution yet
- shell-owned runtime-delivery contract types for authorization basis, artifact-verification basis, compatibility gating basis, runtime usability blocking, and active-run non-mutation requirements without claiming real runtime delivery execution yet
- runtime package, runtime presence, runtime integrity, runtime readiness, startup-check, local-interface, command-note, and bounded frontend integration needed to expose the new Phase 10 contract-foundation truth honestly
- tests for the new updater/runtime-delivery contract and shell/frontend truth surfaces

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for updater-state truth, runtime-delivery truth, artifact verification meaning, installability or usability gating, and active-run mutation protection
- keep the frontend as presentation and input only, with no hidden authority over update availability, update trust, runtime trust, apply safety, or local artifact verification
- preserve the distinction between app-shell update behavior and managed runtime-delivery behavior
- preserve the distinction between `UpdateAvailable`, `UpdateReady`, `UpdateRejected`, and `UpdaterError`
- preserve runtime readiness as separate from updater state while still allowing later runtime-delivery truth to feed it honestly
- preserve the frozen runtime/update split, verification-before-apply or use rule, and active-run non-mutation rule
- preserve Phase 7 through Phase 9 run-state, report, cleanup, shutdown, and no-resume truth without reclassification or semantic collapse
- do not overstate contract or truth-surface existence as proof that real discovery, download, verification execution, apply behavior, or runtime hydration behavior already exists

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- real native app-updater discovery, download, verification, apply, or restart behavior
- real runtime artifact authorization, download, hydration, repair, verification execution, or apply/use behavior
- release-readiness signoff or distribution-operations automation
- simulation, report, cleanup, shutdown, or janitor redesign
- commercial-backend expansion beyond already-governed authorization/support participation
- public API or OpenAPI behavior
- product-feature UI behavior beyond truthful contract-foundation rendering

## Deliverables
- `shell/src/updater_contract.rs` exists and defines the bounded updater lifecycle and trust-result contract surface
- `shell/src/runtime_delivery_contract.rs` exists and defines the bounded runtime-delivery authorization, verification, compatibility, and active-run-safety contract surface
- shell startup, local-interface, runtime, and command surfaces expose the new Phase 10 contract-foundation truth honestly
- the frontend, if touched, reports bounded shell-owned updater/runtime-delivery contract truth without implying real discovery, download, verification execution, or apply behavior

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm no real update discovery, update download, update verification execution, update apply, runtime delivery, runtime hydration, or runtime repair behavior was implemented
- confirm the delivered contracts distinguish app-shell updater state, runtime-delivery state, runtime readiness, and active-run protection without collapsing them into one state family
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful updater or runtime-delivery contract surfaces cannot be defined without changing Documents A, B, or C
- the packet would require real discovery, download, verification execution, or apply/use behavior to remain coherent
- runtime readiness integration would collapse updater state into runtime readiness or active-run lifecycle truth
- any touched shell or frontend surface would weaken the frozen runtime/update split, verification-before-apply or use rule, or active-run non-mutation rule

## Activation Notes
- Authored and activated immediately as the current executable Phase 10 Wave 2 packet for updater lifecycle, runtime-delivery contract, and trust-surface foundation work.
- Repo changes are authorized only within this packet's allowlist and bounded contract-foundation scope until closeout or blockage.
- Real app-update discovery/download/apply behavior and real runtime artifact delivery/verification/apply behavior remain out of scope while this packet is active.

## Completion Decision
COMPLETE — 2026-04-17

## Completion Notes
- Added `shell/src/updater_contract.rs` and `shell/src/runtime_delivery_contract.rs` so the repo now has explicit Phase 10 Wave 2 contract modules for canonical updater vocabulary, runtime-delivery authorization/verification/compatibility meaning, trust-rejection versus technical-failure separation, and active-run non-mutation rules without claiming real discovery/download/apply behavior.
- Integrated the new Phase 10 contract truth through `shell/src/main.rs`, `shell/src/startup_checks.rs`, `shell/src/local_interface_contract.rs`, `shell/src/runtime_package_contract.rs`, `shell/src/runtime_presence.rs`, `shell/src/runtime_integrity.rs`, and `shell/src/runtime_readiness.rs` so updater state, runtime-delivery meaning, runtime readiness, and shell-owned authority stay explicit and non-collapsed.
- Updated `app/src/App.tsx` plus new regression file `app/src/App.wave10.test.tsx` so the diagnostics screen renders the shell-owned updater/runtime-delivery contract foundation honestly without implying real update discovery, runtime acquisition, verification execution, or apply behavior.
- Validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`; no real native app-updater discovery/download/apply behavior and no real runtime authorization/download/verification/apply behavior were implemented under this packet.

## Structural Consistency and Contradiction Review
- Reviewed the touched product-code surfaces together with `execution/progress.md`, `execution/current_constraints.md`, `execution/current_repo_truth.md`, `execution/packet_registry.md`, `execution/live_execution_log.md`, and `execution/phase_10_master_execution_plan.md` against the packet scope and the governing Phase 10 rules in Documents A, B, and C.
- Confirmed the delivered updater contract preserves the canonical `UpdaterIdle` through `UpdaterError` vocabulary, keeps `UpdateAvailable`, `UpdateReady`, `UpdateRejected`, and `UpdaterError` distinct, and does not overclaim real check/download/verification/apply behavior.
- Confirmed the delivered runtime-delivery contract preserves the frozen runtime/update split, verification-before-use meaning, runtime-readiness separation, shell-owned authority, and the ban on active-run mutation without collapsing runtime-delivery meaning into run lifecycle, cleanup truth, or app-shell updater state.
- Confirmed the frontend diagnostics rendering remains presentation-only and shell-owned truth remains authoritative for updater state, runtime-delivery meaning, verification basis, and active-run protection.
- No contradiction, hidden-scope expansion, warning-bearing validation failure, or unresolved blocker remained after review, so the packet is closed honestly.