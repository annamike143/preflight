# Task Packet

## Task ID
P03-WS11-T03

## Task Title
Execute Phase 3 Revalidation and Closure Decision

## Parent Phase
Phase 3 — Licensing, Activation, and Commercial Backend

## Parent Workstream
WS-11 — Validation Recovery and Exit Gates

## Objective
Re-run the bounded Phase 3 exit-gate validation, acceptance mapping, critique pass, and closure decision after `P03-WS11-T02` implemented the missing end-to-end activation request flow so Phase 3 can now either close honestly or remain open for a newly explicit blocking reason.

## Why This Exists
`P03-WS11-T01` denied closure because the repo lacked a real activation request path. `P03-WS11-T02` implemented that path and passed the shell, frontend, and backend gates. This packet now determines whether the recovered repo truth is sufficient to close Phase 3 without inventing success.

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
- `execution/task_packets/p03_ws11_t02_implement_end_to_end_activation_request_flow.md`

## Dependency Preconditions
- `execution/task_packets/p03_ws11_t02_implement_end_to_end_activation_request_flow.md` is complete.
- No other packet may remain active while this packet is active.
- The frozen activation-artifact split remains unchanged: raw device-bound token material belongs only in OS-native secure storage and bounded non-secret support metadata belongs only in the local persistent support domain.

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
- `execution/task_packets/p03_ws11_t01_execute_phase_3_validation_critique_and_exit_gate.md`
- `execution/task_packets/p03_ws11_t02_implement_end_to_end_activation_request_flow.md`
- `shell/src/activation_contract.rs`
- `shell/src/activation_submission.rs`
- `shell/src/activation_secure_store.rs`
- `shell/src/activation_support_store.rs`
- `shell/src/activation_local_validation.rs`
- `shell/src/activation_readiness.rs`
- `shell/src/main.rs`
- `commercial_backend/src/commercial_backend/`
- `commercial_backend/tests/`
- `app/src/App.tsx`

## Files / Modules Expected to Change
Create or update only:
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/open_questions.md` only if unresolved handling changes materially after revalidation
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/decisions/decision_log.md` if the closure decision requires a real local governance decision
- `execution/task_packets/p03_ws11_t03_execute_phase_3_revalidation_and_closure_decision.md`
- `shell/src/` files only if the critique pass finds a material in-scope defect in the recovered Phase 3 activation surfaces
- `commercial_backend/src/` or `commercial_backend/tests/` files only if the critique pass finds a material in-scope defect in the recovered commercial-backend surfaces
- `app/src/App.tsx` only if the critique pass finds a truthful UI contradiction

## Implementation Scope
In scope:
- updated acceptance mapping for `AC-011` through `AC-018` and `AC-121` through `AC-126`
- rerunning the applicable shell, frontend, and backend validation gates for the recovered Phase 3 surfaces
- explicit evidence review for approval, denial, and technical failure through the new activation request path
- critique pass over the touched recovery implementation and execution-state files
- tightly bounded in-scope remediation if the critique pass finds a real defect in the recovered Phase 3 surfaces
- explicit Phase 3 closure decision and execution-state updates

## Required Behavior to Preserve
- the packet must not treat stale Wave 5 denial truth as if it still governs after the recovery packet changed the repo
- the packet must not invent closure if the new activation-flow evidence is still incomplete or contradictory
- approval, denial, technical failure, revoked, and activation-required outcomes must remain distinct
- raw token material must remain only in OS-native secure storage
- bounded local support data must remain minimal and non-secret
- the commercial backend must remain bounded and must not become a simulation archive

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- new product scope beyond revalidation and in-scope remediation
- provider configuration, Preflight, simulation, reporting, cleanup hardening, or updater behavior
- new persistence domains or hidden execution-state surfaces
- broader app-to-cloud feature work beyond what `P03-WS11-T02` already implemented

