# Phase 9 Master Execution Plan

## Status
- Phase: P09
- Phase name: Cleanup, Shutdown, and Startup Janitor Hardening
- Frozen by packet: `P09-WS09-T01`
- Date frozen: 2026-04-16
- Phase closure validated by packet: `P09-WS11-T01`
- Phase status: CLOSED — 2026-04-17
- Scope: phase-specific execution ordering, bounded cleanup/shutdown/janitor basis, and future packetization guidance only

## Purpose
Phase 9 establishes truthful post-run lifecycle closure and abnormal-termination cleanup handling.

## Governing Basis
- Cleanup and data-lifecycle acceptance obligations remain governed by `AC-092` through `AC-098`.
- Cleanup and janitor behavior remain governed by `BEHAV-156` through `BEHAV-166`.
- App lifecycle and run-cleanup state semantics remain governed by Document A Sections `7.3` and `7.8`, including `STATE-013`, `STATE-022`, `STATE-027` through `STATE-028`, `STATE-055`, `STATE-058`, `STATE-061`, and `STATE-181` through `STATE-186`.
- Cleanup scope, preservation boundaries, and local residual-risk rules remain governed by `DATA-073` through `DATA-089`.
- WS-09 ownership remains governed by `BWORK-039` through `BWORK-042`.
- Phase 9 ordering, outcomes, non-goals, and exit intent remain governed by `BPHASE-048` through `BPHASE-052` and Document B Section `6.11.9` and `6.11.10`.
- Cleanup/janitor technical distinction and scope-mapping rules remain governed by `CTERM-018`, `CEVENT-027` through `CEVENT-029`, `CDATA-046` through `CDATA-048`, and `CREPORT-048`.

## Phase 9 Entry Basis Frozen by This Plan
- Phase 7 handed off shell-owned terminal run-state truth, controlled cancellation, active-run shutdown coordination, and app-close lifecycle control without cleanup hardening.
- Phase 6 handed off real fresh-run workspace allocation, transient run-local artifact production, and no-cross-run workspace reuse boundaries.
- Phase 8 handed off truthful completion/report behavior, durable report artifacts outside the transient workspace, and the explicit distinction between report availability and later cleanup outcome.
- The current repo already exposes canonical cleanup lifecycle vocabulary, including `Cleaning`, `Cleaned`, and `CleanupFailed`, but those states are not yet emitted as live product truth because no real cleanup path exists.
- The app lifecycle already reserves `StartupJanitorRunning` before readiness checks and already forbids `AppLaunching -> RunActive`, but no real startup janitor scan or purge behavior exists yet.
- The current repo still lacks cleanup triggering after terminal run paths, cleanup-result classification and exposure, startup janitor purge of abandoned workspaces, and explicit no-resume preservation through cleanup/janitor behavior.
- Phase 9 must materialize truthful cleanup and janitor behavior without redefining run success, without deleting preserved exported reports, without introducing crash-resume, and without overclaiming forensic deletion guarantees.

## Bounded Cleanup, Shutdown, and Janitor Path

### Ownership Split
- The frontend may render shell-owned cleanup status, janitor outcome, shutdown warnings, and residual-workspace diagnostics only when the shell exposes those truths. It does not infer cleanup success, scan the filesystem, or decide whether janitor purge was safe.
- The Rust/Tauri shell remains the authoritative local owner of cleanup triggering, cleanup ordering, cleanup outcome classification, active-run shutdown coordination, startup janitor execution, residual workspace purge decisions, and no-resume preservation at lifecycle boundaries.
- The Python engine remains the producer of run-local artifacts and controlled termination cooperation signals only. It does not own cleanup policy, janitor policy, or externally exposed cleanup/janitor truth.
- The commercial backend remains out of the cleanup/janitor path except for already-governed activation/runtime authorization responsibilities; it must not become a hosted workspace archive, recovery coordinator, or janitor authority.
- Phase 9 may finalize truthful cleanup-result and janitor truth, but it must not overclaim forensic deletion, hosted backup/recovery, updater hardening, or release readiness.

### Storage-Domain and Cleanup Split
| Data class | Storage domain | Primary owner | Cleanup applies | Janitor applies | Explicit exclusions |
| --- | --- | --- | --- | --- | --- |
| Transient run workspace data | per-run workspace root under the shell-owned local workspace boundary | shell | yes | yes | exported report path, secure-store/configuration domains, and durable app support data outside the workspace |
| Run-local coordination metadata used only for cleanup or janitor behavior | workspace-local metadata or tightly bounded shell-managed transient coordination records | shell | yes when transient | yes when abandoned | provider secrets, activation artifacts, durable continuity stores, and hidden cross-run reuse |
| Preserved exported report artifact | durable local report/export path outside the transient workspace boundary | shell mediates path/access | no | no | must not be reclassified as cleanup-targeted workspace data |
| Durable app support and secure-store data | app-controlled support domain plus OS-native secure storage | shell | no | no | not part of run cleanup or janitor sweep |

