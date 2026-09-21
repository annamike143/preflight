# Phase 6 Master Execution Plan

## Status
- Phase: P06
- Phase name: Simulation Engine Core
- Frozen by packet: `P06-WS06-T01`
- Date frozen: 2026-04-15
- Scope: phase-specific execution ordering, bounded run/session-core basis, and future packetization guidance only

## Purpose
Phase 6 establishes truthful bounded moderated run execution.

## Governing Basis
- Fresh-run, workspace-isolation, runtime persona-generation, moderated-debate, and bounded-memory truth remain governed by `AC-041` through `AC-059`.
- WS-06 ownership remains governed by `BWORK-027` through `BWORK-030`.
- Phase ordering, minimum outcomes, non-goals, and exit intent remain governed by `BPHASE-033` through `BPHASE-037`.
- Phase 6 validation and reopen conditions remain governed by Document B Section `6.8.9` and Section `6.8.10`.
- Shell-owned run-start gating still depends on the truthful runtime, activation, provider-configuration, and Preflight surfaces handed off by earlier closed phases.

## Phase 6 Entry Basis Frozen by This Plan
- Phase 5 handed off truthful current-eligible Preflight state, requested-versus-effective plan truth, downgrade acceptance, stale invalidation, and run-start gating without claiming real run execution.
- Phase 2 handed off shell-owned runtime-readiness, trust-boundary, supervision, and lifecycle foundations without claiming active simulation behavior.
- Phase 3 and Phase 4 handed off truthful activation and provider-configuration dependencies without collapsing those readiness domains into run execution.
- The repo still lacks real run/session creation, fresh workspace allocation, seed ingestion for execution, runtime persona generation, Moderator-controlled rounds, bounded run memory, and run token or duration tracking.
- Phase 6 must establish those behaviors without overclaiming Phase 7 live monitoring fidelity, Phase 8 finalization or report truth, or Phase 9 cleanup hardening.

## Bounded Run-Execution Path

### Ownership Split
- The frontend may request run start only from current eligible Preflight truth, show coarse shell-owned run truth, and later consume richer run-state or report surfaces only when later phases authorize them. It does not own run/session identity, workspace allocation, persona authority, Moderator decisions, or active memory.
- The Rust/Tauri shell remains the authoritative local owner of run-start authorization, fresh run/session identity creation, fresh isolated workspace allocation, Preflight-lineage snapshot capture, engine launch and supervision boundaries, and any coarse run-truth surfaced outside the engine.
- The Python engine owns seed ingestion for execution, runtime persona generation, exactly one Moderator, moderated round behavior, bounded memory maintenance, argument/risk continuity, and run-local token or duration tracking once implementation packets begin.
- The commercial backend remains bounded to activation truth and must not become a hosted run coordinator, workspace store, transcript archive, persona store, or memory surface.

### Storage-Domain Split
| Artifact class | Storage domain | Primary owner | Allowed contents | Explicit exclusions | Cleanup posture |
| --- | --- | --- | --- | --- | --- |
| Run-start working state | transient frontend and shell memory only | frontend plus shell | current eligible Preflight reference, start intent, user-visible start error context, bounded effective-plan confirmation data | durable transcripts, provider secrets, activation tokens, hidden continuation state, cross-run memory | discarded when start succeeds, fails, or is abandoned |
| Active run workspace | fresh isolated per-run workspace only | shell plus engine | approved execution-seed material derived from current Preflight lineage, runtime persona set, round artifacts, bounded memory artifacts, transcript archive, token or duration counters, intermediate execution output | provider secrets, activation tokens, reused workspace identity, cross-run semantic memory, hidden provider archives | best-effort ephemeral until later governed cleanup behavior |
| Run support metadata | app-controlled local support domain only if later packets require persistence beyond memory | shell | run/session identifier, workspace reference, coarse run-state fields, timestamps, terminal reason codes, Preflight-lineage fingerprint, bounded usage counters if needed for shell truth | raw seed-content archives, raw provider secrets, activation tokens, transcript replay payloads, semantic memory state, report bodies, crash-resume continuation state | bounded and must not authorize hidden continuation or crash-resume |
| Transcript archive | active run workspace during execution and only later a separately governed preservation domain if future phases authorize it | engine plus shell | append-only speaker-attributed turns, round boundaries, and raw run transcript visibility artifacts | use as unlimited prompt context, fabricated turns, cross-run mixing, secret material | remains distinct from bounded prompt memory; long-term preservation is deferred |

- The exact mechanism for any durable run-support metadata remains open at Phase 6 entry, but it must stay inside the app-controlled local support domain and must not become hidden crash-resume or cross-run memory.
- The transcript archive may exist for visibility and later report appendix use, but it must remain structurally separate from the bounded memory layers used for inference.

