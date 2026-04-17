## 0. Document Control

### 0.1 Section Purpose
This section defines the identity, authority, governance, interpretation rules, and usage boundaries of Document B.

This section exists to ensure that:
- Document B is treated as an execution-planning document rather than a second product brief
- AI coding agents do not use execution convenience to override locked product truth
- human reviewers can determine exactly how Document B must relate to Document A and Document C
- sequencing, dependencies, phase logic, and task decomposition remain subordinate to the governing brief

This section is the highest-authority section inside Document B.

---

### 0.2 Document Identity

#### BCTRL-001 — Document Title
The official title of this document is:

**Master Execution Plan**

#### BCTRL-002 — Document Role
Document B is the **primary authoritative execution-planning document** for the project.

#### BCTRL-003 — Document Type
Document B is an execution-governance document.
It is not:
- the supreme product brief
- the engineering schema annex
- the OpenAPI file
- the SQL migration package
- the QA spreadsheet
- the UI design system
- the source code

#### BCTRL-004 — Core Objective
The objective of Document B is to translate the governing truth of Document A into:
- execution order
- workstreams
- dependency logic
- implementation phases
- task packet structure
- validation gates
- replanning rules

#### BCTRL-005 — Planning Discipline Objective
Document B must make implementation buildable by converting product law into deterministic execution order without changing product law itself.

---

### 0.3 Governing Authority

#### BCTRL-006 — Subordinate Authority Rule
Document B is subordinate to **Document A — Master Project Creative Brief**.

#### BCTRL-007 — Governing Upstream Version Rule
Until explicitly replaced by a later approved version, the governing upstream document for Document B is:

**Document A — Master Project Creative Brief, Version V2.A-Final-Draft-2**

#### BCTRL-008 — Document A Supremacy Rule
If any statement in Document B conflicts with Document A, Document A wins.

#### BCTRL-009 — No Override Authority Rule
Document B has no authority to:
- modify product scope
- redefine behavior
- reinterpret constitutional rules
- override state-machine truth
- weaken architecture boundaries
- loosen data-policy restrictions
- redefine acceptance criteria meaning

#### BCTRL-010 — Execution-Only Authority Rule
Document B is authoritative only for:
- execution order
- implementation grouping
- sequencing
- dependency mapping
- phase gates
- task packet standards
- execution replanning rules
- delivery readiness flow

#### BCTRL-011 — No Hidden Product-Law Rule
Document B must not become a hidden second source of product law.

---

### 0.4 Relationship to Document A and Document C

#### BCTRL-012 — Relationship Map Rule
The project documentation stack consists at minimum of:

- **Document A — Master Project Creative Brief**  
  The supreme source-of-truth for product, behavior, architecture, policy, and acceptance meaning.

- **Document B — Master Execution Plan**  
  The execution-order, dependency, phase, and delivery-control document.

- **Document C — Engineering Annex Pack**  
  The machine-readable and technical-detail annex package including schemas, migrations, event contracts, test matrices, and engineering reference material.

#### BCTRL-013 — Relationship to Document A
Document B must derive its execution logic from Document A.

#### BCTRL-014 — Relationship to Document C
Document B may direct when and why a Document C artifact must be created, but must not replace Document C’s function as the technical annex layer.

#### BCTRL-015 — No A-to-B Reversal Rule
Document A governs Document B.
Document B does not govern Document A.

#### BCTRL-016 — No B-to-C Supremacy Over A Rule
Document B may guide the sequencing of Document C work, but neither Document B nor Document C may override Document A.

---

### 0.5 Intended Audience

#### BCTRL-017 — Primary Audience
The primary audiences of Document B are:
1. the project owner
2. AI coding agents executing implementation work
3. human reviewers coordinating implementation
4. QA and verification planners
5. future execution coordinators maintaining delivery order

#### BCTRL-018 — AI Audience Rule
Document B is explicitly written to be consumed by AI coding agents.
Therefore:
- sequencing must be explicit
- dependencies must be explicit
- gates must be explicit
- task packet structure must be explicit
- escalation conditions must be explicit

#### BCTRL-019 — Non-Expert Owner Support Rule
Document B should remain understandable to the project owner even if lower-level engineering details are delegated to Document C.

---

### 0.6 Document Scope

#### BCTRL-020 — In-Scope Content Rule
Document B is in scope for:
- execution sequencing
- workstream mapping
- phase breakdown
- dependency order
- task packet format
- entry and exit gates
- validation flow
- replan rules
- escalation rules
- execution drift control

#### BCTRL-021 — Out-of-Scope Content Rule
Document B is out of scope for:
- redefining product features
- changing the activation model
- changing the no-resume policy
- changing the English-only policy
- changing data persistence rules
- changing the moderated simulation model
- defining full machine-readable API schemas
- defining SQL DDL in full
- defining full UI component specifications

#### BCTRL-022 — No Product-Rewrite Rule
Document B must not restate all of Document A as if it were rewriting the product brief.
It must instead translate Document A into execution order.

---

### 0.7 Interpretation Rules

#### BCTRL-023 — Descending Authority Rule
Document B must be interpreted in descending authority order:
1. Section 0 — Document Control
2. Section 1 — Execution Constitution
3. Section 4 — Global Dependency Model
4. Section 5 — Phase Map
5. Section 6 — Phase-by-Phase Execution Plans
6. Section 8 — Validation, QA, and Exit Gates
7. Section 9 — Replan, Escalation, and Drift-Control Rules
8. Section 10 — AI-Agent Operating Protocol for Execution
9. Section 11 — Change Log

#### BCTRL-024 — Upstream Supremacy Rule
All interpretation of Document B must remain subordinate to:
- Document A Section 0 — Document Control
- Document A Section 1 — Project Constitution
- Document A Section 3 — Glossary and Canonical Terminology
- Document A Sections 6–14 where execution meaning depends on behavior, state, architecture, data policy, interface contract, and acceptance criteria

#### BCTRL-025 — Normative Language Rule
Inside Document B:
- **MUST** = mandatory execution requirement
- **MUST NOT** = prohibited execution behavior
- **SHOULD** = recommended execution behavior unless strong reason exists otherwise
- **SHOULD NOT** = discouraged execution behavior unless justified
- **MAY** = optional execution behavior within allowed scope

#### BCTRL-026 — No Silent Invention Rule
If execution detail is missing, an AI coding agent or human implementer must not silently invent new product truth.
The gap must be handled through:
- a narrow implementation assumption explicitly labeled as such
- a task-level note
- or escalation back to the governing brief when the gap affects product truth

#### BCTRL-027 — Narrow-Interpretation Rule
If an execution instruction appears ambiguous, the interpretation must prefer:
1. the narrower and safer reading
2. the reading most consistent with Document A
3. the reading least likely to create scope drift or hidden persistence

---

### 0.8 Conflict Resolution Rules

#### BCTRL-028 — Internal Conflict Resolution Rule
If two parts of Document B appear to conflict, precedence must be resolved in the following order:
1. Section 0 — Document Control
2. Section 1 — Execution Constitution
3. Section 4 — Global Dependency Model
4. Section 8 — Validation, QA, and Exit Gates
5. Section 9 — Replan, Escalation, and Drift-Control Rules
6. Section 5 — Phase Map
7. Section 6 — Phase-by-Phase Execution Plans
8. Section 7 — Execution Unit / Task Packet Standard
9. Section 10 — AI-Agent Operating Protocol for Execution
10. Section 11 — Change Log

#### BCTRL-029 — Upstream Conflict Rule
If any part of Document B conflicts with Document A, Document A wins immediately and Document B must be revised rather than reinterpreted.

#### BCTRL-030 — Specific Over General Rule
Where a specific execution rule and a general execution rule both apply, the specific rule governs only within its local scope while the general rule remains valid elsewhere.

---

### 0.9 Traceability Rules

#### BCTRL-031 — Traceability Requirement
Every major execution phase, workstream, and task packet in Document B must trace to one or more governing rules in Document A.

#### BCTRL-032 — No Orphan Execution Rule
No major execution task should exist in Document B without an identifiable justification in Document A or a clearly labeled implementation necessity that does not change product truth.

#### BCTRL-033 — Rule Reference Discipline
Where useful, Document B should reference the relevant upstream rule families, including:
- constitutional rules
- behavior rules
- state rules
- architecture rules
- data rules
- interface rules
- acceptance criteria

#### BCTRL-034 — Acceptance Traceability Rule
Execution exit gates must map back to Document A acceptance logic rather than inventing a parallel definition of completion.

---

### 0.10 Stability and Revision Rules

#### BCTRL-035 — Locked Structure Rule
Once the structure of Document B is frozen, later drafting should preserve that structure unless explicitly revised through the Change Log.

#### BCTRL-036 — No Silent Sequence Change Rule
Material changes to phase order, dependency logic, or exit-gate logic must be recorded in Document B’s Change Log.

#### BCTRL-037 — No Silent Drift Rule
Execution convenience must not silently mutate:
- phase meaning
- workstream ownership
- dependency truth
- validation truth

#### BCTRL-038 — Document Version Rule
Document B must maintain explicit version and status metadata once the unified file is assembled.

---

### 0.11 AI-Agent Usage Rules

#### BCTRL-039 — AI Execution Usage Rule
AI coding agents using Document B must treat it as the authoritative source for:
- execution order
- dependency-safe implementation sequence
- task packet structure
- validation gates
- escalation rules

#### BCTRL-040 — AI Non-Override Rule
AI coding agents must not use Document B as permission to override Document A.

#### BCTRL-041 — AI Drift Prevention Rule
If an AI coding agent detects that a requested execution shortcut would violate Document A or Document B, it must:
- stop
- flag the issue
- and escalate rather than silently changing the plan

---

### 0.12 Boundaries of This Section

#### BCTRL-042 — Section Boundary Rule
This section defines only:
- document identity
- authority
- relationship to other documents
- interpretation
- traceability
- governance rules for execution planning

This section does not yet define:
- the execution constitution in detail
- workstreams
- phases
- task packet format
- validation gates
- replanning logic

Those are defined in later sections.

---

## 1. Execution Constitution

### 1.1 Section Purpose
This section defines the non-negotiable execution truths of Document B.

This section exists to ensure that:
- implementation sequence remains subordinate to product truth
- AI coding agents do not treat execution convenience as authority
- work is sequenced according to dependency truth rather than improvisation
- downstream implementation remains auditable and controlled

This section is the highest execution-law layer of Document B after Section 0.

---

### 1.2 Constitutional Role of Document B

#### BCONST-001 — Execution Translation Rule
Document B exists to translate Document A into:
- build order
- dependency order
- workstreams
- implementation phases
- task packet structure
- validation checkpoints
- release readiness flow

#### BCONST-002 — No Product Redefinition Rule
Document B must not redefine the product.
It may only define how the locked product is executed.

#### BCONST-003 — Execution Governance Rule
Document B is the governing document for execution sequencing, not for product meaning.

#### BCONST-004 — Program-Control Rule
Document B must function as a program-control layer that prevents random, out-of-order, or AI-invented implementation behavior.

---

### 1.3 What Document B Is Allowed to Do

#### BCONST-005 — Allowed Translation Rule
Document B may translate Document A into:
- milestone order
- phase groupings
- workstream boundaries
- dependency maps
- task packet templates
- implementation gates
- QA gates
- replan rules

#### BCONST-006 — Allowed Sequencing Rule
Document B may define what gets built:
- first
- second
- later
- in parallel
- behind which dependencies
- behind which validation gates

#### BCONST-007 — Allowed Execution Detail Rule
Document B may add execution detail where needed for build order, so long as that detail does not change product truth.

#### BCONST-008 — Allowed Stub Planning Rule
Document B may allow temporary stubs, mocks, or scaffolds for sequencing purposes if it clearly states:
- what is stubbed
- why it is stubbed
- what gate requires replacement with a real implementation

#### BCONST-009 — Allowed Risk Sequencing Rule
Document B may prioritize high-risk foundations early if doing so preserves program safety and reduces rework risk.

---

### 1.4 What Document B Must Not Do

#### BCONST-010 — No New Feature Rule
Document B must not introduce new features not authorized in Document A.

#### BCONST-011 — No Scope Expansion Rule
Document B must not expand scope by treating convenience features as execution necessities.

#### BCONST-012 — No Hidden Product Change Rule
Document B must not smuggle product changes into:
- phase notes
- task packets
- implementation assumptions
- delivery shortcuts
- dependency language

#### BCONST-013 — No Architecture Drift Rule
Document B must not reassign ownership boundaries defined in Document A’s architecture without explicit upstream revision.

#### BCONST-014 — No Data-Policy Drift Rule
Document B must not weaken or bypass the data lifecycle and storage rules defined in Document A.

#### BCONST-015 — No Acceptance Drift Rule
Document B must not create a new definition of done that conflicts with Document A acceptance criteria.

#### BCONST-016 — No Deferred Leakage Rule
Deferred or future-phase ideas may be mentioned for future planning context, but they must not be transformed into active MVP obligations.

#### BCONST-017 — No Stronger Claim Rule
Document B must not strengthen privacy, security, deletion, offline, accuracy, or enterprise-readiness claims beyond Document A.

#### BCONST-018 — No Weaker Constraint Rule
Document B must not weaken:
- Windows/macOS-only support
- English-only policy
- no crash-resume rule
- two-device activation limit
- best-effort cleanup posture
- BYOK model
- moderated 3-to-5 speaker-per-round rule
- mandatory max token budget
- hard two-hour duration cap

---

### 1.5 Execution Philosophy

#### BCONST-019 — Dependency-First Execution Rule
The MVP must be built in dependency-safe order, not in arbitrary order.

#### BCONST-020 — Truth-First Implementation Rule
Implementation must prioritize truth-preserving behavior before convenience or polish.

#### BCONST-021 — Lifecycle-First Rule
Lifecycle-correct system behavior must be established before the product is treated as functionally complete.

#### BCONST-022 — Gate-First Rule
A phase is not complete merely because code exists.
A phase is complete only when its exit conditions are satisfied.

#### BCONST-023 — Buildability Rule
Where multiple execution paths are possible, Document B should prefer the path that is:
- simpler
- more deterministic
- easier to audit
- safer for AI-led implementation
- more consistent with Document A

#### BCONST-024 — No Demo-Driven False Completion Rule
A narrow happy-path demo must not be treated as phase completion if lifecycle truth, cleanup truth, or acceptance truth remain unsatisfied.

---

### 1.6 Foundational Execution Priorities

#### BCONST-025 — Foundations Before Features Rule
The execution plan must prioritize foundational system integrity before feature polish.

#### BCONST-026 — Trust Boundary Before Simulation Rule
The app shell, runtime management, local trust boundary, and activation model must be established before the product is treated as a valid simulation system.

#### BCONST-027 — Preflight Before Run Rule
No execution plan may treat simulation start as complete before Preflight exists and correctly gates run start.

#### BCONST-028 — Finalization Before Success Claims Rule
No implementation phase may declare the run flow complete before truthful finalization and report generation exist.

#### BCONST-029 — Cleanup Before Release Signoff Rule
No release-readiness decision may occur before cleanup and cleanup-truth handling are implemented and tested.

#### BCONST-030 — QA Before Completion Rule
No MVP-complete declaration may occur without explicit validation against Document A acceptance criteria.

---

### 1.7 Workstream Constitutional Rules

#### BCONST-031 — Workstream Separation Rule
Execution work must be grouped into stable workstreams that map cleanly to Document A architecture and responsibility boundaries.

#### BCONST-032 — No Layer Collapse Rule
Document B must not collapse frontend, shell, engine, cloud backend, and data-policy responsibilities into one vague implementation lane.

#### BCONST-033 — Ownership Preservation Rule
If a responsibility belongs to a particular layer in Document A, Document B must preserve that ownership in execution planning.

#### BCONST-034 — Cross-Workstream Dependency Rule
Cross-workstream coordination is allowed, but cross-workstream ownership theft is not.

---

### 1.8 Phase Constitutional Rules

#### BCONST-035 — Phase Requirement Rule
Document B must define implementation in phases rather than as an unstructured task list.

#### BCONST-036 — Phase Boundary Rule
Each phase must have:
- purpose
- prerequisites
- included work
- excluded work
- deliverables
- entry gate
- exit gate

#### BCONST-037 — No Phase Drift Rule
A phase must not silently absorb unrelated obligations just because they are convenient to build at the same time.

#### BCONST-038 — No Future-Feature Sneak Rule
A phase must not implement future-scope items merely because the relevant engineers or AI agents are already “in the area.”

---

### 1.9 Task Packet Constitutional Rules

#### BCONST-039 — Task Packet Requirement
Document B must eventually define a standard execution unit for AI coding agents and human implementers.

#### BCONST-040 — Task Packet Subordination Rule
A task packet must function as an execution wrapper for upstream requirements, not as an independent authority source.

#### BCONST-041 — Task Packet Traceability Rule
Every major task packet must reference the Document A rules that justify its existence.

#### BCONST-042 — Non-Goal Requirement
Every major task packet must define explicit non-goals to prevent accidental scope expansion by AI agents.

#### BCONST-043 — Acceptance Proof Requirement
Every major task packet must define what proof is required to consider the task complete.

---

### 1.10 Validation Constitutional Rules

#### BCONST-044 — Validation Must Be Upstream-Grounded
All execution validation in Document B must map back to Document A rather than inventing new product truth.

#### BCONST-045 — Exit Gate Rule
A phase may close only when its exit criteria are satisfied, not merely when its code is written.

#### BCONST-046 — Release Gate Rule
The MVP may be considered execution-ready for release only when the required acceptance, red-team, and truthfulness gates have been satisfied.

#### BCONST-047 — No Narrative Completion Rule
A persuasive implementation summary is not equivalent to verified completion.

---

### 1.11 Replan and Escalation Constitutional Rules

#### BCONST-048 — Replan Boundary Rule
Replanning is allowed only within the boundaries of Document A and the execution constitution of Document B.

#### BCONST-049 — Escalation Requirement
Escalation is required when:
- an implementation path would violate Document A
- an ambiguity materially affects product truth
- a dependency conflict prevents safe execution order
- a requested shortcut would create hidden scope drift
- a deferred item appears necessary to continue

#### BCONST-050 — No Silent Reinterpretation Rule
Implementation pain does not authorize reinterpretation of Document A.

#### BCONST-051 — Spec Escalation Rule
If a product-truth ambiguity cannot be resolved narrowly and safely, the issue must be escalated to the governing brief rather than solved ad hoc in execution planning.

---

### 1.12 AI-Agent Constitutional Rules

#### BCONST-052 — AI Execution Discipline Rule
AI coding agents using Document B must treat it as the authority for order, not for product truth.

#### BCONST-053 — AI Non-Invention Rule
AI coding agents must not invent:
- new features
- new persistence layers
- new lifecycle shortcuts
- new architecture boundaries
- new definitions of done
while operating under Document B

#### BCONST-054 — AI Dependency Obedience Rule
AI coding agents must not execute work out of dependency order merely because code can be generated out of order.

#### BCONST-055 — AI Escalation Rule
If an AI coding agent detects a likely conflict between execution convenience and upstream truth, it must flag the conflict rather than silently “solving” it.

---

### 1.13 Relationship to Deferred Scope

#### BCONST-056 — Deferred Context Rule
Deferred items from Document A may be referenced in Document B only as future planning context.

#### BCONST-057 — No Deferred Promotion Rule
A deferred item does not become approved merely because it appears in an execution note, dependency note, or future milestone placeholder.

#### BCONST-058 — Explicit Promotion Rule
If a deferred item must become active execution scope, Document A must be revised first or in conjunction with an explicit approved governance change.

---

### 1.14 Release Philosophy

#### BCONST-059 — Honest Release Rule
The execution plan must preserve honest release readiness rather than “good enough to demo” readiness.

#### BCONST-060 — Truth Over Polish Rule
Where tradeoffs are necessary, the execution plan should favor:
- truthful lifecycle behavior
- correct gating
- correct cleanup truth
- correct acceptance alignment
over superficial polish

#### BCONST-061 — No Fake MVP Completion Rule
The MVP must not be declared complete if:
- major acceptance gates remain unmet
- cleanup truth is unimplemented
- Preflight gating is incomplete
- run-state truth is incomplete
- finalization/report truth is incomplete

---

### 1.15 Boundaries of This Section

#### BCONST-062 — Section Boundary Rule
This section defines:
- the execution constitution
- allowed and forbidden roles of Document B
- foundational execution philosophy
- tasking discipline
- validation discipline
- replan discipline
- AI-agent execution discipline

This section does not yet define:
- actual workstream contents in full
- full dependency maps
- the phase list in detail
- the phase-by-phase tasks
- the task packet schema in full
- validation matrices in full

