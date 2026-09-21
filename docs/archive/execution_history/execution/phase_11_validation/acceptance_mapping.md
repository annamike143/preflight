# Phase 11 Acceptance Mapping

## Purpose
This file is the repo-local Phase 11 acceptance working map for the final release gate.
It now records W03/W04 evidence plus the W05 final governed synthesis and the T06 governed scope-realignment refresh for the release gate.

## Guardrails
- Document A Section 14 remains the owner of acceptance meaning.
- Document B Section 8.13 remains the owner of acceptance-validation rules.
- This file must not soften, merge away, or reinterpret upstream acceptance criteria.
- Evidence references must point only to real packet records, validation artifacts, gate commands, or controlled manual checks.
- W03 may populate evidence and statuses, W05 may choose the initial final release verdict, and T06 may refresh that release record only after a governed upstream scope change.

## Evidence Status Legend
| Status | Meaning |
| --- | --- |
| EVIDENCE_PENDING | The row exists, but release-gate evidence has not been attached yet. |
| AGGREGATING_CLOSED_PHASE_EVIDENCE | Earlier packet or phase evidence is being traced into the row. |
| AGGREGATING_PHASE_11_EVIDENCE | Phase 11 execution evidence is being attached to the row. |
| BLOCKING_GAP | The row currently lacks sufficient evidence or is known unmet. |
| RESIDUAL_LIMITATION_CANDIDATE | The row may be satisfied only with an explicitly documented minor limitation. |
| PROVEN_FOR_RELEASE_GATE | The row has complete traceable evidence for final release review. |

## W03 Evidence Bundles
| Bundle ID | Evidence basis |
| --- | --- |
| EV-001 | Fresh W03 shell regression rerun on 2026-04-17: `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` all passed in `shell/` with 144 tests green. |
| EV-002 | Fresh W03 frontend regression rerun on 2026-04-17: `npm run lint`, `npm run typecheck`, `npm run test`, and `npm run build` all passed in `app/` with 6 Vitest files and 16 tests green. |
| EV-003 | Fresh W03 engine regression rerun on 2026-04-17: `python -m compileall engine/src engine/tests`, `python -m ruff check engine`, and `python -m pytest engine/tests` all passed with 17 tests green. |
| EV-004 | Fresh W03 commercial-backend regression rerun on 2026-04-17: `python -m compileall commercial_backend/src commercial_backend/tests`, `python -m ruff check commercial_backend`, and `python -m pytest commercial_backend/tests` all passed with 11 tests green. |
| EV-005 | Phase 3 closure evidence in `execution/task_packets/p03_ws11_t03_execute_phase_3_revalidation_and_closure_decision.md` proves `AC-011` through `AC-018` and `AC-121` through `AC-126`. |
| EV-006 | Phase 4 closure evidence in `execution/task_packets/p04_ws11_t01_execute_phase_4_validation_critique_and_exit_gate.md` proves `AC-025` through `AC-030`. |
| EV-007 | Phase 5 closure evidence in `execution/task_packets/p05_ws11_t01_execute_phase_5_validation_critique_and_exit_gate.md` proves `AC-031` through `AC-040`. |
| EV-008 | Phase 6 closure evidence in `execution/task_packets/p06_ws11_t01_execute_phase_6_validation_critique_and_exit_gate.md` proves `AC-041` through `AC-059`. |
| EV-009 | Phase 7 closure evidence in `execution/task_packets/p07_ws11_t03_execute_phase_7_revalidation_and_closure_decision.md` proves `AC-060` through `AC-085`. |
| EV-010 | Phase 8 closure evidence in `execution/task_packets/p08_ws11_t01_execute_phase_8_validation_critique_and_exit_gate.md` proves `AC-086` through `AC-091`, `AC-093`, and `AC-097`. |
| EV-011 | Phase 9 closure evidence in `execution/task_packets/p09_ws11_t01_execute_phase_9_validation_critique_and_exit_gate.md` proves `AC-092` through `AC-098`. |
| EV-012 | Phase 10 closure evidence in `execution/task_packets/p10_ws11_t01_execute_phase_10_validation_critique_and_exit_gate.md` proves `AC-101`, `AC-102`, and materially touched `AC-105` through `AC-108`, and it reaffirms runtime/update trust evidence for `AC-019` through `AC-024`. |
| EV-013 | W03 cross-phase manual review plus the populated `execution/phase_11_validation/qa_regression_matrix.md` aggregate runtime, security/privacy, UI/UX, logging/diagnostics, interface/state, and acceptance-summary evidence that spans multiple already-closed phases without selecting the final Phase 11 verdict. |
| EV-014 | W04 priority red-team review populated `execution/phase_11_validation/red_team_matrix.md` across RT-001 through RT-012 and recorded no blocker, rework, or residual entry from the current live evidence set. |
| EV-015 | W05 final governed aggregation reviewed the remaining platform, UI/UX, diagnostics, and summary rows against live shell/frontend/engine/backend source, tests, blocker accounting, and release-gate rules, then recorded `AC-006` as release-blocking under the then-current Windows-and-macOS contract because macOS support proof was still insufficient. |
| EV-016 | T06 governed scope-realignment refresh reviewed the now-Windows-only Document A platform contract against the existing W03/W04/W05 evidence set, resolved `BL-001`, and refreshed the final release record to `release-ready` without product-code changes or new gate reruns. |

