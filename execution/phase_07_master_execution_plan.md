# Phase 7 Master Execution Plan

## Status
- Phase: P07
- Phase name: Run State, Events, Live Monitoring, and Interruption Handling
- Frozen by packet: `P07-WS07-T01`
- Date frozen: 2026-04-15
- Scope: phase-specific execution ordering, bounded live status/event/interruption basis, and future packetization guidance only

## Purpose
Phase 7 establishes truthful live run visibility and control.

## Governing Basis
- Budget/duration monitoring and control obligations remain governed by `AC-060` through `AC-069`.
- Active-run visibility, interruption/degradation, and cancellation obligations remain governed by `AC-070` through `AC-085`.
- Canonical run/session state vocabulary and allowed transitions remain governed by Document A Section `7.8`, including `STATE-134` through `STATE-186`.
- Controlled cancellation rules remain governed by `BEHAV-079` through `BEHAV-082`.
- WS-07 ownership remains governed by `BWORK-031` through `BWORK-034`.
- Phase 7 ordering, outcomes, non-goals, and exit intent remain governed by `BPHASE-038` through `BPHASE-042` and Document B Section `6.9.9` and `6.9.10`.
- Status/event contract shape and runtime state/event discipline remain governed by `CAPI-025` through `CAPI-030`, `CEVENT-014` through `CEVENT-056`, and the stable naming rules in `CTERM-047` through `CTERM-058`.

## Phase 7 Entry Basis Frozen by This Plan
- Phase 6 handed off shell-owned one-shot run execution with fresh run/session identity, fresh workspace allocation, Preflight-lineage capture, runtime personas, exactly one Moderator, bounded memory, transcript-archive separation, and run-local token/duration tracking without live monitoring fidelity.
- The current repo exposes only coarse shell-owned run-session classes `not_started`, `start_blocked`, `technical_failure`, `pending_execution`, `running`, and `finished`, but the one-shot shell path currently persists the stored run/session snapshot only after execution returns, leaving in-flight run/session identity and ordered-history availability unavailable during active execution.
- The engine already produces transcript-turn archives, round-summary continuity, speaker attribution, and token/duration artifacts inside per-run workspaces.
- The frontend currently renders bounded shell-owned run truth after execution completes but does not yet support incremental live monitoring, ordered event retrieval, or user-visible interruption/cancel controls.
- The repo still lacks canonical state exposure aligned to `RunDraft` through `CleanupFailed`, status retrieval surfaces, ordered event retrieval, paused/degraded truth, budget/duration visibility in active monitoring, cancellation request handling, and truthful active-run chronology.
- Phase 7 must expand from coarse execution truth to live truth without fabricating telemetry, without granting the frontend lifecycle authority, and without overclaiming Phase 8 final-report truth or Phase 9 cleanup hardening.

## Bounded Live Monitoring and Interruption Path

### Ownership Split
- The frontend may render shell-owned live status, incremental ordered events, transcript/event chronology, budget/duration visibility, and a cancel control only when the shell exposes those truths. It does not infer hidden lifecycle state, recover missing history, or decide cancelability on its own.
- The Rust/Tauri shell remains the authoritative local owner of run/session lifecycle truth, canonical state projection, status retrieval, ordered event mediation, cancellation acceptance, app-close run-control boundaries, and consistency between engine activity, monitoring surfaces, and terminal classifications.
- The Python engine remains the producer of underlying execution activity, transcript turns, bounded continuity mutation, runtime usage tracking, and any raw degradation or retry signals arising during run execution. It does not define external UI semantics or bypass shell-owned status/event mediation.
- The commercial backend remains out of the active-run monitoring path except for already-governed activation/runtime authorization responsibilities; it must not become a hosted live-session coordinator, transcript relay, or event history authority.
- Phase 7 may formalize visibility for `Finalizing`, `Cleaning`, `Cleaned`, and `CleanupFailed` as canonical lifecycle terms, but Phase 8 still owns final-report truth and Phase 9 still owns cleanup hardening and janitor behavior.