- Cleanup and janitor may rely on shell-owned workspace manifests, residual markers, or bounded coordination records, but those artifacts must not become a hidden continuity or crash-resume store.
- Report preservation remains distinct from cleanup scope. Any intentionally preserved report artifact outside the transient workspace must survive both normal cleanup and startup janitor behavior.
- Cleanup scope must remain explicit enough for later packetization to determine what is purge-targeted, what is janitor-targeted, and what remains intentionally outside cleanup scope.

### Cleanup and Janitor Semantics
- Cleanup is the best-effort purge attempt for transient run-local workspace material after a governed terminal path or an app-close sequence affecting an active run. It is a post-run responsibility, not a substitute run outcome.
- Cleanup may begin only after active process termination and file-lock conditions make deletion appropriately possible or after authorized forceful shutdown fallback has been applied where governed.
- `Cleaning` means a cleanup attempt is in progress. `Cleaned` means cleanup succeeded sufficiently under the best-effort policy. `CleanupFailed` means cleanup did not fully succeed and that fact must remain visible.
- Startup janitor is the next-launch abandoned-workspace discovery and purge path that runs before normal readiness checks. It may time out safely or fail partially, but it must not be skipped silently when abnormal termination residue is present.
- Janitor behavior may record bounded local diagnostics or residual classifications, but it must not restore interrupted runs, reconstruct in-progress execution, or seed later runs with abandoned working data.
- Active app close during a run must coordinate controlled termination, anti-zombie handling where required, and either immediate cleanup or truthful handoff to later janitor behavior if normal cleanup cannot complete before close.
- No Phase 9 packet may claim forensic-grade erasure, guaranteed cleanup success under all OS conditions, or any form of crash resume.

## Bounded Phase 9 Implementation Basis
- Phase 9 must materialize shell-owned cleanup triggers for `Completed`, `CompletedBudgetCapped`, `Canceled`, `FailedTerminal`, and governed app-close paths affecting active runs.
- Phase 9 must materialize cleanup execution ordering, cleanup outcome classification, and status/event/lifecycle exposure that remain distinct from run outcome truth.
- Phase 9 must materialize cleanup-scope mapping across transient workspace data, cleanup-exempt preserved report artifacts, and any bounded shell-owned coordination metadata.
- Phase 9 must materialize startup janitor discovery, purge attempts, safe timeout or partial-failure visibility, and explicit no-resume preservation for abandoned workspaces after abnormal termination.
- Phase 9 must materialize anti-zombie supervision integration where active-run shutdown or abnormal termination would otherwise leave residual child-process or lock conditions that block truthful cleanup behavior.
- Phase 9 may integrate bounded frontend cleanup or janitor messaging and diagnostics surfaces, but it must not give the frontend filesystem authority, janitor authority, or lifecycle authority.
- Phase 9 does not authorize updater/distribution hardening, hosted backup/archive behavior, forensic deletion claims, crash-resume support, release readiness, or new report/library scope.

## Phase 9 Wave Plan

### P09-W01 — Cleanup, Shutdown, and Startup Janitor Hardening Entry Basis and Master Execution Blueprint
- Status: COMPLETE via `P09-WS09-T01`.
- Outcome: the bounded Phase 9 cleanup/shutdown/janitor path, ownership split, preservation boundary, and later wave order are now frozen for implementation.

### P09-W02 — Cleanup Outcome, Scope Mapping, and Janitor Truth Surface Foundation
- Objective: establish code-level cleanup and janitor contract surfaces, cleanup-result and janitor-result vocabulary, cleanup-scope mapping basis, report-preservation enforcement rules, and shell-owned truth surfaces without yet implementing real purge behavior.
- Required outputs:
  - cleanup and janitor contract surfaces
  - cleanup-result and janitor-result state/event vocabulary
  - cleanup-scope mapping basis aligned to transient workspace, preserved report/output, and durable support/secure-store domains
  - shell lifecycle, startup, local-interface, and frontend truth-surface updates needed to freeze the vocabulary without overclaiming real cleanup or janitor execution yet
- Exit proof:
  - contract surfaces preserve the distinction between run outcome, report availability, cleanup attempt, cleanup outcome, and janitor outcome
  - cleanup applicability and report-preservation exceptions are not ambiguous
  - no contract implies crash-resume, forensic erasure, or guaranteed cleanup success
