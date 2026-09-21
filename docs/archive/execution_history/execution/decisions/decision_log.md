# Decision Log

## DEC-001
Date: 2026-04-14
Decision:
- The live governing document stack is:
  - `Document_A/document_a_master_project_creative_brief_v2a_final_draft_2.md`
  - `Document_B/document_b_master_execution_plan_b1_0_final_draft_2.md`
  - `Document_C/document_c_engineering_annex_pack_c1_0_final_draft_1.md`

Reason:
- These are the current authoritative project documents.

## DEC-002
Date: 2026-04-14
Decision:
- Bootstrap work is treated as Phase 1 work.
- No new canonical Phase 0 is created.

Reason:
- Document B already defines Phase 1 as governance, repository foundation, and execution scaffolding.

## DEC-003
Date: 2026-04-14
Decision:
- The repo execution operating system must exist before feature implementation begins.

Reason:
- AI-led implementation requires bounded packeting, traceability, and handoff discipline.

## DEC-004
Date: 2026-04-14
Decision:
- Empty structure folders may be created for future code boundaries, but no feature implementation is allowed in the first packet.

Reason:
- Phase 1 allows scaffolding and forbids product-feature implementation surfaces such as runtime, activation, Preflight, simulation, reporting, cleanup, and updater behavior.

## DEC-005
Date: 2026-04-14
Decision:
- The authoritative execution operating-system guide location is `execution/README.md`.
- Root `README.md` is not canonical for execution control.

Reason:
- The repository contains `execution/README.md` and no root `README.md`.
- Keeping execution authority inside the `execution/` subtree avoids split execution guidance.

## DEC-006
Date: 2026-04-14
Decision:
- The minimum Phase 2 backbone choices are frozen as:
  - desktop shell technology: Rust/Tauri
  - frontend framework baseline: React with TypeScript
  - engine runtime/language: Python
  - repo test runner baseline: `cargo test`, `vitest`, and `pytest`

Reason:
- Document A already fixes the shell/engine architecture around a Rust/Tauri shell and Python engine.
- Document B requires the intended shell technology and local execution model to be frozen enough before Phase 2 begins.
- No existing repo files or toolchain manifests conflict with these choices.

## DEC-007
Date: 2026-04-14
Decision:
- `app/index.html` is authorized as a packet-scoped exception for `P02-WS02-T01`.
- This exception exists only because a bootable React/Vite placeholder frontend requires an HTML host file.
- This exception does not expand product scope, feature scope, persistence scope, or architectural authority.

Reason:
- The active packet otherwise allowed only the frontend TypeScript and Vite files, which is insufficient for a truthful bootable Vite host page.
- The user explicitly approved this one-file exception and no broader file expansion.

## DEC-008
Date: 2026-04-15
Decision:
- `execution/README.md` remains the canonical execution SOP and is strengthened to require explicit packet lifecycle states, a packet registry, a mandatory critique pass, and enterprise zero-warning validation gates.
- Execution status must live inside the execution operating system files and must not be split into a parallel generic `project_status.md` or appended into Document B.
- Artificial micro-slice packets are disallowed; future packets must be coherent enough to implement, validate, critique, and hand off end-to-end.

Reason:
- The project relies heavily on AI-led development and needs deterministic context reconstruction from repository state.
- Duplicate status surfaces and overly small packets increase execution drift, stale state, and false completion risk.
- Zero-warning gating and a mandatory critique pass raise the quality bar before packet or phase closure.

## DEC-009
Date: 2026-04-15
Decision:
- Frontend dependency audit remediation is elevated into its own bounded packet before further implementation work proceeds.
- `P02-WS12-T03` is the active blocker packet for that remediation work.
- `P02-WS02-T04` is prepared as the next shell-owned implementation packet and must remain inactive until `P02-WS12-T03` is completed or explicitly re-sequenced.

Reason:
- The live frontend audit state still reports 5 moderate vulnerabilities in the Vite/Vitest chain.
- The currently available audit fixes point to semver-major upgrades, so the remediation work needs its own bounded review instead of being hidden inside unrelated implementation packets.
- The user explicitly directed that audit remediation occur before more implementation work.

## DEC-010
Date: 2026-04-15
Decision:
- The frontend audit remediation is accepted on the upgraded toolchain line of `vite` 8.0.8, `vitest` 4.1.4, and `@vitejs/plugin-react` 6.0.1.
- This acceptance is local to the validated T03 remediation outcome and does not authorize arbitrary future semver-major upgrades without the same bounded review and gate proof.

Reason:
- The available remediation path for the reported vulnerabilities required semver-major upgrades.
- `npm audit --json` returned zero vulnerabilities after the upgrade.
- `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` all remained clean after the upgrade.

## DEC-011
Date: 2026-04-15
Decision:
- Phase 2 closure is denied after the T07 exit-gate audit.
- The precise remaining-gap packet is `P02-WS10-T01`, which is assigned to establish the runtime package contract and shell verification basis needed for truthful runtime-ready decisions.

