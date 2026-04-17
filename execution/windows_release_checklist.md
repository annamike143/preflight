# Windows Release Checklist

## Purpose
This file is an operator checklist for producing and smoke-validating a Windows release from the current repo state.
It is not a release-status authority. Current product truth and release truth remain governed by Document A, Document B, and the active Phase 11 release record.

## Current Baseline
- Current governed product scope is Windows-only.
- Current governed Phase 11 release posture is `release-ready` after `P11-WS11-T06`.
- The historical `P11-WS11-T05` `release-blocked` verdict remains part of repo history, but it is no longer the current release record.

## 1. Freeze Inputs
- [ ] Confirm Windows-only product scope in `Document_A/document_a_master_project_creative_brief_v2a_final_draft_2.md`.
- [ ] Confirm current `release-ready` verdict in `execution/phase_11_validation/release_readiness_assessment.md`.
- [ ] Confirm no active packet is open in `execution/progress.md`.
- [ ] Confirm the working tree contains only intended release-related changes.

## 2. Prepare the Windows Build Host
- [ ] Use a Windows machine for the release build and smoke run.
- [ ] Ensure Rust and Cargo are available for the Tauri shell build.
- [ ] Ensure Node.js and npm are available for the frontend build.
- [ ] Ensure Python 3.14 is available because both Python projects declare `requires-python = ">=3.14"`.
- [ ] Ensure a repo-local `.venv` exists. If setting up fresh, use a Python 3.14 virtual environment.
- [ ] Ensure frontend dependencies are installed from `app/package-lock.json`.

### Fresh Setup Commands
Run from the repo root on a fresh Windows machine:

```powershell
py -3.14 -m venv .venv
.\.venv\Scripts\python.exe -m pip install --upgrade pip
.\.venv\Scripts\python.exe -m pip install -e .\engine -e .\commercial_backend pytest ruff
Push-Location app
npm ci
Pop-Location
```

## 3. Run the Required Validation Gates

### Shell
- [ ] Run the Rust shell gates in `shell/`:

```powershell
Push-Location shell
cargo check
cargo clippy -- -D warnings
cargo test
Pop-Location
```

- [ ] Expect the Phase 11 baseline to remain clean. The last governed rerun recorded 144 shell tests green.

### Frontend
- [ ] Run the frontend gates in `app/`:

```powershell
Push-Location app
npm run lint
npm run typecheck
npm run test
npm run build
Pop-Location
```

- [ ] Expect the Phase 11 baseline to remain clean. The last governed rerun recorded 6 Vitest files and 16 frontend tests green.

### Engine
- [ ] Run the engine gates from the repo root:

```powershell
.\.venv\Scripts\python.exe -m compileall engine/src engine/tests
.\.venv\Scripts\python.exe -m ruff check engine
.\.venv\Scripts\python.exe -m pytest engine/tests
```

- [ ] Expect the Phase 11 baseline to remain clean. The last governed rerun recorded 17 engine tests green.

### Commercial Backend
- [ ] Run the commercial backend gates from the repo root:

```powershell
.\.venv\Scripts\python.exe -m compileall commercial_backend/src commercial_backend/tests
.\.venv\Scripts\python.exe -m ruff check commercial_backend
.\.venv\Scripts\python.exe -m pytest commercial_backend/tests
```

- [ ] Expect the Phase 11 baseline to remain clean. The last governed rerun recorded 11 commercial-backend tests green.

## 4. Set Optional Release-Test Environment Hooks
Set only the variables needed for the flows you intend to validate.

### Updater Validation
- [ ] Set `MIRO_FISH_APP_UPDATER_ENDPOINTS` if you want to validate signed app-updater discovery.
- [ ] Set `MIRO_FISH_APP_UPDATER_PUBKEY` if you want updater verification to pass.
- [ ] Use the diagnostics flow to verify refresh, prepare, apply, and restart-request truth only if these values are configured.

### Runtime Delivery Validation
- [ ] Set `MIRO_FISH_RUNTIME_DELIVERY_MANIFEST` if you want to validate direct runtime manifest hydration.
- [ ] Or set `MIRO_FISH_COMMERCIAL_BACKEND_RUNTIME_DELIVERY_MANIFEST_PATH` if you want to validate the backend-authorized manifest fallback path.

### Repo-Local Python and Backend Overrides
- [ ] If needed, set `MIRO_FISH_PYTHON_EXE` to override the default repo-local `.venv\Scripts\python.exe` lookup.
- [ ] If needed, set `MIRO_FISH_COMMERCIAL_BACKEND_DB_PATH` to control the local activation ledger path.
- [ ] If needed, set `MIRO_FISH_COMMERCIAL_BACKEND_SIGNING_KEY`, `MIRO_FISH_COMMERCIAL_BACKEND_KEY_IDENTIFIER`, and `MIRO_FISH_COMMERCIAL_BACKEND_ISSUER` to override the repo-local commercial backend defaults.

## 5. Seed a Local Activation License for Smoke Testing
This is only needed if you want an end-to-end local activation smoke using the repo-local commercial backend.

- [ ] Set `PYTHONPATH` to `commercial_backend/src`.
- [ ] Use `python -m commercial_backend apply-payment-event` to create a usable local license.
- [ ] Use `python -m commercial_backend evaluate-activation-request` only if you want to verify the backend decision path directly outside the app.

