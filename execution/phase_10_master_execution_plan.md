# Phase 10 Master Execution Plan

## Status
- Phase: P10
- Phase name: Updater, Runtime Delivery, and Distribution Hardening
- Frozen by packet: `P10-WS10-T01`
- Date frozen: 2026-04-17
- Phase closure validated by packet: `P10-WS11-T01`
- Phase status: CLOSED — 2026-04-17
- Scope: phase-specific execution ordering, bounded updater/runtime-delivery basis, and future packetization guidance only

## Purpose
Phase 10 establishes trusted update, runtime-delivery, and distribution hardening outside the core run loop.

## Governing Basis
- Runtime and update acceptance obligations remain governed primarily by `AC-101` and `AC-102`; user-visible blocked-state and loading-state clarity obligations remain governed by `AC-105` through `AC-108` where Phase 10 surfaces touch them.
- Updater lifecycle states and transitions remain governed by Document A Section `7.10`, including `STATE-236` through `STATE-264`.
- Runtime distribution architecture, commercial/update interface rules, runtime/update support contract, and runtime/update integrity requirements remain governed by Document A Sections `8`, `10`, `11`, and `13`.
- WS-10 ownership remains governed by `BWORK-043` through `BWORK-046`; supporting shell/runtime and commercial-boundary responsibilities remain governed by `BWORK-011` through `BWORK-018` where they intersect.
- Phase 10 ordering, outcomes, non-goals, and exit intent remain governed by `BPHASE-053` through `BPHASE-057` and Document B Sections `6.12.6` through `6.12.10`.
- The resolved MVP runtime/update split from `OQ-005` and `DEC-019` remains binding: app-shell updates use the native app-updater path, while Python runtime delivery remains a separate shell-owned managed runtime-package hydration flow authorized by the commercial backend and verified before use.

## Phase 10 Entry Basis Frozen by This Plan
- Phase 2 already delivered a bounded runtime package contract, shell-owned presence and integrity basis, runtime readiness gating, and startup-check integration through `P02-WS10-T01` without implementing production runtime download, repair, or updater behavior.
- The execution OS already froze the MVP runtime/update split: native app-updater for app-shell updates, a separate shell-owned managed runtime-package hydration flow for Python runtime, commercial-backend authorization support where governed, and verification before use.
- Phase 3 already delivered the bounded commercial backend and device-bound local activation architecture that later Phase 10 runtime authorization may consult without becoming a hosted simulation archive or hidden lifecycle controller.
- Phases 7 through 9 are now closed, so run lifecycle truth, report truth, cleanup truth, controlled shutdown, startup janitor behavior, preserved-report boundaries, and explicit no-resume preservation are stable and must be preserved by later updater/runtime-delivery work.
- The current repo still lacks production native app-updater discovery and apply flow, truthful update availability projection, runtime artifact authorization and download behavior, verification-before-apply or use enforcement, rejected-artifact handling, compatibility checks, and active-run-safe apply or hydration coordination.
- Phase 10 must materialize trusted delivery behavior without redefining simulation behavior, run-state truth, report truth, or cleanup truth, and without allowing update or runtime flows to mutate active-run execution state or runtime assets during an active run.

## Bounded Phase 10 Delivery Path

### Ownership Split
- The frontend may render shell-owned updater state, update availability, runtime delivery status, verification results, and blocked-action explanations only when the shell exposes those truths. It does not check update feeds, verify artifacts, touch runtime payloads, or decide install or apply safety on its own.
- The Rust/Tauri shell remains the authoritative local owner of updater lifecycle state, app-update discovery, app-update apply coordination, runtime delivery orchestration, verification-before-apply or use, compatibility gating, active-run safety checks, and runtime-readiness integration.
- The commercial backend may provide bounded authorization, manifest, or entitlement-adjacent support where governed, but it does not become the authority for local run-state truth, local filesystem mutation, or hosted simulation archive behavior.
- The Python runtime remains an artifact subject to shell-owned delivery and verification rules. It does not own updater policy, installation trust, or apply coordination.
- Phase 10 may harden updater and runtime-delivery truth, but it does not authorize Phase 11 release-readiness signoff, hosted release-operations automation, or changes to the core run, report, or cleanup model.

### App-Update Versus Runtime-Delivery Split
| Artifact class | Primary mechanism | Local authority | Verification requirement | Apply/use during active run |
| --- | --- | --- | --- | --- |
| App-shell update artifact | native app-updater path | shell | verify according to approved updater trust model before apply | forbidden |
| Managed Python runtime package | separate shell-owned managed runtime-package delivery path | shell | verify integrity, expected-file set, version or compatibility, and trust basis before use | forbidden |
| Update/runtime support metadata | bounded local support metadata plus governed remote authorization inputs where applicable | shell with bounded backend support | must not bypass trust checks or become hidden execution state | forbidden if mutation would affect the active run |

