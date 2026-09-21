# Phase 11 QA and Regression Matrix

## Purpose
This file is the repo-local Phase 11 QA matrix and regression classification inventory.
It now records the W03/W04 evidence set plus the W05 final aggregation and the T06 governed scope-realignment refresh needed to close the release gate honestly.

## QA Outcome Legend
| Outcome | Meaning |
| --- | --- |
| NOT_YET_EXECUTED | The row is instantiated but no evidence run has been recorded yet. |
| PASS | The governed check passed with traceable evidence. |
| FAIL | The governed check failed and requires blocker or rework classification. |
| BLOCKED | The check could not run truthfully because prerequisites or tooling were missing. |
| PARTIAL | Some evidence exists, but the row is not yet comprehensive enough for the release gate. |

## Evidence Type Categories
| Evidence type | Intended use |
| --- | --- |
| interface behavior evidence | command, route, or request or response truth |
| schema conformance evidence | payload shape and field-boundary proof |
| state transition evidence | lifecycle, readiness, status, or terminal-path proof |
| ordered event evidence | chronology and sequence proof |
| artifact existence evidence | report, runtime, or workspace artifact proof |
| secure-store handling evidence | secret-boundary and storage proof |
| cleanup/janitor evidence | cleanup attempt, failure visibility, and janitor proof |
| report/output evidence | report-content, availability, and access proof |
| regression-run evidence | gate rerun output and targeted revalidation proof |
| human review evidence | governed manual review where automation alone is insufficient |

## W03/W04/W05/T06 Validation Execution Record
| Evidence bundle | Command family | Result |
| --- | --- | --- |
| W03-RG-001 | `cargo check`; `cargo clippy -- -D warnings`; `cargo test` in `shell/` | PASS — 144 shell tests green on 2026-04-17 |
| W03-RG-002 | `npm run lint`; `npm run typecheck`; `npm run test`; `npm run build` in `app/` | PASS — 6 Vitest files and 16 frontend tests green on 2026-04-17 |
| W03-RG-003 | `python -m compileall engine/src engine/tests`; `python -m ruff check engine`; `python -m pytest engine/tests` | PASS — 17 engine tests green on 2026-04-17 |
| W03-RG-004 | `python -m compileall commercial_backend/src commercial_backend/tests`; `python -m ruff check commercial_backend`; `python -m pytest commercial_backend/tests` | PASS — 11 commercial-backend tests green on 2026-04-17 |
| W03-RG-005 | Cross-phase manual review against closed-phase packet evidence, populated acceptance rows, and live execution surfaces | PASS with bounded partial scope on final non-functional and red-team synthesis |
| W04-RT-001 | Scenario-by-scenario red-team evidence review across `execution/phase_11_validation/red_team_matrix.md` and targeted code/test inspection | PASS — RT-001 through RT-012 reviewed on 2026-04-17 with no blocker, rework, or residual entry recorded |
| W05-AGG-001 | Final cross-phase aggregation of unresolved acceptance, blocker, and release-gate inputs against the live repo and Document B release rules | PASS — W05 completed on 2026-04-17 with a governed `release-blocked` outcome because `AC-006` remains a platform-proof blocker |
| T06-RR-001 | Governed scope-realignment review across the Windows-only Document A revision, the existing Phase 11 evidence set, and the final release surfaces | PASS — T06 completed on 2026-04-17 by resolving `BL-001`, refreshing `AC-006` and `QA-009`, and selecting the governed `release-ready` outcome without product-code changes or new reruns |

