# Phase 8 Master Execution Plan

## Status
- Phase: P08
- Phase name: Finalization, Report Generation, and Report Access
- Frozen by packet: `P08-WS08-T01`
- Date frozen: 2026-04-16
- Phase closure validated by packet: `P08-WS11-T01`
- Phase status: CLOSED — 2026-04-16
- Scope: phase-specific execution ordering, bounded finalization/report basis, and future packetization guidance only

## Purpose
Phase 8 establishes truthful run completion output.

## Governing Basis
- Final-report and output obligations remain governed by `AC-086` through `AC-091`.
- Report/cleanup distinction obligations that Phase 8 must preserve without claiming cleanup hardening remain governed by `AC-093` and `AC-097`.
- Finalization and report-generation behavior remain governed by `BEHAV-077`, `BEHAV-108`, and `BEHAV-146` through `BEHAV-155`.
- Finalization/completion state semantics remain governed by Document A Section `7.8`, including the completed, constrained-completion, and terminal-failure state family plus their guarded transition rules.
- WS-08 ownership remains governed by `BWORK-035` through `BWORK-038`.
- Phase 8 ordering, outcomes, non-goals, and exit intent remain governed by `BPHASE-043` through `BPHASE-047` and Document B Section `6.10.9` and `6.10.10`.
- Report-availability and access contract rules remain governed by `API-081` through `API-084` and `UI-066` through `UI-069`.
- Report persistence and cleanup-boundary rules remain governed by `DATA-047` through `DATA-051`, `DATA-073` through `DATA-079`, and `DATA-081`.

## Phase 8 Entry Basis Frozen by This Plan
- Phase 7 handed off shell-owned lifecycle, status, ordered event history, in-flight run/session identity, runtime cap control, constrained-completion classification, and cancellation/shutdown truth without report-generation behavior.
- The current repo can now produce meaningful run artifacts, including transcript archives, bounded memory summaries, tracked token/duration usage, finalization timestamps, and runtime-domain chronology, but it still cannot transform those artifacts into a truthful final report.
- The Python engine already owns finalization and report-content synthesis inputs at the architecture level, but no live implementation yet computes the viability score, assembles required report sections, or materializes a final PDF artifact.
- The Rust/Tauri shell already owns privileged run orchestration and external truth surfaces, but it does not yet expose shell-owned report availability, durable report location truth, or local report access behavior.
- The frontend currently renders truthful run monitoring and interruption state, but it does not yet render shell-owned report-availability or report-access affordances.
- Phase 8 must turn actual completed run material into truthful output behavior without fabricating missing debate content, without granting report authority to the frontend, and without collapsing report availability into cleanup success or Phase 9 cleanup hardening.

## Bounded Finalization and Report Output Path

### Ownership Split
- The frontend may render shell-owned report availability, local access affordances, and constrained-completion messaging only when the shell exposes those truths. It does not infer report existence, fabricate report readiness, or generate files itself.
- The Rust/Tauri shell remains the authoritative local owner of externally exposed completion truth, report-availability state, local report access/export commands, durable-path mediation, and the distinction between run outcome truth and cleanup truth.
- The Python engine remains the producer of finalization and report-content synthesis inputs, including viability-score generation, required report-section assembly inputs, transcript-appendix assembly inputs, and tracked usage summaries derived from actual run artifacts.
- The commercial backend remains out of the report path except for already-governed activation/runtime authorization responsibilities; it must not become a hosted report store, report-generation coordinator, or report-access authority.
- Phase 8 may establish truthful report persistence/access distinction, but Phase 9 still owns cleanup hardening, janitor behavior, and cleanup outcome validation.

### Storage-Domain and Output Split
| Artifact class | Storage domain | Primary owner | Allowed contents | Explicit exclusions | Lifecycle posture |
| --- | --- | --- | --- | --- | --- |
| Finalization working state | transient run workspace plus transient shell/engine memory | shell plus engine | actual transcript/event material, bounded summaries, tracked usage, viability-score inputs, section-assembly intermediates, report-generation status | provider secrets, fabricated turns, hidden cloud sync, durable report claims before success | transient and later subject to Phase 9 cleanup policy |
| Report-availability state | shell-owned state and bounded local support metadata only if later packets require persistence beyond memory | shell | report generation success/failure truth, accessible local path truth, generation/file failure classification, constrained-completion flag | phantom report-ready state, cleanup-success implication, frontend-authored availability | exposed to the frontend only when truthful |
| Final exported report | intentional durable local path outside the transient workspace when user retention is expected | shell mediates path/access; engine supplies content inputs | structured PDF containing only approved report sections derived from actual completed run material | provider secrets, invented rounds/turns, placeholder PDFs, hidden vendor-hosted copies | durable local artifact may persist beyond workspace cleanup |