## 14.2 Acceptance-Criteria Philosophy
Evidence anchor: EV-013 cross-phase release-criteria aggregation plus EV-015 historical W05 synthesis and EV-016 T06 scope refresh.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-001 | Verification Rule | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | The final release map ties every release-relevant row to traceable proof, preserving the historical T05 blocker while leaving no active blocker row unresolved after T06. |
| AC-002 | No New Behavior Rule | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | W05 and T06 aggregated only already-materialized repo behavior, closed-phase evidence, and governed scope or release accounting updates without inventing new acceptance behavior. |
| AC-003 | One-Truth Rule | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | The acceptance map, QA matrix, blocker log, and release assessment now tell one coherent story: T05 remains the historical blocked record, and T06 is the current `release-ready` record after governed Windows-only scope realignment. |
| AC-004 | Testability Rule | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | Every release-relevant criterion is now tied to fresh reruns, closed-phase packet evidence, targeted W05 review, or the governed T06 scope-refresh record. |
| AC-005 | Stable Identifier Rule | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | W05 and T06 preserved the upstream `AC-*` identifiers one-to-one and refreshed the release map without renumbering or merging criteria. |

## 14.3 Constitutional and Platform Acceptance Criteria
Evidence anchor: EV-013 cross-phase platform and framing review plus EV-015 historical W05 synthesis and EV-016 T06 scope refresh.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-006 | Windows-Only Scope | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | `Document_A/document_a_master_project_creative_brief_v2a_final_draft_2.md` now governs a Windows-only MVP scope, and the existing Phase 11 evidence already proves explicit Windows scope, desktop-native Tauri architecture, and no contradictory user-facing multi-platform support promise. |
| AC-007 | No Non-Windows Support Claim | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | T06 confirmed that the governed scope now excludes macOS, Linux, and other non-Windows platforms; W05 had already found no Linux support claim in the live frontend, execution artifacts, or release surfaces, and generic Unix code paths do not become support promises. |
| AC-008 | Desktop-Native Product Form | EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | `shell/tauri.conf.json`, `shell/Cargo.toml`, and the shell-owned Tauri command surface prove the MVP remains a desktop-native application rather than a web or hosted product. |
| AC-009 | Local-Data-First Framing Consistency | EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | W05 confirmed that secure local storage, app-controlled support data, shell-owned runtime hydration, and bounded UI copy preserve the local-data-first, cloud-inference-dependent framing. |
| AC-010 | No Absolute Security Claim | EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | The live repo surfaces bounded secure-store and trust behavior without claiming absolute security, absolute deletion, or infallible protection. |