### Monitoring-Surface Split
| Surface class | Primary owner | Allowed contents | Explicit exclusions | Retrieval posture |
| --- | --- | --- | --- | --- |
| Live status snapshot | shell | canonical current run state, state label, run/session identity, workspace reference, round counts, budget/duration counters, degradation/cancelability flags, latest truthful status note, latest visible sequence marker | fabricated phase advancement, report availability, cleanup success, hidden retry work, inferred provider health beyond known evidence | single current snapshot refreshed on demand or via governed polling |
| Ordered run event history | shell plus engine | append-only ordered lifecycle, debate, degradation, cancellation, and later cleanup/finalization events with deterministic sequence references and bounded payload detail | secret material, raw provider credentials, invented events, ambiguous ordering, implicit missing-history assumptions | incremental retrieval by ordered cursor/sequence semantics |
| Transcript/event visibility feed | shell plus engine | speaker-attributed turn visibility, round boundaries, moderation progression, and user-followable chronology derived from real run artifacts | all-agent full-fanout implication, transcript-as-prompt authority, cross-run mixing, hidden reconstruction | consumer-facing chronological feed built from real event and transcript artifacts |
| Cancellation/control surface | shell | cancelability truth, cancellation request outcome, controlled-stop progression, app-close graceful-then-forceful shutdown truth where applicable | frontend-only cancel authority, hidden continuation after cancel, fake completion after cancel, resume semantics | command plus status/event reflection only when current state allows it |

- Ordered event history may use run-support metadata and workspace-local append-only artifacts under shell authority, but it must not require hosted orchestration or undeclared persistence beyond governed local boundaries.
- Transcript visibility remains structurally separate from bounded prompt memory; live monitoring may display transcript chronology without turning the transcript archive into unlimited prompt context.
- Status and event surfaces must remain mutually reconcilable; a consumer must not need to choose which one is "real".

### Canonical Run Lifecycle Exposure Basis
- Phase 7 freezes the canonical exposed run/session lifecycle vocabulary to the upstream state family: `RunDraft`, `RunQueued`, `Ingesting`, `GeneratingPersonas`, `RunningRound`, `PausedRateLimit`, `PausedNetwork`, `Finalizing`, `Completed`, `CompletedBudgetCapped`, `Canceled`, `FailedRetryable`, `FailedTerminal`, `Cleaning`, `Cleaned`, and `CleanupFailed`.
- The current Phase 6 coarse classes are treated as incomplete implementation shorthand only. Later Phase 7 packets must replace or map them into the canonical state family rather than exposing both vocabularies as competing truths.
- Phase 7 must preserve the distinction between degraded-but-nonterminal states, cancellation, constrained completion, terminal failure, and cleanup outcome. No later packet may collapse those into generic `running`, `done`, or `error` labels.
- Cancelability is derived from the canonical state family and upstream guarded transitions, not from frontend heuristics or silent shell conventions.
- `Finalizing` means controlled closing based on completed material; it does not imply report availability. `Cleaning` means cleanup has begun; it does not imply cleanup success.
- No Phase 7 packet may imply crash-resume, hidden continuation, or silent background progression after cancellation, shutdown, or cap-triggered finalization.

### Ordered Event and Retrieval Basis
- Phase 7 event transport must use a stable append-only envelope basis with at minimum `event_id`, `sequence_number`, `occurred_at`, `session_id`, `event_type`, `payload`, and `runtime_domain`.
- Event retrieval must support deterministic ordered synchronization, incremental retrieval boundaries, and gap detection strong enough for the UI to keep a truthful chronology.
- Event taxonomy must remain separated at minimum into lifecycle events, moderation/debate events, interruption/degradation events, finalization/output events, cleanup/janitor events, and runtime/update support events where technically relevant.
- Debate events may expose moderator-driven round progression, selected-speaker activity, and bounded argument/risk or summary progression where emitted, but they must not imply all-agent fanout or fabricate turns.
- Degradation and recovery events must be explicit. A return from `PausedRateLimit` or `PausedNetwork` to `RunningRound` may not be inferred implicitly from missing error text alone.
- Event contracts define retrieval shape only. Canonical state meaning continues to come from the governed state system and its guarded transitions.