Reason:
- The repo can truthfully report runtime-not-ready, but it cannot yet truthfully support a positive runtime-ready path because `runtime_presence` remains not production-determined and `runtime_integrity` remains unaccepted.
- The current shell authority base is already structurally explicit enough that the remaining blocker is no longer a generic supervision/boundary packet.
- OQ-005 forbids implicitly choosing the full updater/runtime-delivery toolchain, so the runtime package contract and trust-verification basis must be packetized explicitly instead of being smuggled into a broader shell packet.

## DEC-012
Date: 2026-04-15
Decision:
- The shell-owned runtime package contract is frozen locally as a persistent installation artifact in an app-controlled support location, separate from the ephemeral run workspace.
- Shell-owned runtime presence and runtime integrity decisions must use that bounded contract and verification basis.
- This decision does not choose the updater/runtime-delivery toolchain and does not authorize runtime download, install, repair, or update behavior.

Reason:
- Document A runtime-package rules require persistence, an app-controlled support location, separation from the run workspace, and integrity-before-use semantics.
- Phase 2 needed a bounded basis so the repo could distinguish contract-defined runtime-ready from runtime-not-ready conditions honestly.
- OQ-005 remains open, so the local contract basis had to be frozen without silently deciding the later delivery/update mechanism.

## DEC-013
Date: 2026-04-15
Decision:
- The live repo-local execution OS is simplified to a lean model centered on `execution/progress.md` and `execution/live_execution_log.md`.
- `execution/current_phase.md`, `execution/handoffs/latest_handoff.md`, and `execution/evidence/latest_evidence.md` are retained only as legacy compatibility/archive surfaces and are no longer mandatory live-update files.
- Packet discipline, critique-pass discipline, and zero-warning gate discipline remain intact.

Reason:
- The prior mirrored execution-state routine was accurate enough but too heavy for sustained autonomous implementation.
- The user explicitly approved simplifying the repo-local SOP before Phase 3.
- A lean dashboard/log model preserves truthful execution control with less administrative overhead.

## DEC-014
Date: 2026-04-15
Decision:
- The first Phase 3 packet is a blueprint and entry-basis packet, not an activation implementation packet.
- That packet must first define the secure local activation-artifact path and the Phase 3 master execution blueprint before later coding packets are activated.

Reason:
- Document B Phase 3 entry preconditions require a defined path for secure local persistence of activation artifacts.
- The current repo truth still leaves that path too open for truthful activation-feature implementation.
- The user explicitly authorized a more elaborate Phase 3 execution plan if it is the safer entry move.

## DEC-015
Date: 2026-04-15
Decision:
- The bounded local activation-artifact path is frozen as a shell-owned split.
- Raw device-bound JWT material belongs only in OS-native secure storage.
- Bounded non-secret activation validation support data belongs only in the app-controlled local persistent support domain.
- Activation artifacts must not be mirrored into transient workspaces, logs, report/export artifacts, or ordinary config files.
- The final local support storage engine remains open, but it must preserve this storage-domain split.

Reason:
- Document A `DATA-012` through `DATA-016`, `AC-015` through `AC-018`, `API-087`, and `DB-040` through `DB-049` require offline post-activation validation, device binding, truthful activation completion, and approved local persistence without ordinary-workspace storage.
- Document C `CDATA-018` through `CDATA-025` and `CDATA-049` through `CDATA-059` require secret separation, prohibit plaintext fallback, and forbid hidden persistence drift.
- Document B `6.5.4` requires a defined path for secure local persistence of activation artifacts before truthful Phase 3 implementation can begin.

## DEC-016
Date: 2026-04-15
Decision:
- The minimum repo-local Wave 3 commercial backend surface is a bounded Python package under `commercial_backend/` backed by SQLite and deterministic SQL migrations.
- This surface owns governed license, payment-event, activation, audit, and token-lineage truth plus activation approval versus denial versus technical-failure decisions.
- This decision does not choose a hosted deployment topology or a real payment-provider ingress mechanism.

Reason:
- Wave 3 required a real durable ledger and activation-decision implementation, but Documents A/B/C do not require a larger cloud framework choice at this stage.
- A Python package with SQLite and deterministic migrations is the minimum surface that can truthfully implement the governed commercial records, hard two-device enforcement, same-device reuse, and token-lineage tracking without silently expanding scope.
- Leaving hosted deployment and real provider ingress unresolved preserves the bounded packet while keeping later integration choices open.

## DEC-017
Date: 2026-04-15
Decision:
- The repo-local Wave 4 local support-store engine is narrowed to a bounded JSON activation support-record file in the app-controlled local support domain.
- Raw device-bound token material remains only in OS-native secure storage under shell authority.
- Repo-local offline post-activation validation is bounded to committed-token fingerprint, signing-lineage, device-binding, expiration, and revoked-state checks against the secure token plus support record; this does not expand into broader offline product capability.