## 14.4 Licensing and Activation Acceptance Criteria
Evidence anchor: EV-005 Phase 3 closure evidence plus fresh W03 shell/backend reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-011 | Commercial Entitlement Requirement | EV-005 + EV-001 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 closure mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-012 | Device-Bound Activation | EV-005 + EV-001 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 closure mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-013 | Two-Device Limit | EV-005 + EV-001 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 closure mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-014 | Activation Denial on Slot Exhaustion | EV-005 + EV-001 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 closure mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-015 | Offline Post-Activation Validation | EV-005 + EV-001 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 closure mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-016 | Device Mismatch Rejection | EV-005 + EV-001 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 closure mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-017 | Activation Outcome Truthfulness | EV-005 + EV-001 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 closure mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-018 | Activation Token Local Persistence | EV-005 + EV-001 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 closure mapping reaffirmed by clean W03 shell/backend reruns. |

## 14.5 Runtime Hydration and Runtime Integrity Acceptance Criteria
Evidence anchor: EV-012 Phase 10 closure evidence plus fresh W03 shell reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-019 | Runtime Hydration Requirement | EV-012 + EV-001 | PROVEN_FOR_RELEASE_GATE | Phase 10 exit-gate mapping plus fresh W03 shell rerun cover runtime hydration and integrity truth. |
| AC-020 | Runtime Hydration Progress Visibility | EV-012 + EV-001 | PROVEN_FOR_RELEASE_GATE | Phase 10 exit-gate mapping plus fresh W03 shell rerun cover runtime hydration and integrity truth. |
| AC-021 | Runtime Resume-on-Failure Support | EV-012 + EV-001 | PROVEN_FOR_RELEASE_GATE | Phase 10 exit-gate mapping plus fresh W03 shell rerun cover runtime hydration and integrity truth. |
| AC-022 | Runtime Integrity Verification | EV-012 + EV-001 | PROVEN_FOR_RELEASE_GATE | Phase 10 exit-gate mapping plus fresh W03 shell rerun cover runtime hydration and integrity truth. |
| AC-023 | Runtime Integrity Failure Block | EV-012 + EV-001 | PROVEN_FOR_RELEASE_GATE | Phase 10 exit-gate mapping plus fresh W03 shell rerun cover runtime hydration and integrity truth. |
| AC-024 | Runtime/Workspace Separation | EV-012 + EV-001 | PROVEN_FOR_RELEASE_GATE | Phase 10 exit-gate mapping plus fresh W03 shell rerun cover runtime hydration and integrity truth. |

## 14.6 Provider Configuration and Secret Handling Acceptance Criteria
Evidence anchor: EV-006 Phase 4 closure evidence plus fresh W03 shell/frontend reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-025 | Provider Selection Requirement | EV-006 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 4 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-026 | Provider Key Storage Requirement | EV-006 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 4 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-027 | OS-Native Secret Storage | EV-006 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 4 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-028 | No Plaintext Secret Persistence | EV-006 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 4 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-029 | Configuration Gating | EV-006 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 4 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-030 | Configuration Non-Promise | EV-006 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 4 closure mapping reaffirmed by clean W03 shell/frontend reruns. |

## 14.7 Seed Document and Preflight Acceptance Criteria
Evidence anchor: EV-007 Phase 5 closure evidence plus fresh W03 shell/frontend reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-031 | Single Seed Document Per Run | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-032 | Supported File-Type Enforcement | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-033 | File Size Enforcement | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-034 | Preflight Required | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-035 | Preflight Outcome Set | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-036 | English-Only Enforcement | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-037 | Rejection Reason Visibility | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-038 | Downgrade Transparency | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-039 | Start Eligibility Gating | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-040 | Preflight Invalidation on Material Change | EV-007 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 5 closure mapping reaffirmed by clean W03 shell/frontend reruns. |

