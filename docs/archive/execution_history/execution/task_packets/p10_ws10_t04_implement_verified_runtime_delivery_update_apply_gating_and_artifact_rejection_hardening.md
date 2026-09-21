# Task Packet

## Task ID
P10-WS10-T04

## Task Title
Implement Verified Runtime Delivery, Update Apply Gating, and Artifact Rejection Hardening

## Parent Phase
Phase 10 — Updater, Runtime Delivery, and Distribution Hardening

## Parent Workstream
WS-10 — Updater, Distribution, and Runtime Delivery

## Objective
Implement the bounded Phase 10 Wave 4 path for managed runtime artifact authorization or acquisition, verification-before-use or apply, compatibility gating, rejected-artifact handling, and shell-owned update apply gating or restart coordination, without introducing hidden background mutation, hosted release-operations automation, or any redefinition of run, report, cleanup, or no-resume truth.

## Why This Exists
`P10-WS10-T03` completed real native app-updater discovery, truthful availability classification, and active-run-safe coordination, but the repo still lacks verified runtime delivery, runtime artifact authorization or download behavior, verification and compatibility enforcement before runtime use, truthful rejected-artifact handling, update download and verification progression through ready or rejected states, and shell-owned idle-only apply gating or restart coordination. The frozen Phase 10 plan requires this bounded Wave 4 implementation before the Phase 10 exit-gate packet can assess updater integrity, runtime integrity, or active-run non-mutation honestly. This packet activates that bounded Wave 4 scope while keeping Phase 10 validation and closure work for the later exit-gate packet.

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
- `execution/task_packets/p10_ws10_t02_establish_updater_lifecycle_runtime_delivery_contract_and_trust_surface_foundation.md` is COMPLETE.
- `execution/task_packets/p10_ws10_t03_implement_native_app_updater_discovery_availability_truth_and_safe_coordination.md` is COMPLETE.
- `execution/phase_10_master_execution_plan.md` remains the authoritative Phase 10 packetization basis.
- Existing native app-updater discovery, runtime package contract, runtime presence/integrity/hydration scaffolds, runtime readiness gating, and bounded frontend diagnostics surfaces remain available as the nearest repo-local pattern for truthful Wave 4 integration.
- Existing closed Phase 7 through Phase 9 lifecycle, report, cleanup, shutdown, startup-janitor, and no-resume surfaces remain stable and must not be redefined by runtime-delivery or update-apply hardening work.
- The current repo still lacks runtime artifact authorization or acquisition, runtime verification execution and compatibility enforcement, rejected-artifact handling, update download and verification progression into ready or rejected truth, and shell-owned apply gating or restart coordination behavior.
- Rust/Cargo/Clippy, frontend validation tooling, and the commercial-backend Python validation baseline remain available locally if touched.

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
- `execution/task_packets/p10_ws10_t03_implement_native_app_updater_discovery_availability_truth_and_safe_coordination.md`
- `execution/task_packets/p02_ws10_t01_establish_runtime_package_contract_and_shell_verification_basis.md`
- `shell/Cargo.toml`
- `shell/tauri.conf.json`
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/updater_contract.rs`
- `shell/src/native_app_updater.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/runtime_readiness.rs`
- `shell/src/runtime_delivery_contract.rs`
- `shell/src/runtime_package_contract.rs`
- `shell/src/runtime_presence.rs`
- `shell/src/runtime_integrity.rs`
- `shell/src/runtime_hydration.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave10.test.tsx`
- `commercial_backend/pyproject.toml`
- `commercial_backend/src/commercial_backend/__main__.py`
- `commercial_backend/src/commercial_backend/cli.py`
- `commercial_backend/src/commercial_backend/models.py`
- `commercial_backend/src/commercial_backend/service.py`
- `commercial_backend/src/commercial_backend/store.py`
- `commercial_backend/src/commercial_backend/migration_inventory.py`
- `commercial_backend/tests/conftest.py`
- `commercial_backend/tests/test_cli.py`

## Files / Modules Expected to Change
Create or update only:
- `shell/Cargo.toml`
- `shell/tauri.conf.json`
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/updater_contract.rs`
- `shell/src/native_app_updater.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/runtime_readiness.rs`
- `shell/src/runtime_delivery_contract.rs`
- `shell/src/runtime_package_contract.rs`
- `shell/src/runtime_presence.rs`
- `shell/src/runtime_integrity.rs`
- `shell/src/runtime_hydration.rs`
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave10.test.tsx`
- `commercial_backend/pyproject.toml`
- `commercial_backend/src/commercial_backend/__main__.py`
- `commercial_backend/src/commercial_backend/cli.py`
- `commercial_backend/src/commercial_backend/models.py`
- `commercial_backend/src/commercial_backend/service.py`
- `commercial_backend/src/commercial_backend/store.py`
- `commercial_backend/src/commercial_backend/migration_inventory.py`
- `commercial_backend/migrations/` bounded migration files only if runtime-delivery authorization persistence must change under this packet
- `commercial_backend/tests/conftest.py`
- `commercial_backend/tests/test_cli.py`
- `commercial_backend/tests/test_runtime_delivery.py`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved runtime-delivery or artifact-trust blockers change materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/phase_10_master_execution_plan.md`
- `execution/task_packets/p10_ws10_t04_implement_verified_runtime_delivery_update_apply_gating_and_artifact_rejection_hardening.md`

