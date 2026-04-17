# Current Constraints

## Authority Constraints
- Document A governs product truth.
- Document B governs execution order.
- Document C governs technical-annex detail.
- Task packets govern bounded implementation work.
- Repository convenience does not override any of the above.

## Locked Product Constraints
- Windows and macOS only
- No Linux support
- English-only seed document support
- Supported seed-document types limited to PDF, DOCX, TXT, and Markdown
- Maximum seed-document size is 10 MiB (10,485,760 bytes)
- Minimum usable extracted seed text is 1,000 normalized characters
- Non-English seed rejection threshold is English-confidence below 0.80
- Non-English seed documents must be rejected in Preflight
- App-shell updates use the native app-updater path
- Python runtime delivery uses a separate shell-owned managed runtime-package hydration flow
- No crash-resume for interrupted runs
- BYOK provider model
- Two-device license limit
- Max token budget required before a run begins
- Hard two-hour duration cap
- Local-data-first, cloud-inference-dependent desktop software
- Best-effort ephemeral local workspace handling
- Reports are preserved differently from transient run artifacts
- No hidden vendor-hosted simulation archive
- No absolute deletion/security claims

## Technical Guardrail Constraints
- No hidden persistence
- No undeclared endpoints
- No hidden debug/admin backdoors
- No layer-boundary collapse
- No misleading state naming
- No fabricated completion or output truth
- No speculative extensibility unless explicitly authorized
- No hidden telemetry/analytics storage unless explicitly authorized

## Execution Hygiene Constraints
- No packet may be marked COMPLETE while any applicable build, lint, type-check, or test gate still reports an error.
- No packet may be marked COMPLETE while any applicable build, lint, type-check, or test gate still reports a warning.
- Missing dependencies, missing scripts, missing config, or missing tooling are blocking conditions, not implied passes.
- Every packet requires a critique pass after implementation and before closeout.
- Material critique findings must be fixed in-scope or escalated explicitly before packet closure.
- Artificial micro-slice packets are forbidden; each packet must be large enough to implement, validate, critique, and hand off coherently.
- Execution status belongs inside the lean execution operating system surfaces, especially `execution/progress.md` and `execution/live_execution_log.md`, not in a parallel project-status document and not appended into Document B.

