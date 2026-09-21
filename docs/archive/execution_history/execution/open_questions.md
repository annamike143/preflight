# Open Questions

No active open questions currently block governed MVP implementation or the reopened Phase 7 packetization basis.

## Recently Closed — 2026-04-16

### OQ-009 — Phase 7 Wave 4 Honest Closeout Versus One-Shot Run Snapshot Visibility
Resolved answer:
- `P07-WS07-T04` cannot close honestly while the current shell one-shot path only persists `latest_snapshot` after execution completes.
- The execution operating system therefore reopens Phase 7 packetization, marks `P07-WS07-T04` BLOCKED, and assigns `P07-WS07-T05` as the bounded shell prerequisite packet for in-flight run/session snapshot truth.

Resolution basis:
- `DEC-022`
- updated `execution/phase_07_master_execution_plan.md`
- authored `execution/task_packets/p07_ws07_t05_expose_in_flight_run_session_snapshot_truth_and_monitoring_availability_basis.md`

## Recently Closed — 2026-04-15

### OQ-004 — Local Persistent Support Store
Resolved answer:
- Repo-local bounded support metadata is stored in a JSON activation support-record file under the app-controlled local support domain.
- Raw device-bound token material remains only in OS-native secure storage.

Resolution basis:
- `DEC-017`
- implemented `shell/src/activation_support_store.rs`

### OQ-005 — Update / Runtime Delivery Mechanism
Resolved answer:
- MVP app-shell updates use the native app-updater path.
- MVP Python runtime delivery uses a separate shell-owned managed runtime-package hydration flow authorized by the commercial backend, verified before use, and kept separate from active-run mutation.
- Later hosting, signing automation, and release-distribution operations remain deferred infrastructure refinements rather than open MVP product-law questions.

Resolution basis:
- Document A V2.A-Final-Draft-3
- `DEC-019`

### OQ-006 — Testing Stack
Resolved answer:
- The canonical repo-level validation baseline is the applicable subset of shell, frontend, and commercial-backend gate families already proven in packet history.
- Documentation-only packets use structural consistency and contradiction review instead of unrelated code gates.

Resolution basis:
- `DEC-021`
- existing clean Phase 3, Phase 4, and Phase 5 gate history

### OQ-007 — Commercial Backend Implementation Surface
Resolved answer:
- Repo-local governed commercial behavior is bounded to the Python-plus-SQLite `commercial_backend/` package with deterministic migrations and a CLI bridge.
- Hosted deployment and real provider-ingress details remain later concerns only.

Resolution basis:
- `DEC-016`
- implemented `commercial_backend/` backend surface

### OQ-008 — Seed Intake Policy Constants
Resolved answer:
- Supported seed documents: PDF, DOCX, TXT, and Markdown.
- Maximum file size: 10 MiB (10,485,760 bytes).
- Minimum usable extracted text: 1,000 normalized characters.
- Non-English rejection threshold: English-confidence below 0.80.

Resolution basis:
- Document A V2.A-Final-Draft-3
- `DEC-020`

## Rule
If any new question affects product truth, execution truth, technical contract meaning, or persistence meaning, escalate before implementing.