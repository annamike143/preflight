# Phase 5 Master Execution Plan

## Status
- Phase: P05
- Phase name: Seed Intake and Preflight
- Frozen by packet: `P05-WS05-T01`
- Date frozen: 2026-04-15
- Scope: phase-specific execution ordering, bounded seed-intake and Preflight basis, and future packetization guidance only

## Purpose
Phase 5 establishes truthful run eligibility.

## Governing Basis
- The single-seed and English-only input basis remains locked by `SCOPE-024`, `SCOPE-025`, `TERM-021` through `TERM-028`, and `AC-031` through `AC-040`.
- Preflight contract and outcome semantics remain governed by `FLOW-044` through `FLOW-054`, `BWORK-023` through `BWORK-026`, `BDEP-021`, `BDEP-022`, `BDEP-028`, `BDEP-029`, and `BDEP-037`.
- Phase ordering and closure remain governed by `BPHASE-028` through `BPHASE-032`.
- Technical naming and contract discipline remain governed by `CAPI-022` through `CAPI-024`, `CEVENT-018` through `CEVENT-022`, and the general Document C terminology rules.

## Phase 5 Entry Basis Frozen by This Plan
- Phase 4 handed off truthful provider-configuration readiness and secure local provider-secret handling without claiming Preflight approval, provider reachability, or model availability.
- Phase 3 handed off truthful activation and entitlement handling without collapsing activation readiness into Preflight eligibility.
- Phase 2 handed off runtime-readiness truth surfaces and shell-owned orchestration boundaries without materializing full run execution.
- The repo still lacks real seed-intake, Preflight, requested-versus-effective plan, stale-invalidation, and run-start-gating implementation.
- Phase 5 must establish those behaviors without prematurely implementing simulation execution, live run monitoring, finalization, reporting, or cleanup hardening.

## Bounded Seed-Intake and Preflight Path

### Ownership Split
- The frontend may collect one seed-document selection and bounded requested run inputs, display immediate obvious file issues, submit Preflight requests, and render Preflight outcomes.
- The Rust/Tauri shell remains the authoritative local owner of run-start gating, pending-run local orchestration, and the command boundary between frontend input and allowed local Preflight execution layers.
- The allowed local execution layer behind Preflight owns document validation, extractable-text checks, language detection, provider or model readiness probing, budget and duration estimation, downgrade or reject decisioning, and effective-plan derivation.
- The commercial backend remains bounded to entitlement and activation truth and must not become a persistence surface for seed documents, seed content, Preflight artifacts, run configuration history, or simulation artifacts.

### Storage-Domain Split
| Artifact class | Storage domain | Primary owner | Allowed contents | Explicit exclusions | Cleanup posture |
| --- | --- | --- | --- | --- | --- |
| Seed selection working state | transient in-memory UI and shell state only | frontend plus shell | selected file handle/path reference, display metadata, immediate obvious validation messages, requested run inputs before Preflight | durable raw seed mirrors, hidden file archives, vendor-hosted storage, routine logs with seed content | discarded when selection changes, Preflight is reset, or the pending configuration is abandoned |
| Preflight working artifacts | allowed local execution scratch or transient workspace only | shell plus allowed local execution layer | extracted text needed for validation, language-detection intermediates, estimation inputs, effective-plan derivation inputs, deterministic result assembly | commercial-backend storage, generic config files, hidden long-term archives, routine logs containing raw seed content | bounded to the local Preflight attempt and later governed workspace handling |
| Preflight result support state | app-controlled local support domain only if later packets require persistence beyond in-memory state | shell | requested input fingerprinting, current outcome classification, explicit rejection or downgrade metadata, stale-invalidation basis, effective-plan summary fields | raw provider secrets, raw activation tokens, hidden seed-content archives, transcript history, report artifacts, commercial ledger data | invalidated on material input change and cleared or refreshed by later governed lifecycle behavior |

- The exact storage mechanism for any durable Preflight result support state remains open at entry, but any later choice must stay within the app-controlled local support domain and must not become a hidden archive of raw seed content.
- No Phase 5 packet may normalize persistent storage of raw seed documents or extracted seed content in routine logs, generic config, the commercial backend, or undeclared archives.

### Preflight Sequence
1. The frontend submits one selected seed document and bounded requested run inputs through the approved shell-owned command boundary.
2. The shell confirms that the minimal local prerequisites for Preflight request submission exist and routes the request into the allowed local execution layer.
3. The allowed local execution layer performs document validation, text extraction, language detection, readiness checks, estimation, and effective-plan derivation.
4. The shell returns one governed Preflight outcome to the frontend: approved, approved with downgrade, or rejected, while preserving technical failure as a separate non-governed command failure class.
5. Run start becomes eligible only when the current Preflight result is start-eligible and any required downgrade has been accepted.
6. Material input changes invalidate the prior current Preflight result and it must not remain silently reusable as current truth.

## Preflight Semantics
- Preflight remains a separate runtime domain from runtime readiness, activation readiness, and provider configuration readiness.
- Seed selection is not equivalent to seed approval; a selected document becomes usable only if Preflight validates it.
- Requested plan and effective plan remain distinct; downgrade is a truthful constrained-approval path rather than a silent mutation of the user request.
- Rejection remains distinct from technical failure.
- A stale invalidated result remains distinct from never-run, in-progress, approved, approved with downgrade, rejected, and technical failure states.
- Run start may not be treated as truthful unless there is a current eligible Preflight result.