- The transcript appendix may be assembled from actual transcript and chronology artifacts, but this does not authorize an undeclared durable transcript store beyond the approved final report and minimized logging policy.
- Report generation success and report accessibility are related but distinct truths: a later packet must preserve the difference between a report that was generated, a report that is accessible locally, and a report that failed during generation or file materialization.

### Finalization and Report Semantics
- Finalization means the controlled phase in which normal round scheduling has stopped and the system shifts from debate execution to truthful synthesis, viability-score generation, report-section assembly, and terminal outcome resolution.
- Successful completion and budget-capped completion are valid only when the required report is generated successfully from actual completed material. If report generation fails, the run must not remain classified as successful completion.
- Constrained-completion reports must reflect only the actually completed portion of the run and must not invent nonexistent rounds, speakers, turns, arguments, or summary content to appear more complete.
- The Market Viability Score is a model-mediated synthesis output generated under finalization authority. It must not be presented as deterministic or objective market truth.
- Report availability must remain distinct from cleanup status. Cleanup may later remove transient workspace artifacts, but it must not be used as shorthand for whether the exported report exists or remains accessible.
- No Phase 8 packet may treat an empty, placeholder, or materially incomplete PDF as successful final report generation.

## Bounded Phase 8 Implementation Basis
- Phase 8 must materialize finalization entry behavior that consumes actual completed run artifacts and resolves terminal success versus constrained success versus failure truth without reopening normal round scheduling.
- Phase 8 must materialize engine-owned viability-score generation and required report-section assembly from actual transcript, bounded-memory, event-history, and tracked-usage artifacts.
- Phase 8 must materialize truthful cost/token summary inclusion based on tracked usage rather than invented totals for unfinished work.
- Phase 8 must materialize transcript appendix generation from actual ordered run material rather than fabricated or silently reconstructed dialogue.
- Phase 8 must materialize a real structured PDF generation path and a durable local report artifact outside the transient workspace when retention is expected.
- Phase 8 must materialize shell-owned report availability/status integration and local report access behavior without giving the frontend report authority.
- Phase 8 must preserve the distinction between successful full completion, constrained completion, report-generation failure, and later cleanup outcome.
- Phase 8 may integrate bounded UI report access affordances and local open/export behavior, but it must not claim cleanup hardening, janitor completeness, hosted report sync, browser report portals, or release readiness.
- Phase 8 does not authorize hosted report storage, cloud report sync, alternate export formats beyond the governed PDF output, crash-resume behavior, or Phase 9 cleanup hardening.

## Phase 8 Wave Plan

### P08-W01 — Finalization, Report Generation, and Report Access Entry Basis and Master Execution Blueprint
- Status: COMPLETE via `P08-WS08-T01`.
- Outcome: the bounded Phase 8 output path, ownership split, persistence boundary, and later wave order are now frozen for implementation.

### P08-W02 — Finalization and Report Output Contract Foundation
- Status: COMPLETE via `P08-WS08-T02`.
- Objective: establish code-level finalization/report contract surfaces, report-required-section basis, report-availability state shape, durable report boundary, and local access/export contract semantics without yet implementing report synthesis or PDF generation.
- Required outputs:
  - report-generation and report-availability contract surfaces
  - finalization entry/exit contract basis for successful completion, constrained completion, and report-generation failure
  - required report-section schema and section-assembly contract basis
  - durable report location and local access/export command contract basis
  - shell lifecycle, interface, and frontend truth-surface updates needed to freeze the vocabulary without overclaiming live report generation yet
- Exit proof:
  - the contracts distinguish report generated, report accessible, and report unavailable due to generation or file error
  - the contracts preserve the distinction between report availability and cleanup outcome
  - no contract implies cleanup success, janitor completion, hosted report sync, or alternate export formats