## Implementation Scope
In scope:
- shell-owned managed runtime artifact authorization or manifest retrieval from the bounded commercial-backend support surface where governed
- runtime artifact acquisition, hydration, verification, expected-file or manifest enforcement, compatibility gating, and rejection handling before runtime use is allowed
- native app-update artifact acquisition, verification, rejection handling, and truthful progression into `DownloadingUpdate`, `VerifyingUpdate`, `UpdateReady`, `UpdateRejected`, or `UpdaterError` where the shell can prove those outcomes honestly
- shell-owned idle-only update apply gating or restart coordination truth that keeps apply forbidden during active runs and never treats unverified artifacts as installable
- startup, lifecycle, readiness, local-interface, run-status, and bounded frontend or diagnostics integration needed to expose ready, rejected, blocked, and error truth honestly
- tests covering verification-before-use or apply, compatibility rejection, active-run mutation blocking, rejected-artifact persistence, and bounded update-ready or blocked-apply behavior where the repo can prove those outcomes deterministically

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for updater download or verification truth, runtime delivery orchestration, runtime compatibility gating, artifact rejection, apply or use gating, and blocked-action explanation
- keep the frontend as presentation-only, with no hidden authority over artifact trust, download policy, verification results, runtime usability, update installability, or restart safety
- keep commercial-backend participation bounded to governed authorization, manifest, or entitlement-adjacent support and never as the authority for local runtime readiness, run state, or filesystem mutation truth
- preserve the distinction between app-shell updater behavior and managed runtime-delivery behavior
- preserve the distinction among `UpdateAvailable`, `UpdateReady`, `UpdateRejected`, and `UpdaterError`
- preserve the rule that runtime readiness remains false until authorization, acquisition where required, expected-file verification, integrity verification, and compatibility checks all pass
- preserve active-run non-mutation for both runtime assets and update apply behavior
- preserve run lifecycle truth, report truth, cleanup truth, shutdown truth, startup-janitor truth, and no-resume truth without collapsing them into updater or runtime-delivery state

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- Phase 10 exit-gate acceptance mapping, closure decisioning, or packet-closeout work beyond what this packet needs for truthful implementation
- hosted signing automation, hosted release-operations automation, release-readiness signoff, or Phase 11 QA-matrix work
- hidden background auto-apply, silent restart, hidden runtime mutation, or any active-run asset mutation path
- simulation, report, cleanup, shutdown, janitor, or no-resume redesign
- commercial-backend expansion beyond the already-governed bounded support role for authorization or manifest-adjacent participation
- public API or OpenAPI behavior
- product-feature UI behavior beyond bounded shell-owned runtime-delivery and updater-truth rendering

