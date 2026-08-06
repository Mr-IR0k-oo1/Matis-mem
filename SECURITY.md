# Security Policy

## Reporting Security Vulnerabilities

Matis treats security as a fundamental platform invariant (Article XVII: Privacy by Default).

If you discover a security vulnerability or secret leakage risk in Matis:
1. **Do NOT open a public GitHub issue.**
2. Report the vulnerability privately via email to `security@matis-mem.org` or through GitHub Private Vulnerability Reporting.
3. Include detailed steps to reproduce, impacted crates/modules, and potential impact.

## Local Privacy & Redaction Invariants

- **Local-First Default**: Matis operates offline by default; zero telemetry leaves the machine.
- **Secret Redaction**: Sensors automatically filter API keys, tokens, passwords, and private SSH keys before publishing events to the Event Bus.
- **Sandboxed Extensions**: Third-party plugins execute within strict WASM or process permission boundaries (`crates/matis-plugin-sdk`).