## Bounded Phase 7 Implementation Basis
- Phase 7 must materialize shell-owned status retrieval that truthfully projects the canonical current run state from the real run/session, workspace, and engine evidence already available or newly materialized in-scope.
- Phase 7 must materialize ordered event retrieval and append-only sequence semantics without fabricating telemetry for work that did not happen.
- Phase 7 must materialize incremental live monitoring UI surfaces that show active run history and current state instead of a static black box when meaningful status/event truth exists.
- Phase 7 must materialize shell-owned in-flight run/session snapshot truth early enough for status, event-history, and live-monitoring surfaces to expose current run/session identity during active execution rather than only after one-shot execution completes.
- Phase 7 must surface budget and duration consumption as live monitoring truth grounded in Phase 6 tracking artifacts, including warnings or cap-driven control transitions where upstream rules require them.
- Phase 7 must materialize paused-rate-limit and paused-network truth, bounded retry-versus-terminal classification where known, and status/event semantics that preserve degraded-state honesty.
- Phase 7 must materialize controlled cancellation request handling and visible cancellation progression without allowing hidden post-cancel debate or mislabeled completion.
- Phase 7 may integrate graceful-then-forceful app-close handling for active runs only to the extent required to keep interruption and cancellation truth coherent with shell-owned lifecycle control.
- Phase 7 may expose `Finalizing` and cleanup-related state semantics where real shell truth exists, but it must not claim report availability, report correctness, cleanup hardening, or janitor completion as part of monitoring alone.
- Phase 7 does not authorize hosted run supervision, crash-resume, hidden state reconstruction, report generation behavior, cleanup hardening, updater work, or release readiness.

## Phase 7 Wave Plan

### P07-W01 — Run State, Events, Live Monitoring, and Interruption Handling Entry Basis and Master Execution Blueprint
- Status: COMPLETE via `P07-WS07-T01`.
- Outcome: the bounded Phase 7 live status, event ordering, degradation/cancellation, and later wave order are now frozen for implementation.

### P07-W02 — Canonical Run Lifecycle Exposure, Status Contract, and Event Envelope Foundation
- Objective: establish code-level canonical run-state exposure, cancellation and status retrieval contract surfaces, event envelope vocabulary, sequence semantics, and shell-owned monitoring authority without yet implementing live event capture or the full monitoring UI.
- Required outputs:
  - canonical run lifecycle state family and mapping basis from the existing Phase 6 coarse classes
  - status retrieval contract shape and shell-owned state projection basis
  - cancellation request/response contract basis with cancelability derived from governed states
  - ordered event envelope and retrieval contract basis
  - shell lifecycle, interface, and supervision surfaces updated to use the new vocabulary without overclaiming real interruption handling yet
- Exit proof:
  - contract surfaces align to governed lifecycle vocabulary and do not invent new states or transitions
  - event envelopes have deterministic ordering fields and do not redefine event meaning
  - no contract implies report availability, cleanup success, hidden continuation, or crash-resume
- Planned packet anchor: `P07-WS07-T02`

### P07-W03 — Shell Status Projection, Ordered Event History, and Monitoring Retrieval Path
- Objective: implement the shell-owned live status and ordered event retrieval path over the existing run/session execution artifacts, including incremental retrieval boundaries, transcript/event chronology, and live usage visibility, without yet claiming cancellation/degradation completeness or final-report truth.
- Required outputs:
  - real shell-owned status retrieval command surface
  - append-only ordered event history materialization with deterministic sequence numbers
  - transcript/event chronology path built from real lifecycle and debate artifacts
  - active budget/duration visibility derived from real tracking artifacts
  - truthful failure handling for unavailable or incomplete event history without fabricating continuity
- Exit proof:
  - status and events describe the same run honestly
  - chronological retrieval is deterministic enough for incremental UI sync
  - active monitoring truth is grounded in real run/session evidence rather than placeholders
- Planned packet anchor: `P07-WS07-T03`

### P07-W04 — Live Monitoring UI Surface, Phase Visibility, and Incremental Feed Rendering
- Objective: implement the user-visible live monitoring surface that consumes shell-owned status and event truth, exposes meaningful phase visibility, and renders transcript/event chronology incrementally without giving the frontend lifecycle authority.
- Required outputs:
  - active run-monitoring UI surface distinct from static configuration
  - current phase/state visibility for ingestion, persona generation, active debate, degraded/paused states, and later finalizing/cleaning states when real truth exists
  - incremental event/transcript feed rendering with user-followable chronology
  - visible budget/duration progress and warning affordances where known
  - frontend tests proving truthful rendering of state, chronology, and no-black-box behavior when events exist
- Exit proof:
  - the user sees an active run surface during execution
  - the frontend does not infer hidden states, missing history, or cancelability on its own
  - active debate is not reduced to an undifferentiated loading indicator when meaningful events exist
- Wave-specific blocker rule:
  - if the current shell execution path still withholds in-flight run/session identity and monitoring availability until after one-shot execution returns, W04 must remain blocked and a bounded shell prerequisite packet must execute before W04 can close honestly
- Planned packet anchor: `P07-WS07-T04`