Those are defined in later sections.

---

### 1.16 Final Constitutional Summary

#### BCONST-063 — Execution Summary Rule
At the constitutional level, Document B is defined as a subordinate execution-governance document that translates Document A into dependency-safe build order, workstreams, phases, task packets, and validation gates without changing product truth, expanding scope, weakening constraints, or allowing AI-led implementation drift.

#### BCONST-064 — Stability Rule
All later sections of Document B must remain consistent with this execution constitution unless revised explicitly through the Document B Change Log.

---

## 2. Program Delivery Strategy

### 2.1 Section Purpose
This section defines the macro-level delivery strategy of the MVP.

This section exists to ensure that:
- implementation order follows dependency truth rather than intuition or convenience
- AI coding agents understand the intended build sequence before receiving task-level instructions
- the project is delivered as a controlled program rather than a pile of disconnected code outputs
- major implementation priorities remain aligned with the governing truth of Document A

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Execution Constitution
- Document A — Master Project Creative Brief

If any strategy described here conflicts with Document A or the Execution Constitution of Document B, the higher-authority rule wins.

---

### 2.2 Strategic Role of This Section

#### BSTRAT-001 — Strategy Layer Rule
This section defines the macro delivery philosophy of the project.
It does not yet define detailed task packets or phase-level checklists.

#### BSTRAT-002 — Program-Level Planning Rule
The purpose of this section is to answer:
- what gets built first
- what must be proven early
- what can wait
- what must be grouped together
- what kind of delivery logic governs the whole MVP

#### BSTRAT-003 — No Product-Law Rewriting Rule
This section must not restate Document A as a second product brief.
It must only define how the approved product is delivered safely and deterministically.

---

### 2.3 Primary Delivery Objective

#### BSTRAT-004 — Delivery Objective Rule
The primary delivery objective of the MVP is:

To produce a release-ready Windows-and-macOS desktop product that faithfully implements the bounded, moderated, local-data-first, cloud-inference-dependent operating model defined in Document A, while preserving lifecycle truth, cleanup truth, activation truth, and report truth.

#### BSTRAT-005 — Buildability Objective Rule
The delivery strategy must optimize for:
- deterministic implementation order
- manageable AI-agent tasking
- low ambiguity
- low hidden architectural drift
- controlled validation
- auditable completion criteria

#### BSTRAT-006 — MVP Completion Objective Rule
The program must aim not merely for “something working,” but for:
- readiness-gated execution
- truthful live state visibility
- truthful finalization
- correct cleanup behavior
- correct activation behavior
- correct dependency order
- acceptance-aligned release readiness

---

### 2.4 Delivery Philosophy

#### BSTRAT-007 — Dependency-First Philosophy Rule
The project must be built in dependency-safe order rather than convenience-first order.

#### BSTRAT-008 — Truth-First Philosophy Rule
The delivery strategy must prioritize implementation of truth-bearing system behavior before visual polish or non-essential refinement.

#### BSTRAT-009 — Foundation-Before-Polish Rule
Foundational system integrity must be delivered before:
- cosmetic UX refinement
- optional convenience abstractions
- non-essential optimization
- packaging polish beyond what is required for safe iteration

#### BSTRAT-010 — Lifecycle-First Philosophy Rule
Lifecycle correctness must be implemented early because many later features depend on correct state behavior and termination behavior.

#### BSTRAT-011 — Gate-Driven Philosophy Rule
Progress must be measured by passing gates, not by volume of code written.

#### BSTRAT-012 — No Demo-Driven Strategy Rule
A demo-friendly path must not override dependency-safe delivery if it would create false confidence or untestable lifecycle drift.

---

### 2.5 Strategic Delivery Priorities

#### BSTRAT-013 — Priority Stack Rule
The program-level priority stack must be, in descending order:

1. governance and traceability discipline  
2. app shell and trust boundary correctness  
3. runtime hydration and runtime integrity  
4. activation and commercial truth  
5. provider configuration and secure secret handling  
6. Preflight gating and input feasibility truth  
7. simulation engine boundedness and moderated execution  
8. run-state visibility and event integrity  
9. finalization and report truth  
10. cleanup and janitor truth  
11. updater and distribution hardening  
12. full QA, red-team, and release readiness

#### BSTRAT-014 — Priority Justification Rule
This order exists because downstream features become unreliable or misleading if the system lacks:
- trustworthy readiness gates
- trustworthy lifecycle states
- trustworthy activation and runtime conditions
- trustworthy finalization and cleanup behavior

#### BSTRAT-015 — No Priority Inversion Rule
The program must not prioritize:
- dashboard polish
- secondary convenience features
- speculative future extensibility
- broad configurability

ahead of required lifecycle, trust, and completion foundations.

---

### 2.6 Delivery Model

#### BSTRAT-016 — Delivery Model Rule
The MVP should be delivered through a hybrid model that combines:
- phased delivery
- workstream-based parallelism
- gate-based validation
- AI-agent task packets

#### BSTRAT-017 — Phase-and-Workstream Coupling Rule
Phases define time-ordered execution logic.
Workstreams define ownership lanes.
Neither one alone is sufficient.

#### BSTRAT-018 — Controlled Parallelism Rule
Parallel implementation is allowed only when it does not violate:
- hard dependencies
- layer boundaries
- acceptance traceability
- cleanup and lifecycle truth

#### BSTRAT-019 — Stub-Aware Delivery Rule
Temporary stubs are allowed during intermediate execution if:
- the stub is explicitly labeled
- the affected gate does not falsely count it as complete
- the stub does not create misleading product behavior
- replacement criteria are defined

#### BSTRAT-020 — Vertical Slice Restriction Rule
Vertical slices may be used for validation, but not in a way that bypasses mandatory foundations such as:
- runtime integrity
- activation truth
- Preflight gating
- finalization truth
- cleanup truth

---

### 2.7 Phase Strategy

#### BSTRAT-021 — Phase Delivery Rule
The execution program must be structured into ordered phases rather than an undifferentiated backlog.

#### BSTRAT-022 — Phase Purpose Rule
Each phase exists to establish a new layer of executable truth that later phases can safely build upon.

#### BSTRAT-023 — Phase Completion Rule
A phase is complete only when:
- its scoped deliverables exist
- its exit gate has passed
- its dependencies for the next phase are satisfied
- its incomplete items are explicitly documented
- its truth-bearing obligations are verified

#### BSTRAT-024 — No Premature Phase Closure Rule
A phase must not be closed just because “most code is written” if:
- required gates are unmet
- unresolved blockers remain
- downstream risk would be shifted silently
- acceptance alignment is incomplete

#### BSTRAT-025 — Phase Reopen Rule
A completed phase may be reopened if later discovery proves that:
- its gate was passed incorrectly
- a hidden dependency was missed
- its deliverables were insufficient for downstream truth
- a major contradiction with Document A has emerged

---

### 2.8 Risk-First Delivery Logic

#### BSTRAT-026 — Risk-First Rule
The program should front-load high-leverage/high-risk foundations rather than postponing them to the end.

#### BSTRAT-027 — High-Risk Foundation Classes
The following must be treated as high-risk foundation classes:
- runtime hydration and integrity
- local trust boundary
- activation and device binding
- Preflight feasibility logic
- run-state truth
- finalization and report truth
- cleanup and startup janitor behavior

#### BSTRAT-028 — Late-Risk Compression Prohibition
The strategy must not postpone too many core risks into the final release-preparation window.

#### BSTRAT-029 — Red-Team Early Awareness Rule
Red-team-relevant concerns should influence sequencing early, especially where they affect:
- auth boundaries
- secret handling
- hidden persistence
- cleanup truth
- update/runtime trust
- state desynchronization

---

### 2.9 Validation-Driven Delivery Logic

#### BSTRAT-030 — Validation-Driven Strategy Rule
The program must be validation-driven rather than hope-driven.

#### BSTRAT-031 — Every Major Milestone Must Prove Something Rule
Each major milestone must prove a meaningful system property, such as:
- activation works truthfully
- Preflight gates correctly
- run states remain coherent
- report generation reflects actual completed work
- cleanup is attempted and surfaced honestly

#### BSTRAT-032 — No Narrative Progress Rule
Progress claims must be grounded in verifiable implementation state and exit-gate evidence.

#### BSTRAT-033 — Acceptance-Backed Delivery Rule
The final release path must map back to Document A acceptance criteria rather than informal confidence.

---

### 2.10 Program Milestone Model

#### BSTRAT-034 — Milestone Model Rule
The program should define milestones as proof-bearing checkpoints rather than calendar-only markers.

#### BSTRAT-035 — Milestone Categories
Milestones should include at minimum:
- foundation milestones
- run-path milestones
- output milestones
- cleanup/reliability milestones
- release-readiness milestones

#### BSTRAT-036 — Milestone Meaning Rule
A milestone should mean:
- a bounded scope is complete
- a gate has passed
- downstream work is now safer and more valid

#### BSTRAT-037 — No Cosmetic Milestone Rule
Milestones must not be declared solely on the basis of:
- polished visuals
- temporary happy-path demos
- unverified intermediate behavior

---

### 2.11 AI-Led Execution Strategy

#### BSTRAT-038 — AI-Led Delivery Rule
The delivery strategy must assume that a substantial amount of implementation work will be generated or assisted by AI coding agents.

#### BSTRAT-039 — AI Suitability Rule
The strategy must therefore prefer:
- smaller bounded execution units
- explicit prerequisites
- explicit non-goals
- explicit file/module targets
- explicit acceptance proof requirements
- explicit escalation triggers

#### BSTRAT-040 — Anti-Hallucination Delivery Rule
The delivery strategy must reduce the chance that AI agents:
- invent extra features
- work out of order
- blur layer boundaries
- add hidden persistence
- bypass lifecycle gates
- fake completion

#### BSTRAT-041 — Human-Auditability Rule
Even if AI agents produce most implementation artifacts, the delivery logic must remain human-auditable and reversible.

---

### 2.12 Release Philosophy

#### BSTRAT-042 — Honest Release Philosophy Rule
The release philosophy of the MVP must be:
- truthful
- bounded
- acceptance-backed
- risk-aware
- cleanup-aware
- lifecycle-aware

#### BSTRAT-043 — No “Works Once” Release Rule
A one-time happy-path success is not sufficient for MVP release readiness.

#### BSTRAT-044 — No Hidden-Weakness Release Rule
The program must not suppress known unresolved issues if they materially affect:
- lifecycle truth
- activation truth
- cleanup truth
- acceptance truth
- trust boundary integrity
- report truthfulness

#### BSTRAT-045 — Release Readiness Rule
The MVP may be treated as release-ready only when:
- critical phases are complete
- required gates are passed
- acceptance coverage is adequate
- red-team priorities are evaluated
- no known blocker remains that would contradict Document A truth

---

### 2.13 Program-Level Non-Goals

#### BSTRAT-046 — Strategy Non-Goal Rule
The purpose of this strategy is not to maximize:
- framework elegance
- future extensibility at all costs
- optional configurability
- parallelization for its own sake
- advanced productization beyond MVP

#### BSTRAT-047 — No Premature Enterprise Preparation Rule
The delivery strategy must not quietly optimize for enterprise-scale deployment if that adds ambiguity or complexity to MVP execution.

#### BSTRAT-048 — No Premature Future-Proofing Rule
The program must not incur major complexity merely to “prepare for later possibilities” unless the complexity is directly justified by MVP requirements.

---

### 2.14 Strategic Integrity Rules

#### BSTRAT-049 — Strategy Integrity Rule
The delivery strategy must remain consistent with:
- Document A constitutional constraints
- Document A state-machine truth
- Document A architecture boundaries
- Document A data-policy constraints
- Document A acceptance criteria

#### BSTRAT-050 — No Strategy-Layer Scope Mutation Rule
No phase, milestone, or workstream prioritization may be used to mutate the locked MVP into a materially different product.

#### BSTRAT-051 — No Hidden Shortcut Rule
The strategy must not normalize shortcuts that would later require rewriting foundational truth-bearing layers.

---

### 2.15 Boundaries of This Section

#### BSTRAT-052 — Section Boundary Rule
This section defines:
- the macro delivery philosophy
- strategic priorities
- delivery model
- milestone logic
- release philosophy
- AI-led execution strategy

This section does not yet define:
- the full workstream map
- the detailed dependency graph
- the detailed phase list
- the phase-by-phase task plans
- the task packet template
- the final validation matrix

Those are defined in later sections.

---

## 3. Workstream Architecture

### 3.1 Section Purpose
This section defines the stable execution lanes of the program.

This section exists to ensure that:
- implementation is grouped by responsibility instead of random task clustering
- work can be parallelized safely where appropriate
- AI coding agents can be constrained to clear execution lanes
- ownership boundaries remain consistent with Document A architecture and system truth

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Execution Constitution
- Section 2 — Program Delivery Strategy
- Document A architecture, behavior, and data-policy boundaries

---

### 3.2 Role of Workstreams

#### BWORK-001 — Workstream Role Rule
A workstream is a stable execution lane grouping related implementation responsibilities.

#### BWORK-002 — Workstream Purpose Rule
Workstreams exist to organize implementation by:
- responsibility domain
- dependency proximity
- validation relationship
- ownership clarity

#### BWORK-003 — No Backlog-Pile Rule
Document B must not degrade into one flat task list.
Workstreams are mandatory because they preserve clarity and anti-drift discipline.

#### BWORK-004 — Workstream Non-Authority Rule
A workstream is an execution grouping, not a source of new product truth.

---

### 3.3 Workstream Map

#### BWORK-005 — Canonical Workstream Set
The canonical workstream set for MVP execution is:

1. WS-01 — Governance and Spec Traceability  
2. WS-02 — Desktop Shell and Runtime Management  
3. WS-03 — Licensing, Activation, and Commercial Backend  
4. WS-04 — Provider Configuration and Secret Handling  
5. WS-05 — Seed Intake and Preflight  
6. WS-06 — Simulation Engine and Persona System  
7. WS-07 — Run State, Events, and Live Monitoring  
8. WS-08 — Finalization, Report Generation, and Export  
9. WS-09 — Cleanup, Shutdown, and Janitor  
10. WS-10 — Updater, Distribution, and Runtime Delivery  
11. WS-11 — QA, Acceptance, Red-Team, and Regression  
12. WS-12 — AI-Agent Task Packaging and Prompt Control

#### BWORK-006 — Stable Workstream Rule
These workstreams should remain stable unless a later change-log revision explicitly restructures them.

#### BWORK-007 — No Casual Extra Workstream Rule
New workstreams must not be added casually merely because a task feels unique.
Execution should prefer stable lanes unless a real structural need exists.

---

### 3.4 WS-01 — Governance and Spec Traceability

#### BWORK-008 — WS-01 Purpose
This workstream governs execution traceability and documentation discipline.

#### BWORK-009 — WS-01 Responsibilities
WS-01 owns:
- traceability from Document B tasks back to Document A
- rule-reference discipline
- anti-drift review logic
- execution-plan integrity checks
- document alignment between A, B, and C
- spec-governance support during execution

#### BWORK-010 — WS-01 Non-Ownership Rule
WS-01 does not own product behavior, code architecture, or direct runtime implementation.
It owns governance discipline.

---

### 3.5 WS-02 — Desktop Shell and Runtime Management

#### BWORK-011 — WS-02 Purpose
This workstream governs privileged local orchestration foundations.

#### BWORK-012 — WS-02 Responsibilities
WS-02 owns:
- shell process structure
- app lifecycle wiring
- startup checks
- runtime hydration flow
- runtime integrity validation
- process supervision
- local app-session trust boundary
- controlled shutdown coordination

#### BWORK-013 — WS-02 Dependency Importance
WS-02 is a foundational workstream because many later workstreams depend on:
- runtime availability
- shell supervision
- correct local orchestration
- correct shutdown control

#### BWORK-014 — WS-02 Non-Ownership Rule
WS-02 does not own simulation logic, report reasoning, or commercial ledger truth.

---

### 3.6 WS-03 — Licensing, Activation, and Commercial Backend

#### BWORK-015 — WS-03 Purpose
This workstream governs commercial entitlement and device-bound activation behavior.

#### BWORK-016 — WS-03 Responsibilities
WS-03 owns:
- license truth handling
- device activation flow
- device-bound JWT issuance/validation support
- two-device slot enforcement
- activation denial/error classification
- commercial backend record discipline
- payment-event-to-entitlement translation support

#### BWORK-017 — WS-03 Dependency Importance
WS-03 is foundational because valid licensed use depends on it.

#### BWORK-018 — WS-03 Non-Ownership Rule
WS-03 does not own the core simulation engine, transcript generation, or local PDF reporting logic.

---

### 3.7 WS-04 — Provider Configuration and Secret Handling

#### BWORK-019 — WS-04 Purpose
This workstream governs provider readiness and secure API-key handling.

#### BWORK-020 — WS-04 Responsibilities
WS-04 owns:
- provider selection UX support
- provider-key entry flow
- OS-native secure storage integration
- configuration readiness state
- key retrieval mediation through allowed local layers
- anti-plaintext handling discipline

#### BWORK-021 — WS-04 Dependency Importance
WS-04 is a hard prerequisite for valid simulation execution.

#### BWORK-022 — WS-04 Non-Ownership Rule
WS-04 does not own provider inference semantics, persona behavior, or Preflight feasibility logic itself, though it feeds them.

---

### 3.8 WS-05 — Seed Intake and Preflight

#### BWORK-023 — WS-05 Purpose
This workstream governs intake validation and run feasibility determination.

#### BWORK-024 — WS-05 Responsibilities
WS-05 owns:
- file intake logic
- supported-type enforcement
- file-size enforcement
- extractable-text checks
- English-only detection/rejection
- provider/model readiness checks during Preflight
- estimated budget/time feasibility calculation
- downgrade/reject decisioning
- effective-plan derivation

#### BWORK-025 — WS-05 Dependency Importance
WS-05 is a hard gate before simulation start.

#### BWORK-026 — WS-05 Non-Ownership Rule
WS-05 does not own the full simulation round system after the run begins.
It owns the pre-run gate.

---

### 3.9 WS-06 — Simulation Engine and Persona System

#### BWORK-027 — WS-06 Purpose
This workstream governs the intelligence core of the run.

#### BWORK-028 — WS-06 Responsibilities
WS-06 owns:
- document ingestion for run execution
- runtime persona generation
- Moderator logic
- round orchestration
- bounded memory strategy
- argument/risk memory maintenance
- token/time tracking within run execution
- controlled multi-round debate behavior

#### BWORK-029 — WS-06 Dependency Importance
WS-06 is the central run-execution workstream but depends on earlier truth-bearing gates.

#### BWORK-030 — WS-06 Non-Ownership Rule
WS-06 does not own shell-level process control, OS-native secure storage, or commercial activation truth.

---

### 3.10 WS-07 — Run State, Events, and Live Monitoring

#### BWORK-031 — WS-07 Purpose
This workstream governs user-visible run truth during execution.

#### BWORK-032 — WS-07 Responsibilities
WS-07 owns:
- run/session lifecycle wiring
- status surface integration
- event stream sequencing
- live transcript/event feed integrity
- interruption-state visibility
- cancelability flow integration
- UI-facing live monitoring truth

#### BWORK-033 — WS-07 Dependency Importance
WS-07 is essential for making the run legible and truthful to the user.

#### BWORK-034 — WS-07 Non-Ownership Rule
WS-07 does not decide product truth on its own.
It expresses already-governed truth in live execution form.

---

### 3.11 WS-08 — Finalization, Report Generation, and Export

#### BWORK-035 — WS-08 Purpose
This workstream governs the truth-bearing output path.

#### BWORK-036 — WS-08 Responsibilities
WS-08 owns:
- finalization transition logic
- synthesis inputs for final report content
- viability score generation pathway
- report section assembly
- PDF generation
- report-availability truth
- local report access/export behavior

#### BWORK-037 — WS-08 Dependency Importance
WS-08 is required before any completion claim can be treated as valid.

#### BWORK-038 — WS-08 Non-Ownership Rule
WS-08 does not own run-start gating, activation, or runtime hydration.

---

### 3.12 WS-09 — Cleanup, Shutdown, and Janitor

#### BWORK-039 — WS-09 Purpose
This workstream governs truthful post-run termination handling.

#### BWORK-040 — WS-09 Responsibilities
WS-09 owns:
- cleanup triggering
- cleanup attempt flow
- cleanup outcome truth
- normal shutdown cleanup coordination
- abnormal termination follow-up logic
- startup janitor behavior
- anti-zombie supervision integration where relevant

#### BWORK-041 — WS-09 Dependency Importance
WS-09 is essential for truthful lifecycle closure and release readiness.

