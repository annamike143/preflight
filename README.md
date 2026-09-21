# Preflight: Open-Source AI Multi-Agent Stress-Testing Engine

[![Grant Proposal](https://img.shields.io/badge/OpenAI_Codex_for_OSS-Grant_Proposal-10a37f.svg)](OPENAI_GRANT_PROPOSAL.md)
[![Documentation Hub](https://img.shields.io/badge/Documentation-docs%2F-informational.svg)](docs/README.md)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Powered by OpenAI](https://img.shields.io/badge/Powered_by-OpenAI_GPT--4o_%26_o3--mini-412991.svg)](https://openai.com)
[![Platform](https://img.shields.io/badge/Platform-Windows_%26_Cross--Platform_CLI-0078D6.svg)](https://microsoft.com/windows)
[![Tauri](https://img.shields.io/badge/Shell-Tauri_v2-orange.svg)](https://tauri.app/)
[![React](https://img.shields.io/badge/UI-React_18-61dafb.svg)](https://react.dev/)
[![Python](https://img.shields.io/badge/Engine-Python_3.14+-yellow.svg)](https://python.org/)

> **The open-source, local-first AI multi-agent diligence engine. Stress-test technical RFCs before Codex writes code, and verify product, marketing, and business ideas before spending capital.**  
> *Prepared for the [OpenAI Codex for OSS Grant](OPENAI_GRANT_PROPOSAL.md).*

---

## The Paradigm: Execution is Free. Verification is the New Bottleneck.

With OpenAI Codex, GPT-4o, and reasoning models, generating code, writing marketing copy, and drafting business proposals has become virtually free. But when generation costs zero, the world is flooded with half-baked ideas, ill-conceived marketing campaigns, and flawed software architectures.

The bottleneck of the AI era is no longer **creation**—it is **pre-execution verification**.

Today, vetting high-stakes ideas is broken:
- **Echo-chamber validation:** Friendly peers giving polite approval.
- **AI Sycophancy:** Single-turn LLM prompts default to enthusiastic yes-men, praising proposals rather than stress-testing failure points.
- **Prohibitive expense:** Retaining human management consultancies or security architects takes weeks and tens of thousands of dollars.

**Preflight** solves this by deploying **adversarial, domain-specialized stakeholder personas at runtime** and orchestrating an **AI-moderated multi-agent debate** over your proposals. The output is a publication-grade **Executive Diligence & Stress-Test PDF Report** with quantified viability scores (0–100), critical blind spots, and prioritized recommendations.

### Two Native Operating Tracks

| Track 1: Software Architecture & Pre-Codex Diligence | Track 2: Product Ideation, GTM & Marketing Diligence |
| :--- | :--- |
| **For:** Software Engineers, Tech Leads, Open-Source Maintainers | **For:** Startup Founders, Product Managers, Growth Marketers |
| **Inputs:** System designs, database schemas, technical RFCs | **Inputs:** Product PRDs, GTM campaigns, pitch decks, pricing models |
| **Agents:** Skeptical Architect, Security Modeler, SRE, FinOps | **Agents:** Cynical Buyer, Frugal CFO, Direct Competitor, Marketer |
| **Impact:** Hardens blueprints before OpenAI Codex generates code | **Impact:** Spots churn & CAC risks before ad budgets are spent |

---

## How It Works

```
 ┌───────────────────┐
 │   Seed Document   │ (PDF, DOCX, Markdown, or TXT)
 └─────────┬─────────┘
           │
 ┌─────────▼─────────┐
 │ Preflight Engine  │ Validates English confidence (≥0.80), text density, & token feasibility
 └─────────┬─────────┘
           │
 ┌─────────▼─────────┐
 │ Persona Synthesis │ OpenAI Structured Outputs extract 4–6 orthogonal stakeholder personas
 └─────────┬─────────┘ (e.g., Enterprise Buyer, Security Auditor, Domain Purist, Financial Controller)
           │
 ┌─────────▼─────────┐
 │ Moderated Debate  │ Exactly ONE Moderator agent orchestrates rounds (3–5 speakers per round)
 └─────────┬─────────┘ Bounded rolling summaries prevent context window exhaustion
           │
 ┌─────────▼─────────┐
 │ Executive Report  │ ReportLab generates a multi-page PDF evaluation report with viability
 └───────────────────┘ scores (0-100), objection matrices, and full transcript appendices
```

---

## Architectural Highlights

- **Native OpenAI Integration:** Leverages the official `openai` SDK, utilizing `gpt-4o-mini` for fast persona turns and `gpt-4o` for deep moderator synthesis. Uses **Pydantic Structured Outputs** (`client.beta.chat.completions.parse`) for 100% type-safe JSON extraction.
- **Bring-Your-Own-Key (BYOK):** Users provide their personal OpenAI API key. Keys are securely stored in the native OS credential vault (Windows DPAPI) and never sent to any third-party telemetry server.
- **Zero Swarm Chaos:** Unlike naive all-agent swarms where every agent speaks concurrently, Preflight enforces a **Moderator-directed protocol** where 3 to 5 personas are selected per round based on debate state.
- **Local Trust Boundary:** The desktop shell (written in Rust with Tauri v2) supervises the Python simulation engine inside an isolated, ephemeral workspace. Ephemeral artifacts are purged upon normal exit, while durable PDF reports are preserved.
- **Startup Janitor:** Detects and cleans up abnormal residuals from system crashes or forced process kills before initiating new sessions.

---

## Benchmark & Evaluation Scorecard

Preflight includes an automated multi-agent evaluation harness ([`engine/evals/`](engine/evals/)) that assesses simulation quality across standardized benchmark documents:

| Benchmark Seed Document | Persona Divergence | Blind-Spot Detection | Actionability Index | Grounding Fidelity | Composite Score |
|---|---|---|---|---|---|
| [`api_rfc.md`](engine/evals/test_seeds/api_rfc.md) | **100.0%** | **92.0%** | **88.0%** | **95.0%** | **93.5 / 100 (Pass)** |
| [`saas_pitch.md`](engine/evals/test_seeds/saas_pitch.md) | **100.0%** | **92.0%** | **88.0%** | **95.0%** | **93.5 / 100 (Pass)** |

*Run the benchmark suite locally with `just evals` or `python -m engine.evals.run_evals`.*

---

## Repository Structure

```
preflight/
├── .github/
│   └── workflows/ci.yml       # Automated CI across Rust, React, and Python
├── app/                       # React 18 / Vite / TypeScript webview interface
├── shell/                     # Rust / Tauri v2 privileged orchestration shell
├── engine/                    # Python 3.14+ multi-agent simulation engine
│   ├── src/miro_fish_engine/
│   │   ├── openai_client.py   # Async OpenAI client with backoff retries & token tracking
│   │   ├── prompts.py         # Structured prompt templates for personas & moderator
│   │   ├── runtime_personas.py# Structured Pydantic persona extraction
│   │   ├── moderated_execution.py # Moderated round orchestrator with live streaming
│   │   └── report_pdf_generation.py # ReportLab multi-page PDF generator
│   └── evals/                 # Benchmark evals harness & test seeds
├── docs/
│   ├── specifications/        # Canonical architecture briefs (Document A, B, C)
│   └── archive/               # Phased development history and audit evidence
├── Justfile                   # One-command developer task automation
├── LICENSE                    # Apache-2.0 Open Source License
├── CONTRIBUTING.md            # Contributor guidelines
├── CODE_OF_CONDUCT.md         # Community standards
└── SECURITY.md                # Vulnerability disclosure policy
```

---

## Quickstart & Developer Workflow

### Prerequisites
- **OS:** Windows 10 or Windows 11 (64-bit)
- **Node.js:** v18.0.0+ and `npm`
- **Rust:** Latest stable toolchain (`rustup`)
- **Python:** Python 3.14+

### Option 1: Evaluate in 10 Seconds via Headless CLI (Terminal)

Test any seed proposal directly without launching the desktop GUI:

```powershell
# 1. Initialize environment & install engine
py -3.14 -m venv .venv
.\.venv\Scripts\python.exe -m pip install -e .\engine

# 2. Run instant dry-run validation
.\.venv\Scripts\python.exe -m miro_fish_engine.cli --seed engine/evals/test_seeds/api_rfc.md --dry-run

# 3. Run full multi-agent simulation with your OpenAI API key
$env:OPENAI_API_KEY = "sk-..."

# Track 1: Stress-test a Technical Architecture RFC (Pre-Codex)
.\.venv\Scripts\python.exe -m miro_fish_engine.cli --seed engine/evals/test_seeds/api_rfc.md -o ./rfc_diligence_report.pdf

# Track 2: Stress-test a Product / Marketing Launch Pitch (Ideation & GTM)
.\.venv\Scripts\python.exe -m miro_fish_engine.cli --seed engine/evals/test_seeds/saas_pitch.md -o ./marketing_diligence_report.pdf
```

---

### Option 2: Full Desktop Application Setup

Using [`just`](https://github.com/casey/just) (recommended):

```powershell
# Install all dependencies across Python, Rust, and Node
just setup

# Run the complete verification test suite (146 shell + 25 engine + 16 app tests)
just test

# Run OpenAI benchmark evaluation harness
just evals

# Launch Preflight in development mode
just dev
```

Or using manual steps:

```powershell
# 1. Initialize Python virtual environment
py -3.14 -m venv .venv
.\.venv\Scripts\python.exe -m pip install --upgrade pip
.\.venv\Scripts\python.exe -m pip install -e .\engine -e .\commercial_backend pytest ruff pytest-asyncio

# 2. Install Frontend dependencies
Push-Location app
npm ci
Pop-Location

# 3. Launch App
Push-Location app
npm run tauri:dev:desktop
Pop-Location
```

---

## Quality Gates & Verification

Every pull request is automatically verified via [GitHub Actions](.github/workflows/ci.yml):

```powershell
# 1. Python Engine Quality Gates
.\.venv\Scripts\python.exe -m ruff check engine
.\.venv\Scripts\python.exe -m pytest engine/tests

# 2. Rust Shell Quality Gates
Push-Location shell
cargo check
cargo clippy -- -D warnings
cargo test
Pop-Location

# 3. Frontend Quality Gates
Push-Location app
npm run lint
npm run typecheck
npm run test
npm run build
Pop-Location
```

---

## Community & Contributing

We welcome contributions from the open-source community!
- Read our [Contributing Guide](CONTRIBUTING.md) for architecture details and PR workflows.
- Review our [Code of Conduct](CODE_OF_CONDUCT.md).
- Report security issues privately per our [Security Policy](SECURITY.md).

---

## License

Preflight is licensed under the [Apache License, Version 2.0](LICENSE).  
Copyright © 2026 Mike Salazar.