## Current Execution Posture
- No Phase 11 packet is active right now; `execution/task_packets/p11_ws11_t06_execute_phase_11_windows_only_scope_realignment_release_record_refresh_and_final_verdict_revalidation.md` is COMPLETE as the truthful post-close Phase 11 release-refresh and closure record.
- `execution/phase_11_master_execution_plan.md` now exists and is the authoritative Phase 11 packetization basis.
- `execution/phase_11_validation/` now exists as the bounded repo-local Phase 11 validation artifact pack.
- Phase 11 is now CLOSED after truthful `P11-WS11-T06` closeout with a governed `release-ready` verdict.
- No Phase 10 packet is active right now; `execution/task_packets/p10_ws11_t01_execute_phase_10_validation_critique_and_exit_gate.md` is COMPLETE as the truthful Phase 10 Wave 5 validation, critique, and closure record.
- `execution/task_packets/p10_ws10_t04_implement_verified_runtime_delivery_update_apply_gating_and_artifact_rejection_hardening.md` is COMPLETE as the truthful Phase 10 Wave 4 verified runtime-delivery, update-apply-gating, and artifact-rejection-hardening packet.
- `execution/task_packets/p10_ws10_t03_implement_native_app_updater_discovery_availability_truth_and_safe_coordination.md` is COMPLETE as the truthful Phase 10 Wave 3 native app-updater discovery, availability truth, and safe-coordination packet.
- `execution/task_packets/p10_ws10_t02_establish_updater_lifecycle_runtime_delivery_contract_and_trust_surface_foundation.md` is COMPLETE as the truthful Phase 10 Wave 2 contract-foundation packet.
- `execution/task_packets/p10_ws10_t01_establish_phase_10_updater_runtime_delivery_and_distribution_hardening_entry_basis_and_master_execution_blueprint.md` is COMPLETE as the truthful Phase 10 entry-basis and master-blueprint packet.
- `execution/task_packets/p09_ws11_t01_execute_phase_9_validation_critique_and_exit_gate.md` is COMPLETE as the truthful Phase 9 exit-gate and closure record.
- `execution/task_packets/p09_ws09_t04_implement_app_close_shutdown_integration_abnormal_termination_residual_detection_and_startup_janitor_execution.md` is COMPLETE as the truthful Phase 9 Wave 4 implementation record.
- `execution/task_packets/p09_ws09_t03_implement_terminal_cleanup_triggering_outcome_classification_and_report_preservation_enforcement.md` is COMPLETE as the most recently completed Phase 9 Wave 3 packet.
- `execution/task_packets/p09_ws09_t02_establish_cleanup_outcome_scope_mapping_and_janitor_truth_surface_foundation.md` remains COMPLETE as the truthful Phase 9 Wave 2 contract-foundation record.
- `execution/task_packets/p09_ws09_t01_establish_phase_9_cleanup_shutdown_and_startup_janitor_hardening_entry_basis_and_master_execution_blueprint.md` remains COMPLETE as the truthful Phase 9 entry-basis freeze.
- `execution/task_packets/p08_ws11_t01_execute_phase_8_validation_critique_and_exit_gate.md` remains COMPLETE as the truthful Phase 8 closure record.
- `execution/task_packets/p08_ws08_t04_implement_pdf_generation_report_availability_and_local_access_integration.md` is COMPLETE as the finished Phase 8 Wave 4 implementation packet.
- `execution/task_packets/p08_ws08_t03_implement_finalization_viability_score_and_report_data_assembly.md` remains COMPLETE as the preceding Phase 8 Wave 3 packet.
- `execution/task_packets/p08_ws08_t02_establish_finalization_and_report_output_contracts_report_boundary_and_availability_truth_surfaces.md` remains COMPLETE as the preceding Phase 8 Wave 2 packet.
- `execution/task_packets/p08_ws08_t01_establish_phase_8_finalization_report_generation_and_report_access_entry_basis_and_master_execution_blueprint.md` remains COMPLETE as the Phase 8 entry-basis packet.
- `execution/task_packets/p08_ws08_t01_establish_phase_8_finalization_report_generation_and_report_access_entry_basis_and_master_execution_blueprint.md`, `execution/task_packets/p07_ws11_t02_implement_runtime_budget_duration_cap_enforcement_and_truthful_capped_finalization.md`, and `execution/task_packets/p07_ws11_t03_execute_phase_7_revalidation_and_closure_decision.md` are COMPLETE.
- `execution/task_packets/p07_ws07_t04_implement_live_monitoring_ui_surface_phase_visibility_and_incremental_feed_rendering.md` is COMPLETE because its prior frontend delivery and gate evidence are now backed by the completed T05 shell prerequisite.
- `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md` is COMPLETE.
- `execution/task_packets/p07_ws07_t06_implement_degradation_retry_controlled_cancellation_and_shutdown_integration.md` is COMPLETE.
- `execution/task_packets/p07_ws11_t01_execute_phase_7_validation_critique_and_exit_gate.md` is COMPLETE.
- `execution/phase_07_master_execution_plan.md` now exists and is the authoritative Phase 7 packetization basis.
- `execution/phase_08_master_execution_plan.md` now exists and is the authoritative Phase 8 packetization basis.
- `execution/phase_09_master_execution_plan.md` now exists and is the authoritative Phase 9 packetization basis.
- `execution/phase_10_master_execution_plan.md` now exists and is the authoritative Phase 10 packetization basis.
- Phase 7 is now CLOSED after the recovered exit-gate pass proved runtime budget-hit/duration-cap enforcement, truthful constrained completion, shell-owned finalizing/capped projection, and consistent monitoring/interruption truth.
- Phase 8 is now CLOSED after `P08-WS08-T04` and `P08-WS11-T01` proved real PDF generation, shell-owned report availability and local access truth, durable report preservation, and truthful exit-gate evidence.
- Phase 9 is now CLOSED after `P09-WS09-T03`, `P09-WS09-T04`, and `P09-WS11-T01` proved cleanup triggering, cleanup-failure visibility, startup janitor execution, preserved-report survival, and no-resume truth.
- Phase 10 is now CLOSED after truthful `P10-WS11-T01` completion.
- No later Phase 11 packet is active because Phase 11 is closed.
- The live execution surfaces are `execution/progress.md` and `execution/live_execution_log.md`.
- `execution/current_phase.md`, `execution/handoffs/latest_handoff.md`, and `execution/evidence/latest_evidence.md` are legacy compatibility/archive surfaces only.

