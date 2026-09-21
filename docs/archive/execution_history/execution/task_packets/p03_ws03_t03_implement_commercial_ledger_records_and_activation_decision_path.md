# Task Packet

## Task ID
P03-WS03-T03

## Task Title
Implement Commercial Ledger Records and Activation Decision Path

## Parent Phase
Phase 3 — Licensing, Activation, and Commercial Backend

## Parent Workstream
WS-03 — Licensing, Activation, and Commercial Backend

## Objective
Implement the bounded commercial backend foundation needed for real license truth, device activation ledger truth, payment-event-to-entitlement translation support, token-issuance lineage tracking, and activation approval versus denial versus technical-failure decisions, without yet implementing local secure activation commit, local support-record persistence, or offline validation behavior on device.

## Why This Exists
`P03-W03` is the first phase packet that moves Phase 3 from shell-side contract scaffolding into real commercial truth. `P03-WS03-T02` already froze request and response shapes plus shell-owned storage boundaries. This packet now establishes the authoritative commercial ledger and activation-decision path that later local commit and offline-validation wiring will consume.

## Source-of-Truth References
- Document A Section 7 — State Machines
- Document A Section 8 — System Architecture
- Document A Section 9 — Data Lifecycle and Storage Policy
- Document A Section 10 — API and Interface Contract
- Document A Section 11 — Database and Commercial Backend Contract
- Document A Section 14 — Acceptance Criteria
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document B Section 3.6 — WS-03 — Licensing, Activation, and Commercial Backend
- Document B Section 4.7 — Workstream Dependency Matrix Logic
- Document B Section 6.5 — Phase 3 — Licensing, Activation, and Commercial Backend
- Document C Annex Family C — Data, Persistence, and Migration Pack
- `execution/phase_03_master_execution_plan.md`

## Dependency Preconditions
- `execution/task_packets/p03_ws03_t02_establish_activation_contracts_and_shell_owned_local_artifact_boundary_surfaces.md` is complete.
- No other packet is active.
- The bounded activation-artifact split remains frozen: raw token material belongs only in OS-native secure storage and bounded non-secret support data belongs only in the local persistent support domain.
- The concrete commercial backend framework or deployment surface may be narrowed inside this packet only if that narrowing is the minimum required to implement the governed ledger and activation-decision behavior truthfully.

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
- `shell/src/main.rs`

## Files / Modules Expected to Change
Create or update only:
- `commercial_backend/` bounded implementation files required for the chosen minimum backend surface
- `commercial_backend/src/` or equivalent source subtree for governed license, activation, audit, token-lineage, and activation-decision code
- `commercial_backend/migrations/` or equivalent deterministic schema-evolution subtree if the chosen backend surface requires migrations
- `commercial_backend/tests/` or equivalent backend test subtree
- `shell/src/activation_contract.rs` only if real backend response semantics require bounded contract refinement
- `shell/src/main.rs` only if shell-visible activation contract notes must be refreshed truthfully
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if unresolved backend-surface handling changes materially
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p03_ws03_t03_implement_commercial_ledger_records_and_activation_decision_path.md`

## Implementation Scope
In scope:
- minimum concrete commercial backend implementation surface required for governed ledger behavior
- durable license records with deterministic status, plan, expiration, and activation-limit handling
- durable device activation records with slot integrity and device-hash discipline
- payment-event-to-license translation support required for entitlement truth
- activation audit records and device-bound token issuance lineage or fingerprint tracking
- activation decision path that returns approval, denial, or technical failure truthfully
- activation slot enforcement and same-device reuse handling where governed
- bounded backend-facing tests and validation for the ledger and decision path
- execution-state updates required to keep the packet honest

## Required Behavior to Preserve
- the commercial backend must remain structurally separate from simulation execution and must not store seed documents, transcripts, run memory, or report semantics beyond governed entitlement support
- approval, denial, and technical failure must remain distinct end to end
- the backend must use device-hash or equivalent privacy-conscious binding rather than raw hardware identity as the preferred durable canonical activation record
- the two-device rule must remain hard unless upstream governance changes
- this packet must not implement local secure-store commit, local support-record persistence, local offline validation, provider configuration, Preflight, simulation, reporting, cleanup hardening, or updater behavior
- this packet must not imply that local licensed use is complete before Wave 4 lands

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- shell-side secure-store token commit or retrieval
- local activation support-record persistence or recovery
- local device-bound token validation or offline-validation behavior on device
- provider configuration, Preflight, simulation, reporting, cleanup, updater, or runtime-delivery behavior
- public API expansion beyond the minimum bounded commercial surface required by activation truth
- hidden analytics, vendor-side run storage, or hidden simulation persistence

## Deliverables
- real commercial ledger record families exist for licenses, activations, payment events, activation audit, and token issuance lineage or audit equivalent
- activation decision path exists and truthfully returns approval, denial, or technical failure
- slot-exhaustion denial and same-device reuse handling are implemented according to governed rules
- backend storage and migration surfaces remain bounded and governed
- shell activation contract stays aligned with real backend decision semantics if refinement is required

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm the backend-specific build, lint, and test gates for the chosen concrete backend surface run cleanly with zero warnings and zero errors
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` also run cleanly if shared shell files change
- confirm no simulation data enters the commercial backend
- confirm third-device activation is denied truthfully on the normal path
- confirm policy denial and technical failure remain distinct
- confirm no local secure-store commit, local support-record persistence, or offline-validation behavior was implemented in this packet

## Escalation Triggers
Escalate immediately if:
- truthful commercial ledger implementation requires architecture or persistence behavior not authorized by Documents A, B, or C
- the minimum concrete backend surface cannot be chosen without silently expanding product scope
- the packet would need local secure-store or offline-validation behavior to remain coherent
- payment-provider integration demands broader scope than bounded payment-event-to-entitlement translation support
- any applicable backend or shell gate reports warnings or errors after in-scope fixes

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Added `commercial_backend/pyproject.toml`, `commercial_backend/src/commercial_backend/`, `commercial_backend/migrations/`, and `commercial_backend/tests/` as the bounded Wave 3 commercial backend implementation surface.
- Implemented durable SQLite-backed commercial ledger records for licenses, payment events, device activations, activation audit entries, and token issuance lineage with deterministic schema migration handling.
- Implemented a bounded activation decision service that translates payment events into license truth, enforces the hard two-device limit, reuses the existing slot for the same device, distinguishes approval versus denial versus technical failure, and persists only token fingerprints rather than raw token material.
- Refined `shell/src/activation_contract.rs` so the shell-side approval contract now truthfully requires plan tier, device binding, issuance timestamp, signing lineage, and optional expiration claim semantics that match the backend-issued token payload.
- Preserved packet scope: no shell-side secure-store commit, no local support-record persistence or recovery, no offline validation on device, no provider configuration, and no public API or broader cloud surface were implemented.
- Validation passed cleanly with Python compile validation, `ruff`, `pytest`, `cargo check`, `cargo clippy -- -D warnings`, and `cargo test`.