#### BWORK-042 — WS-09 Non-Ownership Rule
WS-09 does not redefine run success.
It governs post-run cleanup truth.

---

### 3.13 WS-10 — Updater, Distribution, and Runtime Delivery

#### BWORK-043 — WS-10 Purpose
This workstream governs artifact delivery and trust verification outside the core run loop.

#### BWORK-044 — WS-10 Responsibilities
WS-10 owns:
- runtime download/distribution support
- updater flow
- artifact trust and verification behavior
- version/update availability wiring
- distribution-hardening concerns

#### BWORK-045 — WS-10 Dependency Importance
WS-10 is not the first core simulation dependency, but it is necessary for robust install/update/runtime lifecycle integrity.

#### BWORK-046 — WS-10 Non-Ownership Rule
WS-10 does not own simulation behavior or report semantics.

---

### 3.14 WS-11 — QA, Acceptance, Red-Team, and Regression

#### BWORK-047 — WS-11 Purpose
This workstream governs proof that the system satisfies upstream truth.

#### BWORK-048 — WS-11 Responsibilities
WS-11 owns:
- acceptance mapping
- test planning alignment
- regression discipline
- red-team scenario execution
- phase-gate evidence collection
- release-readiness verification support

#### BWORK-049 — WS-11 Dependency Importance
WS-11 is cross-cutting and must be involved throughout execution, not only at the end.

#### BWORK-050 — WS-11 Non-Ownership Rule
WS-11 validates implementation truth.
It does not define product truth.

---

### 3.15 WS-12 — AI-Agent Task Packaging and Prompt Control

#### BWORK-051 — WS-12 Purpose
This workstream governs how AI coding agents are instructed, constrained, and audited during execution.

#### BWORK-052 — WS-12 Responsibilities
WS-12 owns:
- task packet packaging discipline
- AI prompting structure
- implementation guardrail reinforcement
- anti-scope-drift instructions
- task-level non-goal enforcement
- execution context packaging for AI agents

#### BWORK-053 — WS-12 Dependency Importance
WS-12 is critical because the project relies heavily on AI-led development.

#### BWORK-054 — WS-12 Non-Ownership Rule
WS-12 does not own the product.
It owns safe execution packaging for AI-assisted implementation.

---

### 3.16 Workstream Ownership Rules

#### BWORK-055 — One Primary Owner Rule
Each execution concern must have one primary workstream owner even when supporting coordination from other workstreams is required.

#### BWORK-056 — Cross-Workstream Collaboration Rule
Cross-workstream coordination is expected, but ownership must remain explicit.

#### BWORK-057 — No Ownership Ambiguity Rule
If a task fits multiple workstreams, Document B must assign one primary workstream and treat the others as dependencies or collaborators.

#### BWORK-058 — No Layer-Bleed Rule
Workstream boundaries must not be used to justify layer-boundary violations such as:
- shell responsibilities being implemented in the frontend
- commercial logic being hidden in the simulation engine
- cleanup truth being silently absorbed by general app-close logic

---

### 3.17 Cross-Workstream Dependency Rules

#### BWORK-059 — Dependency-Aware Workstream Rule
Workstreams must be allowed to depend on one another explicitly.

#### BWORK-060 — Upstream Dependency Examples
At minimum, the following dependency truths must be preserved:

- WS-02 is upstream of most local execution work  
- WS-03 is upstream of valid licensed use  
- WS-04 is upstream of provider-dependent run readiness  
- WS-05 is upstream of WS-06 run execution  
- WS-06 feeds WS-07 and WS-08  
- WS-08 depends on meaningful WS-06 output  
- WS-09 depends on terminal run paths from WS-06/WS-07/WS-08  
- WS-11 cuts across all workstreams  
- WS-12 supports all AI-executed workstreams

#### BWORK-061 — No False Independence Rule
No workstream may be treated as independent if its truth depends on another workstream’s completion.

---

### 3.18 Workstream Deliverable Discipline

#### BWORK-062 — Deliverable Rule
Each workstream must eventually define:
- its scoped deliverables
- its phase participation
- its entry dependencies
- its exit proof

#### BWORK-063 — No Infinite Workstream Rule
A workstream must not become a dumping ground for unrelated tasks.
Its boundaries must stay stable and auditable.

#### BWORK-064 — Deliverable Truth Rule
A workstream deliverable must correspond to a real implementation capability or verified control, not just planning prose.

---

### 3.19 Workstream Integrity Rules

#### BWORK-065 — Architecture Alignment Rule
Workstreams must remain aligned with Document A’s architecture responsibilities.

#### BWORK-066 — Data Policy Alignment Rule
No workstream may define or implement storage behavior that violates Document A’s data lifecycle policy.

#### BWORK-067 — Acceptance Alignment Rule
Each workstream must ultimately support one or more Document A acceptance criteria.

#### BWORK-068 — No Future-Feature Smuggling Rule
Workstreams must not be used to quietly introduce future-scope items under the label of “adjacent implementation.”

---

### 3.20 Boundaries of This Section

#### BWORK-069 — Section Boundary Rule
This section defines:
- the canonical workstream map
- the purpose of each workstream
- ownership boundaries
- cross-workstream dependency principles
- workstream integrity rules

This section does not yet define:
- the full detailed dependency graph
- exact task packets per workstream
- phase-by-phase sequencing inside each workstream
- final validation gates per workstream

Those are defined in later sections.

---

### 3.21 Final Workstream Summary

#### BWORK-070 — Workstream Summary Rule
At the workstream level, Document B organizes MVP execution into stable lanes that preserve governance, architectural separation, lifecycle truth, cleanup truth, and AI-agent execution safety.

#### BWORK-071 — Stability Rule
All later sections of Document B must remain consistent with the workstream architecture defined here unless explicitly revised through the Document B Change Log.

---

## 4. Global Dependency Model

### 4.1 Section Purpose
This section defines the global dependency logic that governs all implementation order in the MVP.

This section exists to ensure that:
- execution happens in a dependency-safe order
- AI coding agents do not build downstream behavior on missing foundations
- workstreams can coordinate without creating hidden sequencing errors
- phase readiness is based on real prerequisite satisfaction rather than intuition

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Execution Constitution
- Section 2 — Program Delivery Strategy
- Section 3 — Workstream Architecture
- Document A — Master Project Creative Brief

If any dependency rule described here conflicts with Document A, Document A wins.

---

### 4.2 Dependency Role of This Section

#### BDEP-001 — Dependency Model Rule
This section defines the global logic of what must exist before later work can be treated as validly executable or complete.

#### BDEP-002 — No Arbitrary Ordering Rule
Implementation order must not be treated as arbitrary.
The program has hard dependencies that must be respected.

#### BDEP-003 — Dependency Before Parallelism Rule
Before work is parallelized, its dependency relationships must be understood and recorded.

#### BDEP-004 — No Backward Completion Rule
A downstream capability must not be marked complete if its upstream truth-bearing dependency has not yet been satisfied.

---

### 4.3 Core Dependency Principles

#### BDEP-005 — Hard Dependency Rule
A hard dependency is a prerequisite that must exist before a downstream execution item can be treated as real, valid, or complete.

#### BDEP-006 — Soft Dependency Rule
A soft dependency is a sequencing preference or efficiency aid that improves delivery quality but is not strictly required for logical possibility.

#### BDEP-007 — Upstream Truth Rule
If a downstream implementation depends on upstream truth, the upstream layer must be treated as authoritative and complete enough before downstream closure.

#### BDEP-008 — No Hidden Dependency Rule
Dependencies must be made explicit in Document B rather than discovered only after downstream work begins failing.

#### BDEP-009 — Dependency Traceability Rule
Every major dependency relationship should be explainable in terms of one or more Document A rules or constraints.

#### BDEP-010 — No Dependency Fiction Rule
The plan must not pretend a dependency is satisfied merely because:
- a mock exists
- a UI path exists
- a placeholder value exists
- an unverified demo exists

---

### 4.4 Dependency Categories

#### BDEP-011 — Dependency Category Rule
Global dependencies in Document B must be understood through the following categories:

1. constitutional dependencies  
2. lifecycle dependencies  
3. architecture dependencies  
4. data-policy dependencies  
5. interface dependencies  
6. verification dependencies

#### BDEP-012 — Constitutional Dependency Definition
A constitutional dependency exists when a downstream build decision would violate Document A’s locked project truths if the dependency were ignored.

#### BDEP-013 — Lifecycle Dependency Definition
A lifecycle dependency exists when a downstream behavior depends on correct state handling, state transitions, or terminal-path truth.

#### BDEP-014 — Architecture Dependency Definition
An architecture dependency exists when a downstream component depends on an upstream ownership boundary or service boundary being implemented correctly.

#### BDEP-015 — Data-Policy Dependency Definition
A data-policy dependency exists when a downstream capability depends on correct storage, persistence, secret handling, or cleanup boundaries.

#### BDEP-016 — Interface Dependency Definition
An interface dependency exists when a downstream feature depends on routes, payloads, sequencing, or local trust mechanisms defined upstream.

#### BDEP-017 — Verification Dependency Definition
A verification dependency exists when a task, phase, or release gate cannot be meaningfully signed off until another implementation area has been validated.

---

### 4.5 Global Hard Dependency Truths

The following dependency truths are constitutionally important and must be preserved across all planning and implementation.

#### BDEP-018 — Runtime Dependency Rule
No simulation-capable flow may be treated as complete before runtime hydration and runtime integrity are implemented sufficiently for trustworthy execution.

#### BDEP-019 — Activation Dependency Rule
No licensed run flow may be treated as complete before activation truth exists.

#### BDEP-020 — Provider Configuration Dependency Rule
No provider-dependent run behavior may be treated as complete before provider configuration and secure key handling exist.

#### BDEP-021 — Preflight Dependency Rule
No run-start behavior may be treated as complete before Preflight gating exists and is actually enforced.

#### BDEP-022 — Effective Plan Dependency Rule
No execution-start path may be treated as truthful before requested-plan versus effective-plan logic exists.

#### BDEP-023 — Run-State Dependency Rule
No live monitoring path may be treated as valid before session state and event sequencing are coherent.

#### BDEP-024 — Finalization Dependency Rule
No run completion claim may be treated as valid before finalization logic exists.

#### BDEP-025 — Report Dependency Rule
No successful run path may be treated as valid before report generation and report-availability truth exist.

#### BDEP-026 — Cleanup Dependency Rule
No release-readiness decision may be treated as valid before cleanup, cleanup truth, and janitor behavior are implemented sufficiently.

#### BDEP-027 — Acceptance Dependency Rule
No MVP-complete claim may be treated as valid before acceptance-aligned validation has been executed.

---

### 4.6 Specific Dependency Chains

#### BDEP-028 — Foundation Chain Rule
The first major dependency chain of the product is:

app shell and startup control  
-> runtime hydration and integrity  
-> activation and entitlement readiness  
-> provider configuration and secure storage  
-> Preflight gating  
-> run start eligibility

#### BDEP-029 — Run Execution Chain Rule
The second major dependency chain of the product is:

seed intake and Preflight  
-> run/session creation  
-> ingestion  
-> persona generation  
-> moderated round execution  
-> event emission and live status truth  
-> interruption/cancellation handling

#### BDEP-030 — Output Chain Rule
The third major dependency chain of the product is:

completed run material  
-> finalization  
-> report assembly  
-> PDF generation  
-> report availability truth  
-> post-run access flow

#### BDEP-031 — Post-Run Integrity Chain Rule
The fourth major dependency chain of the product is:

terminal run path  
-> cleanup trigger  
-> cleanup attempt  
-> cleanup outcome truth  
-> startup janitor handling after abnormal termination

#### BDEP-032 — Distribution Chain Rule
The fifth major dependency chain of the product is:

commercial entitlement and version support  
-> runtime distribution authorization  
-> runtime/update artifact retrieval  
-> artifact verification  
-> safe install/apply behavior

---

### 4.7 Workstream Dependency Matrix Logic

#### BDEP-033 — Workstream Dependency Rule
Workstream execution must reflect the actual dependency graph rather than organizational convenience.

#### BDEP-034 — WS-02 Upstream Rule
WS-02 — Desktop Shell and Runtime Management is upstream of most local execution behavior because it establishes:
- app supervision
- runtime presence
- integrity checking
- trusted local orchestration
- controlled shutdown capability

#### BDEP-035 — WS-03 Upstream Rule
WS-03 — Licensing, Activation, and Commercial Backend is upstream of valid licensed use and therefore upstream of truthful run eligibility.

#### BDEP-036 — WS-04 Upstream Rule
WS-04 — Provider Configuration and Secret Handling is upstream of provider-dependent readiness.

#### BDEP-037 — WS-05 Gate Rule
WS-05 — Seed Intake and Preflight is the primary gatekeeper workstream before run execution.

#### BDEP-038 — WS-06 Core Execution Rule
WS-06 — Simulation Engine and Persona System is central for active run behavior, but it is not upstream of everything; it depends on earlier readiness and gating layers.

#### BDEP-039 — WS-07 Expression Rule
WS-07 — Run State, Events, and Live Monitoring depends on coherent lifecycle and engine output.

#### BDEP-040 — WS-08 Output Rule
WS-08 — Finalization, Report Generation, and Export depends on meaningful run output from WS-06 and truthful state handling from WS-07.

#### BDEP-041 — WS-09 Post-Run Rule
WS-09 — Cleanup, Shutdown, and Janitor depends on terminal-path truth from active execution and app-shell control from WS-02.

#### BDEP-042 — WS-10 Distribution Rule
WS-10 — Updater, Distribution, and Runtime Delivery depends on commercial and trust foundations but is partly independent of the core run path once basic product truth is stable.

#### BDEP-043 — WS-11 Cross-Cutting Rule
WS-11 — QA, Acceptance, Red-Team, and Regression is cross-cutting and must observe upstream work throughout the program rather than only after all coding is “done.”

#### BDEP-044 — WS-12 Cross-Cutting Rule
WS-12 — AI-Agent Task Packaging and Prompt Control is cross-cutting and supports all implementation workstreams by constraining execution packets and anti-drift behavior.

---

### 4.8 Entry and Exit Dependency Logic

#### BDEP-045 — Entry Dependency Rule
A phase or major task may begin only if its entry dependencies are explicitly satisfied or explicitly stubbed under approved stub rules.

#### BDEP-046 — Exit Dependency Rule
A phase or major task may close only if its deliverables satisfy the dependencies required by its downstream consumers.

#### BDEP-047 — Downstream Safety Rule
Exit from a phase must mean that the next phase can build on its outputs without silently compensating for missing foundational truth.

#### BDEP-048 — No Faux Exit Rule
A phase must not exit on the basis of incomplete upstream truth masked by local workarounds.

---

### 4.9 Allowed Parallelism

#### BDEP-049 — Controlled Parallelism Rule
Parallel work is allowed only when:
- hard dependencies are not violated
- ownership remains clear
- mock or stub boundaries are explicit
- downstream gates do not mistake incomplete work for complete work

#### BDEP-050 — Parallelizable Work Rule
The following kinds of work may often proceed in parallel when coordinated correctly:
- UI scaffolding with backend contract drafting
- commercial backend record design with shell scaffolding
- report-template scaffolding with finalization contract design
- QA planning with implementation planning

#### BDEP-051 — Non-Parallelizable Truth Rule
The following categories must not be treated as independently completable when their dependencies are absent:
- truthful run start
- truthful run completion
- truthful cleanup
- truthful report readiness
- truthful release readiness

#### BDEP-052 — Parallelism Documentation Rule
If work is intentionally parallelized, the dependency assumptions must be recorded in the relevant phase or task plan.

---

### 4.10 Stub and Mock Policy

#### BDEP-053 — Stub Policy Rule
Stubs and mocks are allowed for sequencing and scaffolding, but they must never be mistaken for production-complete truth-bearing behavior.

#### BDEP-054 — Stub Disclosure Rule
Every approved stub or mock must specify:
- what is stubbed
- which real dependency it stands in for
- what downstream activities may rely on it
- which gate requires its replacement

#### BDEP-055 — No Hidden Stub Rule
A stub must not survive into a completion claim if the relevant phase or workstream requires real implementation.

#### BDEP-056 — Mock Safety Rule
A mock is acceptable only if its use does not create false product claims, false acceptance proof, or hidden architecture drift.

---

### 4.11 Dependency Escalation Rules

#### BDEP-057 — Missing Dependency Escalation Rule
If an expected dependency is missing, the issue must be surfaced before downstream work is declared complete.

#### BDEP-058 — Dependency Conflict Escalation Rule
If two execution paths imply contradictory dependency orders, the conflict must be escalated and resolved explicitly rather than quietly bypassed.

#### BDEP-059 — Document A Escalation Rule
If satisfying a dependency appears to require changing Document A truth, the issue must be escalated to Document A governance rather than solved silently inside Document B.

#### BDEP-060 — Deferred Item Escalation Rule
If a dependency seems to require a deferred feature, the default response is escalation, not automatic scope promotion.

---

### 4.12 Dependency Integrity Prohibitions

#### BDEP-061 — No Out-of-Order Completion Rule
A downstream capability must not be signed off before its upstream hard dependencies are sufficiently implemented.

#### BDEP-062 — No Dependency Washing Rule
A missing dependency must not be hidden under labels such as:
- temporary workaround
- MVP shortcut
- later cleanup
if that workaround changes product truth or gate truth

#### BDEP-063 — No Dependency Blindness Rule
AI coding agents must not be allowed to execute work as if no dependency model exists.

#### BDEP-064 — No Release Shortcut Rule
Release readiness must not be declared by skipping dependency-grounded verification.

---

### 4.13 Boundaries of This Section

#### BDEP-065 — Section Boundary Rule
This section defines:
- dependency principles
- dependency categories
- global hard dependency truths
- workstream dependency logic
- entry/exit dependency rules
- parallelism rules
- stub/mock rules
- escalation rules

This section does not yet define:
- the ordered phase list in full
- the detailed contents of each phase
- the task packet format
- the full validation matrix

Those are defined in later sections.

---

### 4.14 Final Dependency Summary

#### BDEP-066 — Dependency Summary Rule
At the global level, the program is governed by a dependency-first model in which runtime and shell truth precede activation-dependent readiness, activation and provider readiness precede Preflight, Preflight precedes truthful run start, run execution precedes live event truth and finalization, finalization precedes report truth, terminal run truth precedes cleanup truth, and all of the above precede release readiness.

#### BDEP-067 — Stability Rule
All later sections of Document B must remain consistent with this dependency model unless explicitly revised through the Document B Change Log.

---

## 5. Phase Map

### 5.1 Section Purpose
This section defines the ordered execution phases of the MVP.

This section exists to ensure that:
- work is delivered in deterministic stages
- each stage establishes a meaningful new layer of system truth
- AI coding agents are not operating from an unbounded or flat task backlog
- program progress can be evaluated through phase gates rather than intuition

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Execution Constitution
- Section 2 — Program Delivery Strategy
- Section 4 — Global Dependency Model
- Document A — Master Project Creative Brief

---

### 5.2 Role of the Phase Map

#### BPHASE-001 — Phase Map Rule
The phase map defines the canonical ordered stages of execution for the MVP.

#### BPHASE-002 — Phase Purpose Rule
Each phase must establish a meaningful new foundation that later phases can safely depend upon.

#### BPHASE-003 — No Flat Backlog Rule
The program must not be executed as a flat undifferentiated task list once the phase map is frozen.

#### BPHASE-004 — Ordered Program Rule
Phases define program order.
Workstreams define execution lanes inside that order.

---

### 5.3 Canonical Phase List

#### BPHASE-005 — Canonical Phase Set
The canonical MVP execution phases are:

1. Phase 1 — Governance, Repository Foundation, and Execution Scaffolding  
2. Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary  
3. Phase 3 — Licensing, Activation, and Commercial Backend  
4. Phase 4 — Provider Configuration and Secure Local Setup  
5. Phase 5 — Seed Intake and Preflight  
6. Phase 6 — Simulation Engine Core  
7. Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling  
8. Phase 8 — Finalization, Report Generation, and Report Access  
9. Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening  
10. Phase 10 — Updater, Runtime Delivery, and Distribution Hardening  
11. Phase 11 — QA Matrix Execution, Red-Team Review, and Release Readiness

#### BPHASE-006 — Stable Phase Rule
This phase set should remain stable unless a future revision explicitly changes the structure through the Document B Change Log.

#### BPHASE-007 — No Casual Phase Addition Rule
New phases must not be inserted casually unless a real program-control need exists.

---

### 5.4 Phase 1 — Governance, Repository Foundation, and Execution Scaffolding

#### BPHASE-008 — Phase 1 Purpose
Phase 1 establishes the execution-control foundations of the project.