## QA Matrix
| QA test ID | Validation domain | Target artifact or behavior | Preconditions | Action or scenario | Expected governed result | Evidence type | Outcome | Revalidation trigger note |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| QA-001 | structural validation | execution OS, packet boundaries, and Phase 11 artifact pack | W02 artifact pack exists | review packet allowlist, artifact ownership, and live execution surfaces | repo-local Phase 11 artifacts exist without changing product scope or creating a parallel release authority | human review evidence | PASS | W03 manual review populated the acceptance and QA artifacts; rerun after any execution-surface or packet-boundary change |
| QA-002 | behavior validation | shell, frontend, engine, and backend release-critical flows | closed-phase implementation evidence exists | rerun governed behavior families for run, report, cleanup, activation, and updater or runtime flows | user-visible and shell-owned behavior remains aligned to Document A | regression-run evidence | PASS | W03-RG-001 through W03-RG-004 passed cleanly; rerun after any release-critical behavior change |
| QA-003 | state-machine validation | lifecycle, readiness, updater, cleanup, and report state families | canonical state surfaces are present | exercise state transitions and terminal-path truth across active and terminal flows | states remain distinct, truthful, and non-collapsed across lifecycle families | state transition evidence | PASS | Closed Phase 5, 7, 8, and 9 evidence plus W03-RG-001 through W03-RG-003 now cover the current state families |
| QA-004 | data-policy validation | secure storage, cleanup scope, workspace boundaries, and report preservation | persistence and cleanup surfaces are present | review storage, cleanup, residual, and report-boundary behavior | no hidden persistence or forbidden storage path appears, and report preservation rules remain truthful | secure-store handling evidence | PASS | Closed Phase 3, 4, 8, and 9 evidence plus W03-RG-001 and W03-RG-004 now cover the current storage-boundary surfaces |
| QA-005 | interface validation | Tauri command surfaces, local interface truth, and backend CLI contract boundaries | interface and CLI surfaces exist | verify request or response shapes and bounded shell authority | interfaces remain truthful, structured, and bounded without hidden authority transfer to the frontend | interface behavior evidence | PASS | Closed Phase 5, 7, and 10 evidence plus W03-RG-001, W03-RG-002, and W03-RG-004 now cover the current interface surfaces |
| QA-006 | commercial/activation validation | licensing, activation, denial, device binding, and slot enforcement flows | Phase 3 activation surfaces are present | rerun activation and backend validation families with release-gate review | activation truth, denial truth, and two-device enforcement remain evidence-backed | regression-run evidence | PASS | Phase 3 closure evidence reaffirmed by W03-RG-001 and W03-RG-004 |
| QA-007 | UI truthfulness validation | frontend loading, disabled-state, error, diagnostics, and lifecycle rendering | frontend release surfaces are present | verify that the UI reflects shell-owned truth without scope inflation | the UI remains presentation-only, honest, and non-fabricated | interface behavior evidence | PASS | Closed Phase 7, 8, 9, and 10 UI evidence reaffirmed by W03-RG-002 and W03-RG-005 |
| QA-008 | cleanup and janitor validation | terminal cleanup, cleanup result truth, abnormal residual detection, and startup janitor behavior | Phase 9 surfaces are present | rerun cleanup and janitor validation families | cleanup is attempted, failures remain visible, and janitor behavior does not become resume behavior | cleanup/janitor evidence | PASS | Phase 9 closure evidence reaffirmed by W03-RG-001 and W03-RG-002 |
| QA-009 | non-functional validation | boundedness, platform scope, logging discipline, responsiveness, and runtime or update trust posture | NFR and platform constraints remain frozen | verify release-critical NFR families and trust rules | platform, boundedness, logging, and trust posture remain within Document A constraints | human review evidence | PASS | T06 refreshed the non-functional review after the governed Windows-only scope revision; the current platform rule is satisfied by existing evidence and no other non-functional blocker remains |
| QA-010 | acceptance criteria validation | final acceptance mapping coverage | acceptance mapping scaffold exists | populate acceptance rows from phase evidence and focused Phase 11 checks | every release-relevant acceptance row gains traceable evidence or an explicit blocker | regression-run evidence | PASS | T06 leaves every release-relevant acceptance row in `PROVEN_FOR_RELEASE_GATE` status with no remaining blocker placeholder |
| QA-011 | red-team validation | priority misuse, exploit, and truth-failure scenarios | red-team matrix exists | execute the red-team matrix against critical and high-priority focus areas | findings feed blocker classification, rework, residual-risk accounting, or release blocking | human review evidence | PASS | W04 executed the current scenario inventory with no material finding recorded; rerun after any security, cleanup, trust, or truth-surface change |