## Bounded Phase 5 Implementation Basis
- MVP run configuration remains bounded to one seed document, requested agent count, requested round count, max token budget, provider selection, and existing activation/provider/runtime prerequisites.
- Phase 5 must enforce supported file-type, file-size, extractable-text, and English-only rules truthfully.
- Phase 5 must surface meaningful rejection reasons for known governed rejection paths.
- Phase 5 must materialize requested-versus-effective plan logic and downgrade disclosure without smuggling simulation execution into Preflight.
- Phase 5 must materialize stale-result invalidation when material run inputs change.
- Phase 5 must block run start when eligible current Preflight truth is absent.

## Phase 5 Wave Plan

### P05-W01 — Seed Intake and Preflight Entry Basis and Master Execution Blueprint
- Status: COMPLETE via `P05-WS05-T01`.
- Outcome: the bounded seed-intake path, Preflight semantics, requested-versus-effective plan distinction, stale-invalidation basis, and later wave order are now frozen for implementation.

### P05-W02 — Seed Intake Contracts, Preflight Outcome Classes, and Run-Eligibility Truth Surfaces
- Objective: establish code-level seed-intake and Preflight contracts, outcome classes, invalidation semantics, and run-start-gating truth surfaces without pretending that real document validation or estimation behavior already exists.
- Required outputs:
  - seed-intake request and response contract surfaces
  - Preflight result and invalidation contract surfaces
  - requested-plan versus effective-plan contract basis
  - Preflight state and run-start eligibility truth surfaces integrated with the shell lifecycle and frontend truth surface
- Exit proof:
  - Preflight outcome classes are explicit and stable
  - run-start gating truth exists and does not bypass Preflight
  - requested versus effective plan semantics are explicit and downgrade is not hidden
- Planned packet anchor: `P05-WS05-T02`

### P05-W03 — Single-Seed Intake, Document Validation, and Deterministic Rejection Flow
- Objective: implement the real single-seed intake path, supported-type and file-size enforcement, extractable-text validation, English-only rejection, and deterministic known-rejection handling.
- Required outputs:
  - one-document-only intake flow
  - file metadata capture and obvious local failure handling
  - supported-type, size, extractable-text, and English-policy validation behavior
  - truthful rejection reasons for known document-validation failures
  - frontend and shell integration that reflects seed selection separately from Preflight approval
- Exit proof:
  - unsupported, oversized, unextractable, and non-English documents are handled truthfully
  - the frontend shows one selected document only and does not imply Preflight approval from mere selection
  - document-validation rejection remains distinct from technical failure
- Planned packet anchor: `P05-WS05-T03`

### P05-W04 — Effective-Plan Derivation, Stale Invalidation, and Run-Start Gating
- Objective: implement provider or model readiness checks during Preflight, requested-versus-effective plan derivation, downgrade or reject behavior, stale invalidation on material changes, and start-eligibility gating.
- Required outputs:
  - provider or model readiness checks inside Preflight
  - budget and duration estimation inputs and bounded effective-plan derivation
  - approved, approved-with-downgrade, and rejected Preflight outcomes with explicit user-visible downgrade disclosure
  - stale-current-result invalidation when material inputs change
  - run-start gating that requires current eligible Preflight truth and downgrade acceptance where applicable
- Exit proof:
  - approved with downgrade is distinct from full approval
  - rejected is distinct from technical error
  - stale results do not remain start-eligible after material changes
  - run start is blocked without current eligible Preflight truth
- Planned packet anchor: `P05-WS05-T04`

### P05-W05 — Phase 5 Validation, Critique, and Exit Gate
- Objective: prove Phase 5 truth against upstream seed-intake and Preflight acceptance and close only if the result is real.
- Required outputs:
  - acceptance mapping for `AC-031` through `AC-040`
  - evidence for English-only rejection, downgrade transparency, invalidation, and run-start gating
  - critique pass and in-scope remediation or escalation
- Exit proof:
  - non-English input is rejected truthfully
  - approved-with-downgrade is distinct from approval and from rejection
  - stale Preflight truth is invalidated on material change
  - run start cannot bypass current eligible Preflight truth
- Planned packet anchor: `P05-WS11-T01`

## Wave Dependency Rules
- W02 freezes contracts and state semantics needed by W03 and W04.
- W03 establishes deterministic document-validation and known-rejection behavior needed before W04 can claim truthful full Preflight outcomes.
- W04 may not claim simulation execution, live run monitoring, or finalization behavior; it establishes run eligibility only.
- W05 cannot begin exit assessment until W03 and W04 are implemented and validated.
- No wave may claim Phase 5 completion while seed selection is treated as seed approval, while downgrade is hidden, or while run start can bypass current eligible Preflight truth.

## Still Deferred
- real run/session creation
- persona generation
- simulation execution
- live monitoring and interruption handling
- finalization, report generation, and report access
- cleanup, shutdown hardening, and startup janitor behavior beyond earlier scaffolds
- updater/runtime-delivery behavior and public API or OpenAPI expansion

## Packetization Rule
- Future Phase 5 packets must be large enough to implement, validate, critique, and hand off coherently.
- Future Phase 5 packets must cite this plan together with their upstream Document A, B, and C rules.
- No packet may treat seed selection as seed approval.
- No packet may treat provider configuration presence as equivalent to Preflight approval.
- No packet may treat an invalidated or downgraded Preflight result as if it were a full unchanged approval.