#### BPHASE-009 — Phase 1 Core Outcomes
Phase 1 must establish at minimum:
- repository structure
- documentation structure
- traceability discipline
- implementation boundaries
- execution packet conventions
- baseline project scaffolding sufficient for safe later work

#### BPHASE-010 — Phase 1 Workstreams
Phase 1 primarily involves:
- WS-01 — Governance and Spec Traceability
- WS-12 — AI-Agent Task Packaging and Prompt Control
with support from other workstreams where repository scaffolding is needed

#### BPHASE-011 — Phase 1 Non-Goals
Phase 1 is not intended to complete:
- real run execution
- activation flow
- Preflight
- report generation
- cleanup hardening

#### BPHASE-012 — Phase 1 Exit Intent
Phase 1 should end only when later phases can execute against a stable governance and scaffolding base.

---

### 5.5 Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

#### BPHASE-013 — Phase 2 Purpose
Phase 2 establishes the privileged local execution foundation of the product.

#### BPHASE-014 — Phase 2 Core Outcomes
Phase 2 must establish at minimum:
- shell lifecycle control
- startup checks
- runtime hydration flow
- runtime integrity verification
- local app-session trust boundary
- shell supervision of the engine process
- controlled app-close orchestration foundation

#### BPHASE-015 — Phase 2 Workstreams
Phase 2 primarily involves:
- WS-02 — Desktop Shell and Runtime Management
with support from WS-10 where runtime delivery is implicated

#### BPHASE-016 — Phase 2 Non-Goals
Phase 2 is not intended to complete:
- commercial activation truth
- provider configuration
- Preflight
- live debate logic
- final report behavior

#### BPHASE-017 — Phase 2 Exit Intent
Phase 2 should end only when the app can honestly determine runtime readiness and supervise local execution foundations.

---

### 5.6 Phase 3 — Licensing, Activation, and Commercial Backend

#### BPHASE-018 — Phase 3 Purpose
Phase 3 establishes commercial truth and valid device-bound use.

#### BPHASE-019 — Phase 3 Core Outcomes
Phase 3 must establish at minimum:
- commercial entitlement handling
- license state truth
- device activation flow
- two-device enforcement
- JWT issuance/validation flow
- denial/error distinction
- local activation persistence sufficient for offline post-activation validation

#### BPHASE-020 — Phase 3 Workstreams
Phase 3 primarily involves:
- WS-03 — Licensing, Activation, and Commercial Backend
with support from WS-02 for local secure handling and app wiring

#### BPHASE-021 — Phase 3 Non-Goals
Phase 3 is not intended to complete:
- provider readiness
- seed validation
- run execution
- finalization
- cleanup hardening

#### BPHASE-022 — Phase 3 Exit Intent
Phase 3 should end only when the app can truthfully distinguish activated versus unactivated licensed use.

---

### 5.7 Phase 4 — Provider Configuration and Secure Local Setup

#### BPHASE-023 — Phase 4 Purpose
Phase 4 establishes provider-dependent readiness foundations.

#### BPHASE-024 — Phase 4 Core Outcomes
Phase 4 must establish at minimum:
- provider selection flow
- API-key entry flow
- secure OS-native storage handling
- configuration readiness truth
- retrieval mediation for later use by allowed local layers

#### BPHASE-025 — Phase 4 Workstreams
Phase 4 primarily involves:
- WS-04 — Provider Configuration and Secret Handling
with support from WS-02 where shell-secure-storage mediation is involved

#### BPHASE-026 — Phase 4 Non-Goals
Phase 4 is not intended to prove:
- provider/model feasibility under run conditions
- seed-document acceptance
- simulation correctness
- report generation truth

#### BPHASE-027 — Phase 4 Exit Intent
Phase 4 should end only when provider configuration can be stored and surfaced truthfully, without pretending that storage alone means run readiness.

---

### 5.8 Phase 5 — Seed Intake and Preflight

#### BPHASE-028 — Phase 5 Purpose
Phase 5 establishes run eligibility truth.

#### BPHASE-029 — Phase 5 Core Outcomes
Phase 5 must establish at minimum:
- seed-document intake
- supported-type and size enforcement
- extractable-text checks
- English-only enforcement
- provider/model readiness checks during Preflight
- feasibility planning
- requested-versus-effective plan logic
- downgrade/reject behavior
- run-start gating based on eligible Preflight outcomes

#### BPHASE-030 — Phase 5 Workstreams
Phase 5 primarily involves:
- WS-05 — Seed Intake and Preflight
with support from WS-02, WS-03, and WS-04 where readiness dependencies are required

#### BPHASE-031 — Phase 5 Non-Goals
Phase 5 is not intended to complete:
- full simulation rounds
- live monitoring fidelity
- final report generation
- cleanup hardening

#### BPHASE-032 — Phase 5 Exit Intent
Phase 5 should end only when the system can truthfully decide whether a run may start and under what effective plan.

---

### 5.9 Phase 6 — Simulation Engine Core

#### BPHASE-033 — Phase 6 Purpose
Phase 6 establishes the core moderated run behavior of the product.

#### BPHASE-034 — Phase 6 Core Outcomes
Phase 6 must establish at minimum:
- run/session creation
- workspace allocation
- seed ingestion for execution
- runtime persona generation
- Moderator behavior
- moderated round execution
- bounded memory strategy
- argument/risk continuity
- token/time tracking within the run

#### BPHASE-035 — Phase 6 Workstreams
Phase 6 primarily involves:
- WS-06 — Simulation Engine and Persona System
with support from WS-02 and WS-05 where dependencies remain active

#### BPHASE-036 — Phase 6 Non-Goals
Phase 6 is not intended to complete:
- full user-visible live monitoring fidelity
- final report truth
- cleanup end-to-end hardening
- release readiness

#### BPHASE-037 — Phase 6 Exit Intent
Phase 6 should end only when the engine can perform a bounded moderated run path in principle under the approved model.

---

### 5.10 Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

#### BPHASE-038 — Phase 7 Purpose
Phase 7 establishes truthful visibility into the active run.

#### BPHASE-039 — Phase 7 Core Outcomes
Phase 7 must establish at minimum:
- coherent run/session state wiring
- truthful status reporting
- ordered event emission and retrieval
- live transcript/event visibility
- interruption-state handling
- paused/degraded truth
- cancelability wiring
- status/event semantic consistency

#### BPHASE-040 — Phase 7 Workstreams
Phase 7 primarily involves:
- WS-07 — Run State, Events, and Live Monitoring
with support from WS-06 and WS-02

#### BPHASE-041 — Phase 7 Non-Goals
Phase 7 is not intended to complete:
- finalization/report truth
- cleanup hardening
- updater hardening
- final release signoff

#### BPHASE-042 — Phase 7 Exit Intent
Phase 7 should end only when active runs are legible and lifecycle truth is visible to the user during execution.

---

### 5.11 Phase 8 — Finalization, Report Generation, and Report Access

#### BPHASE-043 — Phase 8 Purpose
Phase 8 establishes the truthful output path of the product.

#### BPHASE-044 — Phase 8 Core Outcomes
Phase 8 must establish at minimum:
- finalization entry handling
- budget-capped and constrained-completion truth
- viability score generation path
- report assembly
- PDF generation
- report-availability truth
- local report access behavior
- distinction between report availability and workspace cleanup

#### BPHASE-045 — Phase 8 Workstreams
Phase 8 primarily involves:
- WS-08 — Finalization, Report Generation, and Export
with support from WS-06 and WS-07

#### BPHASE-046 — Phase 8 Non-Goals
Phase 8 is not intended to complete:
- full cleanup hardening
- updater hardening
- full release-readiness signoff

#### BPHASE-047 — Phase 8 Exit Intent
Phase 8 should end only when successful and constrained-completion runs can produce truthful reports.

---

### 5.12 Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening

#### BPHASE-048 — Phase 9 Purpose
Phase 9 establishes truthful run closure and abnormal-termination handling.

#### BPHASE-049 — Phase 9 Core Outcomes
Phase 9 must establish at minimum:
- cleanup triggering
- cleanup attempt behavior
- cleanup-result truth
- controlled shutdown coordination
- anti-zombie behavior where required
- startup janitor discovery and purge of abandoned workspaces
- explicit no-resume preservation

#### BPHASE-050 — Phase 9 Workstreams
Phase 9 primarily involves:
- WS-09 — Cleanup, Shutdown, and Janitor
with support from WS-02, WS-06, WS-07, and WS-08

#### BPHASE-051 — Phase 9 Non-Goals
Phase 9 is not intended to complete:
- updater and distribution hardening
- final release signoff by itself

#### BPHASE-052 — Phase 9 Exit Intent
Phase 9 should end only when the product can terminate runs and app lifecycles truthfully and handle abnormal leftovers through janitor logic instead of crash resume.

---

### 5.13 Phase 10 — Updater, Runtime Delivery, and Distribution Hardening

#### BPHASE-053 — Phase 10 Purpose
Phase 10 establishes trusted delivery and maintenance behavior outside the core run loop.

#### BPHASE-054 — Phase 10 Core Outcomes
Phase 10 must establish at minimum:
- update discovery flow
- runtime/download distribution support
- artifact verification
- update-application trust rules
- runtime/update integrity protection
- distribution behavior aligned with Document A constraints

#### BPHASE-055 — Phase 10 Workstreams
Phase 10 primarily involves:
- WS-10 — Updater, Distribution, and Runtime Delivery
with support from WS-02 and WS-03 where necessary

#### BPHASE-056 — Phase 10 Non-Goals
Phase 10 is not intended to redefine the core simulation or acceptance truth.
It hardens delivery and lifecycle maintenance.

#### BPHASE-057 — Phase 10 Exit Intent
Phase 10 should end only when install/update/runtime-delivery behaviors are trustworthy enough for controlled release preparation.

---

### 5.14 Phase 11 — QA Matrix Execution, Red-Team Review, and Release Readiness

#### BPHASE-058 — Phase 11 Purpose
Phase 11 establishes whether the MVP is actually ready to be treated as release-ready.

#### BPHASE-059 — Phase 11 Core Outcomes
Phase 11 must establish at minimum:
- acceptance mapping execution
- regression evaluation
- red-team review against priority risk classes
- evidence-backed gate assessment
- release-readiness determination
- explicit unresolved-issue accounting

#### BPHASE-060 — Phase 11 Workstreams
Phase 11 primarily involves:
- WS-11 — QA, Acceptance, Red-Team, and Regression
with support from all workstreams and governance lanes

#### BPHASE-061 — Phase 11 Non-Goals
Phase 11 is not intended to add product features.
It is intended to prove or reject readiness.

#### BPHASE-062 — Phase 11 Exit Intent
Phase 11 should end only with one of the following:
- release-ready conclusion
- release-blocked conclusion
- rework-required conclusion

---

### 5.15 Inter-Phase Ordering Rules

#### BPHASE-063 — Ordered Phase Rule
The canonical order of phases is the order listed in this section.
This order must not be rearranged casually.

#### BPHASE-064 — Dependency-Bound Phase Rule
A later phase may begin partial preparation before an earlier phase is fully closed only if:
- hard dependencies are not violated
- the overlap is explicitly documented
- phase gates are not falsely passed
- no downstream truth is misrepresented as complete

#### BPHASE-065 — No Phase Skipping Rule
No phase may be skipped if its outcomes are still required by Document A truth.

#### BPHASE-066 — No Premature Release Rule
Completion of early or middle phases must not be misrepresented as MVP completion.

---

### 5.16 Phase Entry and Exit Logic

#### BPHASE-067 — Entry Gate Rule
Every phase must later receive a formal entry gate in Section 6 or Section 8.

#### BPHASE-068 — Exit Gate Rule
Every phase must later receive a formal exit gate in Section 6 or Section 8.

#### BPHASE-069 — Entry Requires Real Preconditions Rule
A phase entry gate must be based on real prerequisite satisfaction, not planning optimism.

#### BPHASE-070 — Exit Requires Real Proof Rule
A phase exit gate must be based on demonstrable deliverables and validation, not narrative progress claims.

---

### 5.17 Phase Integrity Rules

#### BPHASE-071 — No Scope Mutation Rule
A phase must not be used to mutate the approved MVP into a different product.

#### BPHASE-072 — No Future-Feature Smuggling Rule
Future-facing or deferred items must not be quietly pulled into a phase unless upstream governance changes first.

#### BPHASE-073 — No False Closure Rule
A phase must not be marked complete if unresolved blockers would undermine the next phase’s truth-bearing work.

#### BPHASE-074 — Reopenability Rule
A closed phase may be reopened if later evidence shows its exit conditions were not truly satisfied.

---

### 5.18 Boundaries of This Section

#### BPHASE-075 — Section Boundary Rule
This section defines:
- the canonical phase list
- the purpose of each phase
- the intended scope of each phase
- the non-goals of each phase
- the ordering rules between phases

This section does not yet define:
- the full detailed tasks inside each phase
- the exact task packet structures
- the detailed validation evidence required
- the detailed replan logic

Those are defined in later sections.

---

### 5.19 Final Phase Summary

#### BPHASE-076 — Phase Summary Rule
At the program-map level, the MVP is delivered through eleven ordered phases that establish execution governance, local runtime trust, commercial activation truth, provider readiness, Preflight gating, simulation behavior, live run truth, finalization/report truth, cleanup/janitor truth, distribution/update hardening, and finally acceptance-backed release readiness.

#### BPHASE-077 — Stability Rule
All later sections of Document B must remain consistent with the phase map defined here unless explicitly revised through the Document B Change Log.

---

## 6. Phase-by-Phase Execution Plans

### 6.1 Section Purpose
This section defines the operational execution plan for each canonical phase in the phase map.

This section exists to ensure that:
- each phase is expanded into real execution intent rather than remaining only a label
- AI coding agents receive deterministic phase-level guidance before task-level packetization
- every phase has clear boundaries, prerequisites, deliverables, non-goals, and closure logic
- execution order remains aligned with the dependency model and with Document A

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Execution Constitution
- Section 4 — Global Dependency Model
- Section 5 — Phase Map
- Document A — Master Project Creative Brief

If any phase plan in this section conflicts with Document A or with higher-authority sections of Document B, the higher-authority rule wins.

---

### 6.2 Section-Wide Phase Planning Rules

#### BPLAN-001 — Phase Expansion Rule
Each phase defined in Section 5 must be expanded here using a stable planning template.

#### BPLAN-002 — No Product-Law Rewrite Rule
The plans in this section must remain execution-facing and must not restate or redefine product law from Document A.

#### BPLAN-003 — Entry/Exit Discipline Rule
Each phase plan must define:
- why the phase exists
- what it is allowed to do
- what it must not do
- what must already exist before it begins
- what must be proven before it closes

#### BPLAN-004 — Deliverable Truth Rule
A phase deliverable must correspond to a real implementation capability, control, or verification artifact.

#### BPLAN-005 — Non-Goal Rule
Every phase must define explicit non-goals so AI agents do not expand scope merely because adjacent work becomes reachable.

#### BPLAN-006 — Reopenability Rule
Every phase must be reopenable if later evidence proves its exit gate was passed incorrectly or incompletely.

---

## 6.3 Phase 1 — Governance, Repository Foundation, and Execution Scaffolding

### 6.3.1 Phase Purpose
Phase 1 establishes the governance and execution substrate required for safe AI-led implementation.

### 6.3.2 Why This Phase Exists
The project cannot be executed safely as AI-led development unless:
- repository boundaries exist
- document hierarchy is explicit
- execution packet structure is defined
- file/module organization is stable enough for downstream work
- traceability discipline exists before code volume grows

### 6.3.3 Parent Document A Dependencies
Phase 1 depends primarily on:
- Document A Section 0 — Document Control
- Document A Section 1 — Project Constitution
- Document A Section 16 — Implementation Guardrails for AI Coding Agents
- Document A Section 18 — Change Log

### 6.3.4 Entry Preconditions
Phase 1 may begin only when:
- Document A is available as the governing brief
- the project owner has accepted the execution-document architecture
- a repository or project workspace can be initialized
- version control and document placement conventions can be defined

### 6.3.5 Workstreams Included
Primary workstreams:
- WS-01 — Governance and Spec Traceability
- WS-12 — AI-Agent Task Packaging and Prompt Control

Secondary support:
- light scaffolding support from WS-02 where repo-level runtime/app structure is needed

### 6.3.6 Tasks to Be Completed
Phase 1 should establish at minimum:
- repository root structure
- document folder strategy
- naming conventions for source-of-truth documents
- implementation folder strategy by layer where known
- change-control discipline for Documents A, B, and C
- traceability method linking execution tasks to Document A rules
- baseline prompt/packet standard for AI coding agents
- baseline execution log or planning discipline for future work

### 6.3.7 Explicit Non-Goals
Phase 1 must not attempt to complete:
- actual runtime hydration
- activation flow
- provider configuration
- Preflight
- run execution
- report generation
- cleanup behavior
- updater behavior

### 6.3.8 Deliverables
Phase 1 deliverables should include at minimum:
- repository structure
- document placement plan
- task packet governance standard
- traceability mechanism
- initial execution-control conventions for AI agents

### 6.3.9 Validation Gate
Phase 1 may close only when:
- the execution environment is organized enough for downstream work
- AI-agent tasking can reference upstream rules deterministically
- there is a stable place to add code and annex artifacts without structural drift

### 6.3.10 Failure / Reopen Conditions
Phase 1 must be reopened if:
- downstream work cannot be traced back to Document A
- AI agents are producing unbounded or ambiguous code due to missing scaffolding discipline
- repository structure creates repeated ownership confusion

### 6.3.11 Handoff to Next Phase
Phase 1 hands off:
- repository and document structure
- task packet discipline
- governance stability
to Phase 2.

---

## 6.4 Phase 2 — Desktop Shell, Runtime Hydration, and Local Trust Boundary

### 6.4.1 Phase Purpose
Phase 2 establishes the privileged local execution foundation of the app.

### 6.4.2 Why This Phase Exists
All later local behavior depends on:
- app-shell lifecycle control
- runtime hydration availability
- runtime integrity checking
- a trustworthy local app-to-engine boundary
- controlled shutdown and process supervision

### 6.4.3 Parent Document A Dependencies
Phase 2 depends primarily on:
- Document A Sections 7 and 8 for lifecycle and architecture truth
- Document A Section 9 for storage boundaries
- Document A Section 10 for local interface trust rules
- Document A Section 13 for runtime/update integrity requirements

### 6.4.4 Entry Preconditions
Phase 2 may begin only when:
- Phase 1 governance structure exists
- the intended shell technology and local execution model are frozen enough for implementation
- basic repository scaffolding for frontend/shell/engine separation exists

### 6.4.5 Workstreams Included
Primary workstreams:
- WS-02 — Desktop Shell and Runtime Management

Secondary support:
- WS-10 — Updater, Distribution, and Runtime Delivery where runtime download/verification surfaces are needed
- WS-12 — AI-Agent Task Packaging and Prompt Control

### 6.4.6 Tasks to Be Completed
Phase 2 should establish at minimum:
- app startup path
- startup checks path
- runtime presence detection
- runtime hydration initiation flow
- hydration progress/error states
- runtime integrity validation
- local engine process supervision structure
- local app-session trust boundary concept and implementation path
- basic controlled shutdown orchestration

### 6.4.7 Explicit Non-Goals
Phase 2 must not claim completion of:
- licensing/activation
- provider readiness
- Preflight
- simulation behavior
- reporting
- cleanup hardening beyond what is necessary for shell control

### 6.4.8 Deliverables
Phase 2 deliverables should include at minimum:
- working shell lifecycle scaffold
- runtime hydration flow
- runtime integrity gate
- local trust-boundary scaffold
- engine spawn/supervision base

### 6.4.9 Validation Gate
Phase 2 may close only when:
- the app can truthfully distinguish runtime-ready from runtime-not-ready conditions
- simulation features are blocked when runtime integrity is not acceptable
- the shell is clearly the supervising authority for local execution

### 6.4.10 Failure / Reopen Conditions
Phase 2 must be reopened if:
- the frontend can bypass shell-owned privileged behavior
- runtime integrity is not actually blocking execution when it should
- the local backend trust boundary is effectively absent or decorative

### 6.4.11 Handoff to Next Phase
Phase 2 hands off:
- runtime readiness truth
- shell supervision truth
- local trust-boundary foundations
to Phase 3 and later phases.

---

## 6.5 Phase 3 — Licensing, Activation, and Commercial Backend

### 6.5.1 Phase Purpose
Phase 3 establishes valid licensed use and commercial truth.

### 6.5.2 Why This Phase Exists
The MVP is a licensed product with device-bound use.
Therefore:
- entitlement truth
- activation logic
- denial/error classification
- and offline post-activation validation
must exist before truthful licensed run use can exist.