## W03/W04/W05/T06 QA Outcome Notes
| QA test ID | Evidence bundle | Outcome rationale |
| --- | --- | --- |
| QA-001 | W03-RG-005 | The execution OS, packet boundaries, and artifact ownership stayed bounded while W03 populated evidence. |
| QA-002 | W03-RG-001 through W03-RG-004 | Fresh shell, frontend, engine, and commercial-backend reruns all passed cleanly against the live repo. |
| QA-003 | W03-RG-001 through W03-RG-003 | State, lifecycle, event, finalization, cleanup, and updater projections remain mutually consistent across the live surfaces. |
| QA-004 | W03-RG-001, W03-RG-004, W03-RG-005 | Secret boundaries, cleanup scope, workspace isolation, and durable report preservation remain traceable and bounded. |
| QA-005 | W03-RG-001, W03-RG-002, W03-RG-004 | Shell-owned interface authority and structured command/CLI boundaries remain intact. |
| QA-006 | W03-RG-001, W03-RG-004 | Activation approval/denial/error classification and two-device enforcement remain evidence-backed. |
| QA-007 | W03-RG-002, W03-RG-005 | The frontend continues to consume shell-owned truth without inventing extra lifecycle or release authority. |
| QA-008 | W03-RG-001, W03-RG-002 | Cleanup triggering, cleanup-failure visibility, and janitor no-resume behavior remain evidence-backed. |
| QA-009 | W03-RG-005 + W05-AGG-001 + T06-RR-001 | W05 recorded the historical platform blocker honestly under the earlier contract, and T06 then resolved it after the governed Windows-only scope realignment without exposing any new non-functional contradiction. |
| QA-010 | W03-RG-001 through W03-RG-005 + W05-AGG-001 + T06-RR-001 | T06 completed the acceptance map by converting the historical `AC-006` blocker to `PROVEN_FOR_RELEASE_GATE` after the governed scope revision. |
| QA-011 | W04-RT-001 | W04 reviewed all currently defined priority scenarios and recorded no blocker, rework, or residual entry from the current evidence set. |

## Regression Classification Inventory
| Regression class | Affected validation domains | Trigger change type | Priority level | Re-run scope | Upstream traceability note |
| --- | --- | --- | --- | --- | --- |
| RG-001 full release-gate rerun | all validation domains | any reopened closed phase, release-critical rework, or cross-phase truth repair | highest | rerun all applicable shell, frontend, engine, and backend gates plus acceptance, QA, and red-team aggregation | BVAL-044, BVAL-047, CQA-023 through CQA-025 |
| RG-002 truth-bearing domain rerun | behavior, state-machine, interface, acceptance | shell, engine, backend, or report flow change in a release-critical domain | high | rerun impacted phase closure evidence plus adjacent Phase 11 matrix rows | BVAL-026, BVAL-044, CQA-024 |
| RG-003 frontend truth rerun | UI truthfulness, interface, acceptance | user-visible rendering, disabled-state, loading-state, or diagnostics change | high | rerun frontend gates plus supporting shell contract or state checks | BVAL-018, BVAL-026, CQA-024 |
| RG-004 persistence and cleanup rerun | data-policy, cleanup and janitor, red-team | cleanup, janitor, secure-store, report-preservation, or workspace-boundary change | highest | rerun cleanup, janitor, persistence-boundary, and relevant red-team rows | BVAL-015, BVAL-019, BVAL-057, CQA-024 |
| RG-005 runtime or update trust rerun | behavior, non-functional, red-team, acceptance | runtime hydration, runtime integrity, updater, or artifact-trust-path change | highest | rerun runtime and updater gates plus relevant red-team and acceptance rows | BVAL-020, BVAL-021, BVAL-044, CQA-024 |
| RG-006 activation or backend rerun | commercial/activation, interface, red-team | activation submission, slot enforcement, device binding, or backend ledger change | highest | rerun activation and backend gates plus relevant red-team and acceptance rows | BVAL-017, BVAL-021, BVAL-044, CQA-024 |
| RG-007 governance and drift rerun | structural, acceptance, non-functional | packet, execution-surface, acceptance wording, or release-accounting artifact change | high | rerun contradiction review plus the affected matrix and release-assessment rows | BVAL-025, BVAL-026, CQA-031, CQA-045 |

## W03 Regression Classification Status
| Regression class | Current W03 status | Evidence note |
| --- | --- | --- |
| RG-001 full release-gate rerun | NOT_TRIGGERED | No reopened phase or post-W03 remediation currently forces the full release-gate rerun set. |
| RG-002 truth-bearing domain rerun | EXECUTED | W03 reran the release-critical shell, frontend, engine, and backend suites cleanly. |
| RG-003 frontend truth rerun | EXECUTED | W03-RG-002 reran the user-visible truth surfaces cleanly. |
| RG-004 persistence and cleanup rerun | EXECUTED | W03-RG-001 and W03-RG-004 reaffirmed cleanup, report-boundary, and secret-boundary behavior. |
| RG-005 runtime or update trust rerun | EXECUTED | W03-RG-001 and the closed Phase 10 evidence reaffirm runtime and updater trust posture before red-team review. |
| RG-006 activation or backend rerun | EXECUTED | W03-RG-001 and W03-RG-004 reaffirmed activation, slot enforcement, and backend-ledger behavior. |
| RG-007 governance and drift rerun | EXECUTED | W03 populated the acceptance map, updated live execution surfaces, and completed a contradiction review pass. |