Reason:
- Wave 4 needed a real recoverable local support record to satisfy AC-015 through AC-018 and the Phase 3 storage split frozen in DEC-015.
- A bounded JSON support record is the minimum repo-local persistence surface that can carry license reference, activation reference, slot, device-binding metadata, signing lineage, validity marker, and last-validation status without storing raw token material.
- Keeping the raw token only in OS-native secure storage and limiting offline validation to the committed token plus bounded support metadata preserves the upstream storage-domain split and avoids unauthorized backend or provider-scope expansion.

## DEC-018
Date: 2026-04-15
Decision:
- Phase 3 closure is denied at the Wave 5 exit gate.
- Waves 3 and 4 are validated cleanly and provide real commercial ledger behavior plus real local secure activation commit and offline validation behavior.
- The repo still lacks a real app-to-cloud activation request path that connects the app or shell to backend approval, denial, and technical-failure truth, so Phase 3 cannot yet be declared complete.

Reason:
- The Wave 5 gate suite passed cleanly across shell, backend, and frontend surfaces, so the blocker is not a build, lint, or test failure.
- `commercial_backend/` implements real entitlement and activation decision logic, and the shell implements real local commit and offline validation, but `shell/src/main.rs` currently only exposes local approval commit rather than a real activation request round trip.
- `execution/current_repo_truth.md` still truthfully records full app-to-cloud activation request handling as not yet implemented, which prevents honest end-to-end proof for the remaining activation-flow acceptance target.

## DEC-019
Date: 2026-04-15
Decision:
- MVP app-shell updates are frozen to the native app-updater path.
- MVP Python runtime delivery is frozen to a separate shell-owned managed runtime-package hydration flow authorized by the commercial backend.
- Runtime artifacts must be verified before use, tracked through version/trust metadata, and must not be mutated during an active run.
- This decision does not choose hosted release infrastructure, signing automation, or distribution automation.

Reason:
- Document A now freezes the runtime/update split while the repo already has a bounded runtime package contract basis without a production delivery path.
- Separating app-shell updates from runtime hydration preserves lifecycle truth and prevents later Phase 10 hardening from silently redefining the earlier readiness foundation.
- Remaining release questions are narrower operational infrastructure questions rather than open MVP product truth.

## DEC-020
Date: 2026-04-15
Decision:
- The MVP seed-intake constants are frozen as:
  - supported seed-document types: PDF, DOCX, TXT, and Markdown
  - maximum file size: 10 MiB (10,485,760 bytes)
  - minimum usable extracted text: 1,000 normalized characters
  - non-English rejection threshold: English-confidence below 0.80
- The minimum document-validation rejection families are unsupported file type, file too large, text extraction failed, insufficient extractable text, and non-English seed document.

Reason:
- Phase 5 Wave 3 requires exact constants and rejection families to implement truthful document validation without guessing.
- Documents A and C plus the active Wave 3 packet are now aligned to these exact values.
- Freezing the rejection families keeps rejection truth deterministic while preserving technical failure as a distinct class.

## DEC-021
Date: 2026-04-15
Decision:
- The canonical repo-level validation baseline is now the applicable subset of:
  - shell: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`
  - frontend: `npm run lint`, `npm run typecheck`, `npm run test`, `npm run build`
  - commercial backend: Python compile validation for `commercial_backend/`, `ruff check commercial_backend`, `pytest commercial_backend`
- Documentation-only packets use structural consistency and contradiction review instead of unrelated code gates.

Reason:
- The repo already uses this broader gate set in clean Phase 3, Phase 4, and Phase 5 validation history.
- Treating the validation stack as unresolved would ignore already-governed execution truth and encourage under-validation or ad hoc gate choice.
- Applicable-subset language preserves pragmatism while keeping the baseline deterministic.

## DEC-022
Date: 2026-04-16
Decision:
- Phase 7 packetization is reopened after the T04 critique pass confirmed that the current one-shot shell path publishes the stored run/session snapshot only after execution returns.
- `P07-WS07-T04` must be treated as BLOCKED after its frontend implementation and clean frontend gates because honest Wave 4 closeout still lacks shell-owned in-flight run/session snapshot truth.
- A new bounded shell prerequisite packet is inserted before degradation/cancellation work as `P07-WS07-T05` to materialize in-flight run/session snapshot publication and monitoring-availability basis.
- The previously planned degradation/cancellation packet shifts from Phase 7 Wave 5 to Phase 7 Wave 6 and its planned packet anchor becomes `P07-WS07-T06`.

Reason:
- `AC-070` through `AC-074` and Document B Section `6.9.10` do not allow Phase 7 to claim live monitoring truth beyond what the shell actually knows during active execution.
- The Wave 4 monitoring UI is now implemented and validated, so the remaining contradiction is a bounded shell-availability problem rather than a frontend rendering problem.
- Inserting a dedicated prerequisite packet keeps the remediation smaller and safer than overloading the later degradation/cancellation packet with unrelated shell-availability work.
