# Phase 3 Master Execution Plan

## Status
- Phase: P03
- Phase name: Licensing, Activation, and Commercial Backend
- Frozen by packet: `P03-WS03-T01`
- Date frozen: 2026-04-15
- Scope: phase-specific execution ordering, bounded activation/commercial basis, and future packetization guidance only

## Purpose
Phase 3 establishes valid licensed use and commercial truth.

## Governing Basis
- Device-bound activation and the two-device limit remain locked by `CONST-017`, `CONST-018`, `AC-012`, `AC-013`, `DB-035`, and `DB-039`.
- Offline capability remains bounded by `CONST-038`, `CONST-055`, `AC-015`, `DB-040`, and `DB-043`; no broader offline entitlement is authorized.
- Activation state truth remains governed by `STATE-063` through `STATE-090`.
- App lifecycle gating remains governed by `STATE-017` through `STATE-019` and `STATE-029` through `STATE-041`.
- Local activation storage remains governed by `DATA-012` through `DATA-016`, `AC-018`, and `CDATA-018` through `CDATA-025`.
- Commercial backend entities remain governed by `DB-005` through `DB-070` and `AC-121` through `AC-126`.
- Cloud interface scope remains governed by `API-085` through `API-089`.
- Phase ordering and closure remain governed by `BPHASE-018` through `BPHASE-022`, `BDEP-018` through `BDEP-020`, and `BDEP-028`.

## Phase 3 Entry Basis Frozen by This Plan
- Phase 2 handed off runtime readiness truth, shell supervision truth, and the local trust-boundary foundation honestly.
- Phase 3 entry no longer depends on selecting the final local persistent support storage engine.
- Phase 3 entry now depends on the bounded storage-domain split below, which is concrete enough for later implementation packetization.
- The commercial entity model is frozen enough to implement Phase 3 without inventing new durable business concepts.
- Runtime distribution authorization and update-support compatibility must be preserved, but Phase 10 delivery behavior remains deferred.

## Bounded Activation-Artifact Path

### Ownership Split
- The cloud commercial backend is the authoritative durable owner of license, activation, payment-event, activation-audit, and issued-token lineage truth.
- The Rust/Tauri shell is the authoritative local owner of device identity derivation, activation submission, secure-store commit, local offline validation, and activation-readiness gating.
- The frontend may collect activation input and render activation state, but it does not own secure storage or activation validation authority.
- The Python engine does not own activation issuance, secure storage, or entitlement truth.

### Storage-Domain Split
| Artifact class | Storage domain | Primary owner | Allowed contents | Explicit exclusions | Cleanup posture |
| --- | --- | --- | --- | --- | --- |
| Commercial entitlement and activation ledger | commercial backend domain | cloud backend | licenses, device activations, payment events, activation audit records, issued-token lineage or fingerprint records | seed documents, transcripts, run memory, report semantics, hidden run history, raw hardware identity as the preferred durable canonical record | durable backend data; not local cleanup-targeted |
| Device-bound activation token | OS-native secure storage domain | shell | raw device-bound JWT and only the secure-store metadata needed to retrieve or rotate it safely | plaintext mirrors in local support files, workspaces, logs, report metadata, frontend durable state, engine-owned files | not subject to run cleanup or janitor; governed by activation lifecycle behavior |
| Activation validation support record | local persistent support domain in the app-controlled support location | shell | schema/version marker, bounded license reference, bounded activation reference, slot or binding context, token fingerprint or lineage marker, expiration or validity marker where applicable, secure-store lookup reference, last local validation outcome, last local validation timestamp | raw JWT material, expanded purchaser history, payment-event history, run data, report data, transcript data, raw hardware identity, hidden local activation archives | not run-cleanup targeted; updated only by activation, validation, revocation, or explicit deactivation lifecycle work |
| Activation-attempt working state | transient in-memory UI or shell state only | frontend plus shell | activation form input, in-progress state, retry posture, user-visible denial or error context | durable token copies, hidden attempt archives, logs containing raw token or secret-bearing response data | discarded after the attempt completes or is abandoned |

- The local activation validation support record is the approved non-secret companion to the secure-store token.
- The final storage engine for the local activation validation support record remains open, but it must stay inside the local persistent support domain and keep raw token material out of that domain.
- No activation artifact may be placed in the transient workspace domain, report/export preservation domain, generic config files, or logs.

### Activation Commit and Validation Sequence
1. The frontend submits activation input to the shell through the approved shell-owned command boundary.
2. The shell derives the approved device-binding representation and calls the app-to-cloud activation interface.
3. The backend evaluates entitlement, slot availability, and device eligibility, then returns approval, denial, or technical failure.
4. On approval, the shell validates the returned device-bound JWT and commits the raw token to OS-native secure storage.
5. The shell also commits the bounded non-secret activation validation support record to local persistent support storage.
6. Activation is successful only after both local commits succeed; otherwise the result is `ActivationError`, not `Activated`.
7. Startup checks and approved offline-capable flows validate the current device against the locally stored token and bounded support record without requiring live backend access.
8. If local validation fails for a previously activated device, the activation domain transitions to `ActivationRevoked` and the app lifecycle must return to activation-gated posture.

