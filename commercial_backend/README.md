# Preflight Enterprise Backend (Reference Implementation)

> **Architectural Note:** Under Preflight's Open Core Governance Model ([ADR-001](../docs/decisions/adr_001_open_core_transition.md)), the core Preflight simulation engine, headless CLI, and local desktop application are **100% free and open-source under the Apache License, Version 2.0**.

---

## Purpose of this Module

This directory contains an optional reference implementation of an offline ledger and commercial license validation service. It is designed solely for enterprise organizations requiring air-gapped corporate license governance, audit logging, and seat management.

### Key Guarantees:
1. **Completely Decoupled:** The open-source Preflight engine (`engine/`) runs fully autonomously in `ExecutionMode::Community` with zero dependencies on this backend.
2. **Zero Telemetry:** Neither the community engine nor the shell makes outbound calls to this service unless an enterprise user explicitly configures an enterprise license endpoint.
3. **Bring-Your-Own-Key (BYOK):** Standard users only provide their own OpenAI API key, which is kept strictly in local OS credential vaults (Windows DPAPI).

For open-source usage and evaluation, see the main [README](../README.md) and [Architecture Guide](../docs/ARCHITECTURE.md).
