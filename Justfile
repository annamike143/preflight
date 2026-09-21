# Preflight Developer Task Runner (Justfile)

default:
    @just --list

# Complete developer environment initialization
setup:
    py -3.14 -m venv .venv
    ./.venv/Scripts/python.exe -m pip install --upgrade pip
    ./.venv/Scripts/python.exe -m pip install -e ./engine -e ./commercial_backend pytest ruff pytest-asyncio
    cd app && npm ci

# Run all quality verification gates across Rust, React, and Python
test: test-engine test-commercial test-shell test-app

# Run simulation engine pytest suite
test-engine:
    ./.venv/Scripts/python.exe -m ruff check engine
    ./.venv/Scripts/python.exe -m pytest engine/tests

# Run commercial backend pytest suite
test-commercial:
    ./.venv/Scripts/python.exe -m ruff check commercial_backend
    ./.venv/Scripts/python.exe -m pytest commercial_backend/tests

# Run Rust Tauri shell gates
test-shell:
    cd shell && cargo check && cargo clippy -- -D warnings && cargo test -- --test-threads=1

# Run React frontend quality gates
test-app:
    cd app && npm run lint && npm run typecheck && npm run test && npm run build

# Run OpenAI evaluation and benchmark suite
evals:
    ./.venv/Scripts/python.exe -m engine.evals.run_evals

# Launch the desktop application in development mode
dev:
    cd app && npm run tauri:dev:desktop