- Planned packet anchor: `P09-WS09-T02`

### P09-W03 — Terminal Cleanup Triggering, Outcome Classification, and Report Preservation Enforcement
- Objective: implement the normal terminal cleanup path, cleanup ordering, cleanup-result classification, and preserved-report enforcement across completed, constrained-completion, canceled, and terminal-failure paths without yet claiming abnormal-termination janitor completeness.
- Required outputs:
  - real cleanup trigger after governed terminal run paths
  - cleanup execution ordering aligned to active process termination and file-lock truth
  - shell-owned cleanup outcome classification and status/event integration
  - preserved exported report enforcement across cleanup attempts
  - bounded frontend or diagnostics rendering of shell-owned cleanup truth where meaningful
- Exit proof:
  - cleanup is attempted after governed terminal run paths
  - cleanup failure is visible rather than silently hidden
  - intentionally preserved reports outside the transient workspace are not deleted
  - cleanup success is not treated as run success and cleanup failure is not used to rewrite run outcome truth
- Planned packet anchor: `P09-WS09-T03`

### P09-W04 — App-Close Shutdown Integration, Abnormal-Termination Residual Detection, and Startup Janitor Execution
- Objective: implement app-close cleanup coordination for active runs, abnormal-termination residual detection, startup janitor purge behavior, anti-zombie integration where required, and explicit no-resume preservation.
- Required outputs:
  - app-close integration from `AppShuttingDown` into truthful termination plus cleanup or janitor handoff behavior
  - abnormal-termination residual detection and bounded marker or manifest handling for later janitor use
  - startup janitor scan and purge attempts with safe timeout/failure visibility before readiness checks continue
  - no-resume enforcement and anti-zombie supervision alignment with shell-owned lifecycle control
  - bounded diagnostics or event surfaces exposing janitor outcome where meaningful
- Exit proof:
  - active app close coordinates truthful termination handling rather than leaving silent leftovers
  - the next launch attempts stale-workspace purge after abnormal termination
  - janitor behavior does not imply run restoration or hidden continuity
  - shutdown and janitor behavior do not violate preserved-report boundaries
- Planned packet anchor: `P09-WS09-T04`

### P09-W05 — Phase 9 Validation, Critique, and Exit Gate
- Objective: prove Phase 9 truth against upstream cleanup, no-resume, report-preservation, janitor, and shutdown acceptance and close only if the result is real.
- Required outputs:
  - acceptance mapping for `AC-092` through `AC-098`
  - evidence for cleanup triggering, cleanup failure visibility, startup janitor attempts, no-resume preservation, and report-preservation boundaries
  - critique pass and in-scope remediation or escalation
- Exit proof:
  - cleanup is attempted after terminal paths
  - cleanup failure is not silently hidden
  - next-launch janitor attempts stale purge after abnormal termination
  - the product still does not offer crash-resume
  - exported reports are not deleted as part of workspace purge
- Planned packet anchor: `P09-WS11-T01`

## Wave Dependency Rules
- W02 freezes cleanup/janitor vocabulary, scope mapping, and truth-surface contracts needed by W03 through W05.
- W03 establishes the normal terminal cleanup path and cleanup-result truth before W04 can claim coherent abnormal-termination or startup-janitor behavior.
- W04 establishes startup janitor execution, shutdown integration, and no-resume preservation before W05 can assess truthful lifecycle-closure behavior.
- W04 may integrate anti-zombie supervision only as needed for truthful cleanup/janitor behavior; it may not reopen Phase 7 monitoring scope or Phase 10 updater scope.
- W05 cannot begin exit assessment until W03 and W04 are implemented and validated.
- No wave may collapse run outcome and cleanup outcome, may delete intentionally preserved reports, or may treat janitor purge as resume or recovery.

## Still Deferred
- updater/runtime-delivery hardening and release readiness
- forensic-grade deletion or stronger privacy guarantees than the best-effort policy allows
- crash-resume or session restoration behavior
- hosted backup, archive, or sync behavior for run workspaces or reports
- public API or OpenAPI expansion

## Packetization Rule
- Future Phase 9 packets must be large enough to implement, validate, critique, and hand off coherently.
- Future Phase 9 packets must cite this plan together with their upstream Document A, B, and C rules.
- No packet may let the frontend become the authority for cleanup success, janitor outcome, or filesystem purge decisions.
- No packet may treat cleanup success as run success or cleanup failure as a rewrite of an otherwise distinct run outcome.
- No packet may reclassify preserved report artifacts or secure-store/support data as transient cleanup targets without upstream authorization.
- No packet may introduce crash-resume, hidden continuity, or new durable abandoned-workspace archives under cleanup or janitor language.