### Example Local License Seed

```powershell
$env:PYTHONPATH = (Resolve-Path .\commercial_backend\src)
$db = Join-Path (Resolve-Path .) "commercial_backend\.state\commercial_ledger.sqlite3"

@'
{
  "payment_provider_identifier": "stripe",
  "provider_event_identifier": "evt-local-release-001",
  "event_type": "payment_succeeded",
  "purchaser_email": "owner@example.com",
  "plan_tier": "annual",
  "receipt_timestamp_epoch_seconds": 100,
  "license_id": null,
  "expiration_timestamp_epoch_seconds": null,
  "payload_reference": "evt_ref_release_001",
  "entitlement_active": true
}
'@ | .\.venv\Scripts\python.exe -m commercial_backend apply-payment-event --database-path $db
```

- [ ] Capture the returned `license_id` for the app activation smoke.

## 6. Build the Windows Desktop Artifact
- [ ] Build from `app/` using the repo wrapper script:

```powershell
Push-Location app
npm run tauri:build:desktop
Pop-Location
```

- [ ] Confirm the wrapper runs `npm run build` first and then `npx @tauri-apps/cli build --config tauri.conf.json` against `shell/`.
- [ ] Confirm the build uses the current Tauri product metadata:
  - product name: `Miro Fish Desktop SaaS`
  - version: `0.1.0`
  - identifier: `com.mikesalazar.mirofishdesktopsaas`
  - Windows icon: `shell/icons/icon.ico`
- [ ] Confirm a new release output tree appears under `shell/target/release`.
- [ ] Note that this repo currently has `shell/target/debug` present but no existing `shell/target/release` tree, so a Windows release artifact has not yet been produced on this machine.
- [ ] Collect the Windows installer or bundle emitted by the Tauri release output tree.

## 7. Run the Windows Smoke Pass Against the Built Artifact

### Launch and Activation
- [ ] Launch the built desktop app successfully.
- [ ] Verify the app window opens as `Miro Fish Desktop SaaS`.
- [ ] Complete activation using the locally seeded license or your intended release activation path.
- [ ] Confirm denial and technical-failure paths still render truthful classifications if you intentionally test a bad activation input.

### Provider Setup
- [ ] Configure an OpenAI or Google provider key.
- [ ] Confirm provider readiness becomes available only after shell-owned setup truth is satisfied.
- [ ] Confirm raw provider secrets are not exposed back to the UI.

### Preflight
- [ ] Verify one supported English seed document can be selected and processed.
- [ ] Verify supported types remain limited to PDF, DOCX, TXT, and Markdown.
- [ ] Verify the app rejects invalid inputs truthfully, including unsupported type, oversized file, too-little extracted content, or non-English content.

### Run and Monitoring
- [ ] Start a run only after activation, provider setup, and Preflight readiness are satisfied.
- [ ] Verify live monitoring updates without collapsing lifecycle truth.
- [ ] Verify run completion, constrained completion, or failure states remain distinct.

### Report and Cleanup
- [ ] Generate a report successfully.
- [ ] Verify local report open and export behavior works.
- [ ] Verify cleanup truth remains distinct from report availability truth.
- [ ] Verify preserved reports survive cleanup behavior.

### Restart and Janitor
- [ ] Restart the app after a normal run and confirm no crash-resume behavior is implied.
- [ ] If you test abnormal termination, confirm next-launch janitor behavior is truthful and does not become hidden resume.

### Diagnostics, Updater, and Runtime Delivery
- [ ] If updater env vars are configured, verify refresh, prepare, apply, and restart-request truth from the diagnostics surface.
- [ ] If runtime manifest env vars are configured, verify runtime hydration, compatibility checks, rejection behavior, and readiness truth.
- [ ] Confirm neither updater apply nor runtime mutation occurs during an active run.

## 8. Capture Release Evidence and Handoff Material
- [ ] Record the commit SHA used for the release build.
- [ ] Record the exact gate commands that passed on the build host.
- [ ] Record whether updater validation was exercised and which non-secret configuration path was used.
- [ ] Record whether runtime delivery was exercised through direct manifest configuration or backend-authorized fallback.
- [ ] Archive the produced Windows installer or bundle from `shell/target/release`.
- [ ] Archive any non-secret release notes, version notes, and smoke-test notes needed for later Mac handoff context.

## 9. Windows-Specific Do-Not-Do List
- [ ] Do not reintroduce macOS support claims while the governed scope remains Windows-only.
- [ ] Do not claim Mac installer readiness from a Windows build host.
- [ ] Do not treat updater discovery as equivalent to verified ready-to-apply state.
- [ ] Do not treat runtime delivery availability as equivalent to runtime readiness before verification passes.
- [ ] Do not ship based on a dev build alone; require a real Tauri release build and artifact smoke.

## 10. Exit Condition
This Windows checklist is complete only when all of the following are true:

- [ ] All required validation gates pass cleanly.
- [ ] A real Windows release build is produced.
- [ ] The built artifact passes the Windows smoke flow you actually intend to support.
- [ ] Any optional updater or runtime-delivery path you plan to expose has been tested with truthful configured behavior.
- [ ] The release artifact and evidence notes are captured for later transfer or platform expansion work.