### 6.5.3 Parent Document A Dependencies
Phase 3 depends primarily on:
- Document A Section 1 — Project Constitution
- Document A Section 7 — Activation lifecycle
- Document A Section 10 — Commercial interface rules
- Document A Section 11 — Database and Commercial Backend Contract
- Document A Section 14 — Licensing and activation acceptance criteria

### 6.5.4 Entry Preconditions
Phase 3 may begin only when:
- Phase 2 shell and secure local orchestration base exists
- there is a defined path for local secure persistence of activation artifacts
- the commercial entity model is sufficiently frozen for implementation

### 6.5.5 Workstreams Included
Primary workstreams:
- WS-03 — Licensing, Activation, and Commercial Backend

Secondary support:
- WS-02 — Desktop Shell and Runtime Management
- WS-11 — QA, Acceptance, Red-Team, and Regression for early validation planning

### 6.5.6 Tasks to Be Completed
Phase 3 should establish at minimum:
- license lookup/validation pathway
- device activation request flow
- device-bound token issuance pathway
- local token validation pathway
- two-device enforcement logic
- denial vs technical failure distinction
- local activation persistence for offline post-activation validation
- app lifecycle integration of activated vs unactivated state

### 6.5.7 Explicit Non-Goals
Phase 3 must not claim completion of:
- provider configuration
- Preflight
- simulation engine behavior
- reporting
- cleanup hardening

### 6.5.8 Deliverables
Phase 3 deliverables should include at minimum:
- activation flow implementation
- backend commercial truth scaffold or service
- device-bound validation logic
- denial/error UI state wiring
- activation-state gating inside app readiness

### 6.5.9 Validation Gate
Phase 3 may close only when:
- activation truth is real, not mocked
- third-device activation is denied truthfully
- local activation state is usable for approved offline post-activation validation
- device mismatch does not validate as normal use

### 6.5.10 Failure / Reopen Conditions
Phase 3 must be reopened if:
- unlimited device sharing is effectively possible
- denial and error are conflated
- the local app can proceed as licensed without valid activation truth

### 6.5.11 Handoff to Next Phase
Phase 3 hands off:
- activation truth
- entitlement gating
- local licensed-readiness state
to Phase 4 and later run-readiness phases.

---

## 6.6 Phase 4 — Provider Configuration and Secure Local Setup

### 6.6.1 Phase Purpose
Phase 4 establishes provider-dependent readiness and secret-handling truth.

### 6.6.2 Why This Phase Exists
The simulation depends on user-provided provider credentials.
Therefore:
- provider selection
- secure API-key storage
- configuration readiness state
must exist before truthful Preflight and run execution can begin.

### 6.6.3 Parent Document A Dependencies
Phase 4 depends primarily on:
- Document A Section 8 — Architecture
- Document A Section 9 — Data Lifecycle and Storage Policy
- Document A Section 10 — API and Interface Contract
- Document A Section 12 — UI/UX Contract
- Document A Section 14 — Provider configuration acceptance criteria

### 6.6.4 Entry Preconditions
Phase 4 may begin only when:
- shell-mediated secure local operations exist
- activation and basic app readiness states are wired
- provider choices for MVP are frozen sufficiently

### 6.6.5 Workstreams Included
Primary workstreams:
- WS-04 — Provider Configuration and Secret Handling

Secondary support:
- WS-02 — Desktop Shell and Runtime Management
- WS-11 — QA, Acceptance, Red-Team, and Regression

### 6.6.6 Tasks to Be Completed
Phase 4 should establish at minimum:
- provider selection UI/state
- API-key entry path
- OS-native secure storage integration
- key retrieval mediation through approved local layers
- missing-configuration gating
- truthful configuration-present vs run-ready distinction

### 6.6.7 Explicit Non-Goals
Phase 4 must not claim completion of:
- Preflight feasibility truth
- live provider run behavior
- moderated simulation
- finalization/reporting

### 6.6.8 Deliverables
Phase 4 deliverables should include at minimum:
- provider setup flow
- secure key persistence path
- key retrieval path for authorized execution
- readiness-state wiring for missing vs present configuration

### 6.6.9 Validation Gate
Phase 4 may close only when:
- provider secrets are not being intentionally stored in plaintext app files/workspaces/logs
- the app blocks run start when required provider configuration is missing
- the UI does not overstate configuration-present as full run readiness

### 6.6.10 Failure / Reopen Conditions
Phase 4 must be reopened if:
- secrets are found in plaintext persistence paths
- configuration state is decorative rather than gating
- the frontend bypasses the allowed secure handling boundary

### 6.6.11 Handoff to Next Phase
Phase 4 hands off:
- provider configuration truth
- secure secret-handling baseline
to Phase 5.

---

## 6.7 Phase 5 — Seed Intake and Preflight

### 6.7.1 Phase Purpose
Phase 5 establishes truthful run eligibility.

### 6.7.2 Why This Phase Exists
The product promises bounded, gated runs rather than naive “upload then go.”
Therefore Preflight must determine:
- whether the input is acceptable
- whether the run is feasible
- whether downgrade is required
- whether run start is allowed at all

### 6.7.3 Parent Document A Dependencies
Phase 5 depends primarily on:
- Document A Section 6 — Product Behavior Specification
- Document A Section 7 — Preflight lifecycle
- Document A Section 10 — Interface contracts for Preflight
- Document A Section 12 — Preflight UX rules
- Document A Section 14 — Seed/Preflight acceptance criteria

### 6.7.4 Entry Preconditions
Phase 5 may begin only when:
- runtime readiness is real
- activation truth exists
- provider configuration truth exists
- seed intake plumbing can be implemented safely

### 6.7.5 Workstreams Included
Primary workstreams:
- WS-05 — Seed Intake and Preflight

Secondary support:
- WS-02, WS-03, WS-04
- WS-11 for validation planning

### 6.7.6 Tasks to Be Completed
Phase 5 should establish at minimum:
- single-seed intake flow
- supported file-type enforcement
- file-size enforcement
- extractable-text checks
- English-only detection/rejection
- provider/model readiness checks within Preflight
- requested plan capture
- effective plan derivation
- downgrade/reject response logic
- run-start eligibility gating
- invalidation of stale Preflight results when material inputs change

### 6.7.7 Explicit Non-Goals
Phase 5 must not claim completion of:
- full moderated run execution
- live transcript/event flow
- finalization/report generation
- cleanup hardening

### 6.7.8 Deliverables
Phase 5 deliverables should include at minimum:
- working Preflight route/flow
- truthful Preflight result classes
- gating of run start by eligible Preflight state
- requested-vs-effective plan UI/data support

### 6.7.9 Validation Gate
Phase 5 may close only when:
- non-English input is rejected truthfully
- approved-with-downgrade is distinct from full approval
- rejected is distinct from technical error
- run start is blocked without eligible Preflight
- changed material inputs invalidate stale Preflight truth

### 6.7.10 Failure / Reopen Conditions
Phase 5 must be reopened if:
- run start can bypass Preflight
- stale Preflight results remain usable after material changes
- downgrade behavior is hidden or mislabeled

### 6.7.11 Handoff to Next Phase
Phase 5 hands off:
- truthful run eligibility
- effective-plan truth
- run-start gating
to Phase 6.

---

## 6.8 Phase 6 — Simulation Engine Core

### 6.8.1 Phase Purpose
Phase 6 establishes the core moderated simulation behavior.

### 6.8.2 Why This Phase Exists
The MVP’s defining product behavior is the bounded moderated multi-agent run.
Therefore the engine must implement:
- ingestion
- runtime persona generation
- Moderator control
- bounded rounds
- bounded memory
- token/time tracking

### 6.8.3 Parent Document A Dependencies
Phase 6 depends primarily on:
- Document A Section 6 — Behavior rules
- Document A Section 7 — Run/session state logic
- Document A Section 8 — Engine ownership boundaries
- Document A Section 14 — Persona, moderated debate, and memory acceptance criteria

### 6.8.4 Entry Preconditions
Phase 6 may begin only when:
- run-start eligibility can be enforced
- a run workspace can be created
- input has been accepted via Preflight
- provider configuration and activation are truthfully gated

### 6.8.5 Workstreams Included
Primary workstreams:
- WS-06 — Simulation Engine and Persona System

Secondary support:
- WS-02
- WS-05
- WS-12
- WS-11 for early validation mapping

### 6.8.6 Tasks to Be Completed
Phase 6 should establish at minimum:
- session creation and start path
- workspace allocation
- seed ingestion for execution
- runtime-generated persona set
- exactly one Moderator agent
- per-round 3–5 speaker selection logic
- bounded memory layers
- rolling argument/risk continuity
- token usage tracking
- duration tracking participation
- active round execution path under the approved model

### 6.8.7 Explicit Non-Goals
Phase 6 must not claim completion of:
- full user-visible live monitoring fidelity
- final report truth
- cleanup closure
- updater/release hardening

### 6.8.8 Deliverables
Phase 6 deliverables should include at minimum:
- engine-run core
- persona generation path
- Moderator-controlled round logic
- bounded memory implementation baseline
- budget/time tracking foundation

### 6.8.9 Validation Gate
Phase 6 may close only when:
- persona generation is runtime-driven rather than static-user-authored
- the Moderator model actually limits normal rounds to 3–5 selected speakers
- the engine is not using naive full-transcript replay as the core memory model
- budget/time tracking is structurally integrated into run behavior

### 6.8.10 Failure / Reopen Conditions
Phase 6 must be reopened if:
- all-agent fanout becomes the normal run pattern
- bounded memory is not real
- run execution can occur without workspace isolation or Preflight lineage

### 6.8.11 Handoff to Next Phase
Phase 6 hands off:
- real run execution output
- engine lifecycle activity
- run-state-producing behavior
to Phase 7 and Phase 8.

---

## 6.9 Phase 7 — Run State, Events, Live Monitoring, and Interruption Handling

### 6.9.1 Phase Purpose
Phase 7 makes the run truthfully visible and controllable during execution.

### 6.9.2 Why This Phase Exists
Without this phase, the user cannot tell:
- what the run is doing
- whether it is healthy or degraded
- whether it is paused
- whether cancellation is occurring
- whether finalization has begun

### 6.9.3 Parent Document A Dependencies
Phase 7 depends primarily on:
- Document A Section 7 — Run/session state machine
- Document A Section 10 — Status/events route contracts
- Document A Section 12 — Live monitoring UX rules
- Document A Section 14 — Active-run visibility and interruption acceptance criteria

### 6.9.4 Entry Preconditions
Phase 7 may begin only when:
- the engine can produce meaningful run output
- canonical run states are stable enough for exposure
- the status/event contract can be implemented without inventing fake truth

### 6.9.5 Workstreams Included
Primary workstreams:
- WS-07 — Run State, Events, and Live Monitoring

Secondary support:
- WS-06
- WS-02
- WS-11

### 6.9.6 Tasks to Be Completed
Phase 7 should establish at minimum:
- status route behavior
- event route behavior
- ordered sequence semantics
- UI live-run surface
- phase/state visibility
- transcript/event chronology
- paused-rate-limit and paused-network truth
- cancellation path integration
- status/event consistency protections

### 6.9.7 Explicit Non-Goals
Phase 7 must not claim completion of:
- final report truth
- report availability
- cleanup hardening
- release readiness

### 6.9.8 Deliverables
Phase 7 deliverables should include at minimum:
- truthful live run-monitoring surface
- coherent state exposure
- ordered event synchronization path
- visible interruption/degradation states
- user-visible cancel path when valid

### 6.9.9 Validation Gate
Phase 7 may close only when:
- the user can distinguish key run phases truthfully
- degraded states are not mislabeled as healthy progress
- event order is deterministic enough for incremental UI sync
- status and events do not tell contradictory stories

### 6.9.10 Failure / Reopen Conditions
Phase 7 must be reopened if:
- the UI hides blocked/degraded reality
- event ordering is unreliable
- cancellation state is falsely represented
- live monitoring claims more than the backend actually knows

### 6.9.11 Handoff to Next Phase
Phase 7 hands off:
- truthful visible run history/state
- cancellation/degradation integration
to Phase 8 and Phase 9.

---

## 6.10 Phase 8 — Finalization, Report Generation, and Report Access

### 6.10.1 Phase Purpose
Phase 8 establishes truthful run completion output.

### 6.10.2 Why This Phase Exists
The MVP is not complete when debate merely runs.
It is complete only when actual completed material can be transformed into a truthful output artifact.

### 6.10.3 Parent Document A Dependencies
Phase 8 depends primarily on:
- Document A Section 6 — Finalization and report-generation behavior
- Document A Section 7 — Finalization/completion states
- Document A Section 10 — Report availability contract
- Document A Section 12 — Completion/report UX rules
- Document A Section 14 — Report acceptance criteria

### 6.10.4 Entry Preconditions
Phase 8 may begin only when:
- meaningful run material can be produced
- finalization triggers are understood
- the system can distinguish normal completion, constrained completion, and failure

### 6.10.5 Workstreams Included
Primary workstreams:
- WS-08 — Finalization, Report Generation, and Export

Secondary support:
- WS-06
- WS-07
- WS-11

### 6.10.6 Tasks to Be Completed
Phase 8 should establish at minimum:
- finalization entry behavior
- finalization stop-of-normal-rounds behavior
- viability score generation pathway
- required report section assembly
- truthful cost/token summary inclusion
- transcript appendix inclusion from actual material
- PDF generation
- report-availability truth
- local report access behavior
- distinction between completion and constrained completion

### 6.10.7 Explicit Non-Goals
Phase 8 must not claim completion of:
- cleanup closure
- updater hardening
- release signoff

### 6.10.8 Deliverables
Phase 8 deliverables should include at minimum:
- finalization logic
- truthful PDF report generation path
- report availability/status integration
- constrained-completion classification behavior

### 6.10.9 Validation Gate
Phase 8 may close only when:
- successful paths generate a real report
- constrained completion is not mislabeled as full completion
- report content is based on actual run material
- placeholder/empty PDFs are not treated as success
- report availability is truthful

### 6.10.10 Failure / Reopen Conditions
Phase 8 must be reopened if:
- finalization fabricates missing debate material
- completion is reported without real report generation
- report availability is asserted without an accessible file

### 6.10.11 Handoff to Next Phase
Phase 8 hands off:
- truthful completion/report path
- report persistence/access distinction
to Phase 9 and Phase 11.

---

## 6.11 Phase 9 — Cleanup, Shutdown, and Startup Janitor Hardening

### 6.11.1 Phase Purpose
Phase 9 establishes truthful end-of-life behavior for runs and for abnormal termination aftermath.

### 6.11.2 Why This Phase Exists
Document A requires:
- best-effort cleanup
- cleanup-result truth
- no crash resume
- startup janitor purge of abandoned workspaces

These are not optional hardening extras.
They are part of the product’s lifecycle truth.

### 6.11.3 Parent Document A Dependencies
Phase 9 depends primarily on:
- Document A Section 6 — Cleanup behavior
- Document A Section 7 — Cleanup lifecycle
- Document A Section 9 — Data lifecycle and cleanup scope
- Document A Section 12 — Cleanup UX rules
- Document A Section 14 — Cleanup acceptance criteria

### 6.11.4 Entry Preconditions
Phase 9 may begin only when:
- terminal run states exist
- shell-level shutdown coordination exists
- workspace boundaries are already real
- report export boundaries are already understood

### 6.11.5 Workstreams Included
Primary workstreams:
- WS-09 — Cleanup, Shutdown, and Janitor

Secondary support:
- WS-02
- WS-06
- WS-07
- WS-08
- WS-11

### 6.11.6 Tasks to Be Completed
Phase 9 should establish at minimum:
- cleanup trigger on terminal paths
- cleanup-attempt behavior
- cleanup-result classification
- separation of run outcome from cleanup outcome
- app-close coordination for active runs
- abnormal termination leftover handling
- startup janitor detection and purge
- explicit preservation of no-resume behavior
- protection of exported reports outside workspace cleanup scope

### 6.11.7 Explicit Non-Goals
Phase 9 must not claim:
- forensic-grade deletion
- crash-resume support
- guaranteed cleanup success in all OS conditions

### 6.11.8 Deliverables
Phase 9 deliverables should include at minimum:
- cleanup path implementation
- truthful cleanup-result exposure
- startup janitor behavior
- shutdown-to-cleanup integration
- anti-resume preservation

### 6.11.9 Validation Gate
Phase 9 may close only when:
- cleanup is attempted after terminal paths
- cleanup failure is not silently hidden
- next-launch janitor attempts stale purge after abnormal termination
- the product still does not offer crash-resume
- exported reports are not deleted as part of workspace purge

### 6.11.10 Failure / Reopen Conditions
Phase 9 must be reopened if:
- cleanup truth is hidden
- janitor does not meaningfully attempt stale purge
- resume behavior is accidentally introduced
- report preservation boundaries are violated

### 6.11.11 Handoff to Next Phase
Phase 9 hands off:
- truthful post-run lifecycle behavior
- janitor safety
- no-resume preservation
to Phase 10 and Phase 11.

---

## 6.12 Phase 10 — Updater, Runtime Delivery, and Distribution Hardening

### 6.12.1 Phase Purpose
Phase 10 establishes trustworthy artifact delivery and update behavior.

### 6.12.2 Why This Phase Exists
A correct core app is not sufficient if:
- runtime delivery is unsafe
- updates are not verified
- distribution flows can undermine shell/runtime integrity

### 6.12.3 Parent Document A Dependencies
Phase 10 depends primarily on:
- Document A Section 7 — Updater lifecycle
- Document A Section 8 — Runtime distribution architecture
- Document A Section 10 — Commercial/update interface rules
- Document A Section 11 — Runtime distribution/update support contract
- Document A Section 13 — Runtime/update integrity requirements

### 6.12.4 Entry Preconditions
Phase 10 may begin only when:
- the core app lifecycle is stable enough to integrate updates safely
- runtime delivery boundaries are understood
- artifact verification rules are frozen enough for implementation

### 6.12.5 Workstreams Included
Primary workstreams:
- WS-10 — Updater, Distribution, and Runtime Delivery

Secondary support:
- WS-02
- WS-03
- WS-11

### 6.12.6 Tasks to Be Completed
Phase 10 should establish at minimum:
- update discovery flow
- update availability truth
- runtime/download authorization support
- artifact download behavior
- verification before apply/use
- rejection of unsafe update artifacts
- trusted runtime distribution behavior
- compatibility checks where required

### 6.12.7 Explicit Non-Goals
Phase 10 must not redefine:
- simulation behavior
- run-state truth
- report truth
- cleanup truth

### 6.12.8 Deliverables
Phase 10 deliverables should include at minimum:
- updater behavior implementation
- verified runtime delivery path
- update/runtime integrity enforcement
- user-visible safe update state handling

### 6.12.9 Validation Gate
Phase 10 may close only when:
- unsafe updates are not installable
- runtime artifacts are verified before use
- updater states are truthful
- update flows do not silently override active-run truth

### 6.12.10 Failure / Reopen Conditions
Phase 10 must be reopened if:
- unverified artifacts can be used/applied
- rejected updates still appear installable
- update behavior undermines lifecycle or state truth

### 6.12.11 Handoff to Next Phase
Phase 10 hands off:
- hardened delivery/update behavior
to Phase 11.

---

## 6.13 Phase 11 — QA Matrix Execution, Red-Team Review, and Release Readiness

### 6.13.1 Phase Purpose
Phase 11 determines whether the MVP is actually ready to be considered releasable.

### 6.13.2 Why This Phase Exists
The project must not equate “implemented” with “ready.”
Release readiness must be evidence-backed against acceptance, risk, and truthfulness obligations.

### 6.13.3 Parent Document A Dependencies
Phase 11 depends primarily on:
- Document A Section 14 — Acceptance Criteria
- Document A Section 15 — Red-Team Risks and Known Weaknesses
- Document A Section 13 — Non-Functional Requirements
- Document A Section 16 — AI guardrails where implementation drift risk exists

### 6.13.4 Entry Preconditions
Phase 11 may begin only when:
- all prior required phases have passed their exit gates or have explicitly documented exceptions
- the acceptance and red-team matrix exists in usable form
- the system is stable enough to evaluate release readiness honestly

### 6.13.5 Workstreams Included
Primary workstreams:
- WS-11 — QA, Acceptance, Red-Team, and Regression

Secondary support:
- all other workstreams
- WS-01 for governance traceability
- WS-12 for AI-generated artifact auditability

### 6.13.6 Tasks to Be Completed
Phase 11 should establish at minimum:
- acceptance execution against Document A
- regression execution
- red-team scenario execution
- issue triage and categorization
- residual known-issue accounting
- readiness verdict formation
- rework-required verdict handling where applicable

