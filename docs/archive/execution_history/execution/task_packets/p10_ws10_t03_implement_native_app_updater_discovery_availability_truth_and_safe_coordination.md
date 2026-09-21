# Task Packet

## Task ID
P10-WS10-T03

## Task Title
Implement Native App-Updater Discovery, Availability Truth, and Safe Coordination

## Parent Phase
Phase 10 — Updater, Runtime Delivery, and Distribution Hardening

## Parent Workstream
WS-10 — Updater, Distribution, and Runtime Delivery

## Objective
Implement the bounded Phase 10 Wave 3 path for real native app-updater discovery, truthful availability classification, and active-run-safe coordination, without yet claiming verified update readiness, update apply behavior, restart coordination, or managed runtime-delivery execution.

## Why This Exists
`P10-WS10-T02` completed the updater lifecycle and runtime-delivery contract foundation, but the repo still lacks any real native app-updater discovery path, any truthful shell-owned transition through `CheckingForUpdates`, `NoUpdateAvailable`, `UpdateAvailable`, or `UpdaterError`, and any bounded safe-coordination behavior that keeps discovery separate from active-run lifecycle truth before later apply behavior exists. Phase 10 Wave 4 cannot implement update apply gating, artifact rejection hardening, or managed runtime-delivery execution honestly until Wave 3 first materializes real discovery and truthful availability outcomes over the completed Wave 2 contract basis. This packet activates that bounded Wave 3 scope while keeping download, verification, apply, restart, and runtime-delivery execution for later packets.

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
- `execution/phase_10_master_execution_plan.md` remains the authoritative Phase 10 packetization basis.
- Existing updater contract, startup-check, local-interface, app-lifecycle, runtime-readiness, and bounded frontend diagnostics surfaces remain available as the nearest repo-local pattern for truthful updater discovery integration.
- Existing closed Phase 7 through Phase 9 lifecycle, report, cleanup, shutdown, and no-resume surfaces remain stable and must not be redefined by updater discovery work.
- The current repo still lacks native updater dependency/config integration, real update-check execution, truthful no-update versus update-available classification, and active-run-safe updater coordination behavior.
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
- `execution/task_packets/p10_ws10_t02_establish_updater_lifecycle_runtime_delivery_contract_and_trust_surface_foundation.md`
- `shell/Cargo.toml`
- `shell/tauri.conf.json`
- `shell/src/main.rs`
- `shell/src/startup_checks.rs`
- `shell/src/local_interface_contract.rs`
- `shell/src/updater_contract.rs`
- `shell/src/app_lifecycle.rs`
- `shell/src/run_lifecycle_state.rs`
- `shell/src/run_status_contract.rs`
- `shell/src/run_status_projection.rs`
- `shell/src/runtime_readiness.rs`
- `app/src/App.tsx`
- `app/src/App.wave10.test.tsx`

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
- `app/src/App.tsx`
- `app/src/App.test.tsx`
- `app/src/App.wave10.test.tsx`
- `execution/phase_10_master_execution_plan.md`
- `execution/progress.md`
- `execution/current_constraints.md`
- `execution/current_repo_truth.md`
- `execution/packet_registry.md`
- `execution/live_execution_log.md`
- `execution/open_questions.md` if unresolved updater-discovery blockers change materially
- `execution/decisions/decision_log.md` if a real local governance decision becomes necessary
- `execution/task_packets/p10_ws10_t03_implement_native_app_updater_discovery_availability_truth_and_safe_coordination.md`

## Implementation Scope
In scope:
- shell dependency and configuration updates required to enable native app-updater discovery without yet authorizing download, verification, or apply behavior
- shell-owned native updater discovery execution that can truthfully initiate update checks and classify `CheckingForUpdates`, `NoUpdateAvailable`, `UpdateAvailable`, and `UpdaterError` outcomes using the completed Wave 2 canonical updater vocabulary
- bounded availability metadata or explanatory-note projection needed so the shell can expose truthful updater discovery results without implying verified readiness or installability
- safe coordination with current lifecycle and run-state truth so updater discovery remains separate from active-run execution state and any install/apply path remains blocked or deferred while an active run exists
- shell startup, local-interface, lifecycle, status, and bounded frontend or diagnostics integration needed to expose updater discovery truth honestly
- tests covering no-update versus update-available versus technical-failure classification, canonical updater-state progression, and active-run-safe coordination behavior

