# Matis Project Governance

This document defines the decision-making model and maintainer responsibilities for the Matis open-source project.

## Maintainer Roles & Responsibilities

- **Core Team**: Owns kernel architecture, microkernel boundaries, object specifications, and security policies. Unanimous consent required for constitutional or major version changes.
- **Module Leads**: Maintain specific domain crates (`matis-episodes`, `matis-memory`, `matis-graph`, `matis-context`, `matis-sensors`). Review PRs and enforce specs.
- **Contributors**: Propose issues, write specs, implement code, submit tests, and improve documentation.

## Decision Authority Matrix

| Scope | Approval Authority | Requirement |
|---|---|---|
| Bug Fix / Patch | Module Maintainer | Passing tests & lints |
| New Subcommand / Feature | Module Lead | Specification update & unit tests |
| Public API / Transport Change | Architecture Review | RFC + Spec update |
| Core Engine / Event Store | Core Team | RFC + ADR + Spec + Conformance tests |
| Kernel / Object Model | Core Team Unanimous | RFC + ADR + Spec + Conformance + Replay |
