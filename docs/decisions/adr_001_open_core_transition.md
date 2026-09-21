# ADR-001: Transition to Open Core / Community Edition Architecture

**Status:** Accepted  
**Date:** 2026-09-22  
**Author:** Mike Salazar  
**Supersedes/Amends:** Document A (Section 18 Change Log Amendment)

---

## Context

The initial architectural specification for this project ([Document A](../specifications/Document_A/document_a_master_project_creative_brief_v2a_final_draft_2.md)) envisioned a Windows-first desktop SaaS ("Miro-Fish Desktop SaaS") coupled with a mandatory commercial backend for device activation and subscription licensing.

In preparing the codebase for submission to the **OpenAI Codex for Open-Source Software (OSS) Grant**, two fundamental tensions were identified:
1. **Open Source Alignment:** True open-source software cannot require a proprietary commercial license server to execute on localhost with BYOK API keys.
2. **Developer Adoption:** Technical evaluators and open-source contributors require immediate, frictionless evaluation (via both a desktop GUI and a headless terminal CLI).

## Decision

We formally adopt an **Open Core / Community Edition** architecture:

1. **Community Edition (Default / Public Good):**
   - The desktop shell (`shell/`) and simulation engine (`engine/`) operate in `ExecutionMode::Community` by default.
   - Requires only a user-supplied `OPENAI_API_KEY` (BYOK).
   - Activation gates are automatically satisfied without demanding a commercial license ID.
   - Distributed under the OSI-approved **Apache License, Version 2.0**.
   - Accessible via both the Tauri desktop GUI and the standalone headless CLI (`python -m miro_fish_engine.cli`).

2. **Enterprise Edition (Optional Reference Extension):**
   - The commercial backend (`commercial_backend/`) remains preserved in the repository as an optional reference implementation for enterprise teams requiring centralized seat management, device limits, and custom billing webhooks.
   - When running in `ExecutionMode::Enterprise` (`PREFLIGHT_EXECUTION_MODE=enterprise`), the shell enforces activation tokens against the local commercial SQLite database.

## Consequences & Invariants

- **Backward Compatibility:** All 146 existing Rust shell tests, 25 Python engine tests, and 16 React frontend tests remain green. The enterprise activation test suite continues to verify commercial paths explicitly.
- **Specification Governance:** This decision constitutes an approved Section 18 amendment to Document A, formally recognizing Preflight Community Edition as the canonical open-source distribution.