## Required Behavior to Preserve
- keep the Rust/Tauri shell as the sole local authority for updater discovery, updater-state truth, availability classification, safe deferral, and any blocked-action explanation
- keep the frontend as presentation-only, with no hidden authority over update checks, availability classification, installability, trust, or apply safety
- preserve the distinction between app-shell updater behavior and managed runtime-delivery behavior
- preserve the distinction between `NoUpdateAvailable` and `UpdaterError`, and between `UpdateAvailable` and `UpdateReady`
- preserve the invalid-transition rules that discovered updates may not become `UpdateReady` or `ApplyingUpdate` before later verification/apply packets exist
- preserve updater-state separation from run lifecycle truth, runtime readiness truth, report truth, cleanup truth, and shutdown truth
- preserve the frozen runtime/update split, verification-before-apply or use rule, and active-run non-mutation rule
- do not overstate discovery and availability truth as proof that download, verification, apply, restart, or runtime-delivery behavior already exists

## Explicit Non-Goals / Forbidden Changes
Do not implement:
- update download, update verification execution, update apply, restart coordination, or any claim that an update is already installable
- managed runtime artifact authorization, download, hydration, repair, verification execution, compatibility enforcement, or apply/use behavior
- release-readiness signoff, distribution-operations automation, or hosted release infrastructure
- simulation, report, cleanup, shutdown, janitor, or no-resume redesign
- commercial-backend expansion beyond already-governed support participation
- public API or OpenAPI behavior
- product-feature UI behavior beyond bounded shell-owned updater-truth rendering

## Deliverables
- native app-updater discovery integration exists under shell authority with the minimum dependency/config basis required for truthful local update checks
- shell-owned updater truth can now progress through real `CheckingForUpdates`, `NoUpdateAvailable`, `UpdateAvailable`, and `UpdaterError` outcomes without implying verified readiness or apply behavior
- shell startup, lifecycle, status, local-interface, and bounded frontend surfaces expose updater discovery truth and active-run-safe coordination honestly
- the repo remains explicit that update download, verification, apply, restart, and runtime-delivery behavior are still deferred beyond this packet

## Validation / Acceptance Proof Required
- show exact files created and updated
- confirm only the allowed files changed
- confirm `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` complete with zero warnings and zero errors
- confirm `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` remain clean if frontend files change
- confirm real updater discovery can distinguish no-update, update-available, and technical-failure outcomes truthfully
- confirm discovery does not classify an available update as ready or installable before later verification/apply work exists
- confirm updater discovery does not mutate runtime assets or active-run execution state and does not override run/report/cleanup truth
- perform a structural consistency review and contradiction check across the touched execution files before closeout

## Escalation Triggers
Escalate immediately if:
- truthful native updater discovery cannot be enabled without choosing an unresolved upstream trust, feed, signing, or distribution mechanism
- the packet would require download, verification, apply, restart, or runtime-delivery behavior to remain coherent
- any touched shell, config, or frontend surface would blur `NoUpdateAvailable` into `UpdaterError`, blur `UpdateAvailable` into `UpdateReady`, or otherwise violate Document A Section 7.10
- active-run-safe updater coordination would require redefining run lifecycle truth, cleanup truth, shutdown truth, or no-resume preservation
- a required config or dependency change would weaken the frozen runtime/update split, shell/frontend authority split, or active-run non-mutation rule

## Activation Notes
- Authored and activated immediately as the current executable Phase 10 Wave 3 packet for native app-updater discovery, availability truth, and safe coordination over the completed Wave 2 contract foundation.
- Repo changes are authorized only within this packet's allowlist and bounded Wave 3 discovery/availability scope until closeout or blockage.
- Update download, verification, apply, restart coordination, and managed runtime-delivery execution remain out of scope while this packet is active.

## Completion Notes
- Completed on 2026-04-17 after materializing shell-owned native updater discovery, truthful availability classification, explicit startup/lifecycle/status coordination surfaces, and a focused shell test path that proves active-run updater coordination after a mocked completed discovery check.
- Final validation completed cleanly with `cargo check`, `cargo clippy -- -D warnings`, and `cargo test`; the earlier frontend-touching slice of the same packet had already rerun `npm run lint`, `npm run typecheck`, `npm run test -- --run`, and `npm run build` cleanly, and no frontend files changed after that rerun.
- Because this workspace is not a git repository, the closeout changed-file audit used the applied patch set plus targeted searches; the touched files stayed within this packet's allowlist.
- The critique and contradiction sweep found no material in-scope defect, truth collapse, or execution-surface inconsistency across `execution/progress.md`, `execution/current_constraints.md`, `execution/current_repo_truth.md`, `execution/packet_registry.md`, `execution/live_execution_log.md`, `execution/phase_10_master_execution_plan.md`, and this packet.
- Phase 10 returned to a between-packets posture at closeout; `P10-WS10-T04` remains the prepared successor and is not active.