- Packet anchor: `P08-WS08-T02`

### P08-W03 — Finalization, Viability Score, and Report Data Assembly
- Status: COMPLETE via `P08-WS08-T03`.
- Objective: implement the engine-owned finalization and report-data assembly path from actual run artifacts, including viability-score generation and required section assembly, without yet claiming the final local PDF/access workflow.
- Required outputs:
  - finalization entry behavior for successful and constrained-completion paths
  - Market Viability Score generation pathway under finalization authority
  - required report-section assembly from bounded memory, transcript/event artifacts, and tracked usage
  - truthful transcript appendix and cost/token summary assembly from actual artifacts only
  - shell integration for report-generation success/failure classification based on real assembly results
- Exit proof:
  - assembled report content derives only from actual completed run material
  - constrained-completion report data remains explicitly constrained rather than full-completion fiction
  - report-generation failure remains distinct from successful completion
- Packet anchor: `P08-WS08-T03`

### P08-W04 — PDF Generation, Report Availability, and Local Access Integration
- Status: COMPLETE via `P08-WS08-T04`.
- Objective: materialize the final PDF generation path, shell-owned report availability/status integration, durable local report output boundary, and frontend report-access affordances without implying cleanup success or hosted report storage.
- Required outputs:
  - real PDF generation from assembled report data
  - durable local report artifact written outside the transient workspace when retention is expected
  - shell-owned report availability state and local access/open/export commands
  - frontend rendering of report availability and report access affordances
  - truthful distinction among report generated successfully, report accessible locally, and report unavailable because generation or file access failed
- Exit proof:
  - successful and constrained-completion runs can yield an accessible local report
  - the UI shows report access affordances only when the file actually exists and is accessible
  - report persistence remains distinct from transient workspace cleanup
- Packet anchor: `P08-WS08-T04`

### P08-W05 — Phase 8 Validation, Critique, and Exit Gate
- Status: COMPLETE via `P08-WS11-T01`.
- Objective: prove Phase 8 truth against upstream finalization, report generation, report availability, and report/cleanup distinction acceptance and close only if the result is real.
- Required outputs:
  - acceptance mapping for `AC-086` through `AC-091` plus the Phase 8-relevant distinction obligations `AC-093` and `AC-097`
  - evidence for truthful report generation, required sections, report truthfulness, report availability, cost-summary integrity, and durable report/access distinction
  - critique pass and in-scope remediation or escalation
- Exit proof:
  - successful and constrained-completion runs can produce truthful reports
  - report content is based on actual run material and placeholder/empty PDFs are not treated as success
  - report availability is truthful and remains distinct from cleanup status
  - Phase 8 does not overclaim Phase 9 cleanup hardening or janitor completion
- Packet anchor: `P08-WS11-T01`

## Wave Dependency Rules
- W02 freezes the finalization/report vocabulary, artifact boundary, and access contract surfaces needed by W03 through W05.
- W03 establishes the real engine-owned finalization and report-data assembly basis before W04 can claim truthful PDF generation or report availability behavior.
- W04 establishes the final PDF artifact, durable output boundary, and local access behavior before W05 can assess truthful report availability and completion claims.
- W04 may preserve the distinction between report persistence and cleanup, but it may not claim cleanup hardening, janitor completion, or cleanup outcome validation as complete.
- W05 cannot begin exit assessment until W03 and W04 are implemented and validated.
- No wave may treat debate completion alone as successful completion, may fabricate missing report content, or may assert report availability without an accessible local artifact.

## Still Deferred
- cleanup hardening, startup janitor behavior, and cleanup outcome validation
- hosted/cloud report library or report sync behavior
- alternate export formats beyond the governed PDF output
- updater/runtime-delivery hardening and release readiness
- public API or OpenAPI expansion

## Packetization Rule
- Future Phase 8 packets must be large enough to implement, validate, critique, and hand off coherently.
- Future Phase 8 packets must cite this plan together with their upstream Document A, B, and C rules.
- No packet may let the frontend become the authority for report existence, report readiness, or completion classification.
- No packet may treat successful debate execution without real report generation as a valid completed-report path.
- No packet may fabricate missing rounds, turns, arguments, or appendix content to make the report appear more complete.
- No packet may collapse report availability, report persistence, and cleanup outcome into one undifferentiated success label.