## Deliverables
- updated acceptance mapping exists for the governed Phase 3 criteria
- all applicable Phase 3 gates are rerun truthfully after the recovery packet
- critique findings are either fixed in-scope or escalated explicitly
- the repo records a truthful Phase 3 closure decision

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
  - `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall commercial_backend/src commercial_backend/tests`
  - `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check commercial_backend`
  - `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest commercial_backend`
- confirm approval, denial, and technical-failure evidence now exists through the real activation request path
- confirm activation success still requires successful local commit rather than backend approval alone
- confirm the commercial backend does not persist simulation transcripts, seed documents, or run memory
- confirm whether Phase 3 can close truthfully and record the result explicitly

## Escalation Triggers
Escalate immediately if:
- any governed Phase 3 acceptance target still lacks real evidence after the recovery packet
- any applicable gate reports an error or warning after in-scope remediation attempts
- the critique pass reveals that the recovered activation flow still depends on mock-only or manually injected truth
- closing Phase 3 would require ignoring a remaining contradiction in the recovered activation path

## Completion Decision
COMPLETE — 2026-04-15

## Completion Notes
- Re-ran the full Phase 3 shell, frontend, and backend gate set cleanly after the recovery packet: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m compileall commercial_backend/src commercial_backend/tests`, `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m ruff check commercial_backend`, and `d:/PROJECTS/miro-fish-desktop-saas/.venv/Scripts/python.exe -m pytest commercial_backend`.
- Confirmed real approval, denial, and technical-failure evidence through the recovered activation request path via the `activation_submission` shell tests and the bounded commercial-backend CLI tests.
- Confirmed activation success still requires successful local secure commit rather than backend approval alone and that raw token material still remains out of ordinary files, frontend state, logs, and support metadata.
- Phase 3 now closes honestly because the Wave 5 blocker is resolved and the updated acceptance mapping is fully proven.

## Acceptance Mapping
- `AC-011` — PROVEN by `activation_submission::tests::repo_local_python_backend_round_trip_supports_approval`, which drives a valid entitlement through backend approval and successful local activation commit.
- `AC-012` — PROVEN by the Wave 3 device-bound token issuance path together with the Wave 4 and Wave 5 local validation and recovered activation submission tests.
- `AC-013` — PROVEN by the Wave 3 backend activation-slot enforcement tests.
- `AC-014` — PROVEN by `test_third_device_activation_is_denied_truthfully` in `commercial_backend/tests/test_activation_service.py`.
- `AC-015` — PROVEN by the Wave 4 local commit and offline-validation tests plus the recovered shell submission path that now commits approved activations locally.
- `AC-016` — PROVEN by `validation_rejects_token_on_different_device` in `shell/src/activation_local_validation.rs` test coverage.
- `AC-017` — PROVEN by `repo_local_python_backend_round_trip_supports_approval`, `repo_local_python_backend_round_trip_supports_denial`, `repo_local_python_backend_round_trip_supports_technical_failure`, the bounded CLI tests in `commercial_backend/tests/test_cli.py`, and the frontend activation workflow in `app/src/App.tsx` invoking the shell-owned `submit_device_activation` path.
- `AC-018` — PROVEN by the Wave 4 secure-store commit, support-record persistence, rollback-discipline tests, and the recovered end-to-end approval path that reports technical failure when local commit does not succeed.
- `AC-121` — PROVEN by the durable `licenses` table and Wave 3 entitlement-translation behavior.
- `AC-122` — PROVEN by the durable `device_activations` table and Wave 3 activation ledger behavior.
- `AC-123` — PROVEN by the `payment_events` table plus the idempotent payment-event translation tests and the new bounded CLI bridge that consumes the same governed backend surface.
- `AC-124` — PROVEN by the hard two-device enforcement tests in the Wave 3 backend suite.
- `AC-125` — PROVEN by the JWT device-binding claim contract plus Wave 4 current-device validation logic and the recovered shell submission path.
- `AC-126` — PROVEN by the bounded commercial backend schema, which stores only commercial ledger records and token lineage artifacts and no simulation transcripts, seed documents, or run memory.