## Phase 11 Entry Basis
- All prior required phases are now CLOSED honestly through `P10-WS11-T01`, so runtime readiness, activation truth, provider readiness, Preflight gating, moderated execution, live state truth, report truth, cleanup and janitor truth, and updater/runtime-delivery truth are all available as real upstream evidence-bearing surfaces.
- The governing documents already provide the usable-form acceptance and red-team matrix basis required by Document B Section `6.13.4`: Document A exposes stable acceptance IDs and risk classes, Document B freezes validation domains and the release gate, and Document C Annex Family E defines deterministic QA, red-team, regression, and release-aggregation structures.
- The current repo remains feature-complete only to the extent already proven by closed Phases 1 through 10. Phase 11 is now closed with populated acceptance, QA/regression, and red-team evidence, historical blocker `BL-001` resolved through T06 scope realignment, and a current `release-ready` verdict.
- `execution/phase_11_master_execution_plan.md` now freezes the bounded Phase 11 wave order for matrix instantiation, evidence execution, priority red-team review, blocker classification, and final release-gate verdict formation.
- No Phase 11 packet authorizes new product features, acceptance drift, blocker concealment, hosted release-operations automation, or an independent technical release authority outside Document B.

## Current Packetization Boundary
Allowed now:
- execution-state maintenance for the completed `P11-WS11-T06` packet and the truthful closed Phase 11 release-ready record
- truthful use of `execution/phase_11_master_execution_plan.md` as the only authorized basis for later Phase 11 packetization
- truthful use of the completed `P11-WS11-T01` packet as the frozen Phase 11 entry-basis record, including its explicit interpretation of the usable-form matrix basis already present in Documents A, B, and C
- truthful use of the completed `P11-WS11-T02` packet as the authoritative Phase 11 validation-artifact foundation record
- truthful use of the completed `P11-WS11-T03` packet as the authoritative Phase 11 acceptance/regression evidence record
- truthful use of the completed `P11-WS11-T04` packet as the authoritative Phase 11 red-team review and blocker-triage record
- truthful use of the completed `P11-WS11-T05` packet as the historical interim blocker-accounting and `release-blocked` record under the earlier Windows-and-macOS contract
- truthful use of the completed `P11-WS11-T06` packet as the authoritative current Phase 11 release-gate, blocker-resolution, and closure record
- truthful use of the instantiated repo-local Phase 11 artifact pack under `execution/phase_11_validation/` as the bounded structure and evidence basis for final aggregation
- execution-state maintenance for the completed `P10-WS11-T01` packet, the completed `P10-WS10-T04` packet, and the closed Phase 10 record
- truthful use of the completed `P10-WS10-T04` packet as the established Phase 10 Wave 4 basis for verified runtime delivery, updater preparation, explicit updater apply plus restart-request execution, artifact rejection hardening, and active-run mutation blocking
- truthful use of the completed `P10-WS11-T01` packet as the established Phase 10 acceptance-mapping, critique, and closure evidence record
- truthful use of `execution/phase_10_master_execution_plan.md` as the closed authoritative Phase 10 packetization and scope record
- truthful use of the completed `P10-WS10-T02` packet as the established Phase 10 Wave 2 basis for updater lifecycle vocabulary, runtime-delivery contract meaning, verification-before-apply or use semantics, shell-owned trust surfaces, and bounded frontend diagnostics rendering
- truthful use of the completed `P10-WS10-T01` packet as the frozen Phase 10 entry-basis handoff into Wave 2 contract-foundation work
- truthful use of the completed `P02-WS10-T01` packet as the bounded runtime package contract and verification-basis handoff into Phase 10
- truthful use of the completed `P09-WS11-T01` packet as the established Phase 9 exit-gate evidence and closure basis that Phase 10 must preserve
- execution-state maintenance for the completed Phase 8 packets, the completed `P09-WS09-T01` entry-basis packet, and the completed `P09-WS09-T02` packet as execution evidence
- truthful use of the completed `P08-WS08-T02` shell/frontend finalization, report-output, report-boundary, and report-availability/access contract surfaces as completed repo capabilities
- truthful use of the completed `P08-WS08-T03` engine-owned finalization, viability-score generation, required report-section assembly, transcript appendix assembly, cost/token summary assembly, shell report-generation success/failure classification integration, and bounded frontend rendering as completed repo capabilities
- truthful use of the completed `P08-WS08-T04` real PDF generation, durable local report output, shell-owned report availability and local access behavior, and bounded frontend report-access rendering as completed repo capabilities
- truthful use of `execution/phase_09_master_execution_plan.md` as the only authorized basis for later Phase 9 packetization
- truthful use of the completed `P09-WS09-T02` cleanup/janitor contract-foundation, cleanup-scope mapping, preserved-report boundary alignment, and bounded Reports-screen rendering as completed repo capabilities
- truthful use of the completed `P09-WS09-T03` packet as the established Phase 9 Wave 3 implementation basis for real terminal cleanup triggering, cleanup-result classification, preserved-report enforcement, and cleanup-aware lifecycle/status/event truth
- truthful use of the completed `P09-WS09-T04` packet as the established Phase 9 Wave 4 basis for shell-owned app-close cleanup coordination, abnormal-termination residual detection, startup janitor execution, preserved-report survival across janitor behavior, and no-resume-preserving janitor truth
- truthful use of the closed Phase 9 surfaces as completed repo capabilities without reopening cleanup or janitor product-code scope
- execution-state maintenance for the completed Phase 7 closure record and truthful use of the closed Phase 7 surfaces as completed repo capabilities
- truthful use of the completed Phase 2 controlled-shutdown, shell lifecycle, local-interface, trust-boundary, and engine-supervision scaffolds as completed repo capabilities
- truthful use of the closed Phase 3 activation and commercial-backend surfaces as completed repo capabilities
- truthful use of the closed Phase 4 provider secure-store, provider support-state, submission, retrieval-mediation, provider-readiness, lifecycle-gating, and provider-configuration UI surfaces as completed repo capabilities
- truthful use of the completed Phase 5 Wave 2 Preflight contract, invalidation, lifecycle, startup, interface, trust-boundary, and run-start-eligibility truth surfaces as completed repo capabilities
- truthful use of the completed Phase 5 Wave 3 single-seed validation, deterministic-rejection, and shell-owned submission-state surfaces as completed repo capabilities
- truthful use of the completed Phase 5 Wave 4 provider/model planning-context, effective-plan derivation, downgrade-acceptance, stale-invalidation, and run-start-gating surfaces as completed repo capabilities
- truthful use of the completed Phase 5 Wave 5 acceptance mapping, critique, and closure decision as completed execution evidence
- truthful use of the frozen Phase 5 execution blueprint as the only authorized basis for seed-intake and Preflight meaning
- truthful use of `execution/phase_06_master_execution_plan.md` as the only authorized basis for later Phase 6 packetization
- truthful use of `execution/phase_07_master_execution_plan.md` as the only authorized basis for later Phase 7 packetization
- truthful use of `execution/phase_08_master_execution_plan.md` as the only authorized basis for later Phase 8 packetization
- truthful use of the completed Phase 6 Wave 2 run/session-contract, workspace-boundary, start-lineage, run-start-readiness, lifecycle, startup, trust-boundary, engine-supervision, local-interface, and command-note surfaces as completed repo capabilities
- truthful use of the completed Phase 6 Wave 3 fresh run/session creation, workspace-allocation, run-session state, and execution-seed staging surfaces as completed repo capabilities
- truthful use of the completed Phase 6 Wave 4 runtime persona contract, Moderator basis, bounded-memory foundation, transcript-archive separation, engine bootstrap, and truthful shell/frontend status surfaces as completed repo capabilities
- truthful use of the completed Phase 6 Wave 5 moderated-round execution, 3-to-5-speaker selection, speaker-attributed turn capture, bounded continuity mutation, and run-local token/duration tracking surfaces as completed repo capabilities
- truthful use of the completed Phase 6 Wave 6 acceptance mapping, validation evidence, critique result, and closure decision as completed execution evidence
- truthful use of the completed `P07-WS07-T02` packet as the established basis for canonical lifecycle exposure, status retrieval contract shape, cancellation request/response semantics, and ordered event-envelope foundation
- truthful use of the completed `P07-WS07-T03` packet as the established basis for shell-owned status projection, ordered event history retrieval, transcript/event chronology, deterministic latest-sequence semantics, and live budget/duration visibility
- truthful use of the completed `P07-WS07-T04` packet as the implemented frontend monitoring UI and chronology-consumption basis
- truthful use of the completed `P07-WS07-T05` packet as the shell-owned in-flight snapshot publication and monitoring-availability basis
- truthful use of the completed `P07-WS07-T06` packet as the established basis for paused/degraded truth, bounded retry behavior, controlled cancellation execution, and active-run shutdown integration

