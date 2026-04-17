# Phase 4 Master Execution Plan

## Status
- Phase: P04
- Phase name: Provider Configuration and Secure Local Setup
- Frozen by packet: `P04-WS04-T01`
- Date frozen: 2026-04-15
- Scope: phase-specific execution ordering, bounded provider-configuration and secure-secret-handling basis, and future packetization guidance only

## Purpose
Phase 4 establishes provider-dependent readiness and secret-handling truth.

## Governing Basis
- The MVP provider model remains locked by `CONST-029`, `AC-025`, `DATA-017`, and `BWORK-019` through `BWORK-022`.
- Provider secret storage remains governed by `DATA-018` through `DATA-022`, `DATA-066`, `DATA-071`, and `CDATA-022` through `CDATA-025`.
- App lifecycle configuration gating remains governed by `STATE-018`, `STATE-031`, `STATE-036`, `STATE-039`, and `STATE-041`.
- Secure local authority boundaries remain governed by `ARCH-041`, `ARCH-043`, `FLOW-031` through `FLOW-035`, and `AC-026` through `AC-030`.
- Phase ordering and closure remain governed by `BPHASE-023` through `BPHASE-027`, `BDEP-020`, `BDEP-021`, `BDEP-028`, and `BDEP-036`.

## Phase 4 Entry Basis Frozen by This Plan
- Phase 3 handed off truthful activation readiness, shell-owned secure local handling, configuration-state naming, and a real shell authority boundary honestly.
- Phase 4 entry does not require reopening or extending the closed Phase 3 activation surface.
- Phase 4 entry depends on freezing the provider-secret handling boundary strongly enough that later packets can implement provider setup without guessing where secrets or non-secret provider metadata belong.
- Supported MVP provider choices are frozen enough for packetization at the provider-family level: OpenAI and Google.
- Preflight, provider reachability checks, model availability checks, and run-start eligibility remain explicitly deferred beyond Phase 4.

## Bounded Provider-Secret Path

### Ownership Split
- The Rust/Tauri shell is the authoritative local owner of provider-key submission, secure-store commit, secure retrieval mediation, configuration readiness gating, and any bounded non-secret provider-configuration support state.
- The frontend may collect provider selection and provider-key input and render configuration status, but it does not own secure storage or raw secret retrieval authority.
- The Python engine does not own durable provider-secret storage; later packets may allow it to consume provider credentials only through shell-mediated local retrieval paths that do not expose the raw secret to the frontend.
- The cloud commercial backend does not own provider secrets and must not become a vendor-side secret vault for simulation access.

### Storage-Domain Split
| Artifact class | Storage domain | Primary owner | Allowed contents | Explicit exclusions | Cleanup posture |
| --- | --- | --- | --- | --- | --- |
| Provider API key | OS-native secure storage domain | shell | raw provider secret and only the secure-store metadata needed to retrieve, rotate, or delete it safely | plaintext config files, ordinary app JSON state, app-managed SQLite as normal plaintext, workspaces, logs, reports, transcript archives, frontend durable state | not run-cleanup targeted; governed by explicit provider-setup and provider-clear lifecycle behavior |
| Provider configuration support state | local persistent support domain in the app-controlled support location | shell | schema/version marker, selected provider identifier, secure-store lookup reference, bounded configuration-presence or last-known configuration outcome metadata where needed | raw provider secret, model-availability claims, Preflight outcomes, run data, report data, transcript data, hidden secret mirrors | not run-cleanup targeted; updated only by provider setup, provider change, provider clear, or bounded readiness evaluation work |
| Provider-setup working state | transient in-memory UI or shell state only | frontend plus shell | provider selection input, in-progress key entry, success or failure context, retry posture | durable secret copies, hidden attempt archives, logs containing raw secret material | discarded after setup completes or is abandoned |

- The exact storage engine for bounded non-secret provider-configuration support state remains open at Phase 4 entry, but it must stay inside the app-controlled support domain and must never carry the raw provider secret.
- No provider secret may be placed in transient workspaces, report/export preservation domains, generic config files, or routine logs.

### Provider Configuration Sequence
1. The frontend submits provider-selection and provider-key input to the shell through the approved shell-owned command boundary.
2. The shell validates the bounded request shape and supported-provider rules.
3. The shell commits the raw provider key to OS-native secure storage.
4. The shell commits or refreshes any bounded non-secret provider-configuration support state required to expose truthful configuration readiness.
5. The shell returns success or failure truth to the frontend.
6. Configuration is present only after the secure-store commit succeeds; otherwise the result remains configuration-missing or configuration-error.
7. Later authorized packets may retrieve the provider secret through shell-only mediation for Preflight or execution needs.
8. Successful provider-key storage does not imply provider or model reachability; that remains a Preflight concern.

