# DOCUMENT C — ENGINEERING ANNEX PACK

**Document ID:** DOC-C  
**Canonical Title:** Engineering Annex Pack  
**Version:** C1.0-Final-Draft-2  
**Status:** Draft – Unified Consolidated Version  
**Owner:** MIKE SALAZAR  
**Primary Audience:** AI Coding Agents, Human Implementers, QA Reviewers, Technical Maintainers  
**Authority Level:** Subordinate Technical-Annex Document  
**Governing Upstream Documents:** Document A — Master Project Creative Brief V2.A-Final-Draft-3; Document B — Master Execution Plan B1.0-Final-Draft-3  
**Last Updated:** April 15, 2026  
**Related Documents:** Document A — Master Project Creative Brief; Document B — Master Execution Plan

---

## Authority Notice

This document is subordinate to **Document A — Master Project Creative Brief V2.A-Final-Draft-3** and **Document B — Master Execution Plan B1.0-Final-Draft-3**.

Document C governs:
- structured technical annex detail
- schema and contract formalization
- event and state reference structures
- persistence and migration reference structures
- report and output technical templates
- QA and red-team technical matrix structures
- AI-agent technical reference material

Document C does **not** govern product truth or execution-order truth.

If any part of this document conflicts with Document A or Document B, the **higher-authority upstream document wins**.

---

## Document Usage Notice

This document exists to convert the locked governing brief and the locked execution plan into a deterministic technical annex system.

It serves four purposes:

1. **Technical contract authority**  
   It defines the structured technical shape of governed interfaces, states, events, persistence boundaries, outputs, and validation matrices.

2. **AI-agent engineering reference authority**  
   It provides deterministic technical detail that AI coding agents can consume without inventing product or execution truth.

3. **Technical audit authority**  
   It allows reviewers to verify whether structured technical detail remains aligned to Documents A and B.

4. **Derivation authority for technical artifacts**  
   It serves as the controlled upstream reference for future engineering artifacts such as OpenAPI-style specifications, migration sets, report templates, and QA matrices.

This document must always be interpreted as subordinate to Documents A and B.

---

## Interpretation Notice

Document C must be read with the following mental model:

- Document A defines **what the product is**
- Document B defines **how the approved product gets built**
- Document C defines **the structured technical detail used to implement that approved product safely**

Therefore:
- Document C may formalize technical shape
- Document C may not rewrite product law
- Document C may not rewrite execution law
- Document C may not expand scope
- Document C may not weaken or strengthen locked constraints
- Document C may not override acceptance meaning

---

## Table of Contents