## Deliverables
- managed runtime delivery exists under shell authority with bounded authorization or acquisition, verification-before-use, compatibility gating, and rejected-artifact handling
- native app-updater truth can progress honestly through download, verification, ready, rejected, blocked-apply, and technical-failure outcomes without claiming hidden authority in the frontend
- update apply or restart remains shell-owned, blocked during active runs, and gated on verified ready-state truth rather than discovery-only truth
- startup, lifecycle, status, readiness, local-interface, and bounded frontend surfaces expose runtime-delivery and updater hardening truth consistently
- the repo remains explicit that Phase 10 exit-gate validation and closure are deferred to the later packet

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm `python -m compileall commercial_backend/src commercial_backend/tests`, `python -m ruff check commercial_backend`, and `python -m pytest commercial_backend/tests` remain clean if `commercial_backend/` files change
- confirm runtime artifacts do not become ready or usable until authorization, acquisition where required, integrity or manifest verification, and compatibility checks pass
- confirm unsafe or incompatible update and runtime artifacts classify as rejected or error truthfully and remain non-installable or non-usable
- confirm update apply and runtime mutation remain blocked during active runs and do not override run/report/cleanup truth
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful runtime authorization, manifest retrieval, acquisition, verification, or compatibility enforcement cannot be enabled without choosing an unresolved upstream trust, feed, signing, or distribution mechanism
- bounded commercial-backend support would need to become the authority for local runtime readiness, local filesystem mutation, or hidden update policy
- any touched shell, backend, config, or frontend surface would blur `UpdateReady` into `UpdateAvailable`, blur `UpdateRejected` into `UpdaterError`, or otherwise violate Document A Section 7.10 or the Phase 10 verification-before-use/apply rules
- runtime delivery or update apply gating would require redefining run lifecycle truth, report truth, cleanup truth, shutdown truth, or no-resume preservation
- any applicable shell, frontend, or commercial-backend gate reports warnings or errors after in-scope remediation

## Activation Notes
- Authored and activated immediately as the current executable Phase 10 Wave 4 packet for verified runtime delivery, update apply gating, and artifact rejection hardening over the completed Wave 3 discovery and availability basis.
- Repo changes are authorized only within this packet's allowlist and bounded Wave 4 scope until closeout or blockage.
- The later Phase 10 exit-gate packet remains inactive while this packet is implementing and validating Wave 4 behavior.

