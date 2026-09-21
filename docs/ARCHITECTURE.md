# Preflight Architecture & System Design

This document details the architectural principles, component boundaries, and security model of **Preflight**, an open-source, local-first AI multi-agent stress-testing engine.

---

## 1. High-Level System Architecture

Preflight separates presentation, privileged orchestration, simulation execution, and credential storage across strict boundaries:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        PRESENTATION LAYER (Webview)                    │
│   React 18 + Vite + TypeScript (Single-Page Desktop UI)                │
│   Setup Screen │ Run Monitor │ Reports Viewer │ Diagnostics Surface    │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ Tauri v2 Strongly-Typed IPC
┌───────────────────────────────────▼────────────────────────────────────┐
│                   PRIVILEGED ORCHESTRATION SHELL (Rust)                │
│   • Device Fingerprint & Local Trust Surface                           │
│   • Native Credential Vault (Windows DPAPI / Keyring)                  │
│   • Ephemeral Workspace Allocation & Janitor Supervision               │
│   • Engine Subprocess Lifecycle & Controlled Graceful Termination      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ Subprocess CLI (Isolated Workspace)
┌───────────────────────────────────▼────────────────────────────────────┐
│                    SIMULATION ENGINE (Python 3.14+)                    │
│   • Seed Document Ingestion (PDF, DOCX, TXT, MD) & English Feasibility │
│   • OpenAI Structured Outputs (Pydantic Synthetic Persona Extraction)  │
│   • Moderator-Directed Debate Orchestrator & Bounded Memory Context    │
│   • Executive Diligence Assembly & ReportLab PDF Compilation           │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ HTTPS (Encrypted API Calls)
┌───────────────────────────────────▼────────────────────────────────────┐
│                       OPENAI INFERENCE CLOUD                           │
│   • GPT-4o / GPT-4o-mini (Debate Turns & Persona Interaction)          │
│   • o1 / o3-mini (Adversarial Critic Reasoning & Blind-Spot Detection) │
│   • Automatic OpenAI Prefix Prompt Caching (50% token cost reduction)  │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Component Breakdown

### A. The Privileged Desktop Shell (`shell/`)
- **Technology:** Rust, Tauri v2.
- **Authority:** The shell is the sole authority for device resources, process spawning, and disk allocation.
- **State Machine:** Governed by `shell/src/app_lifecycle.rs`, supporting both `Community` (open-source BYOK) and `Enterprise` execution modes.
- **Subprocess Supervision:** Spawns the Python simulation engine inside an isolated ephemeral directory (`C:\Users\<user>\AppData\Local\Temp\preflight_run_<id>`). If the user cancels the session or closes the app, the shell issues a graceful stop signal before falling back to forceful SIGKILL termination.
- **Startup Janitor:** On application launch, `startup_janitor_execution.rs` scans for residual workspace folders from unexpected system shutdowns, ensuring clean launches without orphaned processes or leaked temporary data.

### B. The Presentation Layer (`app/`)
- **Technology:** React 18, TypeScript, Vite.
- **Zero Raw Secret Storage:** API keys and credentials are submitted directly to the Rust shell over Tauri commands and are never held in frontend local storage or cookies.
- **Reactive State Sync:** Uses unidirectional state projection (`useSetupScreenState`, `useRunScreenState`, `useReportsScreenState`) to reflect shell-owned truth surfaces.

### C. The Multi-Agent Simulation Engine (`engine/`)
- **Technology:** Python 3.14+, `openai` SDK, `pydantic`, `reportlab`.
- **Persona Extraction:** Analyzes seed texts to synthesize 4 to 6 orthogonal stakeholder personas (e.g. *Enterprise Buyer*, *Security Auditor*, *Domain Purist*, *Financial Controller*).
- **Zero Swarm Chaos:** Naive multi-agent swarms collapse when all agents speak at once. Preflight enforces a **Moderator-directed protocol**:
  1. The Moderator establishes the round agenda.
  2. Exactly 3 to 5 personas are selected to speak based on the agenda.
  3. Rolling summaries compact conversation history to preserve bounded context windows.
- **Report Generation:** Generates a publication-grade PDF report with viability scores (0-100), key pushback points, blind spots, and transcript appendices.

---

## 3. Data Flow & Security Model

```
User API Key ────► Native Vault (DPAPI) ────► Shell Memory ────► Subprocess Env
                         ▲                                              │
                         │                                              ▼
                    Zero Egress                                    OpenAI API
                 (No External Server)                          (Direct HTTPS)
```

1. **Bring-Your-Own-Key (BYOK):** The user provides their personal OpenAI API key.
2. **Zero Telemetry Egress:** Keys and seed documents are never transmitted to any third-party telemetry, commercial activation, or analytics servers. API calls egress exclusively to `api.openai.com`.
3. **Prompt Injection Defense:** Untrusted seed text is safely encapsulated within `<seed_document_untrusted>` XML tags across all persona system prompts to prevent prompt injection and jailbreak attacks.
4. **Prompt Caching Layout:** Invariant seed text and system instructions are positioned at the head of prompt payloads, triggering OpenAI's automatic prefix prompt caching and cutting input token costs by up to 50%.