## Activation Readiness Semantics
- Runtime readiness answers whether the heavy runtime is present and integrity-accepted.
- Activation readiness answers whether the current device has valid locally stored activation state for licensed use.
- Provider readiness answers whether required provider configuration exists.
- Preflight eligibility answers whether a specific requested run may start.
- These readiness domains must remain separate.
- `StartupChecksRunning -> ActivationRequired` remains valid when runtime is healthy but activation is absent, invalid, or locally revoked.
- `ActivationRequired -> ConfigurationRequired` or `ReadyIdle` may occur only after successful activation commit.
- `ActivationRevoked` is never equivalent to `Activated`; it returns the app to activation-gated posture.
- Later run eligibility depends on runtime readiness, activation readiness, provider readiness, and a current start-eligible Preflight outcome.

## Bounded Commercial Implementation Basis
- Required backend entity families are: licenses, device activations, payment events, activation audit records, and issued-token tracking or audit equivalent.
- Required activation decision classes are: approval, denial, and technical failure.
- Required denial bases at minimum are: invalid license, expired or unusable license, revoked entitlement, activation limit exceeded, and device mismatch where governed.
- Required local validation outcomes at minimum are: activation required, activation denied, activation error, activated and valid, and activation revoked.
- The commercial backend must remain structurally separate from simulation execution and must not store seed documents, transcripts, run memory, or report semantics beyond governed entitlement support.
- Runtime distribution authorization and update-support compatibility must remain contract-compatible, but Phase 3 implementation does not own Phase 10 delivery behavior.

## Phase 3 Wave Plan

### P03-W01 — Activation Entry Basis and Master Execution Blueprint
- Status: COMPLETE via `P03-WS03-T01`.
- Outcome: the bounded activation-artifact path, activation-readiness separation, and later wave order are now frozen for implementation.

### P03-W02 — Activation Contract and Local Boundary Foundation
- Objective: establish code-level activation domain contracts and shell-owned local artifact boundary surfaces without pretending that full activation truth already exists.
- Required outputs:
  - activation request and response types preserving approval, denial, and technical-failure distinction
  - shell-owned activation command boundary and secure-store abstraction
  - bounded local activation validation support-record contract aligned with this plan
  - activation-state mapping from activation lifecycle truth into app lifecycle gating
- Exit proof:
  - raw token material cannot flow into ordinary files, workspaces, or logs by design
  - activation contract surfaces are explicit and consistent with this plan
  - no feature is counted complete through mock-only activation behavior
- Planned packet anchor: `P03-WS03-T02`

### P03-W03 — Commercial Ledger and Activation Decision Implementation
- Objective: implement the authoritative backend record families and the real activation decision path.
- Required outputs:
  - durable license records
  - device activation records with two-slot enforcement
  - payment-event-to-entitlement translation support
  - activation audit trail
  - device-bound token issuance lineage or fingerprint tracking
  - activation response classification preserving denial versus error truth
- Exit proof:
  - third-device approval is impossible on the normal path
  - policy denial and technical failure are distinct end to end
  - simulation data does not enter the commercial backend
- Planned packet anchor: `P03-WS03-T03`

### P03-W04 — Secure Local Activation Commit, Offline Validation, and Lifecycle Wiring
- Objective: implement the shell-mediated local activation commit path and startup or offline validation behavior.
- Required outputs:
  - secure-store token commit and retrieval
  - bounded local support-record commit and recovery
  - current-device validation against the device-bound token
  - startup activation-readiness determination
  - truthful `ActivationRequired`, `ConfigurationRequired`, and `ReadyIdle` gating
  - revocation or mismatch failure mapping without collapsing into generic error
- Exit proof:
  - a token minted for one device does not validate as normal licensed use on another device
  - offline post-activation validation works for approved offline-capable scenarios
  - activation success requires successful local commit, not mere backend approval
- Planned packet anchor: `P03-WS03-T04`

### P03-W05 — Phase 3 Validation, Critique, and Exit Gate
- Objective: prove Phase 3 truth against upstream acceptance and close only if the result is real.
- Required outputs:
  - acceptance mapping for `AC-011` through `AC-018` and `AC-121` through `AC-126`
  - third-device denial test evidence
  - device mismatch rejection evidence
  - offline validation evidence
  - no-simulation-persistence-in-backend evidence
  - critique pass and in-scope remediation or escalation
- Exit proof:
  - the app truthfully distinguishes activated versus unactivated licensed use
  - activation truth is real, not mocked
  - local activation state supports approved offline post-activation validation
  - backend ledger behavior is durable and bounded
- Planned packet anchor: `P03-WS11-T01`

## Wave Dependency Rules
- W02 freezes contracts and local boundary surfaces needed by W03 and W04.
- W03 and W04 may overlap only after their shared contracts are frozen and neither wave fakes completion through stubs.
- W05 cannot begin exit assessment until W03 and W04 are implemented and validated.
- No wave may claim Phase 3 completion while activation truth is mocked or while local or offline validation remains unproven.

## Still Deferred
- provider configuration and secure provider-key handling
- Preflight and run-start gating beyond activation readiness
- simulation behavior, live events, reporting, cleanup hardening, and updater or runtime-delivery implementation
- the concrete local persistent support storage engine
- the concrete commercial backend framework or deployment choice
- public API or OpenAPI expansion

## Packetization Rule
- Future Phase 3 packets must be large enough to implement, validate, critique, and hand off coherently.
- Future Phase 3 packets must cite this plan together with their upstream Document A, B, and C rules.
- No packet may treat a stubbed activation happy path as exit proof.