### 6.13.7 Explicit Non-Goals
Phase 11 must not:
- add new MVP features
- silently redefine acceptance
- hide unresolved critical blockers
- relabel incomplete work as releasable

### 6.13.8 Deliverables
Phase 11 deliverables should include at minimum:
- acceptance evidence
- red-team evidence
- regression evidence
- release-readiness assessment
- blocker log where relevant

### 6.13.9 Validation Gate
Phase 11 may close only when one of the following is true:
- the MVP is evidence-backed release-ready
- the MVP is evidence-backed release-blocked
- the MVP requires rework and cannot yet be signed off

### 6.13.10 Failure / Reopen Conditions
Phase 11 must be reopened if:
- critical evidence is missing
- acceptance truth was not actually verified
- blocker severity was understated
- red-team findings materially change readiness

### 6.13.11 Handoff to Next Phase
There is no later build phase in the canonical MVP sequence.
Phase 11 hands off only to:
- release action
- rework planning
- or governance revision

---

### 6.14 Cross-Phase Integrity Rules

#### BPLAN-007 — No Hidden Phase Mutation Rule
A later phase must not be used to silently repair a missing earlier truth while still pretending the earlier phase was complete.

#### BPLAN-008 — Downstream Compensation Prohibition
Later phases must not normalize working around missing upstream foundations as if that were acceptable completion logic.

#### BPLAN-009 — Phase Traceability Rule
Each phase must remain traceable to the Document A rules it operationalizes.

#### BPLAN-010 — Phase-to-Workstream Consistency Rule
If a workstream participates in a phase, its role in that phase must remain consistent with Section 3 workstream ownership.

#### BPLAN-011 — Gate Integrity Rule
Phase gates in this section are planning truths and must later be made concrete in validation and QA sections without weakening their meaning.

---

### 6.15 Boundaries of This Section

#### BPLAN-012 — Section Boundary Rule
This section defines:
- the operational purpose of each phase
- phase entry logic
- phase scope
- phase non-goals
- phase deliverables
- phase validation intent
- phase reopen conditions
- inter-phase handoff logic

This section does not yet define:
- the unit-level task packet schema in full
- the formal validation matrix
- the detailed replan rules
- the phase change log

Those are defined in later sections.

---

### 6.16 Final Planning Summary

#### BPLAN-013 — Section Summary Rule
At the execution-plan level, the MVP is implemented as eleven ordered phases, each of which exists to establish a bounded new layer of executable truth that later phases depend upon, without mutating the product law established in Document A.

#### BPLAN-014 — Stability Rule
All later sections of Document B must remain consistent with the phase plans defined here unless explicitly revised through the Document B Change Log.

---

## 7. Execution Unit / Task Packet Standard

### 7.1 Section Purpose
This section defines the standard structure for bounded execution packets that can be safely given to AI coding agents or human implementers.

This section exists to ensure that:
- execution work is packaged deterministically
- AI coding agents do not operate from vague instructions
- every task has traceable scope boundaries
- task completion can be verified without inventing new truth
- anti-drift discipline is embedded at the unit-of-work level

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Execution Constitution
- Section 4 — Global Dependency Model
- Section 6 — Phase-by-Phase Execution Plans
- Document A — Master Project Creative Brief

---

### 7.2 Role of the Task Packet Standard

#### BTASK-001 — Task Packet Rule
A task packet is the smallest authoritative execution unit that Document B should hand to an AI coding agent or human implementer.

#### BTASK-002 — Execution Wrapper Rule
A task packet is an execution wrapper for upstream requirements.
It is not a source of independent product truth.

#### BTASK-003 — Determinism Rule
Every task packet must be specific enough that an implementer can identify:
- what to build
- why it exists
- what must not be changed
- how success is proven
- when escalation is required

#### BTASK-004 — No Vague Task Rule
Document B must not use vague execution units such as:
- “build the app”
- “do Preflight”
- “finish activation”
without structured decomposition

---

### 7.3 Mandatory Task Packet Template

#### BTASK-005 — Canonical Task Packet Template
Every task packet in Document B should use the following structure:

1. Task ID  
2. Task Title  
3. Parent Phase  
4. Parent Workstream  
5. Objective  
6. Why This Exists  
7. Source-of-Truth References from Document A  
8. Dependency Preconditions  
9. Inputs / Required Context  
10. Files / Modules Expected to Change  
11. Implementation Scope  
12. Required Behavior to Preserve  
13. Explicit Non-Goals / Forbidden Changes  
14. Deliverables  
15. Validation / Acceptance Proof Required  
16. Risks / Failure Modes  
17. Escalation Triggers  
18. Rollback / Rework Trigger  
19. Completion Decision

#### BTASK-006 — Template Stability Rule
This template should remain stable unless the Document B Change Log explicitly revises it.

---

### 7.4 Field Definitions

#### BTASK-007 — Task ID Field
The Task ID must be unique and should encode enough information to identify:
- phase
- workstream
- local task number

#### BTASK-008 — Task Title Field
The Task Title must be short, precise, and action-oriented.

#### BTASK-009 — Parent Phase Field
Every task packet must state exactly which phase it belongs to.

#### BTASK-010 — Parent Workstream Field
Every task packet must state exactly which workstream primarily owns it.

#### BTASK-011 — Objective Field
The Objective field must describe what concrete implementation outcome the task is intended to produce.

#### BTASK-012 — Why This Exists Field
This field must explain the rationale of the task in execution terms and tie it back to upstream truth.

#### BTASK-013 — Source-of-Truth References Field
This field must cite the specific Document A and, where relevant, Document B rules that justify the task.

#### BTASK-014 — Dependency Preconditions Field
This field must explicitly state what must already exist before the task may be executed truthfully.

#### BTASK-015 — Inputs / Required Context Field
This field must describe the minimum inputs, artifacts, documents, or system conditions the implementer needs.

#### BTASK-016 — Files / Modules Expected to Change Field
This field must narrow the expected code/artifact surface so AI agents do not roam arbitrarily through the codebase.

#### BTASK-017 — Implementation Scope Field
This field must describe what work is inside the task boundary.

#### BTASK-018 — Required Behavior to Preserve Field
This field must list the upstream truths the task must preserve while implementing its changes.

#### BTASK-019 — Explicit Non-Goals / Forbidden Changes Field
This field must state what the task must not do.

#### BTASK-020 — Deliverables Field
This field must list the expected implementation outputs.

#### BTASK-021 — Validation / Acceptance Proof Required Field
This field must state how the task will be judged complete.

#### BTASK-022 — Risks / Failure Modes Field
This field must identify the main implementation risks associated with the task.

#### BTASK-023 — Escalation Triggers Field
This field must state exactly when the implementer must stop and escalate.

#### BTASK-024 — Rollback / Rework Trigger Field
This field must state what evidence would force the task to be revised or reopened.

#### BTASK-025 — Completion Decision Field
This field must end in one of a limited set of outcomes:
- complete
- complete with noted follow-up dependency
- blocked
- rework required
- escalated

---

### 7.5 Task ID Convention

#### BTASK-026 — Task ID Convention Rule
The recommended task ID format is:

**P[phase]-WS[workstream]-T[task number]**

Example:
- `P05-WS05-T01`
- `P06-WS06-T04`
- `P09-WS09-T02`

#### BTASK-027 — Task ID Stability Rule
Once assigned, a task ID should remain stable unless the task is retired or structurally split.

#### BTASK-028 — No Ambiguous ID Rule
Task IDs must not be vague or reused for unrelated work.

---

### 7.6 Task Packet Integrity Rules

#### BTASK-029 — One Primary Objective Rule
A task packet must have one primary implementation objective even if it has secondary effects.

#### BTASK-030 — No Multi-Feature Dump Rule
A task packet must not bundle too many unrelated goals just because they touch the same area of code.

#### BTASK-031 — Dependency Truth Rule
A task packet must not be marked executable if its required upstream dependencies are absent.

#### BTASK-032 — Scope Boundary Rule
A task packet must explicitly protect against scope creep by stating its non-goals and forbidden changes.

#### BTASK-033 — No Hidden Rewrite Rule
A task packet must not hide product-level rewrites inside implementation details.

#### BTASK-034 — Deliverable Truth Rule
Task deliverables must be real artifacts, verified code behavior, or evidence-bearing outputs.

---

### 7.7 AI-Agent Safety Rules for Task Packets

#### BTASK-035 — AI Readability Rule
Task packets must be written so an AI coding agent can execute them without relying on unspoken project assumptions.

#### BTASK-036 — Anti-Hallucination Rule
Task packets must include enough context to reduce the chance that AI agents:
- invent missing requirements
- change unrelated modules
- violate layer boundaries
- add hidden persistence
- bypass lifecycle gates

#### BTASK-037 — Minimal Necessary Context Rule
A task packet should provide the minimum sufficient context necessary for safe execution, not the entire project history.

#### BTASK-038 — Anti-Roaming Rule
A task packet must constrain the likely file/module surface area so AI agents do not modify large unrelated parts of the project.

#### BTASK-039 — No Open-Ended Prompt Rule
A task packet must not read like a generic brainstorming prompt.
It must read like a bounded engineering instruction set.

---

### 7.8 Validation Rules for Task Packets

#### BTASK-040 — Validation Requirement Rule
Every task packet must define how completion is proven.

#### BTASK-041 — Upstream Acceptance Alignment Rule
Task-level proof should map where possible to one or more Document A acceptance criteria.

#### BTASK-042 — No Narrative Completion Rule
A task must not be considered complete merely because an implementer says it is complete.

#### BTASK-043 — Evidence Rule
Evidence may include:
- code changes
- route behavior
- state behavior
- validation logs
- test results
- generated artifacts
- UI state proof
- manually verified lifecycle proof

#### BTASK-044 — Partial Completion Rule
If a task is only partially complete, the completion decision must not falsely label it complete.

---

### 7.9 Escalation Rules for Task Packets

#### BTASK-045 — Escalation Rule
A task packet must define concrete escalation triggers.

#### BTASK-046 — Mandatory Escalation Cases
At minimum, escalation must occur when:
- the task appears to conflict with Document A
- the task appears to conflict with higher-authority sections of Document B
- the task would require hidden persistence
- the task would require a layer-boundary violation
- the task would introduce unauthorized scope
- the task reveals a major ambiguity affecting product truth
- the task depends on a deferred item becoming active scope

#### BTASK-047 — Stop-and-Escalate Rule
When an escalation trigger is hit, the implementer must stop rather than creatively reinterpreting the spec.

---

### 7.10 Rollback and Rework Rules

#### BTASK-048 — Rework Rule
Every task packet must define what would invalidate its completion claim.

#### BTASK-049 — Rollback Trigger Examples
Rollback or rework triggers may include:
- failing validation evidence
- contradiction with lifecycle truth
- contradiction with cleanup truth
- contradiction with architecture boundaries
- acceptance misalignment
- red-team discovery of hidden unsafe behavior

#### BTASK-050 — No Sticky Completion Rule
A task must not remain labeled complete if later evidence shows it violated upstream truth.

---

### 7.11 Task Packet Example Skeleton

#### BTASK-051 — Example Skeleton Rule
A canonical task packet should look like this structurally:

##### Task ID
`P05-WS05-T01`

##### Task Title
Implement Preflight result invalidation on material input change

##### Parent Phase
Phase 5 — Seed Intake and Preflight

##### Parent Workstream
WS-05 — Seed Intake and Preflight

##### Objective
[one clear implementation objective]

##### Why This Exists
[execution rationale]

##### Source-of-Truth References from Document A
[relevant rule references]

##### Dependency Preconditions
[what must exist first]

##### Inputs / Required Context
[documents, routes, states, files]

##### Files / Modules Expected to Change
[specific bounded implementation surface]

##### Implementation Scope
[what is in scope]

##### Required Behavior to Preserve
[upstream truths that must remain intact]

##### Explicit Non-Goals / Forbidden Changes
[what must not be changed]

##### Deliverables
[expected outputs]

##### Validation / Acceptance Proof Required
[how completion is proven]

##### Risks / Failure Modes
[main risks]

##### Escalation Triggers
[when to stop and escalate]

##### Rollback / Rework Trigger
[what invalidates completion]

##### Completion Decision
[complete / blocked / rework / escalated]

#### BTASK-052 — Example Non-Authority Rule
The example skeleton above is structural guidance only.
Actual task packets must still derive from real upstream rules and phase logic.

---

### 7.12 Task Packet Packaging Rules

#### BTASK-053 — One Packet One Bounded Change Rule
Each task packet should prefer one bounded change set or one tightly related set of changes.

#### BTASK-054 — Packet Sequencing Rule
Task packets must be issued in dependency-safe order.

#### BTASK-055 — Packet Granularity Rule
Task packets should be:
- small enough for clear execution
- large enough to produce a meaningful deliverable
- not so large that they become vague
- not so small that they create administrative fragmentation

#### BTASK-056 — Packet Reusability Rule
A packet template may be reused structurally across phases, but packet content must remain phase-specific and truth-specific.

---

### 7.13 Human Review Rules for Task Packets

#### BTASK-057 — Human Reviewability Rule
Task packets must be readable and reviewable by the project owner even if the code work is AI-assisted.

#### BTASK-058 — Human Override Rule
A human reviewer may reject, pause, or re-scope a task packet if it appears to drift from Document A or Document B.

#### BTASK-059 — No Blind Execution Rule
AI-generated output from a task packet must not be treated as trustworthy merely because the packet was well-formed.
Validation still governs completion.

---

### 7.14 Boundaries of This Section

#### BTASK-060 — Section Boundary Rule
This section defines:
- the canonical execution unit
- the standard task packet template
- field meanings
- AI-agent packaging rules
- validation rules
- escalation rules
- rollback/rework rules

This section does not yet define:
- the full validation matrix
- the phase exit-gate matrix
- the replan governance matrix
- Document B change-log mechanics

Those are defined in later sections.

---

### 7.15 Final Task Packet Summary

#### BTASK-061 — Section Summary Rule
At the execution-unit level, Document B requires all meaningful implementation work to be packaged into bounded, traceable, validation-backed task packets that preserve upstream truth, constrain AI-agent behavior, and explicitly define scope, non-goals, escalation triggers, and proof of completion.

#### BTASK-062 — Stability Rule
All later operational tasking under Document B should remain consistent with this task packet standard unless explicitly revised through the Document B Change Log.

---

## 8. Validation, QA, and Exit Gates

### 8.1 Section Purpose
This section defines how implementation progress is verified and how completion is judged at the task, phase, workstream, and MVP levels.

This section exists to ensure that:
- completion is evidence-based rather than narrative-based
- QA and red-team work are execution-governing functions rather than optional postscript activities
- AI coding agents cannot self-certify work as complete without proof
- release readiness remains traceable to Document A acceptance truth

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Execution Constitution
- Section 4 — Global Dependency Model
- Section 6 — Phase-by-Phase Execution Plans
- Section 7 — Execution Unit / Task Packet Standard
- Document A — Master Project Creative Brief

If any validation or exit-gate rule here conflicts with Document A acceptance truth, Document A wins.

---

### 8.2 Role of Validation in Document B

#### BVAL-001 — Validation Rule
Validation is the process by which implementation claims are tested against upstream truth.

#### BVAL-002 — Exit-Gate Rule
An exit gate is a bounded decision point at which a task, phase, workstream, or release path is judged:
- passable
- blocked
- incomplete
- or requiring rework

#### BVAL-003 — Evidence-Backed Progress Rule
Progress must be measured through evidence-backed gate passage rather than optimism, effort, or volume of code.

#### BVAL-004 — No Narrative Completion Rule
Implementation text such as:
- “it should work”
- “the flow is there”
- “the UI is done”
- “the logic exists”
is not sufficient proof of completion by itself.

#### BVAL-005 — Upstream-Grounded Validation Rule
All validation performed under Document B must map back to Document A truth rather than inventing a parallel truth model.

---

### 8.3 Validation Layers

#### BVAL-006 — Layered Validation Rule
Validation must occur at four levels:

1. task-level validation  
2. phase-level validation  
3. workstream-level validation  
4. MVP/release-level validation

#### BVAL-007 — Task-Level Validation Definition
Task-level validation proves whether a single bounded execution packet achieved its declared deliverable without violating its constraints.

#### BVAL-008 — Phase-Level Validation Definition
Phase-level validation proves whether a phase established the new layer of executable truth it was supposed to establish.

#### BVAL-009 — Workstream-Level Validation Definition
Workstream-level validation proves whether a stable execution lane has delivered the capabilities and controls expected of it across the phases it participates in.

#### BVAL-010 — MVP-Level Validation Definition
MVP-level validation proves whether the overall product is ready to be considered release-ready under Document A’s acceptance, risk, and non-functional rules.

---

### 8.4 Validation Categories

#### BVAL-011 — Validation Category Rule
Validation under Document B must be grouped into the following categories:

1. structural validation  
2. behavior validation  
3. state-machine validation  
4. data-policy validation  
5. interface validation  
6. commercial/activation validation  
7. UI truthfulness validation  
8. cleanup and janitor validation  
9. non-functional validation  
10. red-team validation  
11. acceptance criteria validation

#### BVAL-012 — Structural Validation Definition
Structural validation checks whether the implementation is placed, organized, and bounded correctly according to architecture and workstream rules.

#### BVAL-013 — Behavior Validation Definition
Behavior validation checks whether the system behaves according to Document A’s behavior rules.

#### BVAL-014 — State-Machine Validation Definition
State-machine validation checks whether lifecycle states, transitions, prohibitions, and terminal paths remain truthful.

#### BVAL-015 — Data-Policy Validation Definition
Data-policy validation checks whether persistence, cleanup scope, secret handling, and report/workspace boundaries follow Document A.

#### BVAL-016 — Interface Validation Definition
Interface validation checks whether routes, payload behaviors, sequencing, and local trust boundaries remain correct and truthful.

#### BVAL-017 — Commercial/Activation Validation Definition
Commercial/activation validation checks license truth, device activation truth, denial/error truth, and slot enforcement.

#### BVAL-018 — UI Truthfulness Validation Definition
UI truthfulness validation checks whether the UI communicates real system truth without scope inflation or hidden failure.

#### BVAL-019 — Cleanup and Janitor Validation Definition
Cleanup and janitor validation checks whether cleanup is attempted, cleanup results are surfaced truthfully, and abnormal leftovers are handled through janitor logic instead of run resume.

#### BVAL-020 — Non-Functional Validation Definition
Non-functional validation checks constraints such as platform scope, responsiveness, boundedness, integrity requirements, logging discipline, and update/runtime trust rules.

#### BVAL-021 — Red-Team Validation Definition
Red-team validation checks the implementation against known weakness classes, likely exploit surfaces, and truthfulness failure modes.

#### BVAL-022 — Acceptance Criteria Validation Definition
Acceptance validation checks explicit compliance with Document A acceptance criteria.

---

### 8.5 Validation Sources of Truth

#### BVAL-023 — Primary Validation Source Rule
The primary validation source of truth is Document A.

#### BVAL-024 — Required Validation Reference Families
Validation work in Document B should explicitly map to one or more of:
- constitutional rules
- behavior rules
- state rules
- architecture rules
- data rules
- interface rules
- UI rules
- non-functional rules
- acceptance criteria
- risk classes

#### BVAL-025 — No Parallel Definition of Done Rule
Document B must not create a new independent definition of “done” that can override or dilute Document A.

#### BVAL-026 — Traceable Validation Rule
Every major phase gate and release gate should trace to concrete upstream rule families or acceptance IDs.

---

### 8.6 Task-Level Validation Rules

#### BVAL-027 — Task Validation Requirement
Every task packet must define task-level proof requirements before execution begins.

#### BVAL-028 — Task Completion Rule
A task may be considered complete only if:
- its deliverables exist
- its required behavior is preserved
- its forbidden changes did not occur
- its validation proof has been provided
- no escalation condition remains unresolved

#### BVAL-029 — Task Completion Outcome Set
Task completion outcomes must remain within a bounded set such as:
- complete
- complete with follow-up dependency
- blocked
- rework required
- escalated

#### BVAL-030 — No Task Self-Certification Rule
An AI coding agent must not be allowed to self-certify task completion without evidence.

#### BVAL-031 — Task Validation Evidence Rule
Task-level evidence may include:
- code diffs
- route behavior proof
- state transition proof
- test results
- generated artifact proof
- UI proof
- controlled manual verification
provided the evidence is appropriate to the task

---

### 8.7 Phase-Level Exit Gates

#### BVAL-032 — Phase Exit-Gate Requirement
Every phase must have a clear exit gate before it can be considered closed.

#### BVAL-033 — Phase Exit-Gate Inputs
A phase exit gate should consider at minimum:
- required deliverables
- required upstream truth preservation
- required validation evidence
- unresolved blockers
- downstream safety

