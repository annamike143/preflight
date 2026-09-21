# Contributing to Preflight

Thank you for your interest in contributing to **Preflight**! We welcome contributions from the community to make AI-moderated multi-agent simulations more robust, accessible, and insightful.

---

## Code of Conduct

All contributors and maintainers are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Please report unacceptable behavior to the project maintainer.

---

## Architectural Principles

1. **Local-First & Ephemeral:** Orchestration, state management, and report generation run on the local machine. Workspace runs are ephemeral and cleanly swept.
2. **Bring-Your-Own-Key (BYOK):** Users provide their own OpenAI/Google API keys. Secrets must never be transmitted to third parties or logged.
3. **Bounded Swarm Moderation:** Avoid unconstrained all-agent chatter. Exactly one Moderator agent directs rounds, keeping memory bounded and tokens governed.
4. **Zero-Warning Quality Discipline:** All PRs must pass zero-warning checks across Rust, TypeScript, and Python.

---

## Development Setup

### Prerequisites
- **OS:** Windows 10/11 (Primary target for MVP shell)
- **Node.js:** v18.0.0+ and `npm`
- **Rust:** Latest stable toolchain
- **Python:** Python 3.14+

### Setup Commands
Using `just` (recommended) or standard commands:

```powershell
# 1. Initialize Python virtual environment
py -3.14 -m venv .venv
.\.venv\Scripts\python.exe -m pip install --upgrade pip
.\.venv\Scripts\python.exe -m pip install -e .\engine -e .\commercial_backend pytest ruff pytest-asyncio

# 2. Install Frontend dependencies
Push-Location app
npm ci
Pop-Location
```

---

## Verification & Quality Gates

Before opening a Pull Request, run the local verification suite:

```powershell
# Python Engine
.\.venv\Scripts\python.exe -m ruff check engine
.\.venv\Scripts\python.exe -m pytest engine/tests

# Frontend
Push-Location app
npm run lint
npm run typecheck
npm run test
npm run build
Pop-Location

# Rust Shell
Push-Location shell
cargo check
cargo clippy -- -D warnings
cargo test
Pop-Location
```

---

## Pull Request Guidelines

1. **Focused Changes:** Keep PRs scoped to a single feature or bugfix.
2. **Add Tests:** Ensure new capabilities in `engine/` or `shell/` are covered by automated unit/contract tests.
3. **Do Not Commit Secrets:** Never commit `.env` files or hardcoded API keys.
