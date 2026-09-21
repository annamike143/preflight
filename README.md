# Miro-Fish Desktop SaaS (`preflight`)

[![Platform](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://microsoft.com/windows)
[![Release Posture](https://img.shields.io/badge/Release_Status-Release--Ready-success.svg)](#current-release-status)
[![Tauri](https://img.shields.io/badge/Tauri-v2-orange.svg)](https://tauri.app/)
[![React](https://img.shields.io/badge/Frontend-React_18-61dafb.svg)](https://react.dev/)
[![Python](https://img.shields.io/badge/Engine-Python_3.14+-yellow.svg)](https://python.org/)

> **Local-first, cloud-inference-dependent desktop application for AI-moderated stress-testing of ideas, business offers, educational curricula, and positioning documents.**

---

## Overview

**Miro-Fish Desktop SaaS** (code-named `preflight`) is a premium desktop tool designed for solo founders, operators, educators, and creators. Before launching a new venture, product offer, course, or marketing campaign, operators typically struggle to get unbiased, critical feedback without paying heavy consulting fees or suffering from echo-chamber validation.

Miro-Fish solves this by ingesting an English seed document, running an automated **Preflight Feasibility Check**, generating realistic synthetic stakeholder personas at runtime, and orchestrating a **moderated multi-agent debate simulation**. The end deliverable is a comprehensive, publication-grade **PDF Evaluation Report** with viability scoring, friction points, consensus areas, and actionable recommendations.

### Core Principles

1. **Bring-Your-Own-Key (BYOK):** Users provide their own OpenAI or Google Gemini API keys. You only pay standard third-party inference costs directly to the provider—no vendor compute markup.
2. **Local Trust Boundary:** All privileged operations—API secret management (via native OS credential vaults), seed document parsing, process supervision, and temporary run workspaces—remain strictly on your local machine.
3. **Bounded Multi-Agent Moderation:** Rather than unconstrained and expensive "all-agent swarm" chatter, **exactly one Moderator agent** orchestrates structured rounds, dynamically selecting **3 to 5 personas per round** to deliberate and challenge assumptions.
4. **Governed Token & Duration Caps:** Complete control over AI spend. Every run requires a defined token budget and enforces a hard **2-hour duration cap**. Hitting a cap gracefully triggers controlled finalization to produce a report from all work completed to that point.
5. **Clean Local Footprint:** Ephemeral run workspaces are purged on normal shutdown. If an abnormal shutdown or power cut occurs, a **Startup Janitor** sweeps orphan residuals on the next launch, while preserving all generated PDF reports.

---

## Architecture at a Glance

The project enforces strict separation of concerns, formal contract boundaries, and zero-leakage trust policies:

```
                          ┌──────────────────────────┐
                          │   Frontend (app/)        │
                          │   React 18 + Vite Webview│
                          └────────────┬─────────────┘
                                       │ Tauri IPC
                          ┌────────────▼─────────────┐
                          │  Desktop Shell (shell/)  │
                          │     Rust + Tauri v2      │
                          │   (Local Trust Boundary) │
                          └──────┬────────────┬──────┘
             Subprocess Supervision│            │ Local Activation Validation
                                   │            ▼
 ┌─────────────────────────────────▼─┐    ┌─────────────────────────────────┐
 │   Simulation Engine (engine/)     │    │  Commercial Backend             │
 │   Python 3.14+ (Local Workspace)  │    │  (commercial_backend/)          │
 │   - Dynamic Personas              │    │  Python 3.14+                   │
 │   - Moderator Agent               │    │  - 2-Device Activation Ledger   │
 │   - Bounded Memory & Caps         │    │  - Signed Token Issuance        │
 │   - ReportLab PDF Generation      │    │  - Runtime Manifest Auth        │
 └───────────────────────────────────┘    └─────────────────────────────────┘
```

### Component Breakdown

| Directory | Technology | Role |
|---|---|---|
| [`shell/`](shell/) | **Rust, Tauri v2** | The central orchestrator and security boundary. Manages Windows credential storage, device activation, seed document validation, Python engine supervision, lifecycle events, startup janitor, and native updates. |
| [`app/`](app/) | **React 18, TypeScript, Vite** | Strictly presentation-only UI rendered in a Tauri webview. Provides Setup (keys/seed), Run (live stream of debate turns), Reports (PDF viewer/export), and Diagnostics screens. |
| [`engine/`](engine/) | **Python 3.14+, ReportLab** | Spawns in an isolated ephemeral workspace. Synthesizes runtime personas, runs moderated debate rounds, maintains bounded memory, monitors token/time caps, and generates the final PDF report. |
| [`commercial_backend/`](commercial_backend/) | **Python 3.14+** | Commercial SaaS licensing service. Manages payment reconciliation, enforces 2-device activation limits per license, issues signed tokens for offline validation, and authorizes runtime manifests. |
| [`Document_A/`](Document_A/) | **Markdown Specifications** | **Supreme Source of Truth**: Master Project Creative Brief (constitutional rules, product scope, and architectural invariants). |
| [`Document_B/`](Document_B/) | **Markdown Specifications** | Master Execution Plan (sequencing, milestones, phase definitions, and gating criteria). |
| [`Document_C/`](Document_C/) | **Markdown Specifications** | Engineering Annex Pack (formal schemas, OpenAPI specs, SQL schemas, and QA matrices). |
| [`execution/`](execution/) | **Execution OS** | Complete, auditable historical record of task packets, live logs, gate runs, and verification evidence across Phases 1 through 11. |

---

## User Journey & Workflow

1. **Activation:** The user enters their commercial license key. The shell validates the device against the commercial backend (binding up to 2 devices) and caches an offline validation token.
2. **Provider Configuration:** The user enters their OpenAI or Google Gemini API key. Keys are saved securely into the Windows Credential Store (DPAPI/keyring) and never exposed to the webview DOM.
3. **Seed Intake & Preflight:** The user drops an English document (`.pdf`, `.docx`, `.txt`, `.md`, up to 10 MiB). The shell checks:
   - File integrity and format.
   - Text extractability ($\ge 1,000$ normalized characters).
   - Language confidence score ($\ge 0.80$ English).
   - Feasibility plan derivation based on requested rounds and token budget.
4. **Moderated Simulation:**
   - The shell creates a fresh, isolated run directory and launches the simulation engine.
   - The engine derives synthetic stakeholder personas matching the domain.
   - The Moderator schedules rounds, calling on 3–5 personas per round.
   - Live events stream into the UI in real time showing turns, token burn, and round summaries.
5. **Report Generation & Cleanup:**
   - Upon completion (or upon reaching token/time caps), ReportLab renders a multi-page PDF evaluation report with viability scores, consensus findings, and appendices.
   - The report is stored permanently in the user's local documents folder.
   - Ephemeral working files are swept and cleaned.

---

## Prerequisites & Development Setup

### System Requirements
- **OS:** Windows 10 or Windows 11 (64-bit).
- **Node.js:** v18.0.0+ and `npm`.
- **Rust:** Latest stable toolchain (`rustup` with MSVC toolchain).
- **Python:** Python 3.14+ (available as `py -3.14`).

### 1. Repository Setup

Clone the repository:
```powershell
git clone https://github.com/annamike143/preflight.git
cd preflight
```

### 2. Python Virtual Environment
Initialize the Python virtual environment and install the engine and commercial backend in editable mode:
```powershell
py -3.14 -m venv .venv
.\.venv\Scripts\python.exe -m pip install --upgrade pip
.\.venv\Scripts\python.exe -m pip install -e .\engine -e .\commercial_backend pytest ruff
```

### 3. Frontend Dependencies
Install the web application dependencies:
```powershell
Push-Location app
npm ci
Pop-Location
```

---

## Quality Gates & Verification

Every component enforces strict, zero-warning quality gates:

### Rust Shell Gates
```powershell
Push-Location shell
cargo check
cargo clippy -- -D warnings
cargo test
Pop-Location
```

### Frontend Gates
```powershell
Push-Location app
npm run lint
npm run typecheck
npm run test
npm run build
Pop-Location
```

### Engine Gates
```powershell
.\.venv\Scripts\python.exe -m compileall engine/src engine/tests
.\.venv\Scripts\python.exe -m ruff check engine
.\.venv\Scripts\python.exe -m pytest engine/tests
```

### Commercial Backend Gates
```powershell
.\.venv\Scripts\python.exe -m compileall commercial_backend/src commercial_backend/tests
.\.venv\Scripts\python.exe -m ruff check commercial_backend
.\.venv\Scripts\python.exe -m pytest commercial_backend/tests
```

---

## Running the Application Locally

To start the desktop application in development mode with hot reloading:

```powershell
Push-Location app
npm run tauri:dev:desktop
Pop-Location
```

To run the frontend independently in a browser (with mocked Tauri commands):
```powershell
Push-Location app
npm run dev
Pop-Location
```

---

## Current Release Status

- **Phase 11 Closure:** `CLOSED — RELEASE-READY`
- **Scope:** Windows-only MVP.
- **Audit Basis:** All 11 phases closed with clean quality gates:
  - 144 Rust unit/contract tests passing.
  - 16 Frontend Vitest integration tests passing.
  - 17 Engine simulation/reporting tests passing.
  - 11 Commercial backend activation tests passing.
  - Zero open blockers or unresolved residual issues.

Refer to [`execution/phase_11_validation/release_readiness_assessment.md`](execution/phase_11_validation/release_readiness_assessment.md) and [`execution/windows_release_checklist.md`](execution/windows_release_checklist.md) for full gate evidence.

---

## Project Governance & Documentation

The project is governed strictly by the documents located in the repository:
- **`Document_A/`**: Constitutional rules and core product definition.
- **`Document_B/`**: Phased execution roadmap and milestone contracts.
- **`Document_C/`**: Formal data schemas, test matrices, and engineering specifications.
- **`execution/`**: The live execution log and packet history tracking development truth.

---

## Author & Copyright

- **Owner & Architect:** Mike Salazar
- **Repository:** `annamike143/preflight`
- **Product Name:** Miro-Fish Desktop SaaS
