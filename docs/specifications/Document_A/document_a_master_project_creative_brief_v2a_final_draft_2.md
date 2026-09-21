# DOCUMENT A — MASTER PROJECT CREATIVE BRIEF

**Document ID:** DOC-A  
**Canonical Title:** Master Project Creative Brief  
**Version:** V2.A-Final-Draft-3  
**Status:** Draft – Under Review  
**Owner:** MIKE SALAZAR  
**Primary Audience:** Project Owner, AI Coding Agents, Human Reviewers  
**Authority Level:** Supreme Source-of-Truth  
**Last Updated:** April 15, 2026  
**Related Documents:** Document B — Master Execution Plan; Document C — Engineering Annex Pack

---

## Authority Notice

This document is the supreme source-of-truth for the project.

All later planning documents, execution plans, engineering annexes, prompts, implementation notes, code-generation instructions, and QA artifacts must remain subordinate to this document.

If any lower-level artifact conflicts with this document, this document wins unless it has been explicitly revised through **Section 18 — Change Log**.

---

## Document Usage Notice

This document is written to serve three purposes at once:

1. **Product authority**  
   It defines what the product is, what it is not, and what is locked.

2. **AI implementation authority**  
   It provides deterministic structure and constraints for AI coding agents.

3. **Audit authority**  
   It enables human reviewers to verify whether implementation still matches the intended product.

This document must be interpreted in descending authority order from earlier sections to later sections.

---

## Table of Contents

- [0. Document Control](#0-document-control)
- [1. Project Constitution](#1-project-constitution)
- [2. Executive Product Snapshot](#2-executive-product-snapshot)
- [3. Glossary and Canonical Terminology](#3-glossary-and-canonical-terminology)
- [4. Product Scope and User Outcomes](#4-product-scope-and-user-outcomes)
- [5. User Journeys and Behavioral Narratives](#5-user-journeys-and-behavioral-narratives)
- [6. Product Behavior Specification](#6-product-behavior-specification)
- [7. State Machines](#7-state-machines)
- [8. System Architecture](#8-system-architecture)
- [9. Data Lifecycle and Storage Policy](#9-data-lifecycle-and-storage-policy)
- [10. API and Interface Contract](#10-api-and-interface-contract)
- [11. Database and Commercial Backend Contract](#11-database-and-commercial-backend-contract)
- [12. UI/UX Contract](#12-uiux-contract)
- [13. Non-Functional Requirements](#13-non-functional-requirements)
- [14. Acceptance Criteria](#14-acceptance-criteria)
- [15. Red-Team Risks and Known Weaknesses](#15-red-team-risks-and-known-weaknesses)
- [16. Implementation Guardrails for AI Coding Agents](#16-implementation-guardrails-for-ai-coding-agents)
- [17. Open Questions and Deferred Decisions](#17-open-questions-and-deferred-decisions)
- [18. Change Log](#18-change-log)

---

## Unified Draft Assembly Note

This unified draft is assembled from previously drafted controlled batches.

During consolidation:
- all batch-specific wrapper text must be removed
- only one master title block must remain
- only one master version label must remain
- section numbering and rule IDs must remain intact unless a collision is discovered
- no semantic changes should be introduced unless explicitly approved and recorded in the Change Log

---

## 0. Document Control

### 0.1 Section Purpose
This section defines the identity, authority, governance, interpretation rules, and usage boundaries of Document A.

This section exists to ensure that:
- Document A is treated as the supreme source-of-truth document for the project.
- AI coding agents do not misinterpret lower-priority documents as override authority.
- Human reviewers can determine exactly how this document should be used, updated, and resolved in case of contradiction.

---

### 0.2 Document Identity

#### DOCCTRL-001 — Document Title
The official title of this document is:

**Master Project Creative Brief**

#### DOCCTRL-002 — Document Role
This document is the **primary authoritative product-and-system specification** for the project.

#### DOCCTRL-003 — Document Type
This document is a hybrid:
- product constitution
- behavioral contract
- architecture contract
- implementation-governing brief

It is not merely a narrative PRD, a pitch deck, or an engineering appendix.

#### DOCCTRL-004 — Document Objective
The objective of this document is to define the project in a manner that is:
- ultra-detailed
- deterministic
- hierarchical
- auditable
- usable by AI coding agents with minimal ambiguity

---

### 0.3 Governing Authority

#### DOCCTRL-005 — Supreme Source-of-Truth Rule
Document A is the **supreme source-of-truth document** for the project.

All later planning documents, engineering annexes, implementation artifacts, tickets, prompts, code-generation tasks, and QA workflows MUST derive from this document and MUST NOT contradict it.

#### DOCCTRL-006 — Governance Scope
Document A governs all of the following:
- product identity
- product scope
- system behavior
- system boundaries
- data boundaries
- interface expectations
- platform support
- user-facing obligations
- non-functional constraints
- acceptance conditions
- known limitations
- AI coding agent guardrails

#### DOCCTRL-007 — Non-Governing Areas
Document A does not serve as:
- the actual OpenAPI definition file
- the actual SQL migration package
- the actual QA execution spreadsheet
- the actual implementation backlog
- the actual UI design system
- the actual source code

Those artifacts may refine execution detail, but they must remain subordinate to this document.

---

### 0.4 Intended Audience

#### DOCCTRL-008 — Primary Audience
The primary audiences for Document A are:
1. the project owner
2. AI coding agents used to design, generate, and modify code
3. human reviewers validating implementation quality
4. future execution-planning documents

#### DOCCTRL-009 — AI Agent Interpretation Audience
This document is explicitly written to be consumed by AI coding agents.
Therefore:
- rules must be explicit
- definitions must be stable
- ambiguities must be minimized
- open questions must be labeled
- examples must not masquerade as requirements

---

### 0.5 Related Documents

#### DOCCTRL-010 — Document Relationship Map
The project documentation system consists of at minimum:

- **Document A — Master Project Creative Brief**  
  The supreme source-of-truth document.

- **Document B — Master Execution Plan**  
  The sequencing, milestone, dependency, and implementation-planning document.

- **Document C — Engineering Annex Pack**  
  The structured technical specification pack, including schemas, migrations, test matrices, and machine-readable interface details.

#### DOCCTRL-011 — Authority Over Document B
Document B MAY:
- translate Document A into execution order
- break work into phases
- create task groups
- define sequencing and dependencies

Document B MUST NOT:
- change product scope
- redefine behavior
- override locked decisions
- invent unauthorized features
- weaken or strengthen core claims beyond Document A

#### DOCCTRL-012 — Authority Over Document C
Document C MAY:
- formalize schemas
- provide OpenAPI
- provide SQL migrations
- provide QA test matrices
- provide event definitions
- provide report templates
- provide implementation reference material

Document C MUST NOT:
- redefine product purpose
- contradict locked decisions
- change source-of-truth terminology
- introduce new product obligations not sanctioned by Document A

---

### 0.6 Drafting and Interpretation Rules

#### DOCCTRL-013 — Descending Authority Drafting Rule
This document MUST be interpreted from top to bottom in descending authority order.

Earlier foundational sections govern later derived sections.

#### DOCCTRL-014 — Normative Language Rule
The following terms are normative and must be interpreted strictly:

- **MUST** = mandatory requirement
- **MUST NOT** = prohibited requirement
- **SHOULD** = recommended requirement unless strong reason exists otherwise
- **SHOULD NOT** = discouraged behavior unless justified
- **MAY** = optional behavior

#### DOCCTRL-015 — Non-Normative Language Rule
The following are non-normative unless explicitly labeled otherwise:
- examples
- explanatory notes
- rationale paragraphs
- implementation suggestions
- future ideas

#### DOCCTRL-016 — Example Labeling Rule
Any example included in this document MUST be clearly labeled as an example and MUST NOT be treated as binding behavior unless separately stated as a requirement.

#### DOCCTRL-017 — Ambiguity Rule
If a statement in this document appears ambiguous, the interpretation MUST prefer:
1. the narrower and safer meaning
2. the meaning most consistent with earlier constitutional sections
3. the meaning least likely to expand scope without authorization

#### DOCCTRL-018 — No Silent Invention Rule
If a required behavior is not explicitly defined, an AI coding agent MUST NOT silently invent a new product rule and present it as settled fact.
Instead, the missing item must be treated as:
- unspecified
- deferred
- or in need of clarification

---

### 0.7 Conflict Resolution Rules

#### DOCCTRL-019 — Internal Conflict Resolution Order
If any two sections in Document A appear to conflict, precedence MUST be resolved in the following order:

1. Section 0 — Document Control  
2. Section 1 — Project Constitution  
3. Section 3 — Glossary and Canonical Terminology  
4. Section 6 — Product Behavior Specification  
5. Section 7 — State Machines  
6. Section 8 — System Architecture  
7. Section 9 — Data Lifecycle and Storage Policy  
8. Section 10 — API and Interface Contract  
9. Section 11 — Database and Commercial Backend Contract  
10. Section 12 — UI/UX Contract  
11. Section 13 — Non-Functional Requirements  
12. Section 14 — Acceptance Criteria  
13. Section 15 — Red-Team Risks and Known Weaknesses  
14. Section 16 — Implementation Guardrails for AI Coding Agents  
15. Section 17 — Open Questions and Deferred Decisions  
16. Section 18 — Change Log

#### DOCCTRL-020 — Specific Over General Rule
Where a specific requirement and a general requirement both apply, the specific requirement governs only within its own scope, while the general requirement remains valid elsewhere.

#### DOCCTRL-021 — Earlier Locked Rule Wins
If a later section appears to weaken, expand, or reinterpret a locked rule from an earlier section, the earlier locked rule wins unless the document version is explicitly updated in the Change Log.

---

### 0.8 Canonical Terminology Governance

#### DOCCTRL-022 — Canonical Terminology Rule
All defined project terms in Section 3 are canonical.
Once a term is defined there, all later sections and all subordinate project documents MUST use that term consistently.

#### DOCCTRL-023 — Synonym Suppression Rule
AI coding agents and subordinate documents MUST NOT introduce new synonyms for canonical terms if doing so changes, blurs, or duplicates meaning.

#### DOCCTRL-024 — Terminology Drift Prohibition
The same system concept MUST NOT be described using multiple overlapping names across the project documentation stack unless explicitly mapped in the glossary.

---

### 0.9 Document Stability Rules

#### DOCCTRL-025 — Locked Decision Rule
Any rule explicitly marked or functionally treated as a locked project decision MUST be considered fixed unless changed in a future version with explicit change-log entry.

#### DOCCTRL-026 — Deferred Decision Rule
Any unresolved, future-phase, or optional item MUST be placed only in the appropriate deferred or open-questions section and MUST NOT be embedded as if it were already approved.

#### DOCCTRL-027 — No Hidden Scope Expansion Rule
This document MUST NOT hide new scope inside:
- examples
- notes
- rationale
- suggested implementation comments
- non-goal sections
- risk commentary

New scope is valid only when explicitly declared as a requirement.

---

### 0.10 Usage Rules for AI Coding Agents

#### DOCCTRL-028 — AI Usage Rule
Any AI coding agent using this document as context MUST treat it as:
- authoritative
- hierarchical
- deterministic
- constraint-bearing

#### DOCCTRL-029 — AI Compliance Rule
An AI coding agent MUST:
- obey locked scope
- preserve canonical terminology
- honor the authority hierarchy
- avoid unauthorized feature invention
- avoid stronger product claims than written here
- avoid softer failure handling than written here
- surface ambiguities rather than masking them

#### DOCCTRL-030 — AI Non-Override Rule
An AI coding agent MUST NOT override Document A based on:
- general best practices
- personal preference
- framework defaults
- speculative assumptions
- “common SaaS patterns”
unless Document A explicitly authorizes such behavior.

---

### 0.11 Traceability Rules

#### DOCCTRL-031 — Stable Identifier Rule
Major rules in this document SHOULD use stable identifiers so that downstream documents can reference them without ambiguity.

#### DOCCTRL-032 — Downstream Traceability Rule
Wherever possible, later documents and implementation plans SHOULD map work back to:
- constitutional rules
- behavioral rules
- state rules
- architecture rules
- acceptance criteria

#### DOCCTRL-033 — No Orphan Requirement Rule
No major requirement should exist in subordinate artifacts without an identifiable parent in Document A.

---

### 0.12 Boundaries of This Section

#### DOCCTRL-034 — Section Boundary Rule
This section defines only:
- document authority
- interpretation
- governance
- precedence
- usage rules

This section does not define:
- product features
- user journeys
- technical architecture
- API payloads
- UI details
- database structure

Those belong in later sections.

---

## 1. Project Constitution

### 1.1 Section Purpose
This section defines the non-negotiable truths of the project.

This section exists to prevent:
- scope drift
- undocumented reinterpretation
- exaggerated marketing claims
- architecture decisions that violate the product’s core contract
- AI coding agents improvising on fundamentals

All statements in this section are constitutional in nature and should be treated as the highest project-level truths after Section 0.

---

### 1.2 Product Identity

#### CONST-001 — Official Product Name
The official working product name is:

**Miro-Fish Desktop SaaS**

#### CONST-002 — Product Category
The product is a **desktop software application** with cloud-dependent inference behavior.

It is not a browser-only SaaS application.
It is not a server-hosted simulation service.
It is not a local-model inference app.
It is not an enterprise deployment suite in MVP form.

#### CONST-003 — Core Product Definition
The product is a premium desktop application that:
- runs on the user’s machine
- provides a local user interface and orchestration layer
- delegates LLM inference to user-supplied third-party provider APIs
- generates a structured simulation report from an uploaded English seed document
- uses best-effort ephemeral local storage during active execution

#### CONST-004 — Core Mission
The product exists to let users stress-test an idea, offer, product concept, educational asset, or positioning document through an AI-mediated moderated simulation without paying a vendor markup on inference usage.

#### CONST-005 — Product Framing Rule
The product MUST be framed as:
**local-data-first, cloud-inference-dependent, intermittently connected desktop software**

This phrasing is canonical for external and internal interpretation.

---

### 1.3 Supported Platform Scope

#### CONST-006 — Operating System Scope
The MVP product MUST support:
- Windows

#### CONST-007 — Excluded Operating System Scope
The MVP product MUST NOT support:
- macOS
- Linux
- ChromeOS
- iPadOS
- Android
- iOS

unless a later document version explicitly changes this.

#### CONST-008 — Desktop-Only Scope
The MVP product is desktop-first and desktop-only.
There is no requirement in MVP for:
- mobile apps
- browser-first product experience
- tablet-specific layout
- kiosk mode

---

### 1.4 Primary User Scope

#### CONST-009 — Primary User Type
The MVP target user is an individual operator, not an enterprise administrator.

#### CONST-010 — Primary Audience
The primary audiences are:
- vibe-coding learners
- solo founders
- solo marketers
- independent operators
- creators validating a concept or offer

#### CONST-011 — Secondary Audience Boundary
Small teams MAY use the product, but enterprise IT-managed deployment is not the design center for MVP.

#### CONST-012 — Enterprise Horizon Rule
Enterprise IT-managed deployment is a Horizon 2 concern and is explicitly out of MVP scope unless later approved.

---

### 1.5 Commercial Model

#### CONST-013 — Commercial Philosophy
The product uses a **bring-your-own-key** commercial model for AI inference.

#### CONST-014 — Inference Cost Responsibility
The user is responsible for third-party LLM API costs incurred during simulation runs.

#### CONST-015 — Vendor Compute Rule
The vendor does not provide hosted inference compute for the core simulation flow in MVP.

#### CONST-016 — License Model
The product is licensed commercially with at least these plan categories:
- monthly
- annual
- lifetime

#### CONST-017 — Activation Model
Each paid license MUST support at most two activated devices in MVP.

#### CONST-018 — Activation Binding Rule
Licenses MUST be device-bound through a device activation process rather than treated as unlimited transferable tokens.

---

### 1.6 Core Product Promise

#### CONST-019 — User Value Promise
The product MUST provide a user with the ability to:
1. activate the desktop app
2. configure a supported provider key
3. upload an English seed document
4. run a bounded multi-agent simulation
5. observe the simulation in real time
6. export a structured PDF report

#### CONST-020 — Core Output Promise
A successful or budget-capped run MUST produce a structured PDF report as defined later in this document.

#### CONST-021 — Real-Time Visibility Promise
The product MUST provide real-time visibility into simulation progress through live event-driven UI behavior.

#### CONST-022 — Local Control Promise
The product MUST keep orchestration, local UI logic, temporary run workspace handling, and report export on the user’s machine.

---

### 1.7 Locked Scope Decisions

The following decisions are constitutionally locked for the current product version.

#### CONST-023 — Windows Only
The MVP supports Windows only.

#### CONST-024 — English-Only Product Policy
The MVP is English-only in user-visible behavior and seed document acceptance.

#### CONST-025 — Non-English Rejection Rule
The system MUST reject non-English seed documents during Preflight rather than attempting multilingual simulation.

#### CONST-026 — No Crash Resume Rule
The MVP MUST NOT support resuming interrupted sessions after crash, power loss, or forced termination.

#### CONST-026A — Resume Clarification Rule
The prohibition on resume applies only to restoration of an interrupted run/session after crash, power loss, forced termination, or abnormal app closure.

This prohibition does not apply to bounded resumable recovery of runtime download or runtime hydration interruption before a run has started.

Runtime download or runtime hydration recovery is installation/hydration continuity, not run/session resume.

#### CONST-027 — Janitor Cleanup Rule
The MVP MUST rely on cleanup-on-normal-shutdown plus startup janitor cleanup on next launch after abnormal termination.

#### CONST-028 — Best-Effort Ephemeral Storage Rule
The MVP uses best-effort ephemeral local storage, not forensic-grade deletion guarantees.

#### CONST-029 — BYOK Rule
The MVP MUST use a bring-your-own-key model for OpenAI or Google provider access.

#### CONST-030 — Controlled Simulation Rule
The simulation MUST be moderated and bounded.
It is not an unbounded all-agents-speak-every-round swarm.

#### CONST-031 — Moderator Selection Rule
A Moderator Agent MUST orchestrate the simulation and select only a subset of agents to speak per round.

#### CONST-032 — Per-Round Speaker Bound
The Moderator-selected speaking subset MUST be 3 to 5 agents per round.

#### CONST-033 — Runtime Persona Generation Rule
Personas MUST be generated at runtime from the seed document context rather than configured as a static user-authored roster in MVP.

#### CONST-034 — Budget Governance Rule
The user MUST provide a max token budget before a run begins.

#### CONST-035 — Budget Exhaustion Finalization Rule
If the token budget is exhausted mid-run, the system MUST stop scheduling new rounds, enter controlled finalization, and attempt to generate a report from completed work.

#### CONST-036 — Duration Cap Rule
Each run MUST have a hard duration cap of two hours.

#### CONST-037 — Offline Limitation Rule
The product MUST NOT promise offline simulation execution.
Connectivity is required for cloud inference.

#### CONST-038 — Activation and Viewing Offline Rule
Offline use is limited to:
- local UI access where applicable
- offline license validation after activation
- viewing already-generated local reports

#### CONST-039 — Local Backend Boundary Rule
The local backend MUST be treated as an app-internal localhost service, not a public or remote service.

---

### 1.8 Explicit Non-Goals

The following are constitutionally out of scope for MVP.

#### CONST-040 — No Linux Goal
Linux support is not an MVP goal.

#### CONST-041 — No Session Recovery Goal
Recovery and continuation of a previously interrupted run is not an MVP goal.

#### CONST-042 — No Enterprise Fleet Goal
Enterprise device fleet provisioning, admin consoles, or organization-wide rollout controls are not MVP goals.

#### CONST-043 — No Multilingual Ingestion Goal
Multilingual seed-document ingestion is not an MVP goal.

#### CONST-044 — No Forensic Security Goal
Protection against all forensic recovery, disk imaging, OS indexing residue, or hostile local memory inspection is not an MVP goal.

#### CONST-045 — No Vendor-Hosted Simulation Goal
Running the core simulation in vendor-managed hosted infrastructure is not an MVP goal.

#### CONST-046 — No Fully Offline AI Goal
Fully offline AI inference is not an MVP goal.

#### CONST-047 — No Unlimited Device Sharing Goal
A license behaving as an unlimited device key is not an MVP goal.

#### CONST-048 — No User-Authored Static Persona System Goal
A manual system where the user designs the full agent roster is not an MVP goal.

#### CONST-049 — No Absolute Anti-Piracy Goal
Perfect anti-piracy enforcement, unpatchable binaries, or undefeatable DRM is not an MVP goal.

#### CONST-050 — No Perfect Security Marketing Goal
The product is not intended to claim military-grade, absolute, or perfectly secure local execution.

---

### 1.9 Allowed Product Claims

The following claim categories are allowed because they are consistent with the locked project truth.

#### CONST-051 — Allowed Locality Claim
The product MAY claim that orchestration and temporary workspace execution occur locally on the user’s machine.

#### CONST-052 — Allowed BYOK Claim
The product MAY claim that users bring their own API key and pay provider costs directly.

#### CONST-053 — Allowed Privacy Claim
The product MAY claim that the vendor does not intentionally store the user’s seed document or transcript in a vendor-hosted simulation database in MVP.

#### CONST-054 — Allowed Cleanup Claim
The product MAY claim that the app uses best-effort ephemeral local storage with normal shutdown cleanup and startup janitor cleanup after abnormal termination.

#### CONST-055 — Allowed Offline Claim
The product MAY claim offline capability only for:
- post-activation license validation
- local report viewing
- limited local app access that does not require inference

#### CONST-056 — Allowed Platform Claim
The product MAY claim support for Windows only.

#### CONST-057 — Allowed Simulation Claim
The product MAY claim that the app runs a moderated multi-agent simulation driven by a runtime-generated persona set.

#### CONST-058 — Allowed Budget Claim
The product MAY claim that user-defined token budget controls help bound spend.

#### CONST-059 — Allowed Limitation Disclosure
The product MAY and SHOULD explicitly disclose residual risks and MVP constraints as part of honest product communication.

---

### 1.10 Forbidden Product Claims

The following claims are constitutionally prohibited unless a future version explicitly changes the underlying capability.

#### CONST-060 — Forbidden Offline-First Claim
The product MUST NOT be described as a fully offline-first simulation product if that phrasing implies that simulations can run without internet access.

#### CONST-061 — Forbidden Full Offline AI Claim
The product MUST NOT claim that AI simulation runs fully offline.

#### CONST-062 — Forbidden Absolute Privacy Claim
The product MUST NOT claim that no trace can ever remain on a machine after execution.

#### CONST-063 — Forbidden Guaranteed Deletion Claim
The product MUST NOT claim guaranteed secure erasure, forensic-proof deletion, or irreversible elimination of all local artifacts.

#### CONST-064 — Forbidden Military-Grade Claim
The product MUST NOT use vague high-assurance security marketing phrases such as:
- military-grade
- unhackable
- zero-risk
- impossible to recover
unless those claims are formally justified and explicitly approved in a later version.

#### CONST-065 — Forbidden Enterprise-Ready Claim
The product MUST NOT imply enterprise IT-readiness, enterprise fleet governance, or enterprise-grade compliance support in MVP form.

#### CONST-066 — Forbidden Unlimited Scale Claim
The product MUST NOT imply unbounded simulation scale without feasibility checks, budget checks, and execution constraints.

#### CONST-067 — Forbidden Unlimited Device Claim
The product MUST NOT imply that one license permits unrestricted device activation.

#### CONST-068 — Forbidden Resume Claim
The product MUST NOT claim automatic crash recovery or resume-from-failure behavior in MVP.

#### CONST-069 — Forbidden Multilingual Claim
The product MUST NOT claim support for non-English seed documents in MVP.

#### CONST-070 — Forbidden Deterministic Outcome Claim
The product MUST NOT claim that the Market Viability Score is objective truth, guaranteed prediction accuracy, or deterministic market certainty.

---

### 1.11 Source-of-Truth Rules at the Project Level

#### CONST-071 — Locked Scope Supremacy
All implementation work MUST stay inside the locked scope of this constitution unless an explicit change is made in a future version of Document A.

#### CONST-072 — No Unauthorized Feature Expansion
Neither AI coding agents nor subordinate planning documents may introduce additional product features as if they were approved scope.

#### CONST-073 — No Unauthorized Weakening
No downstream document may weaken a locked requirement by reframing it as optional if this section defines it as mandatory.

#### CONST-074 — No Unauthorized Strengthening
No downstream document may strengthen a constrained or qualified claim into an absolute claim if this section explicitly avoids absolutes.

#### CONST-075 — Constitution Governs Architecture
If an architecture decision conflicts with a constitutional rule, the constitutional rule wins.

#### CONST-076 — Constitution Governs Execution Planning
If an execution strategy conflicts with a constitutional rule, the constitutional rule wins.

#### CONST-077 — Constitution Governs Prompting
If an AI coding prompt, automation, or implementation instruction conflicts with this section, this section wins.

---

### 1.12 Boundaries of This Section

#### CONST-078 — Section Boundary Rule
This section defines:
- project identity
- locked truths
- supported scope
- non-goals
- allowed claims
- forbidden claims
- constitutional boundaries

This section does not yet define:
- full user journey flows
- detailed state transitions
- endpoint payload structures
- database field schemas
- UI screen-by-screen obligations
- exhaustive acceptance criteria

Those will be defined in later sections.

---

### 1.13 Final Constitutional Summary

#### CONST-079 — Foundational Summary Rule
At the constitutional level, the project is defined as follows:

Miro-Fish Desktop SaaS is a Windows-only desktop application for individual operators that uses bring-your-own-key cloud inference to run a moderated, runtime-persona-generated, English-only simulation from an uploaded English seed document, with real-time visibility, bounded spend, report export, best-effort ephemeral local storage, no crash-resume support, and no absolute security or deletion claims.

#### CONST-080 — Constitutional Stability Rule
This summary is not a replacement for the detailed rules above, but all later sections MUST remain consistent with it.

---

## 2. Executive Product Snapshot

### 2.1 Section Purpose
This section provides a compressed, high-clarity orientation layer for both human reviewers and AI coding agents.

This section exists to ensure that:
- the product can be understood quickly before reading deeper sections
- the core operating model is visible in one place
- high-level meaning is anchored before lower-level technical detail appears
- AI coding agents can establish fast contextual awareness before entering behavioral, architectural, and schema-heavy sections

This section is descriptive and orienting, but it remains subordinate to:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology

If any statement in this section conflicts with a higher-authority section, the higher-authority section wins.

---

### 2.2 One-Page Product Definition

#### EXEC-001 — Compressed Product Definition
Miro-Fish Desktop SaaS is a Windows desktop application that lets a user upload an English seed document, run a moderated AI simulation using their own OpenAI or Google API key, observe the simulation in real time, and export a structured PDF report, while keeping orchestration and temporary execution workspace handling local to the user’s machine.

#### EXEC-002 — Product Operating Model
The product operates through the following high-level model:
1. the user activates the app
2. the user stores a provider API key locally in OS-native secure storage
3. the user uploads one English seed document
4. the app performs Preflight validation and feasibility planning
5. the app runs a moderated multi-agent simulation using cloud inference through the user’s provider account
6. the app streams progress and events to the local UI
7. the app finalizes and exports a PDF report
8. the app cleans up the temporary workspace

#### EXEC-003 — Product Framing
The product is not a hosted simulation SaaS.
The product is not a browser-only tool.
The product is not an offline local-model app.

The product is:
- local-data-first
- cloud-inference-dependent
- intermittently connected
- desktop-native in orchestration and UX behavior

---

### 2.3 What the Product Does

#### EXEC-004 — Core Job of the Product
The core job of the product is to help an individual user stress-test an idea or document through a bounded, moderated, AI-generated debate that yields a structured report.

#### EXEC-005 — Supported Seed Material Type
The seed material is a single uploaded English document that serves as the context foundation for the simulation.

#### EXEC-006 — Simulation Nature
The simulation is not a raw free-for-all swarm.
It is a **controlled moderated simulation** in which:
- a Moderator Agent governs flow
- personas are generated at runtime from the seed document
- only a subset of agents speaks in each round
- the run is bounded by budget, time, and execution constraints

#### EXEC-007 — Required User Value
The user must be able to leave a run with:
- a clearer synthesis of the idea
- a viability score
- ranked risks
- strongest arguments for and against
- a full transcript appendix
- token and cost summary

---

### 2.4 Who the Product Is For

#### EXEC-008 — Primary User Profile
The primary user is an individual operator who wants structured AI-assisted debate around a concept, offer, product, curriculum, or positioning asset.

#### EXEC-009 — Typical User Examples
Typical MVP users include:
- vibe-coding learners
- solo founders
- independent product builders
- solo marketers
- creators testing an offer or curriculum

#### EXEC-010 — User Capability Assumption
The product may be used by someone who is not an expert programmer.
Therefore:
- flows must be explicit
- UI state must be legible
- failure feedback must be understandable
- configuration must avoid unnecessary technical friction

#### EXEC-011 — Non-Primary User Boundary
The MVP is not optimized first for:
- enterprise administrators
- centralized IT governance
- fleet deployment operators
- multilingual research teams

---

### 2.5 Primary User Workflow Summary

#### EXEC-012 — Primary Workflow Summary
The primary user workflow is:

1. acquire the product license
2. install the desktop app
3. hydrate runtime if required
4. activate the device
5. store provider API key
6. upload seed document
7. set requested agent count, round count, and max token budget
8. run Preflight
9. accept approval or downgrade outcome
10. start simulation
11. observe real-time progress
12. allow natural completion or controlled budget-cap finalization
13. export report
14. close session and clean workspace

#### EXEC-013 — Workflow Design Principle
This workflow must be understandable without requiring the user to understand:
- Rust internals
- Python service orchestration
- vector database behavior
- event sequencing implementation
- process supervision mechanics

Those details are internal responsibilities, not user obligations.

---

### 2.6 Major System Components

#### EXEC-014 — Major Component Map
The product is composed of these major components:

1. **Frontend UI Layer**  
   A desktop UI built for configuration, progress visibility, transcript viewing, and report export.

2. **Rust/Tauri Application Shell**  
   The supervisor layer responsible for local orchestration, licensing validation, key retrieval, process management, and cleanup coordination.

3. **Python Simulation Engine**  
   The local execution engine responsible for Preflight analysis, persona generation, moderated simulation logic, event emission, and report data assembly.

4. **Cloud Licensing and Distribution Backend**  
   A commercial backend responsible for activation, device-bound JWT issuance, runtime distribution authorization, and update support.

5. **Third-Party LLM Provider Layer**  
   Cloud inference endpoints used through the user’s own provider API key.

#### EXEC-015 — Component Separation Principle
These components must remain separated by responsibility.
No component should silently absorb a responsibility assigned elsewhere.

---

### 2.7 Core Inputs

#### EXEC-016 — Required Run Inputs
A run depends on all of the following inputs:

- valid license state
- activated device state
- selected provider
- stored provider API key
- one uploaded English seed document
- requested agent count
- requested round count
- max token budget

#### EXEC-017 — Derived Inputs
The system derives additional internal inputs during Preflight and run execution, including:
- extracted document text
- detected language confidence
- estimated token usage
- estimated duration
- effective agent count
- effective round count
- concurrency plan
- runtime-generated personas
- round themes
- selected speakers per round

#### EXEC-018 — Input Validity Principle
The product must not assume that user input is directly executable.
Input must first be validated, interpreted, and, if required, downgraded into a feasible execution plan.

---

### 2.8 Core Outputs

#### EXEC-019 — Required Real-Time Outputs
During an active run, the product must provide at minimum:
- visible session status
- current phase visibility
- progress indicators
- round progression visibility
- transcript/event feed
- budget usage visibility
- interruption and failure feedback

#### EXEC-020 — Required Final Output
The final output of a successful or budget-capped run is a structured PDF report.

#### EXEC-021 — Required Report Contents
That report must contain:
- Executive Summary
- Market Viability Score
- Ranked Risks
- Strongest Pro Arguments
- Strongest Anti Arguments
- Cost and Token Summary
- Transcript Appendix

#### EXEC-022 — Output Integrity Principle
The final report must reflect the actual completed portion of the run.
It must not fabricate rounds that did not occur.

---

### 2.9 Definition of a Successful Product Outcome

#### EXEC-023 — User-Level Success Definition
At a high level, the product is successful when a user can:
1. activate and configure the app without ambiguity
2. submit a valid seed document
3. receive a clear Preflight decision
4. run a bounded simulation without catastrophic shell failure
5. observe the process in real time
6. receive a report with usable conclusions

#### EXEC-024 — System-Level Success Definition
At a system level, a run is successful when:
- Preflight passes
- the run starts correctly
- the run completes all planned rounds or reaches controlled budget-cap finalization
- report generation succeeds
- cleanup is attempted according to policy

#### EXEC-025 — Partial Success Definition
A run may still count as a valid completed outcome when:
- the token budget is hit
- the engine enters controlled finalization
- a valid partial-completion report is generated
- the terminal state reflects budget-capped completion

#### EXEC-026 — Non-Success Cases
The following do not count as successful product outcomes:
- non-English seed rejection
- provider key missing
- provider unreachable
- model unavailable
- terminal failure without report
- invalid license or device mismatch
- abandoned run with no generated output

---

### 2.10 Top Project Constraints

#### EXEC-027 — Constraint Summary
The product is defined by these top-level constraints:

- Windows only
- English-only seed acceptance
- BYOK inference only
- online connectivity required for simulation
- runtime persona generation
- moderated subset speaker selection per round
- max token budget mandatory
- hard two-hour duration cap
- no crash-resume support
- best-effort ephemeral cleanup, not forensic guarantees
- device-bound activation with max two devices per license

#### EXEC-028 — Constraint Purpose
These constraints are not incidental implementation choices.
They are intentional design boundaries that keep the MVP:
- buildable
- honest
- cost-bounded
- understandable
- enforceable

---

### 2.11 What the Product Is Not

#### EXEC-029 — Negative Definition Summary
The product is not:
- a generic AI chat wrapper
- an unlimited agent swarm toy
- an enterprise governance suite
- a browser SaaS dashboard
- a multilingual ingestion platform
- a guaranteed secure deletion tool
- a local offline LLM desktop app
- a crash-resumable long-running job processor

#### EXEC-030 — AI Interpretation Guard
AI coding agents must use these negative definitions to avoid inventing adjacent but out-of-scope capabilities.

---

### 2.12 Section Boundary

#### EXEC-031 — Boundary Rule
This section provides a compressed high-level orientation only.

This section does not define in full:
- detailed canonical terminology
- exact user journey step logic
- exact state machines
- endpoint payload structure
- database schema structure
- UI control-by-control obligations
- full acceptance criteria

Those are defined in later sections.

---

## 3. Glossary and Canonical Terminology

### 3.1 Section Purpose
This section defines the canonical meaning of critical project terms.

This section exists to ensure that:
- the same concept is not described with multiple competing names
- AI coding agents do not invent synonyms that alter meaning
- later sections use stable definitions
- subordinate documents inherit a common semantic base

All terms defined here are canonical unless explicitly superseded by a later version of this document through the Change Log.

---

### 3.2 Canonical Terminology Rules

#### TERM-001 — Canonical Term Rule
A term defined in this section has a fixed project meaning.
All later sections and subordinate documents must use that meaning.

#### TERM-002 — Synonym Restriction Rule
A subordinate document or AI coding agent MUST NOT replace a canonical term with a new synonym if doing so changes or blurs scope.

#### TERM-003 — Context Override Restriction
A term’s meaning is not to be inferred from casual surrounding prose if this glossary already defines it.

#### TERM-004 — One Concept, One Primary Name
Each major concept should have one primary canonical name.
Alternate names should be avoided unless explicitly mapped.

#### TERM-005 — State Name Integrity Rule
State names must be used exactly as defined.
They must not be paraphrased in ways that alter technical meaning.

---

### 3.3 Core Product Terms

#### TERM-006 — Product
**Product** means the software system described by this document under the working name Miro-Fish Desktop SaaS.

#### TERM-007 — App
**App** means the installed desktop application as experienced by the user on Windows.

#### TERM-008 — App Shell
**App Shell** means the desktop-hosting layer responsible for local orchestration and system integration.
In this project, this primarily refers to the Rust/Tauri supervisory layer.

#### TERM-009 — Frontend
**Frontend** means the user-facing interface layer rendered inside the desktop application.
It is responsible for displaying state, accepting user input, and presenting outputs.

#### TERM-010 — Local Backend
**Local Backend** means the Python engine process running on the user’s machine and exposed as an app-internal localhost service.
It is not a public cloud API and is not intended for external third-party consumption.

#### TERM-011 — Cloud Backend
**Cloud Backend** means the remote commercial support layer used for licensing, activation, distribution, and related non-simulation commercial functions.

#### TERM-012 — Provider
**Provider** means the third-party LLM service selected by the user for inference.
For MVP, provider means either:
- OpenAI
- Google

#### TERM-013 — Model
**Model** means the concrete provider model selected for inference.
For MVP, the default allowed models are:
- `gpt-4o-mini`
- `gemini-1.5-flash`

---

### 3.4 User and Commercial Terms

#### TERM-014 — User
**User** means the person operating the app locally and supplying the third-party API key.

#### TERM-015 — License
**License** means the commercial entitlement granting the user the right to activate and use the app within plan and device constraints.

#### TERM-016 — Activation
**Activation** means the process by which a specific device is bound to a paid license and issued a device-bound JWT.

#### TERM-017 — Activated Device
**Activated Device** means a machine that has successfully completed the activation process and holds a valid device-bound license token.

#### TERM-018 — Device Hash
**Device Hash** means the privacy-conscious hashed machine identifier used to bind a device activation to a specific machine without storing raw hardware identifiers as the canonical backend record.

#### TERM-019 — JWT
**JWT** means the signed license token used by the app for post-activation offline validation of entitlement and device binding.

#### TERM-020 — Plan
**Plan** means the commercial license tier, such as monthly, annual, or lifetime.

---

### 3.5 Input and Document Terms

#### TERM-021 — Seed Document
**Seed Document** means the single user-uploaded English document that serves as the primary source context for one simulation run.

#### TERM-022 — Seed Content
**Seed Content** means the usable extracted text derived from the seed document during Preflight and ingestion.

#### TERM-023 — Supported Seed Document
**Supported Seed Document** means a seed document that:
- uses one of the approved MVP file types: PDF (`.pdf`), DOCX (`.docx`), TXT (`.txt`), or Markdown (`.md`)
- is at or below 10 MiB (10,485,760 bytes)
- yields extractable text
- produces at least 1,000 normalized extracted characters
- passes English-language validation with English-confidence of 0.80 or higher

#### TERM-024 — Non-English Seed Document
**Non-English Seed Document** means any uploaded document whose detected language is not English or whose English-confidence score is below 0.80.

#### TERM-025 — Preflight
**Preflight** means the mandatory validation and feasibility-planning phase that occurs before a run is allowed to start.

Preflight includes, at minimum:
- entitlement validation
- provider readiness checks
- runtime integrity checks
- document validation
- language detection
- cost and duration estimation
- feasibility evaluation
- downgrade-or-reject decisioning

#### TERM-026 — Preflight Approval
**Preflight Approval** means the outcome where the requested run can proceed without execution-plan downgrade.

#### TERM-027 — Preflight Approval With Downgrade
**Preflight Approval With Downgrade** means the outcome where the run may proceed only under a reduced effective execution plan.

#### TERM-028 — Preflight Rejection
**Preflight Rejection** means the outcome where the run may not proceed.

---

### 3.6 Simulation Terms

#### TERM-029 — Simulation
**Simulation** means the moderated AI-driven debate process performed on the basis of the seed document and executed through provider inference.

#### TERM-030 — Simulation Run
**Simulation Run** or **Run** means one complete execution unit beginning after successful Preflight approval and ending in a terminal run state.

A run is distinct from:
- app installation
- app launch
- device activation
- report viewing
- provider key setup

#### TERM-031 — Session
**Session** means the runtime-tracked local execution context for a specific run, including its ID, state, budget counters, event stream context, and workspace association.

In this project, a session is the run-scoped operational container, not merely “time spent in the app.”

#### TERM-032 — Workspace
**Workspace** means the isolated temporary local directory allocated uniquely to one run for transient execution artifacts.

#### TERM-033 — Workspace ID
**Workspace ID** means the UUID identifier associated with a specific workspace.

#### TERM-034 — Persona
**Persona** means a runtime-generated simulation participant profile derived from the seed document context.

A persona is not a human user account and is not a permanently stored profile in MVP.

#### TERM-035 — Persona Set
**Persona Set** means the total set of runtime-generated personas created for a run, including the Moderator Agent and participant personas.

#### TERM-036 — Moderator Agent
**Moderator Agent** means the special orchestration persona responsible for:
- selecting speakers
- framing round themes
- managing debate flow
- coordinating summaries
- driving finalization output

#### TERM-037 — Participant Agent
**Participant Agent** means any persona other than the Moderator Agent that may be selected to contribute argumentation during a round.

#### TERM-038 — Requested Agent Count
**Requested Agent Count** means the number of agents the user asks for before feasibility planning.

#### TERM-039 — Effective Agent Count
**Effective Agent Count** means the agent count the system actually uses after Preflight planning and any allowed downgrade.

#### TERM-040 — Requested Round Count
**Requested Round Count** means the number of rounds the user asks for before feasibility planning.

#### TERM-041 — Effective Round Count
**Effective Round Count** means the number of rounds the system actually uses after Preflight planning and any allowed downgrade.

#### TERM-042 — Round
**Round** means one discrete debate cycle in which:
- the Moderator sets the thematic focus
- 3 to 5 participant agents are selected
- those selected speakers contribute outputs
- the system updates summaries and counters

#### TERM-043 — Speaker Set
**Speaker Set** means the Moderator-selected subset of participant agents allowed to speak in a given round.

#### TERM-044 — Per-Round Speaker Bound
**Per-Round Speaker Bound** means the fixed allowed speaker-count range for a round, which in MVP is 3 to 5 agents.

#### TERM-045 — Debate Theme
**Debate Theme** means the topical focus assigned by the Moderator to the current round.

#### TERM-046 — Finalization
**Finalization** means the controlled closing phase of a run in which the system stops normal round scheduling, synthesizes accumulated material, computes the Market Viability Score, assembles report content, and transitions toward completion.

#### TERM-047 — Market Viability Score
**Market Viability Score** means the Moderator-generated numeric output from 0 to 100 representing the model-mediated evaluation of the debated concept.

It is a product output, not an objective external truth claim.

---

### 3.7 Budget, Time, and Feasibility Terms

#### TERM-048 — Max Token Budget
**Max Token Budget** means the user-defined maximum token ceiling the run is allowed to consume before forced finalization behavior is triggered.

#### TERM-049 — Budget Usage
**Budget Usage** means the cumulative token consumption tracked during the run.

#### TERM-050 — Budget Hit
**Budget Hit** means the condition where cumulative token usage reaches or exceeds the user’s max token budget threshold.

#### TERM-051 — Budget-Capped Completion
**Budget-Capped Completion** means a valid completion path in which the system ends normal round scheduling due to budget hit, enters finalization, and still generates a report successfully.

#### TERM-052 — Duration Cap
**Duration Cap** means the hard upper execution time limit for a run.
For MVP, that limit is two hours.

#### TERM-053 — Feasible Plan
**Feasible Plan** means an execution plan that fits within the current product constraints, including:
- provider availability
- runtime integrity
- budget tolerance
- duration tolerance
- bounded moderated execution model

#### TERM-054 — Downgrade
**Downgrade** means the controlled reduction of requested execution scope into a smaller feasible plan, typically by reducing effective agent count and/or effective round count.

#### TERM-055 — Rejection
**Rejection** means a deterministic decision that the requested run cannot proceed.

---

### 3.8 Runtime, Process, and Cleanup Terms

#### TERM-056 — Runtime Hydration
**Runtime Hydration** means the first-launch or repair-time acquisition and installation of the heavy Python runtime package required for simulation execution.

#### TERM-057 — Runtime Integrity
**Runtime Integrity** means the verified correctness of the installed runtime package, including expected files, compatible version state, and checksum or signature-based validation as defined later.

#### TERM-058 — Normal Shutdown
**Normal Shutdown** means an orderly app close or session end in which the app has an opportunity to instruct the engine to terminate and to attempt workspace cleanup.

#### TERM-059 — Abnormal Termination
**Abnormal Termination** means app interruption without orderly shutdown, including:
- crash
- force kill
- sudden OS termination
- power loss

#### TERM-060 — Cleanup
**Cleanup** means the process of deleting the temporary workspace and related transient run artifacts according to policy.

#### TERM-061 — Startup Janitor
**Startup Janitor** means the startup-time process that scans for abandoned workspaces from prior abnormal terminations and attempts to remove them before new runs begin.

#### TERM-062 — Anti-Zombie Supervision
**Anti-Zombie Supervision** means the process-management behavior that attempts to prevent or terminate orphaned engine processes after app closure or run end.

---

### 3.9 Interface and Event Terms

#### TERM-063 — Endpoint
**Endpoint** means a defined local backend interface route exposed by the Python engine for app-internal use.

#### TERM-064 — Event
**Event** means a structured unit of run progress information emitted by the backend for transcript continuity, UI updates, and traceable run visibility.

#### TERM-065 — Event Stream
**Event Stream** means the ordered sequence of events associated with a specific session.

#### TERM-066 — Sequence Number
**Sequence Number** means the monotonically increasing integer assigned to each event in a session to preserve ordering and retrieval continuity.

#### TERM-067 — Status Response
**Status Response** means the structured backend representation of current session condition, counters, and progress visibility.

#### TERM-068 — App Session Token
**App Session Token** means the local app-to-backend authorization token used to ensure the local backend is not treated as an unauthenticated localhost service.

---

### 3.10 Output and Report Terms

#### TERM-069 — Report
**Report** means the final structured PDF output generated for a successful or budget-capped run.

#### TERM-070 — Report Export
**Report Export** means the act of producing the final user-accessible PDF file from finalized run data.

#### TERM-071 — Transcript Appendix
**Transcript Appendix** means the report section containing the ordered chronological log of simulation dialogue and related emitted content.

#### TERM-072 — Executive Summary
**Executive Summary** means the high-level synthesis section of the report.

#### TERM-073 — Ranked Risks
**Ranked Risks** means the ordered set of top identified flaws or risk themes surfaced by the run.

#### TERM-074 — Strongest Pro Arguments
**Strongest Pro Arguments** means the report section capturing the most persuasive arguments in favor of the seed concept.

#### TERM-075 — Strongest Anti Arguments
**Strongest Anti Arguments** means the report section capturing the most persuasive arguments against the seed concept.

#### TERM-076 — Cost and Token Summary
**Cost and Token Summary** means the report section showing run token usage and estimated provider cost.

---

### 3.11 State and Outcome Terms

#### TERM-077 — State
**State** means a formally defined lifecycle condition of a subsystem, especially a session, activation process, or cleanup process.

#### TERM-078 — Terminal State
**Terminal State** means a state after which the main execution path of the relevant lifecycle is considered finished and does not continue normal forward operation.

#### TERM-079 — Successful Completion
**Successful Completion** means a terminal outcome in which the report is generated successfully, whether through normal completion or budget-capped completion.

#### TERM-080 — Partial Success
**Partial Success** means a bounded non-ideal but still valid outcome in which the run does not complete all originally requested work yet still finalizes and produces a usable report.

#### TERM-081 — Failure
**Failure** means a terminal outcome in which the product does not achieve a valid report-producing completion for the run.

#### TERM-082 — Canceled
**Canceled** means a terminal outcome produced by controlled user- or app-initiated termination rather than uncontrolled crash behavior.

#### TERM-083 — Cleaned
**Cleaned** means the outcome in which the workspace cleanup attempt succeeded.

#### TERM-084 — Cleanup Failed
**Cleanup Failed** means the outcome in which the workspace cleanup attempt did not fully succeed.

#### TERM-085 — CompletedBudgetCapped
**CompletedBudgetCapped** means the specific terminal run state in which:
- token budget was exhausted or the run was otherwise intentionally truncated into controlled finalization
- finalization succeeded
- the report was generated successfully
- the run is considered valid but constrained

This state name must be used exactly as written.

---

### 3.12 Language and Claim Terms

#### TERM-086 — English-Only Policy
**English-Only Policy** means the rule that:
- user-visible product behavior is English only
- seed document acceptance is English only
- visible simulation output is English only
- report output is English only

#### TERM-087 — Best-Effort Ephemeral Storage
**Best-Effort Ephemeral Storage** means the policy that the app uses temporary local storage for runs and attempts cleanup on normal shutdown and next-launch janitor sweep, while explicitly not guaranteeing forensic-grade artifact elimination.

#### TERM-088 — Local-Data-First
**Local-Data-First** means the product keeps orchestration and temporary run-local handling on the user’s machine rather than persisting seed and transcript data into a vendor-hosted simulation database in MVP.

#### TERM-089 — Cloud-Inference-Dependent
**Cloud-Inference-Dependent** means the product requires live third-party provider access to perform simulation inference.

#### TERM-090 — Intermittently Connected
**Intermittently Connected** means some app features may remain accessible locally while active simulation behavior still depends on network availability.

---

### 3.13 Canonical Distinctions

#### TERM-091 — Run vs Session
A **run** is the execution unit.
A **session** is the runtime-tracked operational container for that run.

In practice they are tightly paired, but they are not semantically identical.

#### TERM-092 — Activation vs License
A **license** is the entitlement.
An **activation** is a device-specific binding event under that entitlement.

#### TERM-093 — Requested vs Effective
**Requested** values are user input.
**Effective** values are the system-approved values actually used for execution.

#### TERM-094 — Completion vs Cleanup
A run may be complete even before cleanup is fully confirmed.
Cleanup is a subsequent post-run responsibility.

#### TERM-095 — Local Backend vs Cloud Backend
The **local backend** runs on the user’s machine for simulation execution.
The **cloud backend** supports licensing, activation, and distribution functions, not core simulation execution.

---

### 3.14 Forbidden Terminology Drift

#### TERM-096 — Forbidden Casual Replacements
The following casual replacements should be avoided because they can blur meaning:
- using “job” instead of “run” unless explicitly mapped
- using “agent pool” instead of “persona set” if meaning changes
- using “cache folder” instead of “workspace”
- using “subscription token” instead of “device-bound JWT”
- using “resume” when only restart is possible
- using “offline mode” when only offline viewing or validation is supported

#### TERM-097 — Marketing Drift Prohibition
Terminology must not drift into exaggerated phrases that imply capabilities the product does not have, such as:
- secure vault simulation
- permanent privacy guarantee
- autonomous market truth engine
- full offline swarm intelligence

---

### 3.15 Section Boundary

#### TERM-098 — Boundary Rule
This section defines terminology only.

This section does not:
- establish full user journey steps
- define exact state transition diagrams
- specify endpoint payload fields in full
- define exact SQL schema
- define UI layouts
- define acceptance test procedures

Those belong in later sections.

---

### 3.16 Final Semantic Summary

#### TERM-099 — Semantic Summary Rule
After this section, the following terms should be treated as fully anchored for the rest of the document:
- app
- local backend
- cloud backend
- license
- activation
- device hash
- seed document
- Preflight
- run
- session
- workspace
- persona
- Moderator Agent
- effective plan
- max token budget
- finalization
- CompletedBudgetCapped
- cleanup
- startup janitor
- report
- English-Only Policy
- best-effort ephemeral storage

#### TERM-100 — Semantic Stability Rule
All later sections MUST remain semantically consistent with these definitions unless a future version explicitly updates them through the Change Log.

---

## 4. Product Scope and User Outcomes

### 4.1 Section Purpose
This section defines:
- the core problem the product exists to solve
- the specific value the MVP must deliver
- the use cases that are in scope
- the use cases that are out of scope
- the minimum required product outcomes
- the limitations that are acceptable for MVP

This section exists to prevent:
- scope drift
- “adjacent feature” invention by AI coding agents
- dilution of the MVP into a generic AI workbench
- confusion between “interesting future possibilities” and “required product behavior”

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology

If any statement in this section conflicts with a higher-authority section, the higher-authority section wins.

---

### 4.2 Core User Problem

#### SCOPE-001 — Primary Problem Statement
The primary problem this product exists to solve is:

An individual operator wants to pressure-test an idea, offer, curriculum, positioning asset, or product concept using structured AI debate, but does not want to:
- pay vendor markup on inference
- rely on a vague single-model chat exchange
- lose visibility into how the conclusion was formed
- expose seed material to a vendor-hosted simulation platform
- operate an enterprise-grade workflow stack just to validate one idea

#### SCOPE-002 — Problem Shape
The problem is not merely “the user wants AI.”
The actual problem is that the user needs:
- structured adversarial or multi-perspective evaluation
- bounded cost
- visible reasoning progression through user-visible dialogue
- a usable artifact at the end of the run
- enough control to trust the output path without becoming a system administrator

#### SCOPE-003 — MVP Relevance Rule
All MVP feature scope must support solving the primary problem defined above.
Features that do not materially strengthen that job-to-be-done are presumptively out of scope unless explicitly approved later.

---

### 4.3 Intended User Value

#### SCOPE-004 — Value Outcome Rule
The MVP must create the following value for the user:

1. **Compression of ambiguity**  
   The product must turn an unstructured concept into a structured evaluative debate.

2. **Multi-perspective scrutiny**  
   The product must expose strengths, weaknesses, objections, and supportive reasoning from multiple runtime-generated personas.

3. **Cost-bounded experimentation**  
   The product must allow the user to bound spend using a max token budget.

4. **Legible process**  
   The user must be able to observe progress and dialogue in real time.

5. **Portable artifact output**  
   The user must receive a structured PDF report that can be saved, reviewed, or shared outside the app.

#### SCOPE-005 — Value Delivery Standard
The MVP is not required to produce perfect truth.
It is required to produce a bounded, legible, structured, useful simulation outcome consistent with the seed document and execution constraints.

---

### 4.4 In-Scope Use Cases

The following use cases are explicitly in scope for MVP.

#### SCOPE-006 — Concept Validation Use Case
A user uploads a concept document and wants the system to debate whether the concept appears viable, risky, weakly positioned, or promising.

#### SCOPE-007 — Offer Evaluation Use Case
A user uploads copy or outline material for an offer and wants structured pro/anti argumentation around value, market fit, and objections.

#### SCOPE-008 — Curriculum Evaluation Use Case
A user uploads a course, module, syllabus, or educational outline and wants the system to simulate reactions from relevant personas such as learners, practitioners, marketers, or skeptics.

#### SCOPE-009 — Messaging and Positioning Use Case
A user uploads messaging or positioning material and wants the system to surface likely strengths, confusion points, risks, and counterarguments.

#### SCOPE-010 — Solo Founder Review Use Case
A solo founder uploads a product or business concept and wants a structured report to inform next-step thinking.

#### SCOPE-011 — Real-Time Observation Use Case
A user wants to watch the debate unfold in real time rather than receiving only a final black-box conclusion.

#### SCOPE-012 — Bounded Experiment Use Case
A user wants to control token spending by providing a max token budget and accepting that a run may finalize early if the budget is exhausted.

#### SCOPE-013 — Local Report Retention Use Case
A user wants to keep the exported report locally and access it later even when not actively running a simulation.

---

### 4.5 Out-of-Scope Use Cases

The following use cases are explicitly out of scope for MVP.

#### SCOPE-014 — General Chat Assistant Use Case
The product is not intended to function as a general-purpose assistant or open-ended conversational chatbot.

#### SCOPE-015 — Multilingual Research Use Case
The product is not intended to analyze or simulate non-English seed documents in MVP.

#### SCOPE-016 — Enterprise Deployment Use Case
The product is not intended to serve as a centrally managed enterprise deployment platform in MVP.

#### SCOPE-017 — Team Collaboration Suite Use Case
The product is not intended to provide collaborative multi-user editing, shared run sessions, team comments, or shared workspace coordination in MVP.

#### SCOPE-018 — Hosted SaaS Dashboard Use Case
The product is not intended to be used as a browser-hosted central dashboard where simulation happens remotely on vendor infrastructure.

#### SCOPE-019 — Resume-Long-Jobs Use Case
The product is not intended to provide robust resume-after-crash processing for interrupted runs.

#### SCOPE-020 — User-Curated Agent Design Use Case
The product is not intended to let the user manually design, save, edit, and reuse a full static persona roster in MVP.

#### SCOPE-021 — Local Offline Inference Use Case
The product is not intended to perform fully offline local-model simulation inference in MVP.

#### SCOPE-022 — Perfect Local Forensics Protection Use Case
The product is not intended to defeat all local forensic or OS-level residue risks in MVP.

#### SCOPE-023 — Unlimited Device Portability Use Case
The product is not intended to behave like a freely shareable license key across unlimited machines.

---

### 4.6 Scope Boundaries for Inputs

#### SCOPE-024 — Single Seed Document Rule
Each run is scoped to one seed document only.

The MVP does not require:
- multi-document run aggregation
- folder ingestion
- drag-in batch document sets
- cross-document comparison within one run

#### SCOPE-025 — User Input Scope
The user-configurable run inputs in MVP are limited to:
- requested agent count
- requested round count
- max token budget
- provider selection
- provider API key
- seed document selection

#### SCOPE-026 — No Advanced Prompting Requirement
The MVP does not require a user-facing advanced prompt editor for:
- moderator prompt editing
- persona prompt editing
- system override editing
- custom memory strategy editing

#### SCOPE-027 — No Manual Concurrency Control Requirement
The user is not required to configure:
- concurrency ceilings
- retry policy
- provider backoff behavior
- memory compression strategy
- event sequencing

Those are internal system responsibilities.

---

### 4.7 Scope Boundaries for Outputs

#### SCOPE-028 — Required Output Scope
The MVP output scope is limited to:
- live in-app run visibility
- final structured PDF report
- local retention of that exported file
- clear terminal state reporting

#### SCOPE-029 — No Hosted Report Portal Requirement
The MVP does not require a vendor-hosted report library, cloud report sync, or browser-accessible report account portal.

#### SCOPE-030 — No Spreadsheet or Slide Output Requirement
The MVP does not require report export to:
- spreadsheets
- slide decks
- dashboards
- BI tools

The required output format is PDF.

#### SCOPE-031 — No Cross-Run Comparative Analytics Requirement
The MVP does not require:
- comparing multiple prior runs
- longitudinal scoring dashboards
- benchmark trend visualization across multiple reports

---

### 4.8 Required Product Outcomes

#### SCOPE-032 — Outcome Layering Rule
The required outcomes of the MVP are divided into:
- user outcomes
- system outcomes
- artifact outcomes

#### SCOPE-033 — Required User Outcome
The user must be able to submit a valid run request and understand, at each major stage, what is happening.

#### SCOPE-034 — Required Execution Outcome
The system must transform a valid run request into either:
- approved execution
- approved-with-downgrade execution
- or deterministic rejection with reason

#### SCOPE-035 — Required Run Outcome
A valid run must either:
- complete normally
- complete under budget-capped finalization
- cancel in a controlled way
- or fail in a clearly surfaced terminal state

#### SCOPE-036 — Required Report Outcome
A successful or budget-capped run must produce the required PDF report sections.

#### SCOPE-037 — Required Cleanup Outcome
The system must attempt cleanup after run termination according to the project’s best-effort ephemeral storage policy.

#### SCOPE-038 — Required Local Control Outcome
The user must not need to understand hidden infrastructure decisions to operate the product successfully.

---

### 4.9 Acceptable MVP Limitations

The following limitations are acceptable in MVP and must not be treated as product defects merely because they exist.

#### SCOPE-039 — Acceptable No-Resume Limitation
It is acceptable that interrupted runs cannot be resumed and must be restarted.

#### SCOPE-040 — Acceptable English-Only Limitation
It is acceptable that non-English seed documents are rejected.

#### SCOPE-041 — Acceptable Best-Effort Cleanup Limitation
It is acceptable that the app attempts cleanup without claiming guaranteed forensic-grade deletion.

#### SCOPE-042 — Acceptable Activation Friction Limitation
It is acceptable that device-bound activation adds some setup friction because license portability is intentionally bounded.

#### SCOPE-043 — Acceptable Downgrade Limitation
It is acceptable that the system downgrades requested scale to a feasible plan rather than blindly honoring the user’s original request.

#### SCOPE-044 — Acceptable Controlled Partial Completion Limitation
It is acceptable that a run may finalize early due to budget or duration cap if the report still reflects completed work honestly.

#### SCOPE-045 — Acceptable Runtime Hydration Limitation
It is acceptable that first launch may involve runtime hydration instead of shipping the entire heavy runtime in the base installer.

#### SCOPE-046 — Acceptable Non-Collaborative Limitation
It is acceptable that the MVP is designed for a single operator rather than collaborative teams.

---

### 4.10 Unacceptable Outcome Classes

The following outcome classes are unacceptable for MVP.

#### SCOPE-047 — Unacceptable Silent Failure
The system must not fail silently when:
- Preflight rejects
- the provider is unreachable
- model availability fails
- budget is hit
- finalization fails
- cleanup fails

#### SCOPE-048 — Unacceptable Hidden Downgrade
The system must not downgrade execution without making the downgrade explicit to the user.

#### SCOPE-049 — Unacceptable Fabricated Completion
The system must not present an incomplete run as if it completed all originally planned rounds when it did not.

#### SCOPE-050 — Unacceptable False Security Messaging
The system must not imply stronger data erasure or local security guarantees than the product actually provides.

#### SCOPE-051 — Unacceptable Scope Drift
The system must not behave like a generic AI workspace, multi-doc research suite, or enterprise management platform without explicit future re-scoping.

#### SCOPE-052 — Unacceptable Opaque Spend Behavior
The system must not let a run proceed without a max token budget or without exposing budget state during execution.

#### SCOPE-053 — Unacceptable Unbounded Swarm Behavior
The system must not attempt an unmoderated full-fanout all-agents-speak design in MVP.

---

### 4.11 MVP Product Success Outcomes

#### SCOPE-054 — Minimum MVP Success Condition
The MVP is considered product-valid only if a user can:
1. activate the app
2. configure a provider key
3. upload a valid English seed document
4. receive a deterministic Preflight result
5. run a moderated simulation
6. see progress in real time
7. export a structured report
8. close the app with cleanup attempted according to policy

#### SCOPE-055 — Buildability Rule
All scope choices must preserve implementation buildability for AI-led development.
If a feature significantly increases ambiguity without materially improving the core outcome, it should remain out of scope.

---

### 4.12 Section Boundary

#### SCOPE-056 — Boundary Rule
This section defines:
- the user problem
- intended user value
- in-scope use cases
- out-of-scope use cases
- required outcome classes
- acceptable MVP limitations
- unacceptable outcome classes

This section does not yet define:
- exact step-by-step user flows
- exact state transitions
- exact endpoint structures
- exact database schema
- exact UI screen obligations
- exact testable acceptance criteria

Those are defined in later sections.

---

## 5. User Journeys and Behavioral Narratives

### 5.1 Section Purpose
This section defines the deterministic end-to-end user journeys of the product.

This section exists to ensure that:
- the product flow is understandable from the user’s point of view
- AI coding agents know the expected sequence of events
- key flows are explicit before implementation details appear
- failure branches are intentionally designed rather than improvised

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- Section 4 — Product Scope and User Outcomes

If any journey described here conflicts with a higher-authority section, the higher-authority section wins.

---

### 5.2 Journey Format Rule

#### FLOW-001 — Journey Structure Rule
Each journey in this section must define:
- trigger
- preconditions
- ordered steps
- success outcome
- failure branches
- user-visible feedback expectations
- relevant state implications where applicable

#### FLOW-002 — Behavioral Intent Rule
These journeys define user-visible and workflow-visible behavior.
They do not replace later state-machine or API-level detail, but they must remain consistent with those later sections.

---

### 5.3 Journey A — Purchase and Acquisition

#### FLOW-003 — Journey Name
**Purchase and Acquisition Journey**

#### FLOW-004 — Trigger
The journey begins when a prospective user decides to acquire the product.

#### FLOW-005 — Preconditions
The user:
- does not yet have an activated local app state
- may or may not already have a provider API key
- intends to obtain a valid commercial entitlement

#### FLOW-006 — Ordered Steps
1. The user visits the product’s commercial purchase surface.
2. The user selects a plan.
3. The user completes payment.
4. The commercial backend records the purchase event.
5. The user gains access to the desktop app acquisition path and activation eligibility.
6. The user downloads the desktop installer.

#### FLOW-007 — Success Outcome
The purchase journey is successful when:
- the user has a valid commercial entitlement
- the user can access the installer
- the user is eligible to activate a device

#### FLOW-008 — Failure Branches
Failure branches include:
- payment failure
- duplicate purchase confusion
- entitlement not recorded correctly
- installer access unavailable despite successful payment

#### FLOW-009 — User-Visible Feedback
The user must receive clear indication of:
- payment success or failure
- entitlement creation success or failure
- next step to obtain and install the app

#### FLOW-010 — Scope Boundary
This journey covers entitlement acquisition only.
It does not yet cover local runtime hydration or device activation completion.

---

### 5.4 Journey B — Installation and First Launch Runtime Hydration

#### FLOW-011 — Journey Name
**Installation and First Launch Runtime Hydration Journey**

#### FLOW-012 — Trigger
The journey begins when the user launches the installed app for the first time on a device that does not yet contain the required heavy runtime.

#### FLOW-013 — Preconditions
- the app shell is installed locally
- the user has a machine supported by MVP
- the heavy runtime is not yet installed or verified on that machine

#### FLOW-014 — Ordered Steps
1. The user launches the app.
2. The app shell performs startup checks.
3. The app detects that the runtime is absent or not yet hydrated.
4. The app enters a runtime hydration flow.
5. The app requests authorized runtime download access from the commercial backend.
6. The app downloads the runtime package.
7. The app verifies integrity of the downloaded package.
8. The app extracts and installs the runtime into the controlled local support location.
9. The app re-verifies runtime integrity after installation.
10. The app transitions the user toward activation or onboarding flow.

#### FLOW-015 — Success Outcome
This journey is successful when:
- the runtime exists locally
- runtime integrity passes
- the user can continue into activation and configuration flow

#### FLOW-016 — Failure Branches
Failure branches include:
- no network during hydration
- runtime download interrupted
- runtime integrity check failure
- extraction failure
- insufficient disk conditions
- version mismatch or corrupted package

#### FLOW-017 — User-Visible Feedback
The user must be able to see:
- that hydration is required
- that download/install is in progress
- whether the process failed
- whether retry or repair is available

#### FLOW-018 — Resume-on-Failure Behavior
If the download is interrupted, the app should attempt resumable recovery where supported rather than restarting blindly.

#### FLOW-019 — Scope Boundary
This journey does not imply that every future launch requires hydration.
It applies only when the required runtime is absent, corrupted, or flagged for repair.

---

### 5.5 Journey C — Device Activation

#### FLOW-020 — Journey Name
**Device Activation Journey**

#### FLOW-021 — Trigger
The journey begins when the user attempts to activate the app on a local machine using a valid entitlement.

#### FLOW-022 — Preconditions
- the app shell is installed
- runtime hydration is complete or not blocking activation
- the user has a valid purchased license or entitlement record
- the machine is not already operating as an activated device for that user unless renewal logic applies

#### FLOW-023 — Ordered Steps
1. The user enters the activation flow.
2. The app collects or derives the machine’s device identity material.
3. The app derives a device hash.
4. The app sends the activation request to the commercial backend.
5. The commercial backend validates:
   - the license state
   - activation slot availability
   - eligibility to issue a device-bound token
6. If approved, the backend issues a device-bound JWT.
7. The app validates and stores the JWT locally in secure storage.
8. The app marks the machine as activated for future offline license validation.

#### FLOW-024 — Success Outcome
The activation journey is successful when:
- the device is bound to the license
- the device-bound JWT is stored successfully
- the app recognizes the local machine as activated

#### FLOW-025 — Failure Branches
Failure branches include:
- invalid license
- expired entitlement
- device-limit exceeded
- device mismatch attempt
- network failure during activation
- token issuance failure
- local secure storage failure

#### FLOW-026 — User-Visible Feedback
The app must clearly indicate:
- whether activation succeeded
- whether activation failed due to entitlement or device limit
- whether retry is possible
- whether offline use is now available for allowed post-activation scenarios

#### FLOW-027 — Important Behavioral Rule
Activation is not identical to payment.
A paid entitlement without successful device activation is not sufficient for local simulation use.

---

### 5.6 Journey D — Provider Key Setup

#### FLOW-028 — Journey Name
**Provider Key Setup Journey**

#### FLOW-029 — Trigger
The journey begins when the user configures or updates the provider API key required for simulation use.

#### FLOW-030 — Preconditions
- the app is installed
- the user has access to the configuration flow
- the device is activated or activation is not the blocking factor for configuration visibility
- the user has a supported provider key

#### FLOW-031 — Ordered Steps
1. The user chooses a supported provider.
2. The user pastes or enters the provider API key.
3. The frontend sends the key to the local supervisory layer.
4. The local supervisory layer stores the key in OS-native secure storage.
5. The UI receives success or failure state.
6. The app updates configuration readiness state.

#### FLOW-032 — Success Outcome
The journey is successful when:
- the provider key is stored in native secure storage
- the app recognizes that the required provider configuration exists

#### FLOW-033 — Failure Branches
Failure branches include:
- unsupported provider
- secure storage write failure
- malformed input
- OS permission issues
- later validation failure during Preflight even though storage succeeded

#### FLOW-034 — User-Visible Feedback
The user must receive clear feedback on:
- whether key storage succeeded
- whether a provider is now configured
- whether further steps are required before a run can begin

#### FLOW-035 — Important Behavioral Rule
Key setup success means the key is stored successfully.
It does not yet guarantee provider reachability or model availability.
Those are validated during Preflight.

---

### 5.7 Journey E — Seed Document Intake

#### FLOW-036 — Journey Name
**Seed Document Intake Journey**

#### FLOW-037 — Trigger
The journey begins when the user selects or uploads a document to serve as the seed document for a run.

#### FLOW-038 — Preconditions
- the user is inside a run-configuration-capable area of the app
- the app has not yet started the run
- the user has an eligible document candidate

#### FLOW-039 — Ordered Steps
1. The user selects one file.
2. The app captures metadata such as name, type, and size.
3. The app associates that file with the pending run configuration.
4. The UI reflects that a seed document has been selected.
5. The document waits for validation during Preflight.

#### FLOW-040 — Success Outcome
The journey is successful when:
- the app has one selected seed document bound to the pending run configuration
- the document is ready for Preflight validation

#### FLOW-041 — Failure Branches
Failure branches include:
- no file selected
- unsupported file type caught early
- file too large caught early
- local file read failure

#### FLOW-042 — User-Visible Feedback
The user must be able to see:
- which file is selected
- whether the file appears eligible
- whether there is an immediate obvious issue before Preflight

#### FLOW-043 — Important Behavioral Rule
Selecting a seed document is not equivalent to approval.
The document becomes usable only if Preflight validates it.

---

### 5.8 Journey F — Run Configuration and Preflight

#### FLOW-044 — Journey Name
**Run Configuration and Preflight Journey**

#### FLOW-045 — Trigger
The journey begins when the user has entered run inputs and requests feasibility validation.

#### FLOW-046 — Preconditions
- device activation is valid
- provider selection exists
- provider key is stored
- one seed document is selected
- requested agent count is entered
- requested round count is entered
- max token budget is entered

#### FLOW-047 — Ordered Steps
1. The user submits the run for Preflight.
2. The app validates that required local inputs exist.
3. The backend begins Preflight.
4. The backend validates entitlement and local readiness.
5. The backend validates runtime integrity.
6. The backend validates seed document properties.
7. The backend extracts text from the document.
8. The backend performs language detection.
9. The backend rejects the document if it fails English policy.
10. The backend probes provider readiness and model availability.
11. The backend estimates token usage and duration.
12. The backend computes a feasible execution plan.
13. The backend returns one of:
   - approved
   - approved with downgrade
   - rejected with reason
14. The UI presents the outcome clearly to the user.

#### FLOW-048 — Success Outcome
The journey is successful when the user receives a deterministic Preflight result and can make an informed next-step decision.

#### FLOW-049 — Failure Branches
Failure branches include:
- invalid or expired local license state
- device mismatch
- runtime integrity failure
- unsupported file type
- file too large
- extraction failure
- non-English detection
- provider unreachable
- model unavailable
- infeasible budget or duration plan

#### FLOW-050 — User-Visible Feedback
The user must see:
- that Preflight is running
- whether the run was approved, downgraded, or rejected
- why a rejection occurred
- what changed if a downgrade occurred
- whether the run is safe to start within constraints

#### FLOW-051 — Hidden Behavior Prohibition
The system must not silently convert the user’s requested plan into a smaller effective plan without explicit user-visible disclosure.

---

### 5.9 Journey G — Start Simulation

#### FLOW-052 — Journey Name
**Start Simulation Journey**

#### FLOW-053 — Trigger
The journey begins when the user starts a run after receiving Preflight approval or approved-with-downgrade and accepting the effective plan.

#### FLOW-054 — Preconditions
- Preflight result exists for the pending run
- the result is either approved or approved-with-downgrade
- the user has accepted any required downgrade
- the runtime is healthy
- the local backend is ready or can be started
- no blocking session conflict exists

#### FLOW-055 — Ordered Steps
1. The user clicks start.
2. The app creates a run/session context.
3. The app allocates a unique workspace.
4. The local backend transitions into ingestion.
5. The seed document is ingested into the run context.
6. Runtime personas are generated.
7. The Moderator Agent is established.
8. The event stream begins.
9. The UI transitions from configuration state to live run state.

#### FLOW-056 — Success Outcome
The journey is successful when:
- the run has started
- the workspace exists
- the backend is emitting run events
- the user sees live progress

#### FLOW-057 — Failure Branches
Failure branches include:
- backend readiness failure
- workspace creation failure
- ingestion failure
- persona generation failure
- event stream initialization failure
- local token handoff or runtime setup failure

#### FLOW-058 — User-Visible Feedback
The user must see:
- the run has moved out of preparation state
- which phase is active
- that progress is live rather than frozen
- if the run cannot proceed, the reason why

---

### 5.10 Journey H — Active Simulation Execution

#### FLOW-059 — Journey Name
**Active Simulation Execution Journey**

#### FLOW-060 — Trigger
The journey begins after the run has entered active execution.

#### FLOW-061 — Preconditions
- the run exists
- ingestion completed sufficiently
- persona generation completed sufficiently
- the event stream is active
- the provider is currently reachable enough for execution

#### FLOW-062 — Ordered Steps
1. The Moderator determines the current round theme.
2. The Moderator selects a speaker set of 3 to 5 participant agents.
3. Selected agents generate outputs through provider inference.
4. Outputs are emitted as ordered run events.
5. The UI updates transcript and status views incrementally.
6. The system updates token counters and time counters.
7. The system updates memory and summary layers.
8. The next round begins if constraints still allow it.

#### FLOW-063 — Success Outcome
The journey is successful when each round:
- executes within the moderated model
- updates visible progress
- preserves ordered event continuity
- respects budget and duration constraints

#### FLOW-064 — Failure Branches
Failure branches include:
- transient provider failures
- rate limiting
- network interruption
- malformed response handling
- internal engine error
- hard terminal provider unavailability

#### FLOW-065 — User-Visible Feedback
The user must be able to see:
- current round progress
- speaker contributions as they occur
- budget consumption state
- interruption state if the provider or network is impaired
- whether the run is still active, paused, finalizing, canceled, or failed

#### FLOW-066 — Important Behavioral Rule
The user must not be forced to wait behind a static non-informative loading screen for the entire run.
Progress must be observable as the run unfolds.

---

### 5.11 Journey I — Budget-Capped Finalization

#### FLOW-067 — Journey Name
**Budget-Capped Finalization Journey**

#### FLOW-068 — Trigger
The journey begins when cumulative token usage reaches or exceeds the user’s max token budget.

#### FLOW-069 — Preconditions
- the run is active
- a valid max token budget exists
- the run has accumulated enough material to support controlled finalization

#### FLOW-070 — Ordered Steps
1. The system detects budget hit.
2. The system stops scheduling new normal rounds.
3. The system emits budget-related visibility to the UI.
4. The run transitions into finalization.
5. The Moderator synthesizes completed material only.
6. The report is assembled using actual completed work.
7. The run terminates in the budget-capped completion path if report generation succeeds.
8. Cleanup is attempted according to policy.

#### FLOW-071 — Success Outcome
This journey is successful when:
- the run does not continue spending beyond its allowed control logic
- the final report is still generated
- the final report honestly reflects partial completion
- the terminal state reflects budget-capped completion

#### FLOW-072 — Failure Branches
Failure branches include:
- finalization failure
- report generation failure
- cleanup failure after otherwise valid finalization

#### FLOW-073 — User-Visible Feedback
The user must see:
- that budget was hit
- that no new rounds will be started
- that the run is finalizing based on completed work
- whether the report succeeded
- whether cleanup succeeded

#### FLOW-074 — Honesty Rule
A budget-capped run must never be represented as if all originally planned rounds completed if they did not.

---

### 5.12 Journey J — Manual Cancellation

#### FLOW-075 — Journey Name
**Manual Cancellation Journey**

#### FLOW-076 — Trigger
The journey begins when the user explicitly requests cancellation of an active or in-progress run.

#### FLOW-077 — Preconditions
- a run exists
- the run is in a cancelable state
- the user has access to the cancel action

#### FLOW-078 — Ordered Steps
1. The user invokes cancel.
2. The app sends the cancel intent to the backend.
3. The backend halts future normal round scheduling.
4. The backend transitions toward controlled cancellation behavior.
5. The backend closes down active execution responsibilities.
6. The run enters a canceled terminal path.
7. Cleanup is attempted.

#### FLOW-079 — Success Outcome
The journey is successful when:
- the run stops in a controlled way
- further normal execution does not continue
- state is clear to the user
- cleanup is attempted

#### FLOW-080 — Failure Branches
Failure branches include:
- cancel request sent during invalid state
- backend unresponsive during cancellation
- process kill needed due to unclean shutdown
- cleanup failure

#### FLOW-081 — User-Visible Feedback
The user must see:
- that cancellation was requested
- whether the run is canceling or already canceled
- whether the app had to force termination
- whether cleanup succeeded

---

### 5.13 Journey K — Normal App Close During or After Run

#### FLOW-082 — Journey Name
**Normal App Close Journey**

#### FLOW-083 — Trigger
The journey begins when the user closes the app window or otherwise exits the app through a normal OS-supported close path.

#### FLOW-084 — Preconditions
- the app is open
- the app is responsive enough to intercept close behavior
- there may or may not be an active run

#### FLOW-085 — Ordered Steps
1. The user initiates app close.
2. The app intercepts the close request.
3. If no active run exists, the app closes normally.
4. If an active run exists, the app initiates shutdown behavior.
5. The backend is instructed to stop gracefully.
6. A bounded wait is allowed for graceful termination.
7. If graceful termination fails, the app force-terminates the engine process tree.
8. The app attempts workspace cleanup where applicable.
9. The app completes closure.

#### FLOW-086 — Success Outcome
The journey is successful when:
- the app does not leave normal active execution continuing in the background
- the run is terminated or concluded appropriately
- cleanup is attempted
- the app closes cleanly

#### FLOW-087 — Failure Branches
Failure branches include:
- backend does not respond to shutdown
- process kill required
- workspace lock prevents cleanup
- close sequence ends with cleanup failure

#### FLOW-088 — User-Visible Feedback
Where relevant, the user should see:
- that the app is shutting down
- that an active run is being closed
- whether cleanup could not fully complete

---

### 5.14 Journey L — Crash or Abnormal Termination and Next Launch

#### FLOW-089 — Journey Name
**Crash / Abnormal Termination and Next Launch Janitor Journey**

#### FLOW-090 — Trigger
The journey begins when the previous app lifecycle ended abnormally and the user later launches the app again.

#### FLOW-091 — Preconditions
- a prior abnormal termination occurred
- a prior workspace may have been left behind
- the new app launch is occurring on the same machine

#### FLOW-092 — Ordered Steps
1. The user launches the app after the earlier abnormal termination.
2. The app performs startup checks.
3. The startup janitor scans for abandoned workspaces.
4. The app verifies that any discovered workspace is not actively owned by a valid live process.
5. The app attempts to delete abandoned workspace artifacts.
6. The app records janitor outcome for local diagnostics.
7. The app continues into normal app readiness once janitor logic completes or times out safely.

#### FLOW-093 — Success Outcome
The journey is successful when:
- abandoned workspaces are identified correctly
- stale workspaces are removed where possible
- the app does not attempt session recovery
- the user may start a fresh run afterward

#### FLOW-094 — Failure Branches
Failure branches include:
- janitor cannot delete due to file locks
- janitor cannot confirm workspace ownership safely
- janitor finds corrupt residual data
- janitor cleanup partially fails

#### FLOW-095 — User-Visible Feedback
The app must not silently pretend that nothing happened if cleanup failure is material.
At minimum, the user should be informed if:
- residual abandoned workspace cleanup did not fully succeed
- manual retry or diagnostic review may be needed

#### FLOW-096 — Important Behavioral Rule
The system must not attempt to reconstruct and resume the interrupted run.
The correct behavior is purge-and-restart, not recovery.

---

### 5.15 Journey M — Report Export and Post-Run Viewing

#### FLOW-097 — Journey Name
**Report Export and Post-Run Viewing Journey**

#### FLOW-098 — Trigger
The journey begins when a run has produced a valid report and the user wants to save or view it.

#### FLOW-099 — Preconditions
- the run completed successfully or as CompletedBudgetCapped
- report generation succeeded
- the report file exists locally

#### FLOW-100 — Ordered Steps
1. The app makes the report available to the user.
2. The user invokes export/open/view behavior.
3. The app presents the PDF as a local artifact.
4. The user may retain the report outside the active run lifecycle.
5. The run workspace cleanup proceeds independently from the retained exported report location, if policy allows the report to persist separately.

#### FLOW-101 — Success Outcome
The journey is successful when:
- the user has access to the generated PDF
- the file is viewable after the run ends
- the report remains an honest reflection of actual run output

#### FLOW-102 — Failure Branches
Failure branches include:
- report render failure
- report file creation failure
- file write permission failure
- local file open failure despite successful generation

#### FLOW-103 — User-Visible Feedback
The user must see:
- whether the report is available
- where or how it can be accessed
- whether export/viewing failed
- whether the failure affected only viewing or the report generation itself

#### FLOW-104 — Offline Viewing Rule
Post-run local report viewing may remain available even when the app is not online, provided the file exists locally and no online inference is required.

---

### 5.16 Cross-Journey Rules

#### FLOW-105 — Readiness Gating Rule
The app must not let the user start a run unless all required prerequisites are satisfied and Preflight has returned an allowed result.

#### FLOW-106 — Controlled Failure Rule
Every major journey must fail in a controlled and user-visible way rather than through silent dead-end behavior.

#### FLOW-107 — Honest State Communication Rule
The UI must communicate actual run state honestly.
It must not say:
- completed when only partial finalization occurred
- cleaned when cleanup failed
- ready when required prerequisites are missing
- offline-capable for simulation when connectivity is required

#### FLOW-108 — No Hidden Continuation Rule
After cancellation, shutdown, or budget-triggered finalization, the system must not silently continue normal round scheduling in the background.

#### FLOW-109 — User Simplicity Rule
User journeys must remain operable by a non-expert user.
The product must not require the user to manage:
- local ports
- process trees
- vector databases
- event offsets
- runtime internals
- token accounting formulas

#### FLOW-110 — Fresh-Run Rule
Each new run must be treated as a fresh run with a new workspace rather than an implicit continuation of a prior run.

---

### 5.17 Section Boundary

#### FLOW-111 — Boundary Rule
This section defines:
- major user journeys
- journey triggers
- preconditions
- ordered steps
- success outcomes
- failure branches
- user-visible feedback requirements

This section does not yet define in full:
- exact state transition tables
- exact API payloads
- exact event schema fields
- exact UI component definitions
- exact acceptance tests

Those belong in later sections.

---

### 5.18 Final Journey Summary

#### FLOW-112 — Journey Summary Rule
At the behavioral-narrative level, the user experience of the MVP is defined as:

1. acquire entitlement  
2. install app  
3. hydrate runtime if needed  
4. activate device  
5. store provider key  
6. select one seed document  
7. configure run inputs  
8. run Preflight  
9. accept approval or downgrade outcome  
10. start simulation  
11. observe real-time progress  
12. complete naturally, cancel, or finalize due to budget/time constraints  
13. export and view report  
14. close app with cleanup attempted  
15. after abnormal termination, purge stale work on next launch instead of resuming it

#### FLOW-113 — Consistency Rule
All later sections, including behavior, state, architecture, interfaces, and acceptance criteria, must remain consistent with the journey summary above.

---

## 6. Product Behavior Specification

### 6.1 Section Purpose
This section defines the product’s operational behavior in deterministic terms.

This section exists to answer, in product-law form:
- what a run is
- when a run may begin
- what must happen before a run begins
- how a run behaves while active
- how personas are created
- how the Moderator governs debate
- how rounds are structured
- what counts as success, partial success, failure, and cancellation

This section is the primary behavioral authority for the product and is subordinate only to:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology

If any later implementation artifact, plan, prompt, or code behavior conflicts with this section, this section wins unless Document A is versioned forward through the Change Log.

---

### 6.2 Behavioral Authority Rules

#### BEHAV-001 — Product Law Rule
This section defines required product behavior, not optional design preferences.

#### BEHAV-002 — User-Visible Truth Rule
All user-facing product behavior must align with the rules in this section, even if lower-level implementation details differ internally.

#### BEHAV-003 — No Hidden Behavior Rule
The system must not perform materially significant behavior that affects run meaning, cost, completion status, or safety without either:
- defining that behavior here
- or defining it later in a non-conflicting derived section

#### BEHAV-004 — Behavior Before Implementation Rule
Implementation convenience must not override defined product behavior.
If implementation is difficult, the implementation must adapt to the product behavior or the document must be explicitly revised.

---

### 6.3 Run Definition and Run Identity

#### BEHAV-005 — Run Identity Rule
A run is the single authoritative execution unit of the product.

#### BEHAV-006 — Run Start Rule
A run begins only after:
- prerequisites are satisfied
- Preflight has returned an allowed result
- the user has accepted any required downgrade
- the app has initiated active execution

#### BEHAV-007 — Run End Rule
A run ends when it reaches one of its allowed terminal outcome paths, whether successful, partially successful, canceled, or failed.

#### BEHAV-008 — One Document Per Run Rule
A run must be scoped to one seed document only.

#### BEHAV-009 — One Workspace Per Run Rule
A run must receive exactly one fresh workspace that is not reused from a prior run.

#### BEHAV-010 — Run Freshness Rule
A new run must always be treated as a fresh execution context rather than a continuation of a prior one.

#### BEHAV-011 — Run Non-Recovery Rule
A run that is interrupted by abnormal termination must not be resumed in MVP.

---

### 6.4 Readiness and Start Preconditions

#### BEHAV-012 — Mandatory Readiness Rule
The system must not allow active run start unless all mandatory prerequisites are satisfied.

#### BEHAV-013 — Mandatory Prerequisites
The minimum prerequisites for run eligibility are:
- valid local license state
- valid device activation state
- supported provider selection
- stored provider API key for that provider
- runtime integrity pass
- one selected seed document
- requested agent count
- requested round count
- max token budget
- successful Preflight result

#### BEHAV-014 — No Implicit Defaults Rule
The system must not silently invent a missing critical input required for run execution.
This applies in particular to:
- provider selection
- token budget
- seed document
- requested round count
- requested agent count

#### BEHAV-015 — Preflight Gate Rule
A run must never move into active execution without a Preflight decision.

#### BEHAV-016 — Approval Gate Rule
Only Preflight outcomes of:
- approved
- approved-with-downgrade
may permit progression toward run start.

#### BEHAV-017 — Downgrade Acceptance Rule
If Preflight returns approved-with-downgrade, the user must be informed of the effective plan before active run start.

#### BEHAV-018 — Invalid Start Prohibition
The system must reject run start if:
- Preflight was rejected
- Preflight was not run
- required local readiness is missing
- the selected provider key is absent
- the seed document is missing
- the runtime is not healthy

---

### 6.5 Preflight Behavioral Contract

#### BEHAV-019 — Preflight Mandate
Preflight is mandatory before every run.

#### BEHAV-020 — Preflight Purpose
Preflight exists to determine whether a requested run is:
- valid
- feasible
- policy-compliant
- safe to start within the product’s bounded model

#### BEHAV-021 — Preflight Must Validate
Preflight must validate at minimum:
- local entitlement readiness
- device activation consistency
- provider configuration presence
- runtime integrity
- file type support
- file size constraints
- extractable text presence
- English-language compliance
- provider reachability
- model availability
- estimated cost and duration feasibility
- downgrade necessity

#### BEHAV-022 — English Enforcement Rule
Preflight must reject the run if the seed document fails English-only policy.

#### BEHAV-023 — Feasibility Planning Rule
Preflight must convert user-requested scale into either:
- an executable effective plan
- or a deterministic rejection

#### BEHAV-024 — Silent Feasibility Guessing Prohibition
Preflight must not approve a run purely because the user requested it.
Approval requires explicit feasibility determination under the bounded moderated model.

#### BEHAV-025 — Preflight Output Rule
Preflight must always end in exactly one of these outcomes:
- approved
- approved-with-downgrade
- rejected

#### BEHAV-026 — Preflight Rejection Honesty Rule
If Preflight rejects a run, the rejection must include a reason the user can understand.
The system must not fail with an opaque generic error where a deterministic reason is known.

#### BEHAV-027 — Downgrade Transparency Rule
If Preflight downgrades a requested plan, the system must explicitly show:
- what was requested
- what was approved for execution
- that the change occurred before the run begins

#### BEHAV-028 — No Start-Then-Reject Rule
The system must not begin active execution and only later reveal that the run never should have passed Preflight.

---

### 6.6 Effective Plan Behavior

#### BEHAV-029 — Requested vs Effective Plan Rule
The user provides a requested plan.
The system executes an effective plan.

#### BEHAV-030 — Effective Plan Components
The effective plan must include at minimum:
- effective agent count
- effective round count
- provider
- model
- max token budget
- bounded speaker model
- bounded concurrency policy

#### BEHAV-031 — Effective Plan Upper-Bound Rule
The effective plan must remain bounded by the product’s operating constraints and must never become an unbounded open-ended swarm execution.

#### BEHAV-032 — Effective Plan Integrity Rule
Once the run starts, the run must execute according to the approved effective plan unless a later defined control event changes normal progression, such as:
- budget hit
- duration cap
- controlled cancellation
- terminal failure
- controlled finalization

#### BEHAV-033 — No User-Deceptive Plan Rule
The system must not show one effective plan to the user and secretly execute a materially different one.

---

### 6.7 Persona Generation Behavior

#### BEHAV-034 — Runtime Persona Generation Rule
The system must generate the persona set at runtime from the seed document context.

#### BEHAV-035 — No Static Roster Requirement
The system must not require a pre-authored user-defined persona roster in MVP.

#### BEHAV-036 — Persona Relevance Rule
Generated personas must be relevant to the domain and content of the seed document.

#### BEHAV-037 — Persona Diversity Rule
The persona set must include meaningful perspective diversity.
At minimum, the generated set must include:
- favorable or opportunity-seeking perspectives
- skeptical or risk-seeking perspectives
- neutral or analytical perspectives
- one Moderator Agent

#### BEHAV-038 — Moderator Existence Rule
Every run must include exactly one Moderator Agent.

#### BEHAV-039 — Persona Set Sufficiency Rule
The persona set must be large enough to support repeated round selection without degenerating into a single repeated perspective pattern.

#### BEHAV-040 — Persona Identity Rule
Each persona must have a stable identity within the run.

#### BEHAV-041 — Persona Traceability Rule
A persona that appears in the live feed or final report must remain identifiable by stable label within that run.

#### BEHAV-042 — Persona Persistence Boundary
Personas are run-local artifacts.
They are not required to persist as reusable long-term profiles across runs in MVP.

#### BEHAV-043 — Persona Derivation Honesty Rule
The system must not claim that personas are real users, real market respondents, or verified human research subjects.
They are simulation constructs derived from model-driven interpretation of the seed document.

---

### 6.8 Moderator Agent Behavior

#### BEHAV-044 — Moderator Centrality Rule
The Moderator Agent is the central orchestration actor of the simulation.

#### BEHAV-045 — Moderator Responsibilities
The Moderator Agent must be responsible for:
- selecting round themes
- choosing round speakers
- maintaining debate flow
- incorporating prior summaries
- steering the simulation toward structured output
- producing final synthesis inputs for the report
- generating or governing the Market Viability Score computation flow

#### BEHAV-046 — Moderator Not Equal to Participant Rule
The Moderator Agent is not a normal participant agent.
Its role is orchestration, synthesis, and bounded control.

#### BEHAV-047 — Moderator Selection Rule
For each round, the Moderator must select a speaker set of 3 to 5 participant agents.

#### BEHAV-048 — Speaker Bound Rule
The system must never run a normal round with fewer than 3 or more than 5 selected speakers under the MVP moderated model.

#### BEHAV-049 — Contextual Selection Rule
The Moderator must choose speakers based on run context, including:
- seed document content
- current debate theme
- prior round summaries
- surfaced risks and arguments
- need for perspective diversity
- budget and run progression constraints

#### BEHAV-050 — No Random Speaker Fanout Rule
The Moderator must not behave as a purely random scheduler.
Selection must be contextually grounded.

#### BEHAV-051 — No Full-Fanout Rule
The Moderator must not schedule all personas to speak in every round.

#### BEHAV-052 — Debate Control Rule
The Moderator must keep the simulation bounded and structured rather than allowing uncontrolled transcript growth or unbounded turn participation.

#### BEHAV-053 — Finalization Responsibility Rule
When a run enters finalization, the Moderator must shift from round orchestration to synthesis and report-output support.

---

### 6.9 Round Behavior

#### BEHAV-054 — Round Structure Rule
Each round must be treated as one discrete cycle of moderated debate.

#### BEHAV-055 — Round Minimum Elements
A valid round must include:
- a current round identity
- a Moderator-defined theme or focus
- a speaker set of 3 to 5 participant agents
- generated participant outputs
- a post-round summary update
- budget/time counter updates

#### BEHAV-056 — Round Theme Rule
Every round must have a thematic focus, even if that focus is implicit from the debate state.

#### BEHAV-057 — Speaker Selection Before Response Rule
Participant speaking rights in a round must be determined before normal participant outputs are generated for that round.

#### BEHAV-058 — Unselected Silence Rule
Participant agents not selected for the round must not emit normal round dialogue for that round.

#### BEHAV-059 — Ordered Round Execution Rule
The system must preserve internal ordering coherence within each round so that:
- selected speakers are known
- their outputs are attributable
- round completion can be summarized honestly

#### BEHAV-060 — Round Summary Rule
At the end of each round, the system must update a summary of what that round contributed.

#### BEHAV-061 — Round Contribution Honesty Rule
A round summary must reflect actual generated content from that round and must not invent contributions that did not occur.

#### BEHAV-062 — Round Continuation Rule
A new round may begin only if all of the following remain true:
- the run is not canceled
- the run is not in terminal failure
- budget control has not forced finalization
- duration control has not forced finalization
- provider/network conditions have not forced a different terminal or paused path
- the effective round ceiling has not already been reached

#### BEHAV-063 — Round Completion Bound Rule
The run must not exceed the effective round count under normal execution.

---

### 6.10 User-Visible Live Behavior

#### BEHAV-064 — Live Progress Visibility Rule
During active execution, the system must provide user-visible progress rather than opaque background silence.

#### BEHAV-065 — Transcript Visibility Rule
Participant outputs selected for live presentation must appear in a way that lets the user follow debate progression chronologically.

#### BEHAV-066 — Attribution Rule
Visible participant contributions must remain attributable to the persona label that produced them.

#### BEHAV-067 — Phase Visibility Rule
The user must be able to distinguish major phases such as:
- Preflight
- ingestion
- persona generation
- active rounds
- finalization
- cancellation
- failure
- cleanup attempt

#### BEHAV-068 — Real-Time Incrementality Rule
The UI must receive and present run progress incrementally during execution rather than only after full run completion.

#### BEHAV-069 — No Fake Liveness Rule
The system must not simulate false activity if no real progress is occurring.
If the run is paused, throttled, degraded, or blocked, the UI must reflect that honestly.

---

### 6.11 Success, Partial Success, Failure, and Cancellation Definitions

#### BEHAV-070 — Successful Completion Rule
A run is a successful completion only when:
- active execution reached a valid completion path
- finalization succeeded
- the report was generated successfully

#### BEHAV-071 — Normal Completion Rule
Normal completion occurs when the run completes its effective round plan and then finalizes successfully.

#### BEHAV-072 — Partial Success Rule
A run qualifies as partial success when:
- it does not complete its full originally requested or effective plan
- but it enters controlled finalization
- and still generates a valid report from completed work

#### BEHAV-073 — Budget-Capped Completion Rule
Budget-capped completion is a valid partial success path if:
- budget is hit
- no new normal rounds are scheduled
- finalization succeeds
- the report is generated successfully

#### BEHAV-074 — Failure Rule
A run is a failure if it cannot produce a valid report-producing completion path and instead terminates without usable final output.

#### BEHAV-075 — Controlled Cancellation Rule
Cancellation is not the same as crash failure.
Cancellation is a deliberate or controlled stop path that halts forward execution.

#### BEHAV-076 — Abnormal Termination Distinction
Abnormal termination is not a valid run outcome category the user should rely on.
It is an infrastructure interruption condition handled by cleanup and next-launch janitor policy.

#### BEHAV-077 — Report Truthfulness Rule
A successful or partially successful run must generate a report that corresponds only to actual completed work.

#### BEHAV-078 — False Success Prohibition
The system must not classify a run as completed if:
- report generation failed
- finalization failed
- the run died before valid output could be assembled

---

### 6.12 Controlled Cancellation Behavior

#### BEHAV-079 — Cancelability Rule
A run may be canceled only when it is in a cancelable non-terminal state.

#### BEHAV-080 — Cancellation Effect Rule
Once cancellation is accepted:
- no new normal rounds may be scheduled
- the system must move toward controlled termination
- the user must be informed that the run is no longer progressing normally

#### BEHAV-081 — Cancellation Honesty Rule
The system must not label a canceled run as successful completion unless a later explicitly allowed controlled-output path says otherwise.

#### BEHAV-082 — No Hidden Post-Cancel Activity Rule
After cancellation is initiated, the system must not continue normal debate activity in the background.

---

### 6.13 Behavioral Boundaries of This Part

#### BEHAV-083 — Part Boundary Rule
This part of Section 6 defines:
- run identity
- readiness conditions
- Preflight behavior
- effective-plan behavior
- persona generation rules
- Moderator rules
- round structure rules
- live visibility rules
- success/failure/cancellation definitions

---

### 6.14 Memory and Context Management Behavior

#### BEHAV-084 — Bounded Memory Rule
The system must use bounded memory behavior.
It must not rely on naively appending the full historical transcript into every model call throughout the run.

#### BEHAV-085 — Memory Layering Rule
The run must manage context through multiple layers rather than one unbounded transcript stream.

At minimum, those layers must include:
- seed-document retrieval context
- cumulative round-summary context
- rolling argument/risk memory
- transcript archive for visibility and report generation

#### BEHAV-086 — Seed Context Rule
The seed document must remain the foundational source context for the run.
However, the entire seed document must not be blindly inserted into every model call if a more bounded retrieval strategy is available.

#### BEHAV-087 — Retrieval Grounding Rule
When the system grounds debate or synthesis in source material, it should prefer relevant extracted seed content over unrelated speculative expansion.

#### BEHAV-088 — Round Summary Memory Rule
After each completed round, the system must update a bounded summary representation of what the round contributed.

#### BEHAV-089 — Argument Memory Rule
The system must maintain a rolling record of surfaced claims, risks, objections, and supportive arguments sufficient to preserve continuity across rounds.

#### BEHAV-090 — Transcript Archive Rule
The full transcript archive exists primarily for:
- live user visibility
- report appendix generation
- traceability of what occurred

The transcript archive must not be treated as an unlimited prompt context source.

#### BEHAV-091 — Context Collapse Prevention Rule
The system must actively prevent context-window collapse by using bounded memory strategies instead of unbounded replay.

#### BEHAV-092 — Memory Honesty Rule
The system must not claim to remember full historical detail if it is actually using summarized or reduced memory forms.

#### BEHAV-093 — Per-Call Context Discipline Rule
Each inference call should receive only the context necessary for its function, not all historically available context by default.

#### BEHAV-094 — Round Continuity Rule
Despite bounded memory behavior, later rounds must still preserve enough continuity that the debate appears coherent rather than reset each round.

#### BEHAV-095 — No Cross-Run Memory Rule
Memory artifacts from one run must not be reused as semantic input for a later run.

#### BEHAV-096 — Fresh Slate Rule
Each run must begin with a blank run-memory state except for:
- the current seed document
- the effective plan
- the runtime-generated persona set
- the current run-local counters and scaffolding

#### BEHAV-097 — No Phantom Memory Rule
The system must not allow agents to reference prior-run content, prior-run debates, or prior-run documents as if they belong to the current run.

#### BEHAV-098 — Memory Reduction Acceptability Rule
It is acceptable for older debate content to be compressed into summaries or structured memory abstractions so long as this preserves meaningful continuity and bounded cost.

---

### 6.15 Budget Behavior

#### BEHAV-099 — Mandatory Budget Rule
Every run must have a user-defined max token budget before it can begin.

#### BEHAV-100 — No Unlimited Spend Rule
The system must not permit an active run with no declared budget ceiling.

#### BEHAV-101 — Budget Counter Rule
During an active run, the system must track cumulative token usage.

#### BEHAV-102 — Budget Visibility Rule
The user must be able to see budget-related execution state during the run at a meaningful level, including at minimum:
- maximum token budget
- consumed budget progress
- whether a warning or cap condition has been reached

#### BEHAV-103 — Budget Warning Rule
The system should surface budget warnings before hard budget exhaustion when meaningful thresholds are crossed.

#### BEHAV-104 — Budget Exhaustion Detection Rule
The system must detect when cumulative token use reaches or exceeds the max token budget.

#### BEHAV-105 — Budget Hit Control Rule
When budget hit occurs, the system must:
- stop scheduling new normal rounds
- preserve already completed work
- transition into controlled finalization behavior if feasible

#### BEHAV-106 — No Budget Overrun Drift Rule
The system must not keep scheduling normal debate activity as if no cap exists after budget hit is detected.

#### BEHAV-107 — Finalization-on-Budget Rule
Budget exhaustion is not, by itself, an automatic failure.
It is a trigger for controlled finalization if sufficient material exists.

#### BEHAV-108 — Budget-Capped Report Rule
If report generation succeeds after budget-triggered finalization, the run must be classified under the budget-capped completion path rather than as a normal full completion.

#### BEHAV-109 — Budget Honesty Rule
The system must not hide from the user that the final output resulted from a budget-constrained run.

#### BEHAV-110 — Budget Feasibility Discipline Rule
Budget behavior during the run must remain consistent with the effective plan returned by Preflight, except where real usage causes earlier-than-estimated budget exhaustion.

#### BEHAV-111 — No Spend Fabrication Rule
Budget and token reporting must reflect the system’s actual tracked execution behavior and must not invent synthetic token totals for unfinished work.

#### BEHAV-112 — Controlled Truncation Rule
If budget is exhausted mid-run, the system may truncate the originally planned debate path, but only through an honest finalization path rather than abrupt silent termination.

#### BEHAV-113 — Budget Failure Rule
If budget is exhausted before the system can produce enough material for a meaningful finalization path, the run may enter terminal failure rather than pretending success.

---

### 6.16 Duration-Cap Behavior

#### BEHAV-114 — Mandatory Duration Cap Rule
Every run must be governed by a hard duration cap of two hours.

#### BEHAV-115 — Duration Counter Rule
The system must track elapsed run time throughout active execution.

#### BEHAV-116 — Duration Visibility Rule
The user should be able to see that the run is time-bounded and, where practical, how much time has elapsed.

#### BEHAV-117 — No Unbounded Runtime Rule
The system must not allow normal round execution to continue indefinitely.

#### BEHAV-118 — Duration-Cap Trigger Rule
If the hard duration cap is reached, the system must:
- stop scheduling new normal rounds
- enter controlled finalization if feasible
- avoid pretending that further normal execution is still allowed

#### BEHAV-119 — Duration-Capped Finalization Rule
Duration-cap-triggered finalization is behaviorally analogous to budget-triggered finalization, except the limiting constraint is time rather than token budget.

#### BEHAV-120 — Duration Honesty Rule
The system must not classify a duration-capped run as if it completed all originally planned rounds unless it actually did.

#### BEHAV-121 — Duration Failure Rule
If the duration cap is reached and meaningful finalization cannot be completed, the run may end in terminal failure.

#### BEHAV-122 — Pre-Cap Discipline Rule
The system should avoid wasting available run time on unnecessary uncontrolled expansion or prompt inflation that undermines the bounded-execution model.

---

### 6.17 Network and Provider Interruption Behavior

#### BEHAV-123 — Connectivity Dependency Rule
Normal simulation execution depends on active connectivity to the selected third-party provider.

#### BEHAV-124 — No Offline Simulation Fallback Rule
The system must not pretend to continue true inference execution offline when provider connectivity is unavailable.

#### BEHAV-125 — Interruption Classification Rule
If inference problems arise during a run, the system should distinguish between at minimum:
- rate-limit degradation
- transient provider-side failure
- local network interruption
- terminal provider unavailability

#### BEHAV-126 — Controlled Interruption Rule
Transient interruption must be handled through controlled degraded behavior rather than shell-level crash behavior where possible.

#### BEHAV-127 — Retry-Boundedness Rule
Retry behavior for retryable failures must be bounded.
The system must not enter infinite retry loops.

#### BEHAV-128 — Rate-Limit Pause Rule
If the provider rate-limits the run, the system may temporarily pause forward progress while honoring backoff logic and preserving run coherence.

#### BEHAV-129 — Network Pause Rule
If the network is temporarily unavailable, the system may pause or degrade the run while determining whether finalization or failure is more appropriate.

#### BEHAV-130 — User Visibility During Interruption Rule
When interruption affects run progression, the user must be informed that the run is:
- delayed
- paused
- degraded
- finalizing
- or failed

The system must not show misleading “healthy progress” when the run is blocked.

#### BEHAV-131 — Partial Material Preservation Rule
Already completed round content must remain preserved for possible finalization even if later provider/network interruptions occur.

#### BEHAV-132 — Finalize-vs-Fail Decision Rule
When interruption occurs late enough in the run, the system may choose controlled finalization instead of terminal failure if enough completed material exists to support a meaningful report.

#### BEHAV-133 — Terminal Interruption Rule
If interruption prevents meaningful continuation and meaningful finalization is not possible, the run must enter terminal failure rather than fabricating completion.

#### BEHAV-134 — Interruption Honesty Rule
The final run classification must reflect what actually happened.
A network- or provider-disrupted run must not be mislabeled as a full normal completion.

#### BEHAV-135 — Shell Stability Rule
Transient provider/network failures must not, by themselves, be allowed to crash the app shell or corrupt the run workspace.

---

### 6.18 Finalization Behavior

#### BEHAV-136 — Finalization Purpose Rule
Finalization is the controlled closing phase in which the system transforms completed run material into structured output.

#### BEHAV-137 — Finalization Entry Rule
A run may enter finalization for at least the following reasons:
- normal completion of effective rounds
- budget-cap trigger
- duration-cap trigger
- controlled degradation where enough material exists
- explicitly authorized controlled end path

#### BEHAV-138 — Finalization Stop Rule
Once a run enters finalization, it must stop normal round scheduling.

#### BEHAV-139 — Finalization Input Rule
Finalization must operate only on material actually produced during the run, including:
- seed-derived context
- completed-round summaries
- surfaced risks and arguments
- transcript material
- tracked budget/time state
- persona outputs that actually occurred

#### BEHAV-140 — No Fabricated Debate Rule
Finalization must not invent nonexistent rounds, nonexistent speakers, or nonexistent debate turns to make the report appear more complete.

#### BEHAV-141 — Finalization Synthesis Rule
During finalization, the system must synthesize:
- the overall debate trajectory
- strongest pro arguments
- strongest anti arguments
- top risks
- a Moderator-governed viability assessment
- cost and token summary inputs
- report-assembly content

#### BEHAV-142 — Market Score Rule
The Market Viability Score must be produced only during or under the authority of finalization behavior and must reflect actual run content.

#### BEHAV-143 — Finalization Honesty Rule
The finalization output must be truthful about whether the run:
- completed fully
- completed under budget constraint
- completed under duration constraint
- completed after degraded but controlled execution
- failed to produce a valid report

#### BEHAV-144 — Finalization Failure Rule
If finalization cannot successfully assemble the required output, the run must not be labeled completed.

#### BEHAV-145 — Finalization Atomicity Preference Rule
The system should treat finalization as a cohesive controlled phase rather than as a scattered set of unrelated hidden tasks.

---

### 6.19 Report Generation Behavior

#### BEHAV-146 — Report Requirement Rule
A successful completion path or valid partial-success completion path must generate a structured PDF report.

#### BEHAV-147 — Report Mandatory Sections Rule
The report must contain all required sections defined by the project, including:
- Executive Summary
- Market Viability Score
- Ranked Risks
- Strongest Pro Arguments
- Strongest Anti Arguments
- Cost and Token Summary
- Transcript Appendix

#### BEHAV-148 — Report Integrity Rule
The report must be generated from actual run artifacts and must remain consistent with:
- actual completed debate content
- actual tracked token/cost data
- actual final run classification

#### BEHAV-149 — Report Scope Truthfulness Rule
If the run completed under constrained conditions, the report must reflect that the output is based on completed work only.

#### BEHAV-150 — Report Appendix Truth Rule
The Transcript Appendix must reflect actual ordered dialogue or event-derived content from the run and must not contain invented dialogue.

#### BEHAV-151 — Report Availability Rule
Once report generation succeeds, the report must be made available to the user as a local artifact.

#### BEHAV-152 — Report Failure Classification Rule
If report generation fails, the run must not be labeled as successful completion even if debate execution had previously progressed well.

#### BEHAV-153 — No Placeholder Report Rule
The system must not emit an empty, placeholder, or materially incomplete PDF and present it as the intended final report.

#### BEHAV-154 — Cost Summary Honesty Rule
The report’s cost and token summary must be based on tracked usage and must not imply cost certainty beyond what the product actually measures.

#### BEHAV-155 — Report Persistence Boundary Rule
The final exported report may persist locally beyond the cleanup of the transient run workspace if the product’s export path stores it outside the ephemeral workspace boundary.

---

### 6.20 Cleanup Behavior

#### BEHAV-156 — Cleanup Obligation Rule
After any run terminal path, the system must attempt cleanup according to the best-effort ephemeral storage policy.

#### BEHAV-157 — Cleanup Trigger Rule
Cleanup must be attempted after:
- successful completion
- budget-capped completion
- controlled cancellation
- terminal failure
- normal app close affecting an active run
- next-launch janitor detection of abandoned run artifacts

#### BEHAV-158 — Workspace Deletion Rule
Cleanup must target the run workspace and associated transient run-local artifacts governed by the ephemeral storage policy.

#### BEHAV-159 — Cleanup Ordering Rule
The system must not attempt destructive workspace deletion until active process and file-lock conditions make cleanup appropriately possible or until forced termination policy has been applied where authorized.

#### BEHAV-160 — Cleanup Best-Effort Rule
Cleanup is mandatory as an attempt, not as an absolute guarantee of perfect forensic erasure.

#### BEHAV-161 — Cleanup Outcome Visibility Rule
Where cleanup materially fails, the system must not silently pretend cleanup succeeded.

#### BEHAV-162 — Cleanup State Honesty Rule
A run or app lifecycle may distinguish between:
- run completion state
- cleanup success state

The system must not merge those dishonestly.

#### BEHAV-163 — Next-Launch Janitor Rule
If abnormal termination prevented normal cleanup, the next app launch must attempt janitor cleanup of abandoned workspaces.

#### BEHAV-164 — No Recovery-Through-Cleanup Rule
Cleanup behavior must not be used as a hidden recovery mechanism that revives interrupted runs.
Its role is purge, not resume.

#### BEHAV-165 — Report Preservation Rule
Cleanup must not delete the final exported report if that report has been intentionally written outside the transient workspace boundary.

#### BEHAV-166 — Partial Cleanup Failure Rule
If cleanup only partially succeeds, the system must classify that honestly rather than reporting full cleanup success.

---

### 6.21 Behavioral Prohibitions

#### BEHAV-167 — No Unauthorized Feature Behavior Rule
The system must not behave as if it supports capabilities that are constitutionally out of scope, including:
- multilingual seed support
- crash resume
- unlimited device use
- hosted simulation mode
- fully offline inference
- enterprise fleet administration

#### BEHAV-168 — No Hidden Scope Expansion Rule
The system must not silently introduce adjacent behavior that materially changes the product category or operating model without explicit spec revision.

#### BEHAV-169 — No False Security Rule
The system must not present itself as providing stronger local privacy, deletion, or anti-forensic protection than the project constitution allows.

#### BEHAV-170 — No False Determinism Rule
The system must not represent the Market Viability Score or debate output as guaranteed objective truth.

#### BEHAV-171 — No Silent Downgrade Drift Rule
The system must not alter the effective plan during execution in a way that materially changes user expectations without surfacing the reason or resulting state honestly.

#### BEHAV-172 — No Transcript Fabrication Rule
The system must not fabricate agent turns, round summaries, or report content to conceal failures, interruptions, or truncated execution.

#### BEHAV-173 — No State Mislabeling Rule
The system must not label:
- failed as completed
- canceled as completed
- cleanup failed as cleaned
- budget-capped as fully completed
- duration-capped as fully completed
unless those labels are behaviorally justified by the spec

#### BEHAV-174 — No Cross-Run Leakage Rule
The system must not knowingly let one run’s content influence another run as active memory.

#### BEHAV-175 — No Invisible Budget Behavior Rule
The system must not conceal budget-driven truncation from the user.

#### BEHAV-176 — No Opaque Failure Rule
Where the system knows why a run cannot proceed or continue, it should not replace that reason with a meaningless generic failure label.

---

### 6.22 Behavioral Integrity Rules

#### BEHAV-177 — End-to-End Truthfulness Rule
From run start through report export and cleanup, the product must behave in a way that preserves truthful mapping between:
- what was requested
- what was approved
- what actually happened
- what was reported to the user
- what was written into the final output

#### BEHAV-178 — Output Integrity Rule
No final output may claim more execution than actually occurred.

#### BEHAV-179 — State Integrity Rule
Lifecycle states must correspond to real execution conditions rather than decorative UI language.

#### BEHAV-180 — Artifact Integrity Rule
Artifacts produced by the run must remain semantically consistent with the run’s actual history and terminal path.

#### BEHAV-181 — Visibility Integrity Rule
User-visible progress indicators must correspond to real backend progression or real degradation states.

#### BEHAV-182 — Boundedness Integrity Rule
All run behavior must remain compatible with the product’s bounded, moderated, cost-controlled operating model.

#### BEHAV-183 — Constitution Consistency Rule
Nothing in behavioral implementation may contradict the constitutional rules of Document A.

---

### 6.23 Behavioral Summary of Section 6

#### BEHAV-184 — Section Summary Rule
At the behavioral level, the product is defined as follows:

A run begins only after mandatory prerequisites and successful Preflight. The system generates a run-local persona set from one English seed document, establishes one Moderator Agent, and executes a bounded multi-round debate in which only 3 to 5 selected participant agents speak per round. The system preserves continuity through bounded memory layers rather than unbounded transcript replay. The run remains governed by a mandatory max token budget and a hard two-hour duration cap. If budget or duration constraints are hit, or if degraded conditions make further execution impractical, the system must stop normal round scheduling and enter controlled finalization when meaningful output remains possible. A valid completion path must generate a structured PDF report based only on actual completed work. After the run terminates, the system must attempt best-effort cleanup of transient workspace artifacts and use next-launch janitor cleanup after abnormal termination rather than session recovery.

#### BEHAV-185 — Section 6 Completion Rule
Section 6 is complete only when all later sections remain consistent with the behavioral law established here.

#### BEHAV-186 — Section Boundary Rule
Section 6 defines product behavior only.
It does not yet define:
- full formal state transition tables
- architecture ownership boundaries
- low-level API payloads
- database DDL
- UI component hierarchy
- detailed acceptance-test wording

Those are governed in later sections.

---

## 7. State Machines

### 7.1 Section Purpose
This section defines the formal lifecycle states that govern the product.

This section exists to ensure that:
- the product does not behave as an ambiguous collection of ad hoc flows
- AI coding agents implement lifecycle logic deterministically
- invalid transitions are explicitly prohibited
- user-visible status is tied to real execution conditions
- later API and acceptance criteria can map back to stable state logic

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- Section 6 — Product Behavior Specification

If any later implementation artifact or planning document conflicts with the state logic defined here, this section wins unless Document A is explicitly versioned forward.

---

### 7.2 Global State Machine Rules

#### STATE-001 — Formal State Rule
A state machine defined in this section is authoritative.
Its states, transition triggers, invalid transitions, and terminal outcomes must be treated as binding implementation logic.

#### STATE-002 — Real State Rule
A state name must correspond to a real underlying system condition.
A state must not exist only as decorative UI wording.

#### STATE-003 — Exact State Name Rule
State names must be used exactly as defined.
Equivalent paraphrases may be displayed to the user for readability, but the canonical internal state name must remain stable.

#### STATE-004 — Transition Rule
A transition may occur only when:
- its preconditions are satisfied
- its trigger is valid
- no higher-authority rule prohibits it

#### STATE-005 — Invalid Transition Rule
A transition not explicitly allowed by this section or a later non-conflicting state subsection is invalid.

#### STATE-006 — Truthful UI Rule
User-visible state labels must remain truthfully mapped to the underlying canonical state.

#### STATE-007 — Side Effect Rule
A transition may carry required side effects, but side effects must not silently alter the meaning of the state itself.

#### STATE-008 — State Machine Separation Rule
Different lifecycle domains must remain semantically distinct.
At minimum, the following state machines are separate:
- app lifecycle
- activation lifecycle
- Preflight lifecycle
- run/session lifecycle
- cleanup lifecycle
- updater lifecycle

#### STATE-009 — No State Collapsing Rule
Implementation convenience must not collapse distinct states into one if doing so hides materially different behavior.

#### STATE-010 — No Fake Terminal Rule
A state must not be treated as terminal if valid forward transitions still exist under normal behavior.

---

### 7.3 State Machine A — App Lifecycle

### 7.3.1 Purpose
This state machine governs the lifecycle of the desktop application as a whole.

It answers:
- what happens from app launch to ready state
- how runtime hydration and startup janitor behavior fit into launch
- what state the app enters when a run is active
- how normal shutdown differs from startup failure

---

### 7.3.2 Canonical App Lifecycle States

#### STATE-011 — `AppNotRunning`
The app process is not currently active.

#### STATE-012 — `AppLaunching`
The user has started the app process, but startup evaluation is not yet complete.

#### STATE-013 — `StartupJanitorRunning`
The app is performing startup janitor logic to detect and purge abandoned workspaces from prior abnormal termination.

#### STATE-014 — `StartupChecksRunning`
The app is evaluating core readiness conditions, including at minimum:
- runtime presence or absence
- runtime integrity
- local entitlement availability
- activation token availability
- configuration readiness

#### STATE-015 — `RuntimeHydrationRequired`
The app has determined that the required heavy runtime is absent or requires reinstallation/repair before simulation features can be enabled.

#### STATE-016 — `RuntimeHydrating`
The app is actively downloading, installing, or repairing the heavy runtime package.

#### STATE-017 — `ActivationRequired`
The app is locally operational enough to proceed, but the device does not yet have valid activation state for normal licensed use.

#### STATE-018 — `ConfigurationRequired`
The device is activated, but one or more required simulation configuration items are missing, such as provider selection or provider API key.

#### STATE-019 — `ReadyIdle`
The app is fully ready to accept or prepare a new run, with no active run in progress.

#### STATE-020 — `RunPreparationActive`
The app is engaged in run preparation behavior such as seed selection, configuration collection, or Preflight interaction, but no active run has started yet.

#### STATE-021 — `RunActive`
A run is currently active, including ingestion, persona generation, active debate, paused/degraded run handling, finalization, or controlled cancellation before full run termination.

#### STATE-022 — `AppShuttingDown`
The app is performing normal close behavior, including any required active-run shutdown coordination.

#### STATE-023 — `AppClosed`
The app process has completed its close path successfully.

#### STATE-024 — `StartupFatalError`
The app encountered a startup-blocking condition that prevents safe progression into activation, configuration, or ready idle behavior.

---

### 7.3.3 App Lifecycle Start State

#### STATE-025 — Start State Rule
The canonical start state for the app lifecycle is `AppNotRunning`.

---

### 7.3.4 Allowed App Lifecycle Transitions

#### STATE-026 — Launch Transition
`AppNotRunning -> AppLaunching`  
Trigger: user starts the app.

#### STATE-027 — Janitor Entry Transition
`AppLaunching -> StartupJanitorRunning`  
Trigger: process startup reaches initial lifecycle control.

#### STATE-028 — Janitor Completion Transition
`StartupJanitorRunning -> StartupChecksRunning`  
Trigger: janitor completes, times out safely, or yields control to standard startup checks.

#### STATE-029 — Runtime Missing Transition
`StartupChecksRunning -> RuntimeHydrationRequired`  
Trigger: runtime is missing or repair is required.

#### STATE-030 — Runtime Healthy / No Hydration Needed Transition
`StartupChecksRunning -> ActivationRequired`  
Trigger: runtime is healthy enough, but valid activation state is absent.

#### STATE-031 — Runtime Healthy / Activation Already Valid Transition
`StartupChecksRunning -> ConfigurationRequired`  
Trigger: runtime and activation are valid, but required provider configuration is incomplete.

#### STATE-032 — Fully Ready Transition
`StartupChecksRunning -> ReadyIdle`  
Trigger: runtime, activation, and required configuration readiness conditions are all satisfied.

#### STATE-033 — Startup Fatal Transition
`StartupChecksRunning -> StartupFatalError`  
Trigger: a startup-blocking condition exists that cannot be safely bypassed.

#### STATE-034 — Hydration Begin Transition
`RuntimeHydrationRequired -> RuntimeHydrating`  
Trigger: the user or app initiates authorized runtime hydration.

#### STATE-035 — Hydration Success Transition to Activation
`RuntimeHydrating -> ActivationRequired`  
Trigger: hydration completes successfully and activation is still missing.

#### STATE-036 — Hydration Success Transition to Configuration
`RuntimeHydrating -> ConfigurationRequired`  
Trigger: hydration completes successfully, activation is valid, but configuration is incomplete.

#### STATE-037 — Hydration Success Transition to Ready
`RuntimeHydrating -> ReadyIdle`  
Trigger: hydration completes successfully and all other readiness conditions are already satisfied.

#### STATE-038 — Hydration Failure Transition
`RuntimeHydrating -> RuntimeHydrationRequired`  
Trigger: hydration fails but remains retryable.

#### STATE-039 — Activation Completion Transition
`ActivationRequired -> ConfigurationRequired`  
Trigger: activation succeeds, but configuration is still incomplete.

#### STATE-040 — Activation Completion Direct-to-Ready Transition
`ActivationRequired -> ReadyIdle`  
Trigger: activation succeeds and required configuration is already present.

#### STATE-041 — Configuration Completion Transition
`ConfigurationRequired -> ReadyIdle`  
Trigger: required provider configuration becomes valid.

#### STATE-042 — Enter Run Preparation Transition
`ReadyIdle -> RunPreparationActive`  
Trigger: the user begins seed selection, run setup, or Preflight-related activity.

#### STATE-043 — Return to Idle from Preparation
`RunPreparationActive -> ReadyIdle`  
Trigger: preparation is abandoned, reset, or concluded without an active run start.

#### STATE-044 — Run Start Transition
`RunPreparationActive -> RunActive`  
Trigger: a run starts successfully after approved Preflight.

#### STATE-045 — Run End Return Transition
`RunActive -> ReadyIdle`  
Trigger: the active run fully terminates and the app returns to idle-ready state.

#### STATE-046 — Normal Close from Idle Transition
`ReadyIdle -> AppShuttingDown`  
Trigger: user initiates app close.

#### STATE-047 — Normal Close from Preparation Transition
`RunPreparationActive -> AppShuttingDown`  
Trigger: user initiates app close before run start.

#### STATE-048 — Normal Close from Active Run Transition
`RunActive -> AppShuttingDown`  
Trigger: user initiates app close during or after active run control period.

#### STATE-049 — Final Close Transition
`AppShuttingDown -> AppClosed`  
Trigger: close sequence completes.

#### STATE-050 — Fatal Startup Exit Transition
`StartupFatalError -> AppClosed`  
Trigger: user exits app or app terminates after unrecoverable startup failure.

---

### 7.3.5 Invalid App Lifecycle Transitions

#### STATE-051 — Invalid Direct Idle Transition
`AppLaunching -> ReadyIdle` is invalid.  
Reason: startup janitor and startup checks must not be skipped.

#### STATE-052 — Invalid Hydration Skip Transition
`RuntimeHydrationRequired -> ReadyIdle` is invalid unless runtime health is actually restored through successful hydration or repair.

#### STATE-053 — Invalid Activation Skip Transition
`ActivationRequired -> RunPreparationActive` is invalid if activation remains unsatisfied.

#### STATE-054 — Invalid Configuration Skip Transition
`ConfigurationRequired -> RunActive` is invalid if mandatory configuration remains missing.

#### STATE-055 — Invalid Resume-by-Launch Transition
`AppLaunching -> RunActive` is invalid.  
Reason: the MVP does not support crash-resume run restoration.

#### STATE-056 — Invalid Fatal Recovery Bypass
`StartupFatalError -> ReadyIdle` is invalid unless the fatal condition is explicitly corrected through a new startup cycle.

---

### 7.3.6 App Lifecycle Terminal States

#### STATE-057 — App Lifecycle Terminal States
The terminal states of the app lifecycle are:
- `AppClosed`
- `StartupFatalError` only if the app remains blocked pending close or restart

---

### 7.3.7 App Lifecycle Side Effects

#### STATE-058 — Janitor Side Effect Rule
Entering `StartupJanitorRunning` must trigger abandoned-workspace discovery logic before normal readiness logic proceeds.

#### STATE-059 — Hydration Side Effect Rule
Entering `RuntimeHydrating` must trigger runtime acquisition or repair behavior rather than passive waiting.

#### STATE-060 — Active Run Side Effect Rule
Entering `RunActive` implies that a real run/session exists or is in immediately active startup phases, not merely that the user is looking at a run screen.

#### STATE-061 — Shutdown Side Effect Rule
Entering `AppShuttingDown` during an active run implies that controlled run termination or process shutdown behavior must be executed according to the product rules.

---

### 7.3.8 App Lifecycle User-Visible Mapping

#### STATE-062 — User-Visible Mapping Rule
The app should map canonical states to understandable user-visible messaging, for example:
- `RuntimeHydrationRequired` -> runtime installation required
- `RuntimeHydrating` -> downloading/installing runtime
- `ActivationRequired` -> activation needed
- `ConfigurationRequired` -> setup incomplete
- `ReadyIdle` -> ready
- `RunPreparationActive` -> preparing run
- `RunActive` -> run in progress
- `AppShuttingDown` -> closing app

These user-facing labels are illustrative and non-canonical.
The canonical internal state names above remain authoritative.

---

### 7.4 State Machine B — Device Activation Lifecycle

### 7.4.1 Purpose
This state machine governs the device activation process under a valid commercial license model.

It answers:
- when a device is considered unactivated
- how activation requests proceed
- how activation success and denial are represented
- how activation validity affects readiness

---

### 7.4.2 Canonical Activation States

#### STATE-063 — `Unactivated`
The device does not currently possess valid activation state for licensed use.

#### STATE-064 — `ActivationInitiated`
The user has entered activation flow and activation has been requested but not yet evaluated fully.

#### STATE-065 — `ActivationValidating`
The system is actively validating entitlement, device eligibility, and activation-slot permissibility.

#### STATE-066 — `ActivationApproved`
The backend has approved activation and a device-bound token is being or has just been issued for secure local persistence.

#### STATE-067 — `Activated`
The device possesses valid locally stored activation state and may participate in licensed local use subject to other readiness conditions.

#### STATE-068 — `ActivationDenied`
Activation was rejected due to a known disqualifying condition.

#### STATE-069 — `ActivationError`
Activation could not complete because of a non-policy error, such as transient network or secure-storage failure.

#### STATE-070 — `ActivationRevoked`
A previously activated device is no longer considered valid for licensed use due to revocation or local validation failure against the stored activation contract.

---

### 7.4.3 Activation Lifecycle Start State

#### STATE-071 — Activation Start State Rule
The canonical start state for activation lifecycle on a new device is `Unactivated`.

---

### 7.4.4 Allowed Activation Transitions

#### STATE-072 — Activation Begin Transition
`Unactivated -> ActivationInitiated`  
Trigger: user starts activation flow.

#### STATE-073 — Validation Transition
`ActivationInitiated -> ActivationValidating`  
Trigger: activation request is submitted for entitlement/device evaluation.

#### STATE-074 — Approval Transition
`ActivationValidating -> ActivationApproved`  
Trigger: entitlement, slot, and device eligibility checks pass.

#### STATE-075 — Local Activation Commit Transition
`ActivationApproved -> Activated`  
Trigger: device-bound JWT is successfully validated and stored locally.

#### STATE-076 — Denial Transition
`ActivationValidating -> ActivationDenied`  
Trigger: known policy disqualifier, such as invalid license or activation-slot limit.

#### STATE-077 — Error Transition
`ActivationValidating -> ActivationError`  
Trigger: activation cannot complete due to technical failure rather than policy denial.

#### STATE-078 — Retryable Error Return Transition
`ActivationError -> Unactivated`  
Trigger: the failed attempt ends without valid activation state.

#### STATE-079 — Denied Return Transition
`ActivationDenied -> Unactivated`  
Trigger: user exits failed activation state or later retries after correction of the denial cause.

#### STATE-080 — Revocation Transition
`Activated -> ActivationRevoked`  
Trigger: local or backend-governed activation validity is no longer acceptable under the product’s license model.

#### STATE-081 — Re-activation from Revoked Transition
`ActivationRevoked -> ActivationInitiated`  
Trigger: user starts a new valid activation attempt.

---

### 7.4.5 Invalid Activation Transitions

#### STATE-082 — Invalid Direct Activation Transition
`Unactivated -> Activated` is invalid.  
Reason: activation must not bypass validation and local token storage steps.

#### STATE-083 — Invalid Denial-to-Activated Transition
`ActivationDenied -> Activated` is invalid unless a new successful activation cycle occurs.

#### STATE-084 — Invalid Error-to-Activated Transition
`ActivationError -> Activated` is invalid unless a new successful activation cycle occurs.

#### STATE-085 — Invalid Revoked Ready-State Assumption
`ActivationRevoked` must not be treated as functionally equivalent to `Activated`.

---

### 7.4.6 Activation Lifecycle Terminal and Stable States

#### STATE-086 — Activation Stable States
The stable endpoint states of the activation lifecycle are:
- `Activated`
- `Unactivated`
- `ActivationDenied`
- `ActivationRevoked`

#### STATE-087 — Activation Transient States
The transient operational states are:
- `ActivationInitiated`
- `ActivationValidating`
- `ActivationApproved`
- `ActivationError`

---

### 7.4.7 Activation Denial Behavior Mapping

#### STATE-088 — Activation Denial Mapping Rule
`ActivationDenied` should be used when the product knows activation is disallowed under the current licensing rules, including at minimum:
- invalid entitlement
- expired license where not permitted
- device limit exceeded
- device-binding mismatch logic failure where applicable

#### STATE-089 — Activation Error Mapping Rule
`ActivationError` should be used for technical failures rather than policy denials, including at minimum:
- connectivity failure during request
- secure local storage failure
- malformed or unreadable token response
- unexpected backend failure

---

### 7.4.8 Activation User-Visible Mapping

#### STATE-090 — Activation UI Truth Rule
The UI must distinguish between:
- activation required
- activation in progress
- activation denied
- activation failed due to technical error
- activation successful

These must not be collapsed into one ambiguous “activation problem” message when the underlying cause is known.

---

### 7.5 State Machine C — Preflight Lifecycle

### 7.5.1 Purpose
This state machine governs the lifecycle of Preflight evaluation for a pending run configuration.

It answers:
- when Preflight exists
- when it is running
- what outcome states are allowed
- when a prior result is invalidated
- how run start eligibility relates to the latest Preflight state

---

### 7.5.2 Canonical Preflight States

#### STATE-091 — `PreflightNotStarted`
No Preflight has yet been run for the current pending run configuration.

#### STATE-092 — `PreflightRunning`
Preflight evaluation is actively executing.

#### STATE-093 — `PreflightApproved`
The current pending run configuration is valid and feasible without downgrade.

#### STATE-094 — `PreflightApprovedWithDowngrade`
The current pending run configuration is valid only under an explicitly reduced effective plan.

#### STATE-095 — `PreflightRejected`
The current pending run configuration may not proceed to active run start.

#### STATE-096 — `PreflightInvalidated`
A previously obtained Preflight result is no longer valid for the current pending run configuration.

#### STATE-097 — `PreflightError`
Preflight could not complete because of a technical error rather than a policy or feasibility rejection.

---

### 7.5.3 Preflight Lifecycle Start State

#### STATE-098 — Preflight Start State Rule
The canonical start state for a new pending run configuration is `PreflightNotStarted`.

---

### 7.5.4 Allowed Preflight Transitions

#### STATE-099 — Preflight Begin Transition
`PreflightNotStarted -> PreflightRunning`  
Trigger: user initiates Preflight.

#### STATE-100 — Approval Transition
`PreflightRunning -> PreflightApproved`  
Trigger: the pending run configuration is valid and feasible without downgrade.

#### STATE-101 — Downgraded Approval Transition
`PreflightRunning -> PreflightApprovedWithDowngrade`  
Trigger: the pending configuration is feasible only under a reduced effective plan.

#### STATE-102 — Rejection Transition
`PreflightRunning -> PreflightRejected`  
Trigger: the pending configuration is invalid, unsupported, or infeasible.

#### STATE-103 — Technical Failure Transition
`PreflightRunning -> PreflightError`  
Trigger: technical error prevents Preflight completion.

#### STATE-104 — Retry After Error Transition
`PreflightError -> PreflightRunning`  
Trigger: user or system retries Preflight.

#### STATE-105 — Retry After Rejection Transition
`PreflightRejected -> PreflightRunning`  
Trigger: the user has changed configuration or reattempts evaluation under a corrected setup.

#### STATE-106 — Retry After Invalidated Transition
`PreflightInvalidated -> PreflightRunning`  
Trigger: a new Preflight attempt is started for the changed configuration.

#### STATE-107 — Invalidation Transition from Approval
`PreflightApproved -> PreflightInvalidated`  
Trigger: a materially relevant run input changes after approval.

#### STATE-108 — Invalidation Transition from Downgraded Approval
`PreflightApprovedWithDowngrade -> PreflightInvalidated`  
Trigger: a materially relevant run input changes after the result is obtained.

#### STATE-109 — Invalidation Transition from Rejection
`PreflightRejected -> PreflightInvalidated`  
Trigger: a materially relevant run input changes after rejection, meaning the prior decision is no longer about the current configuration.

#### STATE-110 — Invalidation Transition from Error
`PreflightError -> PreflightInvalidated`  
Trigger: materially relevant input changes after the failed attempt.

#### STATE-111 — Reset Transition
`PreflightInvalidated -> PreflightNotStarted`  
Trigger: the app resets pending configuration state or explicitly clears the stale result.

---

### 7.5.5 Material Input Change Rules for Invalidation

#### STATE-112 — Material Input Change Rule
A Preflight result must be invalidated if any material run input changes after the result is produced.

#### STATE-113 — Material Inputs Requiring Invalidation
Material changes include at minimum:
- seed document changed
- provider changed
- model changed where model selection is exposed
- requested agent count changed
- requested round count changed
- max token budget changed
- stored provider key changed for the active provider
- runtime validity materially changed
- activation/licensing validity materially changed

#### STATE-114 — Non-Material Change Rule
A purely cosmetic UI change that does not affect run feasibility or execution semantics must not invalidate a valid Preflight result.

---

### 7.5.6 Invalid Preflight Transitions

#### STATE-115 — Invalid Direct Approval Transition
`PreflightNotStarted -> PreflightApproved` is invalid.  
Reason: approval must not exist without a real Preflight evaluation.

#### STATE-116 — Invalid Direct Run Eligibility from Rejected
`PreflightRejected` must not be treated as run-start eligible.

#### STATE-117 — Invalid Silent Reuse Rule
A stale approval state must not be silently reused after material input changes.

#### STATE-118 — Invalid Auto-Upgrade Rule
`PreflightApprovedWithDowngrade -> PreflightApproved` is invalid unless a new Preflight evaluation occurs.

#### STATE-119 — Invalid Error-as-Rejection Rule
`PreflightError` must not be treated as functionally equivalent to `PreflightRejected`.
One is technical failure; the other is a determined non-approval outcome.

---

### 7.5.7 Preflight Run-Start Eligibility

#### STATE-120 — Run-Start Eligible States
Only these Preflight states are eligible for run start:
- `PreflightApproved`
- `PreflightApprovedWithDowngrade`

#### STATE-121 — Non-Eligible Preflight States
These states are not eligible for run start:
- `PreflightNotStarted`
- `PreflightRunning`
- `PreflightRejected`
- `PreflightInvalidated`
- `PreflightError`

#### STATE-122 — Downgrade Acceptance Requirement
`PreflightApprovedWithDowngrade` is run-start eligible only if the user is informed of the effective plan and proceeds under that approved reduced plan.

---

### 7.5.8 Preflight User-Visible Mapping

#### STATE-123 — Preflight Truthful Status Rule
The UI must distinguish at minimum between:
- not run yet
- currently running
- approved
- approved with downgrade
- rejected
- technical evaluation error
- stale/invalidated result

#### STATE-124 — Rejection vs Error Rule
The UI must not collapse `PreflightRejected` and `PreflightError` into the same message when the system knows which one occurred.

#### STATE-125 — Invalidated Result Rule
If a previously valid Preflight result is invalidated by configuration change, the app must not present it as still current.

---

### 7.5.9 Preflight Terminal and Stable States

#### STATE-126 — Stable Preflight States
The stable non-transient Preflight states are:
- `PreflightNotStarted`
- `PreflightApproved`
- `PreflightApprovedWithDowngrade`
- `PreflightRejected`
- `PreflightInvalidated`
- `PreflightError`

#### STATE-127 — Transient Preflight State
The transient operational state is:
- `PreflightRunning`

---

### 7.6 Cross-State Constraints for Part 1

#### STATE-128 — App and Activation Consistency Rule
The app must not present itself as fully ready for licensed run use if the activation lifecycle is not in `Activated`.

#### STATE-129 — App and Preflight Consistency Rule
The app must not enter `RunActive` unless the Preflight lifecycle is in a run-start-eligible state.

#### STATE-130 — Startup and Run Separation Rule
The app lifecycle must not skip startup janitor and startup checks just because a prior run existed.
Crash-resume behavior is out of scope.

#### STATE-131 — Preflight Freshness Rule
Run start must depend on the current valid Preflight state for the current pending configuration, not on an obsolete approval result.

#### STATE-132 — Truthful Lifecycle Reporting Rule
Where app state, activation state, and Preflight state are shown together, the app must reflect their real combination rather than flattening them into one misleading “ready/not ready” binary.

---

### 7.7 Section Boundary for Part 1

#### STATE-133 — Part Boundary Rule
This part of Section 7 defines:
- app lifecycle states
- device activation lifecycle states
- Preflight lifecycle states
- their valid transitions
- their invalid transitions
- their user-visible truth requirements

This part does not yet define:
- run/session lifecycle states
- cleanup lifecycle states
- updater lifecycle states
- full cross-machine or backend synchronization semantics

Those are defined in the continuation of Section 7.

---

### 7.8 State Machine D — Run / Session Lifecycle

### 7.8.1 Purpose
This state machine governs the lifecycle of a run/session after the user has moved beyond basic app readiness and into actual run execution logic.

It answers:
- what the canonical run states are
- how a run progresses from creation to completion, cancellation, or failure
- how degraded and paused conditions behave
- how finalization is entered
- which terminal states are valid
- how cleanup-related state follows run termination

This is the most important execution state machine in the product and must remain consistent with:
- Section 5 — User Journeys and Behavioral Narratives
- Section 6 — Product Behavior Specification

---

### 7.8.2 Canonical Run / Session States

#### STATE-134 — `RunDraft`
A run context exists conceptually or locally in preparation form, but active Preflight-backed execution has not yet started.

#### STATE-135 — `RunQueued`
The run has been accepted for start and local run/session scaffolding exists, but active execution work has not yet entered ingestion or persona generation.

#### STATE-136 — `Ingesting`
The seed document is being ingested into the run-local execution context.

#### STATE-137 — `GeneratingPersonas`
The system is generating the runtime persona set, including the Moderator Agent.

#### STATE-138 — `RunningRound`
The run is in active moderated debate execution for a specific round.

#### STATE-139 — `PausedRateLimit`
The run is temporarily unable to make normal forward progress because provider rate-limit handling is active.

#### STATE-140 — `PausedNetwork`
The run is temporarily unable to make normal forward progress because network connectivity or provider reachability is impaired.

#### STATE-141 — `Finalizing`
The run has stopped normal round scheduling and is synthesizing completed material into final output.

#### STATE-142 — `Completed`
The run reached valid completion and successfully generated the required report through the normal or fully planned completion path.

#### STATE-143 — `CompletedBudgetCapped`
The run reached valid completion after controlled early finalization due to budget or equivalent bounded truncation condition, and the required report was generated successfully.

#### STATE-144 — `Canceled`
The run was deliberately and controlledly stopped before valid completion.

#### STATE-145 — `FailedRetryable`
The run encountered a technical condition that may allow retry or re-attempt logic without implying that the run already completed.

#### STATE-146 — `FailedTerminal`
The run encountered a terminal failure condition and did not produce a valid report-generating completion.

#### STATE-147 — `Cleaning`
The run has already reached a terminal execution path and the system is now attempting cleanup of transient run-local artifacts.

#### STATE-148 — `Cleaned`
Cleanup succeeded sufficiently according to the product’s best-effort policy.

#### STATE-149 — `CleanupFailed`
Cleanup did not fully succeed.

---

### 7.8.3 Run / Session Start State

#### STATE-150 — Run Start State Rule
The canonical start state for a newly created run/session context is `RunDraft`.

---

### 7.8.4 Run / Session Allowed Transitions

#### STATE-151 — Draft to Queued Transition
`RunDraft -> RunQueued`  
Trigger: the user has a run-start-eligible Preflight result and the app commits to run creation.

#### STATE-152 — Queued to Ingesting Transition
`RunQueued -> Ingesting`  
Trigger: active run execution begins and the seed document enters ingestion.

#### STATE-153 — Ingesting to Persona Generation Transition
`Ingesting -> GeneratingPersonas`  
Trigger: document ingestion has progressed sufficiently for persona generation to begin.

#### STATE-154 — Persona Generation to RunningRound Transition
`GeneratingPersonas -> RunningRound`  
Trigger: persona generation succeeds and the first round is ready to begin.

#### STATE-155 — RunningRound Self-Continuation Transition
`RunningRound -> RunningRound`  
Trigger: one round completes and another valid round begins under the same canonical active-round state family.

#### STATE-156 — RunningRound to PausedRateLimit Transition
`RunningRound -> PausedRateLimit`  
Trigger: bounded rate-limit handling blocks normal immediate progression.

#### STATE-157 — RunningRound to PausedNetwork Transition
`RunningRound -> PausedNetwork`  
Trigger: bounded network/provider reachability degradation blocks normal immediate progression.

#### STATE-158 — PausedRateLimit to RunningRound Transition
`PausedRateLimit -> RunningRound`  
Trigger: rate-limit pause resolves and normal round progression may resume.

#### STATE-159 — PausedNetwork to RunningRound Transition
`PausedNetwork -> RunningRound`  
Trigger: connectivity or provider reachability recovers sufficiently for normal round progression.

#### STATE-160 — RunningRound to Finalizing Transition
`RunningRound -> Finalizing`  
Trigger: one of the following occurs:
- effective round plan completes
- budget cap is hit
- duration cap is hit
- a controlled degraded completion path is chosen
- a manual controlled finalize path is validly invoked

#### STATE-161 — PausedRateLimit to Finalizing Transition
`PausedRateLimit -> Finalizing`  
Trigger: the system determines meaningful finalization is preferable to continued waiting.

#### STATE-162 — PausedNetwork to Finalizing Transition
`PausedNetwork -> Finalizing`  
Trigger: the system determines meaningful finalization is preferable to continued waiting.

#### STATE-163 — RunningRound to FailedRetryable Transition
`RunningRound -> FailedRetryable`  
Trigger: a technical execution problem interrupts current forward progress but is not yet terminally classified.

#### STATE-164 — PausedRateLimit to FailedRetryable Transition
`PausedRateLimit -> FailedRetryable`  
Trigger: bounded retry logic exhausts or a retryable but unresolved degraded state must be surfaced.

#### STATE-165 — PausedNetwork to FailedRetryable Transition
`PausedNetwork -> FailedRetryable`  
Trigger: temporary degraded behavior has not resolved and a retryable failure must be surfaced.

#### STATE-166 — FailedRetryable to RunningRound Transition
`FailedRetryable -> RunningRound`  
Trigger: valid retry/recovery succeeds and normal debate execution may continue.

#### STATE-167 — FailedRetryable to Finalizing Transition
`FailedRetryable -> Finalizing`  
Trigger: enough run material exists and the system chooses controlled finalization instead of terminal failure.

#### STATE-168 — FailedRetryable to FailedTerminal Transition
`FailedRetryable -> FailedTerminal`  
Trigger: retry/recovery is exhausted or impossible and meaningful finalization is not viable.

#### STATE-169 — Ingesting to FailedTerminal Transition
`Ingesting -> FailedTerminal`  
Trigger: ingestion fails irrecoverably for the run.

#### STATE-170 — GeneratingPersonas to FailedTerminal Transition
`GeneratingPersonas -> FailedTerminal`  
Trigger: persona generation fails irrecoverably for the run.

#### STATE-171 — Finalizing to Completed Transition
`Finalizing -> Completed`  
Trigger: finalization succeeds and the run qualifies as normal successful completion.

#### STATE-172 — Finalizing to CompletedBudgetCapped Transition
`Finalizing -> CompletedBudgetCapped`  
Trigger: finalization succeeds under budget-capped or other controlled truncated completion conditions.

#### STATE-173 — Finalizing to FailedTerminal Transition
`Finalizing -> FailedTerminal`  
Trigger: finalization fails to produce a valid report-completion outcome.

#### STATE-174 — Cancel Request from Queued Transition
`RunQueued -> Canceled`  
Trigger: user or app cancels before ingestion begins.

#### STATE-175 — Cancel Request from Ingesting Transition
`Ingesting -> Canceled`  
Trigger: controlled cancellation is accepted before forward execution completes ingestion.

#### STATE-176 — Cancel Request from GeneratingPersonas Transition
`GeneratingPersonas -> Canceled`  
Trigger: controlled cancellation is accepted during persona generation.

#### STATE-177 — Cancel Request from RunningRound Transition
`RunningRound -> Canceled`  
Trigger: controlled cancellation is accepted during active debate.

#### STATE-178 — Cancel Request from PausedRateLimit Transition
`PausedRateLimit -> Canceled`  
Trigger: controlled cancellation is accepted during rate-limit pause.

#### STATE-179 — Cancel Request from PausedNetwork Transition
`PausedNetwork -> Canceled`  
Trigger: controlled cancellation is accepted during network pause.

#### STATE-180 — Cancel Request from FailedRetryable Transition
`FailedRetryable -> Canceled`  
Trigger: the system or user chooses cancellation instead of retry/recovery or finalization.

#### STATE-181 — Completed to Cleaning Transition
`Completed -> Cleaning`  
Trigger: the run has reached terminal completion and cleanup begins.

#### STATE-182 — CompletedBudgetCapped to Cleaning Transition
`CompletedBudgetCapped -> Cleaning`  
Trigger: the run has reached terminal constrained completion and cleanup begins.

#### STATE-183 — Canceled to Cleaning Transition
`Canceled -> Cleaning`  
Trigger: controlled run termination has occurred and cleanup begins.

#### STATE-184 — FailedTerminal to Cleaning Transition
`FailedTerminal -> Cleaning`  
Trigger: terminal run failure has occurred and cleanup begins.

#### STATE-185 — Cleaning to Cleaned Transition
`Cleaning -> Cleaned`  
Trigger: cleanup succeeds sufficiently under best-effort policy.

#### STATE-186 — Cleaning to CleanupFailed Transition
`Cleaning -> CleanupFailed`  
Trigger: cleanup attempt does not fully succeed.

---

### 7.8.5 Run / Session Invalid Transitions

#### STATE-187 — Invalid Draft-to-Running Transition
`RunDraft -> RunningRound` is invalid.  
Reason: run creation, queueing, ingestion, and persona generation must not be skipped.

#### STATE-188 — Invalid Preflight Bypass Transition
No run state may transition directly from `RunDraft` to any active execution state without prior run-start eligibility under the Preflight lifecycle.

#### STATE-189 — Invalid Completed-to-Running Transition
`Completed -> RunningRound` is invalid.  
Reason: MVP does not support post-completion continuation.

#### STATE-190 — Invalid CompletedBudgetCapped-to-Running Transition
`CompletedBudgetCapped -> RunningRound` is invalid.  
Reason: constrained completion is still terminal.

#### STATE-191 — Invalid FailedTerminal-to-Running Transition
`FailedTerminal -> RunningRound` is invalid.  
Reason: terminal failure ends the run.

#### STATE-192 — Invalid Canceled-to-Running Transition
`Canceled -> RunningRound` is invalid.  
Reason: controlled cancellation is terminal.

#### STATE-193 — Invalid Cleaned-to-Running Transition
`Cleaned -> RunningRound` is invalid.  
Reason: a cleaned run cannot restart.

#### STATE-194 — Invalid CleanupFailed-to-Running Transition
`CleanupFailed -> RunningRound` is invalid.  
Reason: cleanup failure does not imply a resumable active run.

#### STATE-195 — Invalid Finalization Backflow Transition
`Finalizing -> RunningRound` is invalid.  
Reason: once finalization begins, normal round scheduling must not resume.

#### STATE-196 — Invalid Pause-to-Completed Transition
`PausedRateLimit -> Completed` and `PausedNetwork -> Completed` are invalid.  
Reason: a paused run must still go through finalization or another valid terminal path.

#### STATE-197 — Invalid Failure-as-Success Transition
`FailedRetryable -> Completed` is invalid unless it first passes through `Finalizing` and produces a valid report.

---

### 7.8.6 Run / Session Stable and Terminal States

#### STATE-198 — Stable Non-Transient Run States
The stable non-transient run states are:
- `RunDraft`
- `RunQueued`
- `PausedRateLimit`
- `PausedNetwork`
- `Completed`
- `CompletedBudgetCapped`
- `Canceled`
- `FailedRetryable`
- `FailedTerminal`
- `Cleaned`
- `CleanupFailed`

#### STATE-199 — Transient Operational Run States
The transient run states are:
- `Ingesting`
- `GeneratingPersonas`
- `RunningRound`
- `Finalizing`
- `Cleaning`

#### STATE-200 — Terminal Run States
The terminal execution states of the run before cleanup distinction are:
- `Completed`
- `CompletedBudgetCapped`
- `Canceled`
- `FailedTerminal`

#### STATE-201 — Terminal Full-Lifecycle States
The fully terminal end states including cleanup distinction are:
- `Cleaned`
- `CleanupFailed`

---

### 7.8.7 Run / Session User-Visible Mapping

#### STATE-202 — Run State Visibility Rule
The UI must expose truthful user-visible run status aligned with the canonical run state.

#### STATE-203 — Required Distinctions Rule
The UI must distinguish at minimum between:
- queued / starting
- ingesting
- generating personas
- active round execution
- paused due to rate limiting
- paused due to network/provider issues
- finalizing
- completed
- completed under budget cap
- canceled
- failed
- cleaning
- cleanup failed

#### STATE-204 — Completed vs Budget-Capped Distinction
The UI must not collapse `Completed` and `CompletedBudgetCapped` into an indistinguishable success label if that would hide meaningful run truth.

#### STATE-205 — Failure Distinction Rule
The UI should distinguish retryable degraded states from terminal failure when that distinction is known.

---

### 7.9 State Machine E — Cleanup Lifecycle

### 7.9.1 Purpose
This state machine governs the lifecycle of cleanup behavior for transient run-local artifacts and abandoned workspace handling.

It answers:
- when cleanup is pending
- when cleanup is actively running
- what counts as cleanup success vs failure
- how next-launch janitor cleanup relates to run cleanup

---

### 7.9.2 Canonical Cleanup States

#### STATE-206 — `NoCleanupPending`
No cleanup action is currently required for the relevant lifecycle scope.

#### STATE-207 — `CleanupPending`
Cleanup is required but has not yet begun.

#### STATE-208 — `CleanupRunning`
Cleanup activity is currently being executed.

#### STATE-209 — `CleanupSucceeded`
Cleanup completed sufficiently under best-effort policy.

#### STATE-210 — `CleanupFailed`
Cleanup did not fully succeed.

#### STATE-211 — `JanitorPending`
A stale or abandoned cleanup obligation has been discovered on startup and awaits janitor processing.

#### STATE-212 — `JanitorRunning`
Startup janitor cleanup logic is actively processing stale or abandoned run artifacts.

#### STATE-213 — `JanitorCompleted`
Startup janitor processing completed and stale cleanup obligations were resolved sufficiently.

#### STATE-214 — `JanitorFailed`
Startup janitor processing did not fully succeed.

---

### 7.9.3 Cleanup Lifecycle Start State

#### STATE-215 — Cleanup Start State Rule
For a run with no cleanup obligation yet, the cleanup lifecycle begins in `NoCleanupPending`.

---

### 7.9.4 Allowed Cleanup Transitions

#### STATE-216 — Run-Termination Cleanup Entry Transition
`NoCleanupPending -> CleanupPending`  
Trigger: a run reaches a terminal execution state that requires cleanup.

#### STATE-217 — Cleanup Start Transition
`CleanupPending -> CleanupRunning`  
Trigger: cleanup activity begins.

#### STATE-218 — Cleanup Success Transition
`CleanupRunning -> CleanupSucceeded`  
Trigger: cleanup completes sufficiently.

#### STATE-219 — Cleanup Failure Transition
`CleanupRunning -> CleanupFailed`  
Trigger: cleanup does not fully succeed.

#### STATE-220 — Abandoned Workspace Discovery Transition
`NoCleanupPending -> JanitorPending`  
Trigger: app startup discovers stale run artifacts from abnormal termination.

#### STATE-221 — Janitor Start Transition
`JanitorPending -> JanitorRunning`  
Trigger: startup janitor begins processing stale artifacts.

#### STATE-222 — Janitor Success Transition
`JanitorRunning -> JanitorCompleted`  
Trigger: janitor resolves stale artifacts sufficiently.

#### STATE-223 — Janitor Failure Transition
`JanitorRunning -> JanitorFailed`  
Trigger: janitor processing does not fully succeed.

#### STATE-224 — Cleanup Failure to Janitor Pending Transition
`CleanupFailed -> JanitorPending`  
Trigger: the app is later relaunched and stale artifacts remain subject to janitor handling.

#### STATE-225 — Janitor Completion Reset Transition
`JanitorCompleted -> NoCleanupPending`  
Trigger: stale cleanup obligations are fully cleared.

#### STATE-226 — Janitor Failure Persistence Transition
`JanitorFailed -> JanitorPending`  
Trigger: a future startup will retry janitor logic.

---

### 7.9.5 Cleanup Invalid Transitions

#### STATE-227 — Invalid Immediate Success Transition
`CleanupPending -> CleanupSucceeded` is invalid.  
Reason: cleanup must actually run.

#### STATE-228 — Invalid Failure-to-Success Skip Transition
`CleanupFailed -> CleanupSucceeded` is invalid unless a new cleanup or janitor run actually occurs.

#### STATE-229 — Invalid Janitor-as-Resume Transition
No janitor state may transition back into any active run state.  
Reason: janitor behavior purges stale artifacts; it does not restore interrupted runs.

#### STATE-230 — Invalid Cleanup Ignoring Transition
A terminal run state must not be treated as equivalent to `NoCleanupPending` if cleanup has not yet been attempted.

---

### 7.9.6 Cleanup Stable and Terminal States

#### STATE-231 — Stable Cleanup States
The stable cleanup states are:
- `NoCleanupPending`
- `CleanupPending`
- `CleanupSucceeded`
- `CleanupFailed`
- `JanitorPending`
- `JanitorCompleted`
- `JanitorFailed`

#### STATE-232 — Transient Cleanup States
The transient cleanup states are:
- `CleanupRunning`
- `JanitorRunning`

---

### 7.9.7 Cleanup User-Visible Mapping

#### STATE-233 — Cleanup Honesty Rule
The user-facing product must not imply cleanup succeeded if the canonical cleanup state is `CleanupFailed` or `JanitorFailed`.

#### STATE-234 — Cleanup Severity Display Rule
The app should surface cleanup failure when:
- stale workspace data may remain
- manual retry may be needed
- next-launch janitor retry remains pending

#### STATE-235 — Report Preservation Distinction Rule
Cleanup success or failure must not be described in a way that implies the final exported report was deleted, if the report was intentionally stored outside the transient workspace boundary.

---

### 7.10 State Machine F — Updater Lifecycle

### 7.10.1 Purpose
This state machine governs application update discovery, verification, and installation readiness.

It answers:
- how the app checks for updates
- how available updates are classified
- how verification affects installability
- how invalid or unsafe updates are rejected

This state machine applies to app update behavior, not simulation run behavior.

---

### 7.10.2 Canonical Updater States

#### STATE-236 — `UpdaterIdle`
No update check is currently in progress and no pending update action is active.

#### STATE-237 — `CheckingForUpdates`
The app is actively querying its update source for availability information.

#### STATE-238 — `NoUpdateAvailable`
The update check completed successfully and no newer applicable update is available.

#### STATE-239 — `UpdateAvailable`
A newer update has been discovered but has not yet been downloaded or verified for installability.

#### STATE-240 — `DownloadingUpdate`
The app is downloading the update artifact.

#### STATE-241 — `VerifyingUpdate`
The app is validating the integrity and signature of the downloaded update artifact.

#### STATE-242 — `UpdateReady`
The update artifact has been downloaded and verified successfully and is ready for application under the updater rules.

#### STATE-243 — `UpdateRejected`
The candidate update must not be installed because it failed trust or integrity requirements.

#### STATE-244 — `ApplyingUpdate`
The app is applying the verified update through the approved updater mechanism.

#### STATE-245 — `UpdateApplied`
The update has been applied successfully and the app is ready for restart or has transitioned through the required restart boundary.

#### STATE-246 — `UpdaterError`
The updater encountered a technical failure that is not the same as a trust-based rejection.

---

### 7.10.3 Updater Lifecycle Start State

#### STATE-247 — Updater Start State Rule
The canonical updater start state is `UpdaterIdle`.

---

### 7.10.4 Allowed Updater Transitions

#### STATE-248 — Update Check Begin Transition
`UpdaterIdle -> CheckingForUpdates`  
Trigger: scheduled or user-initiated update check begins.

#### STATE-249 — No Update Transition
`CheckingForUpdates -> NoUpdateAvailable`  
Trigger: update check completes and no applicable update exists.

#### STATE-250 — Update Found Transition
`CheckingForUpdates -> UpdateAvailable`  
Trigger: a newer applicable update is found.

#### STATE-251 — Update Download Begin Transition
`UpdateAvailable -> DownloadingUpdate`  
Trigger: app or user proceeds to fetch the update artifact.

#### STATE-252 — Verification Begin Transition
`DownloadingUpdate -> VerifyingUpdate`  
Trigger: the update artifact download completes sufficiently for verification.

#### STATE-253 — Ready Transition
`VerifyingUpdate -> UpdateReady`  
Trigger: signature and integrity checks succeed.

#### STATE-254 — Rejection Transition
`VerifyingUpdate -> UpdateRejected`  
Trigger: signature, integrity, or trust validation fails.

#### STATE-255 — Technical Failure Transition
`CheckingForUpdates -> UpdaterError`  
Trigger: update discovery fails technically.

#### STATE-256 — Download Failure Transition
`DownloadingUpdate -> UpdaterError`  
Trigger: update download fails technically.

#### STATE-257 — Verification Technical Failure Transition
`VerifyingUpdate -> UpdaterError`  
Trigger: verification cannot complete due to technical rather than trust-class failure.

#### STATE-258 — Apply Transition
`UpdateReady -> ApplyingUpdate`  
Trigger: the app applies the verified update according to updater policy.

#### STATE-259 — Apply Success Transition
`ApplyingUpdate -> UpdateApplied`  
Trigger: update apply completes successfully.

#### STATE-260 — Apply Failure Transition
`ApplyingUpdate -> UpdaterError`  
Trigger: applying the update fails technically.

#### STATE-261 — Recovery to Idle from No Update
`NoUpdateAvailable -> UpdaterIdle`  
Trigger: updater returns to idle.

#### STATE-262 — Recovery to Idle from Rejection
`UpdateRejected -> UpdaterIdle`  
Trigger: candidate update is discarded or updater resets.

#### STATE-263 — Recovery to Idle from Applied
`UpdateApplied -> UpdaterIdle`  
Trigger: updater lifecycle resets after successful application flow.

#### STATE-264 — Recovery to Idle from Error
`UpdaterError -> UpdaterIdle`  
Trigger: updater flow resets after error handling.

---

### 7.10.5 Updater Invalid Transitions

#### STATE-265 — Invalid Direct Apply Transition
`UpdateAvailable -> ApplyingUpdate` is invalid.  
Reason: download and verification must not be skipped.

#### STATE-266 — Invalid Rejected-to-Applying Transition
`UpdateRejected -> ApplyingUpdate` is invalid.  
Reason: rejected artifacts must never be applied.

#### STATE-267 — Invalid Error-to-Ready Transition
`UpdaterError -> UpdateReady` is invalid unless a new valid updater cycle succeeds.

#### STATE-268 — Invalid Trust Bypass Transition
No updater flow may transition from a discovered update into `UpdateReady` without passing through verification.

---

### 7.10.6 Updater Stable and Terminal States

#### STATE-269 — Stable Updater States
The stable updater states are:
- `UpdaterIdle`
- `NoUpdateAvailable`
- `UpdateAvailable`
- `UpdateReady`
- `UpdateRejected`
- `UpdateApplied`
- `UpdaterError`

#### STATE-270 — Transient Updater States
The transient updater states are:
- `CheckingForUpdates`
- `DownloadingUpdate`
- `VerifyingUpdate`
- `ApplyingUpdate`

---

### 7.10.7 Updater User-Visible Mapping

#### STATE-271 — Updater Truth Rule
The app must not present an update as ready if it has not yet passed verification.

#### STATE-272 — Rejection Messaging Rule
If an update is rejected because trust or integrity checks fail, the app must not collapse that into a generic “update unavailable” message if the distinction is known.

#### STATE-273 — No Unsafe Update Messaging Rule
The product must never encourage installation of an update artifact that has been rejected under the canonical updater state machine.

---

### 7.11 Full-System Cross-State Integrity Rules

#### STATE-274 — Activation-to-App Readiness Rule
The app lifecycle must not enter a truly ready-for-run condition if the activation lifecycle is not effectively `Activated`.

#### STATE-275 — Preflight-to-Run Eligibility Rule
The run/session lifecycle must not leave `RunDraft` or enter `RunQueued` for actual start unless the Preflight lifecycle is in a run-start-eligible state.

#### STATE-276 — Configuration-to-Run Eligibility Rule
If required provider configuration is missing, the app lifecycle must not honestly represent the system as capable of entering valid active run execution.

#### STATE-277 — Run-to-App Mapping Rule
When the run/session lifecycle is in any of the following states:
- `Ingesting`
- `GeneratingPersonas`
- `RunningRound`
- `PausedRateLimit`
- `PausedNetwork`
- `Finalizing`
- `FailedRetryable`
- `Cleaning`

the app lifecycle must remain within a truthful run-active or shutdown-associated posture rather than pretending the app is idle.

#### STATE-278 — Cleanup-to-Run Terminal Rule
Cleanup lifecycle entry requires that the run/session lifecycle has already reached a terminal execution path or has been otherwise force-terminated under allowed shutdown behavior.

#### STATE-279 — No Cleanup-as-Success Rule
Cleanup success must not retroactively convert a failed or canceled run into a successful run classification.

#### STATE-280 — No Cleanup-failure-as-Run-failure Rewrite Rule
Cleanup failure must not rewrite a previously truthful run outcome.
For example:
- `Completed -> CleanupFailed` is still a completed run with failed cleanup
- not a non-completed run

#### STATE-281 — Abnormal Termination Rule
If abnormal termination interrupts the app lifecycle, the app must not re-enter a prior run/session active state on next launch.
Instead:
- app lifecycle restarts from launch
- cleanup lifecycle may enter janitor paths
- run/session lifecycle does not resume

#### STATE-282 — Updater Isolation Rule
The updater lifecycle must remain semantically separate from run execution.
The presence of an available update must not rewrite current run truth.

#### STATE-283 — No Unsafe Update During Critical Run Rule
The system should avoid forcing update application in a way that silently disrupts an active run.
If update timing intersects with run activity, active run truth and controlled shutdown rules still apply.

#### STATE-284 — Terminal Run Truth Rule
The final user-visible understanding of a run must be determined by the canonical run/session state, not by later unrelated app or updater state.

#### STATE-285 — User-Visible Composite Truth Rule
If multiple state machines are simultaneously relevant, the user-facing product should communicate the composite truth, for example:
- app ready but activation missing
- app healthy but Preflight rejected
- run completed but cleanup failed
- update available while app otherwise ready
- run paused due to network degradation

#### STATE-286 — No Cross-State Ambiguity Rule
The system must not flatten multiple state-machine truths into a single misleading label such as “ready” or “error” if that would conceal important distinctions.

#### STATE-287 — Logging and Diagnostics Consistency Rule
Diagnostics and logs should preserve the canonical state names or unambiguous mappings so engineering and QA can trace actual lifecycle behavior.

#### STATE-288 — Acceptance Traceability Rule
All later acceptance criteria and QA tests must map to these state machines rather than inventing contradictory lifecycle language.

---

### 7.12 Section 7 Summary

#### STATE-289 — Section Summary Rule
At the lifecycle level, the product is defined through distinct but coordinated state machines for:
- app lifecycle
- device activation
- Preflight
- run/session execution
- cleanup
- updater behavior

These state machines collectively ensure that:
- readiness is truthful
- execution is bounded
- completion is honest
- cleanup is distinguished from run outcome
- updates are verified before installation
- abnormal termination leads to janitor cleanup rather than hidden resume behavior

#### STATE-290 — Section Completion Rule
Section 7 is complete only when all later architecture, interface, and acceptance sections remain consistent with the canonical states and transition rules defined here.

#### STATE-291 — Section Boundary Rule
Section 7 defines lifecycle states and transitions only.
It does not yet define:
- architecture ownership responsibilities
- exact endpoint payloads
- exact UI component structures
- exact database tables
- full acceptance test wording

Those belong in later sections.

---

## 8. System Architecture

### 8.1 Section Purpose
This section defines the structural composition of the system and the ownership boundaries between its major components.

This section exists to ensure that:
- each major responsibility has one authoritative owner
- AI coding agents do not blur layer boundaries
- implementation remains consistent with the product behavior and state machines
- the system remains buildable, inspectable, and debuggable
- security, orchestration, simulation, and commercial functions are not accidentally collapsed into one layer

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- Section 6 — Product Behavior Specification
- Section 7 — State Machines

If any architectural choice conflicts with a higher-authority section, the higher-authority section wins.

---

### 8.2 Architecture Philosophy

#### ARCH-001 — Architecture Purpose Rule
The architecture must serve the product’s actual operating model:
- local-data-first
- cloud-inference-dependent
- bounded
- moderated
- desktop-native in orchestration
- commercially licensed but not commercially cloud-executed

#### ARCH-002 — Separation of Concerns Rule
The system must be split into distinct responsibility layers rather than implemented as one monolithic process with ambiguous ownership.

#### ARCH-003 — One Owner per Responsibility Rule
Every critical responsibility in the product must have exactly one primary owner.
No responsibility should be owned by multiple layers unless this document explicitly states shared responsibility.

#### ARCH-004 — Local Supervision Rule
The desktop application shell must supervise local execution rather than delegating process-control authority to the frontend.

#### ARCH-005 — Cloud Boundary Rule
The commercial cloud backend must support licensing, activation, runtime distribution, and updates, but it must not become the core execution environment for simulation runs in MVP.

#### ARCH-006 — Bounded Local Service Rule
The Python engine may expose a local backend interface, but that interface must remain an app-internal localhost service rather than a public platform API.

#### ARCH-007 — Product-Truth Preservation Rule
Architecture must preserve the behavioral truth of the product.
Implementation shortcuts must not weaken:
- Preflight gating
- bounded execution
- budget control
- cleanup behavior
- activation control
- truthfulness of run state

---

### 8.3 Canonical Component Map

#### ARCH-008 — Major Component Rule
The product architecture consists of the following major components:

1. **Frontend UI Layer**
2. **Rust/Tauri Application Shell**
3. **Python Simulation Engine**
4. **Cloud Licensing and Distribution Backend**
5. **Third-Party LLM Provider Layer**
6. **Payment Provider Layer**
7. **OS-Native Facilities Layer**

#### ARCH-009 — Component Role Boundary Rule
Each component exists to perform a different class of work.
A component must not silently absorb another component’s duties without explicit authorization in this document.

---

### 8.4 Frontend UI Layer

### 8.4.1 Purpose
The frontend exists to present product state, collect user input, and render user-visible outputs.

#### ARCH-010 — Frontend Technology Role
The frontend is the user-facing desktop interface layer rendered inside the app shell.

#### ARCH-011 — Frontend Primary Responsibilities
The frontend must own:
- rendering screens, flows, and forms
- collecting run configuration inputs
- displaying readiness state
- displaying Preflight outcomes
- displaying live run progress
- rendering transcript/event feeds
- rendering warnings, interruptions, and failure states
- providing user actions such as start, cancel, close, export, and retry
- presenting report access affordances

#### ARCH-012 — Frontend Non-Ownership Rule
The frontend must not own:
- secure key storage
- local process spawning
- runtime hydration
- license validation logic authority
- cleanup authority
- file-system deletion authority
- authoritative run orchestration
- provider authentication logic
- cryptographic trust validation

#### ARCH-013 — Frontend Trust Rule
The frontend must be treated as a presentation and input layer, not as the final authority for security-sensitive or lifecycle-sensitive decisions.

#### ARCH-014 — Frontend Command Rule
The frontend may request actions from the local supervisory layer, but must not directly bypass it to perform privileged system behavior.

#### ARCH-015 — Frontend Data Handling Rule
The frontend may temporarily hold user-entered values in UI state where required for immediate interaction, but must not intentionally persist secrets or sensitive run-local artifacts outside the approved storage paths.

---

### 8.5 Rust / Tauri Application Shell

### 8.5.1 Purpose
The Rust/Tauri layer is the authoritative local supervisor of the desktop app.

#### ARCH-016 — Shell Authority Rule
The Rust/Tauri application shell is the primary owner of local orchestration responsibilities.

#### ARCH-017 — Shell Primary Responsibilities
The shell must own:
- app lifecycle control
- startup checks
- startup janitor triggering
- activation token validation at app level
- secure access to OS-native credential storage
- runtime hydration coordination
- runtime integrity verification at the shell level
- local process spawn and supervision
- local port allocation and session-token issuance
- controlled shutdown coordination
- force-termination fallback where authorized
- workspace creation coordination
- cleanup triggering coordination
- separation between privileged local operations and UI requests

#### ARCH-018 — Shell Security Responsibility
The shell must act as the privileged broker between:
- frontend input
- local backend execution
- OS-native secure storage
- file-system operations
- local process control

#### ARCH-019 — Shell Non-Ownership Rule
The shell must not own:
- simulation logic
- persona generation logic
- round orchestration logic
- report-content reasoning
- provider-side debate prompts as primary behavioral owner
- payment processing logic

#### ARCH-020 — Shell Enforcement Rule
If the frontend requests an action that violates a constitutional, behavioral, or lifecycle rule, the shell must reject the request rather than passing it through blindly.

#### ARCH-021 — Shell Process Supervision Rule
The shell must remain the authoritative owner of Python engine process supervision.

#### ARCH-022 — Shell Local Trust Boundary Rule
Any action requiring:
- secret retrieval
- process spawn
- process kill
- file deletion
- port discovery
- privileged local state access
must pass through the shell rather than originating from the frontend alone.

---

### 8.6 Python Simulation Engine

### 8.6.1 Purpose
The Python engine exists to execute the simulation-specific intelligence and run-local execution workflow.

#### ARCH-023 — Engine Authority Rule
The Python engine is the primary owner of simulation execution behavior once a run has started.

#### ARCH-024 — Engine Primary Responsibilities
The engine must own:
- Preflight computation execution
- seed-document extraction and ingestion processing
- runtime persona generation
- Moderator-guided round orchestration
- memory-layer management
- provider call scheduling under bounded execution logic
- event emission
- budget tracking within run execution
- duration-tracking participation
- finalization logic
- report-content assembly inputs
- run-state progression within the run/session domain

#### ARCH-025 — Engine Non-Ownership Rule
The engine must not own:
- OS-native secure storage
- app activation authority
- payment ledger behavior
- update distribution authority
- app-close interception authority
- global app lifecycle control
- direct UI rendering
- unrestricted file-system authority beyond its assigned workspace and approved paths

#### ARCH-026 — Engine Locality Rule
The engine must run locally on the user’s machine for simulation execution in MVP.

#### ARCH-027 — Engine Backend Rule
The engine may expose app-internal local endpoints for:
- Preflight
- session start
- status retrieval
- event retrieval
- cancellation
- finalization
- shutdown
but these endpoints must remain bounded by app-internal trust controls.

#### ARCH-028 — Engine Workspace Rule
The engine must operate within the run-local workspace boundaries assigned by the shell and must not treat arbitrary user directories as normal write targets for transient run artifacts.

#### ARCH-029 — Engine Truth Rule
The engine must not fabricate run progression, report content, or transcript history in order to conceal failures or truncation.

---

### 8.7 Cloud Licensing and Distribution Backend

### 8.7.1 Purpose
The cloud backend exists to support commercial and distribution responsibilities, not to perform core run execution.

#### ARCH-030 — Cloud Backend Authority Rule
The cloud backend is the primary owner of:
- commercial entitlement record handling
- device-bound activation issuance
- activation slot enforcement
- runtime distribution authorization
- update-manifest hosting support
- update artifact trust support where applicable

#### ARCH-031 — Cloud Backend Primary Responsibilities
The cloud backend must own:
- payment-webhook handling
- license issuance records
- activation request evaluation
- device hash acceptance and tracking
- device-bound JWT minting
- runtime access authorization
- update metadata serving or update access support

#### ARCH-032 — Cloud Backend Non-Ownership Rule
The cloud backend must not own:
- core simulation execution
- vendor-hosted run transcript persistence
- vendor-hosted seed-document execution pipeline
- run-local memory
- live moderated debate orchestration

#### ARCH-033 — Commercial Isolation Rule
Commercial backend logic must remain isolated from core simulation inference logic.
This preserves the product’s local-data-first architecture and reduces accidental cloud creep.

---

### 8.8 Third-Party LLM Provider Layer

### 8.8.1 Purpose
This layer performs external inference using the user’s own provider account.

#### ARCH-034 — Provider Layer Role
The provider layer is external to the product’s codebase and is accessed through the user’s configured credentials.

#### ARCH-035 — Provider Layer Usage Rule
The product may call provider APIs only through bounded, policy-compliant execution paths defined by the run logic.

#### ARCH-036 — Provider Layer Non-Ownership Rule
The provider layer does not own:
- product lifecycle state
- report truthfulness
- budget policy design
- app security design
- local cleanup behavior

#### ARCH-037 — Provider Dependency Rule
The architecture must explicitly acknowledge that simulation execution depends on provider availability and network connectivity.

---

### 8.9 Payment Provider Layer

### 8.9.1 Purpose
The payment provider exists solely for commercial transaction handling.

#### ARCH-038 — Payment Layer Role
The payment provider layer is responsible for payment transaction handling and event origination only.

#### ARCH-039 — Payment Layer Non-Ownership Rule
The payment provider must not be treated as the owner of:
- activation policy
- local app readiness
- simulation access control state on device
- runtime download authorization policy
except insofar as its event data feeds the cloud backend’s entitlement decisions

---

### 8.10 OS-Native Facilities Layer

### 8.10.1 Purpose
This layer consists of operating-system-provided services and resources the product relies on.

#### ARCH-040 — OS Facilities Scope
This layer includes at minimum:
- secure credential stores
- local file system
- temp directories
- process management primitives
- localhost networking
- keychain / credential-manager integration surfaces
- standard app data / support locations

#### ARCH-041 — OS Dependency Rule
The product architecture must use OS-native facilities where appropriate rather than emulating them poorly in app-managed plaintext storage.

#### ARCH-042 — OS Non-Guarantee Rule
Use of OS-native facilities does not create absolute guarantees against:
- memory inspection
- indexing residue
- swap behavior
- forensic recovery
- third-party local monitoring

These residual risks remain outside the MVP’s absolute guarantee scope.

---

### 8.11 Responsibility Matrix

#### ARCH-043 — Responsibility Matrix Rule
The following responsibilities are owned by these primary components:

- **UI rendering and user interaction** -> Frontend
- **privileged local orchestration** -> Rust/Tauri Shell
- **secure local key retrieval mediation** -> Rust/Tauri Shell
- **simulation execution** -> Python Engine
- **persona generation** -> Python Engine
- **round control** -> Python Engine
- **finalization and report-content synthesis inputs** -> Python Engine
- **activation issuance** -> Cloud Backend
- **payment handling** -> Payment Provider + Cloud Backend integration
- **runtime package authorization** -> Cloud Backend
- **runtime package installation coordination** -> Rust/Tauri Shell
- **update discovery / verification flow ownership in app** -> Rust/Tauri Shell
- **third-party inference execution** -> Provider Layer under Python Engine control
- **OS-native secret persistence** -> OS Facilities via Rust/Tauri Shell

#### ARCH-044 — No Split-Authority Ambiguity Rule
If ownership is ambiguous during implementation, the tie must be resolved in favor of the most conservative reading of this section until explicitly revised.

---

### 8.12 Communication Boundaries

#### ARCH-045 — Frontend-to-Shell Boundary
Frontend-to-shell communication must be explicit, bounded, and command-based.

#### ARCH-046 — Shell-to-Engine Boundary
Shell-to-engine communication must be explicit, authenticated within the app’s local trust model, and bounded to the allowed local interface contract.

#### ARCH-047 — Engine-to-Provider Boundary
Engine-to-provider communication must occur only through the configured provider/model pathway and under bounded run policy.

#### ARCH-048 — App-to-Cloud Boundary
The app may communicate with the cloud backend only for:
- activation
- entitlement-related operations
- runtime distribution
- update-related operations
- similar commercial/support functions explicitly allowed by the product scope

#### ARCH-049 — Forbidden Boundary Expansion Rule
The app-to-cloud boundary must not silently expand into vendor-hosted simulation execution or persistent vendor-side run storage in MVP.

---

### 8.13 Localhost Service Boundary

#### ARCH-050 — Localhost Restriction Rule
The engine’s app-internal backend must bind only to localhost under the app’s trust model.

#### ARCH-051 — Localhost Access Rule
The app must treat localhost access as privileged within the app architecture and must not assume localhost is universally safe just because it is local.

#### ARCH-052 — App Session Token Rule
Requests from the app frontend to the local backend must be mediated through the app’s local authorization model rather than assuming any local caller is trusted.

#### ARCH-053 — No Public API Assumption Rule
The local backend is not a public developer platform API and must not be architected as one for MVP.

---

### 8.14 Process Model

#### ARCH-054 — Process Model Rule
At minimum, the app architecture includes:
- a desktop shell process
- a frontend UI runtime inside the shell environment
- a local Python engine process for simulation execution

#### ARCH-055 — Parent Supervision Rule
The shell must remain the supervising parent authority for the local engine process lifecycle.

#### ARCH-056 — No Detached-Normal-Mode Rule
The architecture must not intentionally rely on detached uncontrolled engine execution as the normal operating model.

#### ARCH-057 — Controlled Shutdown Rule
The shell must attempt graceful engine shutdown first, then forceful termination only when required by allowed shutdown policy.

---

### 8.15 Runtime Distribution Architecture

#### ARCH-058 — Thin Installer Rule
The base app installer must not be required to contain the entire heavy simulation runtime if runtime hydration is part of the approved architecture.

#### ARCH-059 — Runtime Hydration Architecture Rule
The heavy runtime package must be hydrated after install through a shell-owned managed runtime-package distribution path that is authorized by the commercial backend and verified before local use.

#### ARCH-060 — Runtime Integrity Verification Rule
The architecture must include runtime integrity verification before simulation features are treated as available.

#### ARCH-061 — Distribution Separation Rule
Runtime distribution support must remain architecturally separate from both the native app-updater path and run execution logic.

---

### 8.16 Architecture Integrity Prohibitions

#### ARCH-062 — No Frontend Privilege Escalation Rule
The frontend must not directly gain authority over secure storage, process management, or destructive cleanup operations.

#### ARCH-063 — No Engine Commercial Drift Rule
The Python engine must not be turned into the commercial authority for entitlement, activation, or payment state.

#### ARCH-064 — No Cloud Execution Drift Rule
The cloud backend must not silently become the primary simulation execution environment in MVP.

#### ARCH-065 — No Responsibility Duplication Rule
Multiple layers must not independently implement conflicting versions of:
- run truth
- budget truth
- activation truth
- cleanup truth
- report truth

#### ARCH-066 — No Hidden Persistence Layer Rule
The architecture must not introduce an undeclared persistence layer for seed documents, transcripts, or run memory outside the approved storage policy.

---

### 8.17 Architecture Summary

#### ARCH-067 — Section Summary Rule
At the system-architecture level, the product is defined as a desktop-supervised local execution system in which:
- the frontend handles presentation and user interaction
- the Rust/Tauri shell owns privileged local orchestration
- the Python engine owns simulation execution
- the cloud backend owns licensing and distribution support
- third-party providers supply inference through the user’s own key
- OS-native facilities provide secure storage and local execution primitives

#### ARCH-068 — Architecture Consistency Rule
All later API, database, UI, and non-functional sections must remain consistent with the ownership boundaries defined here.

#### ARCH-069 — Section Boundary Rule
This section defines:
- major system components
- ownership boundaries
- communication boundaries
- process boundaries
- architecture prohibitions

This section does not yet define:
- exact data retention rules by data type
- exact endpoint payload schemas
- exact database fields
- exact UI controls
- exact acceptance tests

Those are governed in later sections.

---

## 9. Data Lifecycle and Storage Policy

### 9.1 Section Purpose
This section defines where data may exist, how long it may exist, what may persist, what must remain transient, and what the product is allowed to claim about data handling.

This section exists to ensure that:
- data behavior remains consistent with the product’s privacy posture
- AI coding agents do not introduce undeclared persistence
- local storage behavior remains bounded and auditable
- secret handling remains separated from ordinary app data
- cleanup obligations are clear
- residual risk disclosures stay honest

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- Section 6 — Product Behavior Specification
- Section 7 — State Machines
- Section 8 — System Architecture

If any lower-level implementation detail conflicts with this section, this section wins unless Document A is versioned forward.

---

### 9.2 Data Policy Philosophy

#### DATA-001 — Local-Data-First Policy
The product must handle run execution data according to a local-data-first policy.

#### DATA-002 — Best-Effort Ephemeral Policy
Run-local execution artifacts must be treated as transient and subject to best-effort cleanup rather than intentional long-term app persistence.

#### DATA-003 — Secret Separation Policy
Secrets and entitlement tokens must not be stored using the same data-handling model as ordinary run-local artifacts.

#### DATA-004 — Honest Persistence Policy
The product must be explicit about which artifacts may persist intentionally and which are expected to be transient.

#### DATA-005 — No Hidden Data Lake Policy
The product must not create an undeclared vendor-side or app-side persistent data lake of:
- seed documents
- transcripts
- run memory
- generated debate history

for MVP.

---

### 9.3 Data Classification Model

#### DATA-006 — Data Classification Rule
For policy purposes, the product must treat data in at least the following categories:

1. commercial entitlement data  
2. activation/device-binding data  
3. provider secret data  
4. user-supplied seed document data  
5. extracted seed text and run-derived working data  
6. runtime package data  
7. run-local workspace data  
8. final exported report data  
9. diagnostic/logging data  
10. update/distribution metadata  
11. transient in-memory operational state

#### DATA-007 — Category-Specific Policy Rule
Each data category must have an explicitly defined persistence, storage, and cleanup policy.

---

### 9.4 Commercial Entitlement Data

#### DATA-008 — Entitlement Data Definition
Commercial entitlement data includes at minimum:
- purchaser identity or license identity
- plan/tier
- entitlement status
- expiration data where applicable
- payment-linked commercial records

#### DATA-009 — Entitlement Storage Authority
Commercial entitlement data is primarily owned by the cloud commercial backend.

#### DATA-010 — Local Entitlement Usage Rule
The local app may hold only the minimum local entitlement representation required for valid post-activation app use, such as the device-bound JWT and associated validation context.

#### DATA-011 — No Local Ledger Rule
The app must not treat itself as the primary commercial ledger of record for payments or full entitlement history.

---

### 9.5 Activation and Device-Binding Data

#### DATA-012 — Activation Data Definition
Activation data includes at minimum:
- activation status
- device-bound token
- device hash or device-binding representation
- activation slot relationship
- token validation metadata as needed

#### DATA-013 — Activation Storage Split Rule
Activation data is split between:
- authoritative backend activation records
- minimal local activation state required for offline validation

#### DATA-014 — Device Hash Policy
The canonical backend identity for device binding should use a hash or similarly privacy-conscious identifier rather than raw hardware identity as the normal durable representation.

#### DATA-015 — Local Activation Storage Rule
The local device-bound JWT may persist locally because post-activation offline validation is an approved product behavior.

#### DATA-016 — Activation Token Handling Rule
Activation tokens must not be stored inside ordinary transient run workspaces.

---

### 9.6 Provider Secret Data

#### DATA-017 — Provider Secret Definition
Provider secret data includes:
- OpenAI API key
- Google API key
- equivalent future supported provider credentials if later approved

#### DATA-018 — Secret Storage Rule
Provider secrets must be stored only in approved OS-native secure storage facilities.

#### DATA-019 — No Plaintext Secret Persistence Rule
Provider secrets must not be intentionally persisted in:
- plaintext config files
- ordinary app JSON state
- app-managed SQLite as normal plaintext
- temporary workspaces
- diagnostic logs
- report files
- transcript archives

#### DATA-020 — Secret Runtime Access Rule
Secrets may be retrieved into process memory only when necessary for allowed execution behavior.

#### DATA-021 — Secret Exposure Minimization Rule
The architecture should minimize the time and surface area in which provider secrets exist in app-controlled memory, even though absolute memory secrecy is not guaranteed in MVP.

#### DATA-022 — Secret Scope Rule
Provider secret data must never be embedded into final user-facing outputs such as the report.

---

### 9.7 Seed Document Data

#### DATA-023 — Seed Document Definition
The seed document is the single uploaded English file used as the source context for one run.

#### DATA-024 — Seed Document Local Intake Rule
The seed document may be read locally by the app for Preflight and run execution preparation.

#### DATA-025 — Seed Document Vendor-Cloud Storage Prohibition
The product must not intentionally persist the seed document into a vendor-hosted simulation database in MVP.

#### DATA-026 — Seed Document Run Scope Rule
The seed document belongs to one run scope at a time and must not become undeclared reusable run memory for later runs.

#### DATA-027 — Seed Document Persistence Boundary
The app may temporarily access the selected file from its original local path or a controlled copied/processed location as needed for the run, but run-local copies and derivatives must follow transient workspace policy unless otherwise explicitly allowed.

#### DATA-028 — Non-English Seed Rejection Rule
A non-English seed document may be inspected only far enough to determine rejection under Preflight policy.
It must not proceed into full simulation execution.

---

### 9.8 Extracted Text and Run-Derived Working Data

#### DATA-029 — Working Data Definition
Working data includes at minimum:
- extracted text
- chunked text representations
- retrieval indexes
- vector store data
- temporary database files
- round summaries
- argument memory
- persona generation artifacts
- finalization intermediates

#### DATA-030 — Working Data Ephemeral Rule
Working data is transient run-local data and must be stored only within approved transient run boundaries unless explicitly allowed elsewhere in this document.

#### DATA-031 — No Cross-Run Working Data Reuse Rule
Working data from one run must not become active semantic input for another run.

#### DATA-032 — Working Data Persistence Prohibition
The app must not intentionally preserve run working data as a durable long-term library of prior runs in MVP.

#### DATA-033 — Working Data Cloud Prohibition
Working data must not be intentionally synced to a vendor-hosted persistent simulation store in MVP.

---

### 9.9 Runtime Package Data

#### DATA-034 — Runtime Package Definition
Runtime package data includes the heavy locally installed simulation runtime and any approved support files required for it to operate.

#### DATA-035 — Runtime Package Persistence Rule
Unlike transient run-local artifacts, the runtime package is an approved persistent local installation artifact.

#### DATA-036 — Runtime Package Storage Location Rule
The runtime package must be stored in the approved app-controlled support location, not in the ephemeral run workspace.

#### DATA-037 — Runtime Package Separation Rule
Persistent runtime installation data must remain separate from transient run-local execution data.

#### DATA-038 — Runtime Package Integrity Rule
Persistent runtime package data must be subject to integrity checking before use.

---

### 9.10 Run-Local Workspace Data

#### DATA-039 — Workspace Data Definition
Workspace data includes all transient run-local artifacts created for one specific run inside the assigned workspace boundary.

#### DATA-040 — Workspace Isolation Rule
Each run must have one isolated workspace.

#### DATA-041 — Workspace Separation Rule
Workspace data from different runs must not be co-mingled.

#### DATA-042 — Workspace Persistence Rule
Workspace data is transient by design and must be subject to cleanup after terminal run behavior under best-effort policy.

#### DATA-043 — Workspace Allowed Contents Rule
The workspace may contain only approved transient run artifacts, such as:
- extracted text intermediates
- temporary storage/index files
- temporary report-assembly intermediates
- local event or run-control artifacts required for execution
- other approved transient run data

#### DATA-044 — Workspace Forbidden Contents Rule
The workspace must not intentionally hold durable copies of:
- provider secrets
- activation tokens as normal storage
- long-term app preferences
- payment ledger data
- multi-run persistent knowledge base data

#### DATA-045 — Workspace Cleanup Rule
After run termination, the workspace must be targeted for cleanup according to lifecycle policy.

#### DATA-046 — Janitor Recovery Rule
If normal cleanup does not occur because of abnormal termination, the startup janitor must attempt cleanup of stale workspace data on next launch.

---

### 9.11 Final Exported Report Data

#### DATA-047 — Report Data Definition
Final exported report data means the structured PDF artifact intentionally produced as a durable user-facing output.

#### DATA-048 — Report Persistence Allowance
The final exported report is allowed to persist locally after the run ends.

#### DATA-049 — Report Boundary Rule
The report must persist outside the transient workspace if the product expects the user to retain it after cleanup.

#### DATA-050 — Report Content Rule
The report may contain:
- structured conclusions
- token/cost summary
- transcript appendix
- other approved report sections
but must not contain secrets such as provider API keys.

#### DATA-051 — Report Cleanup Distinction Rule
Workspace cleanup must not be treated as a mandate to delete the intentionally exported final report.

#### DATA-052 — Local Viewing Rule
Because report persistence is approved, local report viewing is allowed as an offline-capable behavior after generation.

---

### 9.12 Diagnostic and Logging Data

#### DATA-053 — Diagnostic Data Definition
Diagnostic data includes:
- startup failures
- hydration failures
- updater failures
- cleanup failures
- process supervision failures
- non-sensitive run control diagnostics
- other operational debugging metadata approved for MVP

#### DATA-054 — Diagnostic Minimization Rule
Diagnostic data must be minimized to what is necessary for supportability and debugging.

#### DATA-055 — Forbidden Diagnostic Data Rule
Production diagnostics must not intentionally store:
- provider API keys
- raw device-bound JWT strings unless explicitly protected and justified
- full seed document content
- full transcript content as routine logs
- raw long-form extracted working data as routine logs

#### DATA-056 — Sensitive Logging Caution Rule
If a diagnostic event refers to a sensitive artifact, the product should log only the minimum identifying metadata necessary for troubleshooting.

#### DATA-057 — Cleanup Failure Logging Rule
Cleanup failures may be logged because they are operationally significant, but such logs must still avoid unnecessary sensitive content.

---

### 9.13 Update and Distribution Metadata

#### DATA-058 — Update Metadata Definition
Update and distribution metadata includes:
- version metadata
- manifest data
- runtime package version information
- integrity/signature metadata
- distribution status indicators

#### DATA-059 — Update Metadata Persistence Rule
Update metadata may persist locally where required for safe update behavior and runtime integrity verification.

#### DATA-060 — Update Metadata Non-Sensitivity Rule
Update metadata is not equivalent to run-local simulation data and may be handled separately under the updater architecture.

---

### 9.14 In-Memory Operational Data

#### DATA-061 — In-Memory Data Definition
In-memory operational data includes:
- active UI state
- active run state
- active provider secret usage
- event buffering
- temporary process-local summaries
- live round artifacts not yet written to final outputs

#### DATA-062 — In-Memory Necessity Rule
The product may use in-memory data as required for correct operation.

#### DATA-063 — In-Memory Non-Guarantee Rule
The product must acknowledge that in-memory handling does not guarantee protection against hostile local memory inspection in MVP.

#### DATA-064 — In-Memory Scope Minimization Rule
The system should avoid holding more sensitive or historical material in memory than is necessary for correct bounded execution.

---

### 9.15 Storage Locations Policy

#### DATA-065 — Approved Storage Location Rule
Data must be stored only in storage classes appropriate to its category.

#### DATA-066 — Secret Location Rule
Provider secrets must live in OS-native secure storage, not ordinary app data directories.

#### DATA-067 — Runtime Location Rule
Persistent runtime package data must live in app-controlled support/install locations.

#### DATA-068 — Workspace Location Rule
Transient run-local data must live in the approved temporary or temp-compatible workspace boundary.

#### DATA-069 — Report Location Rule
Final exported reports must be written to an intentional durable location outside the transient workspace if the user is expected to keep them.

#### DATA-070 — No Arbitrary File Sprawl Rule
The app must not scatter sensitive or transient run artifacts across arbitrary user folders without explicit approved reason.

---

### 9.16 Persistence Matrix

#### DATA-071 — Persistence Matrix Rule
The following persistence expectations are constitutionally approved:

- **provider API keys** -> persistent locally in OS-native secure storage
- **device-bound activation token** -> persistent locally for offline validation
- **runtime package** -> persistent locally
- **transient run workspace** -> non-persistent by policy; cleanup targeted
- **seed-document working copies** -> transient by policy if copied into run workspace
- **run memory / vector/index artifacts** -> transient by policy
- **final exported report** -> persistent locally by user-facing design
- **diagnostic logs** -> persistent only in minimized approved form
- **payment and license ledger** -> persistent in cloud commercial backend
- **vendor-hosted simulation history** -> prohibited in MVP

#### DATA-072 — No Persistence Assumption Rule
If a data category is not explicitly approved for persistence, it must not be assumed persistable by default.

---

### 9.17 Cleanup and Deletion Policy

#### DATA-073 — Cleanup Policy Rule
Cleanup applies to transient run-local artifacts, not to all local app data indiscriminately.

#### DATA-074 — Cleanup Scope Rule
Cleanup must target:
- run workspace
- run-local intermediates
- run-local working data
- other transient artifacts defined by execution policy

#### DATA-075 — Cleanup Exclusion Rule
Cleanup must not target:
- persistent runtime package files
- valid activation tokens
- OS-native stored provider keys
- intentionally exported final reports
- legitimate app installation files
unless explicit repair/uninstall flows say otherwise

#### DATA-076 — Normal Cleanup Rule
On normal terminal run handling, cleanup must be attempted promptly after the run enters its cleanup phase.

#### DATA-077 — Abnormal-Termination Cleanup Rule
If normal cleanup is skipped by abnormal termination, janitor cleanup on next launch must attempt stale artifact removal.

#### DATA-078 — Best-Effort Deletion Rule
Cleanup is best-effort deletion of approved transient artifacts, not a guarantee of forensic-grade eradication across all system layers.

#### DATA-079 — Cleanup Failure Honesty Rule
If cleanup fails, the product must preserve truthful state and must not imply that transient data definitely vanished.

---

### 9.18 Data-Handling Prohibitions

#### DATA-080 — No Vendor Simulation Archive Rule
The product must not intentionally build a vendor-hosted archive of user seed documents, transcripts, or run memory in MVP.

#### DATA-081 — No Secret-in-Report Rule
The product must not place provider secrets into reports, transcripts, or ordinary logs.

#### DATA-082 — No Secret-in-Workspace Rule
The product must not intentionally persist provider secrets inside transient run workspaces.

#### DATA-083 — No Cross-Run Memory Leakage Rule
The product must not persist run working memory in a way that later runs silently inherit it as active semantic context.

#### DATA-084 — No Hidden Durable Transcript Store Rule
The product must not silently retain full transcript history in an undeclared durable local or cloud store beyond approved report output and minimized logging policy.

#### DATA-085 — No False Deletion Claim Rule
The product must not claim that all traces are guaranteed erased after cleanup.

#### DATA-086 — No Overcollection Rule
The product must not collect or persist more data than is materially necessary for:
- licensing
- activation
- run execution
- report generation
- debugging within approved limits
- update/distribution support

---

### 9.19 Residual Risk Disclosure Policy

#### DATA-087 — Residual Risk Disclosure Rule
The data policy must explicitly acknowledge residual risks outside the MVP’s guarantee scope.

#### DATA-088 — Residual Local Risk Examples
Residual risks include, at minimum:
- OS indexing side effects
- memory inspection
- swap/pagefile residue
- filesystem journaling effects
- SSD and storage-layer secure-erasure limitations
- third-party backup or monitoring software behavior
- forced termination before cleanup

#### DATA-089 — Disclosure Honesty Rule
These residual risks must not be hidden behind absolute privacy or deletion marketing language.

#### DATA-090 — MVP Scope Limitation Rule
The existence of residual risk does not invalidate the MVP, but it does constrain what the product may honestly claim.

---

### 9.20 Data Governance Integrity Rules

#### DATA-091 — Category Integrity Rule
Each data item must be handled according to its category rather than convenience.

#### DATA-092 — Storage Integrity Rule
A sensitive category must not be downgraded into a weaker storage policy simply because doing so is easier to implement.

#### DATA-093 — Lifecycle Integrity Rule
Data retention, cleanup, and persistence behavior must remain consistent with lifecycle state truth.

#### DATA-094 — Cross-Document Integrity Rule
Later API, database, UI, and execution documents must not introduce storage behavior that violates this section.

#### DATA-095 — AI Implementation Integrity Rule
AI coding agents must not invent undeclared persistence layers, local caches, transcript stores, or secret mirroring behavior.

---

### 9.21 Data Policy Summary

#### DATA-096 — Section Summary Rule
At the data-governance level, the product is defined as follows:

Commercial and activation data may persist under the approved license model. Provider secrets may persist only in OS-native secure storage. The heavy runtime may persist as an installed support artifact. Run-local execution data, including extracted text, memory artifacts, vector/index files, and transcript-generation intermediates, must be treated as transient workspace data subject to best-effort cleanup. The final exported report may persist locally as an intentional user-facing artifact. Diagnostics may persist only in minimized approved form. The product must not intentionally create a vendor-hosted simulation archive or claim forensic-grade deletion guarantees.

#### DATA-097 — Data Consistency Rule
All later sections must remain consistent with the storage and cleanup boundaries defined here.

#### DATA-098 — Section Boundary Rule
This section defines:
- data categories
- persistence rules
- storage locations
- cleanup scope
- residual risk policy
- data-handling prohibitions

This section does not yet define:
- exact endpoint schemas
- exact database DDL
- exact UI controls
- exact acceptance-test phrasing

Those are governed in later sections.

---

## 10. API and Interface Contract

### 10.1 Section Purpose
This section defines the contract between major system layers at the interface level.

This section exists to ensure that:
- the local backend is treated as a bounded app-internal service
- AI coding agents do not invent extra endpoints or hidden control paths
- request/response expectations are explicit before implementation
- authentication, sequencing, and error behavior are deterministic
- run state and event behavior remain interface-consistent with earlier behavioral and state-machine rules

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- Section 6 — Product Behavior Specification
- Section 7 — State Machines
- Section 8 — System Architecture
- Section 9 — Data Lifecycle and Storage Policy

If any lower-level interface artifact conflicts with this section, this section wins unless Document A is versioned forward.

---

### 10.2 Interface Philosophy

#### API-001 — Interface Contract Rule
All interfaces defined in this section are authoritative product-contract interfaces, not optional implementation suggestions.

#### API-002 — Local Interface Scope Rule
The primary interface contract in MVP is the app-internal local backend interface between:
- frontend UI
- Rust/Tauri shell
- Python simulation engine

#### API-003 — No Public Platform API Rule
The local backend interface is not a public third-party platform API and must not be architected as an open developer surface in MVP.

#### API-004 — Truthfulness Rule
An interface response must reflect real system truth and must not:
- overstate readiness
- understate failure
- conceal truncation
- fabricate progress
- mislabel run state

#### API-005 — Boundedness Rule
The interface contract must remain minimal, explicit, and bounded to the product’s approved flows.
It must not become a generic task orchestration surface.

---

### 10.3 Canonical Interface Layers

#### API-006 — Interface Layer Map
The product contains at minimum the following interface classes:

1. **Frontend-to-Shell privileged command interface**
2. **Shell-to-Engine app-internal local HTTP interface**
3. **Engine-to-Frontend event/status interface**
4. **App-to-Cloud commercial interface**
5. **Cloud-to-Payment webhook interface**
6. **App-to-OS secure storage and local-system interfaces**

#### API-007 — Layer Boundary Rule
Each interface class exists for a specific purpose and must not silently absorb the duties of another interface class.

---

### 10.4 Frontend-to-Shell Command Contract

### 10.4.1 Purpose
This interface class governs privileged local commands issued by the frontend to the shell.

#### API-008 — Frontend Command Scope
The frontend may request privileged actions from the shell only through explicitly approved commands.

#### API-009 — Shell Command Ownership
The shell is the authoritative owner of commands involving:
- secure storage
- activation-token handling
- runtime hydration coordination
- process supervision
- workspace coordination
- cleanup triggering
- app-close orchestration
- update-check invocation where applicable

#### API-010 — No Direct Privileged Frontend Rule
The frontend must not directly perform or simulate privileged operations that belong to the shell.

#### API-011 — Payload Validation Rule
All frontend-to-shell command payloads must be validated for:
- required fields
- type correctness
- enum validity
- size bounds
- file/path safety where relevant

#### API-012 — Rejection Rule
If the frontend requests a prohibited, malformed, or out-of-state action, the shell must reject it explicitly rather than attempting undefined behavior.

---

### 10.5 Shell-to-Engine Local Backend Contract

### 10.5.1 Purpose
This interface class governs the app-internal local backend routes exposed by the Python engine.

#### API-013 — Engine Interface Scope
The engine must expose only the routes necessary to support:
- Preflight
- run creation/start
- run status retrieval
- ordered event retrieval
- controlled cancellation
- controlled finalization
- controlled shutdown

#### API-014 — Canonical Route Set
The canonical local backend route set for MVP is:

- `POST /sessions/preflight`
- `POST /sessions`
- `POST /sessions/{id}/start`
- `GET /sessions/{id}/status`
- `GET /sessions/{id}/events`
- `POST /sessions/{id}/cancel`
- `POST /sessions/{id}/finalize`
- `POST /shutdown`

No additional route should be treated as product-authoritative unless Document A is revised.

#### API-015 — No Undeclared Route Rule
Implementation must not depend on hidden undeclared production routes for normal app operation.

#### API-016 — Route Stability Rule
Canonical route purpose must remain stable across implementation revisions unless explicitly revised in Document A.

---

### 10.6 Canonical Route Intent

#### API-017 — `POST /sessions/preflight` Intent
This route exists to validate the pending run request and return:
- approval
- approval-with-downgrade
- or rejection

It must not start the active run.

#### API-018 — `POST /sessions` Intent
This route exists to create the run/session context after a valid start-eligible Preflight outcome.
It must not silently bypass Preflight.

#### API-019 — `POST /sessions/{id}/start` Intent
This route exists to begin active run execution for an already created session under an allowed effective plan.

#### API-020 — `GET /sessions/{id}/status` Intent
This route exists to return the current truthful state of the run/session, including progress-relevant data.

#### API-021 — `GET /sessions/{id}/events` Intent
This route exists to return ordered event history after a specified sequence boundary for incremental UI synchronization.

#### API-022 — `POST /sessions/{id}/cancel` Intent
This route exists to trigger controlled cancellation of a cancelable run.

#### API-023 — `POST /sessions/{id}/finalize` Intent
This route exists to trigger or confirm controlled finalization where a valid finalize path exists.

#### API-024 — `POST /shutdown` Intent
This route exists to support controlled engine shutdown initiated by the supervising shell.

---

### 10.7 Authentication and Trust Requirements

#### API-025 — Local Backend Auth Rule
The local backend must not assume that localhost traffic is inherently trusted.

#### API-026 — App Session Token Rule
Requests from the app frontend or shell to the local backend must use the app-session authorization model defined by the product architecture.

#### API-027 — Auth Scope Rule
The app-session token exists to prove that the caller belongs to the current authorized local app context.

#### API-028 — Missing Token Rejection Rule
A request that requires an app-session token and lacks a valid one must be rejected.

#### API-029 — Invalid Token Rejection Rule
A request with an invalid, expired, or mismatched app-session token must be rejected.

#### API-030 — No Anonymous Control Rule
State-changing local backend routes must not be callable anonymously.

#### API-031 — Token Boundary Rule
The app-session token is not a replacement for provider credentials or license tokens.
It is a local trust-boundary control for app-to-engine communication.

---

### 10.8 Request and Response Contract Rules

#### API-032 — Structured Request Rule
Every route must accept a structured request shape appropriate to its purpose and state.

#### API-033 — Structured Response Rule
Every route must return a structured response shape appropriate to its purpose and state.

#### API-034 — Deterministic Response Rule
A route response must deterministically reflect the canonical run, Preflight, or lifecycle truth at the moment the route completes.

#### API-035 — No Ambiguous Success Rule
A response must not imply success when the underlying operation actually failed or remains ineligible.

#### API-036 — Response Stability Rule
The meaning of key response fields such as:
- state
- status
- effective plan
- budget state
- report readiness
must remain stable across calls.

#### API-037 — No Hidden Mutation Rule
A read-oriented route must not perform hidden destructive state changes unless explicitly documented and authorized.

---

### 10.9 Preflight Route Contract

#### API-038 — Preflight Input Contract
The Preflight request must include at minimum:
- provider selection
- model if applicable in MVP UI
- requested agent count
- requested round count
- max token budget
- selected seed document payload or approved document reference

#### API-039 — Preflight Output Contract
The Preflight response must include at minimum:
- canonical Preflight outcome
- requested plan summary
- effective plan summary
- estimated token and duration information
- downgrade warning if applicable
- rejection reason if applicable
- budget feasibility truth
- provider/model context used for planning

#### API-040 — Preflight Non-Execution Rule
The Preflight route must not begin active debate execution.

#### API-041 — Preflight Repeatability Rule
Preflight may be re-run when the pending configuration changes, but prior results must be invalidated when required by the state rules.

---

### 10.10 Session Creation and Start Contract

#### API-042 — Session Creation Rule
A session may be created only from a current run-start-eligible Preflight result.

#### API-043 — Session Creation Output Rule
Session creation must return at minimum:
- session identity
- initial run state
- effective plan identity
- timing metadata sufficient for later tracking

#### API-044 — Start Route Preconditions
The start route must require:
- an existing session
- a valid start-eligible Preflight lineage
- current local readiness
- accepted downgrade state if applicable

#### API-045 — No Start Without Session Rule
The engine must not treat Preflight approval alone as implicit run start without a created session context.

#### API-046 — Start Response Rule
The start response must confirm that the session has actually moved into active execution entry behavior rather than merely acknowledging a button click.

---

### 10.11 Status Route Contract

#### API-047 — Status Truth Rule
The status route must return the canonical current run/session truth.

#### API-048 — Status Minimum Content Rule
The status response must include at minimum:
- session identity
- canonical run state
- effective plan
- current round indicators as applicable
- budget state
- elapsed time
- provider-health or degradation posture at a meaningful level
- report readiness state
- latest event sequencing marker

#### API-049 — Status Readability Rule
The status response must be suitable both for UI presentation logic and for debugging/lifecycle interpretation.

#### API-050 — No False Ready Rule
The status response must not mark the report as ready if report generation has not actually succeeded.

#### API-051 — No Silent Degradation Rule
If the run is paused, degraded, retrying, or finalizing, the status route must reflect that rather than pretending normal healthy progression.

---

### 10.12 Events Route Contract

#### API-052 — Ordered Events Rule
The events route must return session events in deterministic sequence order.

#### API-053 — Sequence-Bound Retrieval Rule
The events route must support retrieval after a known sequence boundary rather than forcing the UI to re-fetch the full event history every time.

#### API-054 — Event Continuity Rule
Sequence numbering must support continuity and loss detection.
The UI must be able to determine whether it is up to date or needs more events.

#### API-055 — Event Payload Integrity Rule
Every event payload must correspond to a real run occurrence.
Event emission must not invent debate content or progress.

#### API-056 — Event Minimum Envelope Rule
Each event must include at minimum:
- session identifier
- sequence number
- timestamp
- event type
- event payload

#### API-057 — Event Type Stability Rule
Core event types must remain stable once defined in the engineering annex unless Document A is revised.

#### API-058 — Event/Status Consistency Rule
The event stream and status route must remain semantically consistent.
One must not claim a different run truth than the other.

---

### 10.13 Cancellation and Finalization Contract

#### API-059 — Cancellation Route Rule
The cancel route must only succeed when the run is in a cancelable state.

#### API-060 — Cancellation Response Rule
A successful cancellation response must reflect that the run has entered or is entering its controlled cancellation path.

#### API-061 — Finalization Route Rule
The finalize route must only be accepted when a valid finalization path exists under the product behavior.

#### API-062 — No Fake Finalization Rule
The finalize route must not be used to force a false success state when the run lacks meaningful material for valid output.

#### API-063 — Finalization Response Rule
A finalization response must indicate whether the run has:
- entered finalization
- completed
- completed under budget cap
- or failed terminally

#### API-064 — Cancel-vs-Finalize Distinction Rule
Cancellation and finalization are behaviorally distinct and must not be collapsed into one control path.

---

### 10.14 Shutdown Contract

#### API-065 — Shutdown Ownership Rule
Shutdown is a shell-supervised control action exposed to the engine through the shutdown route.

#### API-066 — Shutdown Purpose Rule
The shutdown route exists to support graceful engine termination before forceful fallback is used.

#### API-067 — Shutdown Non-Run-Truth Rule
A successful acknowledgment of shutdown initiation does not by itself mean:
- cleanup has succeeded
- the run completed successfully
- the app has fully closed

#### API-068 — Shutdown Bound Rule
The shutdown route must not be treated as a general-purpose administrative interface beyond its approved lifecycle role.

---

### 10.15 Error Contract

#### API-069 — Structured Error Rule
All interface failures must be returned in a structured error form rather than as vague or opaque failures when structured classification is possible.

#### API-070 — Error Minimum Content Rule
A structured error must include at minimum:
- machine-readable error code
- human-readable message
- retryability posture where meaningful
- timestamp or equivalent traceability marker

#### API-071 — Error Classification Rule
Errors should distinguish at minimum between:
- validation failures
- authorization failures
- state-transition violations
- unsupported file/input failures
- provider/network failures
- runtime integrity failures
- terminal internal failures

#### API-072 — Rejection vs Error Rule
A deterministic business/policy rejection must not be mislabeled as a technical error if the distinction is known.

#### API-073 — Retryable vs Terminal Rule
The contract should distinguish retryable problems from terminal ones where meaningful to the user or shell.

#### API-074 — No Generic Catch-All Misuse Rule
A generic internal error code must not be used in place of a more precise known error category when that category is available.

---

### 10.16 File Upload and Seed Document Interface Rules

#### API-075 — Single Seed Input Rule
The run input interface must support exactly one seed document per run.

#### API-076 — Supported File Type Rule
The interface must enforce the approved MVP seed-document file types: PDF, DOCX, TXT, and Markdown.

#### API-077 — File Size Enforcement Rule
The interface must enforce the approved maximum file size of 10 MiB (10,485,760 bytes) for MVP rather than allowing arbitrarily large seed documents into normal run flow.

#### API-078 — File Metadata Rule
At minimum, the interface must carry enough file metadata to evaluate:
- filename
- file type / MIME class
- size
- content or content reference
for Preflight processing.

#### API-079 — File Acceptance Non-Promise Rule
Accepting a file into pending run configuration does not mean the file is valid for execution.
Final validity is determined by Preflight.

#### API-080 — Non-English Handling Rule
The interface must allow Preflight to return explicit non-English rejection when English-confidence is below 0.80 rather than forcing a generic upload failure.

---

### 10.17 Report Access Interface Rules

#### API-081 — Report Availability Rule
Once report generation succeeds, the interface layer must provide a truthful way for the UI to know that the report is available.

#### API-082 — Report Location Rule
The interface must distinguish between:
- report generated successfully
- report accessible locally
- report unavailable due to generation or file error

#### API-083 — No Phantom Report Rule
The interface must not expose a report-ready state if the file does not exist or was not successfully generated.

#### API-084 — Report Persistence Distinction Rule
The interface contract must preserve the distinction between transient workspace cleanup and durable report availability.

---

### 10.18 App-to-Cloud Commercial Interface Rules

#### API-085 — Cloud Interface Scope Rule
The app may interact with the commercial backend only for approved commercial/support functions, including:
- activation
- entitlement validation where required
- runtime distribution authorization
- update support
- similar approved non-simulation functions

#### API-086 — No Simulation Offload Rule
The app-to-cloud interface must not silently carry core simulation execution payloads as a normal MVP behavior.

#### API-087 — Activation Interface Rule
The activation interface must support:
- device identity submission in approved form
- entitlement evaluation
- device-bound token issuance
- clear denial/error distinction

#### API-088 — Runtime Distribution Interface Rule
The runtime distribution interface must support:
- authorized access request
- bounded download authorization
- integrity-verifiable distribution flow
- version-record and trust-manifest aware verification inputs where required

#### API-089 — Update Interface Rule
The update interface must support:
- version discovery
- artifact trust information
- integrity-verifiable update retrieval through the approved native app-updater path

---

### 10.19 Interface Integrity Prohibitions

#### API-090 — No Undeclared Backdoor Route Rule
The product must not depend on hidden backdoor routes that bypass lifecycle, auth, or policy constraints.

#### API-091 — No Privilege Bypass Rule
The interface design must not let the frontend bypass the shell to perform shell-owned privileged actions.

#### API-092 — No Auth-Free State Mutation Rule
State-changing local backend operations must not be callable without the approved local trust mechanism.

#### API-093 — No Event Fabrication Rule
The interface contract must not allow fabricated event data to stand in for real run history.

#### API-094 — No State Drift Between Interfaces Rule
Different routes must not describe mutually contradictory truths about the same run/session.

#### API-095 — No Public API Creep Rule
The local backend interface must not evolve into a generic external developer API surface in MVP.

---

### 10.20 Section Summary

#### API-096 — Section Summary Rule
At the interface-contract level, the product is defined as a bounded app-internal local-service system in which the shell supervises privileged actions, the engine exposes only the approved run-control routes, the UI consumes structured state and ordered events, errors are explicit and categorized, file intake is bounded and validated through Preflight, report availability is truthful, and cloud communication remains limited to approved commercial and distribution functions.

#### API-097 — Interface Consistency Rule
All later engineering annexes, OpenAPI artifacts, and implementation code must remain consistent with the route purposes, auth boundaries, sequencing rules, and error principles defined here.

#### API-098 — Section Boundary Rule
This section defines:
- interface classes
- local backend route set
- route intent
- auth expectations
- error behavior
- event sequencing behavior
- file and report interface rules
- app-to-cloud commercial interface boundaries

This section does not yet define:
- full machine-readable schema documents
- exact SQL DDL
- screen-level UI obligations
- acceptance-test phrasing

Those are governed in later sections or annexes.

---

## 11. Database and Commercial Backend Contract

### 11.1 Section Purpose
This section defines the commercial backend’s data-model responsibilities, licensing behavior, activation model, and durable business-record obligations.

This section exists to ensure that:
- the business backend remains structurally separate from the simulation engine
- activation and license logic are deterministic
- device-binding behavior is explicit
- AI coding agents do not invent vague or contradictory commercial rules
- durable backend records support entitlement truth, activation limits, and auditability

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- Section 8 — System Architecture
- Section 9 — Data Lifecycle and Storage Policy
- Section 10 — API and Interface Contract

If any lower-level backend artifact conflicts with this section, this section wins unless Document A is versioned forward.

---

### 11.2 Commercial Backend Philosophy

#### DB-001 — Commercial Backend Purpose Rule
The commercial backend exists to support:
- payment-linked entitlement creation
- license state tracking
- device-bound activation issuance
- runtime distribution authorization
- update support metadata
- basic commercial auditability

#### DB-002 — Commercial Isolation Rule
The commercial backend must not become the normal persistent execution environment for simulation runs in MVP.

#### DB-003 — Ledger Truth Rule
The commercial backend must remain the authoritative durable source of truth for license and activation ledger state.

#### DB-004 — No Simulation Ledger Rule
The commercial backend must not act as the durable ledger of:
- seed documents
- transcripts
- run memory
- simulation debate history
in MVP.

---

### 11.3 Required Commercial Entities

#### DB-005 — Minimum Entity Rule
The commercial backend data model must include durable representations for at minimum:
- licenses
- device activations
- payment events
- activation audit records
- issued device-bound tokens or their audit equivalents

#### DB-006 — Entity Traceability Rule
These durable entities must be sufficient to answer, at minimum:
- who holds the license
- what plan is active
- whether the license is active/expired/revoked
- how many activation slots are in use
- which device hashes are bound
- whether a given device-bound token issuance occurred
- what commercial event created or changed the entitlement

---

### 11.4 License Entity Contract

#### DB-007 — License Entity Purpose
The license entity represents the purchaser’s durable commercial entitlement.

#### DB-008 — Minimum License Fields
A license record must include at minimum:
- unique license identifier
- purchaser identity or purchaser email
- plan/tier
- entitlement status
- issue timestamp
- expiration timestamp where applicable
- activation limit
- creation timestamp
- last-updated timestamp

#### DB-009 — Allowed License Statuses
The license status model must support at minimum:
- active
- expired
- revoked
- payment-failed or equivalent non-usable commercial state where relevant

#### DB-010 — Activation Limit Rule
For MVP, the license entity must support an activation limit of exactly two devices per license unless later revised.

#### DB-011 — Lifetime License Handling Rule
If lifetime licenses exist, the license model must represent them without pretending they are time-boxed in the same manner as monthly or annual plans.

#### DB-012 — License Durability Rule
License records must persist durably in the commercial backend and must not depend on the local app for authoritative existence.

---

### 11.5 Device Activation Entity Contract

#### DB-013 — Device Activation Purpose
The device activation entity represents one approved binding between a license and one device identity.

#### DB-014 — Minimum Activation Fields
A device activation record must include at minimum:
- unique activation identifier
- parent license identifier
- activation slot index or equivalent slot-traceable concept
- device hash
- client platform identity
- activation status
- first activation timestamp
- last-seen or last-confirmed timestamp where applicable
- creation timestamp / update timestamp as appropriate

#### DB-015 — Activation Status Model
The activation status model must support at minimum:
- active
- revoked
- replaced or equivalent if device turnover is later supported
- inactive where needed by backend lifecycle

#### DB-016 — Device Hash Durability Rule
The durable activation record should store the approved device hash representation rather than raw hardware identifiers as the canonical backend value.

#### DB-017 — One Device Hash per Active Binding Rule
A specific active activation binding must correspond to one device hash identity under one license slot at a time.

#### DB-018 — Activation Slot Integrity Rule
The backend must not allow active activation count to exceed the license’s activation limit.

---

### 11.6 Payment Event Entity Contract

#### DB-019 — Payment Event Purpose
The payment event entity exists to record external commercial events received from the payment provider and their backend processing outcomes.

#### DB-020 — Minimum Payment Event Fields
A payment event record must include at minimum:
- unique internal event identifier
- payment provider identifier
- provider-side event identifier
- event type
- receipt timestamp
- processing status
- payload capture or approved payload reference
- related license identifier where determinable

#### DB-021 — Idempotency Rule
Payment event processing must support idempotent handling so the same external event does not create duplicate commercial state.

#### DB-022 — Payment Event Non-Authority Rule
A raw payment event is not itself the same as a ready-to-use local device activation.
The commercial backend must still translate payment truth into license truth and activation policy.

---

### 11.7 Activation Audit Entity Contract

#### DB-023 — Audit Entity Purpose
An activation audit entity exists to support traceability for important activation decisions and token issuance behavior.

#### DB-024 — Minimum Audit Fields
An activation audit record must include at minimum:
- unique audit identifier
- related license identifier
- related activation identifier where applicable
- action type
- actor type or origin
- reason or classification
- timestamp
- metadata reference or minimal approved metadata payload

#### DB-025 — Audit Use Rule
The audit log should be sufficient to trace:
- activation requested
- activation approved
- activation denied
- token issued
- activation revoked
- device replacement behavior if later supported

---

### 11.8 Issued Token / JWT Tracking Contract

#### DB-026 — Issued Token Tracking Purpose
The commercial backend should be able to track issued device-bound token issuance events or their functional equivalent for auditability and revocation support.

#### DB-027 — Minimum Issued Token Fields
If a dedicated issued-token record exists, it should include at minimum:
- unique token issuance identifier
- parent license identifier
- parent activation identifier
- issuance timestamp
- expiration timestamp where applicable
- key identifier or signing lineage marker
- token fingerprint or hash representation where appropriate
- token status

#### DB-028 — No Raw Token Overexposure Rule
The backend should avoid unnecessary long-term storage of raw token material where a safer fingerprint or issuance trace is sufficient.

#### DB-029 — Token Status Model
The token tracking model should support at minimum:
- active
- revoked
- superseded or replaced
where applicable

---

### 11.9 License Issuance Rules

#### DB-030 — License Creation Trigger Rule
A valid license record must be created or updated only from an approved commercial event flow.

#### DB-031 — Payment-to-License Translation Rule
The commercial backend must translate payment-provider truth into internal entitlement truth rather than assuming that a raw provider event is already the final app-usable state.

#### DB-032 — Plan Mapping Rule
A payment outcome must map deterministically to an internal plan/tier representation.

#### DB-033 — Expiration Mapping Rule
Where time-bounded plans exist, the backend must compute or assign a durable expiration value consistent with plan rules.

#### DB-034 — License State Correction Rule
If later commercial events change entitlement usability, the license record must be updated to reflect that new truth.

---

### 11.10 Activation Slot Rules

#### DB-035 — Two-Device Rule
Each license may have at most two active device activations in MVP.

#### DB-036 — Slot Enforcement Rule
Activation approval must fail when approving a new activation would exceed the activation limit, unless replacement logic is explicitly authorized and correctly applied.

#### DB-037 — Device Reuse Rule
The backend should recognize when the same device hash attempts to activate again under the same license and should avoid consuming an unnecessary extra slot where policy permits.

#### DB-038 — Replacement Caution Rule
If later device replacement flows are added, they must be explicit and auditable.
They must not silently mutate activation truth.

#### DB-039 — No Unlimited Sharing Rule
The backend must not issue device-bound activation truth in a manner that effectively creates unlimited device sharing under one license.

---

### 11.11 JWT Claim Contract

#### DB-040 — JWT Purpose Rule
The device-bound JWT exists to allow local offline validation after activation while preserving device-binding semantics.

#### DB-041 — Minimum Claim Rule
A device-bound JWT must include at minimum:
- subject or license-holder identity reference
- license identifier
- plan/tier
- device hash or equivalent binding claim
- issuance time
- expiration claim where applicable
- activation slot or equivalent binding context
- signing lineage sufficient for local verification

#### DB-042 — Device Binding Rule
The JWT must bind to the activated device identity.
A token minted for one device must not validate as a normal valid token on another device.

#### DB-043 — Offline Validation Rule
The JWT must support local offline validation after issuance, subject to the project’s known limitations around local clock trust and anti-tamper boundaries.

#### DB-044 — Lifetime Token Representation Rule
If a lifetime plan uses a far-future expiration model or equivalent representation, that must still remain explicit and consistent rather than pretending no expiration concept exists at all.

#### DB-045 — No Generic Transferable Token Rule
The JWT must not be treated as a freely transferable generic license string detached from device identity.

---

### 11.12 Activation Decision Rules

#### DB-046 — Activation Approval Rule
Activation may be approved only if all of the following are true:
- the license exists
- the license is usable
- the activation limit would not be violated
- the device identity is acceptable under policy
- token issuance can be completed safely

#### DB-047 — Activation Denial Rule
Activation must be denied when known policy blockers exist, including at minimum:
- invalid license
- expired or unusable license
- revoked entitlement
- activation limit exceeded
- device mismatch under the activation model where applicable

#### DB-048 — Activation Error Rule
Technical failures such as network, storage, or backend runtime problems must be classified as errors rather than policy denials when appropriate.

#### DB-049 — Local Storage Completion Rule
Activation is not fully successful until the local device-bound token has been issued and stored successfully on the device.

---

### 11.13 Commercial Backend to App Contract

#### DB-050 — Activation Response Rule
The commercial backend must return a response that clearly distinguishes:
- approval
- denial
- technical failure

#### DB-051 — Runtime Authorization Response Rule
For runtime distribution flows, the backend must clearly distinguish:
- authorized runtime access
- unauthorized runtime access
- technical delivery failure

#### DB-052 — Update Metadata Response Rule
For update support flows, the backend or its approved support surface must provide sufficient trusted metadata for the native app updater to apply its verification rules.

#### DB-053 — No Simulation Data Upload Contract
The app-to-commercial-backend contract must not require upload of seed documents or transcripts as a normal part of activation, runtime distribution, or update flows.

---

### 11.14 Backend Auditability and Traceability Rules

#### DB-054 — Durable Audit Rule
Commercial backend behavior must be traceable enough to explain later:
- why a license exists
- why an activation was granted or denied
- what payment event led to entitlement changes
- whether a device-bound token was issued

#### DB-055 — Minimal Necessary Audit Rule
Auditability must be achieved without collecting unnecessary sensitive simulation data.

#### DB-056 — Cross-Entity Consistency Rule
The backend must maintain internally consistent relationships between:
- payment events
- licenses
- activations
- token issuance records

#### DB-057 — No Orphan Activation Rule
An activation record must not exist without a valid parent license relationship.

#### DB-058 — No Orphan Token Rule
A device-bound token issuance must not exist without traceability to a valid activation and license context.

---

### 11.15 Commercial Data Integrity Prohibitions

#### DB-059 — No Simulation Persistence Rule
The commercial backend must not store persistent simulation execution artifacts as part of normal MVP backend behavior.

#### DB-060 — No Raw Hardware Identity Normalization Rule
The backend must not rely on raw hardware identity as the preferred durable canonical activation record if a privacy-conscious hash representation is the approved model.

#### DB-061 — No Unlimited Activation Drift Rule
The backend must not silently weaken the two-device rule through permissive logic or undocumented exemptions.

#### DB-062 — No Token-without-Entitlement Rule
The backend must not issue a valid app-usable device-bound token without an underlying usable entitlement state.

#### DB-063 — No Denial/Failure Confusion Rule
The backend must not conflate policy denial with technical failure if the distinction is known.

#### DB-064 — No Secret Oversharing Rule
Commercial backend records must not overshare sensitive local simulation data not required for commercial function.

---

### 11.16 Runtime Distribution and Update Support Contract

#### DB-065 — Runtime Distribution Support Rule
The commercial backend may authorize shell-owned managed runtime-package download access, but this does not make it the owner of local runtime installation.

#### DB-066 — Distribution Access Rule
Runtime-distribution authorization must be tied to legitimate app entitlement/use policy where appropriate.

#### DB-067 — Update Metadata Trust Rule
The commercial backend or its approved support surface must provide trustworthy update metadata consistent with the native app-updater lifecycle rules.

#### DB-068 — Distribution/Update Separation Rule
Runtime distribution and update support must remain distinct from simulation behavior and must not require simulation-data persistence.

---

### 11.17 Commercial Backend Summary

#### DB-069 — Section Summary Rule
At the commercial-backend level, the product is defined as follows:

The backend durably owns license truth, activation truth, payment-event traceability, device-binding enforcement, token issuance support, runtime distribution authorization, and update-support metadata. It must support exactly bounded commercial behavior consistent with the MVP’s two-device activation model and offline-validation token strategy. It must remain structurally separate from simulation execution and must not become a persistent storage layer for run content, transcripts, or seed documents.

#### DB-070 — Backend Consistency Rule
All later engineering annexes, SQL migrations, activation flows, and audit designs must remain consistent with the entity obligations and backend rules defined here.

#### DB-071 — Section Boundary Rule
This section defines:
- commercial backend entities
- license rules
- activation rules
- JWT claim obligations
- slot enforcement
- auditability requirements
- runtime/update support boundaries

This section does not yet define:
- exact SQL column types
- exact migration syntax
- full machine-readable API schemas
- full QA procedure wording

Those are governed in later sections or annexes.

---

## 12. UI/UX Contract

### 12.1 Section Purpose
This section defines the required user-facing behavior of the application without over-specifying visual style.

This section exists to ensure that:
- the user experience remains consistent with the product’s constitutional and behavioral rules
- AI coding agents do not invent screens, controls, or interaction patterns that change product meaning
- the UI communicates lifecycle truth honestly
- the user can operate the product successfully without understanding internal technical implementation
- user-visible states, warnings, and actions remain deterministic

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- Section 5 — User Journeys and Behavioral Narratives
- Section 6 — Product Behavior Specification
- Section 7 — State Machines
- Section 8 — System Architecture
- Section 9 — Data Lifecycle and Storage Policy
- Section 10 — API and Interface Contract
- Section 11 — Database and Commercial Backend Contract

If a lower-level UI design artifact conflicts with this section, this section wins unless Document A is versioned forward.

---

### 12.2 UI/UX Philosophy

#### UI-001 — Behavioral UI Rule
The UI exists to truthfully expose product behavior, not to cosmetically hide complexity or failure.

#### UI-002 — Non-Expert Operability Rule
The UI must be operable by a user who is not a software engineer and does not understand the app’s internal runtime architecture.

#### UI-003 — Truthful State Communication Rule
The UI must communicate the real product state and must not:
- present readiness when readiness does not exist
- present completion when completion did not occur
- present cleanup success when cleanup failed
- present unlimited freedom when bounded execution rules apply

#### UI-004 — No Marketing-over-Truth Rule
UI copy and status language must not overstate privacy, deletion guarantees, offline capability, or prediction certainty.

#### UI-005 — Guided Simplicity Rule
The UI should reduce cognitive load by guiding the user through the approved product flow rather than exposing unnecessary system internals.

#### UI-006 — Bounded Interaction Rule
The UI must expose only the controls necessary for the approved MVP scope and must not behave like a generic AI workstation or developer console by default.

---

### 12.3 Required Screen-Level Surfaces

#### UI-007 — Minimum Surface Rule
The MVP must provide, at minimum, user-visible surfaces for:

1. first-launch / runtime hydration
2. activation
3. provider configuration
4. run configuration
5. Preflight results
6. active run monitoring
7. report access / export
8. cleanup or failure feedback where relevant
9. update notification/support where relevant to the approved updater behavior

#### UI-008 — Surface Integrity Rule
Each required surface must map to a real product responsibility and not merely to an aesthetic navigation concept.

---

### 12.4 Runtime Hydration UX Contract

#### UI-009 — Hydration Visibility Rule
If the heavy runtime is missing or repair is required, the UI must clearly communicate that runtime hydration is required before simulation can be used.

#### UI-010 — Hydration Progress Rule
During runtime hydration, the UI must expose meaningful progress feedback, such as:
- download/install in progress
- verification in progress
- completion
- failure and retry opportunity

#### UI-011 — Hydration Failure Rule
If runtime hydration fails, the UI must clearly indicate:
- that hydration failed
- whether retry is possible
- whether simulation remains unavailable until hydration succeeds

#### UI-012 — No Hidden Hydration Rule
The UI must not imply that the product is fully ready while runtime hydration remains incomplete.

---

### 12.5 Activation UX Contract

#### UI-013 — Activation Requirement Visibility Rule
If the device is not activated, the UI must clearly indicate that activation is required before licensed use can proceed.

#### UI-014 — Activation Flow Control Rule
The activation surface must allow the user to:
- begin activation
- understand success or denial outcomes
- distinguish denial from technical failure
- proceed only when activation is complete

#### UI-015 — Activation Feedback Rule
The UI must clearly differentiate between:
- activation in progress
- activation approved/successful
- activation denied
- activation failed due to technical error

#### UI-016 — Device-Limit Honesty Rule
If activation is denied because device-slot limits are reached, the UI must not present that as a generic network or unknown error.

---

### 12.6 Provider Configuration UX Contract

#### UI-017 — Provider Selection Control Rule
The UI must provide a clear mechanism for selecting a supported provider.

#### UI-018 — Provider Key Entry Rule
The UI must provide a clear input path for entering the provider API key.

#### UI-019 — Secure Entry Messaging Rule
The UI should communicate that provider credentials are stored through OS-native secure storage rather than ordinary app text files.

#### UI-020 — Configuration Success Rule
After successful storage of a provider key, the UI must indicate that provider configuration is now present.

#### UI-021 — Configuration Non-Promise Rule
The UI must not imply that storing the provider key automatically means provider connectivity and model readiness are guaranteed.
That truth is determined during Preflight.

#### UI-022 — Missing Configuration Gating Rule
If provider configuration is missing, the UI must block run-start behavior and indicate what is missing.

---

### 12.7 Run Configuration UX Contract

#### UI-023 — Required Run Inputs Rule
The run-configuration UI must expose controls for:
- seed document selection
- requested agent count
- requested round count
- max token budget
- provider/model selection where applicable in MVP UX

#### UI-024 — Single Seed Rule
The run-configuration UI must support one seed document per run, not multi-document batch composition.

#### UI-025 — Explicit Budget Input Rule
The max token budget must be a visible, required input rather than a hidden default.

#### UI-026 — Visible Constraints Rule
Where reasonable, the UI should communicate relevant input constraints, including:
- English-only seed policy
- supported file types
- bounded run design
- max token budget requirement

#### UI-027 — No Hidden Advanced Control Rule
The MVP UI must not require the user to configure low-level internals such as:
- concurrency
- backoff policy
- memory compression
- prompt strategy
- port selection
- vector database settings

---

### 12.8 Seed Document Intake UX Contract

#### UI-028 — Seed Selection Feedback Rule
When the user selects a file, the UI must clearly show which file is selected.

#### UI-029 — Preliminary Validation Feedback Rule
If the app can detect immediate obvious issues such as unsupported type or excessive file size before Preflight, the UI should surface that early.

#### UI-030 — Preflight-Gated Validity Rule
The UI must not imply that a selected file is fully accepted for execution until Preflight approves it.

#### UI-031 — English-Only Feedback Rule
The UI must communicate that non-English seed documents are not supported in MVP.

---

### 12.9 Preflight UX Contract

#### UI-032 — Preflight Action Rule
The UI must provide an explicit action that triggers Preflight rather than silently merging Preflight into hidden behavior.

#### UI-033 — Preflight Running Visibility Rule
When Preflight is executing, the UI must indicate that evaluation is in progress.

#### UI-034 — Preflight Outcome Visibility Rule
The UI must present one of exactly these high-level result classes:
- approved
- approved with downgrade
- rejected

#### UI-035 — Downgrade Disclosure Rule
If Preflight returns approved-with-downgrade, the UI must explicitly show:
- what the user requested
- what the effective plan became
- that a downgrade occurred

#### UI-036 — Rejection Reason Rule
If Preflight rejects the run, the UI must surface a meaningful reason rather than only a generic failure label.

#### UI-037 — Start Gating Rule
The run-start control must remain blocked until the UI has a valid start-eligible Preflight outcome.

#### UI-038 — Invalidated Preflight Rule
If the user changes a material input after a valid Preflight result, the UI must mark the prior result as stale or invalid and must not imply it is still current.

---

### 12.10 Active Run Monitoring UX Contract

#### UI-039 — Live Run Surface Rule
During an active run, the UI must present a dedicated run-monitoring surface rather than leaving the user on a static configuration form.

#### UI-040 — Required Live Elements Rule
The live run surface must include, at minimum:
- current run state
- current phase visibility
- transcript or event feed
- current round indicator where applicable
- budget usage visibility
- elapsed time visibility
- report readiness visibility when applicable

#### UI-041 — Phase Truth Rule
The UI must distinguish meaningful active phases, including:
- ingesting
- generating personas
- running round activity
- paused due to rate limits
- paused due to network/provider issues
- finalizing
- cleaning

#### UI-042 — Event Feed Rule
The active run surface must show ordered ongoing debate/event output in a way the user can follow chronologically.

#### UI-043 — Persona Attribution Rule
Visible debate messages must remain attributable to their persona labels.

#### UI-044 — No Static Black-Box Rule
The user must not be forced to stare at a static undifferentiated loading screen during the main run when meaningful progress is available.

---

### 12.11 Budget and Time UX Contract

#### UI-045 — Budget Visibility Rule
The UI must expose budget state throughout run execution at a level meaningful to the user.

#### UI-046 — Budget Warning Rule
If the run approaches the token ceiling, the UI should surface warnings before hard budget exhaustion where meaningful thresholds are crossed.

#### UI-047 — Budget Hit Rule
If the budget is hit, the UI must clearly indicate:
- that new normal rounds will stop
- that the run is entering controlled finalization
- that the final output will reflect completed work only

#### UI-048 — Duration Visibility Rule
The UI must make it legible that the run is time-bounded and should expose elapsed time during execution.

#### UI-049 — Duration-Cap Truth Rule
If a duration cap contributes to run truncation or finalization, the UI must communicate that honestly.

---

### 12.12 Interruption and Degradation UX Contract

#### UI-050 — Degradation Visibility Rule
If the run is paused or degraded by rate limits, network failure, or provider problems, the UI must say so explicitly.

#### UI-051 — Retry vs Failure Distinction Rule
The UI should distinguish between:
- temporary degraded state
- retryable problem
- terminal failure

when that distinction is known.

#### UI-052 — No Fake Healthy Progress Rule
The UI must not continue to display healthy-progress language when the run is actually blocked or paused.

#### UI-053 — Finalize-vs-Fail Messaging Rule
If the product chooses controlled finalization after interruption instead of terminal failure, the UI must communicate that this is a constrained completion path rather than a normal full run.

---

### 12.13 Cancellation UX Contract

#### UI-054 — Cancel Action Rule
The UI must provide a clearly labeled cancellation control when the run is in a cancelable state.

#### UI-055 — Cancel Confirmation Rule
The product may use confirmation UX for cancellation if needed, but such UX must not misrepresent what cancellation means.

#### UI-056 — Cancellation Truth Rule
When cancellation is accepted, the UI must indicate that:
- normal run progression has stopped
- the run is canceling or canceled
- cleanup may still follow

#### UI-057 — No Cancel-as-Success Rule
The UI must not present a canceled run as a successful report-producing completion unless the product explicitly defines a valid output path for that state.

---

### 12.14 Finalization and Completion UX Contract

#### UI-058 — Finalization Visibility Rule
When the run enters finalization, the UI must indicate that normal rounds have ended and output synthesis is in progress.

#### UI-059 — Completion Distinction Rule
The UI must distinguish between:
- full completion
- budget-capped completion
- terminal failure
- canceled
- cleanup failure after an otherwise completed run

#### UI-060 — Report Readiness Rule
The UI must only present the report as ready when the report actually exists and is available.

#### UI-061 — Honest Completion Messaging Rule
If the report is based on constrained or truncated execution, the UI must not imply that the run completed all planned rounds.

---

### 12.15 Cleanup UX Contract

#### UI-062 — Cleanup Visibility Rule
Where cleanup is material to user understanding, the UI must surface that cleanup is occurring or was attempted.

#### UI-063 — Cleanup Failure Visibility Rule
If cleanup fails materially, the UI must not conceal that outcome.

#### UI-064 — Completion-vs-Cleanup Distinction Rule
The UI must preserve the distinction between:
- a truthful run outcome
- a truthful cleanup outcome

Example:
A run may be completed successfully while cleanup still failed.

#### UI-065 — Janitor Messaging Rule
If stale abandoned workspace cleanup is detected on next launch and materially fails, the UI should inform the user rather than silently implying full recovery.

---

### 12.16 Report Access UX Contract

#### UI-066 — Report Access Control Rule
When a report is available, the UI must provide a clear way to:
- view it
- open it
- or otherwise access the local PDF artifact

#### UI-067 — Report Availability Rule
The UI must not show report access affordances if no report exists.

#### UI-068 — Offline Viewing Consistency Rule
The UI may allow viewing or opening already-generated local reports while offline, provided the report file exists locally.

#### UI-069 — Report-Persistence Distinction Rule
The UI must not imply that transient workspace cleanup deletes the intentionally exported final report.

---

### 12.17 Disabled, Empty, and Loading State Contract

#### UI-070 — Disabled-State Rule
Controls that cannot validly be used in the current lifecycle state must appear disabled or otherwise clearly unavailable.

#### UI-071 — Disabled-State Explanation Rule
Where a key action is disabled, the UI should explain why at a user-meaningful level.

#### UI-072 — Empty-State Rule
If no report, no seed file, no provider key, or no activation exists, the UI must present a clear empty-state explanation rather than a blank or confusing surface.

#### UI-073 — Loading-State Truth Rule
Loading states must correspond to real work and should indicate what kind of work is occurring where meaningful.

#### UI-074 — No Infinite Ambiguous Spinner Rule
The UI must not trap the user behind an indefinite ambiguous spinner without meaningful state or error feedback when the underlying system knows more.

---

### 12.18 Update UX Contract

#### UI-075 — Update Discovery Visibility Rule
If the updater finds an available update, the UI may surface that information in a bounded, truthful way.

#### UI-076 — Unsafe Update Prohibition Rule
The UI must not present an update as installable if it has been rejected by trust or integrity checks.

#### UI-077 — Active Run Safety Rule
The UI should avoid encouraging disruptive update application that would silently violate active-run truth or shutdown rules.

---

### 12.19 UX Integrity Prohibitions

#### UI-078 — No Hidden Scope Expansion Rule
The UI must not imply support for out-of-scope capabilities, such as:
- multilingual ingestion
- crash resume
- unlimited device use
- vendor-hosted run history
- offline simulation
- enterprise team collaboration

#### UI-079 — No False Security Messaging Rule
The UI must not imply guaranteed forensic deletion, perfect local secrecy, or military-grade protection.

#### UI-080 — No False Certainty Messaging Rule
The UI must not imply that the Market Viability Score is guaranteed objective truth.

#### UI-081 — No Misleading Simplicity Rule
The UI may simplify complexity for usability, but it must not simplify by lying about product truth.

---

### 12.20 Section Summary

#### UI-082 — Section Summary Rule
At the UI/UX-contract level, the product is defined as a guided desktop experience that:
- truthfully exposes readiness, run, and cleanup state
- supports non-expert operation
- provides explicit activation, configuration, Preflight, run, and report flows
- surfaces bounded execution, budget, and time constraints honestly
- preserves truthful distinctions between success, constrained completion, cancellation, failure, and cleanup outcomes
- avoids unauthorized scope or security-claim inflation

#### UI-083 — UX Consistency Rule
All future screen designs, interaction specs, and implementation work must remain consistent with the user-facing obligations defined here.

#### UI-084 — Section Boundary Rule
This section defines:
- required surfaces
- required user-visible controls and states
- messaging truth obligations
- disabled/empty/loading behavior
- report and cleanup UX expectations

This section does not yet define:
- pixel-perfect design system rules
- branding details
- visual identity
- acceptance-test wording

Those belong in later documents or design artifacts.

---

## 13. Non-Functional Requirements

### 13.1 Section Purpose
This section defines the system-wide quality, performance, safety, boundedness, and operational constraints that govern the product beyond pure functional behavior.

This section exists to ensure that:
- the product remains buildable and operable within realistic limits
- AI coding agents do not optimize only for “feature exists” while ignoring engineering discipline
- constraints on speed, memory, security, logging, updates, and reliability are explicit
- acceptance criteria later have a constraint foundation to point to

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- Sections 6–12 where applicable

If a lower-level engineering artifact conflicts with these non-functional requirements, this section wins unless Document A is versioned forward.

---

### 13.2 NFR Philosophy

#### NFR-001 — Constraint Rule
These non-functional requirements are product constraints, not optional engineering preferences.

#### NFR-002 — Boundedness Rule
The MVP must remain bounded in cost, time, scope, and operational complexity.

#### NFR-003 — Honest Constraint Rule
The product must not claim non-functional qualities stronger than those it can realistically support.

#### NFR-004 — Buildability Rule
Where multiple valid engineering options exist, the implementation should prefer the option that best preserves determinism, clarity, and bounded risk for AI-led development.

---

### 13.3 Platform and Environment Requirements

#### NFR-005 — Platform Scope Rule
The MVP must support Windows only.

#### NFR-006 — Unsupported Platform Rule
macOS, Linux, and other non-Windows platforms are out of scope for MVP and must not be treated as implied support targets.

#### NFR-007 — Desktop-Native Rule
The product must operate as a desktop application rather than requiring the user to use a browser-hosted vendor dashboard for core run execution.

#### NFR-008 — Local Runtime Requirement
The simulation runtime must execute locally on the user’s machine, subject to the architecture and hydration model.

---

### 13.4 Performance and Responsiveness Requirements

#### NFR-009 — UI Responsiveness Rule
The UI must remain meaningfully responsive during ordinary app use and active run monitoring.

#### NFR-010 — Incremental Progress Rule
Long operations such as Preflight, runtime hydration, and active simulation must expose incremental state rather than appearing as indefinite silent hangs.

#### NFR-011 — Transcript Rendering Performance Rule
The live transcript/event feed must be implemented in a way that avoids unbounded UI degradation during long or dense runs.

#### NFR-012 — Virtualization Requirement
The UI must use a virtualized or equivalently bounded rendering approach for long transcript/event lists.

#### NFR-013 — No DOM Explosion Rule
The product must not rely on rendering the full historical transcript as unrestricted live DOM content if doing so risks major responsiveness collapse.

#### NFR-014 — Shell Stability Rule
Ordinary provider/network degradation must not, by itself, be allowed to crash the app shell.

---

### 13.5 Time and Duration Requirements

#### NFR-015 — Hard Duration Cap Rule
Each run must be bounded by a hard duration cap of two hours.

#### NFR-016 — Preflight Duration Awareness Rule
Preflight must produce an estimated duration band sufficient to help the user understand likely execution scope before start.

#### NFR-017 — No Unbounded Run Rule
The system must not allow normal run progression to continue indefinitely.

#### NFR-018 — Controlled End Rule
If a duration cap is reached, the system must stop scheduling new normal rounds and choose controlled finalization or truthful failure.

---

### 13.6 Cost and Budget Requirements

#### NFR-019 — Mandatory Budget Rule
A max token budget is required before a run can begin.

#### NFR-020 — Budget Visibility Rule
The product must expose budget consumption meaningfully during the run.

#### NFR-021 — Budget Enforcement Rule
The system must treat the budget cap as operationally meaningful, not decorative.

#### NFR-022 — No Unlimited Spend Rule
The product must not allow normal run execution with no budget boundary.

#### NFR-023 — Cost Transparency Rule
The product must provide a cost/token summary in the final report and should not obscure that the user bears provider inference cost responsibility.

---

### 13.7 Concurrency and Provider Interaction Requirements

#### NFR-024 — Bounded Concurrency Rule
Outbound provider-call concurrency must be bounded under the run execution model.

#### NFR-025 — Provider-Aware Rule
The provider-call behavior must respect provider-specific availability, rate-limit, and degradation realities to the extent the implementation can observe them.

#### NFR-026 — No Full-Fanout Rule
The product must not use an all-agents-speak-at-once architecture as its normal moderated execution model.

#### NFR-027 — Retry Bound Rule
Retry logic must be bounded and must not enter infinite loops.

#### NFR-028 — Degradation Handling Rule
The product must support degraded execution states such as paused-by-rate-limit or paused-by-network without conflating them with full successful progress.

---

### 13.8 File, Input, and Ingestion Requirements

#### NFR-029 — Supported File-Type Rule
The product must accept only PDF, DOCX, TXT, and Markdown seed documents in MVP.

#### NFR-030 — File Size Enforcement Rule
The product must enforce a maximum seed-document file size of 10 MiB (10,485,760 bytes).

#### NFR-031 — English-Only Enforcement Rule
The product must reject seed documents during Preflight when detected English-confidence is below 0.80.

#### NFR-032 — Minimum Usable Content Rule
The product must require at least 1,000 normalized extracted characters before approving execution.

#### NFR-033 — Single-Document Rule
The product must be engineered for one seed document per run in MVP.

---

### 13.9 Reliability and Lifecycle Requirements

#### NFR-034 — No Crash Resume Rule
The product must not promise crash-resume behavior in MVP.

#### NFR-035 — Startup Janitor Requirement
The app must perform startup janitor logic to detect and attempt cleanup of abandoned workspaces from abnormal termination.

#### NFR-036 — Graceful-Then-Forceful Shutdown Rule
Normal shutdown should attempt graceful engine termination first, then forceful termination only when necessary under policy.

#### NFR-037 — Truthful Lifecycle Rule
Run outcome, cleanup outcome, and app lifecycle outcome must remain distinguishable and truthfully reported.

#### NFR-038 — No Silent Failure Rule
Material failures must not disappear silently when the product knows that something went wrong.

#### NFR-039 — Recovery Scope Rule
Where recovery is supported in MVP, it applies to bounded retriable operations such as transient provider/network problems or runtime download retry — not to crash-resume run restoration.

---

### 13.10 Security and Privacy Requirements

#### NFR-040 — OS-Native Secret Storage Rule
Provider secrets must be stored only in approved OS-native secure storage.

#### NFR-041 — Local Auth Boundary Rule
The local backend must use the app’s approved local trust model rather than assuming localhost is inherently safe.

#### NFR-042 — Runtime Integrity Rule
Simulation execution must not be enabled if runtime integrity checks fail.

#### NFR-043 — Update Integrity Rule
Update artifacts must be validated according to the updater trust model before application.

#### NFR-044 — Best-Effort Cleanup Rule
The system must attempt cleanup of transient workspace artifacts after terminal run paths and on next-launch janitor flow after abnormal termination.

#### NFR-045 — No Absolute Security Claim Rule
The product must not imply perfect local secrecy, perfect anti-forensics, or undefeatable DRM.

#### NFR-046 — Residual Risk Honesty Rule
Residual local risks such as indexing residue, memory inspection, and storage-layer limitations must remain inside the product’s honest disclosure boundary.

---

### 13.11 Logging and Diagnostics Requirements

#### NFR-047 — Minimal Diagnostics Rule
Diagnostics must be minimized to what is necessary for debugging, supportability, and truthful operational inspection.

#### NFR-048 — No Secret Logging Rule
Production logs must not intentionally contain provider secrets.

#### NFR-049 — No Raw Transcript Logging Rule
The product must not treat full transcript capture as ordinary production diagnostic logging.

#### NFR-050 — No Full Seed Logging Rule
The product must not treat full seed-document content as ordinary production diagnostic logging.

#### NFR-051 — Cleanup Failure Logging Rule
Material cleanup failures should be logged in an approved minimal form because they are operationally significant.

#### NFR-052 — State-Traceability Logging Rule
Diagnostics should preserve enough canonical lifecycle information that engineering and QA can trace what actually occurred.

---

### 13.12 Update and Distribution Requirements

#### NFR-053 — Thin Installer Support Rule
The product may use a thin-installer plus runtime-hydration model for MVP.

#### NFR-054 — Runtime Verification Requirement
The runtime package must be verified before the app treats simulation features as available.

#### NFR-055 — Resume-on-Download-Failure Requirement
Runtime download behavior should support bounded recovery or resume where practical rather than needless restart-from-zero behavior.

#### NFR-056 — Unsafe Update Rejection Rule
The app must not apply an update artifact that fails required verification checks.

#### NFR-057 — Update/Run Separation Rule
Update availability must not silently override active-run truth or shutdown policy.

---

### 13.13 Data Handling and Persistence Requirements

#### NFR-058 — Local-Data-First Requirement
Run-local simulation data must remain local to the user’s machine under the approved data-policy boundaries.

#### NFR-059 — No Vendor Simulation Archive Requirement
The product must not intentionally build a vendor-hosted persistent simulation archive in MVP.

#### NFR-060 — Persistent-vs-Transient Distinction Requirement
The implementation must preserve the distinction between:
- persistent installation/commercial artifacts
- transient run-local execution artifacts
- intentionally persistent exported reports

#### NFR-061 — Cross-Run Isolation Requirement
The system must prevent cross-run semantic leakage of working memory and workspace artifacts.

---

### 13.14 AI-Led Development Requirements

#### NFR-062 — Deterministic Implementation Preference
Where multiple solutions are possible, implementation should prefer the simpler and more deterministic solution that preserves product truth.

#### NFR-063 — No Cleverness-over-Clarity Rule
AI-generated implementation must not prioritize clever abstraction over maintainable behavioral fidelity.

#### NFR-064 — No Unauthorized Scope Expansion Rule
Implementation must not introduce adjacent features or hidden persistence layers not authorized by the spec.

#### NFR-065 — Traceability Rule
Engineering artifacts should remain traceable back to:
- behavioral rules
- state-machine rules
- architecture rules
- data-policy rules
- acceptance criteria

---

### 13.15 User Experience Quality Requirements

#### NFR-066 — Clear Messaging Rule
The product must prefer clear, direct user messaging over jargon-heavy or misleading status text.

#### NFR-067 — Honest Constraint Messaging Rule
The UI must communicate boundedness honestly, including:
- English-only policy
- token budget requirement
- two-hour cap
- possible downgrade behavior
- no crash-resume behavior

#### NFR-068 — Non-Expert Accessibility Rule
The interaction model must not presume expert knowledge of developer infrastructure concepts.

#### NFR-069 — No Overloaded Control Surface Rule
The UI must not expose so many advanced controls that the product ceases to function as a guided bounded MVP tool.

---

### 13.16 Operational Integrity Prohibitions

#### NFR-070 — No Hidden Persistence Prohibition
The implementation must not introduce undeclared persistent storage for run-local content.

#### NFR-071 — No State Dishonesty Prohibition
The implementation must not collapse materially different states into misleading user-visible outcomes.

#### NFR-072 — No Unsafe Privilege Bypass Prohibition
The system must not let the frontend bypass shell-owned privileged operations.

#### NFR-073 — No Unverified Runtime Use Prohibition
The product must not launch simulation behavior on a runtime package whose integrity has not been accepted.

#### NFR-074 — No Unsafe Update Apply Prohibition
The product must not apply rejected or unverified updates.

#### NFR-075 — No Generic “Works Once” Quality Bar
A one-time happy-path demo is insufficient.
The product must satisfy the bounded lifecycle, cleanup, integrity, and truthfulness rules of this document.

---

### 13.17 Section Summary

#### NFR-076 — Section Summary Rule
At the non-functional level, the MVP is defined as a bounded Windows-only desktop product that must remain responsive, truthful, cost-controlled, time-bounded, security-conscious without false absolutes, locally orchestrated, update-verifiable, runtime-verifiable, cleanup-aware, and suitable for non-expert operation under AI-led development.

#### NFR-077 — Constraint Consistency Rule
All later acceptance criteria, implementation plans, and engineering annexes must remain consistent with the constraint model defined here.

#### NFR-078 — Section Boundary Rule
This section defines:
- platform support constraints
- responsiveness constraints
- time and budget constraints
- concurrency and ingestion constraints
- reliability constraints
- security/privacy constraints
- logging constraints
- update/runtime integrity constraints
- AI-led development quality constraints

This section does not yet define:
- exact test-case wording
- final acceptance IDs
- detailed UI copy
- exact schema syntax

Those belong in later sections or annexes.

---

## 14. Acceptance Criteria

### 14.1 Section Purpose
This section defines the explicit criteria by which the MVP may be judged as implemented correctly.

This section exists to ensure that:
- the project can be verified against deterministic conditions
- AI coding agents know what “done” means
- QA, red-team review, and execution planning can trace back to stable acceptance IDs
- no new behavior is invented during testing that was not already defined in earlier sections

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- Sections 4 through 13 as applicable

If an acceptance criterion appears to conflict with a higher-authority behavioral, architectural, lifecycle, data, or non-functional rule, the higher-authority rule wins and the acceptance criterion must be interpreted accordingly.

---

### 14.2 Acceptance-Criteria Philosophy

#### AC-001 — Verification Rule
Each acceptance criterion must describe a verifiable product obligation, not a vague aspiration.

#### AC-002 — No New Behavior Rule
Acceptance criteria must not introduce new product behavior that was not already defined in earlier sections.

#### AC-003 — One-Truth Rule
Acceptance criteria exist to verify previously defined truth, not to redefine it.

#### AC-004 — Testability Rule
Every acceptance criterion must be testable through:
- inspection
- controlled execution
- state verification
- output verification
- or auditable system evidence

#### AC-005 — Stable Identifier Rule
Acceptance criteria must retain stable identifiers so downstream QA matrices, engineering annexes, and execution plans can reference them unambiguously.

---

### 14.3 Constitutional and Platform Acceptance Criteria

#### AC-006 — Windows-Only Scope
The product MUST support Windows only for MVP.

#### AC-007 — No Non-Windows Support Claim
The product MUST NOT require macOS, Linux, or any other non-Windows platform support for MVP completion and MUST NOT present them as supported MVP platforms.

#### AC-008 — Desktop-Native Product Form
The product MUST function as a desktop application rather than requiring a browser-hosted vendor simulation dashboard for core execution.

#### AC-009 — Local-Data-First Framing Consistency
The product’s behavior and user-facing truth MUST remain consistent with the framing:
**local-data-first, cloud-inference-dependent, intermittently connected desktop software**.

#### AC-010 — No Absolute Security Claim
The implemented product MUST NOT require false absolute claims such as guaranteed forensic erasure, fully offline simulation, or military-grade secrecy in order to be considered complete.

---

### 14.4 Licensing and Activation Acceptance Criteria

#### AC-011 — Commercial Entitlement Requirement
The product MUST require valid entitlement and activation for licensed local use.

#### AC-012 — Device-Bound Activation
The product MUST support device-bound activation rather than unlimited generic transferable license use.

#### AC-013 — Two-Device Limit
A single license MUST support at most two activated devices in MVP.

#### AC-014 — Activation Denial on Slot Exhaustion
If a third device activation would exceed the limit, activation MUST be denied rather than silently approved.

#### AC-015 — Offline Post-Activation Validation
After valid activation, the local app MUST be able to validate the activation token offline for approved offline-capable scenarios.

#### AC-016 — Device Mismatch Rejection
A device-bound token minted for one machine MUST fail normal local validation on a materially different unauthorized machine.

#### AC-017 — Activation Outcome Truthfulness
The app MUST clearly distinguish activation success, activation denial, and activation technical failure.

#### AC-018 — Activation Token Local Persistence
The local app MUST persist the approved device-bound activation artifact in an approved secure/local manner sufficient for offline validation after activation.

---

### 14.5 Runtime Hydration and Runtime Integrity Acceptance Criteria

#### AC-019 — Runtime Hydration Requirement
If the heavy simulation runtime is not present or requires repair, the app MUST require runtime hydration before simulation features become available.

#### AC-020 — Runtime Hydration Progress Visibility
During runtime hydration, the user MUST be able to see that hydration is actively occurring.

#### AC-021 — Runtime Resume-on-Failure Support
If runtime download is interrupted, the implementation MUST support bounded recovery or resumable continuation where practical rather than requiring needless restart from zero for ordinary interrupted downloads.

#### AC-022 — Runtime Integrity Verification
The app MUST verify runtime integrity before enabling simulation execution.

#### AC-023 — Runtime Integrity Failure Block
If runtime integrity verification fails, the app MUST block simulation execution until repair or re-hydration succeeds.

#### AC-024 — Runtime/Workspace Separation
The persistent runtime package MUST be stored separately from transient run-local workspace artifacts.

---

### 14.6 Provider Configuration and Secret Handling Acceptance Criteria

#### AC-025 — Provider Selection Requirement
The app MUST support selecting an approved provider for MVP use.

#### AC-026 — Provider Key Storage Requirement
The app MUST provide a way to store a provider API key required for run execution.

#### AC-027 — OS-Native Secret Storage
Provider secrets MUST be stored only through approved OS-native secure storage facilities.

#### AC-028 — No Plaintext Secret Persistence
Provider secrets MUST NOT be intentionally persisted in ordinary plaintext app configuration files, normal workspace files, or routine logs.

#### AC-029 — Configuration Gating
If required provider configuration is missing, the app MUST block valid run start and explain what is missing.

#### AC-030 — Configuration Non-Promise
Successful provider-key storage MUST NOT be misrepresented as guaranteed provider/model readiness before Preflight has validated it.

---

### 14.7 Seed Document and Preflight Acceptance Criteria

#### AC-031 — Single Seed Document Per Run
The product MUST support exactly one seed document per run in MVP.

#### AC-032 — Supported File-Type Enforcement
The product MUST enforce the approved supported file-type boundary for seed-document intake in MVP: PDF, DOCX, TXT, and Markdown only.

#### AC-033 — File Size Enforcement
The product MUST enforce the approved maximum file-size boundary for seed-document intake in MVP at 10 MiB (10,485,760 bytes).

#### AC-034 — Preflight Required
The product MUST require Preflight before any run may begin active execution.

#### AC-035 — Preflight Outcome Set
Preflight MUST return exactly one of these high-level outcomes:
- approved
- approved with downgrade
- rejected

#### AC-036 — English-Only Enforcement
Preflight MUST reject seed documents whose English-confidence is below 0.80.

#### AC-037 — Rejection Reason Visibility
When Preflight rejects a run for a known reason, the product MUST surface a meaningful rejection reason rather than only a generic failure label.
For document-validation rejection, the minimum known rejection reasons are:
- unsupported file type
- file too large
- text extraction failed
- insufficient extractable text
- non-English seed document

#### AC-038 — Downgrade Transparency
When Preflight approves with downgrade, the product MUST clearly show:
- what the user requested
- what the effective plan is
- that a downgrade occurred

#### AC-039 — Start Eligibility Gating
The run-start path MUST be blocked unless the current Preflight result is run-start-eligible.

#### AC-040 — Preflight Invalidation on Material Change
If a material run input changes after a valid Preflight result, the prior result MUST be invalidated and MUST NOT be silently reused as current.

---

### 14.8 Run Initialization and Persona Generation Acceptance Criteria

#### AC-041 — Fresh Run Creation
Each run MUST start as a fresh run rather than as a hidden continuation of a prior run.

#### AC-042 — Fresh Workspace Allocation
Each run MUST receive a fresh isolated workspace.

#### AC-043 — No Cross-Run Workspace Reuse
Run workspaces MUST NOT be reused as active execution context for later runs.

#### AC-044 — Persona Generation at Runtime
The product MUST generate personas at runtime from the seed document context rather than requiring a static user-authored persona roster.

#### AC-045 — Moderator Agent Existence
Every run MUST contain exactly one Moderator Agent.

#### AC-046 — Perspective Diversity
The runtime persona set MUST include meaningful perspective diversity, including favorable, skeptical, and analytical/neutral perspectives.

#### AC-047 — Persona Traceability
Persona outputs shown in the live UI or final report MUST remain attributable to stable persona labels within the run.

---

### 14.9 Moderated Debate Acceptance Criteria

#### AC-048 — Moderated Debate Model
The product MUST use a moderated debate model rather than an unbounded all-agents-speak swarm.

#### AC-049 — Speaker Subset Rule
For each round, the Moderator MUST select a subset of 3 to 5 participant agents to speak.

#### AC-050 — No Full-Fanout Normal Rounds
The product MUST NOT use all-agent full-fanout speaking as the normal round model in MVP.

#### AC-051 — Round Summary Continuity
Each completed round MUST produce or update a truthful round summary representation.

#### AC-052 — Speaker Attribution
Round outputs shown to the user MUST remain attributable to the correct persona.

#### AC-053 — No Fabricated Debate Content
The product MUST NOT fabricate agent turns, round content, or report content to conceal missing execution.

---

### 14.10 Memory and Context Management Acceptance Criteria

#### AC-054 — Bounded Memory Strategy
The product MUST use bounded memory/context management rather than unbounded full-transcript replay into every inference call.

#### AC-055 — Seed Context Grounding
The run MUST remain grounded in the seed document context rather than drifting into unrelated content.

#### AC-056 — Round Summary Memory
The system MUST maintain a bounded summary representation across rounds sufficient for continuity.

#### AC-057 — Argument/Risk Memory
The system MUST maintain a bounded representation of surfaced arguments, objections, and risks sufficient for coherent finalization.

#### AC-058 — Transcript Archive Distinction
The full transcript archive MUST remain available for visibility and report appendix purposes, but MUST NOT be treated as an unlimited prompt context source.

#### AC-059 — No Cross-Run Semantic Leakage
Content from one run MUST NOT leak into a later run as active memory.

---

### 14.11 Budget and Duration Acceptance Criteria

#### AC-060 — Mandatory Max Token Budget
A max token budget MUST be required before a run can begin.

#### AC-061 — Budget Tracking
The system MUST track cumulative token usage during the run.

#### AC-062 — Budget Visibility
The user MUST be able to observe budget-related run state in the UI.

#### AC-063 — Budget Warning
The product SHOULD provide meaningful budget warnings before hard budget exhaustion where thresholds make that practical.

#### AC-064 — Budget-Hit Control
If the budget is hit, the system MUST stop scheduling new normal rounds and enter controlled finalization if meaningful finalization remains possible.

#### AC-065 — Budget-Capped Completion Classification
If finalization succeeds after budget hit, the terminal run classification MUST be `CompletedBudgetCapped` or its exact canonical equivalent defined in the document.

#### AC-066 — No Hidden Budget Overrun
The system MUST NOT continue normal round scheduling as if no cap exists once budget-hit logic is active.

#### AC-067 — Hard Two-Hour Duration Cap
Each run MUST be bounded by a hard two-hour duration cap.

#### AC-068 — Duration-Cap Control
If the duration cap is reached, the system MUST stop scheduling new normal rounds and choose controlled finalization or truthful terminal failure.

#### AC-069 — No False Full Completion After Truncation
A budget- or duration-capped run MUST NOT be presented as if all originally planned rounds completed if they did not.

---

### 14.12 Active Run Visibility Acceptance Criteria

#### AC-070 — Live Run Surface
During an active run, the user MUST see a run-monitoring surface rather than only a static configuration screen.

#### AC-071 — Phase Visibility
The user MUST be able to distinguish meaningful phases such as:
- ingesting
- generating personas
- active debate
- paused/degraded state
- finalizing
- cleaning

#### AC-072 — Incremental Event Visibility
The system MUST render meaningful run progress incrementally rather than withholding all visibility until the end.

#### AC-073 — Transcript/Event Chronology
The transcript/event feed MUST preserve chronological order in a way that the user can follow.

#### AC-074 — No Static Black-Box During Active Debate
During active debate, the user MUST NOT be forced to rely solely on an undifferentiated loading indicator if meaningful progress events exist.

---

### 14.13 Interruption, Retry, and Degradation Acceptance Criteria

#### AC-075 — Rate-Limit Degradation Support
The product MUST support a truthful paused/degraded posture when provider rate limits interrupt normal progress.

#### AC-076 — Network Degradation Support
The product MUST support a truthful paused/degraded posture when network/provider reachability interrupts normal progress.

#### AC-077 — Bounded Retry Logic
Retry behavior for retryable provider/network issues MUST be bounded.

#### AC-078 — Retry-vs-Failure Distinction
Where the product knows the distinction, it MUST differentiate retryable degraded conditions from terminal failure.

#### AC-079 — Finalize-vs-Fail Truthfulness
If interruption leads to controlled finalization rather than failure, the final state and UI messaging MUST reflect constrained completion rather than normal full completion.

#### AC-080 — Shell Stability Under Provider Failure
Ordinary provider/network degradation MUST NOT by itself crash the app shell.

---

### 14.14 Cancellation and Shutdown Acceptance Criteria

#### AC-081 — Cancel Control Availability
The UI MUST provide a cancellation path when the run is in a cancelable state.

#### AC-082 — Controlled Cancellation
When cancellation is accepted, the system MUST stop normal forward run progression and transition into truthful cancellation handling.

#### AC-083 — No Cancel-as-Completion
A canceled run MUST NOT be mislabeled as successful completion unless the product explicitly defines and truthfully satisfies a valid output path for that case.

#### AC-084 — Graceful-Then-Forceful Shutdown
On normal app close affecting an active run, the shell MUST attempt graceful engine shutdown first and use forceful termination only when required by policy.

#### AC-085 — No Hidden Post-Cancel Debate
After cancellation is accepted, the run MUST NOT continue normal round scheduling in the background.

---

### 14.15 Report Generation Acceptance Criteria

#### AC-086 — Report Requirement
A successful or valid constrained-completion run MUST generate a structured PDF report.

#### AC-087 — Required Report Sections
The report MUST contain all required sections:
- Executive Summary
- Market Viability Score
- Ranked Risks
- Strongest Pro Arguments
- Strongest Anti Arguments
- Cost and Token Summary
- Transcript Appendix

#### AC-088 — Report Truthfulness
The report MUST reflect only actual completed run material and MUST NOT invent nonexistent rounds or debate content.

#### AC-089 — Report Availability Truth
The UI MUST show report availability only when the report has actually been generated and is accessible.

#### AC-090 — No Placeholder Success Report
The product MUST NOT output an empty or materially placeholder PDF and present it as the intended final report.

#### AC-091 — Cost Summary Integrity
The report’s token/cost summary MUST reflect tracked usage rather than invented totals for work that did not occur.

---

### 14.16 Cleanup and Data-Lifecycle Acceptance Criteria

#### AC-092 — Cleanup Attempt Requirement
After every terminal run path, the system MUST attempt cleanup of transient run-local artifacts according to policy.

#### AC-093 — Cleanup Distinction Requirement
The product MUST preserve the distinction between:
- run outcome truth
- cleanup outcome truth

#### AC-094 — Cleanup Failure Visibility
If cleanup fails materially, the product MUST not silently imply cleanup success.

#### AC-095 — Startup Janitor Requirement
After abnormal termination, the next app launch MUST attempt janitor cleanup of stale abandoned workspaces.

#### AC-096 — No Crash Resume Requirement
The product MUST NOT attempt crash-resume restoration of interrupted runs in MVP.

#### AC-097 — Report Preservation Requirement
Cleanup MUST NOT delete the intentionally exported final report if it resides outside the transient workspace boundary.

#### AC-098 — No Cross-Run Working-Data Reuse
Run-local working data MUST NOT persist as active semantic input to later runs.

---

### 14.17 Security, Privacy, and Storage Acceptance Criteria

#### AC-099 — Localhost Trust Boundary
The local backend MUST not assume localhost traffic is inherently trusted and MUST require the approved local app trust mechanism for protected routes.

#### AC-100 — No Plaintext Secret Leakage
Provider secrets MUST NOT be intentionally persisted in plaintext config, ordinary workspace files, or routine diagnostics.

#### AC-101 — Runtime Integrity Block
Simulation execution MUST be blocked if runtime integrity checks fail.

#### AC-102 — Update Integrity Verification
The product MUST verify update artifacts according to the approved updater trust model before application.

#### AC-103 — Local-Data-First Compliance
The product MUST NOT intentionally create a vendor-hosted persistent simulation archive of seed documents, transcripts, or run memory in MVP.

#### AC-104 — Residual-Risk Honesty
The product’s behavior and messaging MUST remain consistent with best-effort cleanup and residual local-risk disclosure rather than absolute anti-forensic guarantees.

---

### 14.18 UI/UX Acceptance Criteria

#### AC-105 — Non-Expert Operability
A non-expert user MUST be able to understand the core app flow without needing to manage ports, process trees, vector databases, or provider-retry internals.

#### AC-106 — Disabled-State Clarity
If a key action is unavailable due to missing prerequisites or invalid lifecycle state, the UI MUST clearly indicate that the action is unavailable and why.

#### AC-107 — Empty-State Clarity
If required setup elements are missing, the UI MUST present clear empty-state guidance rather than a blank or confusing surface.

#### AC-108 — Honest Loading States
Loading states MUST correspond to real work and MUST NOT continue indefinitely without truthful progress or failure feedback when the system knows more.

#### AC-109 — Cleanup and Completion Distinction in UX
The UI MUST preserve the truthful distinction between successful report generation and failed cleanup where both are relevant.

#### AC-110 — No Scope-Inflating UI
The UI MUST NOT imply support for out-of-scope capabilities such as multilingual runs, crash resume, enterprise fleet control, or offline simulation.

---

### 14.19 Logging and Diagnostics Acceptance Criteria

#### AC-111 — Minimal Diagnostics
The product MUST keep diagnostics within the approved minimal operational scope.

#### AC-112 — No Secret Logging
Production diagnostics MUST NOT intentionally contain provider API keys.

#### AC-113 — No Routine Full Transcript Logging
The product MUST NOT treat full transcript capture as ordinary production diagnostics.

#### AC-114 — No Routine Full Seed Logging
The product MUST NOT treat full seed-document content as ordinary production diagnostics.

#### AC-115 — Lifecycle Traceability in Diagnostics
Diagnostics SHOULD preserve enough canonical lifecycle information to support QA and debugging of real state transitions.

---

### 14.20 State and Interface Acceptance Criteria

#### AC-116 — Canonical State Fidelity
The implementation MUST honor the canonical state-machine model defined in Section 7.

#### AC-117 — No Invalid Start Without Eligible Preflight
The system MUST NOT enter active run execution without a current run-start-eligible Preflight state.

#### AC-118 — Ordered Event Interface
The events interface MUST preserve ordered sequence behavior suitable for incremental UI synchronization.

#### AC-119 — Status/Event Consistency
The status interface and event interface MUST remain semantically consistent about run truth.

#### AC-120 — Structured Error Responses
Known interface failures MUST be returned through structured error behavior rather than only vague generic failure text where structured classification is possible.

---

### 14.21 Commercial Backend Acceptance Criteria

#### AC-121 — Durable License Ledger
The commercial backend MUST durably store license truth separate from the local app.

#### AC-122 — Durable Activation Ledger
The commercial backend MUST durably store device activation truth separate from the local app.

#### AC-123 — Payment Event Traceability
Commercial backend processing MUST maintain traceability from payment events to resulting entitlement changes.

#### AC-124 — Activation Slot Enforcement
The commercial backend MUST enforce the two-device limit consistently.

#### AC-125 — JWT Device Binding
The issued activation token MUST bind to the activated device identity according to the approved device-binding model.

#### AC-126 — No Simulation Persistence in Commercial Backend
The commercial backend MUST NOT become the durable persistence layer for simulation transcripts, seed documents, or run memory in MVP.

---

### 14.22 Acceptance-Criteria Summary

#### AC-127 — Section Summary Rule
The MVP is acceptable only if it satisfies all of the following at a minimum:
- correct platform scope
- valid licensing and activation behavior
- runtime hydration and integrity behavior
- secure provider-key handling
- deterministic Preflight gating
- moderated bounded debate behavior
- bounded memory, budget, and duration behavior
- truthful live state visibility
- truthful interruption and cancellation handling
- truthful report generation
- best-effort cleanup with truthful cleanup outcomes
- secure and bounded local/backend interfaces
- durable and bounded commercial backend behavior

#### AC-128 — No Selective Completion Rule
The product must not be considered complete merely because the happy path works once.
Completion requires honoring the boundedness, truthfulness, lifecycle, storage, and commercial rules of this document.

#### AC-129 — Section Boundary Rule
This section defines verification targets only.
It does not add new product scope, new architecture, or new lifecycle meaning beyond what earlier sections already established.

---

## 15. Red-Team Risks and Known Weaknesses

### 15.1 Section Purpose
This section explicitly records the known weaknesses, residual risks, attack surfaces, and accepted MVP tradeoffs of the product.

This section exists to ensure that:
- the project remains honest about its limitations
- engineering and QA do not confuse accepted tradeoffs with accidental omissions
- AI coding agents do not “fix” accepted MVP tradeoffs by inventing unapproved scope
- red-team review has an explicit map of what to inspect and what is already acknowledged as vulnerable or limited

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- all earlier sections that define actual system behavior

This section does not override earlier guarantees.
It documents the places where guarantees are intentionally limited, incomplete, or vulnerable.

---

### 15.2 Risk Philosophy

#### RISK-001 — Honest Limitation Rule
The product must explicitly acknowledge its known limitations rather than hiding them behind optimistic language.

#### RISK-002 — MVP Tradeoff Rule
A known weakness is not automatically a defect if it is:
- explicitly acknowledged
- consistent with the MVP scope
- not falsely marketed as solved

#### RISK-003 — No Risk-as-Excuse Rule
The existence of a known risk does not excuse failure to meet already-approved requirements.
This section documents accepted boundaries, not permission to violate the spec.

#### RISK-004 — Red-Team Priority Rule
Risks in this section are intended to guide focused attack, failure, and truthfulness review.

---

### 15.3 Commercial and Licensing Risks

#### RISK-005 — Local Clock Spoofing Risk
Because offline post-activation validation depends in part on local device time, a user may manipulate the local clock to extend the apparent validity of a time-bounded entitlement.

#### RISK-006 — Best-Effort DRM Risk
The licensing model is a practical anti-casual-piracy model, not undefeatable DRM.
A determined reverse engineer may patch, bypass, or alter local validation behavior.

#### RISK-007 — Binary Patch Risk
A sophisticated attacker may attempt to patch the local application so that entitlement or activation checks always return success.

#### RISK-008 — Device-Fingerprint Stability Risk
Device-binding logic may be imperfect under hardware changes, OS changes, or unusual machine environments, potentially causing false mismatch or re-activation friction.

#### RISK-009 — Activation Slot Abuse Risk
Coordinated users may attempt to share activations or manipulate the two-device limit through token copying or other circumvention attempts.

#### RISK-010 — Commercial Backend Availability Risk
Activation, runtime hydration authorization, or update support may be blocked if the commercial backend is unavailable, even though simulation execution itself is local once prerequisites are met.

#### RISK-011 — Accepted Licensing Boundary
These risks are accepted in MVP so long as the product does not falsely claim tamper-proof licensing.

---

### 15.4 Local Security and Secret-Handling Risks

#### RISK-012 — Memory Exposure Risk
Even if provider secrets are not stored in plaintext on disk, they may exist in process memory during active use and could be exposed through local memory inspection by hostile software or an advanced user.

#### RISK-013 — Localhost Abuse Risk
A malicious local process may attempt to access or interfere with the local backend if local trust boundaries are weak or misimplemented.

#### RISK-014 — App-Session Token Theft Risk
If the local app-session authorization token is exposed in memory or logs through poor implementation, a local attacker may attempt unauthorized local backend calls.

#### RISK-015 — OS-Native Secure Storage Dependence Risk
The product’s secret-handling quality depends on correct use of OS-native secure storage and does not magically eliminate local-host compromise risk.

#### RISK-016 — Accepted Secret Boundary
The MVP accepts that best-effort secret minimization is not equal to absolute immunity against local hostile inspection.

---

### 15.5 Privacy and Data-Residue Risks

#### RISK-017 — OS Indexing Residue Risk
OS-level indexing or search subsystems may retain traces of transient workspace content even after the source files are deleted.

#### RISK-018 — Filesystem Residue Risk
Filesystem journaling, SSD behavior, snapshots, backups, or related storage-layer behaviors may preserve traces of transient files beyond application control.

#### RISK-019 — Swap/Pagefile Residue Risk
Sensitive text or secrets may leave traces in swap/pagefile or other OS-level memory spill mechanisms outside application control.

#### RISK-020 — Abnormal-Termination Residue Risk
If the app or OS terminates abnormally before cleanup, run-local data may remain until janitor cleanup occurs or may remain longer if cleanup later fails.

#### RISK-021 — Backup/Monitoring Software Risk
Third-party backup, endpoint monitoring, AV, or forensic software on the user’s system may observe or retain run-local artifacts outside the app’s control.

#### RISK-022 — Accepted Privacy Boundary
These risks are accepted in MVP so long as the product remains honest about best-effort cleanup and does not claim perfect erasure.

---

### 15.6 Simulation and Model-Behavior Risks

#### RISK-023 — Persona Quality Drift Risk
Runtime-generated personas may become repetitive, bland, or poorly targeted if seed interpretation or prompting quality is weak.

#### RISK-024 — Moderator Bias Risk
The Moderator Agent may over-select certain perspectives and underrepresent others, creating skewed debate dynamics.

#### RISK-025 — Score Interpretability Risk
Users may over-trust the Market Viability Score as objective truth rather than model-mediated synthesis.

#### RISK-026 — Prompt/Context Compression Risk
Bounded memory strategies may compress or omit details that a user assumes the system still “remembers” in full.

#### RISK-027 — Context-Window Pressure Risk
Large or complex seeds plus long runs may pressure context boundaries even under bounded memory strategies.

#### RISK-028 — Provider Variability Risk
Different provider behaviors, latency, output shape, or transient availability may cause run-to-run variance.

#### RISK-029 — Output Variability Risk
Even under the same general input, identical-seeming runs may not produce identical final debates or scores because model-driven behavior is probabilistic or provider-variable.

#### RISK-030 — Accepted Simulation Boundary
These risks are accepted in MVP so long as the product does not claim deterministic truth or guaranteed predictive accuracy.

---

### 15.7 Cost and Performance Risks

#### RISK-031 — Estimation Inaccuracy Risk
Preflight token and duration estimates may be directionally useful but still materially imperfect versus real execution.

#### RISK-032 — Budget-Hit Earlier-Than-Expected Risk
Actual token use may exceed estimates sooner than the user expects, causing earlier controlled finalization.

#### RISK-033 — Long-Transcript UI Pressure Risk
If transcript rendering is not correctly bounded, long runs may degrade frontend responsiveness or memory use.

#### RISK-034 — Provider Latency Risk
Slow provider responses may make the run feel stalled even if the architecture is functioning correctly.

#### RISK-035 — Thin-Installer Friction Risk
First-launch runtime hydration may add visible friction or failure opportunities compared with shipping a very large all-in-one installer.

#### RISK-036 — Accepted Cost/Performance Boundary
These risks are accepted in MVP so long as boundedness, visibility, and truthfulness rules remain intact.

---

### 15.8 Reliability and Lifecycle Risks

#### RISK-037 — Zombie Process Risk
If process supervision is poorly implemented, the Python engine could outlive the app shell after abnormal paths.

#### RISK-038 — Cleanup Race Risk
Workspace deletion may fail due to file locks, late process exit, OS timing, or platform-specific filesystem behavior.

#### RISK-039 — Next-Launch Janitor Failure Risk
Startup janitor may fail to remove abandoned workspaces because of permissions, locks, or corrupted state.

#### RISK-040 — No-Resume User Friction Risk
Users may perceive the no-resume policy as painful after a long interrupted run, even though it is an intentional MVP tradeoff.

#### RISK-041 — Partial-Finalization Failure Risk
A run may gather useful debate material but still fail during finalization or report generation, leaving the user without a completed artifact.

#### RISK-042 — Accepted Reliability Boundary
These risks are accepted in MVP so long as the product remains truthful about cleanup, failure, and no-resume limitations.

---

### 15.9 Interface and State-Truth Risks

#### RISK-043 — State Desynchronization Risk
The status route, event stream, UI view state, and actual engine state may diverge if sequencing or lifecycle implementation is weak.

#### RISK-044 — Event Ordering Risk
Poor event sequencing could cause transcript confusion, duplicate rendering, or misleading UI chronology.

#### RISK-045 — Hidden-Downgrade Risk
If UI and backend contract discipline is weak, the system may accidentally execute a smaller effective plan than the user believes was approved.

#### RISK-046 — Error-Classification Risk
Poor error taxonomy may cause policy denial, technical failure, retryable degradation, and terminal failure to be conflated into one confusing “error” bucket.

#### RISK-047 — Accepted Interface Boundary
These risks are accepted only as things to defend against, not as permitted outcomes.
They directly threaten product truthfulness and should be considered high-priority red-team targets.

---

### 15.10 Update and Distribution Risks

#### RISK-048 — Runtime Artifact Abuse Risk
Authorized runtime download URLs or delivery paths may be abused if poorly scoped or poorly rate-limited.

#### RISK-049 — Update Trust Risk
If update verification is weak or misimplemented, unsafe artifacts could be presented as installable.

#### RISK-050 — Version Mismatch Risk
A mismatched shell/runtime combination may behave incorrectly if version-compatibility checks are weak.

#### RISK-051 — Supply-Chain Friction Risk
Signed builds, notarization, runtime packaging, and update verification add delivery complexity and operational failure points.

#### RISK-052 — Accepted Distribution Boundary
These risks are accepted in MVP only if runtime and update integrity rules remain enforced and truthfully surfaced.

---

### 15.11 UI/UX and User-Interpretation Risks

#### RISK-053 — False Confidence Risk
Users may interpret a polished UI as evidence of stronger truth, security, or prediction certainty than the product actually provides.

#### RISK-054 — Constraint Blindness Risk
Users may ignore or misunderstand budget, duration, English-only, or no-resume constraints unless the UI surfaces them clearly.

#### RISK-055 — Completion Misinterpretation Risk
Users may incorrectly assume that any report means “full run completed” unless constrained-completion states are clearly distinguished.

#### RISK-056 — Over-Simplification Risk
An overly simplified UI may hide important lifecycle truth and lead to false assumptions about what the system is actually doing.

#### RISK-057 — Accepted UX Boundary
These are accepted UX risks only if the implemented product still satisfies the truthfulness rules defined in earlier sections.

---

### 15.12 AI-Led Development Risks

#### RISK-058 — Unauthorized Feature Invention Risk
AI coding agents may invent adjacent features not specified in the brief, especially around analytics, persistence, collaboration, or advanced controls.

#### RISK-059 — Layer-Boundary Drift Risk
AI coding agents may place logic in the wrong layer, such as:
- putting shell responsibilities in the frontend
- putting commercial logic in the Python engine
- adding hidden persistence in the wrong place

#### RISK-060 — Spec-Override-by-Pattern Risk
AI coding agents may substitute generic SaaS or framework conventions for the project’s specific documented rules.

#### RISK-061 — Silent Scope Expansion Risk
AI coding agents may add “helpful” persistence, resume, analytics, or multi-document support that the product never approved.

#### RISK-062 — Accepted AI-Development Boundary
These risks are precisely why the project requires a hierarchical deterministic source-of-truth document.
They are not acceptable if they materialize unchallenged in implementation.

---

### 15.13 Red-Team Priority Classes

#### RISK-063 — Critical Priority Class
The following risk classes are red-team critical because they threaten product truth, secrets, or architectural integrity:
- local auth-boundary weakness
- secret leakage
- hidden persistence
- false cleanup claims
- state desynchronization
- unsafe update acceptance
- runtime-integrity bypass
- activation-limit bypass

#### RISK-064 — High Priority Class
The following risk classes are high priority because they threaten boundedness, usability, or trust:
- budget enforcement drift
- hidden downgrade behavior
- finalization dishonesty
- report fabrication
- transcript/event ordering faults
- cleanup failure concealment

#### RISK-065 — Medium Priority Class
The following risk classes are medium priority because they primarily affect UX quality, estimate quality, or operator friction:
- latency variability
- estimate inaccuracy
- no-resume frustration
- hydration friction
- UI constraint visibility weakness

---

### 15.14 Explicitly Accepted MVP Tradeoffs

#### RISK-066 — Accepted Tradeoff: No Crash Resume
The product explicitly accepts no crash-resume support in MVP.

#### RISK-067 — Accepted Tradeoff: English Only
The product explicitly accepts English-only seed support in MVP.

#### RISK-068 — Accepted Tradeoff: Best-Effort Cleanup
The product explicitly accepts best-effort cleanup instead of forensic-grade deletion.

#### RISK-069 — Accepted Tradeoff: Practical DRM Only
The product explicitly accepts practical anti-casual-piracy measures rather than absolute anti-tamper guarantees.

#### RISK-070 — Accepted Tradeoff: Thin Installer + Hydration
The product explicitly accepts first-launch runtime hydration complexity in exchange for a lighter installer and better distribution separation.

#### RISK-071 — Accepted Tradeoff: Bounded Moderated Debate
The product explicitly accepts a controlled moderated 3–5 speaker-per-round design rather than maximal swarm fanout.

#### RISK-072 — Accepted Tradeoff: Controlled Partial Completion
The product explicitly accepts constrained but truthful report generation after budget or duration truncation when meaningful output still exists.

---

### 15.15 Forbidden Misuse of This Risk Section

#### RISK-073 — No Risk-Washing Rule
This section must not be used to excuse violations of the approved requirements.

#### RISK-074 — No Hidden-Scope Rule
This section must not be used to smuggle in new scope under the label of “risk mitigation” unless the scope change is explicitly approved upstream.

#### RISK-075 — No False-Claim Override Rule
This section must not be read as permission to make stronger claims than earlier sections allow.

#### RISK-076 — No “Known Weakness Therefore Not Important” Rule
A known weakness remains important even when accepted as an MVP tradeoff.
Acceptance means honest scoping, not neglect.

---

### 15.16 Section Summary

#### RISK-077 — Section Summary Rule
At the risk-boundary level, the MVP explicitly acknowledges limitations in:
- anti-tamper enforcement
- local memory secrecy
- OS-level residue control
- probabilistic simulation quality
- estimate accuracy
- no-resume behavior
- process-cleanup certainty
- interface synchronization risk
- runtime/update distribution complexity
- user over-trust risk
- AI-led implementation drift risk

These risks are acceptable only within the truthful, bounded, and non-exaggerated product posture defined by earlier sections.

#### RISK-078 — Risk-Truth Rule
The product is acceptable with known weaknesses only if those weaknesses are:
- honestly represented
- not contradicted by stronger claims
- not allowed to silently violate mandatory requirements

#### RISK-079 — Section Boundary Rule
This section defines:
- known weaknesses
- red-team target areas
- accepted MVP tradeoffs
- forbidden misuse of risk acknowledgments

This section does not redefine product behavior, architecture, or acceptance criteria.
It exists to make the project’s vulnerability and tradeoff boundaries explicit.

---

## 16. Implementation Guardrails for AI Coding Agents

### 16.1 Section Purpose
This section defines the operating rules that any AI coding agent must follow when using Document A as implementation context.

This section exists because:
- the project owner is relying heavily on AI-led development
- AI coding agents frequently overgeneralize from frameworks, defaults, or common SaaS patterns
- AI coding agents may invent features, layers, persistence, or abstractions not approved in the spec
- a deterministic source-of-truth document is only useful if the AI is explicitly told how to obey it

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- Section 3 — Glossary and Canonical Terminology
- all earlier sections of Document A

This section does not override earlier product rules.
It operationalizes how AI coding agents must behave while interpreting and implementing them.

---

### 16.2 Guardrail Philosophy

#### AI-GR-001 — Source-of-Truth Obedience Rule
An AI coding agent must treat Document A as the highest implementation context authority for the project.

#### AI-GR-002 — No Autonomous Product Design Rule
The AI coding agent is not authorized to redesign the product while implementing it.

#### AI-GR-003 — Determinism Preference Rule
Where multiple implementation approaches are available, the AI coding agent should prefer the solution that is:
- simpler
- clearer
- more traceable
- more deterministic
- more consistent with the document’s authority structure

#### AI-GR-004 — Truth Preservation Rule
The AI coding agent must preserve the product’s truthfulness constraints, especially around:
- completion state
- cleanup state
- security/privacy claims
- offline limitations
- budget and truncation behavior
- no-resume behavior

---

### 16.3 Scope Discipline Rules

#### AI-GR-005 — No Unauthorized Feature Invention
The AI coding agent must not invent features that are not explicitly authorized by Document A.

This includes, but is not limited to:
- multilingual seed support
- crash resume
- multi-document runs
- team collaboration
- hosted simulation execution
- analytics dashboards across runs
- enterprise admin surfaces
- advanced prompt editors
- agent-roster authoring systems
- offline local-model inference

#### AI-GR-006 — No “Helpful” Scope Expansion Rule
The AI coding agent must not add adjacent capabilities merely because they seem useful, common, or modern.

#### AI-GR-007 — MVP Boundary Preservation Rule
If a capability is out of scope in Document A, the AI coding agent must preserve that boundary even if a framework or library makes the feature easy to add.

#### AI-GR-008 — Deferred Means Deferred Rule
If a feature is deferred or explicitly not an MVP goal, the AI coding agent must not partially implement it “for future readiness” in a way that changes current product behavior without approval.

---

### 16.4 Authority and Conflict Obedience Rules

#### AI-GR-009 — Precedence Obedience Rule
If the AI coding agent detects conflicting instructions across project materials, it must obey the precedence hierarchy defined in Section 0 and the constitutional rules in Section 1.

#### AI-GR-010 — Constitution Supremacy Rule
If a lower-level artifact, prompt, note, example, or code stub conflicts with the Project Constitution, the constitution wins.

#### AI-GR-011 — Glossary Discipline Rule
The AI coding agent must use canonical terminology exactly as defined in Section 3.

#### AI-GR-012 — No Terminology Drift Rule
The AI coding agent must not replace canonical project terms with invented synonyms that change meaning or blur scope.

#### AI-GR-013 — Earlier Section Protection Rule
The AI coding agent must not reinterpret a locked earlier section in order to justify a convenient implementation shortcut.

---

### 16.5 Ambiguity Handling Rules

#### AI-GR-014 — No Silent Guessing Rule
If a behavior is materially important and not fully specified, the AI coding agent must not silently invent a definitive product rule and present it as if the spec already approved it.

#### AI-GR-015 — Safe-Assumption Rule
If a gap must be handled temporarily to continue implementation, the AI coding agent should choose the assumption that is:
- narrowest in scope
- safest operationally
- least likely to create hidden persistence, extra surface area, or misleading user-visible behavior

#### AI-GR-016 — Mark Assumptions Rule
If the AI coding agent must make an assumption during implementation planning, the assumption should be explicitly labeled as an assumption or temporary implementation choice rather than disguised as settled product truth.

#### AI-GR-017 — No Ambiguity Masking Rule
The AI coding agent must not hide ambiguity by overconfidently generating product behavior not explicitly authorized.

---

### 16.6 Layer-Boundary Rules

#### AI-GR-018 — Frontend Boundary Rule
The AI coding agent must not place shell-owned privileged responsibilities into the frontend.

#### AI-GR-019 — Shell Boundary Rule
The AI coding agent must preserve the shell as the owner of:
- privileged local orchestration
- secure storage mediation
- process control
- runtime hydration coordination
- cleanup coordination
- app lifecycle supervision

#### AI-GR-020 — Engine Boundary Rule
The AI coding agent must preserve the Python engine as the owner of:
- Preflight execution
- persona generation
- moderated run logic
- bounded memory behavior
- finalization logic
- run-local event production

#### AI-GR-021 — Commercial Backend Boundary Rule
The AI coding agent must not move simulation execution or transcript persistence into the commercial backend in MVP.

#### AI-GR-022 — No Hidden Layer Bleed Rule
The AI coding agent must not blur responsibilities across layers in ways that make truth, cleanup, or security harder to reason about.

---

### 16.7 State and Lifecycle Rules for AI Agents

#### AI-GR-023 — Canonical State Fidelity Rule
The AI coding agent must implement lifecycle logic consistent with Section 7 and must not invent undocumented state shortcuts.

#### AI-GR-024 — No Invalid Transition Rule
The AI coding agent must not implement invalid lifecycle transitions simply because they make the code easier.

#### AI-GR-025 — No Resume Drift Rule
The AI coding agent must not introduce crash-resume logic or implicit recovery behavior into the run lifecycle without explicit approval.

#### AI-GR-026 — Cleanup Distinction Rule
The AI coding agent must preserve the distinction between:
- run completion
- cleanup success
- app shutdown
and must not collapse them into one simplistic “done” path.

#### AI-GR-027 — Finalization Integrity Rule
The AI coding agent must preserve finalization as a distinct state and must not treat report generation as an afterthought hidden inside generic completion logic.

---

### 16.8 Data and Persistence Rules for AI Agents

#### AI-GR-028 — No Hidden Persistence Rule
The AI coding agent must not introduce undeclared persistent storage for:
- seed documents
- extracted text
- run memory
- vector stores
- transcripts
- persona sets
- report intermediates
unless Document A explicitly authorizes persistence for that category

#### AI-GR-029 — Secret Storage Obedience Rule
The AI coding agent must store provider secrets only through approved OS-native secure-storage pathways.

#### AI-GR-030 — No Secret Leakage Rule
The AI coding agent must not place secrets into:
- plaintext files
- transient workspaces
- routine diagnostics
- report content
- transcript artifacts

#### AI-GR-031 — Report Boundary Rule
The AI coding agent must preserve the distinction between:
- transient run-local workspace artifacts
- intentionally persistent exported reports

#### AI-GR-032 — No Cross-Run Memory Rule
The AI coding agent must not cause one run’s working memory to influence later runs as active semantic input.

---

### 16.9 Truthfulness and Messaging Rules

#### AI-GR-033 — No False Success Rule
The AI coding agent must not implement user-visible logic that labels a run as completed if:
- finalization failed
- report generation failed
- or only a canceled/failed path occurred

#### AI-GR-034 — No Fake Cleanup Rule
The AI coding agent must not display cleanup success when cleanup actually failed.

#### AI-GR-035 — No Fake Liveness Rule
The AI coding agent must not create deceptive UI progress behavior that suggests active debate progress when the system is actually blocked, paused, or failed.

#### AI-GR-036 — No Stronger Claims Rule
The AI coding agent must not introduce copy, labels, comments, or user-facing text that strengthens the product’s claims beyond what Document A allows.

#### AI-GR-037 — Budget Honesty Rule
The AI coding agent must preserve truthful user-visible treatment of:
- budget warnings
- budget hit
- budget-capped finalization
- constrained completion

#### AI-GR-038 — No Deterministic-Truth Marketing Rule
The AI coding agent must not present the Market Viability Score as objective certainty or guaranteed market truth.

---

### 16.10 Error and Failure Handling Rules

#### AI-GR-039 — Structured Failure Preference Rule
The AI coding agent should prefer explicit categorized errors over vague catch-all errors when the true category is known.

#### AI-GR-040 — No Silent Failure Rule
The AI coding agent must not allow materially significant failures to disappear without user-visible consequence when the system can detect them.

#### AI-GR-041 — Retry Bound Rule
The AI coding agent must keep retry logic bounded and must not create infinite-retry loops.

#### AI-GR-042 — Rejection-vs-Error Discipline Rule
The AI coding agent must preserve the distinction between:
- rejection due to policy or feasibility
- technical failure
- degraded temporary pause
- terminal failure

#### AI-GR-043 — Fail-Honestly Rule
When the product cannot produce a truthful valid report path, the AI coding agent must prefer honest failure over fabricated completion.

---

### 16.11 Interface and API Rules for AI Agents

#### AI-GR-044 — No Undeclared Endpoint Rule
The AI coding agent must not make core product behavior depend on hidden undeclared endpoints outside the approved route set.

#### AI-GR-045 — Auth Boundary Obedience Rule
The AI coding agent must not bypass the approved local app trust mechanism for protected local backend routes.

#### AI-GR-046 — Status/Event Consistency Rule
The AI coding agent must keep event emission, status reporting, and UI state semantically aligned.

#### AI-GR-047 — No Backdoor Debug Surface Rule
The AI coding agent must not leave behind a hidden admin/debug interface that bypasses lifecycle, cleanup, activation, or auth rules in production behavior.

---

### 16.12 UX Implementation Rules for AI Agents

#### AI-GR-048 — No Over-Engineer UX Rule
The AI coding agent must not turn the MVP into a highly parameterized control panel that exposes internal infrastructure choices to the user.

#### AI-GR-049 — Clarity-over-Cleverness Rule
The AI coding agent should prefer clearer user-visible state communication over clever compact UI patterns that obscure lifecycle truth.

#### AI-GR-050 — Disabled-State Obedience Rule
The AI coding agent must implement disabled and gated actions consistently with lifecycle truth.

#### AI-GR-051 — No Scope-Inflating UI Rule
The AI coding agent must not create UI affordances that imply unsupported product capabilities.

---

### 16.13 Engineering Quality Rules for AI Agents

#### AI-GR-052 — Maintainability Preference Rule
The AI coding agent should prefer maintainable, auditable code organization over opaque abstraction layers that hide product behavior.

#### AI-GR-053 — Traceability Preference Rule
The AI coding agent should structure code and implementation notes so they can be traced back to:
- behavior rules
- state-machine rules
- architecture rules
- data-policy rules
- acceptance criteria

#### AI-GR-054 — No Framework-Default Override Rule
The AI coding agent must not allow framework defaults to override project-specific requirements simply because the default behavior is easier.

#### AI-GR-055 — No “Works Once” Standard Rule
The AI coding agent must not treat a one-time happy-path demo as sufficient implementation quality.

---

### 16.14 AI Guardrail Summary

#### AI-GR-056 — Section Summary Rule
An AI coding agent using this project brief must behave as an obedient implementer of a hierarchical deterministic source-of-truth document. It must not redesign the product, widen scope, weaken truthfulness, strengthen claims, blur layers, add hidden persistence, bypass lifecycle gates, or invent unsupported capabilities. Where ambiguity exists, it must choose the narrowest safe interpretation and preserve traceability to the document.

#### AI-GR-057 — Guardrail Consistency Rule
All future AI-generated plans, code, and prompts must remain consistent with these guardrails unless Document A is explicitly revised.

#### AI-GR-058 — Section Boundary Rule
This section defines implementation obedience rules for AI coding agents.
It does not add new product features, new architecture, or new commercial behavior.

---

## 17. Open Questions and Deferred Decisions

### 17.1 Section Purpose
This section records items that are not yet fully locked or that are intentionally deferred beyond MVP.

This section exists to ensure that:
- unresolved items are visible rather than hidden
- AI coding agents do not assume unresolved items were accidentally omitted
- future planning work knows which decisions remain open
- deferred scope is tracked without contaminating the current MVP contract

This section is governed by:
- Section 0 — Document Control
- Section 1 — Project Constitution
- all earlier locked sections

This section does not override locked decisions.
If an item conflicts with a locked earlier rule, the locked earlier rule wins.

---

### 17.2 Deferred-Decision Philosophy

#### DEFER-001 — Deferred Means Not Yet Approved Rule
A deferred item is not approved scope.
It is a tracked future possibility or unresolved design item.

#### DEFER-002 — Open Question Transparency Rule
If something is not fully decided, it should be recorded here rather than silently guessed by downstream artifacts.

#### DEFER-003 — No Hidden MVP Mutation Rule
No deferred item should be treated as active MVP scope unless Document A is revised to lock it in.

#### DEFER-004 — Future Planning Boundary Rule
Items in this section may inform Document B — Master Execution Plan or later versions of Document A, but they do not automatically become implementation requirements.

---

### 17.3 Explicitly Deferred Beyond MVP

#### DEFER-005 — Enterprise Deployment Deferred
Enterprise IT-managed deployment is deferred beyond MVP.

#### DEFER-006 — Team Collaboration Deferred
Shared team workspaces, collaborative commenting, and multi-user run collaboration are deferred beyond MVP.

#### DEFER-007 — Multilingual Seed Support Deferred
Support for non-English seed documents is deferred beyond MVP.

#### DEFER-008 — Crash Resume Deferred
Crash-resume or stateful interrupted-run recovery is deferred beyond MVP.

#### DEFER-009 — Manual Persona Authoring Deferred
A user-authored static persona/agent design system is deferred beyond MVP.

#### DEFER-010 — Multi-Document Runs Deferred
Multi-document ingestion or comparative multi-source runs are deferred beyond MVP.

#### DEFER-011 — Hosted Simulation Backend Deferred
Vendor-hosted simulation execution is deferred beyond MVP.

#### DEFER-012 — Cross-Run Analytics Deferred
Comparative analytics across prior runs, run history dashboards, or report portfolio analytics are deferred beyond MVP.

#### DEFER-013 — Enterprise Admin Console Deferred
Organization-level license administration, seat assignment, and fleet control are deferred beyond MVP.

#### DEFER-014 — Full Local-Model Inference Deferred
Offline local-model inference is deferred beyond MVP.

#### DEFER-015 — Stronger Anti-Tamper Measures Deferred
More advanced anti-tamper and anti-piracy controls beyond the MVP’s practical DRM model are deferred.

---

### 17.4 Open Product and Design Questions

These are questions that may require a future product decision but are not required to unlock the current MVP contract.

#### DEFER-016 — Future Model Expansion Question
Will the product support additional models beyond the MVP-approved defaults, and if so, what is the model-governance process?

#### DEFER-017 — Device Replacement UX Question
What exact user-facing policy and workflow should govern voluntary device replacement under the two-device activation model, if such a flow is later introduced?

#### DEFER-018 — Report Template Refinement Question
How much visual formatting sophistication should later report versions include beyond the current required content structure?

#### DEFER-019 — Optional Report Export Formats Question
Should later versions support additional export formats such as markdown, HTML, or DOCX, or remain PDF-only for a longer period?

#### DEFER-020 — Post-MVP Collaboration Question
If collaboration is later introduced, what level of shared run data is acceptable without violating the local-data-first philosophy?

#### DEFER-021 — Deeper Persona Controls Question
Should later versions expose limited user controls over persona composition without violating MVP simplicity?

---

### 17.5 Open Technical Questions for Later Evaluation

These questions may matter later but are not required to be fully resolved in Document A to preserve the MVP contract.

#### DEFER-022 — Device Fingerprint Stability Question
What exact fingerprinting strategy best balances:
- stability
- privacy
- spoof resistance
- low false mismatch rates
across supported Windows environments?

#### DEFER-023 — Report Rendering Stack Question
What exact PDF-generation stack or rendering pipeline best satisfies the report contract with acceptable complexity?

#### DEFER-024 — Memory Compression Strategy Refinement Question
What specific bounded memory strategy performs best across supported providers without overcomplicating the MVP?

#### DEFER-025 — Runtime/Update Infrastructure Refinement Question
What exact hosting, signing automation, and release-distribution operations best support the locked MVP split between shell-owned managed runtime-package delivery and the native app updater?

#### DEFER-026 — Advanced Runtime Repair UX Question
How much automated runtime self-repair should be added later without making the startup lifecycle confusing?

#### DEFER-027 — Extended Diagnostic Strategy Question
What additional diagnostic surfaces might be added later without violating the minimized logging policy?

---

### 17.6 Open Policy Questions for Later Evaluation

#### DEFER-028 — License Revocation Policy Question
What exact user-facing policy should apply when a license is revoked after prior activation has already occurred?

#### DEFER-029 — Grace Period Policy Question
Should time-bounded plans ever support an explicit grace period beyond the current bounded MVP assumptions?

#### DEFER-030 — Device Management Self-Service Policy Question
Should users later get a self-service way to inspect and manage their two activation slots?

#### DEFER-031 — Support Escalation Policy Question
What exact support workflows should exist for activation, hydration, cleanup, or runtime-failure edge cases after MVP?

---

### 17.7 Items Explicitly Not Open

To avoid ambiguity, the following are not open questions for MVP and are already locked:

#### DEFER-032 — Windows-Only Scope Locked
Windows-only scope is locked for MVP.

#### DEFER-033 — English-Only Policy Locked
English-only seed support is locked for MVP.

#### DEFER-034 — No Crash Resume Locked
No crash-resume behavior is locked for MVP.

#### DEFER-035 — Best-Effort Cleanup Locked
Best-effort cleanup rather than forensic-grade deletion is locked for MVP.

#### DEFER-036 — Two-Device Activation Limit Locked
The two-device activation model is locked for MVP.

#### DEFER-037 — Moderated 3-to-5 Speaker Model Locked
The Moderator-driven 3–5 speaker-per-round model is locked for MVP.

#### DEFER-038 — Max Token Budget Requirement Locked
The max token budget requirement is locked for MVP.

#### DEFER-039 — Two-Hour Duration Cap Locked
The hard two-hour duration cap is locked for MVP.

#### DEFER-040 — Local-Data-First / Cloud-Inference-Dependent Framing Locked
The canonical product framing is locked for MVP.

---

### 17.8 Deferred-Section Usage Rules

#### DEFER-041 — No Spec Override Rule
This section must not be used to reinterpret or weaken already-locked requirements.

#### DEFER-042 — No Future-Feature Leakage Rule
Deferred items must not leak into the MVP implementation simply because they are documented here.

#### DEFER-043 — Explicit Promotion Rule
If a deferred item is later approved, it must be promoted through an explicit versioned update to Document A and recorded in the Change Log.

#### DEFER-044 — Planning Reference Rule
Document B and future planning documents may reference deferred items, but they must still respect the fact that deferred items are not yet MVP obligations.

---

### 17.9 Section Summary

#### DEFER-045 — Section Summary Rule
This section records future-facing possibilities and unresolved questions without altering the locked MVP contract. It explicitly separates what is deferred from what is already settled, so that AI coding agents and downstream planning documents do not confuse future possibilities with current obligations.

#### DEFER-046 — Section Boundary Rule
This section defines:
- deferred scope
- open questions
- explicitly not-open items
- rules for handling deferred items

It does not authorize implementation of deferred features.

---

## 18. Change Log

### 18.1 Section Purpose
This section records formal revisions to Document A over time.

This section exists to ensure that:
- the document evolves transparently
- locked changes are visible and auditable
- future AI coding agents and human reviewers can tell which version they are using
- authority conflicts can be resolved through explicit version history rather than memory or guesswork

This section is governed by:
- Section 0 — Document Control
- the versioning decisions of the project owner

---

### 18.2 Change-Log Philosophy

#### CHANGE-001 — Explicit Revision Rule
Any material change to Document A must be recorded here.

#### CHANGE-002 — No Silent Revision Rule
A locked section must not be treated as changed unless the change is explicitly reflected in the Change Log.

#### CHANGE-003 — Version Traceability Rule
Each change-log entry must identify:
- document version
- date
- changed sections
- reason for change
- impact summary

#### CHANGE-004 — Authority Preservation Rule
The Change Log is the only approved mechanism for formally changing locked content without rewriting document history invisibly.

---

### 18.3 Change-Log Entry Template

#### CHANGE-005 — Entry Template Rule
Each future change-log entry should follow this structure:

- **Version:**  
- **Date:**  
- **Changed Sections:**  
- **Change Type:** major / moderate / minor  
- **Reason:**  
- **Summary of Change:**  
- **Impact on Existing Implementation:**  
- **Impact on Documents B/C:**  
- **Migration or Re-Validation Required:** yes / no  

---

### 18.4 Initial Draft Record

#### CHANGE-006 — Initial Batch-Constructed Draft Record
The current assembled document has been created in controlled batches to optimize for deterministic drafting and review.

#### CHANGE-007 — Initial Draft Construction Summary
The draft was constructed in the following major batches:
- Batch 1 — Document Control; Project Constitution
- Batch 2 — Executive Product Snapshot; Glossary and Canonical Terminology
- Batch 3 — Product Scope and User Outcomes; User Journeys and Behavioral Narratives
- Batch 4A — Product Behavior Specification Part 1
- Batch 4B — Product Behavior Specification Part 2
- Batch 5A — State Machines Part 1
- Batch 5B — State Machines Part 2
- Batch 6 — System Architecture; Data Lifecycle and Storage Policy
- Batch 7 — API and Interface Contract; Database and Commercial Backend Contract
- Batch 8 — UI/UX Contract; Non-Functional Requirements
- Batch 9 — Acceptance Criteria; Red-Team Risks and Known Weaknesses
- Batch 10 — AI Guardrails; Open Questions and Deferred Decisions; Change Log

---

### 18.5 Current Formal Entries

#### CHANGE-008 — Current Entry
- **Version:** V2.A-Final-Draft-1  
- **Date:** April 14, 2026  
- **Changed Sections:** Sections 0–18 initial assembled draft  
- **Change Type:** major  
- **Reason:** Creation of the first full hierarchical deterministic source-of-truth brief for AI-led implementation  
- **Summary of Change:** Established full MVP constitutional, behavioral, lifecycle, architectural, data, interface, backend, UX, constraint, acceptance, risk, and AI-guardrail structure  
- **Impact on Existing Implementation:** foundational source-of-truth established; future implementation must align to this version  
- **Impact on Documents B/C:** Document B and Document C must be derived from and remain subordinate to this version  
- **Migration or Re-Validation Required:** yes

#### CHANGE-008A — Post-Consolidation Clarification Entry
- **Version:** V2.A-Final-Draft-2  
- **Date:** April 14, 2026  
- **Changed Sections:** top metadata block, Section 1.7, Section 18.5, canonical footer  
- **Change Type:** minor  
- **Reason:** Post-consolidation governance cleanup to make the unified draft explicitly auditable and to clarify the boundary between prohibited run/session resume and permitted runtime-download or runtime-hydration recovery.  
- **Summary of Change:** Added an explicit constitutional clarification that no crash-resume applies only to interrupted runs/sessions and not to pre-run runtime-download or runtime-hydration recovery; added an explicit post-consolidation change-log entry; bumped version metadata and footer accordingly.  
- **Impact on Existing Implementation:** No product-scope change; clarification only.  
- **Impact on Documents B/C:** Document B and Document C should use V2.A-Final-Draft-2 as the governing draft.  
- **Migration or Re-Validation Required:** no

#### CHANGE-008B — Governance Freeze Entry
- **Version:** V2.A-Final-Draft-3  
- **Date:** April 15, 2026  
- **Changed Sections:** top metadata block, Section 3.4, Section 8.15, Section 10.16, Section 10.18, Section 11.11, Section 11.16, Section 13.8, Section 14.7, Section 17.5, Section 18.5, canonical footer  
- **Change Type:** moderate  
- **Reason:** Promote deferred MVP seed-intake constants and the runtime/update delivery split into explicit governing product truth so Phase 5 execution can proceed without guessing and later updater work remains bounded to hardening rather than toolchain selection.  
- **Summary of Change:** Locked supported seed-document inputs to PDF, DOCX, TXT, and Markdown; locked the maximum seed-document size to 10 MiB (10,485,760 bytes); locked the minimum usable extracted text threshold to 1,000 normalized characters; locked non-English rejection to English-confidence below 0.80; documented the minimum document-validation rejection families; froze the MVP split between shell-owned managed runtime-package hydration and the native app updater; and narrowed DEFER-025 to later infrastructure refinement rather than base-strategy selection.  
- **Impact on Existing Implementation:** Downstream implementation must consume these exact seed-intake constants and the locked runtime/update split rather than inferring alternate values or mechanisms.  
- **Impact on Documents B/C:** Document B and Document C must align execution, technical contract, and validation surfaces to V2.A-Final-Draft-3.  
- **Migration or Re-Validation Required:** yes

---

### 18.6 Future Change Rules

#### CHANGE-009 — Major Change Rule
A change is major if it modifies:
- product identity
- supported platforms
- constitutional scope
- lifecycle truth
- data policy boundaries
- security/privacy claim boundaries
- acceptance criteria in a way that changes implementation obligations

#### CHANGE-010 — Moderate Change Rule
A change is moderate if it refines:
- architecture ownership
- interface rules
- report structure
- UI obligations
- deferred-item promotion
without rewriting the project constitution entirely

#### CHANGE-011 — Minor Change Rule
A change is minor if it corrects:
- wording clarity
- formatting
- traceability labels
- non-semantic consistency issues
without changing implementation truth

#### CHANGE-012 — Re-Validation Rule
If a change affects:
- lifecycle
- run truth
- persistence
- activation
- interface contract
- acceptance criteria
then downstream implementation artifacts and QA mappings must be revalidated.

---

### 18.7 Section Summary

#### CHANGE-013 — Section Summary Rule
The Change Log is the formal governance record for modifications to Document A. No locked decision should be treated as changed unless this section says it changed. This preserves determinism for the project owner, human reviewers, AI coding agents, and all subordinate documents.

#### CHANGE-014 — Final Boundary Rule
This section governs revision traceability only.
It does not itself grant permission to change the document without an explicit recorded entry.
---
**Appendix Boundary Note**

Low-level machine-readable artifacts such as:
- OpenAPI specifications
- JSON schemas
- SQL migrations
- QA execution matrices
- report-generation templates
- state transition tables used for engineering execution

belong to **Document C — Engineering Annex Pack** and remain subordinate to this document.

This document remains the governing product-and-system source of truth.

---
**Canonical Footer**  
Document: Document A — Master Project Creative Brief  
Document ID: DOC-A  
Version: V2.A-Final-Draft-3  
Change Authority: Section 18 — Change Log  
Override Rule: No subordinate document may override this document
---
