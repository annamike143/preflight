# Task Packet

## Task ID
P03-WS03-T02

## Task Title
Establish Activation Contracts and Shell-Owned Local Artifact Boundary Surfaces

## Parent Phase
Phase 3 — Licensing, Activation, and Commercial Backend

## Parent Workstream
WS-03 — Licensing, Activation, and Commercial Backend

## Objective
Establish the code-level activation domain contracts, shell-owned activation command boundary, secure-store abstraction surface, bounded local activation support-record contract, and activation-readiness truth surface that later Phase 3 packets need, without pretending that full activation, backend, slot-enforcement, or offline-validation truth already exists.

## Why This Exists
`P03-W02` is the first implementation wave after the Phase 3 blueprint freeze. `P03-W03` and `P03-W04` both depend on a real contract and boundary layer for activation outcomes, storage-domain separation, and activation-readiness wiring. This packet creates that foundation first so later implementation does not invent request shapes, storage splits, or readiness semantics ad hoc.

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
- `execution/task_packets/p03_ws03_t01_establish_phase_3_activation_entry_basis_and_master_execution_blueprint.md` is complete.
- `execution/phase_03_master_execution_plan.md` exists and remains authoritative for Phase 3 wave order.
- No other Phase 3 implementation packet is active.
- Rust/Cargo/Clippy and frontend validation tooling remain available locally.

## Inputs / Required Context
- live Documents A, B, and C
- `execution/README.md`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/open_questions.md`
- `execution/live_execution_log.md`
- `execution/phase_03_master_execution_plan.md`
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/app_session_trust_boundary.rs`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/main.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/app_session_trust_boundary.rs`
- `shell/src/activation_contract.rs`
- `shell/src/activation_storage_boundary.rs`
- `shell/src/activation_readiness.rs`
- `app/src/App.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p03_ws03_t02_establish_activation_contracts_and_shell_owned_local_artifact_boundary_surfaces.md`

## Implementation Scope
In scope:
- shell-owned activation contract types covering activation request shape, activation approval or denial or technical-failure outcomes, denial classification, and bounded activation support-record structure
- shell-owned activation storage-boundary surface preserving the frozen split between OS-native secure-store token material and app-controlled local persistent support metadata
- shell-owned secure-store abstraction and frontend-to-shell activation command boundary surfaces without real backend or secure-store execution
- activation-readiness truth surface and startup/lifecycle integration that preserve canonical `ActivationRequired`, `ConfigurationRequired`, and `ReadyIdle` gating semantics honestly
- truthful frontend placeholder updates if frontend text or state surfacing must change to reflect the new activation contract and boundary scaffolds
- tests and execution-state updates required to keep the packet honest

## Required Behavior to Preserve
- raw device-bound token material must not be able to flow into ordinary files, workspaces, logs, or report artifacts by design
- activation readiness must stay separate from runtime readiness and provider readiness
- the shell must remain the privileged local authority for activation token handling and secure local state access
- the frontend must remain presentation/input only
- this packet must not silently choose the final local persistent support storage engine or the commercial backend framework
- this packet must not treat mocked approval paths as real activation truth

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- real app-to-cloud activation request handling
- backend commercial service, payment-event, or device-slot ledger implementation
- real JWT issuance, signing, or cryptographic local validation behavior
- real OS-native secure-store commit or retrieval behavior
- real local activation support-record persistence or recovery behavior
- real activation success, denial, revocation, or offline-validation behavior beyond bounded contract surfaces
- two-device slot enforcement behavior
- provider configuration, Preflight, simulation, reporting, cleanup, or updater behavior
- product-feature UI behavior beyond truthful placeholder updates

## Deliverables
- shell-owned activation contract surface exists
- shell-owned activation storage-boundary surface exists
- shell-owned secure-store abstraction surface exists
- activation-readiness truth surface exists and is wired into startup/lifecycle truth surfaces
- shell activation command boundary is explicit
- execution state reflects the active packet honestly

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm frontend gates remain clean if frontend files change
- confirm no real backend, secure-store persistence, offline-validation, or JWT-validation behavior was implemented
- confirm raw token material cannot flow into ordinary files, workspaces, or logs by design
- confirm activation readiness remains separate from runtime readiness and provider readiness

## Escalation Triggers
Escalate immediately if:
- the packet would require choosing the final local support storage engine or concrete backend framework to stay coherent
- the activation lifecycle mapping cannot remain consistent with canonical app-lifecycle gating
- the shell/frontend boundary cannot remain explicit without expanding scope
- raw token material cannot be kept out of ordinary persistence surfaces by design within the allowed scope
- any applicable shell or frontend gate still reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Added `shell/src/activation_contract.rs`, `shell/src/activation_storage_boundary.rs`, and `shell/src/activation_readiness.rs` as the new Wave 2 shell-owned activation foundation modules.
- Integrated the new activation surfaces into `shell/src/main.rs`, `shell/src/startup_checks.rs`, `shell/src/app_lifecycle.rs`, `shell/src/local_interface_contract.rs`, and `shell/src/app_session_trust_boundary.rs` so activation boundary truth is explicit across lifecycle, startup, trust, and command surfaces.
- Updated `app/src/App.tsx` so the frontend placeholder now reflects runtime-and-activation boundary truth instead of Phase 2-only startup wording.
- Preserved packet scope: no real app-to-cloud activation flow, no backend ledger behavior, no secure-store commit or retrieval behavior, no local support-record persistence, no JWT validation, and no offline validation behavior were implemented.
- Validation passed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build`.