# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

---

## Reporting a Vulnerability

We take the security of Preflight and its handling of local user credentials seriously.

If you believe you have discovered a security vulnerability regarding:
- Credential isolation (Windows DPAPI / keyring)
- Local IPC trust boundary between webview and shell
- Ephemeral workspace isolation or data leakage
- Prompt injection or untrusted seed file parsing

Please **do not** report security vulnerabilities through public GitHub issues.

Instead, please send an email with the details to the project maintainer or file a private security advisory via GitHub Security Advisories.

Please include:
- A description of the vulnerability and its potential impact.
- Steps to reproduce the issue or a proof of concept.
- Any suggested fixes or mitigations.

We will acknowledge receipt of your report within 48 hours and work with you to release a patch promptly.