## Provider Readiness Semantics
- Runtime readiness answers whether the heavy runtime is present and integrity-accepted.
- Activation readiness answers whether the current device has valid local activation state for licensed use.
- Provider readiness answers whether required provider selection and secure local provider-secret configuration are present.
- Preflight eligibility answers whether a specific requested run may start under the selected provider, model, budget, and input constraints.
- These readiness domains must remain separate.
- `StartupChecksRunning -> ConfigurationRequired` remains valid when runtime and activation are ready enough but required provider configuration is still incomplete.
- `ActivationRequired -> ConfigurationRequired` remains valid only after activation succeeds while provider configuration is still incomplete.
- `ConfigurationRequired -> ReadyIdle` may occur only after required provider configuration becomes valid.
- `ReadyIdle` still does not imply Preflight approval for any specific run.

## Bounded Provider Implementation Basis
- Required provider choices for MVP are OpenAI and Google only.
- Required provider-configuration outcomes at minimum are: configuration stored, unsupported provider, malformed input, secure-storage failure, OS permission or access failure, configuration missing, and configuration present without overclaiming run readiness.
- The shell must remain the authoritative local mediator for storing, retrieving, rotating, and deleting provider secrets.
- Raw provider secrets must remain structurally separate from Preflight results, run workspaces, reports, diagnostics, and commercial-backend data.
- Phase 4 may establish a truthful secure retrieval path for later authorized local execution, but it does not authorize Preflight feasibility logic, model-availability checks, or live inference behavior.

## Phase 4 Wave Plan

### P04-W01 — Provider Configuration Entry Basis and Master Execution Blueprint
- Status: COMPLETE via `P04-WS04-T01`.
- Outcome: the bounded provider-secret path, provider-readiness separation, and later wave order are now frozen for implementation.

### P04-W02 — Provider Configuration Contract and Secret Boundary Foundation
- Objective: establish code-level provider-configuration contracts, secure-secret boundaries, and provider-readiness truth surfaces without pretending that real secure provider handling already exists.
- Required outputs:
  - provider selection and key-entry request or response types
  - shell-owned provider secret boundary and secure-store abstraction surface
  - bounded non-secret provider-configuration support-state contract aligned with this plan
  - provider-readiness truth surface and lifecycle mapping for `ConfigurationRequired` versus `ReadyIdle`
- Exit proof:
  - raw provider secrets cannot flow into ordinary files, workspaces, reports, or logs by design
  - provider-readiness surfaces are explicit and consistent with this plan
  - configuration-present is not misrepresented as Preflight-ready or run-ready
- Planned packet anchor: `P04-WS04-T02`

### P04-W03 — Secure Local Provider Commit, Retrieval Mediation, and Configuration Wiring
- Objective: implement the shell-mediated secure provider setup path, bounded non-secret configuration support state, authorized retrieval mediation, and truthful configuration gating.
- Required outputs:
  - real OS-native secure-store commit, retrieval, and clear behavior for provider secrets
  - bounded local provider-configuration support-state persistence and recovery where needed
  - authorized secret retrieval mediation through approved local layers
  - truthful `ConfigurationRequired` versus `ReadyIdle` determination based on required provider configuration presence
  - frontend provider setup flow that exercises the real shell-owned path
- Exit proof:
  - provider secrets are not intentionally stored in plaintext app files, workspaces, reports, or logs
  - the frontend cannot directly retrieve raw provider secrets
  - the app truthfully distinguishes configuration-present from full run readiness
- Planned packet anchor: `P04-WS04-T03`

### P04-W04 — Phase 4 Validation, Critique, and Exit Gate
- Objective: prove Phase 4 truth against upstream provider-configuration acceptance and close only if the result is real.
- Required outputs:
  - acceptance mapping for `AC-025` through `AC-030`
  - plaintext-secret prohibition evidence
  - configuration-gating evidence
  - critique pass and in-scope remediation or escalation
- Exit proof:
  - provider secrets are stored only in approved OS-native secure storage
  - missing required provider configuration truthfully blocks ready-for-run posture
  - configuration-present is not misrepresented as provider/model readiness before Preflight
- Planned packet anchor: `P04-WS11-T01`

## Wave Dependency Rules
- W02 freezes contracts and local secret boundaries needed by W03.
- W03 may not claim Preflight or run-start behavior; it establishes only provider-configuration and secure-secret-handling truth.
- W04 cannot begin exit assessment until W03 is implemented and validated.
- No wave may claim Phase 4 completion while provider configuration truth is mocked or while configuration-present is treated as equivalent to Preflight-ready or run-ready.

## Still Deferred
- Preflight feasibility truth and provider-model reachability checks
- run-start gating beyond provider configuration presence
- live provider inference behavior
- simulation execution, event sequencing, finalization, and reporting
- local interface authentication and engine secret-consumption behavior beyond the bounded retrieval-mediation foundation Phase 4 may prepare
- updater/runtime-delivery behavior and public API or OpenAPI expansion

## Packetization Rule
- Future Phase 4 packets must be large enough to implement, validate, critique, and hand off coherently.
- Future Phase 4 packets must cite this plan together with their upstream Document A, B, and C rules.
- No packet may treat stored provider configuration as proof that the selected provider or model is actually reachable or valid for run execution.