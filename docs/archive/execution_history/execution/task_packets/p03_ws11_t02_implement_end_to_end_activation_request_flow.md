# Task Packet

## Task ID
P03-WS11-T02

## Task Title
Implement End-to-End Activation Request Flow

## Parent Phase
Phase 3 — Licensing, Activation, and Commercial Backend

## Parent Workstream
WS-11 — Validation Recovery and Exit-Gate Remediation

## Objective
Implement the missing shell-owned activation submission path that accepts frontend activation input, derives the approved device-binding representation, invokes the bounded repo-local commercial backend activation interface, preserves approval versus denial versus technical-failure truth, and counts activation as successful only after the existing secure local commit succeeds.

## Why This Exists
`P03-WS11-T01` reran the full Phase 3 gate set cleanly but denied closure because the repo still lacked a real activation request path joining the frontend or shell boundary to the Wave 3 commercial backend decision surface and the Wave 4 local secure activation commit path. This packet exists to close that exact contradiction without inventing hosted deployment, public API, payment-provider ingress, or broader product scope.

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
- `execution/task_packets/p03_ws11_t01_execute_phase_3_validation_critique_and_exit_gate.md`

## Dependency Preconditions
- `execution/task_packets/p03_ws03_t03_implement_commercial_ledger_records_and_activation_decision_path.md` is complete.
- `execution/task_packets/p03_ws03_t04_implement_local_secure_activation_commit_recovery_and_offline_validation_wiring.md` is complete.
- `execution/task_packets/p03_ws11_t01_execute_phase_3_validation_critique_and_exit_gate.md` is complete and remains the authoritative reason this recovery packet exists.
- No other packet may remain active while this packet is active.
- The frozen activation-artifact split remains unchanged: raw device-bound token material belongs only in OS-native secure storage and bounded non-secret support metadata belongs only in the app-controlled local persistent support domain.

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
- `execution/task_packets/p03_ws03_t03_implement_commercial_ledger_records_and_activation_decision_path.md`
- `execution/task_packets/p03_ws03_t04_implement_local_secure_activation_commit_recovery_and_offline_validation_wiring.md`
- `execution/task_packets/p03_ws11_t01_execute_phase_3_validation_critique_and_exit_gate.md`
- `shell/src/activation_contract.rs`
- `shell/src/activation_local_validation.rs`
- `shell/src/activation_readiness.rs`
- `shell/src/activation_secure_store.rs`
- `shell/src/activation_support_store.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/main.rs`
- `commercial_backend/src/commercial_backend/`
- `commercial_backend/tests/`
- `app/package.json`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `shell/src/activation_contract.rs` only if bounded activation outcome or round-trip semantics require truthful refinement
- `shell/src/activation_local_validation.rs` only if bounded pre-commit approval-payload validation helpers are required by the end-to-end submission flow
- `shell/src/local_interface_contract.rs`
- `shell/src/main.rs`
- `shell/src/activation_submission.rs`
- `commercial_backend/src/commercial_backend/__main__.py`
- `commercial_backend/src/commercial_backend/cli.py`
- `commercial_backend/tests/test_cli.py`
- `app/package.json`
- `app/package-lock.json`
- `app/src/App.tsx`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if a real unresolved boundary question appears during implementation
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p03_ws11_t02_implement_end_to_end_activation_request_flow.md`

## Implementation Scope
In scope:
- a shell-owned activation submission command that accepts activation input from the frontend and derives the approved device-binding representation locally
- a bounded repo-local commercial-backend command interface over the existing Wave 3 activation decision surface
- truthful approval versus denial versus technical-failure outcome propagation from the backend through the shell boundary
- successful local activation only after the existing secure token commit and bounded support-record commit both succeed
- bounded frontend activation workflow updates sufficient to exercise the real shell activation command and render truthful outcomes
- shell-side and backend-side tests for the new activation request path
- execution-state updates required to keep the packet honest

## Required Behavior to Preserve
- raw token material must remain inside OS-native secure storage and must not be mirrored into ordinary files, workspaces, logs, report artifacts, or generic config
- bounded non-secret support metadata must remain minimal and must not quietly accumulate entitlement history, payment-event history, or run data
- approval, denial, and technical failure must remain distinct through the activation request round trip
- activation success must require successful local commit rather than backend approval alone
- activation readiness must remain separate from runtime readiness and provider readiness
- the frontend must remain a presentation-and-input surface and must not own secure storage or activation validation authority
- this packet must not silently choose a final hosted backend deployment or public API surface

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- public API or OpenAPI expansion
- hosted cloud deployment selection
- payment-provider ingress beyond consuming the already bounded payment-event translation surface where tests or repo-local setup require it
- provider configuration, Preflight, simulation, reporting, cleanup hardening, updater behavior, or runtime-delivery behavior
- hidden persistence domains outside the already governed commercial-backend ledger, OS-native secure storage, and bounded local support-record domains
- broader product UI scope than the minimum truthful activation workflow needed to exercise the new path

## Deliverables
- a real shell-owned activation submission path exists
- the repo-local commercial backend exposes a bounded command interface for activation evaluation
- approval, denial, and technical failure now surface through a real activation workflow rather than contract-only scaffolding
- successful activation now requires backend approval plus successful local secure commit in one product-owned path
- execution state reflects the packet lifecycle honestly

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` complete with zero warnings and zero errors if frontend files or dependencies change
- confirm `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall commercial_backend/src commercial_backend/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check commercial_backend`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest commercial_backend` complete cleanly
- prove one approval path, one denial path, and one technical-failure path through the new activation request flow
- confirm no raw token material is written to ordinary files, workspaces, logs, or frontend durable state
- confirm the packet does not choose a final hosted deployment or broader public API surface

## Escalation Triggers
Escalate immediately if:
- closing the activation-flow gap requires inventing a public API, hosted deployment, or broader backend framework choice not authorized by Documents A, B, or C
- the shell cannot consume the existing Wave 3 activation decision surface without violating the frozen activation-artifact split
- truthful approval versus denial versus technical-failure mapping cannot be preserved through the new command boundary
- any applicable shell, frontend, or backend gate reports warnings or errors after in-scope remediation attempts

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Implemented `shell/src/activation_submission.rs` plus a new `submit_device_activation` shell command that derives the current device binding, invokes the bounded repo-local commercial backend activation interface, and returns approval versus denial versus technical-failure truth without exposing raw token material to the frontend.
- Added `commercial_backend/src/commercial_backend/cli.py` and `commercial_backend/src/commercial_backend/__main__.py` as the bounded repo-local command interface over the existing Wave 3 payment-event and activation-decision surfaces without selecting a hosted deployment or public API surface.
- Updated `app/src/App.tsx` into a bounded activation workflow and added `@tauri-apps/api` so the frontend now exercises the real shell-owned activation path while remaining presentation and input only.
- Added shell and backend tests proving approval, denial, and technical failure through the recovered activation request path, including real shell-to-Python-commercial-backend round trips.
- Validation passed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall commercial_backend/src commercial_backend/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check commercial_backend`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest commercial_backend`.