### Run Execution Sequence
1. The frontend requests run start only against current eligible Preflight truth and any already-accepted downgrade basis.
2. The shell rechecks runtime readiness, activation readiness, provider-configuration readiness, and current Preflight eligibility before authorizing execution.
3. The shell creates a fresh run/session identity and a fresh isolated workspace; if either step fails, the run does not begin.
4. The shell freezes a run-start snapshot from current Preflight truth, including approved seed lineage and effective-plan basis, and passes only the approved execution inputs into the engine boundary.
5. The engine ingests the approved seed execution context, generates the runtime persona set with exactly one Moderator and stable persona labels, and initializes bounded memory from the approved seed context rather than from any prior run.
6. The engine executes bounded moderated rounds, with the Moderator selecting 3 to 5 speakers per normal round while updating truthful round-summary and argument/risk memory.
7. The engine tracks token usage and duration participation as part of run progression and surfaces coarse progress or terminal truth back through the shell boundary.
8. Transcript capture may exist for visibility and later reporting, but it remains distinct from bounded prompt context and cannot act as unlimited replay memory.
9. Later phases own richer live monitoring, interruption handling, finalization, report generation, export, cleanup closure, and release hardening.

## Phase 6 Semantics
- Fresh run creation means every run begins from a new run/session identity and must not silently continue prior semantic state.
- Fresh workspace allocation means each run receives a new isolated execution workspace, and no prior run workspace may be reused as the active execution context for a later run.
- Preflight lineage is mandatory execution input: run start must derive from the current eligible Preflight basis rather than from ad hoc seed selection or stale historical state.
- Persona generation must be runtime-driven from approved seed context rather than from a static user-authored roster.
- Exactly one Moderator must exist per run, and the non-Moderator persona set must provide meaningful favorable, skeptical, and analytical or neutral perspective diversity.
- Persona outputs must remain attributable to stable persona labels within the run even before later phases add richer monitoring or reporting surfaces.
- Normal rounds must remain Moderator-controlled 3-to-5-speaker subsets rather than all-agent full fanout.
- Round-summary continuity and argument/risk continuity are required bounded memory outputs of the run itself rather than later report-only reconstruction.
- Full transcript visibility may exist, but the transcript archive must not become the default unlimited prompt source for every inference step.
- Token and duration tracking must participate structurally in run behavior rather than appearing only as after-the-fact reporting metadata.
- Cross-run semantic leakage is forbidden: no prior-run memory, persona state, summary state, or transcript content may become active memory for a later run.

## Bounded Phase 6 Implementation Basis
- Phase 6 must materialize a shell-owned run-start path that fails truthfully whenever current eligible Preflight truth is absent, stale, or no longer aligned with readiness prerequisites.
- Phase 6 must materialize fresh run/session identity and fresh workspace allocation before active engine execution begins.
- Phase 6 must materialize seed ingestion for execution from the approved Preflight lineage rather than by bypassing or redefining the closed Phase 5 submission and approval path.
- Phase 6 must materialize runtime persona generation, exactly one Moderator, stable persona labels, and required perspective diversity without introducing a static user-authored persona configuration surface.
- Phase 6 must materialize bounded memory layers for seed grounding, round-summary continuity, and argument/risk continuity without using naive full-transcript replay as the core memory model.
- Phase 6 must materialize speaker-attributed moderated round execution with 3-to-5-speaker selection as the normal rule.
- Phase 6 must materialize run-local token and duration tracking in a way the shell can supervise truthfully without collapsing WS-06 ownership into shell-only process control.
- Phase 6 may expose only the coarse run-state truth needed for shell supervision and validation; full user-visible monitoring fidelity belongs to Phase 7.
- Phase 6 may produce bounded run artifacts needed for later finalization, but final report truth and export remain Phase 8 concerns.
- Phase 6 does not authorize cleanup hardening, startup janitor behavior, updater work, release readiness, hidden persistence, or crash-resume behavior.

## Phase 6 Wave Plan

### P06-W01 — Simulation Engine Core Entry Basis and Master Execution Blueprint
- Status: COMPLETE via `P06-WS06-T01`.
- Outcome: the bounded Phase 6 run/session path, workspace-isolation basis, Preflight-lineage basis, runtime persona semantics, Moderator model, bounded memory basis, and later wave order are now frozen for implementation.

### P06-W02 — Run/Session Contracts, Workspace Isolation, and Start-Lineage Foundation
- Objective: establish code-level run/session contract surfaces, workspace-allocation and Preflight-lineage contracts, shell-engine start-boundary truth, and coarse run-state truth surfaces without pretending that real execution already exists.
- Required outputs:
  - run/session start request and response contract surfaces
  - run/session identifier, workspace-reference, and terminal-state contract basis
  - Preflight-lineage snapshot contract basis for execution start
  - shell-engine start-boundary and coarse run-state truth surfaces integrated with lifecycle and trust-boundary layers
- Exit proof:
  - run start contractually depends on current eligible Preflight truth
  - fresh run/session identity and fresh workspace semantics are explicit
  - no contract surface implies live monitoring fidelity, final reports, cleanup closure, or crash-resume support
- Planned packet anchor: `P06-WS06-T02`