### P07-W05 — In-Flight Run/Session Snapshot Publication and Monitoring Availability Basis
- Objective: expose shell-owned in-flight run/session snapshot truth early enough for live status, ordered event-history retrieval, and the already-implemented Wave 4 monitoring UI to reflect active execution honestly without introducing push streaming, degraded-state logic, or controlled cancellation behavior.
- Required outputs:
  - shell-owned nonterminal run/session snapshot publication before one-shot execution returns
  - current run/session identity, workspace reference, and bounded start-path progress visibility during active execution
  - truthful monitoring-availability basis for status and event-history retrieval while ordered history is still pending or partially unavailable
  - shell lifecycle, interface, startup, and contract truth-surface updates needed to reflect the new in-flight monitoring basis without inventing new event meaning
- Exit proof:
  - the shell exposes current run/session identity during active execution instead of only after completion
  - status and event-history surfaces remain mutually truthful when ordered history is not yet available
  - no hidden continuation, crash-resume, push-streaming claim, or fabricated chronology is introduced
- Planned packet anchor: `P07-WS07-T05`

### P07-W06 — Degradation, Retry, Controlled Cancellation, and Shutdown Integration
- Objective: implement paused-rate-limit and paused-network truth, bounded retry-versus-failure semantics, controlled cancellation handling, and shutdown-integrity wiring needed for truthful live monitoring and interruption behavior.
- Required outputs:
  - rate-limit and network degradation state/event integration
  - bounded retry logic visibility and retryable-versus-terminal distinction where known
  - cancel control exposure only in valid states plus real cancellation acceptance behavior
  - no-hidden-post-cancel progression enforcement with matching status/event truth
  - graceful-then-forceful app-close handling integrated with active-run lifecycle truth as governed
- Exit proof:
  - degraded states are not mislabeled as healthy progress
  - accepted cancellation stops normal round progression and is reflected truthfully
  - app-close handling preserves shell stability and lifecycle honesty without implying resume
- Planned packet anchor: `P07-WS07-T06`

### P07-W07 — Phase 7 Validation, Critique, and Exit Gate
- Objective: prove Phase 7 truth against upstream monitoring, interruption, cancellation, and live-visibility acceptance and close only if the result is real.
- Required outputs:
  - acceptance mapping for `AC-060` through `AC-085`
  - evidence for budget/duration visibility, phase/state visibility, deterministic chronology, degraded truth, retry/failure distinction, cancellation, and shutdown handling
  - critique pass and in-scope remediation or escalation
- Exit proof:
  - users can distinguish key run phases truthfully
  - event ordering is deterministic enough for incremental UI sync
  - status and events do not tell contradictory stories
  - Phase 7 does not overclaim Phase 8 report truth or Phase 9 cleanup hardening
- Planned packet anchor: `P07-WS11-T01`

## Wave Dependency Rules
- W02 freezes the canonical monitoring vocabulary and contract surfaces needed by W03 through W06.
- W03 establishes the real shell-owned status and ordered-event foundation before W04 can claim truthful live monitoring UI behavior.
- W04 establishes the read-only monitoring UI surface, but it cannot close honestly if the shell still withholds in-flight run/session truth needed for active execution visibility.
- W05 establishes the shell-owned in-flight monitoring availability basis before W06 adds interruption and cancellation control behavior.
- W06 may integrate `Finalizing` and cleanup-related visibility only where real shell truth exists, but it may not claim report correctness, report availability, cleanup success, or janitor behavior as complete.
- W07 cannot begin exit assessment until W03 through W06 are implemented and validated.
- No wave may collapse canonical run states into coarse generic labels, may treat degraded conditions as healthy progress, or may let cancellation or finalization tell a different story in status versus events.

## Still Deferred
- final report generation, report availability, export, and final-report truth
- cleanup hardening, startup janitor behavior, and cleanup outcome validation
- hosted orchestration or vendor-hosted live session supervision
- updater/runtime-delivery hardening and release readiness
- public API or OpenAPI expansion

## Packetization Rule
- Future Phase 7 packets must be large enough to implement, validate, critique, and hand off coherently.
- Future Phase 7 packets must cite this plan together with their upstream Document A, B, and C rules.
- No packet may invent a second run-state vocabulary beside the frozen canonical state family.
- No packet may treat missing or out-of-order events as acceptable live-monitoring truth.
- No packet may let the frontend become the authority for lifecycle state, event ordering, cancelability, or degraded/healthy classification.
- No packet may represent canceled, failed, constrained-completion, finalizing, cleaning, or cleanup-failed paths as if they were the same outcome.