## 14.8 Run Initialization and Persona Generation Acceptance Criteria
Evidence anchor: EV-008 Phase 6 closure evidence plus fresh W03 shell/frontend/engine reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-041 | Fresh Run Creation | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-042 | Fresh Workspace Allocation | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-043 | No Cross-Run Workspace Reuse | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-044 | Persona Generation at Runtime | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-045 | Moderator Agent Existence | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-046 | Perspective Diversity | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-047 | Persona Traceability | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |

## 14.9 Moderated Debate Acceptance Criteria
Evidence anchor: EV-008 Phase 6 closure evidence plus fresh W03 shell/frontend/engine reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-048 | Moderated Debate Model | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-049 | Speaker Subset Rule | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-050 | No Full-Fanout Normal Rounds | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-051 | Round Summary Continuity | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-052 | Speaker Attribution | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-053 | No Fabricated Debate Content | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |

## 14.10 Memory and Context Management Acceptance Criteria
Evidence anchor: EV-008 Phase 6 closure evidence plus fresh W03 shell/frontend/engine reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-054 | Bounded Memory Strategy | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-055 | Seed Context Grounding | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-056 | Round Summary Memory | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-057 | Argument/Risk Memory | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-058 | Transcript Archive Distinction | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-059 | No Cross-Run Semantic Leakage | EV-008 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 6 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |

## 14.11 Budget and Duration Acceptance Criteria
Evidence anchor: EV-009 Phase 7 closure evidence plus fresh W03 shell/frontend/engine reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-060 | Mandatory Max Token Budget | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-061 | Budget Tracking | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-062 | Budget Visibility | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-063 | Budget Warning | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-064 | Budget-Hit Control | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-065 | Budget-Capped Completion Classification | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-066 | No Hidden Budget Overrun | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-067 | Hard Two-Hour Duration Cap | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-068 | Duration-Cap Control | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-069 | No False Full Completion After Truncation | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |

## 14.12 Active Run Visibility Acceptance Criteria
Evidence anchor: EV-009 Phase 7 closure evidence plus fresh W03 shell/frontend/engine reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-070 | Live Run Surface | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-071 | Phase Visibility | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-072 | Incremental Event Visibility | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-073 | Transcript/Event Chronology | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-074 | No Static Black-Box During Active Debate | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |

## 14.13 Interruption, Retry, and Degradation Acceptance Criteria
Evidence anchor: EV-009 Phase 7 closure evidence plus fresh W03 shell/frontend/engine reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-075 | Rate-Limit Degradation Support | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-076 | Network Degradation Support | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-077 | Bounded Retry Logic | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-078 | Retry-vs-Failure Distinction | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-079 | Finalize-vs-Fail Truthfulness | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-080 | Shell Stability Under Provider Failure | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |

## 14.14 Cancellation and Shutdown Acceptance Criteria
Evidence anchor: EV-009 Phase 7 closure evidence plus fresh W03 shell/frontend/engine reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-081 | Cancel Control Availability | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-082 | Controlled Cancellation | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-083 | No Cancel-as-Completion | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-084 | Graceful-Then-Forceful Shutdown | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-085 | No Hidden Post-Cancel Debate | EV-009 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 7 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |

## 14.15 Report Generation Acceptance Criteria
Evidence anchor: EV-010 Phase 8 closure evidence plus fresh W03 shell/frontend/engine reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-086 | Report Requirement | EV-010 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 8 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-087 | Required Report Sections | EV-010 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 8 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-088 | Report Truthfulness | EV-010 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 8 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-089 | Report Availability Truth | EV-010 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 8 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-090 | No Placeholder Success Report | EV-010 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 8 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |
| AC-091 | Cost Summary Integrity | EV-010 + EV-001 + EV-002 + EV-003 | PROVEN_FOR_RELEASE_GATE | Phase 8 closure mapping reaffirmed by clean W03 shell/frontend/engine reruns. |