### P06-W03 — Fresh Run Creation, Workspace Allocation, and Seed Execution Ingestion
- Objective: implement the shell-owned fresh run creation path, fresh workspace allocation, Preflight-lineage capture, and seed-ingestion handoff into the engine boundary without yet claiming runtime persona generation or full moderated rounds.
- Required outputs:
  - real run/session creation and start authorization path
  - fresh isolated workspace allocation and no-active-reuse enforcement
  - approved Preflight-lineage snapshot capture at run start
  - seed execution-ingestion handoff into the engine boundary
  - truthful failure handling when run start prerequisites are missing or workspace/start initialization fails
- Exit proof:
  - each approved run receives a fresh identity and fresh isolated workspace
  - run execution cannot begin from stale or absent Preflight truth
  - execution seed input comes from the approved lineage rather than bypassing Phase 5
- Planned packet anchor: `P06-WS06-T03`

### P06-W04 — Runtime Persona Generation, Moderator Basis, and Bounded Memory Foundation
- Objective: implement runtime persona generation from approved seed context, exactly one Moderator, stable persona labeling, perspective diversity, and bounded memory structures needed for moderated execution without yet claiming full Phase 7 monitoring behavior or Phase 8 report behavior.
- Required outputs:
  - runtime-generated persona-set construction from approved seed context
  - exactly one Moderator and participant persona labeling model
  - favorable, skeptical, and analytical or neutral perspective coverage basis
  - bounded seed-grounding, round-summary, and argument/risk memory structures
  - transcript-archive versus bounded-memory separation inside the execution path
- Exit proof:
  - personas are runtime-generated rather than static-user-authored
  - exactly one Moderator exists and persona outputs remain attributable within the run
  - bounded memory is real and is not replaced by naive full-transcript replay
- Planned packet anchor: `P06-WS06-T04`

### P06-W05 — Moderated Round Execution and Token/Duration Tracking
- Objective: implement the active bounded moderated run path, including per-round 3-to-5-speaker selection, speaker-attributed turn execution, round-summary and argument/risk continuity updates, and integrated token or duration tracking.
- Required outputs:
  - active Moderator-controlled multi-round execution path
  - per-round 3-to-5-speaker selection as the normal round rule
  - speaker-attributed turn capture and truthful round-summary updates
  - bounded argument/risk continuity updates across rounds
  - run-local token-usage and duration-tracking integration surfaced through shell-owned supervision truth
- Exit proof:
  - normal rounds are not all-agent full fanout
  - outputs are speaker-attributed and not fabricated to conceal missing execution
  - token and duration tracking participate in real run behavior rather than report-only accounting
- Planned packet anchor: `P06-WS06-T05`

### P06-W06 — Phase 6 Validation, Critique, and Exit Gate
- Objective: prove Phase 6 truth against upstream run/session, persona, moderated-debate, and bounded-memory acceptance and close only if the result is real.
- Required outputs:
  - acceptance mapping for `AC-041` through `AC-059`
  - evidence for fresh-run behavior, workspace isolation, runtime persona generation, Moderator-limited normal rounds, bounded memory, and token or duration tracking
  - critique pass and in-scope remediation or escalation
- Exit proof:
  - run execution depends on fresh workspace isolation and current eligible Preflight lineage
  - persona generation is runtime-driven and includes exactly one Moderator
  - normal rounds remain 3-to-5-speaker moderated subsets with real bounded memory
  - Phase 6 does not overclaim Phase 7 monitoring fidelity, Phase 8 final reports, or Phase 9 cleanup closure
- Planned packet anchor: `P06-WS11-T01`

## Wave Dependency Rules
- W02 freezes the contracts and truth surfaces needed by W03 through W05.
- W03 establishes the shell-owned start path, fresh workspace isolation, and Preflight-lineage handoff needed before W04 or W05 can claim truthful active execution.
- W04 establishes runtime-generated personas, Moderator basis, and bounded memory foundation before W05 can claim truthful moderated rounds.
- W05 may not claim full live monitoring fidelity, final report truth, or cleanup closure; it establishes only bounded active run behavior.
- W06 cannot begin exit assessment until W03 through W05 are implemented and validated.
- No wave may claim Phase 6 completion while run execution bypasses current eligible Preflight truth, while normal rounds use all-agent fanout, or while memory still relies on unlimited transcript replay.

## Still Deferred
- full user-visible live monitoring fidelity and interruption handling
- final report generation, export, and final-report truth
- cleanup closure and end-to-end cleanup hardening
- updater/runtime-delivery behavior and release readiness
- public API or OpenAPI expansion

## Packetization Rule
- Future Phase 6 packets must be large enough to implement, validate, critique, and hand off coherently.
- Future Phase 6 packets must cite this plan together with their upstream Document A, B, and C rules.
- No packet may treat current eligible Preflight truth as optional at run start.
- No packet may treat transcript archive visibility as permission to use unbounded transcript replay as prompt context.
- No packet may collapse shell-owned start or supervision authority into the frontend or commercial backend.