- [0. Document Control](#0-document-control)
- [1. Annex Constitution](#1-annex-constitution)
- [2. Annex Package Strategy](#2-annex-package-strategy)
- [3. Source-of-Truth and Traceability Map](#3-source-of-truth-and-traceability-map)
- [4. Canonical Terminology and Technical Naming Rules](#4-canonical-terminology-and-technical-naming-rules)
- [5. Annex Family A — Interface and API Contract Pack](#5-annex-family-a--interface-and-api-contract-pack)
- [6. Annex Family B — Event, State, and Runtime Contract Pack](#6-annex-family-b--event-state-and-runtime-contract-pack)
- [7. Annex Family C — Data, Persistence, and Migration Pack](#7-annex-family-c--data-persistence-and-migration-pack)
- [8. Annex Family D — Report and Output Pack](#8-annex-family-d--report-and-output-pack)
- [9. Annex Family E — QA, Acceptance, and Red-Team Pack](#9-annex-family-e--qa-acceptance-and-red-team-pack)
- [10. Annex Family F — AI-Agent Reference Pack](#10-annex-family-f--ai-agent-reference-pack)
- [11. Annex Family G — Pack Index, Versioning, and Change Control](#11-annex-family-g--pack-index-versioning-and-change-control)

---

## Unified Draft Assembly Note

This unified draft is assembled from previously drafted controlled batches.

During consolidation:
- all batch-specific wrapper text must be removed
- only one master title block must remain
- only one master metadata block must remain
- only one authority notice must remain
- only one interpretation notice must remain
- only one table of contents must remain
- section numbering and rule IDs must remain intact unless a real collision is discovered
- no semantic technical-detail changes should be introduced unless explicitly approved and recorded in Section 11 — Annex Family G — Pack Index, Versioning, and Change Control

---

## Upstream Dependency Notice

This document depends on the continued authority and stability of:

- **Document_A/document_a_master_project_creative_brief_v2a_final_draft_2.md**
- **Document_B/document_b_master_execution_plan_b1_0_final_draft_2.md**

If either governing upstream document is materially revised, Document C may require:
- contract revalidation
- state/event revalidation
- persistence-boundary revalidation
- report/output revalidation
- QA-matrix revalidation
- AI-agent reference revalidation

Document C must not silently remain “current” if its governing upstream pair changes materially.

---

## 0. Document Control

### 0.1 Section Purpose
This section defines the identity, authority, governance, interpretation rules, and usage boundaries of Document C.

This section exists to ensure that:
- Document C is treated as a technical annex layer rather than a third narrative specification
- AI coding agents do not use technical detail as a back door to rewrite product or execution truth
- human reviewers can determine exactly how Document C relates to Document A and Document B
- machine-readable and engineering-facing detail remain subordinate to the governing brief and the execution plan

This section is the highest-authority section inside Document C.

---

### 0.2 Document Identity

#### CCTRL-001 — Document Title
The official title of this document is:

**Engineering Annex Pack**

#### CCTRL-002 — Document Role
Document C is the **primary authoritative technical annex document** for the project.

#### CCTRL-003 — Document Type
Document C is a technical-annex and machine-reference document.
It is not:
- the supreme product brief
- the execution-governance document
- the source code repository
- the running application
- the database itself
- the OpenAPI export file by itself
- the migration directory by itself
- the QA runner by itself

#### CCTRL-004 — Core Objective
The objective of Document C is to formalize the governed truth of Documents A and B into:
- technical contracts
- structured schemas
- engineering reference tables
- migration definitions
- event definitions
- report/output templates
- QA matrices
- AI-agent engineering reference material

#### CCTRL-005 — Technical Formalization Objective
Document C must make implementation safer and more deterministic by converting governed product and execution truth into structured technical detail without changing that truth itself.

---

### 0.3 Governing Authority

#### CCTRL-006 — Subordinate Authority Rule
Document C is subordinate to:
- **Document A — Master Project Creative Brief**
- **Document B — Master Execution Plan**

#### CCTRL-007 — Governing Upstream Version Rule
Until explicitly replaced by later approved versions, the governing upstream documents for Document C are:

- **Document A — Master Project Creative Brief, Version V2.A-Final-Draft-3**
- **Document B — Master Execution Plan, Version B1.0-Final-Draft-3**

#### CCTRL-008 — Upstream Supremacy Rule
If any statement in Document C conflicts with Document A or Document B, the higher-authority upstream document wins.

#### CCTRL-009 — No Override Authority Rule
Document C has no authority to:
- modify product scope
- redefine product behavior
- reinterpret constitutional rules
- override lifecycle truth
- override execution order
- weaken architecture boundaries
- loosen data-policy restrictions
- redefine acceptance meaning
- invent new technical obligations that upstream documents did not authorize

#### CCTRL-010 — Technical-Annex-Only Authority Rule
Document C is authoritative only for:
- structured technical detail
- schema formalization
- engineering-facing contract definition
- migration and data-structure detail
- report template structure
- event payload structure
- QA matrix structure
- technical naming and reference conventions

#### CCTRL-011 — No Hidden Product-Law Rule
Document C must not become a hidden new source of product law.

#### CCTRL-012 — No Hidden Execution-Law Rule
Document C must not become a hidden new source of execution-order law.

---

### 0.4 Relationship to Document A and Document B

#### CCTRL-013 — Relationship Map Rule
The project documentation stack consists at minimum of:

- **Document A — Master Project Creative Brief**  
  The supreme source-of-truth for product, behavior, architecture, policy, and acceptance meaning.

- **Document B — Master Execution Plan**  
  The execution-order, dependency, phase, validation, and drift-control document.

- **Document C — Engineering Annex Pack**  
  The technical annex layer for structured engineering detail, machine-readable contracts, and implementation reference material.

#### CCTRL-014 — Relationship to Document A
Document C must derive its technical definitions from Document A wherever product, behavior, state, architecture, storage, interface, report, and acceptance meaning originate.

#### CCTRL-015 — Relationship to Document B
Document C must derive its implementation-order-sensitive technical detail from Document B wherever execution grouping, dependency, phase, validation, and AI-agent tasking context are relevant.

#### CCTRL-016 — No C-to-A Reversal Rule
Document A governs Document C.
Document C does not govern Document A.

#### CCTRL-017 — No C-to-B Reversal Rule
Document B governs execution use of Document C.
Document C does not govern Document B.

#### CCTRL-018 — No Lateral Override Rule
Document C must not be used to silently resolve conflicts between Document A and Document B by creating a third interpretation path.
If upstream ambiguity exists, it must be escalated upstream.

---

### 0.5 Intended Audience

#### CCTRL-019 — Primary Audience
The primary audiences of Document C are:
1. AI coding agents implementing technical work
2. human implementers and reviewers
3. QA and validation planners
4. future technical maintainers
5. the project owner when technical verification or annex lookup is needed

#### CCTRL-020 — AI Audience Rule
Document C is explicitly written to be consumed by AI coding agents.
Therefore:
- schemas must be explicit
- technical naming must be explicit
- state/event labels must be explicit
- payload structures must be explicit
- migration detail must be explicit
- reference boundaries must be explicit

#### CCTRL-021 — Human Review Support Rule
Document C should remain readable enough that a human reviewer can verify whether the annex detail remains consistent with Document A and Document B.

---

### 0.6 Document Scope

#### CCTRL-022 — In-Scope Content Rule
Document C is in scope for:
- endpoint and interface contract detail
- request/response schema formalization
- error schema families
- event catalog and payload detail
- state enumeration tables
- transition tables for engineering reference
- database tables and migration definitions
- report template structures
- QA matrix structures
- technical naming conventions
- AI-agent technical reference guidance

#### CCTRL-023 — Out-of-Scope Content Rule
Document C is out of scope for:
- redefining product features
- changing the activation model
- changing the no-resume policy
- changing the English-only policy
- changing moderated debate behavior
- changing execution order or phase order
- changing workstream ownership
- changing acceptance meaning
- re-arguing UX philosophy as narrative product design

#### CCTRL-024 — No Narrative-Rewrite Rule
Document C must not restate all of Document A or Document B as if it were rewriting those documents.
It must instead technicalize what those documents already govern.

---

### 0.7 Interpretation Rules

#### CCTRL-025 — Descending Authority Rule
Document C must be interpreted in descending authority order:
1. Section 0 — Document Control
2. Section 1 — Annex Constitution
3. Section 2 — Annex Package Strategy
4. Section 3 — Source-of-Truth and Traceability Map
5. Section 4 — Canonical Terminology and Technical Naming Rules
6. Section 5 — Annex Family A — Interface and API Contract Pack
7. Section 6 — Annex Family B — Event, State, and Runtime Contract Pack
8. Section 7 — Annex Family C — Data, Persistence, and Migration Pack
9. Section 8 — Annex Family D — Report and Output Pack
10. Section 9 — Annex Family E — QA, Acceptance, and Red-Team Pack
11. Section 10 — Annex Family F — AI-Agent Reference Pack
12. Section 11 — Annex Family G — Pack Index, Versioning, and Change Control

#### CCTRL-026 — Upstream Supremacy Rule
All interpretation of Document C must remain subordinate to:
- Document A Section 0 — Document Control
- Document A Section 1 — Project Constitution
- Document A Section 3 — Glossary and Canonical Terminology
- Document A Sections 6–15 wherever product, behavior, state, architecture, storage, interfaces, reporting, acceptance, and known risks are governed
- Document B Section 0 — Document Control
- Document B Section 1 — Execution Constitution
- Document B Sections 3–10 wherever execution grouping, dependency, phases, validation, and AI-agent execution rules are governed

#### CCTRL-027 — Normative Language Rule
Inside Document C:
- **MUST** = mandatory technical-annex requirement
- **MUST NOT** = prohibited technical-annex behavior
- **SHOULD** = recommended technical-annex behavior unless strong reason exists otherwise
- **SHOULD NOT** = discouraged technical-annex behavior unless justified
- **MAY** = optional technical-annex behavior within allowed scope

#### CCTRL-028 — No Silent Invention Rule
If technical detail is missing, an AI coding agent or human implementer must not silently invent new product or execution truth.
The gap must be handled through:
- a narrow technical assumption explicitly labeled as such
- a technical note marked non-authoritative
- or escalation back to the governing document layer when the gap affects upstream truth

#### CCTRL-029 — Narrow-Interpretation Rule
If a technical instruction appears ambiguous, interpretation must prefer:
1. the narrower and safer reading
2. the reading most consistent with Documents A and B
3. the reading least likely to create scope drift, lifecycle drift, persistence drift, or validation drift

---

### 0.8 Conflict Resolution Rules

#### CCTRL-030 — Internal Conflict Resolution Rule
If two parts of Document C appear to conflict, precedence must be resolved in the following order:
1. Section 0 — Document Control
2. Section 1 — Annex Constitution
3. Section 2 — Annex Package Strategy
4. Section 3 — Source-of-Truth and Traceability Map
5. Section 4 — Canonical Terminology and Technical Naming Rules
6. Section 5 — Annex Family A — Interface and API Contract Pack
7. Section 6 — Annex Family B — Event, State, and Runtime Contract Pack
8. Section 7 — Annex Family C — Data, Persistence, and Migration Pack
9. Section 8 — Annex Family D — Report and Output Pack
10. Section 9 — Annex Family E — QA, Acceptance, and Red-Team Pack
11. Section 10 — Annex Family F — AI-Agent Reference Pack
12. Section 11 — Annex Family G — Pack Index, Versioning, and Change Control

#### CCTRL-031 — Upstream Conflict Rule
If any part of Document C conflicts with Document A or Document B, the upstream document wins immediately and Document C must be revised rather than reinterpreted.

#### CCTRL-032 — Specific Over General Rule
Where a specific annex rule and a general annex rule both apply, the specific rule governs within its local technical scope while the general rule remains valid elsewhere.

---

### 0.9 Traceability Rules

#### CCTRL-033 — Traceability Requirement
Every major annex family, schema family, event family, migration family, report family, and QA family in Document C must trace to one or more governing rules in Document A and, where relevant, Document B.

#### CCTRL-034 — No Orphan Annex Rule
No major technical annex item should exist in Document C without identifiable upstream justification or a clearly labeled implementation necessity that does not change product or execution truth.

#### CCTRL-035 — Rule Reference Discipline
Where useful, Document C should reference the relevant upstream rule families, including:
- constitutional rules
- behavior rules
- state rules
- architecture rules
- data rules
- interface rules
- acceptance criteria
- execution rules
- validation rules

#### CCTRL-036 — Acceptance and Validation Traceability Rule
QA, acceptance, and red-team annex material in Document C must map back to Document A acceptance meaning and Document B validation logic rather than inventing a third completion system.

---

### 0.10 Stability and Revision Rules

#### CCTRL-037 — Locked Structure Rule
Once the structure of Document C is frozen, later drafting should preserve that structure unless explicitly revised through the Change Log.

#### CCTRL-038 — No Silent Contract Change Rule
Material changes to technical contracts, schema meanings, state tables, migration structures, or validation matrices must be recorded in Document C’s Change Log.

#### CCTRL-039 — No Silent Drift Rule
Technical convenience must not silently mutate:
- product behavior meaning
- execution-order meaning
- state meaning
- persistence meaning
- acceptance meaning
- report meaning

#### CCTRL-040 — Document Version Rule
Document C must maintain explicit version and status metadata once the unified file is assembled.

---

### 0.11 AI-Agent Usage Rules

#### CCTRL-041 — AI Technical Usage Rule
AI coding agents using Document C must treat it as the authoritative source for:
- technical annex detail
- structured schemas
- engineering naming conventions
- technical reference tables
- machine-readable contract formalization
within the limits of its subordinate authority

#### CCTRL-042 — AI Non-Override Rule
AI coding agents must not use Document C as permission to override Document A or Document B.

#### CCTRL-043 — AI Drift Prevention Rule
If an AI coding agent detects that a requested technical shortcut would violate Document A, Document B, or Document C, it must:
- stop
- flag the issue
- and escalate rather than silently changing technical meaning

---

### 0.12 Boundaries of This Section

#### CCTRL-044 — Section Boundary Rule
This section defines only:
- document identity
- authority
- relationship to other documents
- interpretation
- traceability
- governance rules for the technical annex layer

This section does not yet define:
- the annex constitution in detail
- the pack strategy
- the traceability map mechanics
- naming conventions in full
- the contents of each annex family
- versioning mechanics in full

Those are defined in later sections.

---

## 1. Annex Constitution

### 1.1 Section Purpose
This section defines the non-negotiable annex truths of Document C.

This section exists to ensure that:
- technical detail remains subordinate to upstream product and execution truth
- AI coding agents do not treat machine-readable detail as permission to redesign the system
- technical contracts remain auditable and bounded
- downstream implementation stays consistent with Documents A and B

This section is the highest technical-law layer of Document C after Section 0.

---

### 1.2 Constitutional Role of Document C

#### CCONST-001 — Technical Formalization Rule
Document C exists to formalize Documents A and B into:
- technical contracts
- schema definitions
- event and state reference structures
- migration and data structures
- report/output templates
- QA and red-team matrices
- AI-agent technical reference material

#### CCONST-002 — No Product Redefinition Rule
Document C must not redefine the product.
It may only define technical detail for the locked product.

#### CCONST-003 — No Execution Redefinition Rule
Document C must not redefine execution order, phase meaning, workstream meaning, or validation-gate meaning.
It may only formalize technical detail that supports those upstream truths.

#### CCONST-004 — Technical Governance Rule
Document C is the governing document for technical annex detail, not for product meaning or execution-order meaning.

---

### 1.3 What Document C Is Allowed to Do

#### CCONST-005 — Allowed Contract Rule
Document C may define:
- interface and API contract structures
- request and response schema families
- error payload families
- event catalog structures
- event payload structures
- state enumeration tables
- transition tables for engineering use
- migration definitions
- database table definitions
- report field templates
- QA matrix structures
- technical naming conventions

#### CCONST-006 — Allowed Technical Detail Rule
Document C may add technical detail where needed for implementation determinism, so long as that detail does not change upstream truth.

#### CCONST-007 — Allowed Machine-Readable Formalization Rule
Document C may express governed truth in:
- tabular form
- schema form
- pseudo-OpenAPI form
- migration reference form
- checklist form
- matrix form
- template form

#### CCONST-008 — Allowed Reference Pack Rule
Document C may include AI-agent-oriented technical reference material if that material reduces hallucination, ambiguity, or unsafe implementation drift.

#### CCONST-009 — Allowed Validation Mapping Rule
Document C may technicalize validation structures so long as they map back to Document A acceptance meaning and Document B validation logic.

---

### 1.4 What Document C Must Not Do

#### CCONST-010 — No New Feature Rule
Document C must not introduce new features not authorized in Document A.

#### CCONST-011 — No Scope Expansion Rule
Document C must not expand scope by treating technical convenience as a new requirement.

#### CCONST-012 — No Hidden Product Change Rule
Document C must not smuggle product changes into:
- schemas
- event names
- field definitions
- migration structures
- report templates
- QA matrices
- technical examples
- AI-agent reference notes

#### CCONST-013 — No Hidden Execution Change Rule
Document C must not smuggle execution-order changes into:
- implementation contracts
- validation tables
- migration sequencing assumptions
- AI-agent technical reference notes

#### CCONST-014 — No Architecture Drift Rule
Document C must not reassign ownership boundaries defined in Document A or operationalized in Document B.

#### CCONST-015 — No Data-Policy Drift Rule
Document C must not weaken, bypass, or broaden the data lifecycle and storage rules defined in Document A.

#### CCONST-016 — No Acceptance Drift Rule
Document C must not create a new definition of done or a new acceptance regime that conflicts with Document A or Document B.

#### CCONST-017 — No Terminology Drift Rule
Document C must not introduce technical terminology that conflicts with the canonical terminology defined upstream.

#### CCONST-018 — No Deferred Leakage Rule
Deferred or future-scope ideas may be mentioned only as future context and must not become active technical obligations inside the MVP annex pack.

---

### 1.5 Annex Philosophy

#### CCONST-019 — Technical-Determinism Rule
Document C must optimize for deterministic technical interpretation.

#### CCONST-020 — Truth-Preservation Rule
Technical detail must preserve upstream truth before pursuing compactness, elegance, abstraction, or extensibility.

#### CCONST-021 — Non-Duplication Rule
Document C should not duplicate narrative explanation that already belongs in Documents A or B unless repetition is necessary to keep technical meaning safe.

#### CCONST-022 — Machine-Readiness Rule
Where multiple documentation forms are possible, Document C should prefer the form that is:
- more deterministic
- easier for AI coding agents to consume
- easier for human reviewers to audit
- less likely to create ambiguity
- more consistent with upstream truth

#### CCONST-023 — No Schema-Driven Scope Mutation Rule
Technical schema design must not be used to widen the system beyond the approved MVP.

---

### 1.6 Foundational Annex Priorities

#### CCONST-024 — Contracts Before Convenience Rule
Document C should prioritize formalization of truth-bearing interfaces, states, events, and storage boundaries before optional reference refinements.

#### CCONST-025 — State and Event Truth Priority Rule
State and event definitions must remain tightly aligned with upstream lifecycle truth.

#### CCONST-026 — Storage Boundary Priority Rule
Persistence and migration detail must remain tightly aligned with upstream data-policy truth.

#### CCONST-027 — Report Truth Priority Rule
Report templates and output structures must remain tightly aligned with upstream report/output truth.

#### CCONST-028 — Validation Traceability Priority Rule
QA and red-team annex material must remain tightly aligned with upstream acceptance and validation truth.

---

### 1.7 Annex Family Constitutional Rules

#### CCONST-029 — Annex Family Requirement Rule
Document C must be organized into annex families rather than an unstructured technical dump.

#### CCONST-030 — Annex Family Boundary Rule
Each annex family must have:
- purpose
- scope
- inclusions
- exclusions
- traceability expectations
- technical-boundary rules

#### CCONST-031 — No Annex Collapse Rule
Document C must not collapse interfaces, events, data, reports, QA, and AI-agent reference material into one vague undifferentiated section.

#### CCONST-032 — No Cross-Family Ownership Theft Rule
Annex families may reference one another, but one family must not silently absorb another family’s core technical responsibility.

---

### 1.8 Technical Contract Constitutional Rules

#### CCONST-033 — Contract Formalization Requirement
Document C must formalize contracts in a way that is reviewable and technically deterministic.

#### CCONST-034 — Contract Subordination Rule
A technical contract must function as a formalization of upstream requirements, not as an independent authority source.

#### CCONST-035 — Schema Traceability Rule
Every major schema family must trace back to upstream product or execution truth.

#### CCONST-036 — Explicit Non-Goal Requirement
Where relevant, major annex families should define what they are not intended to govern so AI agents do not overread their authority.

---

### 1.9 Validation and QA Constitutional Rules

#### CCONST-037 — Validation Technicalization Rule
Document C may technicalize QA, acceptance, and red-team structures, but must not redefine the meaning of successful validation.

#### CCONST-038 — No Parallel Acceptance Rule
Document C must not create a second independent acceptance framework.

#### CCONST-039 — Matrix Traceability Rule
Every major QA or red-team matrix family must map back to upstream acceptance or validation obligations.

---

### 1.10 AI-Agent Constitutional Rules

#### CCONST-040 — AI Technical Discipline Rule
AI coding agents using Document C must treat it as a bounded technical reference layer rather than as a permission structure for product or execution redesign.

#### CCONST-041 — AI Non-Invention Rule
AI coding agents must not invent:
- new fields with behavioral implications
- new persistence classes
- new event types with semantic drift
- new states with lifecycle drift
- new validation obligations
while operating under Document C

#### CCONST-042 — AI Technical Escalation Rule
If an AI coding agent detects likely conflict between a technical formalization choice and upstream truth, it must flag the conflict rather than silently solving it through annex expansion.

---

### 1.11 Relationship to Deferred Scope

#### CCONST-043 — Deferred Context Rule
Deferred items from Document A may be referenced in Document C only as future technical context.

#### CCONST-044 — No Deferred Promotion Rule
A deferred item does not become approved merely because it appears in:
- a schema placeholder
- a future table column note
- a migration placeholder
- a QA backlog note
- an annex example

#### CCONST-045 — Explicit Promotion Rule
If a deferred item must become active technical scope, Document A must be revised first or in conjunction with an explicit approved governance change.

---

### 1.12 Change and Revalidation Philosophy

#### CCONST-046 — Upstream Revalidation Rule
If Document A or Document B changes in a way that affects interfaces, states, persistence, reports, QA mappings, or AI-agent reference meaning, Document C must be revalidated.

#### CCONST-047 — No Silent Annex Drift Rule
Technical cleanup must not silently mutate upstream meaning.

#### CCONST-048 — Honest Technical Change Rule
If a technical annex item no longer matches upstream truth, it must be revised explicitly rather than left stale because implementation already exists.

---

### 1.13 Annex Integrity Prohibitions

#### CCONST-049 — No Contract-Washing Rule
A technical contract must not disguise unauthorized behavior as mere implementation detail.

#### CCONST-050 — No Schema-Washing Rule
A schema must not normalize scope expansion by treating invented fields as harmless.

#### CCONST-051 — No Migration-Washing Rule
A migration must not normalize unauthorized persistence by treating it as technical necessity.

#### CCONST-052 — No QA-Washing Rule
A matrix or checklist must not soften upstream validation obligations through technical wording.

#### CCONST-053 — No Reference-Pack Override Rule
AI-agent reference material inside Document C must not override upstream authority or current task packets.

---

### 1.14 Boundaries of This Section

#### CCONST-054 — Section Boundary Rule
This section defines:
- the annex constitution
- allowed and forbidden roles of Document C
- foundational technical-annex philosophy
- family-level discipline
- validation/QA discipline
- AI-agent technical discipline
- revalidation philosophy

This section does not yet define:
- the annex strategy in detail
- the traceability map mechanics
- naming conventions in full
- the content of each annex family
- pack indexing and version-control mechanics

Those are defined in later sections.

---

### 1.15 Final Constitutional Summary

#### CCONST-055 — Annex Summary Rule
At the constitutional level, Document C is defined as a subordinate technical-annex document that formalizes Documents A and B into deterministic technical contracts, schemas, reference tables, migration structures, report templates, QA matrices, and AI-agent engineering reference material without changing product truth, changing execution truth, expanding scope, weakening constraints, or allowing technical-detail drift to become hidden governance.

#### CCONST-056 — Stability Rule
All later sections of Document C must remain consistent with this annex constitution unless revised explicitly through the Document C Change Log.

---

## 2. Annex Package Strategy

### 2.1 Section Purpose
This section defines the macro-level internal architecture of Document C as a technical annex system.

This section exists to ensure that:
- Document C is structured as an intelligible technical reference system rather than a flat technical dump
- AI coding agents can navigate annex material deterministically
- human reviewers can distinguish contract families, schema families, migration families, QA families, and reference material cleanly
- machine-facing detail remains subordinate to governed upstream meaning

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- Document A — Master Project Creative Brief
- Document B — Master Execution Plan

If any package strategy described here conflicts with Documents A or B, the higher-authority upstream document wins.

---

### 2.2 Strategic Role of This Section

#### CPACK-001 — Package Strategy Rule
This section defines the internal package architecture of Document C.

#### CPACK-002 — Non-Flat-Document Rule
Document C must not be structured as a flat, undifferentiated accumulation of technical notes.

#### CPACK-003 — Technical-System Role Rule
The purpose of this section is to answer:
- how technical annex material is grouped
- how different annex families are separated
- how AI agents should navigate the pack
- how human reviewers should interpret technical detail in context
- how machine-oriented material fits inside one governed annex document

#### CPACK-004 — No Product-Law Rewriting Rule
This section must not restate Documents A or B as narrative governing documents.
It must define only how technical material is organized.

---

### 2.3 Core Packaging Objective

#### CPACK-005 — Packaging Objective Rule
The primary packaging objective of Document C is:

To provide a deterministic, subordinate, technically structured annex system that converts upstream product and execution truth into explicit engineering contracts, schemas, templates, tables, and reference material without changing the meaning of the governed system.

#### CPACK-006 — Buildability Objective Rule
The packaging strategy must optimize for:
- deterministic technical lookup
- low ambiguity for AI coding agents
- low cross-family contamination
- high traceability to Documents A and B
- auditability of technical detail
- ease of future maintenance and revalidation

#### CPACK-007 — Non-Duplication Objective Rule
The packaging strategy must minimize narrative duplication while still preserving sufficient technical clarity for safe implementation.

---

### 2.4 Pack-of-Packs Philosophy

#### CPACK-008 — Pack-of-Packs Rule
Document C must be organized as one master engineering annex document containing multiple annex families.

#### CPACK-009 — Why Pack-of-Packs Rule
This structure is required because the project needs one governed technical reference source while still separating:
- interface contracts
- event/state definitions
- data and migration structures
- report/output structures
- QA and red-team matrices
- AI-agent technical reference material

#### CPACK-010 — No Flat Appendix Rule
Document C must not collapse all technical material into one appendix section or one generic annex block.

#### CPACK-011 — Unified Master-Pack Rule
Even though Document C contains multiple annex families, it must still read as one governed document rather than as unrelated fragments.

---

### 2.5 Required Internal Documentation Modes

#### CPACK-012 — Multi-Mode Rule
Document C must support multiple internal technical documentation modes.

#### CPACK-013 — Human-Readable Reference Mode
Document C must support a human-readable reference mode for:
- tables
- lists
- technical explanations
- contract interpretation notes
- revalidation notes

#### CPACK-014 — AI-Agent Deterministic Mode
Document C must support an AI-agent deterministic mode for:
- exact field lists
- payload structures
- state sets
- event schemas
- migration detail
- validation matrix structure
- naming conventions

#### CPACK-015 — Machine-Oriented Contract Mode
Document C must support a machine-oriented contract mode for:
- pseudo-OpenAPI definitions
- structured request/response schemas
- state/event enumerations
- migration structures
- output templates
- technical matrices

#### CPACK-016 — No Mode Confusion Rule
The three modes may coexist, but the document must not blur:
- illustrative prose
- normative technical definition
- machine-oriented structure

---

### 2.6 Annex Family System

#### CPACK-017 — Annex Family Requirement Rule
Document C must be organized into explicit annex families.

#### CPACK-018 — Canonical Annex Family Set
The canonical annex families for Document C are:

1. Annex Family A — Interface and API Contract Pack  
2. Annex Family B — Event, State, and Runtime Contract Pack  
3. Annex Family C — Data, Persistence, and Migration Pack  
4. Annex Family D — Report and Output Pack  
5. Annex Family E — QA, Acceptance, and Red-Team Pack  
6. Annex Family F — AI-Agent Reference Pack  
7. Annex Family G — Pack Index, Versioning, and Change Control  

#### CPACK-019 — Stable Family Rule
These annex families should remain stable unless a future change-log revision explicitly restructures them.

#### CPACK-020 — No Casual New Family Rule
New annex families must not be added casually merely because a topic is complex.
The preferred default is to place material in the correct existing annex family unless a real structural need exists.

---

### 2.7 Annex Family Packaging Rules

#### CPACK-021 — Family Purpose Rule
Each annex family must have one clear primary technical purpose.

#### CPACK-022 — Family Boundary Rule
Each annex family must define:
- what it includes
- what it excludes
- what upstream truth it formalizes
- what other families it depends on
- what other families may depend on it

#### CPACK-023 — No Family Overreach Rule
An annex family must not silently absorb responsibilities that belong to another annex family.

#### CPACK-024 — Cross-Family Reference Rule
Cross-family reference is allowed and expected, but cross-family semantic takeover is not.

#### CPACK-025 — Primary-Ownership Rule
Every major technical artifact inside Document C should have one primary annex family owner even if other families reference it.

---

### 2.8 Internal Content Forms

#### CPACK-026 — Approved Content Form Rule
Document C may use the following content forms where appropriate:
- formal rule statements
- enumerated tables
- structured field lists
- pseudo-OpenAPI blocks
- structured schemas
- migration tables
- mapping tables
- templates
- validation matrices
- implementation reference checklists

#### CPACK-027 — Preferred Form Rule
Where multiple forms are possible, Document C should prefer the form that is:
- more deterministic
- easier to validate
- easier for AI agents to consume
- less likely to create ambiguous interpretation
- more consistent with upstream truth

#### CPACK-028 — No Decorative Format Rule
Formatting choices must not make non-authoritative examples appear more authoritative than actual contract definitions.

---

### 2.9 Technical Layering Rules

#### CPACK-029 — Layered Detail Rule
Technical detail inside Document C should be layered from:
1. family purpose
2. family scope and exclusions
3. family-level inventories
4. contract/table/schema definitions
5. examples or reference notes where needed

#### CPACK-030 — Deterministic-First Rule
Normative technical definitions must appear before examples whenever ambiguity could arise.

#### CPACK-031 — Example Subordination Rule
Examples inside Document C are subordinate to the formal contract, schema, table, or matrix they illustrate.

#### CPACK-032 — No Example-Led Meaning Rule
A technical example must not become the de facto authority if it conflicts with the formal annex definition.

---

### 2.10 Packaging Strategy for AI-Agent Use

#### CPACK-033 — AI Navigation Rule
Document C must be navigable by AI coding agents without requiring the agent to infer where a technical topic probably belongs.

#### CPACK-034 — Lookup Predictability Rule
A technical implementer or AI coding agent should be able to predict:
- where interface schemas live
- where event/state definitions live
- where database and migration detail lives
- where report structures live
- where QA matrices live
- where AI-agent implementation reference detail lives

#### CPACK-035 — No Technical Roaming Rule
The package strategy should reduce the chance that AI agents search the entire document for every technical question due to poor family separation.

#### CPACK-036 — Technical-Context Sufficiency Rule
Each annex family should contain enough local context to make its technical definitions usable without repeating the entire upstream narrative.

---

### 2.11 Non-Duplication and Compression Rules

#### CPACK-037 — Non-Duplication Rule
Document C should not duplicate upstream text unless duplication is necessary to preserve technical precision or safe implementation.

#### CPACK-038 — Safe Compression Rule
When upstream meaning is carried into Document C, it should be compressed into technical form rather than copied as large narrative blocks.

#### CPACK-039 — No Ambiguity Through Compression Rule
Compression must not remove distinctions that matter technically, including:
- state differences
- terminal classifications
- error vs rejection differences
- report availability vs run completion differences
- cleanup outcome vs run outcome differences

#### CPACK-040 — No Over-Compression Rule
Document C must not become so compressed that a reviewer or AI coding agent can no longer tell what a technical contract actually means.

---

### 2.12 Family Ordering Logic

#### CPACK-041 — Family Ordering Rule
The order of annex families must reflect a deterministic technical reading path.

#### CPACK-042 — Technical Reading Path Rule
The intended internal reading path of Document C is:
1. identity and authority
2. annex constitution
3. package strategy
4. traceability
5. terminology and naming
6. interface contracts
7. event/state/runtime contracts
8. data and migrations
9. reports and outputs
10. QA and red-team structures
11. AI-agent technical reference
12. pack index, versioning, and change control

#### CPACK-043 — No Arbitrary Ordering Rule
Annex families must not be ordered arbitrarily if that would increase ambiguity or hide dependencies.

---

### 2.13 Dependency-Aware Packaging Rules

#### CPACK-044 — Packaging Dependency Rule
The package strategy must respect technical dependency relationships between families.

#### CPACK-045 — Interface-to-State Dependency Rule
Interface and API contracts must remain consistent with state/event/runtime definitions.

#### CPACK-046 — State-to-Data Dependency Rule
State/runtime technical structures must remain consistent with persistence and migration structures where persistence or cleanup boundaries are implicated.

#### CPACK-047 — Output Dependency Rule
Report and output structures must remain consistent with:
- upstream behavior truth
- event/state truth where completion classification matters
- data/output boundary rules where report availability or export is implicated

#### CPACK-048 — QA Dependency Rule
QA and red-team matrices must remain consistent with every other technical annex family they validate.

---

### 2.14 Packaging Integrity Prohibitions

#### CPACK-049 — No Family Drift Rule
A family must not drift into broader authority than it was designed to hold.

#### CPACK-050 — No Schema-Dump Rule
Document C must not become a raw schema dump with no family logic, scope, or traceability.

#### CPACK-051 — No Checklist-Dump Rule
Document C must not become a large checklist repository detached from governing contracts.

#### CPACK-052 — No Implicit-New-Obligation Rule
The package structure itself must not create the impression that every technical placeholder is an approved MVP obligation.

---

### 2.15 Boundaries of This Section

#### CPACK-053 — Section Boundary Rule
This section defines:
- the macro internal structure of Document C
- the pack-of-packs strategy
- annex family separation logic
- approved technical content forms
- AI-agent navigation strategy
- non-duplication and packaging discipline

This section does not yet define:
- exact upstream traceability mechanics
- technical naming conventions in full
- the detailed contents of each annex family
- the versioning/change-control mechanics in full

Those are defined in later sections.

---

### 2.16 Final Package Strategy Summary

#### CPACK-054 — Section Summary Rule
At the package-strategy level, Document C is structured as one governed engineering annex document composed of stable annex families that separate interfaces, runtime truth, persistence, outputs, validation structures, AI-agent technical references, and pack governance into deterministic technical lanes without creating a new product or execution authority layer.

#### CPACK-055 — Stability Rule
All later sections of Document C must remain consistent with this annex package strategy unless revised explicitly through the Document C Change Log.

---

## 3. Source-of-Truth and Traceability Map

### 3.1 Section Purpose
This section defines how Document C maps back to its governing upstream documents.

This section exists to ensure that:
- no major technical artifact in Document C exists without upstream justification
- AI coding agents can trace technical detail back to governing product and execution truth
- human reviewers can audit whether technical detail is still aligned to Documents A and B
- change impact can be identified when upstream rules change

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- Section 2 — Annex Package Strategy
- Document A — Master Project Creative Brief
- Document B — Master Execution Plan

If any traceability rule described here conflicts with Documents A or B, the upstream document wins.

---

### 3.2 Role of Traceability in Document C

#### CTRACE-001 — Traceability Role Rule
Traceability is the mechanism by which technical detail inside Document C is tied back to governed upstream meaning.

#### CTRACE-002 — No Orphan Formalization Rule
Technical formalization must not exist without traceable upstream purpose.

#### CTRACE-003 — Technical Justification Rule
Every major annex item should be explainable in terms of:
- what upstream rule family requires it
- what technical purpose it serves
- what technical scope it does not have authority to expand

#### CTRACE-004 — Change Impact Role Rule
Traceability is also the mechanism by which upstream changes in Documents A or B trigger revalidation of affected parts of Document C.

---

### 3.3 Upstream Source-of-Truth Model

#### CTRACE-005 — Dual-Upstream Rule
Document C has two governing upstream sources:
- Document A for product, behavior, architecture, data policy, interface meaning, report meaning, acceptance meaning, and known-risk meaning
- Document B for execution grouping, dependency relevance, phase context, validation structure, and AI-agent execution context where technical detail must remain execution-aware

#### CTRACE-006 — Document A Primary Truth Rule
When a technical detail depends on what the system is, what it means, or what it is allowed to do, Document A is the primary source of truth.

#### CTRACE-007 — Document B Execution Context Rule
When a technical detail depends on how the system is grouped, sequenced, validated, or packaged for execution, Document B is the primary execution-context source.

#### CTRACE-008 — No Split-Authority Invention Rule
Document C must not invent a third authority source where Documents A and B already define the meaning.

---

### 3.4 Required Traceability Targets

#### CTRACE-009 — Minimum Traceability Target Rule
Every major technical artifact in Document C should trace to one or more of the following upstream target classes:

1. product-constitution target  
2. behavior target  
3. state/lifecycle target  
4. architecture target  
5. data-policy target  
6. interface-contract target  
7. report/output target  
8. acceptance/validation target  
9. execution/dependency target  
10. AI-agent guardrail target  

#### CTRACE-010 — Product-Constitution Target Definition
A product-constitution target is an upstream rule that defines the product’s fixed scope, non-negotiable constraints, or governing intent.

#### CTRACE-011 — Behavior Target Definition
A behavior target is an upstream rule that defines what the system must do, must not do, or how it must behave under governed conditions.

#### CTRACE-012 — State/Lifecycle Target Definition
A state/lifecycle target is an upstream rule that defines states, transitions, terminal paths, invalid transitions, or lifecycle truth.

#### CTRACE-013 — Architecture Target Definition
An architecture target is an upstream rule that defines technical ownership boundaries or layer responsibilities.

#### CTRACE-014 — Data-Policy Target Definition
A data-policy target is an upstream rule that defines allowed persistence, disallowed persistence, cleanup scope, secret handling, or retention boundaries.

#### CTRACE-015 — Interface-Contract Target Definition
An interface-contract target is an upstream rule that defines local or remote interface behavior, route meaning, or status/event/report access meaning.

#### CTRACE-016 — Report/Output Target Definition
A report/output target is an upstream rule that defines report structure, output meaning, availability conditions, or export boundaries.

#### CTRACE-017 — Acceptance/Validation Target Definition
An acceptance/validation target is an upstream rule that defines how success, validation, or release-readiness must be judged.

#### CTRACE-018 — Execution/Dependency Target Definition
An execution/dependency target is an upstream rule that defines sequencing, phase relevance, workstream ownership, dependency order, or validation-gate relevance.

#### CTRACE-019 — AI-Agent Guardrail Target Definition
An AI-agent guardrail target is an upstream rule that defines how AI coding agents must behave, what they must avoid, and when they must escalate.

---

### 3.5 Annex-Family-to-Upstream Mapping Rules

#### CTRACE-020 — Family Mapping Rule
Each annex family must have a predictable upstream traceability profile.

#### CTRACE-021 — Family A Mapping Rule
Annex Family A — Interface and API Contract Pack should primarily trace to:
- Document A interface behavior and contract meaning
- Document A architecture/interface sections
- Document B execution/validation context where interface timing or readiness matters

#### CTRACE-022 — Family B Mapping Rule
Annex Family B — Event, State, and Runtime Contract Pack should primarily trace to:
- Document A state/lifecycle truth
- Document A behavior rules
- Document B dependency and phase context
- Document B validation relevance where state/event truth is checked

#### CTRACE-023 — Family C Mapping Rule
Annex Family C — Data, Persistence, and Migration Pack should primarily trace to:
- Document A data lifecycle and storage policy
- Document A architecture boundaries
- Document B workstream and dependency context where persistence or cleanup relevance is execution-sensitive

#### CTRACE-024 — Family D Mapping Rule
Annex Family D — Report and Output Pack should primarily trace to:
- Document A report/output behavior
- Document A completion/finalization meaning
- Document B output-path phase context and validation context

#### CTRACE-025 — Family E Mapping Rule
Annex Family E — QA, Acceptance, and Red-Team Pack should primarily trace to:
- Document A acceptance criteria
- Document A known weakness/risk framing
- Document B validation and exit-gate logic
- Document B replan/escalation relevance where failure handling matters

#### CTRACE-026 — Family F Mapping Rule
Annex Family F — AI-Agent Reference Pack should primarily trace to:
- Document A AI-agent implementation guardrails
- Document B AI-agent execution protocol
- Document B task packet and drift-control structures where technical reference guidance must remain execution-safe

#### CTRACE-027 — Family G Mapping Rule
Annex Family G — Pack Index, Versioning, and Change Control should primarily trace to:
- Document C internal governance
- upstream revalidation rules from Documents A and B
- Document B change-control relevance where execution assumptions may be affected

---

### 3.6 Artifact-Level Traceability Rules

#### CTRACE-028 — Artifact Traceability Requirement
Every major artifact inside Document C should carry or imply enough traceability to identify:
- its annex family
- its upstream governing source(s)
- its technical purpose
- its boundary of authority

#### CTRACE-029 — Major Artifact Definition
Major artifacts include at minimum:
- endpoint contract families
- schema families
- error families
- event catalogs
- event payload schemas
- state tables
- transition tables
- migration sets
- table definitions
- report templates
- QA matrices
- AI-agent technical reference maps

#### CTRACE-030 — No Anonymous Artifact Rule
A major technical artifact must not appear in Document C as an anonymous standalone block with no clear upstream or family alignment.

#### CTRACE-031 — Minimal Traceability Fields Rule
Where a formal artifact record or table is used, the preferred minimum traceability fields are:
- artifact name
- annex family
- upstream source family
- upstream reference note
- technical purpose
- non-authority note where needed

---

### 3.7 Source-of-Truth Priority Rules

#### CTRACE-032 — Priority Resolution Rule
When multiple upstream sources are relevant, priority must be resolved according to the type of meaning in question.

#### CTRACE-033 — Product-Meaning Priority Rule
If the issue concerns product meaning, Document A has priority.

#### CTRACE-034 — Execution-Grouping Priority Rule
If the issue concerns execution order, dependency, phase context, or workstream placement, Document B has priority.

#### CTRACE-035 — Technical-Shape Priority Rule
If the issue concerns only technical formalization shape and does not affect upstream meaning, Document C may define the technical shape within its allowed authority.

#### CTRACE-036 — No Technical Override Rule
Technical shape must never be used to override upstream meaning.

---

### 3.8 Traceability for Change Impact and Revalidation

#### CTRACE-037 — Upstream Change Impact Rule
If Document A or Document B changes in a way that affects the meaning of a technical artifact in Document C, that artifact must be revalidated.

#### CTRACE-038 — Revalidation Trigger Classes
At minimum, revalidation must be triggered when upstream changes affect:
- interface behavior
- state or lifecycle meaning
- data retention or cleanup scope
- report structure or completion meaning
- acceptance logic
- execution-order-sensitive technical assumptions
- AI-agent technical guardrails

#### CTRACE-039 — No Stale-Annex Rule
A technical artifact must not remain active in Document C if its upstream meaning has changed materially and no revalidation has occurred.

#### CTRACE-040 — Change-Surface Mapping Rule
Where useful, Document C should identify which annex families are affected by specific upstream change classes.

---

### 3.9 Traceability for AI-Agent Use

#### CTRACE-041 — AI Traceability Rule
AI coding agents must be able to determine which upstream document family a technical definition depends on before implementing against it.

#### CTRACE-042 — No AI Orphan Consumption Rule
AI agents must not consume a technical artifact as self-justifying if its upstream grounding is absent or unclear.

#### CTRACE-043 — Escalation on Traceability Failure Rule
If an AI coding agent cannot determine whether a technical artifact is grounded in Document A or B, it must escalate rather than treating the artifact as independent authority.

#### CTRACE-044 — Traceability Visibility Rule
Document C should present traceability clearly enough that AI agents do not need to reconstruct the authority chain from memory.

---

### 3.10 Cross-Family Traceability Rules

#### CTRACE-045 — Cross-Family Mapping Rule
Annex families must be allowed to reference one another when technical meaning requires it.

#### CTRACE-046 — Cross-Family Reference Discipline
Cross-family references should clarify:
- which family owns the primary definition
- which family is consuming or relying on it
- whether the reference is normative or illustrative

#### CTRACE-047 — No Circular-Authority Rule
Cross-family reference must not create circular authority such that two technical artifacts appear to justify each other without upstream grounding.

#### CTRACE-048 — No Hidden Dependency Rule
If one annex artifact depends on another annex artifact, that dependency should be made explicit where omission would create ambiguity.

---

### 3.11 Traceability Integrity Prohibitions

#### CTRACE-049 — No Orphan Schema Rule
A schema must not exist without upstream justification.

#### CTRACE-050 — No Orphan Migration Rule
A migration must not exist without upstream justification.

#### CTRACE-051 — No Orphan Event Rule
An event type or payload family must not exist without upstream justification.

#### CTRACE-052 — No Orphan Matrix Rule
A QA or red-team matrix family must not exist without upstream justification.

#### CTRACE-053 — No Traceability-Washing Rule
A vague statement like “needed for implementation” is not sufficient traceability if the artifact affects product, execution, state, persistence, report, or validation meaning.

---

### 3.12 Traceability Maintenance Rules

#### CTRACE-054 — Maintenance Rule
Traceability in Document C must remain maintainable over time and must not require reverse-engineering the full drafting history.

#### CTRACE-055 — Stable Reference Rule
Traceability should reference stable upstream rule families and sections where possible rather than relying only on informal prose memory.

#### CTRACE-056 — No Over-Referencing Rule
Traceability should be sufficient and precise, but it should not become so dense that it obscures the technical artifact itself.

#### CTRACE-057 — Reviewability Rule
A reviewer should be able to inspect a technical annex family and determine whether it still aligns to Documents A and B without needing to reread the entire project history.

---

### 3.13 Boundaries of This Section

#### CTRACE-058 — Section Boundary Rule
This section defines:
- the source-of-truth model for Document C
- upstream traceability target classes
- family-level traceability rules
- artifact-level traceability rules
- change-impact and revalidation logic
- AI-agent traceability rules
- cross-family traceability discipline

This section does not yet define:
- the full naming convention system
- the detailed contracts of each annex family
- the versioning and change-log mechanics in full

Those are defined in later sections.

---

### 3.14 Final Traceability Summary

#### CTRACE-059 — Section Summary Rule
At the traceability level, Document C is governed by a dual-upstream model in which Document A defines product meaning and Document B defines execution context, while Document C formalizes that meaning into technical structures only when each major annex family and artifact can be traced back to clear upstream justification.

#### CTRACE-060 — Stability Rule
All later sections of Document C must remain consistent with this source-of-truth and traceability map unless revised explicitly through the Document C Change Log.

---

## 4. Canonical Terminology and Technical Naming Rules

### 4.1 Section Purpose
This section defines the canonical technical naming discipline used throughout Document C.

This section exists to ensure that:
- technical artifacts use stable and predictable names
- AI coding agents do not invent competing synonyms for the same governed concept
- human reviewers can cross-check technical definitions against Documents A and B without terminological drift
- schemas, endpoints, events, states, migrations, reports, and QA artifacts remain internally coherent

This section is subordinate to:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- Section 3 — Source-of-Truth and Traceability Map
- Document A — Master Project Creative Brief
- Document B — Master Execution Plan

If any naming or terminology rule described here conflicts with Documents A or B, the upstream document wins.

---

### 4.2 Role of Canonical Terminology in Document C

#### CTERM-001 — Canonical Terminology Rule
Document C must inherit its primary terminology from Documents A and B rather than inventing a parallel vocabulary.

#### CTERM-002 — Naming Determinism Rule
Technical naming must be deterministic enough that:
- one concept maps to one preferred technical label
- one artifact family maps to one predictable naming pattern
- AI coding agents can infer where a technical object belongs without guessing
- human reviewers can detect drift quickly

#### CTERM-003 — No Synonym Proliferation Rule
Document C must not normalize multiple competing names for the same governed concept unless upstream documents explicitly distinguish them.

#### CTERM-004 — Technical Precision Rule
Where upstream narrative terminology must be converted into technical naming, the conversion must preserve meaning rather than optimize for brevity alone.

---

### 4.3 Upstream Terminology Inheritance Rules

#### CTERM-005 — Document A Inheritance Rule
When a technical name depends on product meaning, behavior meaning, state meaning, report meaning, or acceptance meaning, the canonical upstream terminology must be inherited from Document A.

#### CTERM-006 — Document B Inheritance Rule
When a technical name depends on execution grouping, workstream relevance, phase relevance, dependency relevance, task-packet use, or validation/replan relevance, the canonical upstream terminology must be inherited from Document B.

#### CTERM-007 — No Terminology Override Rule
Document C must not replace upstream canonical terms merely because an alternative sounds more technical.

#### CTERM-008 — Narrow Technical Adaptation Rule
A technical label may adapt upstream wording for schema or contract purposes only when:
- the adaptation is structurally useful
- the original meaning is preserved
- ambiguity is reduced rather than increased
- the adapted term does not create a new governed concept

---

### 4.4 Canonical Concept Families

#### CTERM-009 — Concept Family Rule
Document C naming must be organized around stable concept families.

#### CTERM-010 — Required Concept Families
At minimum, Document C must maintain canonical terminology for:
1. document and authority concepts  
2. session/run concepts  
3. state/lifecycle concepts  
4. interface/endpoint concepts  
5. event concepts  
6. data/persistence concepts  
7. report/output concepts  
8. validation/QA concepts  
9. AI-agent technical-reference concepts  

#### CTERM-011 — No Family Blurring Rule
A term from one concept family must not be reused in a way that causes ambiguity in another family unless the shared meaning is intentional and safe.

---

### 4.5 Canonical Terms for Authority and Governance

#### CTERM-012 — Governing Document Terms
The canonical governing document terms are:
- **Document A** = Master Project Creative Brief
- **Document B** = Master Execution Plan
- **Document C** = Engineering Annex Pack

#### CTERM-013 — Authority Terms
The canonical authority terms are:
- **supreme source-of-truth**
- **subordinate execution-governance document**
- **technical annex**
- **governing upstream document**
- **live governing pair**
- **current live supporting files**

#### CTERM-014 — No Governance Synonym Drift Rule
Document C must not substitute vague labels such as:
- “core spec”
- “main document”
- “master file”
- “system law”
when the governed document identity matters and the canonical A/B/C naming is clearer.

---

### 4.6 Canonical Terms for Runtime and Lifecycle

#### CTERM-015 — Core Runtime Terms
The canonical runtime terms are:
- **run**
- **session**
- **workspace**
- **Preflight**
- **finalization**
- **cleanup**
- **startup janitor**
- **activation**
- **runtime hydration**
- **runtime integrity**

#### CTERM-016 — No Run/Session Collapse Rule
Document C must not collapse **run** and **session** into interchangeable technical synonyms if upstream documents distinguish them.

#### CTERM-017 — No Finalization/Completion Collapse Rule
Document C must not collapse **finalization**, **completion**, **constrained completion**, **failure**, and **cancellation** into one generic technical label.

#### CTERM-018 — No Cleanup/Outcome Collapse Rule
Document C must preserve the difference between:
- run outcome
- report availability
- cleanup attempt
- cleanup outcome

---

### 4.7 Canonical Terms for States

#### CTERM-019 — State Naming Rule
State names in Document C must remain semantically aligned with upstream lifecycle meaning.

#### CTERM-020 — State Label Stability Rule
If a state is defined in a table, schema, event payload, or interface contract, the state label must remain stable across all relevant annex families.

#### CTERM-021 — No Decorative State Rule
A state label must not be invented merely to make a UI or payload appear more expressive if upstream documents do not support that state meaning.

#### CTERM-022 — Terminal Classification Rule
Terminal classifications must preserve upstream distinctions between successful completion, constrained completion, cancellation, failure, cleanup result, and archival post-state where relevant.

---

### 4.8 Canonical Terms for Interfaces and Endpoints

#### CTERM-023 — Interface Naming Rule
Endpoint and interface names must be short, deterministic, and semantically aligned with their governed meaning.

#### CTERM-024 — Endpoint Verb Rule
Where reasonable, endpoint names should reflect the dominant technical action or retrieval meaning, such as:
- create
- validate
- start
- cancel
- get
- list
- export
- activate

#### CTERM-025 — No Behavior Inflation Rule
Interface names must not imply broader capability than the upstream documents authorize.

#### CTERM-026 — Resource Clarity Rule
Resource labels such as:
- preflight
- session
- status
- events
- report
- activation
- provider
should remain stable across route naming, schema naming, and technical reference material.

---

### 4.9 Canonical Terms for Events

#### CTERM-027 — Event Naming Rule
Event types must use stable names that identify:
- the event domain
- the event meaning
- the event granularity
without overloading the same label across unrelated domains.

#### CTERM-028 — Event Taxonomy Rule
Event names should distinguish between at minimum:
- lifecycle events
- moderation/debate events
- interruption/degradation events
- finalization/output events
- cleanup events
- activation/runtime/update events where applicable

#### CTERM-029 — No Event-State Confusion Rule
An event name must not be used as if it were a persistent state label unless upstream meaning explicitly makes that relationship clear.

---

### 4.10 Canonical Terms for Data and Persistence

#### CTERM-030 — Data Naming Rule
Data structure names must reflect whether the artifact is:
- transient
- persistent
- report-bound
- workspace-bound
- commercial-backend-bound
- secure-secret-bound
- archive/history-bound

#### CTERM-031 — Persistence Boundary Rule
Names for stored artifacts must not hide whether they are:
- local temporary workspace artifacts
- local persistent support records
- secure-store entries
- commercial backend records
- archived reports
- prohibited persistence classes

#### CTERM-032 — No Hidden-Persistence Naming Rule
Document C must not use neutral-sounding names that hide prohibited persistence or blur retention boundaries.

---

### 4.11 Canonical Terms for Reports and Outputs

#### CTERM-033 — Report Naming Rule
Report-related technical names must align with the governed report structure.

#### CTERM-034 — Report Section Stability Rule
The report section names used in templates and schemas should remain stable across:
- report templates
- generation contracts
- output field definitions
- validation matrices

#### CTERM-035 — No Output Inflation Rule
Output naming must not imply:
- richer scoring semantics
- broader export support
- extra report sections
- alternative report modes
unless upstream documents authorize them

---

### 4.12 Canonical Terms for QA and Validation

#### CTERM-036 — Validation Naming Rule
Technical validation names must remain aligned with upstream acceptance and execution validation meaning.

#### CTERM-037 — Gate Terminology Rule
The terms:
- **validation gate**
- **exit gate**
- **release gate**
- **acceptance mapping**
- **red-team matrix**
must remain distinct where their meanings differ.

#### CTERM-038 — No Completion Drift Rule
QA terminology must not soften unmet requirements by using optimistic substitute labels such as:
- mostly complete
- near pass
- effectively done
when the formal gate meaning is stricter

---

### 4.13 Canonical Terms for AI-Agent Reference Material

#### CTERM-039 — AI Reference Naming Rule
AI-agent-facing technical reference items must use names that make their authority scope obvious.

#### CTERM-040 — Non-Authority Label Rule
Where a technical note is illustrative, advisory, or non-authoritative, it should be labeled in a way that prevents it from being mistaken for a governing contract definition.

#### CTERM-041 — No Reference-Pack Override Rule
Reference-pack naming must not imply that AI-agent helper notes override schemas, contracts, or upstream documents.

---

### 4.14 Technical Naming Conventions by Artifact Type

#### CTERM-042 — Schema Naming Convention Rule
Schema families should use stable, human-readable names that clearly identify:
- domain
- payload type
- context
- direction where relevant

Preferred examples:
- `PreflightRequest`
- `PreflightResponse`
- `SessionStatusResponse`
- `ActivationRequest`
- `ReportMetadata`

#### CTERM-043 — Event Payload Naming Convention Rule
Event payload schemas should use stable names that identify the event family and payload role.

Preferred examples:
- `RunEventEnvelope`
- `RunLifecycleEventPayload`
- `DebateTurnEventPayload`
- `CleanupEventPayload`

#### CTERM-044 — Table Naming Convention Rule
Data table names should be explicit and domain-scoped.

Preferred style:
- lowercase snake_case for SQL table identifiers
- stable singular/plural policy applied consistently inside the family once chosen

#### CTERM-045 — Migration Naming Convention Rule
Migration names should identify:
- ordered sequence
- primary purpose
- affected domain
without pretending to be product law.

Preferred style example:
- `001_create_licenses_table`
- `002_add_device_activation_index`

#### CTERM-046 — QA Identifier Convention Rule
QA and red-team items should use stable deterministic IDs.

Preferred style examples:
- `QA-PREFLIGHT-001`
- `QA-ACTIVATION-003`
- `RT-CLEANUP-002`

---

### 4.15 Character, Case, and Formatting Rules

#### CTERM-047 — Case Discipline Rule
Document C should apply case consistently by artifact type.

Recommended defaults:
- section/rule titles: Title Case
- rule IDs: uppercase family prefix with numeric suffix
- schema names: PascalCase
- SQL identifiers: snake_case
- event type string values: stable lowercase or dot-scoped lowercase format, chosen once and applied consistently
- enum values: one stable casing strategy per enum family

#### CTERM-048 — No Mixed-Case Drift Rule
A single artifact family must not alternate unpredictably between naming styles.

#### CTERM-049 — ASCII Stability Rule
Where technical identifiers are intended for code or schema use, they should prefer plain ASCII-safe naming unless upstream constraints require otherwise.

---

### 4.16 Naming Integrity Prohibitions

#### CTERM-050 — No Scope-Washing Name Rule
A name must not make a limited feature sound broader than it is.

#### CTERM-051 — No Persistence-Washing Name Rule
A name must not make persistent data sound temporary if it is actually persistent.

#### CTERM-052 — No Failure-Washing Name Rule
A name must not make a failure, rejection, or degraded state sound like successful progress.

#### CTERM-053 — No Resume-Washing Name Rule
A technical name must not imply crash-resume or interrupted-run restoration where upstream rules prohibit that behavior.

#### CTERM-054 — No Hidden-Future-Scope Name Rule
A technical identifier must not quietly reserve active semantic meaning for deferred scope unless clearly marked as future-context only and non-authoritative.

---

### 4.17 Naming Maintenance Rules

#### CTERM-055 — Stable Vocabulary Rule
Once a canonical technical name is adopted, it should remain stable unless a change-log revision explicitly changes it.

#### CTERM-056 — Upstream Revalidation Rule
If an upstream term in Documents A or B changes materially, the affected technical names in Document C must be reviewed for alignment.

#### CTERM-057 — Local Consistency Review Rule
When new annex material is added, it must be checked against the existing naming system before being treated as governed.

#### CTERM-058 — No Casual Renaming Rule
Technical renaming must not be used casually during cleanup or refactoring because it can break traceability and AI-agent predictability.

---

### 4.18 Boundaries of This Section

#### CTERM-059 — Section Boundary Rule
This section defines:
- canonical terminology inheritance
- artifact naming discipline
- casing and identifier conventions
- naming integrity rules
- naming maintenance rules

This section does not yet define:
- the specific contract inventory of Annex Family A
- the specific state/event catalog of Annex Family B
- the specific table/migration inventory of Annex Family C
- the specific output template inventory of Annex Family D
- the specific QA matrix inventory of Annex Family E

Those are defined in later sections.

---

### 4.19 Final Terminology Summary

#### CTERM-060 — Section Summary Rule
At the terminology level, Document C requires a stable technical vocabulary and predictable artifact naming system that inherits canonical meaning from Documents A and B, reduces AI-agent ambiguity, and prevents technical naming from becoming a vehicle for scope, lifecycle, persistence, validation, or execution drift.

#### CTERM-061 — Stability Rule
All later sections of Document C must remain consistent with these canonical terminology and technical naming rules unless revised explicitly through the Document C Change Log.

---

## 5. Annex Family A — Interface and API Contract Pack

### 5.1 Section Purpose
This annex family defines the structured technical contract layer for local and remote interfaces used by the MVP.

This section exists to ensure that:
- interface behavior is formalized into deterministic technical contracts
- request/response shapes are explicit
- AI coding agents can implement interfaces without inventing route semantics
- human reviewers can verify that contract detail still matches upstream meaning

This annex family is subordinate to:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- Section 3 — Source-of-Truth and Traceability Map
- Section 4 — Canonical Terminology and Technical Naming Rules
- Document A interface, lifecycle, report, activation, and provider behavior rules
- Document B execution, phase, validation, and AI-agent context rules where interface timing matters

---

### 5.2 Family Role

#### CAPI-001 — Interface Contract Family Rule
Annex Family A is the primary technical family for interface and endpoint contract detail.

#### CAPI-002 — Family Purpose Rule
This family exists to formalize:
- local API surfaces
- backend commercial/activation interaction surfaces
- report access/export surfaces
- status and event retrieval surfaces
- provider and setup surfaces
- runtime/update interaction surfaces where such interfaces are contractually relevant

#### CAPI-003 — No Behavior Rewrite Rule
This family defines contract shape and interface semantics only.
It must not redefine product behavior or execution policy.

#### CAPI-004 — No Execution Rewrite Rule
This family may reflect execution-sensitive interface meaning, but it must not redefine phase order, dependency order, or gate logic.

---

### 5.3 Family Scope

#### CAPI-005 — In-Scope Rule
Annex Family A is in scope for:
- endpoint inventory
- endpoint purpose tables
- route naming conventions
- request schemas
- response schemas
- error response families
- success/failure classification at the contract level
- polling/retrieval contracts
- export/access contracts
- backend activation/license interaction contracts
- provider configuration interaction contracts
- update/runtime interaction contracts where contract formalization is needed

#### CAPI-006 — Out-of-Scope Rule
Annex Family A is out of scope for:
- state-machine formalization in full
- event catalog ownership in full
- database table ownership
- migration definitions
- report template ownership in full
- QA matrix ownership in full
- task packet design
- UI layout or component design

#### CAPI-007 — Primary Family Boundary Rule
If the primary question is “what route or interface contract shape does this use?”, Annex Family A owns it.
If the primary question is “what state/event meaning does this represent?”, Annex Family B owns the primary definition.

---

### 5.4 Interface Surface Categories

#### CAPI-008 — Surface Category Rule
Interface contracts in this family must be organized by stable surface category.

#### CAPI-009 — Required Interface Surface Categories
At minimum, the interface surface categories are:

1. local app/session control surfaces  
2. Preflight and seed intake surfaces  
3. run/session monitoring surfaces  
4. activation and licensing surfaces  
5. provider configuration surfaces  
6. report/output access surfaces  
7. runtime/update support surfaces  

#### CAPI-010 — No Surface Mixing Rule
Contracts from different surface categories must not be intermixed in a way that obscures:
- ownership
- permission context
- lifecycle relevance
- upstream meaning

---

### 5.5 Endpoint Inventory Rules

#### CAPI-011 — Endpoint Inventory Requirement
This family must maintain an endpoint inventory or equivalent contract index.

#### CAPI-012 — Minimum Endpoint Inventory Fields
For each endpoint or interface contract family, the preferred minimum inventory fields are:
- endpoint name
- route or interface label
- primary purpose
- surface category
- caller context
- request schema family
- response schema family
- error family
- upstream traceability note

#### CAPI-013 — No Anonymous Endpoint Rule
A meaningful endpoint or interface surface must not appear in Document C without an identifiable place in the contract inventory.

---

### 5.6 Canonical Interface Contract Groups

#### CAPI-014 — Canonical Contract Group Rule
The canonical interface contract groups for the MVP are:

1. activation and entitlement contracts  
2. provider configuration contracts  
3. Preflight contracts  
4. run/session start and cancel contracts  
5. session status contracts  
6. event retrieval contracts  
7. report access and export contracts  
8. runtime/update support contracts  

#### CAPI-015 — Stable Contract Group Rule
These groups should remain stable unless changed explicitly through the Document C Change Log.

#### CAPI-016 — No Casual Contract Group Invention Rule
New contract groups must not be added casually unless a real governed surface exists upstream.

---

### 5.7 Activation and Entitlement Contract Rules

#### CAPI-017 — Activation Contract Rule
Activation and entitlement contracts must formalize:
- activation request shape
- activation response shape
- denial/error distinction
- device-binding context
- token/license validation response shape where applicable

#### CAPI-018 — No Activation Drift Rule
Activation contracts must not imply:
- unlimited device usage
- broader offline rights than upstream documents authorize
- silent fallback from denial to success
- authentication/entitlement semantics broader than the commercial backend supports

---

### 5.8 Provider Configuration Contract Rules

#### CAPI-019 — Provider Contract Rule
Provider configuration contracts must formalize:
- provider selection request/response structure
- key-submission request/response structure
- configuration status retrieval shape where relevant
- missing-vs-present configuration distinction

#### CAPI-020 — No Secret Leakage Contract Rule
No provider contract example, schema, or error body may normalize exposing secrets in plaintext response bodies, logs, or non-secure contexts.

#### CAPI-021 — No Run-Readiness Inflation Rule
Provider configuration contracts must not imply that “configuration present” automatically means “run eligible” when Preflight is still required.

---

### 5.9 Preflight Contract Rules

#### CAPI-022 — Preflight Contract Rule
Preflight contracts must formalize:
- seed intake request shape
- supported MVP seed-document constants: PDF, DOCX, TXT, and Markdown only; maximum file size 10 MiB (10,485,760 bytes)
- validation result response shape
- document-validation rejection families for unsupported file type, file too large, text extraction failed, insufficient extractable text, and non-English seed document
- minimum usable extracted-text threshold of 1,000 normalized characters
- non-English rejection threshold at English-confidence below 0.80
- approved / approved-with-downgrade / rejected distinction
- requested-plan vs effective-plan fields
- rejection/error distinction
- stale-result invalidation semantics where they appear at the interface layer

#### CAPI-023 — No Preflight Bypass Rule
No Preflight contract should imply that run start is valid without an eligible Preflight outcome.

#### CAPI-024 — No Rejection/Error Collapse Rule
Preflight contracts must preserve the difference between governed rejection and technical failure.

---

### 5.10 Run and Session Contract Rules

#### CAPI-025 — Run/Session Contract Rule
Run/session contracts must formalize:
- session creation/start request/response shape
- cancellation request/response shape
- status retrieval shape
- session identity fields
- run identity fields where distinct
- report linkage fields where applicable

#### CAPI-026 — No Resume Implication Rule
Run/session contracts must not imply crash-resume or interrupted-run restoration when upstream rules prohibit it.

#### CAPI-027 — No State Inflation Rule
Run/session contract fields must not imply lifecycle states or transitions that upstream documents do not govern.

---

### 5.11 Status and Event Retrieval Contract Rules

#### CAPI-028 — Status Contract Rule
Status contracts must formalize the technical shape of live status retrieval without replacing the authoritative state/event meaning owned by Annex Family B.

#### CAPI-029 — Event Retrieval Contract Rule
Event retrieval contracts must formalize:
- event retrieval request shape
- pagination or sequence semantics where applicable
- ordered retrieval semantics
- event envelope structure at the interface level

#### CAPI-030 — No Event-Meaning Override Rule
The event retrieval contract may define interface shape but must not redefine event-type meaning, which belongs primarily to Annex Family B.

---

### 5.12 Report Access and Export Contract Rules

#### CAPI-031 — Report Contract Rule
Report access and export contracts must formalize:
- report metadata retrieval shape
- report availability representation
- report file access/export request shape
- failure cases for unavailable or missing outputs

#### CAPI-032 — No Completion/Availability Collapse Rule
Report contracts must preserve the distinction between:
- run completion classification
- report generation success
- report availability
- export accessibility

#### CAPI-033 — No Output Inflation Rule
Report contracts must not imply additional output modes or report variants not governed upstream.

---

### 5.13 Runtime and Update Support Contract Rules

#### CAPI-034 — Runtime/Update Contract Rule
Where contract formalization is required, runtime and update support contracts must formalize:
- shell-owned managed runtime-package hydration authorization and trigger shape
- native app-updater status and action shape where update behavior is exposed
- runtime/update status retrieval shape
- artifact availability signals where applicable
- version-record and trust-manifest-related verification fields where upstream documents require them
- separation from active-run mutation semantics

#### CAPI-035 — No Resume Drift Rule
Runtime/download recovery contracts must not be mislabeled in a way that implies run/session resume.

#### CAPI-036 — No Unsafe Artifact Implication Rule
Runtime/update contracts must not imply use of unverified artifacts as normal or acceptable behavior.

---

### 5.14 Request Schema Rules

#### CAPI-037 — Request Schema Requirement
Every meaningful interface contract group should define stable request schema families where requests are not trivially empty.

#### CAPI-038 — Request Minimalism Rule
Request schemas should include only the fields necessary for the governed contract behavior and must not speculate future scope into current MVP requests.

#### CAPI-039 — No Hidden-Scope Field Rule
Request fields must not quietly introduce future capabilities or options that upstream documents do not authorize.

#### CAPI-040 — Required Field Clarity Rule
Required versus optional fields must be explicit in contract definitions.

---

### 5.15 Response Schema Rules

#### CAPI-041 — Response Schema Requirement
Every meaningful interface contract group should define stable response schema families.

#### CAPI-042 — Response Truth Rule
Responses must communicate governed outcomes truthfully and must not blur:
- success
- constrained success
- rejection
- denial
- technical error
- unavailable state

#### CAPI-043 — Response Minimalism Rule
Response schemas should not contain speculative or decorative fields that create apparent semantics unsupported upstream.

#### CAPI-044 — Structured Outcome Rule
Where relevant, response schemas should include explicit structured outcome fields rather than forcing the consumer to infer meaning from prose alone.

---

### 5.16 Error and Failure Contract Rules

#### CAPI-045 — Error Family Requirement
Document C should define stable error family structures for interface contracts.

#### CAPI-046 — Failure Clarity Rule
Error contracts must preserve distinctions among:
- technical failure
- rejection
- denial
- unavailable state
- invalid state for requested action

#### CAPI-047 — No Failure-Washing Rule
Error payloads must not use language or structure that makes failure appear as successful forward progress.

#### CAPI-048 — Safe Error Detail Rule
Error detail should aid technical diagnosis without exposing prohibited secrets or broader backend internals than are safe and necessary.

---

### 5.17 Contract Inventory and Cross-Family Dependencies

#### CAPI-049 — Interface Ownership Rule
Annex Family A owns interface shape, route grouping, and request/response structure.

#### CAPI-050 — State Dependency Rule
Where response payloads expose states, those state values must depend on Annex Family B’s owned definitions.

#### CAPI-051 — Data Dependency Rule
Where interface contracts reference persistent records or cleanup/report boundaries, the relevant persistence meaning must remain aligned with Annex Family C.

#### CAPI-052 — Output Dependency Rule
Where interface contracts reference reports or exports, the report/output structures must remain aligned with Annex Family D.

#### CAPI-053 — Validation Dependency Rule
Where interface contracts are validated through QA matrices, the relevant validation structure must remain aligned with Annex Family E.

---

### 5.18 Contract Integrity Prohibitions

#### CAPI-054 — No Contract-Led Scope Expansion Rule
Interface definitions must not be used to widen the MVP beyond governed scope.

#### CAPI-055 — No Hidden Endpoint Rule
A materially meaningful endpoint must not exist in implementation planning without a corresponding governed contract family presence in this annex.

#### CAPI-056 — No Fake-Success Contract Rule
A response contract must not structurally frame a blocked, denied, rejected, or failed condition as ordinary success.

#### CAPI-057 — No Authority Drift Rule
This annex family must not be used to reinterpret lifecycle, validation, cleanup, reporting, or activation truth beyond what upstream documents authorize.

---

### 5.19 Boundaries of This Section

#### CAPI-058 — Section Boundary Rule
This section defines:
- the role and boundaries of the interface and API contract family
- the canonical contract groups
- endpoint inventory rules
- request/response/error schema discipline
- cross-family dependency rules for interfaces

This section does not yet define:
- the canonical state/event catalog in full
- the database and migration structures
- the report template structures in full
- the QA matrix structures in full
- the AI-agent technical-reference pack in full

Those are defined in later sections.

---

### 5.20 Final Interface Pack Summary

#### CAPI-059 — Section Summary Rule
At the interface-contract level, Annex Family A formalizes the local and remote technical contract surface of the governed MVP through stable endpoint inventories, request/response/error schema families, and cross-family-aligned interface structures without changing product behavior, lifecycle truth, execution order, storage policy, report meaning, or validation meaning.

#### CAPI-060 — Stability Rule
All later sections of Document C must remain consistent with this interface and API contract pack unless revised explicitly through the Document C Change Log.

---

## 6. Annex Family B — Event, State, and Runtime Contract Pack

### 6.1 Section Purpose
This annex family defines the structured technical reference layer for lifecycle states, runtime states, event families, event payload structures, transition legality, and runtime-classified execution truth.

This section exists to ensure that:
- state and lifecycle meaning are formalized into deterministic technical structures
- event families and event payloads are explicit and reviewable
- AI coding agents do not invent new runtime states, event types, or transition meanings
- interface contracts, data handling, report generation, and QA matrices remain aligned to the same runtime truth

This annex family is subordinate to:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- Section 3 — Source-of-Truth and Traceability Map
- Section 4 — Canonical Terminology and Technical Naming Rules
- Document A lifecycle, behavior, cleanup, finalization, and interruption truth
- Document B dependency, phase, validation, and AI-agent context where runtime truth is execution-relevant

---

### 6.2 Family Role

#### CEVENT-001 — Runtime Contract Family Rule
Annex Family B is the primary technical family for lifecycle/state/event/runtime meaning.

#### CEVENT-002 — Family Purpose Rule
This family exists to formalize:
- canonical state sets
- allowed and forbidden transitions
- terminal classification logic
- event type catalogs
- event envelope structures
- event payload families
- sequence and ordering rules
- interruption and degraded-state technical definitions
- cleanup-state technical definitions
- runtime hydration and updater-adjacent runtime states where upstream rules require them

#### CEVENT-003 — No Lifecycle Rewrite Rule
This family defines technical formalization of governed lifecycle meaning only.
It must not redefine the lifecycle itself.

#### CEVENT-004 — No Interface Ownership Rewrite Rule
This family may define event and state payload meaning, but it does not own the request/response surface contracts of Annex Family A.

---

### 6.3 Family Scope

#### CEVENT-005 — In-Scope Rule
Annex Family B is in scope for:
- run/session state enumerations
- transition tables
- terminal state classification
- runtime status domains
- event taxonomy
- event envelope definitions
- event payload schemas
- sequence and replay rules
- degradation/interruption state definitions
- finalization state definitions
- cleanup state definitions
- startup janitor-discovered residual-state classifications where technically needed

#### CEVENT-006 — Out-of-Scope Rule
Annex Family B is out of scope for:
- route surface ownership
- database table ownership
- migration ownership
- report template ownership in full
- QA ownership in full
- UI presentation design
- product-scope decisions about adding or removing lifecycle features

#### CEVENT-007 — Primary Family Boundary Rule
If the primary question is “what state/event does this mean and how does it transition?”, Annex Family B owns it.
If the primary question is “what endpoint exposes it?”, Annex Family A owns the primary contract.

---

### 6.4 Canonical Runtime Domains

#### CEVENT-008 — Runtime Domain Rule
Runtime truth in this family must be organized into stable runtime domains.

#### CEVENT-009 — Required Runtime Domains
At minimum, the runtime domains are:

1. app/runtime readiness domain  
2. activation/readiness domain  
3. Preflight domain  
4. run/session lifecycle domain  
5. event-stream domain  
6. finalization/output domain  
7. cleanup/janitor domain  
8. runtime hydration/update support domain where applicable  

#### CEVENT-010 — No Domain Blurring Rule
States and events from different runtime domains must not be intermixed in a way that hides:
- ownership
- lifecycle stage
- user-visible consequence
- terminal meaning
- technical boundary relevance

---

### 6.5 Canonical State System Rules

#### CEVENT-011 — State System Requirement
This family must maintain a canonical state system or equivalent structured state inventory.

#### CEVENT-012 — Minimum State Inventory Fields
For each state or runtime-state family, the preferred minimum fields are:
- state name
- parent runtime domain
- plain-language meaning
- entry conditions
- allowed outgoing transitions
- forbidden outgoing transitions
- terminal classification
- upstream traceability note

#### CEVENT-013 — No Anonymous State Rule
A meaningful runtime state must not appear in implementation planning without a canonical place in this family’s state inventory.

---

### 6.6 Canonical Run/Session Lifecycle States

#### CEVENT-014 — Canonical Run Lifecycle Rule
The run/session lifecycle state family must preserve the governed run path without flattening it into generic “processing” semantics.

#### CEVENT-015 — Required Run Lifecycle Distinctions
At minimum, the run/session state family must preserve distinctions among:
- run/session not created
- eligible to start vs not eligible to start
- actively running
- degraded/interrupted but not terminal
- cancellation in progress
- finalization in progress
- completed
- constrained completion
- failed
- canceled

#### CEVENT-016 — No Completion Collapse Rule
The state system must not collapse:
- completed
- constrained completion
- failed
- canceled
into one generic terminal success/failure shorthand if upstream truth distinguishes them.

#### CEVENT-017 — No Resume State Rule
No state in this family may imply crash-resume or interrupted-run restoration where upstream documents prohibit that behavior.

---

### 6.7 Preflight State Rules

#### CEVENT-018 — Preflight State Rule
The Preflight state family must formalize the technical states relevant to:
- not yet run
- in progress
- approved
- approved with downgrade
- rejected
- technically failed
- stale/invalidated after material input change

#### CEVENT-019 — No Preflight Outcome Collapse Rule
This family must preserve the distinction among:
- approval
- approval with downgrade
- rejection
- technical failure
- stale invalidation

#### CEVENT-020 — No Run-Eligibility Inflation Rule
A Preflight-approved state must not be technically defined in a way that bypasses other governed run-start requirements.

---

### 6.8 App/Runtime Readiness State Rules

#### CEVENT-021 — Readiness State Rule
The app/runtime readiness state family must formalize the technical states relevant to:
- runtime absent
- runtime hydrating
- runtime ready
- runtime integrity failed
- activation absent/invalid
- provider configuration missing
- locally ready for Preflight
- not ready for Preflight

#### CEVENT-022 — No Readiness Conflation Rule
This family must not collapse:
- runtime readiness
- activation readiness
- provider readiness
- Preflight eligibility
into one vague “ready” state.

#### CEVENT-023 — No Decorative Readiness Rule
A readiness state must correspond to a real technical gate, not a UI decoration.

---

### 6.9 Finalization and Output State Rules

#### CEVENT-024 — Finalization State Rule
The finalization/output family must formalize the technical distinctions relevant to:
- finalization entered
- finalization succeeded
- report generation in progress
- report generation succeeded
- report generation failed
- report available
- report unavailable

#### CEVENT-025 — No Finalization/Output Collapse Rule
This family must preserve the distinction between:
- entering finalization
- finalization success
- report generation success
- report availability

#### CEVENT-026 — No Output Inflation Rule
Finalization/output states must not imply extra output modes or alternate report systems not governed upstream.

---

### 6.10 Cleanup and Janitor State Rules

#### CEVENT-027 — Cleanup State Rule
The cleanup/janitor family must formalize the technical distinctions relevant to:
- cleanup pending
- cleanup running
- cleanup succeeded
- cleanup partially failed where classification exists
- cleanup failed
- janitor pending next launch
- janitor running
- janitor purge succeeded
- janitor purge failed

#### CEVENT-028 — No Cleanup/Run Outcome Collapse Rule
Cleanup states must not be used as substitutes for run outcome states.

#### CEVENT-029 — No Janitor-Resume Rule
Janitor-related states must not imply run restoration or crash-resume semantics.

---

### 6.11 Allowed Transition Rules

#### CEVENT-030 — Transition Table Requirement
This family must define allowed transition tables or equivalent transition references for all major runtime domains.

#### CEVENT-031 — Transition Table Minimum Fields
The preferred minimum transition fields are:
- from state
- to state
- transition class
- guard/precondition
- allowed/prohibited flag
- terminal impact note
- upstream traceability note

#### CEVENT-032 — No Implicit Transition Rule
A materially meaningful transition must not be left implicit if omission would create implementation ambiguity.

#### CEVENT-033 — Guarded Transition Rule
Where a transition depends on a gate or prerequisite, the transition rule must identify that guard clearly.

---

### 6.12 Forbidden Transition Rules

#### CEVENT-034 — Forbidden Transition Requirement
This family should identify forbidden transitions where omission would create unsafe implementation drift.

#### CEVENT-035 — Required Forbidden Transition Classes
At minimum, forbidden transition logic must prevent:
- run start without required eligibility
- terminal completion without finalization/report truth
- report available without generation success
- canceled run being treated as completed
- failed run being treated as constrained completion
- cleanup success being treated as run success
- janitor recovery being treated as resume

#### CEVENT-036 — No Silent Shortcut Rule
A transition that would violate upstream lifecycle truth must be explicitly forbidden where likely implementation ambiguity exists.

---

### 6.13 Event System Rules

#### CEVENT-037 — Event Catalog Requirement
This family must maintain a canonical event catalog or equivalent event taxonomy reference.

#### CEVENT-038 — Event Catalog Minimum Fields
For each event type, the preferred minimum fields are:
- event type name
- runtime domain
- event purpose
- payload schema family
- ordering/sequence relevance
- user-visibility relevance
- upstream traceability note

#### CEVENT-039 — No Anonymous Event Rule
A materially meaningful runtime event must not appear in implementation planning without a place in the event catalog.

---

### 6.14 Canonical Event Taxonomy

#### CEVENT-040 — Event Taxonomy Rule
The canonical event taxonomy for the MVP must distinguish at minimum:

1. lifecycle events  
2. moderation/debate events  
3. interruption/degradation events  
4. finalization/output events  
5. cleanup/janitor events  
6. activation/runtime/update support events where technically relevant  

#### CEVENT-041 — No Cross-Taxonomy Blur Rule
Event types from different categories must not share ambiguous names that obscure their runtime domain or purpose.

#### CEVENT-042 — Debate Event Rule
Moderation/debate events must be able to represent at minimum:
- moderator-driven round progression
- selected-speaker activity
- strongest argument/risk material where emitted incrementally
without implying all-agent fanout as a normal event model

---

### 6.15 Event Envelope Rules

#### CEVENT-043 — Event Envelope Requirement
This family should define a canonical event envelope structure for runtime event transport and storage-safe representation where applicable.

#### CEVENT-044 — Minimum Event Envelope Fields
The preferred minimum event envelope fields are:
- event_id
- sequence_number
- occurred_at
- session_id
- event_type
- payload
- runtime_domain
- correlation/reference fields where needed

#### CEVENT-045 — Ordering Field Requirement
Event envelopes must contain enough ordered reference information to support deterministic retrieval and UI synchronization.

#### CEVENT-046 — No Envelope Inflation Rule
The event envelope must not carry speculative fields that create new semantics unsupported upstream.

---

### 6.16 Event Payload Family Rules

#### CEVENT-047 — Payload Family Requirement
Each major event category should have stable payload schema families.

#### CEVENT-048 — Payload Meaning Rule
Payload schemas must communicate governed event meaning without requiring consumers to infer critical distinctions from prose alone.

#### CEVENT-049 — No State/Event Collapse Rule
Event payloads may reference state changes, but an event payload must not redefine the canonical state system independently.

#### CEVENT-050 — No Secret or Unsafe Detail Rule
Event payloads must not normalize disclosure of secrets, prohibited persistence detail, or unsafe internals unrelated to the governed event meaning.

---

### 6.17 Event Ordering and Retrieval Semantics

#### CEVENT-051 — Deterministic Ordering Rule
This family must define deterministic event ordering semantics sufficient for:
- incremental retrieval
- live monitoring
- transcript chronology
- downstream report appendix generation where applicable

#### CEVENT-052 — Sequence Integrity Rule
The sequence system must allow consumers to determine:
- what came before
- what came after
- whether gaps exist
- whether incremental retrieval boundaries are valid

#### CEVENT-053 — No Ambiguous Replay Rule
Replay or retrieval semantics must not leave consumers unable to distinguish missing history from complete ordered history where that distinction matters.

---

### 6.18 Degradation and Interruption Rules

#### CEVENT-054 — Degradation Event Rule
This family must define runtime event/state distinctions for governed degraded conditions, including at minimum:
- network/provider interruption
- rate-limit or temporary provider unavailability
- other governed pause/degrade conditions exposed to users

#### CEVENT-055 — No Healthy/Degraded Collapse Rule
Degraded or paused conditions must not be technically represented as ordinary healthy progress.

#### CEVENT-056 — No Implicit Recovery Rule
Recovery from degraded state must remain explicit and must not imply hidden lost-state reconstruction where upstream rules prohibit it.

---

### 6.19 Cross-Family Dependencies

#### CEVENT-057 — Interface Dependency Rule
Where states and events are exposed through interfaces, Annex Family A must consume the definitions owned here rather than reauthor them.

#### CEVENT-058 — Data Dependency Rule
Where events or states imply persistence, cleanup, report preservation, or migration relevance, the applicable persistence meaning must remain aligned with Annex Family C.

#### CEVENT-059 — Output Dependency Rule
Where finalization or completion events feed output/report meaning, the applicable output structure must remain aligned with Annex Family D.

#### CEVENT-060 — QA Dependency Rule
Where states/events are validated through QA or red-team matrices, Annex Family E must consume the definitions owned here rather than redefine them.

---

### 6.20 Runtime Integrity Prohibitions

#### CEVENT-061 — No Fake-Completion Event Rule
An event or state must not frame incomplete or failed output as ordinary successful completion.

#### CEVENT-062 — No Resume-Washing Rule
Event/state structures must not disguise prohibited resume behavior as hydration recovery, janitor recovery, or continuity semantics.

#### CEVENT-063 — No Hidden Terminal Drift Rule
A terminal classification must not silently change meaning across families or contexts.

#### CEVENT-064 — No Schema-Led Lifecycle Mutation Rule
Payload design must not be used to widen or alter lifecycle meaning beyond upstream governance.

---

### 6.21 Boundaries of This Section

#### CEVENT-065 — Section Boundary Rule
This section defines:
- the runtime/event/state family role
- state inventories
- transition discipline
- event taxonomy
- event envelopes
- payload family rules
- ordering semantics
- degradation and interruption formalization
- cross-family runtime dependencies

This section does not yet define:
- interface route contracts in full
- persistence and migration structures in full
- report template structures in full
- QA matrix structures in full
- AI-agent technical reference structures in full

Those are defined in later sections.

---

### 6.22 Final Runtime Contract Summary

#### CEVENT-066 — Section Summary Rule
At the runtime-contract level, Annex Family B formalizes the governed lifecycle, state, event, terminal-classification, and ordering truth of the MVP into deterministic technical structures that downstream interfaces, persistence rules, report generation, and QA matrices must consume without redefining.

#### CEVENT-067 — Stability Rule
All later sections of Document C must remain consistent with this event, state, and runtime contract pack unless revised explicitly through the Document C Change Log.

---

## 7. Annex Family C — Data, Persistence, and Migration Pack

### 7.1 Section Purpose
This annex family defines the structured technical reference layer for data classes, persistence boundaries, local and backend record structures, migration families, retention boundaries, and cleanup-relevant storage meaning.

This section exists to ensure that:
- persistence and storage behavior are formalized deterministically
- AI coding agents do not introduce hidden persistence classes
- local, secure, transient, report-bound, and backend-bound data are clearly separated
- migration detail remains subordinate to the governed data lifecycle and storage policy

This annex family is subordinate to:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- Section 3 — Source-of-Truth and Traceability Map
- Section 4 — Canonical Terminology and Technical Naming Rules
- Document A storage, privacy, cleanup, report-preservation, and secret-handling truth
- Document B workstream, dependency, validation, and cleanup execution context where persistence relevance is execution-sensitive

---

### 7.2 Family Role

#### CDATA-001 — Data Pack Rule
Annex Family C is the primary technical family for data classes, persistence boundaries, record structures, and migration formalization.

#### CDATA-002 — Family Purpose Rule
This family exists to formalize:
- what data classes exist
- whether each class is transient, persistent, secure-store-bound, backend-bound, or prohibited
- which data structures belong to local support storage versus commercial backend storage
- what cleanup applies to which data classes
- what migration families are authorized
- how report preservation boundaries differ from transient workspace cleanup

#### CDATA-003 — No Data-Policy Rewrite Rule
This family defines technical formalization of governed data-policy truth only.
It must not redefine retention, deletion, secrecy, privacy, or persistence rights.

#### CDATA-004 — No Architecture Rewrite Rule
This family may formalize data structures and migration boundaries, but it must not reassign architectural ownership among shell, engine, secure-store, local DB, and commercial backend layers.

---

### 7.3 Family Scope

#### CDATA-005 — In-Scope Rule
Annex Family C is in scope for:
- data class inventory
- persistence boundary tables
- local support data structures
- secure-store-referenced data structures
- backend commercial table definitions
- migration inventories
- migration ordering rules
- cleanup-scope tables
- report-preservation boundary tables
- prohibited persistence classes
- retention boundary notes where governed upstream

#### CDATA-006 — Out-of-Scope Rule
Annex Family C is out of scope for:
- redefining cleanup behavior
- redefining run/session lifecycle meaning
- redefining interface route behavior
- owning report template content
- owning QA matrix logic in full
- inventing new stored analytics or telemetry classes

#### CDATA-007 — Primary Family Boundary Rule
If the primary question is “what data exists, where may it live, and how may it migrate or be cleaned up?”, Annex Family C owns it.
If the primary question is “what event/state causes the cleanup?”, Annex Family B owns the primary runtime definition.

---

### 7.4 Data Class Inventory Rules

#### CDATA-008 — Data Class Inventory Requirement
This family must maintain a canonical data class inventory or equivalent structured data map.

#### CDATA-009 — Minimum Data Class Fields
For each data class, the preferred minimum fields are:
- data class name
- parent storage domain
- allowed location(s)
- prohibited location(s)
- persistence classification
- cleanup applicability
- retention boundary note
- upstream traceability note

#### CDATA-010 — No Anonymous Data Class Rule
A meaningful data class must not exist in implementation planning without a clear place in this family’s data inventory.

---

### 7.5 Canonical Storage Domains

#### CDATA-011 — Storage Domain Rule
This family must organize technical data structures into stable storage domains.

#### CDATA-012 — Required Storage Domains
At minimum, the storage domains are:

1. transient workspace domain  
2. local persistent support domain  
3. OS-native secure storage domain  
4. report/export preservation domain  
5. commercial backend domain  
6. prohibited persistence domain  

#### CDATA-013 — No Domain Collapse Rule
These storage domains must not be blurred in a way that hides:
- whether data survives app close
- whether data survives crash
- whether data is secure-store protected
- whether data is cleanup-targeted
- whether data is explicitly preserved as a report/export artifact
- whether persistence is forbidden entirely

---

### 7.6 Transient Workspace Data Rules

#### CDATA-014 — Workspace Data Rule
Transient workspace data is the technical class for run-bounded local material that exists to support active or just-completed run execution and is subject to cleanup or janitor purge.

#### CDATA-015 — Workspace Data Examples Rule
Where upstream documents authorize the class, transient workspace data may include:
- extracted seed text
- transient debate material
- transient transcript material
- transient report assembly intermediates
- temporary vector/index artifacts
- run-bounded runtime working files

#### CDATA-016 — No Workspace Persistence Inflation Rule
Workspace data must not be silently reclassified as durable local history or product memory.

#### CDATA-017 — Cleanup Applicability Rule
Transient workspace classes must be clearly marked as cleanup-targeted unless an upstream exception exists.

---

### 7.7 Local Persistent Support Data Rules

#### CDATA-018 — Local Support Data Rule
Local persistent support data is the technical class for governed local records that may persist across launches because they are required for app operation, entitlement handling, settings continuity, or report catalog/access support.

#### CDATA-019 — Allowed Local Support Examples
Where upstream documents authorize them, local persistent support data may include:
- activation validation support records
- stable provider selection metadata
- report metadata/index references
- updater support metadata
- non-sensitive app support settings

#### CDATA-020 — No Hidden History Rule
Local persistent support storage must not be used to create undeclared historical archives of runs, transcripts, seed content, or debate memory.

#### CDATA-021 — No Secret Misclassification Rule
Secrets must not be placed into this domain if upstream documents require secure-store handling instead.

---

### 7.8 OS-Native Secure Storage Rules

#### CDATA-022 — Secure Storage Rule
OS-native secure storage is the required technical domain for secret-bearing or secret-adjacent material that upstream documents authorize for secure persistence.

#### CDATA-023 — Allowed Secure-Store Examples
Where upstream documents authorize them, secure-store entries may include:
- provider API keys
- device-bound activation tokens
- other secret-bearing credentials explicitly authorized upstream

#### CDATA-024 — No Plaintext Fallback Rule
Secure-store material must not be silently mirrored into plaintext config, workspace files, or logs as a convenience fallback.

#### CDATA-025 — Secure-Store Boundary Rule
This family must identify clearly which fields are:
- stored in secure storage
- referenced indirectly outside secure storage
- never exposed in ordinary payloads or logs

---

### 7.9 Report and Export Preservation Rules

#### CDATA-026 — Report Preservation Rule
The report/export preservation domain is the technical class for final user-facing output artifacts that are intentionally preserved outside transient workspace cleanup scope.

#### CDATA-027 — Required Report Distinction Rule
This family must preserve the distinction between:
- report generation intermediates inside transient workspace
- final exported or preserved report artifacts outside cleanup scope
- report metadata/index references in local persistent support storage where governed upstream

#### CDATA-028 — No Report/Workspace Collapse Rule
Final output artifacts must not be treated as ordinary workspace garbage if upstream documents require their preservation.

---

### 7.10 Commercial Backend Data Rules

#### CDATA-029 — Commercial Backend Rule
The commercial backend domain is the technical class for upstream-authorized backend records required for licensing, activation, device-slot enforcement, entitlement truth, and other explicitly governed commercial support behavior.

#### CDATA-030 — Allowed Commercial Examples
Where upstream documents authorize them, backend commercial records may include:
- license records
- device activation records
- device-slot occupancy records
- commercial audit/support references needed for entitlement handling

#### CDATA-031 — No Run-Persistence Backend Rule
The commercial backend domain must not be used as a hidden run-history, transcript, simulation-state, or workspace-storage backend unless upstream documents explicitly authorize such behavior, which they do not for MVP.

#### CDATA-032 — No Semantic Overflow Rule
Commercial tables must not quietly absorb report, run, or seed-document semantics beyond what entitlement handling requires.

---

### 7.11 Prohibited Persistence Rules

#### CDATA-033 — Prohibited Persistence Requirement
This family must identify prohibited persistence classes clearly enough to prevent accidental implementation drift.

#### CDATA-034 — Required Prohibited Classes
At minimum, prohibited or tightly constrained persistence classes must include:
- hidden durable seed-document archives
- hidden durable extracted-text archives
- hidden durable transcript archives beyond governed output/report structures
- hidden durable debate-memory archives
- hidden durable vector/index histories beyond governed transient workspace use
- plaintext secret persistence
- unauthorized cloud persistence for local transient runtime material

#### CDATA-035 — No Neutral Naming Rule
Prohibited persistence classes must not be given neutral-sounding names that hide their policy status.

---

### 7.12 Local Data Structure Rules

#### CDATA-036 — Local Structure Rule
This family should define the structure of local persistent support records where such persistence is governed and necessary.

#### CDATA-037 — Local Structure Minimalism Rule
Local persistent support structures should include only fields necessary for the governed support purpose and must not quietly accumulate convenience history.

#### CDATA-038 — Stable Identifier Rule
Local data structures should use stable deterministic identifiers where the structure must be referenced across app sessions or validated by other technical families.

---

### 7.13 Backend Table Definition Rules

#### CDATA-039 — Backend Table Requirement
Where backend storage is governed, this family should define stable table families or equivalent record structures.

#### CDATA-040 — Minimum Table Definition Fields
The preferred minimum table-definition fields are:
- table name
- storage domain
- primary purpose
- key fields
- sensitive-field note
- retention/persistence note
- upstream traceability note

#### CDATA-041 — No Backend Table Inflation Rule
Backend table definitions must not include speculative future-business fields or analytics scope unless upstream documents authorize them.

---

### 7.14 Migration Inventory Rules

#### CDATA-042 — Migration Inventory Requirement
This family must maintain a migration inventory or equivalent schema-evolution reference for governed persistent structures.

#### CDATA-043 — Migration Ordering Rule
Migration definitions must use deterministic ordering and stable naming conventions.

#### CDATA-044 — Migration Scope Rule
A migration must clearly state:
- affected storage domain
- purpose
- whether it is local or backend-facing
- whether it adds, modifies, or constrains a governed structure

#### CDATA-045 — No Migration-Led Scope Expansion Rule
A migration must not be used to introduce new persistent product behavior not already authorized upstream.

---

### 7.15 Cleanup-Scope Mapping Rules

#### CDATA-046 — Cleanup Scope Table Requirement
This family should define a cleanup-scope mapping for relevant data classes.

#### CDATA-047 — Minimum Cleanup Mapping Fields
The preferred minimum cleanup mapping fields are:
- data class
- storage domain
- cleanup applies yes/no
- janitor applies yes/no
- export/report preservation exception yes/no
- notes on partial failure relevance where governed upstream

#### CDATA-048 — No Cleanup Ambiguity Rule
A technically relevant data class must not leave it ambiguous whether it is subject to cleanup, exempt from cleanup, or preserved as report/output material.

---

### 7.16 Secret-Handling Boundary Rules

#### CDATA-049 — Secret Boundary Requirement
This family must define enough secret-handling boundary detail to prevent accidental cross-domain leakage.

#### CDATA-050 — Secret Field Rule
If a record structure contains or references secret-bearing material, this family must identify whether:
- the secret itself is stored in secure storage
- only a reference handle is stored elsewhere
- the field must never appear outside secure storage

#### CDATA-051 — No Secret Echo Rule
Secret-bearing fields must not be normalized into logs, report metadata, workspace intermediates, or exported artifacts.

---

### 7.17 Cross-Family Dependencies

#### CDATA-052 — Interface Dependency Rule
Where persistence-relevant data appears in interface payloads, Annex Family A must consume the storage meanings defined here rather than invent them.

#### CDATA-053 — Runtime Dependency Rule
Where cleanup, janitor, finalization, or activation lifecycle meaning affects storage semantics, Annex Family B owns the triggering runtime meaning and this family owns the storage-boundary meaning.

#### CDATA-054 — Output Dependency Rule
Where report metadata or export preservation is implicated, Annex Family D must remain aligned with the storage and cleanup distinctions owned here.

#### CDATA-055 — QA Dependency Rule
Where storage, cleanup, secret handling, or migration behavior is validated, Annex Family E must consume the definitions owned here rather than redefine them.

---

### 7.18 Data Integrity Prohibitions

#### CDATA-056 — No Hidden Persistence Rule
This family must not normalize undeclared persistence as harmless technical convenience.

#### CDATA-057 — No Cleanup-Washing Rule
A cleanup-targeted data class must not be reclassified as ordinary durable data merely because persistence is easier to implement.

#### CDATA-058 — No Report-Washing Rule
A transient report intermediate must not be mislabeled as a durable report artifact.

#### CDATA-059 — No Secret-Washing Rule
A secret-bearing field must not be mislabeled as harmless configuration metadata.

#### CDATA-060 — No Migration-Washing Rule
A migration must not be framed as routine technical housekeeping if it materially changes governed persistence behavior.

---

### 7.19 Boundaries of This Section

#### CDATA-061 — Section Boundary Rule
This section defines:
- the data/persistence/migration family role
- storage domains
- data class inventories
- local/backend structure discipline
- migration inventory rules
- cleanup-scope mapping
- secret-handling boundaries
- cross-family persistence dependencies

This section does not yet define:
- the report template structures in full
- the QA matrix structures in full
- the AI-agent technical-reference pack in full
- the pack index and change-control mechanics in full

Those are defined in later sections.

---

### 7.20 Final Data Pack Summary

#### CDATA-062 — Section Summary Rule
At the data-contract level, Annex Family C formalizes the governed storage, persistence, cleanup, secure-store, report-preservation, backend-commercial, and migration boundaries of the MVP into deterministic technical structures that prevent hidden persistence, secret leakage, cleanup ambiguity, and migration-led scope drift.

#### CDATA-063 — Stability Rule
All later sections of Document C must remain consistent with this data, persistence, and migration pack unless revised explicitly through the Document C Change Log.

---

## 8. Annex Family D — Report and Output Pack

### 8.1 Section Purpose
This annex family defines the structured technical reference layer for generated reports, report fields, output artifacts, report availability states, export boundaries, and output-validation-relevant technical structures.

This section exists to ensure that:
- the final report is formalized into deterministic technical structures
- AI coding agents do not invent report sections, output modes, or scoring semantics
- report generation, report availability, and export behavior remain technically explicit
- output artifacts remain aligned with upstream completion, finalization, and cleanup truth

This annex family is subordinate to:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- Section 3 — Source-of-Truth and Traceability Map
- Section 4 — Canonical Terminology and Technical Naming Rules
- Document A report/output, finalization, completion, and cleanup truth
- Document B phase, validation, and release-readiness context where report/output behavior is execution-relevant

---

### 8.2 Family Role

#### CREPORT-001 — Report Pack Rule
Annex Family D is the primary technical family for report and output structures.

#### CREPORT-002 — Family Purpose Rule
This family exists to formalize:
- report section inventory
- report field structures
- viability-score representation
- ranked-risk representation
- strongest-pro/anti argument representation
- cost/token summary representation
- transcript appendix representation
- PDF generation input/output contract
- report metadata and report availability structures
- export and preservation boundaries where output artifacts are implicated

#### CREPORT-003 — No Scoring Philosophy Rewrite Rule
This family may formalize score fields and output structure, but it must not redefine the philosophical meaning or upstream logic of the Market Viability Score.

#### CREPORT-004 — No Completion Rewrite Rule
This family may formalize output outcomes, but it must not redefine what counts as completion, constrained completion, failure, cancellation, or cleanup outcome.

---

### 8.3 Family Scope

#### CREPORT-005 — In-Scope Rule
Annex Family D is in scope for:
- report section inventory
- section-level template structures
- output schema definitions
- report metadata structures
- output artifact classification
- PDF generation contract structure
- report availability and export structure
- appendix structure
- output-related validation reference fields

#### CREPORT-006 — Out-of-Scope Rule
Annex Family D is out of scope for:
- redefining report-generation triggers
- redefining state-machine meaning
- redefining cleanup policy
- redefining interface route ownership
- redefining QA gate ownership
- inventing additional user-facing output formats not governed upstream

#### CREPORT-007 — Primary Family Boundary Rule
If the primary question is “what does the report/output technically contain and how is it structured?”, Annex Family D owns it.
If the primary question is “when does output become available?”, Annex Family B owns the runtime meaning and Annex Family A owns the interface exposure.

---

### 8.4 Canonical Output Classes

#### CREPORT-008 — Output Class Rule
This family must organize outputs into stable output classes.

#### CREPORT-009 — Required Output Classes
At minimum, the output classes are:

1. report metadata class  
2. report section-content class  
3. transcript appendix class  
4. generated PDF artifact class  
5. export-access class  
6. report availability class  

#### CREPORT-010 — No Output-Class Collapse Rule
These output classes must not be collapsed in a way that hides:
- whether content is metadata or full section content
- whether a file artifact exists
- whether output is available but not yet exported
- whether transcript content is appendix-only versus summary content

---

### 8.5 Canonical Report Section Inventory

#### CREPORT-011 — Section Inventory Requirement
This family must maintain a canonical report section inventory.

#### CREPORT-012 — Required Report Sections
The structured PDF report must contain the following required sections:

1. Executive Summary  
2. Market Viability Score  
3. Ranked Risks  
4. Strongest Pro Arguments  
5. Strongest Anti Arguments  
6. Cost and Token Summary  
7. Transcript Appendix  

#### CREPORT-013 — No Section Invention Rule
No new report section may be added unless it is explicitly authorized upstream.

#### CREPORT-014 — No Section Omission Rule
A required report section must not be silently omitted from the technical template merely because some upstream material was constrained or partial.
Where governed upstream, partiality must be reflected through section content state, not hidden through omission.

---

### 8.6 Report Metadata Rules

#### CREPORT-015 — Metadata Requirement
This family should define stable report metadata structures.

#### CREPORT-016 — Minimum Metadata Fields
The preferred minimum report metadata fields are:
- report_id
- session_id
- run_outcome_classification
- report_generation_status
- report_availability_status
- generated_at
- constrained_completion_flag where applicable
- token_cost_summary_reference
- artifact_location_reference where applicable

#### CREPORT-017 — No Completion/Availability Collapse Rule
Report metadata must preserve the distinction among:
- run outcome
- report generation success
- report availability
- artifact export/access success

#### CREPORT-018 — No Fake-Availability Rule
Metadata must not imply report availability when no governed output artifact exists.

---

### 8.7 Executive Summary Structure Rules

#### CREPORT-019 — Executive Summary Rule
The Executive Summary structure must formalize the one-page synthesis of the swarm’s debate.

#### CREPORT-020 — Summary Structure Fields
The Executive Summary structure should define at minimum:
- summary heading
- high-level synthesis body
- summary scope note where constrained completion affects completeness
- relationship to the debate material actually completed

#### CREPORT-021 — No Fabricated Synthesis Rule
The Executive Summary structure must not normalize fabricating debate synthesis for material not actually generated.

---

### 8.8 Market Viability Score Structure Rules

#### CREPORT-022 — Score Structure Rule
The Market Viability Score structure must formalize the quantitative score representation.

#### CREPORT-023 — Minimum Score Fields
The preferred minimum fields are:
- score_label
- score_value
- score_range_min
- score_range_max
- scoring_actor_reference
- constrained-context note where applicable

#### CREPORT-024 — Score Range Integrity Rule
The Market Viability Score must remain represented as a 0–100 scale unless upstream governance changes.

#### CREPORT-025 — No Score Inflation Rule
This family must not introduce extra score types, alternate scales, hidden weighting systems, or auxiliary rating systems not governed upstream.

---

### 8.9 Ranked Risks Structure Rules

#### CREPORT-026 — Ranked Risks Rule
The Ranked Risks structure must formalize the technical representation of the top identified flaws.

#### CREPORT-027 — Minimum Ranked-Risk Fields
The preferred minimum fields are:
- rank
- risk_title
- risk_type
- risk_description
- originating_actor_class reference where applicable
- confidence or emphasis note where governed upstream

#### CREPORT-028 — Top-Three Integrity Rule
The ranked-risk structure must preserve the requirement that the report identifies the top 3 technical or market flaws identified by skeptical personas.

#### CREPORT-029 — No Risk Inflation Rule
This family must not silently expand the ranked-risk requirement into a larger mandatory list or a different ranking philosophy.

---

### 8.10 Strongest Pro/Anti Argument Structure Rules

#### CREPORT-030 — Argument Structure Rule
The report must formalize separate structures for strongest pro arguments and strongest anti arguments.

#### CREPORT-031 — Minimum Argument Fields
The preferred minimum fields are:
- argument_title or short label
- argument_body
- argument_side
- source_actor_class reference where applicable
- supporting debate linkage note where applicable

#### CREPORT-032 — Separation Rule
Strongest pro arguments and strongest anti arguments must remain technically distinct and must not be merged into one generic “key arguments” block if upstream meaning distinguishes them.

---

### 8.11 Cost and Token Summary Rules

#### CREPORT-033 — Cost Summary Rule
The Cost and Token Summary structure must formalize exact run cost metrics as governed upstream.

#### CREPORT-034 — Minimum Cost Fields
The preferred minimum fields are:
- token_budget_requested where relevant
- token_usage_total
- cost_estimate_or_actual_cost_value
- cost_currency_or_cost_unit_note where relevant
- duration_elapsed_reference
- constrained-stop reason reference where applicable

#### CREPORT-035 — No Approximation Drift Rule
Where upstream documents require exact metrics, this family must not normalize vague approximations as the governed default.

#### CREPORT-036 — No Hidden-Cost Field Rule
This family must not imply hidden billing or billing semantics broader than the actual governed run-cost summary.

---

### 8.12 Transcript Appendix Rules

#### CREPORT-037 — Transcript Appendix Rule
The Transcript Appendix structure must formalize the full raw chronological log of the debate material included in the report.

#### CREPORT-038 — Minimum Transcript Fields
The preferred minimum transcript appendix fields are:
- ordered transcript entries
- sequence reference
- speaker reference
- timestamp or ordered occurrence reference
- event/body content
- appendix completeness note where constrained completion applies

#### CREPORT-039 — Chronology Integrity Rule
The transcript appendix must preserve chronological ordering consistent with the governed event and sequence system.

#### CREPORT-040 — No Transcript Fabrication Rule
The transcript appendix must not normalize generation of transcript entries that did not actually occur in the run.

---

### 8.13 PDF Generation Contract Rules

#### CREPORT-041 — PDF Generation Contract Requirement
This family must define the technical contract for PDF generation sufficiently for implementation and validation.

#### CREPORT-042 — Minimum PDF Contract Fields
The preferred minimum PDF-generation contract fields are:
- report input structure reference
- generation status
- generation failure classification
- output artifact identifier
- output artifact location/reference
- report availability result

#### CREPORT-043 — No Placeholder-PDF Success Rule
A placeholder or empty PDF must not be classified as successful governed report generation.

#### CREPORT-044 — No Hidden-Format Expansion Rule
This family must not imply new mandatory output formats beyond the governed PDF report and governed report-access structures.

---

### 8.14 Report Availability and Export Rules

#### CREPORT-045 — Availability Structure Rule
This family must define technical structures for report availability and export access.

#### CREPORT-046 — Minimum Availability Fields
The preferred minimum fields are:
- availability_status
- artifact_exists_flag
- accessible_flag
- export_ready_flag where relevant
- availability_failure_reason where applicable

#### CREPORT-047 — No Generation/Access Collapse Rule
This family must preserve the distinction between:
- report generated
- report exists
- report accessible
- report export action succeeded

#### CREPORT-048 — No Cleanup-Boundary Drift Rule
Availability/export structures must remain aligned with the governed rule that preserved report artifacts are distinct from transient workspace cleanup targets.

---

### 8.15 Output Partiality and Constrained Completion Rules

#### CREPORT-049 — Constrained Completion Rule
This family must support explicit technical representation of constrained completion where a report is generated from partial rounds due to budget or duration cap.

#### CREPORT-050 — Minimum Partiality Fields
Where constrained completion is relevant, the preferred minimum fields are:
- constrained_completion_flag
- constraint_trigger_type
- completed_round_count reference
- content_scope_note
- report_truthfulness note

#### CREPORT-051 — No Full-Completion Mislabeling Rule
A constrained-completion report must not be technically labeled as indistinguishable from a full normal-completion report when upstream meaning distinguishes them.

---

### 8.16 Cross-Family Dependencies

#### CREPORT-052 — Runtime Dependency Rule
Where report generation depends on finalization and completion classification, Annex Family B owns the runtime meaning and this family owns the output structure.

#### CREPORT-053 — Interface Dependency Rule
Where report metadata or file access is exposed through interfaces, Annex Family A must consume the output meanings defined here rather than redefine them.

#### CREPORT-054 — Persistence Dependency Rule
Where report artifacts, metadata, or export preservation are implicated, Annex Family C owns the storage-boundary meaning and this family owns the output-shape meaning.

#### CREPORT-055 — QA Dependency Rule
Where report correctness, availability, or export behavior is validated, Annex Family E must consume the structures owned here rather than redefine them.

---

### 8.17 Output Integrity Prohibitions

#### CREPORT-056 — No Output-Fabrication Rule
This family must not normalize fabricated report content for material not actually generated by the governed run.

#### CREPORT-057 — No Score-Washing Rule
A score field must not be technically framed in a way that hides constrained context, generation failure, or absent source material where those distinctions matter.

#### CREPORT-058 — No Availability-Washing Rule
A report must not be labeled available when only metadata exists or generation actually failed.

#### CREPORT-059 — No Appendix-Washing Rule
A transcript appendix must not be labeled complete if governed transcript content is partial and the system knows that constrained completion occurred.

---

### 8.18 Boundaries of This Section

#### CREPORT-060 — Section Boundary Rule
This section defines:
- report section inventory
- report metadata structures
- output artifact classes
- PDF generation contract structures
- availability/export structures
- constrained-completion output structures
- cross-family output dependencies

This section does not yet define:
- the QA matrix structures in full
- the AI-agent technical-reference pack in full
- pack versioning and change-control mechanics in full

Those are defined in later sections.

---

### 8.19 Final Report Pack Summary

#### CREPORT-061 — Section Summary Rule
At the output-contract level, Annex Family D formalizes the governed report, PDF, appendix, score, risk, argument, cost, availability, and constrained-completion structures of the MVP into deterministic technical templates that downstream interfaces, persistence rules, and QA matrices must consume without redefining output truth.

#### CREPORT-062 — Stability Rule
All later sections of Document C must remain consistent with this report and output pack unless revised explicitly through the Document C Change Log.

---

## 9. Annex Family E — QA, Acceptance, and Red-Team Pack

### 9.1 Section Purpose
This annex family defines the structured technical reference layer for QA matrices, acceptance mappings, red-team test structures, validation artifact groupings, and engineering-grade verification reference material.

This section exists to ensure that:
- validation structures are formalized deterministically
- AI coding agents do not invent a parallel definition of done
- QA and red-team artifacts remain aligned with upstream acceptance and execution-gate truth
- validation surfaces across interfaces, states, data, outputs, and AI-agent behavior can be audited coherently

This annex family is subordinate to:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- Section 3 — Source-of-Truth and Traceability Map
- Section 4 — Canonical Terminology and Technical Naming Rules
- Document A acceptance criteria and known-risk framing
- Document B validation, exit-gate, replan, escalation, and AI-agent execution rules

---

### 9.2 Family Role

#### CQA-001 — QA Pack Rule
Annex Family E is the primary technical family for QA, acceptance, and red-team technical validation structures.

#### CQA-002 — Family Purpose Rule
This family exists to formalize:
- acceptance mapping tables
- QA matrix structures
- red-team scenario structures
- regression classification models
- validation artifact inventories
- cross-family verification references
- failure classification and retest relevance where governed upstream

#### CQA-003 — No Acceptance Rewrite Rule
This family may formalize acceptance and validation structures, but it must not redefine the meaning of acceptance, release readiness, or gate success.

#### CQA-004 — No Execution-Gate Rewrite Rule
This family may formalize technical validation references, but it must not redefine the exit-gate, release-gate, or replan logic governed upstream.

---

### 9.3 Family Scope

#### CQA-005 — In-Scope Rule
Annex Family E is in scope for:
- acceptance mapping structures
- QA test matrices
- red-team matrix structures
- regression classification models
- cross-family validation inventories
- validation identifier conventions
- failure classification support fields
- verification coverage reference tables

#### CQA-006 — Out-of-Scope Rule
Annex Family E is out of scope for:
- inventing new acceptance obligations
- inventing new lifecycle states
- inventing new storage policy
- inventing new output structures
- rewriting execution order
- replacing the narrative explanation of risks in Document A
- replacing the gate logic in Document B

#### CQA-007 — Primary Family Boundary Rule
If the primary question is “how is this technically validated, mapped, or red-teamed?”, Annex Family E owns it.
If the primary question is “what does the thing mean?”, the owning technical family or upstream document owns it.

---

### 9.4 Canonical Validation Domains

#### CQA-008 — Validation Domain Rule
This family must organize validation structures into stable validation domains.

#### CQA-009 — Required Validation Domains
At minimum, the validation domains are:

1. interface-contract validation  
2. runtime/state/event validation  
3. persistence/cleanup/secret-handling validation  
4. report/output validation  
5. activation/licensing validation  
6. Preflight/eligibility validation  
7. updater/runtime-trust validation  
8. AI-agent drift-control validation  
9. acceptance and release-readiness aggregation  

#### CQA-010 — No Validation-Domain Collapse Rule
These validation domains must not be collapsed in a way that hides what is being validated or which family owns the underlying meaning.

---

### 9.5 Acceptance Mapping Rules

#### CQA-011 — Acceptance Mapping Requirement
This family must maintain an acceptance mapping structure that ties technical validation artifacts back to upstream acceptance meaning.

#### CQA-012 — Minimum Acceptance Mapping Fields
The preferred minimum fields are:
- acceptance mapping ID
- upstream acceptance reference
- validation domain
- related annex family
- validation artifact reference
- coverage status
- notes on exclusions or limits where relevant

#### CQA-013 — No Parallel Acceptance Rule
Acceptance mapping must not become a separate acceptance framework.
It is only a technical mapping layer back to upstream acceptance truth.

#### CQA-014 — No Orphan Acceptance Test Rule
A major technical validation artifact must not claim acceptance relevance if it cannot trace back to an upstream acceptance criterion or validation obligation.

---

### 9.6 QA Matrix Rules

#### CQA-015 — QA Matrix Requirement
This family must define stable QA matrix structures for recurring technical verification.

#### CQA-016 — Minimum QA Matrix Fields
The preferred minimum matrix fields are:
- QA test ID
- validation domain
- target artifact or behavior
- preconditions
- action or scenario
- expected governed result
- evidence type
- pass/fail outcome field
- revalidation trigger note where applicable

#### CQA-017 — Deterministic QA Rule
QA matrices must be written so that an implementer or reviewer can tell exactly what condition is being validated and what governed outcome is expected.

#### CQA-018 — No Narrative-Only QA Rule
A QA matrix entry must not rely on vague language such as “looks right” or “seems fine” where a governed outcome can be stated more deterministically.

---

### 9.7 Red-Team Matrix Rules

#### CQA-019 — Red-Team Matrix Requirement
This family must define stable red-team matrix structures.

#### CQA-020 — Minimum Red-Team Matrix Fields
The preferred minimum fields are:
- red-team test ID
- risk class
- target surface
- attack or misuse scenario
- expected governed defense or truth-preserving outcome
- evidence type
- severity relevance
- escalation relevance

#### CQA-021 — Required Red-Team Focus Areas
At minimum, red-team matrix structures must support scenarios involving:
- activation-limit bypass attempts
- secret leakage attempts
- hidden persistence attempts
- cleanup-failure concealment
- state desynchronization
- false completion/reporting
- unsafe runtime/update trust paths
- report or transcript fabrication attempts
- AI-agent scope or drift failure modes where technically testable

#### CQA-022 — No Cosmetic Red-Team Rule
Red-team matrices must not reduce meaningful risk classes to superficial or purely UI-level checks if upstream risk framing is deeper.

---

### 9.8 Regression Classification Rules

#### CQA-023 — Regression Classification Requirement
This family should define a stable regression classification model for repeated validation after change.

#### CQA-024 — Minimum Regression Fields
The preferred minimum regression fields are:
- regression class
- affected validation domains
- trigger change type
- priority level
- re-run scope
- upstream traceability note

#### CQA-025 — No Regression Drift Rule
Regression classification must not be based solely on engineering convenience; it must remain aligned with governed technical risk and validation impact.

---

### 9.9 Cross-Family Validation Mapping Rules

#### CQA-026 — Cross-Family Validation Rule
This family must map validation structures to the technical families whose meaning they verify.

#### CQA-027 — Interface Validation Rule
Interface validation structures must consume route/contract shapes from Annex Family A rather than restating them independently.

#### CQA-028 — Runtime Validation Rule
State/event/runtime validation structures must consume canonical runtime truth from Annex Family B.

#### CQA-029 — Persistence Validation Rule
Storage, cleanup, secure-store, and migration validation structures must consume persistence truth from Annex Family C.

#### CQA-030 — Output Validation Rule
Report-generation, availability, and export validation structures must consume output truth from Annex Family D.

#### CQA-031 — AI-Agent Validation Rule
Where validation is aimed at AI-agent behavior, the relevant guardrails must trace back to Documents A and B as formalized by Annex Family F.

---

### 9.10 Required Validation Structure Classes

#### CQA-032 — Required Structure Class Rule
At minimum, this family should support the following validation structure classes:

1. endpoint contract verification tables  
2. state-transition verification tables  
3. event-order verification tables  
4. activation/licensing verification tables  
5. Preflight outcome verification tables  
6. cleanup and janitor verification tables  
7. report-generation and availability verification tables  
8. secure-storage and secret-handling verification tables  
9. migration and persistence-boundary verification tables  
10. AI-agent drift-control verification tables  

#### CQA-033 — No Structure-Class Omission Rule
If a governed technical surface exists and is central to release-readiness, its validation structure class must not be silently absent.

---

### 9.11 Failure Classification Rules

#### CQA-034 — Failure Classification Rule
This family should define stable technical failure-classification support structures for QA and red-team work.

#### CQA-035 — Minimum Failure Fields
The preferred minimum fields are:
- failure class
- affected domain
- blocker severity relevance
- release relevance
- retest requirement
- escalation relevance

#### CQA-036 — No Failure-Washing Rule
Validation structures must not soften technically meaningful failures into ambiguous “warnings” if upstream gates would treat them as blockers or rework-required conditions.

---

### 9.12 Evidence-Type Rules

#### CQA-037 — Evidence Type Rule
This family should define stable evidence-type categories used in QA and red-team matrices.

#### CQA-038 — Allowed Evidence Categories
Evidence categories may include:
- interface behavior evidence
- schema conformance evidence
- state transition evidence
- ordered event evidence
- artifact existence evidence
- secure-store handling evidence
- cleanup/janitor evidence
- report/output evidence
- regression-run evidence
- human review evidence where governed upstream permits it

#### CQA-039 — No Opaque Evidence Rule
Evidence categories must be specific enough that a reviewer can understand what kind of proof is expected.

---

### 9.13 Release-Readiness Aggregation Rules

#### CQA-040 — Release Aggregation Rule
This family may formalize technical aggregation structures that support upstream release-readiness judgment.

#### CQA-041 — Aggregation Boundary Rule
Release-readiness aggregation structures must summarize technical validation status, but they must not independently declare release readiness contrary to Document B’s governed gate logic.

#### CQA-042 — No Independent Go/No-Go Rule
This family must not create an autonomous technical release authority separate from Document B’s gate model.

---

### 9.14 AI-Agent Validation Structures

#### CQA-043 — AI Validation Rule
Where AI-agent implementation drift is technically validated, this family should define stable structures for that validation.

#### CQA-044 — AI Drift Test Focus Areas
AI-agent drift validation may include checks for:
- unauthorized field invention
- unauthorized persistence introduction
- layer-boundary violations
- scope-expanding schema changes
- acceptance-softening technical edits
- misleading completion/status semantics

#### CQA-045 — No Prompt-Only Validation Rule
AI-agent validation structures must not assume a prompt alone guarantees safe behavior; they must define what technical evidence or review pattern checks for drift.

---

### 9.15 Cross-Document Traceability Rules for QA

#### CQA-046 — A-to-E Traceability Rule
Each major QA or acceptance mapping artifact must trace to one or more relevant upstream acceptance or risk families in Document A.

#### CQA-047 — B-to-E Traceability Rule
Each major QA, gate, regression, or release-aggregation artifact must trace to one or more validation, exit-gate, replan, or AI-agent execution-control families in Document B where relevant.

#### CQA-048 — No Untraceable Severity Rule
A blocker, release-relevant failure, or escalation-relevant red-team result must not appear in this family without traceable upstream grounding.

---

### 9.16 Validation Integrity Prohibitions

#### CQA-049 — No Acceptance-Washing Rule
This family must not reframe unmet upstream acceptance criteria as acceptable merely because technical work is close.

#### CQA-050 — No Gate-Washing Rule
Technical matrix language must not soften upstream exit-gate or release-gate meaning.

#### CQA-051 — No Cosmetic Coverage Rule
A large number of low-value tests must not be used to imply strong validation coverage if critical truth-bearing domains remain weakly validated.

#### CQA-052 — No False-Pass Rule
A validation structure must not normalize partial success as full pass where upstream logic distinguishes them.

#### CQA-053 — No Risk-Downgrade Drift Rule
Red-team structures must not quietly downgrade meaningful risk classes without explicit upstream justification.

---

### 9.17 Boundaries of This Section

#### CQA-054 — Section Boundary Rule
This section defines:
- acceptance mapping structures
- QA matrix structures
- red-team matrix structures
- regression classification structures
- failure-classification support
- evidence-type categories
- release-readiness aggregation support
- cross-family validation mapping discipline

This section does not yet define:
- the AI-agent technical-reference pack in full
- pack versioning and change-control mechanics in full

Those are defined in later sections.

---

### 9.18 Final QA Pack Summary

#### CQA-055 — Section Summary Rule
At the validation-structure level, Annex Family E formalizes the governed QA, acceptance-mapping, red-team, regression, evidence, and release-support structures of the MVP into deterministic technical matrices that validate interfaces, runtime truth, persistence boundaries, outputs, and AI-agent implementation behavior without redefining acceptance or gate meaning.

#### CQA-056 — Stability Rule
All later sections of Document C must remain consistent with this QA, acceptance, and red-team pack unless revised explicitly through the Document C Change Log.

---

## 10. Annex Family F — AI-Agent Reference Pack

### 10.1 Section Purpose
This annex family defines the structured technical reference layer intended specifically to reduce AI-agent ambiguity, hallucination risk, layer-boundary drift, schema drift, and unsafe implementation behavior during coding work.

This section exists to ensure that:
- AI coding agents can consume highly deterministic technical reference material without confusing it for product or execution authority
- implementation-facing technical notes remain subordinate to Documents A and B
- technical reference aids reduce hallucination without silently expanding scope
- AI-agent behavior remains aligned with governed contracts, states, data boundaries, outputs, and validation structures

This annex family is subordinate to:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- Section 3 — Source-of-Truth and Traceability Map
- Section 4 — Canonical Terminology and Technical Naming Rules
- Document A AI-agent implementation guardrails
- Document B AI-agent execution protocol, task-packet rules, validation rules, and drift-control rules

---

### 10.2 Family Role

#### CAI-001 — AI Reference Pack Rule
Annex Family F is the primary technical family for AI-agent-facing engineering reference material.

#### CAI-002 — Family Purpose Rule
This family exists to formalize:
- implementation-facing technical reference tables
- file/module expectation maps where useful
- artifact ownership reminders
- contract-usage notes
- schema-usage notes
- migration-usage notes
- technical “do not infer” notes
- AI-agent technical non-goal reminders
- cross-family implementation reference summaries

#### CAI-003 — No Product-Authority Rule
This family must not act as a source of product authority.

#### CAI-004 — No Execution-Authority Rule
This family must not act as a source of execution-order authority.
It exists to assist implementation safely within authority already defined by Documents A and B.

---

### 10.3 Family Scope

#### CAI-005 — In-Scope Rule
Annex Family F is in scope for:
- technical reference tables for AI agents
- implementation-facing artifact maps
- family-by-family usage guidance
- field-usage reminders
- boundary reminders
- escalation reminders
- “do not infer” reference notes
- technical anti-drift notes
- review-oriented engineering checklists that do not redefine validation meaning

#### CAI-006 — Out-of-Scope Rule
Annex Family F is out of scope for:
- redefining product behavior
- redefining execution order
- redefining acceptance or release readiness
- inventing new implementation requirements
- replacing task packets
- replacing source code review
- replacing QA matrices
- replacing canonical contracts owned by other annex families

#### CAI-007 — Primary Family Boundary Rule
If the primary question is “what should an AI coding agent keep in mind while implementing governed technical material?”, Annex Family F owns it.
If the primary question is “what is the actual contract/state/schema/table?”, the owning family retains primary authority.

---

### 10.4 AI-Agent Reference Philosophy

#### CAI-008 — Reference Philosophy Rule
This family must optimize for implementation safety, not creative flexibility.

#### CAI-009 — Deterministic Reference Rule
Reference material in this family should be highly deterministic, low-ambiguity, and easy to consume in small bounded packets.

#### CAI-010 — Non-Override Rule
No technical reference note in this family may override:
- a formal contract in Annex Family A
- a state/event/runtime definition in Annex Family B
- a persistence or migration rule in Annex Family C
- an output definition in Annex Family D
- a QA or red-team structure in Annex Family E
- any higher-authority upstream document

#### CAI-011 — Anti-Hallucination Rule
This family must be designed primarily to reduce the chance that AI coding agents:
- invent new fields
- invent new states
- invent new event types
- invent hidden persistence
- blur architectural boundaries
- mislabel constrained completion as full success
- weaken validation meaning
- infer forbidden resume behavior

---

### 10.5 Canonical AI-Agent Reference Classes

#### CAI-012 — Reference Class Rule
This family must organize AI-agent reference material into stable reference classes.

#### CAI-013 — Required Reference Classes
At minimum, the reference classes are:

1. authority and usage reminders  
2. artifact ownership maps  
3. contract consumption reminders  
4. state/event consumption reminders  
5. persistence and secret-handling reminders  
6. report/output implementation reminders  
7. validation and QA alignment reminders  
8. escalation and non-goal reminders  

#### CAI-014 — No Reference-Class Collapse Rule
These classes must not be collapsed in a way that obscures whether a note is about:
- authority
- ownership
- implementation boundary
- prohibited inference
- escalation requirement

---

### 10.6 Authority and Usage Reminder Rules

#### CAI-015 — Authority Reminder Requirement
This family should explicitly remind AI agents of the governing hierarchy under which Document C operates.

#### CAI-016 — Required Hierarchy Reminder
The AI-agent reference layer must preserve the hierarchy:
- Document A defines product truth
- Document B defines execution order
- Document C defines technical annex detail
- task packets define bounded execution work

#### CAI-017 — No Reference-Layer Supremacy Rule
Reference notes must never be phrased as if they outrank formal schemas, formal contract tables, or upstream documents.

---

### 10.7 Artifact Ownership Map Rules

#### CAI-018 — Ownership Map Requirement
This family should define artifact ownership maps to help AI agents identify which family owns which technical meaning.

#### CAI-019 — Minimum Ownership Map Fields
The preferred minimum ownership-map fields are:
- artifact class
- owning annex family
- upstream dependency
- typical consuming families
- non-ownership note

#### CAI-020 — Required Ownership Distinctions
At minimum, ownership maps should preserve that:
- Annex Family A owns interface contract shape
- Annex Family B owns state/event/runtime meaning
- Annex Family C owns persistence and migration boundaries
- Annex Family D owns report/output structure
- Annex Family E owns validation matrix structures
- Annex Family F owns AI-agent reference aids only
- Annex Family G owns pack indexing, versioning, and change control

#### CAI-021 — No Ownership Drift Rule
Ownership maps must not quietly reassign authority for convenience.

---

### 10.8 Contract Consumption Reminder Rules

#### CAI-022 — Contract Consumption Rule
This family should define reminders for how AI agents must consume interface and schema contracts.

#### CAI-023 — Required Contract Reminder Content
At minimum, contract reminders should reinforce:
- do not invent fields
- do not broaden enum meanings
- do not infer optional routes or alternate modes
- do not infer hidden success states from vague payloads
- do not reinterpret response classes contrary to formal outcome meaning

#### CAI-024 — No Contract-Expansion Reminder Rule
Reference notes must explicitly discourage treating a formal contract as permission to add adjacent contract surfaces.

---

### 10.9 State and Event Consumption Reminder Rules

#### CAI-025 — State/Event Reminder Rule
This family should define reminders for consuming runtime states and event families safely.

#### CAI-026 — Required State/Event Reminder Content
At minimum, state/event reminders should reinforce:
- do not invent new lifecycle states
- do not collapse distinct terminal states
- do not reinterpret degraded states as healthy progress
- do not imply crash-resume through event or state handling
- do not treat interface transport semantics as redefining event meaning

#### CAI-027 — No Runtime Drift Reminder Rule
Reference notes must make clear that runtime convenience must not change lifecycle truth.

---

### 10.10 Persistence and Secret-Handling Reminder Rules

#### CAI-028 — Persistence Reminder Rule
This family should define AI-facing reminders about governed storage and cleanup boundaries.

#### CAI-029 — Required Persistence Reminder Content
At minimum, persistence reminders should reinforce:
- transient workspace data is not durable history
- secure-store secrets must not be mirrored into plaintext
- commercial backend is not a hidden run-history store
- final reports are preserved differently from transient run artifacts
- migrations must not introduce new governed persistence classes

#### CAI-030 — No Hidden-Persistence Reminder Rule
Reference notes must explicitly warn against undeclared persistence introduced for convenience, debugging, caching, or future-proofing.

---

### 10.11 Report and Output Implementation Reminder Rules

#### CAI-031 — Output Reminder Rule
This family should define AI-facing reminders for output and report implementation.

#### CAI-032 — Required Output Reminder Content
At minimum, output reminders should reinforce:
- do not invent extra report sections
- do not fabricate transcript content
- do not frame constrained completion as full normal completion
- do not mark a report available if generation failed
- do not collapse metadata existence into artifact availability

#### CAI-033 — No Output-Truth Drift Rule
Reference notes must protect the difference among:
- finalization success
- report generation success
- report availability
- export accessibility

---

### 10.12 Validation and QA Alignment Reminder Rules

#### CAI-034 — Validation Reminder Rule
This family should define AI-facing reminders about how implementation work must remain compatible with governed validation structures.

#### CAI-035 — Required Validation Reminder Content
At minimum, validation reminders should reinforce:
- do not weaken pass/fail meaning
- do not soften blocker-class failures into warnings
- do not assume prompt compliance replaces validation evidence
- do not add technical shortcuts that invalidate QA matrix expectations
- do not reinterpret acceptance through implementation optimism

#### CAI-036 — No Validation-Authority Override Rule
Reference notes must not be used as an informal alternative to Annex Family E or Document B gate logic.

---

### 10.13 Escalation and Non-Goal Reminder Rules

#### CAI-037 — Escalation Reminder Rule
This family should define explicit reminders about when AI agents must escalate rather than infer.

#### CAI-038 — Required Escalation Trigger Classes
At minimum, AI-facing escalation reminders should cover:
- unclear upstream authority
- apparent conflict between families
- request to add a field with behavioral meaning
- request to persist forbidden data
- request to add a new state or event type
- request to imply broader offline/resume behavior
- request to broaden output semantics
- request to soften validation or acceptance meaning

#### CAI-039 — Non-Goal Reminder Rule
This family should define explicit technical non-goals that help AI agents avoid drift.

#### CAI-040 — Required Non-Goal Classes
At minimum, AI-facing non-goals should warn against:
- hidden scope expansion
- speculative extensibility
- hidden persistence
- layer-boundary collapse
- narrative-style rewrite of contracts
- silent renaming of canonical artifacts
- schema-led product redesign

---

### 10.14 File and Module Expectation Map Rules

#### CAI-041 — File/Module Map Rule
Where useful, this family may define implementation-facing file/module expectation maps.

#### CAI-042 — Allowed File/Module Map Purpose
File/module maps may exist to indicate likely implementation locations or ownership expectations, but only as non-authoritative technical guidance consistent with Document B workstreams and task packets.

#### CAI-043 — No File-Map Authority Rule
A file/module map must not be treated as permission to violate workstream or architectural boundaries if implementation reality differs.

#### CAI-044 — Bounded Guidance Rule
File/module expectation maps should remain bounded and high-signal; they must not try to encode the entire repository structure if doing so would become stale or misleading.

---

### 10.15 “Do Not Infer” Note Rules

#### CAI-045 — Do-Not-Infer Requirement
This family should contain explicit “do not infer” notes wherever a technical area is likely to attract hallucinated behavior.

#### CAI-046 — Required Do-Not-Infer Classes
At minimum, “do not infer” notes should cover:
- hidden alternate endpoints
- hidden resume/recovery semantics
- extra score fields
- extra persistence layers
- extra output modes
- extra acceptance pathways
- extra event categories
- implicit migration obligations
- implicit telemetry/analytics storage

#### CAI-047 — No Vague Warning Rule
A “do not infer” note should be specific enough that an AI agent can identify the prohibited inference class.

---

### 10.16 Reference Artifact Structure Rules

#### CAI-048 — Reference Artifact Requirement
Reference artifacts in this family should use stable, structured forms rather than ad hoc prose blocks wherever possible.

#### CAI-049 — Preferred Reference Forms
Preferred forms include:
- ownership tables
- reminder tables
- prohibited-inference tables
- escalation trigger tables
- family-consumption summaries
- technical non-goal checklists

#### CAI-050 — No Reference Dump Rule
This family must not become a miscellaneous pile of unstructured AI advice.

---

### 10.17 Cross-Family Dependencies

#### CAI-051 — Cross-Family Dependency Rule
Annex Family F depends on every other annex family because its purpose is to help AI agents consume them safely.

#### CAI-052 — No Reauthoring Rule
When Annex Family F summarizes or reminds, it must consume the primary definitions owned elsewhere rather than rewriting them as alternate contract sources.

#### CAI-053 — Upstream Dependency Rule
Where AI-agent reference notes concern authority, escalation, or drift control, they must remain aligned with Document A and Document B first, and only then with the technical families of Document C.

---

### 10.18 AI Reference Integrity Prohibitions

#### CAI-054 — No Friendly-Override Rule
A helpful engineering note must not override a formal contract, state definition, persistence rule, output rule, QA matrix, or upstream document.

#### CAI-055 — No Hallucination-Led Guidance Rule
This family must not include speculative implementation advice unsupported by the governed A/B/C stack.

#### CAI-056 — No Scope-Washing Rule
Reference material must not normalize scope expansion under labels such as:
- future-ready
- convenient extension
- compatibility option
- implied field
- likely needed later

#### CAI-057 — No Drift-Washing Rule
Reference notes must not normalize technical drift as harmless implementation detail.

---

### 10.19 Boundaries of This Section

#### CAI-058 — Section Boundary Rule
This section defines:
- the AI-agent reference family role
- reference classes
- ownership maps
- consumption reminders
- persistence/output/validation reminders
- escalation and non-goal reminders
- structured AI-facing technical guidance forms

This section does not yet define:
- pack indexing mechanics in full
- pack versioning and change-control mechanics in full

Those are defined in the next section.

---

### 10.20 Final AI Reference Pack Summary

#### CAI-059 — Section Summary Rule
At the AI-agent-reference level, Annex Family F formalizes structured technical guidance that helps AI coding agents consume governed contracts, states, persistence rules, outputs, and validation structures safely and predictably without inventing new authority, new scope, hidden persistence, or implementation drift.

#### CAI-060 — Stability Rule
All later sections of Document C must remain consistent with this AI-agent reference pack unless revised explicitly through the Document C Change Log.

---

## 11. Annex Family G — Pack Index, Versioning, and Change Control

### 11.1 Section Purpose
This annex family defines how Document C is indexed, versioned, revised, revalidated, and maintained over time.

This section exists to ensure that:
- Document C remains auditable as a governed technical annex source
- annex-family completeness and change status are visible
- changes to technical contracts do not happen silently
- revalidation against Documents A and B is explicit when upstream changes affect annex meaning

This annex family is governed by:
- Section 0 — Document Control
- Section 1 — Annex Constitution
- the supremacy of Documents A and B
- the requirement that Document C remain a subordinate technical-annex layer

---

### 11.2 Family Role

#### CCHANGE-001 — Pack Governance Rule
Annex Family G is the primary technical family for pack indexing, versioning, revision control, completeness tracking, and revalidation governance.

#### CCHANGE-002 — Family Purpose Rule
This family exists to formalize:
- annex inventory
- pack completeness tracking
- versioning rules
- change-log rules
- revalidation trigger rules
- cross-document impact checks
- technical-annex maintenance discipline

#### CCHANGE-003 — No Upstream Override Rule
No entry in this family may be interpreted as revising Document A or Document B.

---

### 11.3 Pack Index Rules

#### CCHANGE-004 — Pack Index Requirement
Document C must maintain a pack index or equivalent structured annex inventory.

#### CCHANGE-005 — Minimum Pack Index Fields
The preferred minimum pack-index fields are:
- annex family name
- section number
- primary purpose
- status
- completeness note
- upstream dependency note
- last materially changed version where useful

#### CCHANGE-006 — Required Indexed Families
At minimum, the pack index must cover:
- Annex Family A — Interface and API Contract Pack
- Annex Family B — Event, State, and Runtime Contract Pack
- Annex Family C — Data, Persistence, and Migration Pack
- Annex Family D — Report and Output Pack
- Annex Family E — QA, Acceptance, and Red-Team Pack
- Annex Family F — AI-Agent Reference Pack
- Annex Family G — Pack Index, Versioning, and Change Control

#### CCHANGE-007 — No Unindexed Family Rule
A major annex family must not exist in the document without a corresponding indexed identity in pack governance.

---

### 11.4 Pack Status and Completeness Rules

#### CCHANGE-008 — Pack Status Requirement
This family should define stable status values for annex-family maturity and completeness.

#### CCHANGE-009 — Allowed Pack Status Values
Recommended controlled status values include:
- structure_frozen
- draft_in_progress
- technically_defined
- under_review
- approved_current
- superseded

#### CCHANGE-010 — Completeness Tracking Rule
Each annex family should have a visible completeness note sufficient for reviewers to understand whether the family is:
- structurally defined only
- partially detailed
- technically defined in current scope
- awaiting revalidation
- superseded by later annex content

#### CCHANGE-011 — No False Completeness Rule
A family must not be labeled technically complete if material governed technical detail is still absent.

---

### 11.5 Document C Versioning Rules

#### CCHANGE-012 — Versioning Requirement
Document C must maintain explicit version and status metadata once the unified file is assembled.

#### CCHANGE-013 — Canonical Identity Fields
Document C should maintain at minimum:
- Document ID
- Canonical Title
- Version
- Status
- Governing Upstream Documents
- Last Updated
- Related Documents

#### CCHANGE-014 — Recommended Initial Version Rule
The initial unified version should follow a controlled versioning scheme parallel in discipline to Documents A and B.

Recommended starting pattern:
- C1.0-Final-Draft-1

#### CCHANGE-015 — Version Format Rule
The preferred version format is:

**C[major].[minor]-[status]-[revision]**

Example:
- `C1.0-Final-Draft-1`

#### CCHANGE-016 — Allowed Version Status Labels
Recommended allowed version-status labels include:
- Working-Draft
- Final-Draft
- Review-Draft
- Release-Candidate
- Locked

#### CCHANGE-017 — No Casual Versioning Rule
Document C must not use casual version names such as:
- latest annex
- current contracts
- new schema pack
- final final

---

### 11.6 Change-Log Philosophy

#### CCHANGE-018 — Explicit Revision Rule
Any material change to Document C must be recorded in this family’s change-log structures.

#### CCHANGE-019 — No Silent Revision Rule
A locked annex item must not be treated as changed unless the change is explicitly reflected in the Document C change log.

#### CCHANGE-020 — Version Traceability Rule
Each change-log entry should identify:
- document version
- date
- changed sections or families
- change type
- reason
- summary of change
- impact on technical contracts
- impact on revalidation requirements
- impact on upstream alignment

#### CCHANGE-021 — No Retroactive Legitimization Rule
A change-log entry does not legitimize an earlier unauthorized technical drift that violated upstream truth.

---

### 11.7 Change Categories

#### CCHANGE-022 — Major Change Rule
A Document C change is major if it modifies:
- annex family structure
- contract families in a meaning-changing way
- state/event naming or classifications
- storage boundary meaning
- report structure meaning
- QA matrix philosophy
- AI-agent reference pack authority behavior
- or any technical detail whose change materially alters governed implementation meaning

#### CCHANGE-023 — Moderate Change Rule
A Document C change is moderate if it refines:
- field structures
- schema wording
- matrix wording
- migration detail
- traceability notes
- pack completeness structure
without changing the overall governed technical architecture

#### CCHANGE-024 — Minor Change Rule
A Document C change is minor if it corrects:
- wording clarity
- formatting
- naming consistency
- traceability references
- section consistency
without changing technical meaning

---

### 11.8 Required Change-Log Entry Template

#### CCHANGE-025 — Entry Template Rule
Each future change-log entry should follow this structure:

- **Version:**  
- **Date:**  
- **Changed Sections / Families:**  
- **Change Type:** major / moderate / minor  
- **Reason:**  
- **Summary of Change:**  
- **Impact on Technical Contracts:**  
- **Impact on QA / Validation / Revalidation:**  
- **Impact on Documents A/B Alignment:**  
- **Upstream Revalidation Required:** yes / no  

---

### 11.9 Revalidation Rules

#### CCHANGE-026 — Upstream Revalidation Requirement
If a change to Document A or Document B affects technical meaning inside Document C, the affected annex families must be revalidated.

#### CCHANGE-027 — Required Revalidation Trigger Classes
At minimum, revalidation must occur when upstream changes affect:
- interface behavior
- state/lifecycle meaning
- persistence or cleanup scope
- secret-handling boundaries
- report structure or completion/output meaning
- validation or acceptance logic
- AI-agent execution or technical guardrails

#### CCHANGE-028 — Cross-Family Revalidation Rule
If one annex family changes in a way that affects another family’s consumed meaning, the downstream consuming family must be reviewed for alignment.

#### CCHANGE-029 — No Stale-Annex Rule
An annex family must not remain marked current if upstream or cross-family change has made its content materially stale.

---

### 11.10 Pack Completeness and Coverage Rules

#### CCHANGE-030 — Completeness Review Rule
This family should maintain a pack-level view of whether major technical surfaces are covered.

#### CCHANGE-031 — Coverage Classes
At minimum, coverage review should consider:
- interface coverage
- state/event coverage
- persistence/migration coverage
- output/report coverage
- QA/red-team coverage
- AI-agent guidance coverage
- traceability coverage

#### CCHANGE-032 — No Coverage-Washing Rule
A large number of minor annex entries must not be used to imply that a family has strong coverage if major governed surfaces are still weakly specified.

---

### 11.11 Relationship to Documents A and B

#### CCHANGE-033 — Upstream Alignment Rule
This family must explicitly preserve that Document C remains subordinate to Documents A and B.

#### CCHANGE-034 — No Upstream Rewrite Rule
No Document C change-log entry may be read as changing:
- product truth in Document A
- execution truth in Document B
- acceptance meaning in Document A or B

#### CCHANGE-035 — Upstream Version Reference Rule
Document C should explicitly reference the current governing upstream versions it is aligned to.

---

### 11.12 Initial Structure Record

#### CCHANGE-036 — Initial Structure Freeze Record
The structure of Document C was intentionally frozen before drafting full annex content so that the technical annex layer would remain subordinate, deterministic, and stable.

#### CCHANGE-037 — Initial Batch Construction Summary
The initial draft of Document C was constructed in controlled batches with the following sequence:

- Batch 1 — Section 0: Document Control; Section 1: Annex Constitution
- Batch 2 — Section 2: Annex Package Strategy; Section 3: Source-of-Truth and Traceability Map
- Batch 3 — Section 4: Canonical Terminology and Technical Naming Rules; Section 5: Annex Family A — Interface and API Contract Pack
- Batch 4 — Section 6: Annex Family B — Event, State, and Runtime Contract Pack; Section 7: Annex Family C — Data, Persistence, and Migration Pack
- Batch 5 — Section 8: Annex Family D — Report and Output Pack; Section 9: Annex Family E — QA, Acceptance, and Red-Team Pack
- Batch 6 — Section 10: Annex Family F — AI-Agent Reference Pack; Section 11: Annex Family G — Pack Index, Versioning, and Change Control

---

### 11.13 Current Formal Entry

#### CCHANGE-038 — Current Entry
- **Version:** C1.0-Final-Draft-1  
- **Date:** April 14, 2026  
- **Changed Sections / Families:** Sections 0–11 initial assembled draft  
- **Change Type:** major  
- **Reason:** Creation of the first full technical annex pack subordinate to Document A — Master Project Creative Brief V2.A-Final-Draft-2 and Document B — Master Execution Plan B1.0-Final-Draft-2  
- **Summary of Change:** Established the initial annex constitution, pack strategy, traceability system, terminology discipline, interface contract family, runtime/state/event family, persistence/migration family, report/output family, QA/red-team family, AI-agent reference family, and pack-governance family for Document C  
- **Impact on Technical Contracts:** Canonical technical annex structure established  
- **Impact on QA / Validation / Revalidation:** Canonical validation-structure and revalidation framework established  
- **Impact on Documents A/B Alignment:** Document C explicitly aligned to the current live governing pair  
- **Upstream Revalidation Required:** no  

#### CCHANGE-038A — Governance Realignment Entry
- **Version:** C1.0-Final-Draft-2  
- **Date:** April 15, 2026  
- **Changed Sections / Families:** top metadata block, Section 2.5, Section 5.9, Section 5.13, Section 11.13, canonical footer  
- **Change Type:** moderate  
- **Reason:** Align Document C to the newly promoted upstream seed-intake constants and the locked MVP runtime/update split without expanding annex scope.  
- **Summary of Change:** Updated governing upstream version references to Document A V2.A-Final-Draft-3 and Document B B1.0-Final-Draft-3; formalized fixed seed-intake constants and minimum document-validation rejection families in the Preflight contract rules; and clarified that runtime/update contracts cover shell-owned managed runtime-package hydration plus native app-updater status and verification fields.  
- **Impact on Technical Contracts:** Preflight and runtime/update support contracts now carry explicit consumed constants and boundary distinctions rather than leaving them implicit.  
- **Impact on QA / Validation / Revalidation:** Phase 5 and later updater/runtime-trust validation must consume the fixed constants and explicit rejection families.  
- **Impact on Documents A/B Alignment:** Document C is now realigned to the current live governing pair.  
- **Upstream Revalidation Required:** no  

---

### 11.14 Version Progression Rules

#### CCHANGE-039 — Version Progression Rule
The project owner may adopt a predictable version progression such as:
- C1.0-Final-Draft-1
- C1.0-Final-Draft-2
- C1.0-Review-Draft-1
- C1.0-Release-Candidate-1
- C1.0-Locked

or an equivalent clearly governed versioning system.

#### CCHANGE-040 — Revision Discipline Rule
If Document C changes materially, the version, last-updated field, and change-log entry should be updated in the same revision cycle.

#### CCHANGE-041 — No Silent Semantic Change Rule
A locked annex family or contract meaning must not be changed semantically without:
- updating version metadata
- updating last-updated metadata
- adding a change-log entry
- revalidating affected families where needed

---

### 11.15 Pack Governance Integrity Prohibitions

#### CCHANGE-042 — No Silent Family Mutation Rule
Annex-family structure must not change silently.

#### CCHANGE-043 — No Silent Contract Mutation Rule
Technical contract meaning must not change silently.

#### CCHANGE-044 — No Silent State/Storage/Output Mutation Rule
State/event meaning, persistence boundaries, output meaning, and QA/validation structures must not change silently.

#### CCHANGE-045 — No Silent AI-Reference Mutation Rule
The AI-agent reference pack must not change silently if the change affects implementation safety or authority interpretation.

---

### 11.16 Boundaries of This Section

#### CCHANGE-046 — Section Boundary Rule
This section defines:
- pack indexing
- pack status and completeness tracking
- versioning rules
- change-log rules
- revalidation rules
- cross-family maintenance discipline
- initial structure and current-entry records

This section does not authorize changes to Documents A or B.
It governs Document C only.

---

### 11.17 Final Pack Governance Summary

#### CCHANGE-047 — Section Summary Rule
At the pack-governance level, Annex Family G formalizes how Document C is indexed, versioned, changed, revalidated, and maintained so that the engineering annex layer remains auditable, subordinate to the live governing pair, and resistant to silent technical drift.

#### CCHANGE-048 — Stability Rule
No locked decision inside Document C should be treated as changed unless this family says it changed.

---

## Technical-Annex Boundary Note

Document C is the governed technical-annex layer.

It may formalize, organize, and reference:
- contract shapes
- schemas
- state and event structures
- migration structures
- report templates
- QA and red-team matrix structures
- AI-agent technical reference material

It must not:
- redefine product scope
- redefine product behavior
- redefine execution order
- redefine acceptance meaning
- create a third independent authority layer beside Documents A and B

---
**Canonical Footer**  
Document: Document C — Engineering Annex Pack  
Document ID: DOC-C  
Version: C1.0-Final-Draft-2  
Governing Upstream Documents: Document A — Master Project Creative Brief V2.A-Final-Draft-3; Document B — Master Execution Plan B1.0-Final-Draft-3  
Change Authority: Section 11 — Annex Family G — Pack Index, Versioning, and Change Control  
Override Rule: If Document C conflicts with Document A or Document B, the higher-authority upstream document wins
---