#### BVAL-034 — No Premature Phase Pass Rule
A phase must not pass its exit gate merely because:
- code has been generated
- the main happy path appears to work once
- a demo exists
- mock behavior looks convincing

#### BVAL-035 — Downstream Safety Gate Rule
A phase exit gate must ask:
Can the next phase build on this output without compensating for hidden foundational missing truth?

#### BVAL-036 — Phase Closure Outcome Set
A phase exit decision must remain within a bounded set such as:
- passed
- passed with tracked limitations
- blocked
- failed
- rework required

#### BVAL-037 — Phase Reopen Rule
A phase that has passed may be reopened if later evidence shows that its gate was passed on false or incomplete grounds.

---

### 8.8 Workstream-Level Validation Rules

#### BVAL-038 — Workstream Validation Requirement
Each workstream must eventually demonstrate that it has delivered its defined responsibilities across the phases in which it participates.

#### BVAL-039 — Workstream Proof Rule
Workstream proof should include:
- implemented capabilities
- preserved boundaries
- passed validations relevant to its domain
- unresolved limitations still carried
- explicit non-completions where applicable

#### BVAL-040 — No Workstream Narrative Completion Rule
A workstream must not be declared complete merely because many of its tasks are closed if its core responsibilities remain unproven.

#### BVAL-041 — Cross-Phase Workstream Review Rule
Because workstreams span multiple phases, a workstream’s validation must consider its integrated performance across phase boundaries.

---

### 8.9 MVP / Release-Level Exit Gates

#### BVAL-042 — Release-Level Exit-Gate Requirement
Document B must define an MVP/release-level gate that determines whether the product is:
- release-ready
- release-blocked
- or rework-required

#### BVAL-043 — Release Gate Must Be Evidence-Backed
The MVP/release gate must be based on evidence aggregated from task, phase, and workstream validations.

#### BVAL-044 — Release Gate Required Inputs
The MVP/release gate should consider at minimum:
- acceptance criteria coverage
- critical lifecycle truth
- cleanup and janitor truth
- activation and licensing truth
- runtime/update trust
- report-generation truth
- red-team results
- unresolved blockers
- non-functional compliance posture

#### BVAL-045 — No “Works Once” Release Rule
A one-time successful run or demo is not sufficient for release readiness.

#### BVAL-046 — No Cosmetic Readiness Rule
Visual polish, animated UI, or apparently smooth demos must not substitute for lifecycle and acceptance truth.

#### BVAL-047 — Release Outcome Set
The release gate must end in one of a bounded set of outcomes:
- release-ready
- release-ready with documented minor limitations
- release-blocked
- rework-required

---

### 8.10 Validation Evidence Standards

#### BVAL-048 — Evidence Standard Rule
Validation evidence must be specific enough that a reviewer can understand what was proved and what was not proved.

#### BVAL-049 — Minimum Evidence Rule
Evidence should identify at minimum:
- what was tested or verified
- under what conditions
- what outcome occurred
- what acceptance or rule family it supports
- what limitations remain

#### BVAL-050 — No Opaque Evidence Rule
Evidence must not be so vague that it becomes impossible to distinguish:
- implemented
- partially implemented
- mocked
- unverified
- or broken

#### BVAL-051 — Evidence Retention Rule
Important phase and release evidence should be retained in a durable execution record or annex reference sufficient for later audit.

---

### 8.11 QA Role and Scope

#### BVAL-052 — QA Workstream Rule
QA under Document B is not only a late-stage test sweep.
It is a cross-cutting execution function.

#### BVAL-053 — QA Scope Rule
QA should participate in:
- task-packet validation planning
- phase-gate proof planning
- acceptance mapping
- regression control
- red-team scenario preparation
- release gate evidence review

#### BVAL-054 — No End-Only QA Rule
QA must not be deferred entirely to the final release phase.

#### BVAL-055 — QA Non-Authority Rule
QA validates implementation truth.
QA does not redefine product truth.

---

### 8.12 Red-Team Validation Rules

#### BVAL-056 — Red-Team Requirement
Red-team review is a required validation class for the MVP and not an optional stretch activity.

#### BVAL-057 — Red-Team Focus Areas
Red-team validation should focus especially on:
- local auth-boundary weakness
- secret leakage
- hidden persistence
- false cleanup claims
- state desynchronization
- unsafe update/runtime trust failures
- activation-limit bypass
- report or transcript fabrication
- hidden downgrade behavior
- cleanup-failure concealment

#### BVAL-058 — Red-Team Timing Rule
Red-team activity should influence execution earlier than final release signoff where risk classes are foundational.

#### BVAL-059 — Red-Team Outcome Rule
Red-team findings must feed into:
- blocker classification
- rework decisions
- release readiness
- or explicit residual-risk accounting

---

### 8.13 Acceptance Criteria Validation Rules

#### BVAL-060 — Acceptance Mapping Requirement
Document B must eventually map implementation validation back to Document A acceptance criteria.

#### BVAL-061 — Acceptance Mapping Granularity Rule
Acceptance mapping should happen at multiple levels:
- task-to-acceptance where useful
- phase-to-acceptance where useful
- final release gate-to-acceptance comprehensively

#### BVAL-062 — No Acceptance Drift Rule
Acceptance validation must verify Document A’s criteria as written, not a softened implementation reinterpretation.

#### BVAL-063 — Blocking Acceptance Rule
If a critical acceptance criterion remains unmet, the MVP must not be labeled release-ready.

---

### 8.14 Gate Failure Handling Rules

#### BVAL-064 — Gate Failure Rule
If a task, phase, workstream, or release gate fails, the outcome must be explicit rather than hidden.

#### BVAL-065 — Failure Classification Rule
Gate failure should be classified at minimum as:
- blocked
- incomplete
- failed
- rework required
- escalated
depending on the context

#### BVAL-066 — No Silent Gate Bypass Rule
A failed gate must not be bypassed by relabeling the work as complete or “good enough” without explicit governance approval.

#### BVAL-067 — Rework Traceability Rule
If a gate failure triggers rework, the reason for rework must be recorded clearly enough that the same failure mode can be checked again later.

---

### 8.15 Exit-Gate Integrity Prohibitions

#### BVAL-068 — No Decorative Gate Rule
An exit gate must not exist only as ritual language.
It must actually constrain completion.

#### BVAL-069 — No Hope-Based Pass Rule
A gate must not pass on the basis of hope, assumption, or confidence without evidence.

#### BVAL-070 — No Fake Partial-Success Rule
A materially incomplete truth-bearing capability must not be mislabeled as complete merely because it produced a narrow success path.

#### BVAL-071 — No Release-Washing Rule
Known blockers must not be hidden inside optimistic release language.

---

### 8.16 Boundaries of This Section

#### BVAL-072 — Section Boundary Rule
This section defines:
- validation layers
- validation categories
- task/phase/workstream/release gate rules
- evidence expectations
- QA scope
- red-team role
- acceptance mapping rules
- failure-handling rules

This section does not yet define:
- the replan mechanics after failure
- escalation routing logic in full
- Document B change-log procedure

Those are defined in later sections.

---

### 8.17 Final Validation Summary

#### BVAL-073 — Section Summary Rule
At the validation level, Document B requires all meaningful implementation progress to be judged by evidence-backed gates tied to Document A truth, with QA, red-team, and acceptance mapping treated as execution-governing functions rather than optional late-stage polish.

#### BVAL-074 — Stability Rule
All later execution control in Document B must remain consistent with the validation and exit-gate rules defined here unless explicitly revised through the Document B Change Log.

---

## 9. Replan, Escalation, and Drift-Control Rules

### 9.1 Section Purpose
This section defines how execution may change without violating the governing truth of the project.

This section exists to ensure that:
- implementation problems do not silently mutate the product
- AI coding agents do not convert ambiguity into unauthorized scope
- replanning remains bounded and auditable
- execution drift is detected and controlled before it becomes structural

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Execution Constitution
- Section 4 — Global Dependency Model
- Section 8 — Validation, QA, and Exit Gates
- Document A — Master Project Creative Brief

If any replan or escalation rule described here conflicts with Document A, Document A wins.

---

### 9.2 Role of This Section

#### BDRIFT-001 — Replan Rule
Replanning is the controlled revision of execution order, task decomposition, timing, or implementation route without changing governing product truth.

#### BDRIFT-002 — Escalation Rule
Escalation is the required act of surfacing a problem when safe execution cannot continue within the current authority boundaries.

#### BDRIFT-003 — Drift-Control Rule
Drift control is the discipline that prevents:
- scope expansion
- architecture erosion
- hidden persistence
- lifecycle shortcutting
- acceptance weakening
- false completion narratives

#### BDRIFT-004 — No Product-Law Rewriting Rule
This section exists to control execution changes, not to authorize product-law changes.

---

### 9.3 What Replanning Is Allowed to Change

#### BDRIFT-005 — Allowed Replan Scope Rule
Document B may allow replanning of:
- task order
- phase-internal sequencing
- packet granularity
- temporary stubbing strategy
- staffing or AI-agent assignment
- validation timing inside a phase
- non-substantive packaging of execution work

#### BDRIFT-006 — Dependency-Safe Replan Rule
Any allowed replan must still preserve hard dependency truth.

#### BDRIFT-007 — No Meaning Drift Rule
An allowed replan must not change:
- product scope
- behavior meaning
- state-machine truth
- architecture ownership
- data-policy truth
- acceptance meaning

#### BDRIFT-008 — Traceable Replan Rule
A material replan must be recorded in a traceable way sufficient for later audit.

---

### 9.4 What Replanning Must Not Change

#### BDRIFT-009 — No Scope Replan Rule
Replanning must not be used to add features or broaden MVP commitments.

#### BDRIFT-010 — No Constraint Weakening Rule
Replanning must not weaken:
- platform constraints
- no-resume rule
- English-only rule
- activation/device-limit truth
- moderated 3-to-5 speaker model
- max token budget requirement
- two-hour cap
- cleanup truth
- report truth

#### BDRIFT-011 — No Architecture Reassignment Rule
Replanning must not quietly move ownership from:
- shell to frontend
- commercial backend to engine
- engine to cloud
- secure storage to plaintext config
or similar forbidden boundary changes

#### BDRIFT-012 — No Acceptance Rewrite Rule
Replanning must not redefine completion standards simply because implementation is harder than expected.

---

### 9.5 Replanning Triggers

#### BDRIFT-013 — Valid Replan Trigger Rule
Replanning is valid when execution conditions change in a way that does not require changing product truth, such as:
- task granularity proves too large
- task ordering inside a phase proves inefficient
- a safe stub strategy would accelerate non-blocking work
- validation evidence reveals a sequencing error
- workstream coordination can be improved without changing authority boundaries

#### BDRIFT-014 — Invalid Replan Trigger Rule
Implementation discomfort alone is not enough to justify a truth-changing replan.

#### BDRIFT-015 — Gate-Driven Replan Rule
A failed or blocked gate may trigger replanning, but only within current governing constraints unless escalation succeeds.

#### BDRIFT-016 — AI-Generated Replan Caution Rule
AI coding agents may suggest replans, but their proposals must still be checked against:
- dependency truth
- authority boundaries
- acceptance truth
- anti-drift rules

---

### 9.6 Mandatory Escalation Triggers

#### BDRIFT-017 — Escalation Requirement
Escalation is mandatory when any of the following occur:

1. a proposed implementation would violate Document A  
2. a proposed implementation would violate higher-authority sections of Document B  
3. a dependency conflict cannot be resolved narrowly  
4. a task seems to require a deferred feature to continue  
5. an ambiguity materially affects product truth  
6. a layer-boundary violation seems required  
7. hidden persistence seems likely to be introduced  
8. validation evidence suggests a supposedly closed phase was not actually complete  
9. release-readiness claims conflict with known blocker evidence

#### BDRIFT-018 — Stop-and-Escalate Rule
When a mandatory escalation trigger is hit, execution must pause on the affected decision rather than continuing through creative reinterpretation.

#### BDRIFT-019 — No Silent Override Rule
An implementer must not silently override Document A or higher-authority Document B rules in order to keep momentum.

---

### 9.7 Escalation Destinations

#### BDRIFT-020 — Escalation Routing Rule
Escalations must be routed according to the type of problem.

#### BDRIFT-021 — Product-Truth Escalation Rule
If the issue affects:
- scope
- behavior
- lifecycle meaning
- architecture ownership
- data policy
- acceptance meaning
then escalation must go to Document A governance.

#### BDRIFT-022 — Execution-Only Escalation Rule
If the issue affects:
- task order
- workstream coordination
- phase sequencing
- packet granularity
- validation timing
but not product truth, it may be resolved inside Document B governance.

#### BDRIFT-023 — Technical-Annex Escalation Rule
If the issue is purely machine-readable or implementation-detail oriented without changing truth, it may be resolved in Document C while preserving A and B authority.

#### BDRIFT-024 — Mixed Escalation Rule
If the issue spans truth and implementation detail, Document A governance must be consulted first before B/C updates are treated as authoritative.

---

### 9.8 Drift Types That Must Be Prevented

#### BDRIFT-025 — Scope Drift
Scope drift means unauthorized addition of features, capabilities, or commitments beyond Document A.

#### BDRIFT-026 — Behavior Drift
Behavior drift means implementation changes that alter what the product actually does versus what Document A defines.

#### BDRIFT-027 — Lifecycle Drift
Lifecycle drift means state and terminal-path behavior no longer matches the governing state-machine truth.

#### BDRIFT-028 — Architecture Drift
Architecture drift means ownership or trust boundaries are eroded for convenience.

#### BDRIFT-029 — Data Drift
Data drift means storage, persistence, cleanup, or secret handling behavior diverges from Document A.

#### BDRIFT-030 — Acceptance Drift
Acceptance drift means implementation teams begin using softer or different definitions of completion than Document A.

#### BDRIFT-031 — AI Drift
AI drift means AI coding agents gradually broaden scope, blur layers, or reinterpret requirements through repeated small “helpful” changes.

---

### 9.9 Drift Detection Rules

#### BDRIFT-032 — Drift Detection Requirement
Document B must require active drift detection during execution, not only at the end.

#### BDRIFT-033 — Detection Sources
Drift may be detected through:
- task review
- phase-gate review
- QA evidence
- red-team findings
- code review
- document traceability review
- discrepancy between implementation and acceptance truth

#### BDRIFT-034 — Early Detection Rule
The program should prefer early drift detection because late discovery increases rewrite cost and false confidence.

#### BDRIFT-035 — No Drift Normalization Rule
Repeated small deviations must not be normalized just because they accumulate gradually.

---

### 9.10 Drift-Control Mechanisms

#### BDRIFT-036 — Required Drift Controls
Document B should use at minimum the following drift-control mechanisms:
- explicit task packet non-goals
- explicit dependency preconditions
- explicit escalation triggers
- phase exit gates
- acceptance mapping
- change-log discipline
- cross-document traceability
- human review at meaningful gates

#### BDRIFT-037 — Anti-Hidden-Persistence Control
Execution work must explicitly guard against accidental introduction of undeclared persistence.

#### BDRIFT-038 — Anti-Layer-Bleed Control
Execution reviews must explicitly guard against responsibilities moving into the wrong layer.

#### BDRIFT-039 — Anti-Fake-Completion Control
Validation reviews must explicitly guard against false completion narratives.

#### BDRIFT-040 — Anti-Deferred-Leakage Control
Execution reviews must explicitly guard against deferred items becoming active implementation scope.

---

### 9.11 Replan Documentation Rules

#### BDRIFT-041 — Replan Documentation Rule
A material replan inside Document B must be documented clearly enough that later reviewers can understand:
- what changed
- why it changed
- what did not change
- why the replan did not violate Document A

#### BDRIFT-042 — Minimum Replan Record
A replan record should include at minimum:
- affected phase(s)
- affected workstream(s)
- prior sequence
- new sequence
- rationale
- dependency justification
- validation implications
- whether Document C artifacts must be updated

#### BDRIFT-043 — No Silent Reordering Rule
Material phase or dependency reordering must not happen silently.

---

### 9.12 AI-Agent Replan Rules

#### BDRIFT-044 — AI Suggestion Rule
AI coding agents may propose replans, but proposals are not authoritative by default.

#### BDRIFT-045 — AI Constraint Rule
Any AI-suggested replan must be checked against:
- Document A truth
- Document B execution constitution
- dependency rules
- exit-gate rules
- anti-drift rules

#### BDRIFT-046 — AI Non-Authorization Rule
An AI coding agent must not self-authorize:
- new scope
- dependency bypass
- weaker acceptance interpretation
- hidden persistence
- layer-boundary mutation

#### BDRIFT-047 — AI Escalation Rule
If an AI agent detects that the spec may be insufficient, contradictory, or blocking, it must escalate rather than improvising a new product truth.

---

### 9.13 Deferred-Scope Control Rules

#### BDRIFT-048 — Deferred Scope Rule
Deferred features from Document A may be referenced only as future context and must remain non-executable in current MVP planning unless explicitly promoted upstream.

#### BDRIFT-049 — No Future-Feature Backfill Rule
A replan must not justify introducing a deferred capability as a “temporary” way to unblock current work.

#### BDRIFT-050 — Promotion Requirement
If a deferred item truly becomes necessary, the issue must be escalated to Document A governance before Document B treats it as active scope.

---

### 9.14 Gate Failure and Replan Interaction

#### BDRIFT-051 — Gate Failure Interaction Rule
When a task or phase gate fails, the default response is not automatic scope reduction.
The response must be one of:
- rework
- bounded replan
- escalation
- blocked status
depending on the cause

#### BDRIFT-052 — No Failure-Washing Rule
Gate failure must not be disguised as strategic reprioritization if the real issue is unmet truth-bearing work.

#### BDRIFT-053 — Replan-After-Failure Rule
A bounded replan after failure is allowed only when:
- the failure cause is understood
- the new path preserves governing truth
- the new path is documented
- validation will re-check the affected truth

---

### 9.15 Change-Control Relationship

#### BDRIFT-054 — Document B Change-Control Rule
Replans that materially affect Document B sequencing or gate logic must be reflected in the Document B Change Log.

#### BDRIFT-055 — No Retroactive Legitimization Rule
A change log entry does not legitimize an unauthorized change after the fact if the change violated higher-authority truth.

#### BDRIFT-056 — Document A Change-Control Rule
If a replan would require changing Document A meaning, Document A must be revised first or in conjunction with the change.
Document B alone cannot legalize it.

---

### 9.16 Drift-Control Integrity Prohibitions

#### BDRIFT-057 — No Momentum Override Rule
Implementation momentum must not override governance.

#### BDRIFT-058 — No Convenience Override Rule
Convenience must not override lifecycle truth, acceptance truth, or architecture boundaries.

#### BDRIFT-059 — No Scope-Washing Rule
Future scope must not be disguised as MVP execution refinement.

#### BDRIFT-060 — No Ambiguity-Washing Rule
Ambiguity must not be “resolved” through silent assumption when it materially affects product truth.

#### BDRIFT-061 — No Success-Narrative Override Rule
A strong narrative that the project is progressing well must not override failed gates or unresolved blockers.

---

### 9.17 Boundaries of This Section

#### BDRIFT-062 — Section Boundary Rule
This section defines:
- allowed and forbidden replanning
- mandatory escalation triggers
- escalation routing
- drift types
- drift detection
- drift-control mechanisms
- AI-agent replan discipline
- relationship to change control

This section does not yet define:
- the final Document B change-log entries themselves
- the full operational release process outside the MVP gate model

Those are defined elsewhere or later.

---

### 9.18 Final Replan / Drift Summary

#### BDRIFT-063 — Section Summary Rule
At the execution-control level, Document B allows bounded replanning of execution order and packetization while strictly prohibiting drift in product truth, scope, architecture, data policy, acceptance meaning, and lifecycle behavior. When safe execution cannot continue within those bounds, escalation is mandatory.

#### BDRIFT-064 — Stability Rule
All later execution control under Document B must remain consistent with these replan, escalation, and drift-control rules unless explicitly revised through the Document B Change Log.

---

## 10. AI-Agent Operating Protocol for Execution

### 10.1 Section Purpose
This section defines the operating rules that AI coding agents must follow when executing work under Document B.

This section exists to ensure that:
- AI coding agents treat Document A and Document B with the correct authority hierarchy
- execution remains bounded, traceable, and dependency-safe
- AI agents do not use task execution as an excuse to redesign the product
- implementation work remains auditable and consistent across many discrete AI-driven steps

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Execution Constitution
- Section 4 — Global Dependency Model
- Section 7 — Execution Unit / Task Packet Standard
- Section 9 — Replan, Escalation, and Drift-Control Rules
- Document A — Master Project Creative Brief