## 14.16 Cleanup and Data-Lifecycle Acceptance Criteria
Evidence anchor: EV-011 Phase 9 closure evidence plus fresh W03 shell/frontend reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-092 | Cleanup Attempt Requirement | EV-011 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 9 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-093 | Cleanup Distinction Requirement | EV-011 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 9 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-094 | Cleanup Failure Visibility | EV-011 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 9 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-095 | Startup Janitor Requirement | EV-011 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 9 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-096 | No Crash Resume Requirement | EV-011 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 9 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-097 | Report Preservation Requirement | EV-011 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 9 closure mapping reaffirmed by clean W03 shell/frontend reruns. |
| AC-098 | No Cross-Run Working-Data Reuse | EV-011 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 9 closure mapping reaffirmed by clean W03 shell/frontend reruns. |

## 14.17 Security, Privacy, and Storage Acceptance Criteria
Evidence anchor: EV-013 cross-phase security/privacy review plus closed-phase packet evidence.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-099 | Localhost Trust Boundary | EV-013 + EV-014 + EV-001 | PROVEN_FOR_RELEASE_GATE | Cross-phase trust-boundary evidence is now aggregated and W04 recorded no local auth-boundary finding. |
| AC-100 | No Plaintext Secret Leakage | EV-013 + EV-014 + EV-001 + EV-004 | PROVEN_FOR_RELEASE_GATE | Cross-phase secret-boundary evidence is now aggregated and W04 recorded no plaintext leakage finding. |
| AC-101 | Runtime Integrity Block | EV-012 + EV-001 | PROVEN_FOR_RELEASE_GATE | Phase 10 closure and fresh W03 shell rerun prove runtime integrity blocking. |
| AC-102 | Update Integrity Verification | EV-012 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Phase 10 closure and fresh W03 shell/frontend reruns prove verified update handling. |
| AC-103 | Local-Data-First Compliance | EV-013 + EV-014 + EV-004 + EV-011 | PROVEN_FOR_RELEASE_GATE | Cross-phase persistence-boundary evidence is now aggregated and W04 recorded no hidden-persistence finding. |
| AC-104 | Residual-Risk Honesty | EV-013 + EV-014 + EV-011 + EV-002 | PROVEN_FOR_RELEASE_GATE | Cleanup-truth and residual-risk disclosure evidence is now aggregated and W04 recorded no concealment finding. |

## 14.18 UI/UX Acceptance Criteria
Evidence anchor: EV-012 Phase 10 UI truth evidence plus EV-013 cross-phase review and EV-015 final W05 synthesis.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-105 | Non-Expert Operability | EV-012 + EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | W05 confirmed the Setup, Run, Reports, and Diagnostics workflow stays bounded to guided user actions instead of requiring users to operate raw internal mechanics. |
| AC-106 | Disabled-State Clarity | EV-012 + EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | `app/src/hooks/useSetupScreenState.ts` and `app/src/App.test.tsx` prove the UI disables submit/start actions until real prerequisites are satisfied. |
| AC-107 | Empty-State Clarity | EV-012 + EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | The live screens and shared state panels render unavailable, missing, or not-yet-configured states explicitly instead of implying silent success. |
| AC-108 | Honest Loading States | EV-012 + EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | `app/src/App.tsx` and `app/src/App.wave10.test.tsx` prove updater/runtime-delivery loading, refresh, and error states remain explicit and non-fabricated. |
| AC-109 | Cleanup and Completion Distinction in UX | EV-012 + EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | `app/src/screens/ReportsScreen.tsx` and `app/src/App.wave8.test.tsx` explicitly separate completion truth, report availability truth, cleanup truth, and janitor truth. |
| AC-110 | No Scope-Inflating UI | EV-012 + EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | `app/src/appViewModels.ts` keeps the UI bounded to OpenAI/Google, supported seed types, and no local HTTP, hidden runtime hydration, or extra setup-screen promises. |