## Implementation Progress
- The first shell-only slice of this active packet is now materialized in product code across `shell/src/runtime_package_contract.rs`, `shell/src/runtime_hydration.rs`, `shell/src/runtime_presence.rs`, `shell/src/runtime_integrity.rs`, `shell/src/runtime_delivery_contract.rs`, `shell/src/runtime_readiness.rs`, `shell/src/startup_checks.rs`, `shell/src/app_lifecycle.rs`, `shell/src/local_interface_contract.rs`, `shell/src/native_app_updater.rs`, `shell/src/updater_contract.rs`, `shell/src/run_status_contract.rs`, and `shell/src/main.rs`.
- That slice persists runtime installation and rejected-candidate metadata under the app-controlled support location, hydrates runtime artifacts from the `MIRO_FISH_RUNTIME_DELIVERY_MANIFEST` allowlist, verifies expected files and compatibility before runtime use, propagates hydration-required versus blocked runtime truth through readiness and lifecycle surfaces, and exposes idle-only updater apply-gate truth without mutating active runs.
- The packet critique remediation pass is now also materialized in that same shell slice: runtime installs restore the previous package if metadata commit fails after file movement, trust metadata must match the installed package instead of merely existing, runtime-delivery authorization truth now depends on validated trust rather than trust-file presence, and hydration-required truth stays anchored to actual presence and integrity rather than metadata alone.
- Validation for the current shell-only slice completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` after one ownership fix in `shell/src/runtime_hydration.rs`, one apply-gate exposure cleanup in `shell/src/main.rs`, one redundant-closure Clippy cleanup, one runtime-presence semantics fix that restored determined missing-state truth, and the later critique-remediation rerun that added rollback and stale-trust regression coverage and finished with 136 passing shell tests.
- The later updater-preparation slice is now also materialized across `shell/src/native_app_updater.rs`, `shell/src/updater_contract.rs`, `shell/src/local_interface_contract.rs`, `shell/src/main.rs`, `app/src/App.tsx`, and `app/src/App.wave10.test.tsx`: discovery remains separate from ready-state truth, `prepare_native_app_updater` now performs real signed download-plus-verification progression into `UpdateReady`, `UpdateRejected`, or `UpdaterError`, and the diagnostics screen now exposes bounded refresh-versus-prepare actions, ready/rejected/error truth, release metadata, and apply-gate notes.
- The later bounded commercial-backend support slice is now materialized across `commercial_backend/src/commercial_backend/models.py`, `commercial_backend/src/commercial_backend/service.py`, `commercial_backend/src/commercial_backend/cli.py`, `commercial_backend/tests/test_cli.py`, `commercial_backend/tests/test_runtime_delivery.py`, and `shell/src/runtime_hydration.rs`: the repo-local backend now exposes `resolve-runtime-delivery-manifest`, validates license usability before returning an authorized manifest reference plus fingerprint, and the shell can fall back to that backend-managed manifest resolution when direct runtime-manifest configuration is absent.
- Validation for the newer updater/frontend/backend slice completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `python -m compileall commercial_backend/src commercial_backend/tests`, `python -m ruff check commercial_backend`, and `python -m pytest commercial_backend/tests`.
- The final Wave 4 updater-apply slice is now materialized across `shell/src/native_app_updater.rs`, `shell/src/updater_contract.rs`, `shell/src/local_interface_contract.rs`, `shell/src/main.rs`, `app/src/App.tsx`, and `app/src/App.wave10.test.tsx`: the shell now stages verified `UpdateReady` artifacts in memory, exposes `apply_native_app_updater` as the explicit install path, transitions truthfully through `ApplyingUpdate`, `UpdateApplied`, or `UpdaterError`, requests app restart after a successful install, and surfaces bounded diagnostics truth for refresh, prepare, and apply actions plus restart-request status.
- Final validation reran cleanly after the updater-apply slice with `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `python -m compileall commercial_backend/src commercial_backend/tests`, `python -m ruff check commercial_backend`, and `python -m pytest commercial_backend/tests`.

## Completion Decision
COMPLETE — 2026-04-17

## Completion Notes
- Completed on 2026-04-17 after materializing verified runtime delivery, runtime artifact rejection hardening, bounded backend manifest authorization support, real shell-owned updater preparation, and the final shell-owned updater apply plus restart-request path over verified `UpdateReady` state.
- The final Wave 4 validation pass completed cleanly across shell, frontend, and commercial-backend gates: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`, `python -m compileall commercial_backend/src commercial_backend/tests`, `python -m ruff check commercial_backend`, and `python -m pytest commercial_backend/tests` all passed without warnings or errors.
- Because this workspace is not a git repository, the closeout changed-file audit used the applied patch set plus targeted searches; the touched files stayed within this packet's allowlist.
- The closeout critique and contradiction sweep found no material in-scope defect or execution-surface inconsistency across `execution/progress.md`, `execution/current_constraints.md`, `execution/current_repo_truth.md`, `execution/packet_registry.md`, `execution/live_execution_log.md`, `execution/phase_10_master_execution_plan.md`, and this packet.
- Phase 10 returned to a between-packets posture at closeout; `P10-WS11-T01` is now authored as the prepared successor and is not active.