- Updater lifecycle semantics apply to app update behavior, not simulation run behavior.
- Runtime delivery semantics remain separate from app-shell updater state even when both surfaces appear in the same shell-owned trust boundary.
- No Phase 10 packet may blur `UpdateAvailable` with `UpdateReady`, treat downloaded artifacts as usable before verification, or reclassify runtime artifacts as run-workspace data.

### Verification and Active-Run Safety Rules
- `UpdateAvailable` means a newer applicable app update is known, not yet trusted for installation.
- `UpdateReady` means the app update artifact has been downloaded and verified successfully under the approved trust model.
- `UpdateRejected` means trust or integrity checks failed and the candidate must not be installable.
- Runtime delivery must not mark the runtime ready or usable until presence, expected-file verification, version compatibility, and trust-manifest or equivalent integrity checks all pass.
- Update and runtime flows may discover availability while the app is idle, but they must not silently override active-run lifecycle truth, interrupt active simulation semantics, mutate runtime assets during an active run, or rewrite cleanup or report state.
- Technical delivery failures and trust-based rejections must remain distinct.
- Later packets may add user-visible safe deferral or restart coordination behavior, but they may not smuggle in crash resume, hidden continuity, or background mutation of active execution state.

## Bounded Phase 10 Implementation Basis
- Phase 10 must materialize the canonical updater lifecycle and shell-owned truth exposure for update discovery, availability, download, verification, ready, rejected, applying, applied, and error states.
- Phase 10 must materialize native app-updater discovery flow and truthful update availability behavior without permitting unverified apply.
- Phase 10 must materialize managed runtime delivery authorization, artifact download or acquisition behavior, verification before use, rejection of unsafe artifacts, and compatibility gating.
- Phase 10 must integrate safe updater and runtime status rendering into the bounded frontend without giving the frontend authority over local apply or use behavior or artifact trust.
- Phase 10 must preserve existing runtime readiness, run lifecycle, report, and cleanup semantics while integrating later update and runtime-delivery truth.
- Phase 10 does not authorize simulation changes, report-path redesign, cleanup or janitor redesign, hosted archive behavior, or release-readiness claims.

## Phase 10 Wave Plan

### P10-W01 — Updater, Runtime Delivery, and Distribution Hardening Entry Basis and Master Execution Blueprint
- Status: COMPLETE via `P10-WS10-T01`.
- Outcome: the bounded Phase 10 updater and runtime-delivery path, ownership split, verification-before-apply or use rules, active-run safety constraints, and later wave order are now frozen for implementation planning and hand off cleanly to the Wave 2 contract-foundation packet.

### P10-W02 — Updater Lifecycle, Runtime Delivery Contract, and Trust Surface Foundation
- Status: COMPLETE via `P10-WS10-T02`.
- Objective: establish code-level updater and runtime-delivery contract surfaces, canonical updater vocabulary, runtime authorization and verification contract basis, and shell-owned truth surfaces without yet implementing artifact download or apply behavior.
- Outcome: updater lifecycle and runtime-delivery contract modules now exist in product code, shell/startup/interface/runtime truth surfaces now expose the bounded Phase 10 vocabulary honestly, and the frontend diagnostics surface now renders that shell-owned contract foundation without overclaiming real discovery/download/apply behavior.
- Required outputs:
  - updater lifecycle contract surfaces aligned to `UpdaterIdle` through `UpdaterError`
  - runtime-delivery contract surfaces aligned to the existing runtime package contract and runtime readiness basis
  - explicit verification-before-apply or use and active-run non-mutation rules in shell-owned truth surfaces
  - shell, startup, local-interface, and bounded frontend truth-surface updates needed to freeze the vocabulary honestly
- Exit proof:
  - updater states remain distinct from run lifecycle states
  - runtime delivery remains distinct from app-shell update behavior
  - no contract implies verified installability or runtime usability before trust checks pass
  - no surface implies hidden background mutation of active-run state or runtime assets
- Planned packet anchor: `P10-WS10-T02`

### P10-W03 — Native App-Updater Discovery, Availability Truth, and Safe Coordination
- Status: COMPLETE via `P10-WS10-T03`.
- Objective: implement the native app-updater discovery flow, truthful availability classification, bounded technical-failure versus trust-rejection distinction where applicable, and safe coordination with lifecycle truth before app-update apply behavior is finalized.
- Outcome: native app-updater discovery now exists in product code with signed runtime-configured endpoint and pubkey resolution, truthful `NoUpdateAvailable` versus `UpdateAvailable` versus `UpdaterError` projection, explicit startup/lifecycle/status coordination surfaces, bounded diagnostics rendering, and focused shell proof for active-run-safe coordination after discovery, while download, verification, apply, restart, and managed runtime-delivery behavior remain deferred.
- Required outputs:
  - real update-check initiation and availability classification
  - shell-owned updater lifecycle progression through check, no-update, update-available, and technical-failure states
  - bounded frontend or diagnostics rendering of shell-owned updater truth
  - active-run-safe deferral logic where update action would otherwise collide with governed lifecycle truth