## 14.19 Logging and Diagnostics Acceptance Criteria
Evidence anchor: EV-013 cross-phase diagnostics review plus EV-015 final W05 synthesis.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-111 | Minimal Diagnostics | EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | W05 confirmed the diagnostics screen stays narrowly scoped to runtime personas, bounded memory, moderated rounds, live state projections, and repo truth rather than broad hidden telemetry surfaces. |
| AC-112 | No Secret Logging | EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | The live repo keeps raw provider secrets and raw activation tokens out of ordinary logs, and the backend stores only token fingerprints rather than plaintext tokens. |
| AC-113 | No Routine Full Transcript Logging | EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | `engine/src/miro_fish_engine/transcript_archive.py` keeps transcript archival append-only and distinct from prompt memory, and W05 found no routine frontend or shell transcript logging surface. |
| AC-114 | No Routine Full Seed Logging | EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | The bounded Preflight and run-start surfaces retain metadata and fingerprints without a routine raw-seed logging channel in the live diagnostics or UI layers. |
| AC-115 | Lifecycle Traceability in Diagnostics | EV-013 + EV-015 | PROVEN_FOR_RELEASE_GATE | The diagnostics and monitoring surfaces expose lifecycle, runtime-delivery, and report-access truth explicitly enough for operator review without fabricating extra authority. |

## 14.20 State and Interface Acceptance Criteria
Evidence anchor: EV-013 cross-phase interface aggregation plus closed Phase 5 and Phase 7 evidence.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-116 | Canonical State Fidelity | EV-013 + EV-007 + EV-009 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Closed Phase 5/7 interface-state evidence aggregated and reaffirmed during W03. |
| AC-117 | No Invalid Start Without Eligible Preflight | EV-013 + EV-007 + EV-009 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Closed Phase 5/7 interface-state evidence aggregated and reaffirmed during W03. |
| AC-118 | Ordered Event Interface | EV-013 + EV-007 + EV-009 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Closed Phase 5/7 interface-state evidence aggregated and reaffirmed during W03. |
| AC-119 | Status/Event Consistency | EV-013 + EV-007 + EV-009 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Closed Phase 5/7 interface-state evidence aggregated and reaffirmed during W03. |
| AC-120 | Structured Error Responses | EV-013 + EV-007 + EV-009 + EV-001 + EV-002 | PROVEN_FOR_RELEASE_GATE | Closed Phase 5/7 interface-state evidence aggregated and reaffirmed during W03. |

## 14.21 Commercial Backend Acceptance Criteria
Evidence anchor: EV-005 Phase 3 closure evidence plus fresh W03 commercial-backend reruns.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-121 | Durable License Ledger | EV-005 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 commercial-backend mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-122 | Durable Activation Ledger | EV-005 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 commercial-backend mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-123 | Payment Event Traceability | EV-005 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 commercial-backend mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-124 | Activation Slot Enforcement | EV-005 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 commercial-backend mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-125 | JWT Device Binding | EV-005 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 commercial-backend mapping reaffirmed by clean W03 shell/backend reruns. |
| AC-126 | No Simulation Persistence in Commercial Backend | EV-005 + EV-004 | PROVEN_FOR_RELEASE_GATE | Phase 3 commercial-backend mapping reaffirmed by clean W03 shell/backend reruns. |

## 14.22 Acceptance-Criteria Summary
Evidence anchor: EV-013 cross-phase release synthesis staging plus EV-015 final W05 verdict formation.

| Acceptance ID | Criterion title | Validation artifact reference | Evidence status | Notes |
| --- | --- | --- | --- | --- |
| AC-127 | Section Summary Rule | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | W05 and T06 kept the summary grounded in already-populated acceptance, regression, red-team, blocker, and scope-governance evidence rather than inventing a new completion story. |
| AC-128 | No Selective Completion Rule | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | The final release record preserves the historical T05 blocker truth and now records a current `release-ready` outcome only because `AC-006` was resolved through governed Windows-only scope realignment. |
| AC-129 | Section Boundary Rule | EV-013 + EV-015 + EV-016 | PROVEN_FOR_RELEASE_GATE | W05 and T06 kept the summary bounded to governed release inputs, blocker accounting, and acceptance synthesis without introducing new scope or parallel success criteria. |