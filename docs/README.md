# Preflight Documentation Hub

Welcome to the Preflight documentation portal. Preflight is an open-source, local-first AI multi-agent simulation engine that stress-tests proposals, system designs, and business offers before launch.

---

## Documentation Taxonomy & Navigation

The documentation is organized into three distinct tiers based on technical depth and governance authority:

```
docs/
├── README.md                      # [You are here] Documentation Hub & Index
├── ARCHITECTURE.md                # System Architecture, IPC boundaries, Thread models
├── decisions/                     # Architectural Decision Records (ADRs)
│   └── adr_001_open_core_transition.md # Transition to Open Core / Community Edition
├── specifications/                # Canonical Architectural Specifications
│   ├── Document_A/                # Master Project Creative Brief (DOC-A)
│   ├── Document_B/                # Master Execution Plan (DOC-B)
│   └── Document_C/                # Engineering Annex Pack (DOC-C)
└── archive/                       # Historical Build Logs & Evidence
    └── execution_history/         # Complete 11-Phase phased construction record (65 task packets)
```

---

## 1. Tier 1: Surface & Onboarding (Start Here)

For contributors, evaluators, and end users wanting to get started in under 60 seconds:
- **[Root README](../README.md):** High-level product overview, problem statement, architectural scorecard, and quickstart commands.
- **[Justfile](../Justfile):** One-command task automation (`just setup`, `just test`, `just evals`, `just dev`).
- **[Grant Proposal](../OPENAI_GRANT_PROPOSAL.md):** Turn-key dossier prepared for the **OpenAI Codex for OSS Grant**.
- **[Contributing Guide](../CONTRIBUTING.md):** Git workflow, code styles, and PR quality gates.
- **[Security Policy](../SECURITY.md):** Responsible vulnerability disclosure and secret boundary commitments.

---

## 2. Tier 2: Architecture & System Design

For engineers evaluating system design, security isolation, and IPC boundaries:
- **[Architecture Guide](ARCHITECTURE.md):** In-depth technical guide covering:
  - Tauri v2 (Rust) privileged orchestration shell.
  - Subprocess IPC and ephemeral workspace boundaries.
  - Moderator-Directed Multi-Agent debate protocol.
  - Zero-egress native credential storage (Windows DPAPI).
  - Abnormal termination detection and startup janitor cleanup.
- **[ADR-001: Open Core / Community Edition Transition](decisions/adr_001_open_core_transition.md):** The formal governance decision separating the pure open-source BYOK engine from optional enterprise licensing.

---

## 3. Tier 3: Canonical Specifications & Phased History

For auditors verifying formal compliance against the project constitution:
- **[Document A — Master Creative Brief](specifications/Document_A/document_a_master_project_creative_brief_v2a_final_draft_2.md):** The comprehensive 9,600-line functional and behavioral specification for the simulation engine.
- **[Document B — Master Execution Plan](specifications/Document_B/document_b_master_execution_plan_b1_0_final_draft_2.md):** The foundational engineering execution blueprint.
- **[Document C — Engineering Annex Pack](specifications/Document_C/document_c_engineering_annex_pack_c1_0_final_draft_1.md):** Formal interface definitions, schemas, and contract annexes.
- **[Execution Archive](archive/execution_history/):** The complete historical record of the 11-phase development cycle, including 65 task execution packets, decision logs, and release gate assessments.