- Exit proof:
  - updater states are truthful and use canonical lifecycle names
  - no-update outcomes do not appear as errors
  - available updates do not silently apply and do not override active-run truth
  - discovery behavior does not mutate runtime assets or active-run execution state
- Planned packet anchor: `P10-WS10-T03`

### P10-W04 — Verified Runtime Delivery, Update Apply Gating, and Artifact Rejection Hardening
- Status: COMPLETE via `P10-WS10-T04`.
- Objective: implement managed runtime delivery authorization and artifact acquisition, verification-before-use or apply, rejection of unsafe update and runtime artifacts, compatibility checks, and bounded app-update apply gating or restart coordination.
- Current implementation state: `P10-WS10-T04` now materializes runtime installation or rejection persistence, manifest-driven hydration from `MIRO_FISH_RUNTIME_DELIVERY_MANIFEST`, expected-file verification, compatibility rejection, rollback-safe metadata commit, trust-manifest validation against the installed package, hydration-required lifecycle/readiness truth grounded in actual presence and integrity, a bounded commercial-backend `resolve-runtime-delivery-manifest` support surface, explicit updater preparation through `prepare_native_app_updater`, and explicit updater apply through `apply_native_app_updater` with truthful `ApplyingUpdate`, `UpdateApplied`, or `UpdaterError` outcomes plus shell-owned restart requests after successful install. Bounded diagnostics now surface refresh, prepare, and apply actions plus ready, rejected, error, and restart-request truth while remaining presentation-only.
- Required outputs:
  - runtime artifact authorization and download or acquisition flow
  - artifact verification, compatibility checks, and rejection handling for both app updates and runtime packages where governed
  - shell-owned apply or use gating that blocks unverified or incompatible artifacts
  - protections preventing runtime or update flows from mutating active-run execution state or runtime assets during an active run
  - truthful frontend or diagnostics rendering of ready, rejected, blocked, and error states
- Exit proof:
  - unsafe update artifacts are not installable
  - runtime artifacts are not usable before verification passes
  - rejected artifacts remain non-installable and non-usable
  - active-run truth, report truth, and cleanup truth remain intact while delivery hardening is active
- Planned packet anchor: `P10-WS10-T04`

### P10-W05 — Phase 10 Validation, Critique, and Exit Gate
- Status: COMPLETE via `P10-WS11-T01`.
- Objective: prove Phase 10 truth against upstream updater and runtime-delivery acceptance and close only if the result is real.
- Outcome: fresh shell, frontend, and commercial-backend reruns are clean; `AC-101`, `AC-102`, and materially touched `AC-105` through `AC-108` are now proven against the live implementation; no in-scope remediation was required; and Phase 10 closes honestly without overclaiming release readiness or hosted release operations.
- Required outputs:
  - acceptance mapping for the applicable runtime and update integrity plus updater-clarity criteria, including `AC-101`, `AC-102`, and any user-visible blocked-state or loading-state obligations materially touched by the implementation
  - evidence for updater lifecycle truth, verification-before-apply or use, rejected-artifact handling, runtime authorization and compatibility gating, and active-run non-mutation
  - critique pass and in-scope remediation or escalation
- Exit proof:
  - unsafe updates are not installable
  - runtime artifacts are verified before use
  - updater states are truthful
  - update flows do not silently override active-run truth
  - runtime or update flows do not mutate active-run execution state or runtime assets during an active run
- Planned packet anchor: `P10-WS11-T01`

## Wave Dependency Rules
- W02 freezes the updater and runtime-delivery vocabulary and trust surfaces needed by W03 through W05.
- W03 establishes truthful app-update discovery and availability behavior before W04 can claim coherent apply gating or artifact-rejection hardening.
- W04 establishes verified runtime delivery and apply or use protections before W05 can assess honest Phase 10 closure.
- W04 may integrate bounded commercial-backend authorization support only where the frozen runtime/update split already allows it; it may not reopen activation truth or hosted backend scope.
- W05 cannot begin exit assessment until W03 and W04 are implemented and validated.
- No wave may redefine simulation behavior, run-state truth, report truth, cleanup truth, or no-resume preservation.

## Still Deferred
- Phase 11 QA matrix execution, red-team review, and release-readiness signoff
- hosted release infrastructure, signing automation, and distribution-operations automation beyond the bounded product behavior needed for truthful local delivery or update handling
- public API or OpenAPI expansion
- simulation, report, or cleanup redesign

## Packetization Rule
- Future Phase 10 packets must be large enough to implement, validate, critique, and hand off coherently.
- Future Phase 10 packets must cite this plan together with their upstream Document A, B, and C rules.
- No packet may let the frontend become the authority for updater state, runtime trust, apply safety, or artifact verification.
- No packet may let update or runtime delivery mutate active-run execution state or runtime assets during an active run.
- No packet may collapse app-shell updates and managed runtime delivery into one hidden mechanism or bypass the frozen runtime/update split.
- No packet may treat unverified artifacts as installable or usable.