Still not allowed without explicit later-packet activation:
- any new Phase 10 product-code work now that Phase 10 is closed
- any product-code or feature-expansion work under Phase 11 validation language without a later explicitly activated packet that authorizes it
- treating the completed `P11-WS11-T05` historical `release-blocked` verdict as the current Phase 11 release posture after `P11-WS11-T06`
- any claim that contradicts the completed `P11-WS11-T06` `release-ready` verdict or silently reopens `BL-001` without new governed evidence
- any update or runtime-delivery behavior beyond the completed Wave 4 repo truth or without the shell-owned trust checks it requires
- any product-code work outside a later explicitly activated post-Phase-10 packet
- any additional commercial-backend expansion beyond the bounded activation-decision and runtime-manifest-authorization command interfaces now already implemented on top of the Wave 3 ledger surface
- reopening or extending closed Phase 3 implementation scope without an explicitly activated packet
- reopening or extending closed Phase 4 implementation scope without an explicitly activated packet
- reopening or extending closed Phase 5 implementation scope without an explicitly activated packet
- reopening or extending closed Phase 6 implementation scope without an explicitly activated packet
- reopening or extending closed Phase 7 implementation scope without an explicitly activated packet
- reopening or extending the completed Phase 8 entry-basis packet beyond execution-planning scope without an explicitly activated later packet
- reopening or extending closed Phase 8 product-code scope without an explicitly activated later packet
- reopening or extending the completed Phase 9 entry-basis packet beyond execution-planning scope without an explicitly activated later packet
- reopening or extending the completed `P09-WS09-T02` packet beyond its closed contract-foundation scope without an explicitly activated later packet
- reopening or extending the completed `P09-WS09-T03` packet beyond its closed Wave 3 scope without an explicitly activated later packet
- reopening or extending the completed `P09-WS11-T01` packet beyond its closed exit-gate scope without an explicitly activated later packet
- reopening or extending the completed `P10-WS10-T01` packet beyond its closed entry-basis scope without an explicitly activated later packet
- any report catalog persistence, metadata-index persistence, hosted report library behavior, or alternate export format work until a later packet explicitly activates that scope
- crash-resume, hidden continuity, or janitor-as-recovery behavior
- forensic deletion guarantees or stronger privacy/deletion claims than upstream documents allow
- static user-authored persona configuration or naive full-transcript replay as the active memory model
- changing the locked seed-document constants or runtime/update split without an upstream document revision
- release behavior implementation without a later explicitly activated post-Phase-10 packet
- updater/distribution implementation beyond the completed Wave 4 and exit-gate-proven Phase 10 repo truth before later upstream planning explicitly authorizes it
- OpenAPI or public API implementation
- additional real product-feature UI implementation beyond the bounded activation, provider-configuration, Preflight, Wave 3 run-start truth surfaces, Wave 4 foundation truth surfaces, completed Wave 5 status rendering, completed Phase 7 Wave 2 contract/boundary rendering, and the completed bounded Phase 7 Wave 4 through Wave 6 monitoring/interruption surface unless a later explicitly activated packet authorizes it

## Escalate Immediately If
- a request implies a new feature
- a request implies new persistence not already governed
- a request implies a new state/event/field/output
- a request requires overriding Documents A/B/C
- any governing document, decision-log entry, or active packet text contradicts the locked seed-document constants or runtime/update split
- a request requires guessing unresolved technical truth
- a packet cannot satisfy the applicable build, lint, type-check, or test gates truthfully
- the requested packet is so narrow that it cannot support coherent validation and critique
- a Phase 11 planning or validation step would redefine acceptance, invent an untraceable matrix family, hide blocker severity, fabricate findings, or create an independent release authority outside Document B