If any part of this section conflicts with Document A, Document A wins.

---

### 10.2 Role of AI Agents Under Document B

#### BAI-001 — AI Execution Role Rule
Under Document B, an AI coding agent is an implementation executor operating inside bounded execution packets.

#### BAI-002 — No Product-Author Rule
An AI coding agent is not authorized to act as:
- product designer
- scope setter
- architecture governor
- policy author
- acceptance reinterpreter

#### BAI-003 — Execution Role Rule
The AI coding agent is authorized only to:
- interpret task packets
- implement bounded work
- preserve upstream truth
- report blockers
- escalate ambiguity
- produce evidence of completion or blockage

#### BAI-004 — No Independent Authority Rule
An AI coding agent has no independent authority to reinterpret Document A or Document B simply because an implementation path appears easier.

---

### 10.3 Authority Hierarchy for AI Agents

#### BAI-005 — Authority Stack Rule
AI coding agents must obey the following authority hierarchy:

1. Document A — Master Project Creative Brief  
2. Document B — Master Execution Plan  
3. Document C — Engineering Annex Pack  
4. current approved task packet  
5. local implementation notes that do not conflict with higher authority

#### BAI-006 — Document A Supremacy Rule
If an AI coding agent detects any conflict between its task and Document A, Document A wins immediately.

#### BAI-007 — Document B Execution Supremacy Rule
If a task packet conflicts with Document B but not with Document A, Document B wins and the task packet must be treated as needing correction.

#### BAI-008 — Document C Subordination Rule
Document C may supply technical detail and machine-readable specificity, but it must not override Document A or B.

#### BAI-009 — No Prompt Supremacy Rule
A local prompt or conversational instruction to an AI coding agent must not be treated as superior to Document A or Document B unless the project owner explicitly revises those governing documents.

---

### 10.4 Required Mental Model for AI Agents

#### BAI-010 — Required Mental Model Rule
An AI coding agent operating under Document B must use the following mental model:

“I am not redesigning the product. I am executing one bounded implementation unit inside a governed documentation stack. My job is to preserve upstream truth while completing the assigned execution packet.”

#### BAI-011 — Preservation-First Rule
The AI agent must preserve meaning before attempting cleanup, simplification, abstraction, or convenience-driven modification.

#### BAI-012 — Dependency-Aware Rule
The AI agent must understand that correct execution order matters, and must not treat all implementation work as equally executable at any time.

#### BAI-013 — No Cleverness-Over-Truth Rule
The AI agent must not prioritize elegance, clever abstraction, or future extensibility over explicit fidelity to Document A and Document B.

---

### 10.5 Required Execution Workflow for AI Agents

#### BAI-014 — Required Workflow Rule
When an AI agent receives a task packet, it must execute in this order:

1. inspect the task packet  
2. inspect referenced upstream rules  
3. confirm dependency preconditions  
4. identify implementation surface area  
5. identify protected non-goals and forbidden changes  
6. execute bounded work  
7. validate against the packet’s proof requirements  
8. report result as complete, blocked, escalated, or rework-required

#### BAI-015 — No Jump-to-Code Rule
An AI agent must not jump directly into code generation before understanding:
- the task objective
- the dependency state
- the required behavior to preserve
- the non-goals

#### BAI-016 — Preconditions Confirmation Rule
If a task packet lists dependency preconditions, the AI agent must verify or request confirmation that those conditions are satisfied before claiming a valid implementation path.

#### BAI-017 — Evidence-Backed Completion Rule
An AI agent must not stop at “code written.”
It must also assess whether the required validation evidence for the packet has been satisfied or remains pending.

---

### 10.6 Task Packet Consumption Rules

#### BAI-018 — Packet-Bounded Execution Rule
The AI agent must treat the task packet as the boundary of authorized execution for that unit of work.

#### BAI-019 — No Packet Expansion Rule
The AI agent must not quietly expand a task packet into:
- extra features
- adjacent refactors
- cross-workstream redesign
- speculative future-proofing
unless explicitly approved

#### BAI-020 — Non-Goal Obedience Rule
The AI agent must obey the packet’s explicit non-goals and forbidden changes.

#### BAI-021 — File-Surface Obedience Rule
If the packet identifies expected files/modules to change, the AI agent must treat those as a strong boundary and avoid unrelated files unless the task proves that broader changes are genuinely necessary.

#### BAI-022 — No Silent Packet Mutation Rule
If the AI agent believes the task packet is underspecified, incorrect, or too broad, it must report that issue rather than silently rewriting the packet’s meaning through implementation choices.

---

### 10.7 Dependency Obedience Rules

#### BAI-023 — Dependency Obedience Rule
AI agents must not execute tasks as if dependency order is optional.

#### BAI-024 — No Dependency Bypass Rule
The AI agent must not claim a downstream task is complete if its upstream hard dependencies are absent, mocked without permission, or unresolved.

#### BAI-025 — Stub Awareness Rule
If a task depends on an approved stub or mock, the AI agent must:
- recognize that fact explicitly
- avoid treating the stub as production-complete truth
- preserve the downstream gate that requires replacement

#### BAI-026 — No Out-of-Order Completion Rule
An AI agent must not use a temporary workaround to falsely mark a later phase as complete while an earlier truth-bearing phase remains incomplete.

---

### 10.8 Scope and Feature Discipline Rules

#### BAI-027 — No Feature Invention Rule
An AI coding agent must not invent new features not authorized by Document A.

#### BAI-028 — No “Helpful” Expansion Rule
The AI agent must not add capabilities merely because they seem useful, modern, or likely to be needed later.

#### BAI-029 — No Deferred Leakage Rule
Deferred items from Document A must not be pulled into active implementation without explicit upstream authorization.

#### BAI-030 — No Enterprise Drift Rule
The AI agent must not quietly optimize for enterprise deployment, team collaboration, fleet administration, or other Horizon 2 behaviors that are outside MVP scope.

#### BAI-031 — No Multilingual Drift Rule
The AI agent must not introduce multilingual seed-document support or behavior that contradicts the English-only policy.

#### BAI-032 — No Resume Drift Rule
The AI agent must not introduce crash-resume or interrupted-run restoration behavior.
Allowed pre-run runtime download/hydration recovery must not be expanded into run/session resume semantics.

---

### 10.9 Architecture and Layer-Boundary Rules

#### BAI-033 — Architecture Obedience Rule
The AI agent must preserve the ownership boundaries defined in Document A and operationalized in Document B workstreams.

#### BAI-034 — Frontend Boundary Rule
The AI agent must not place shell-owned privileged logic into the frontend.

#### BAI-035 — Shell Boundary Rule
The AI agent must preserve the shell as the owner of:
- privileged local orchestration
- process supervision
- secure local mediation
- runtime hydration coordination
- controlled shutdown coordination

#### BAI-036 — Engine Boundary Rule
The AI agent must preserve the engine as the owner of:
- Preflight computation
- simulation execution
- persona generation
- Moderator behavior
- round orchestration
- memory handling
- finalization logic

#### BAI-037 — Commercial Backend Boundary Rule
The AI agent must not transform the commercial backend into a simulation-execution or run-persistence backend.

#### BAI-038 — No Layer-Bleed Rule
If implementation appears to require violating layer boundaries, the AI agent must escalate rather than quietly collapsing responsibilities.

---

### 10.10 Data, Storage, and Cleanup Rules for AI Agents

#### BAI-039 — Data-Policy Obedience Rule
The AI agent must preserve the data lifecycle rules defined in Document A.

#### BAI-040 — No Hidden Persistence Rule
The AI agent must not introduce undeclared persistent storage for:
- seed documents
- extracted text
- run memory
- vector/index files
- transcripts
- persona sets
- report intermediates

#### BAI-041 — Secret Handling Rule
Provider secrets must only be handled through approved secure-storage paths.

#### BAI-042 — No Secret Leakage Rule
The AI agent must not place secrets into:
- plaintext config
- workspace files
- logs
- reports
- transcripts

#### BAI-043 — Cleanup Truth Rule
The AI agent must preserve the distinction between:
- run completion
- cleanup attempt
- cleanup outcome
- abnormal termination leftovers
- janitor handling

#### BAI-044 — No Anti-Forensic Overclaim Rule
The AI agent must not add user-facing or implementation comments implying perfect deletion, perfect secrecy, or forensic-grade wipe behavior.

---

### 10.11 State and Lifecycle Rules for AI Agents

#### BAI-045 — State-Machine Fidelity Rule
The AI agent must preserve lifecycle states and state transitions consistent with Document A and the execution phases of Document B.

#### BAI-046 — No Invalid Transition Rule
The AI agent must not implement invalid lifecycle shortcuts because they simplify code or demos.

#### BAI-047 — Finalization Integrity Rule
The AI agent must preserve finalization as a real distinct state and not reduce it to a decorative status label.

#### BAI-048 — Completion Integrity Rule
The AI agent must not mark a run as completed if:
- finalization failed
- report generation failed
- or the path was actually canceled/failed

#### BAI-049 — Cleanup Integrity Rule
The AI agent must not report cleanup success when cleanup actually failed or was not attempted as required.

---

### 10.12 Validation and Evidence Rules for AI Agents

#### BAI-050 — Validation Obedience Rule
The AI agent must treat validation requirements as part of the task, not as optional extra work.

#### BAI-051 — No Self-Certified Success Rule
The AI agent must not self-certify its work as complete without referencing the packet’s proof requirements.

#### BAI-052 — Evidence Discipline Rule
Where a task requires validation evidence, the AI agent must provide or describe the required evidence rather than ending with a generic success statement.

#### BAI-053 — Incomplete Means Incomplete Rule
If evidence is missing, the AI agent must report the task as:
- incomplete
- blocked
- needing review
- or needing validation
rather than complete

---

### 10.13 Escalation Rules for AI Agents

#### BAI-054 — Mandatory Escalation Rule
The AI agent must escalate rather than improvise when:
- a task conflicts with Document A
- a task conflicts with Document B
- a deferred item appears required
- a layer-boundary violation seems necessary
- hidden persistence seems likely
- an ambiguity materially affects product truth
- acceptance criteria and implementation seem misaligned

#### BAI-055 — Stop-and-Escalate Rule
When a mandatory escalation trigger is hit, the AI agent must stop on the affected decision path.

#### BAI-056 — No Silent Interpretation Rule
The AI agent must not silently interpret ambiguity in the broadest or most permissive direction.

#### BAI-057 — Narrow-Safe Assumption Rule
If a narrow temporary assumption is unavoidable, it must be:
- clearly labeled
- minimal in scope
- reversible
- non-authoritative
- reported back for review

---

### 10.14 Change Discipline Rules for AI Agents

#### BAI-058 — Minimal-Change Rule
The AI agent should prefer the smallest correct change set that satisfies the task packet.

#### BAI-059 — No Opportunistic Refactor Rule
The AI agent must not perform unrelated refactors merely because it is already touching the codebase.

#### BAI-060 — No Hidden Rewrite Rule
The AI agent must not smuggle broad rewrites inside small task implementations.

#### BAI-061 — Change Visibility Rule
The AI agent should be able to explain:
- what changed
- why it changed
- what did not change
- how upstream truth was preserved

---

### 10.15 Communication and Reporting Rules for AI Agents

#### BAI-062 — Explicit Reporting Rule
An AI agent must report outcomes in explicit bounded terms such as:
- complete
- complete with follow-up dependency
- blocked
- escalated
- rework required

#### BAI-063 — No Bluffing Rule
The AI agent must not imply certainty where it does not have validation or dependency confirmation.

#### BAI-064 — No Completion Inflation Rule
The AI agent must not describe partial or provisional work as fully complete.

#### BAI-065 — Limitation Disclosure Rule
If a task is only partially validated or still dependent on another phase/workstream, that limitation must be stated clearly.

---

### 10.16 Multi-Agent Coordination Rules

#### BAI-066 — Shared Governance Rule
If multiple AI agents are used, all of them must remain subordinate to the same Document A and Document B authority stack.

#### BAI-067 — No Divergent Truth Rule
Different AI agents must not be allowed to operate from conflicting assumptions about:
- scope
- lifecycle
- persistence
- acceptance meaning
- architecture boundaries

#### BAI-068 — Packet Isolation Rule
Each AI agent should be given bounded packetized work rather than broad overlapping mandates wherever possible.

#### BAI-069 — Coordination Visibility Rule
If multiple AI-generated changes interact, the interaction must still be reviewable under the same validation and drift-control rules.

---

### 10.17 AI-Agent Integrity Prohibitions

#### BAI-070 — No Scope-Washing Rule
The AI agent must not disguise scope expansion as refactoring, simplification, or technical necessity.

#### BAI-071 — No Acceptance-Washing Rule
The AI agent must not redefine or soften acceptance truth to fit incomplete implementation.

#### BAI-072 — No Persistence-Washing Rule
The AI agent must not normalize undeclared persistence as harmless convenience.

#### BAI-073 — No Architecture-Washing Rule
The AI agent must not normalize layer-boundary violations as harmless implementation detail.

#### BAI-074 — No Release-Washing Rule
The AI agent must not frame the project as release-ready if gate evidence does not support that claim.

---

### 10.18 Boundaries of This Section

#### BAI-075 — Section Boundary Rule
This section defines:
- how AI agents must interpret authority
- how AI agents must consume task packets
- dependency obedience
- scope discipline
- layer-boundary discipline
- validation discipline
- escalation discipline
- reporting discipline

This section does not itself define:
- the product
- the architecture
- the acceptance criteria
- the task packet contents for every task
- the phase change history

Those are defined in Document A, earlier sections of Document B, and later change-log entries.

---

### 10.19 Final AI Execution Summary

#### BAI-076 — Section Summary Rule
At the execution level, AI coding agents operating under Document B are bounded implementers that must obey the Document A → Document B → Document C → task-packet authority hierarchy, preserve upstream truth, execute in dependency-safe order, avoid scope drift and hidden persistence, provide evidence-backed completion claims, and escalate rather than improvise whenever safe execution cannot continue within the approved boundaries.

#### BAI-077 — Stability Rule
All future AI-assisted implementation work under Document B must remain consistent with this protocol unless explicitly revised through the Document B Change Log.

---

## 11. Change Log

### 11.1 Section Purpose
This section records formal revisions to Document B over time.

This section exists to ensure that:
- execution-planning changes are visible and auditable
- phase order changes do not happen silently
- workstream restructuring is traceable
- AI agents and human reviewers know which execution plan version they are operating under
- execution drift is not retroactively normalized without record

This section is governed by:
- Section 0 — Document Control
- the versioning decisions of the project owner
- the supremacy of Document A

---

### 11.2 Change-Log Philosophy

#### BCHANGE-001 — Explicit Revision Rule
Any material change to Document B must be recorded here.

#### BCHANGE-002 — No Silent Revision Rule
A locked section of Document B must not be treated as changed unless the change is explicitly reflected in this Change Log.

#### BCHANGE-003 — Version Traceability Rule
Each change-log entry must identify:
- document version
- date
- changed sections
- change type
- reason
- summary of change
- impact on execution order
- whether Document A revalidation is required

#### BCHANGE-004 — Subordinate Change Rule
No entry in this change log may be interpreted as silently revising Document A.

---

### 11.3 Document B Change Categories

#### BCHANGE-005 — Major Change Rule
A Document B change is major if it modifies:
- phase structure
- dependency order
- workstream architecture
- validation gate model
- AI-agent operating protocol in a materially different way
- or any execution rule whose change could alter implementation sequencing substantially

#### BCHANGE-006 — Moderate Change Rule
A Document B change is moderate if it refines:
- task packet standards
- phase internals
- gate wording
- escalation routing
- traceability process
without changing the overall structure of the execution plan

#### BCHANGE-007 — Minor Change Rule
A Document B change is minor if it corrects:
- wording clarity
- formatting
- traceability references
- section consistency
- naming consistency
without changing execution meaning

---

### 11.4 Required Entry Template

#### BCHANGE-008 — Entry Template Rule
Each future change-log entry should follow this structure:

- **Version:**  
- **Date:**  
- **Changed Sections:**  
- **Change Type:** major / moderate / minor  
- **Reason:**  
- **Summary of Change:**  
- **Impact on Execution Order:**  
- **Impact on Task Packets / Validation:**  
- **Impact on Document C:**  
- **Document A Revalidation Required:** yes / no  

---

### 11.5 Initial Structure Record

#### BCHANGE-009 — Initial Structure Freeze Record
The structure of Document B was intentionally frozen before drafting full execution content so that the execution plan would remain subordinate, deterministic, and stable.

#### BCHANGE-010 — Initial Batch Construction Summary
The initial draft of Document B was constructed in controlled batches with the following sequence:

- Batch 1 — Section 0: Document Control; Section 1: Execution Constitution
- Batch 2 — Section 2: Program Delivery Strategy; Section 3: Workstream Architecture
- Batch 3 — Section 4: Global Dependency Model; Section 5: Phase Map
- Batch 4 — Section 6: Phase-by-Phase Execution Plans; Section 7: Execution Unit / Task Packet Standard
- Batch 5 — Section 8: Validation, QA, and Exit Gates; Section 9: Replan, Escalation, and Drift-Control Rules
- Batch 6 — Section 10: AI-Agent Operating Protocol for Execution; Section 11: Change Log

---

### 11.6 Current Formal Entry

#### BCHANGE-011 — Current Entry
- **Version:** Draft B1.0  
- **Date:** [TO BE FILLED BY PROJECT OWNER]  
- **Changed Sections:** Sections 0–11 initial assembled draft  
- **Change Type:** major  
- **Reason:** Creation of the first full execution-governance plan subordinate to Document A — Master Project Creative Brief V2.A-Final-Draft-2  
- **Summary of Change:** Established the initial execution constitution, workstream map, dependency model, phase map, phase execution plan structure, task packet standard, validation model, drift-control rules, AI-agent execution protocol, and change-log governance for Document B  
- **Impact on Execution Order:** Canonical MVP execution order established  
- **Impact on Task Packets / Validation:** Canonical task packet and validation-gate model established  
- **Impact on Document C:** Document C must remain subordinate to both Document A and this execution-order model  
- **Document A Revalidation Required:** no  

---

### 11.7 Revalidation Rules

#### BCHANGE-012 — Document A Revalidation Rule
If a proposed change to Document B appears to require altering:
- product truth
- scope
- lifecycle meaning
- architecture boundaries
- data policy
- acceptance meaning

then Document A must be revalidated and possibly revised first.

#### BCHANGE-013 — Document C Alignment Rule
If a change to Document B affects:
- execution ordering assumptions
- task packet fields
- validation expectations
- dependency routing

then Document C artifacts should be checked for alignment.

#### BCHANGE-014 — No Retroactive Legitimization Rule
A change-log entry in Document B does not legalize an execution drift that already violated higher-authority rules.

---

### 11.8 Versioning Rules for Document B

#### BCHANGE-015 — Versioning Rule
Document B must maintain explicit version and status metadata once the unified document file is assembled.

#### BCHANGE-016 — Version Progression Rule
The project owner may adopt a predictable version progression such as:
- Draft B1.0
- Draft B1.1
- Review B1.0
- Locked B1.0

or an equivalent clearly governed versioning system.

#### BCHANGE-017 — Version Consistency Rule
If Document B version metadata changes, the corresponding Change Log entry must be updated in the same revision cycle.

---

### 11.9 Change-Log Integrity Rules

#### BCHANGE-018 — No Silent Phase Reorder Rule
Phase order must not change silently.

#### BCHANGE-019 — No Silent Gate Mutation Rule
Validation and exit-gate logic must not change silently.

#### BCHANGE-020 — No Silent Workstream Mutation Rule
Workstream ownership or structure must not change silently.

#### BCHANGE-021 — No Silent AI-Protocol Mutation Rule
The AI-agent operating protocol must not change silently.

#### BCHANGE-022 — No Silent Dependency Mutation Rule
Hard dependency relationships must not change silently.

---

### 11.10 Boundaries of This Section

#### BCHANGE-023 — Section Boundary Rule
This section defines:
- how Document B revisions are recorded
- change categories
- entry template
- initial draft record
- revalidation rules
- versioning rules
- integrity rules for future changes

This section does not itself authorize changes to Document A.
It records and governs changes to Document B only.

---

### 11.11 Final Change-Log Summary

#### BCHANGE-024 — Section Summary Rule
At the governance level, the Document B Change Log is the formal record of execution-plan evolution. It preserves auditability of phase order, dependency logic, workstream structure, validation gates, and AI-agent operating rules, while remaining explicitly subordinate to Document A.

#### BCHANGE-025 — Stability Rule
No locked decision inside Document B should be treated as changed unless this section says it changed.

---

