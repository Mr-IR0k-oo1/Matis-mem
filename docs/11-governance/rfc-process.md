# Matis Request for Comments (RFC) Process

# Purpose

The Request for Comments (RFC) process defines how major features, architectural changes, protocols, and platform evolution are proposed, discussed, reviewed, and accepted.

RFCs prevent architectural drift by ensuring significant changes are evaluated systematically before implementation. Every foundational change begins as an RFC.

---

# Philosophy & Scope

Ideas are cheap; stable architecture is expensive. The RFC process exists to refine ideas before they become code. Implementation follows consensus, not enthusiasm.

### When is an RFC Required?
An RFC is strictly required for changes involving:
* Microkernel architecture (`matis-kernel`)
* Engineering Object Model (`matis-objects`)
* Engineering Context Protocol (`matis-protocol`)
* Public APIs (`matis-api`)
* Plugin SDK (`matis-plugin-sdk`)
* Storage architecture (`matis-storage`)
* Security model & permission sandboxing
* Deterministic Replay & `KernelClock` semantics
* Schema or Specification changes (`specs/`)
* Backward-compatibility impact

Bug fixes and minor refactors do **not** require RFCs.

---

# The RFC Lifecycle

```text
Draft RFC  ──►  Community Discussion  ──►  Technical Review  ──►  Accepted  ──►  ADR  ──►  Specification Update  ──►  Implementation  ──►  Release
```

An accepted RFC becomes a permanent part of the project design. If an RFC is rejected, it remains archived in `docs/12-rfcs/` to provide historical context and prevent repeating discarded proposals without new evidence.

---

# Repository Layout (`docs/12-rfcs/`) & Template

```text
docs/
└── 12-rfcs/
    ├── README.md                          (This process document)
    ├── template.md                        (Standard RFC template)
    ├── RFC-0001-engineering-episodes.md   (Accepted)
    ├── RFC-0002-context-protocol.md       (Accepted)
    ├── RFC-0003-plugin-sandbox.md         (Accepted)
    └── RFC-0004-distributed-runtime.md    (Proposed)
```

### Standard RFC Template (`docs/12-rfcs/template.md`)
```markdown
# RFC-XXXX: [Short Title]

## Summary
Brief explanation of the proposed change.

## Motivation
What problem does this solve? Why is this change necessary now?

## Goals & Non-Goals
- **Goals**: Explicit outcomes.
- **Non-Goals**: Explicitly excluded scope.

## Detailed Design
Technical architecture, data structures, and trait boundaries.

## Alternatives Considered
Why this design over other options?

## Tradeoffs & Drawbacks
What overhead or complexity does this introduce?

## Compatibility & Migration
Impact on existing events, replays, APIs, and client configurations.

## Security & Replay Impact
Does this preserve deterministic replay, local-first privacy, and secret redaction?

## Open Questions
Unresolved issues requiring feedback.
```

---

# Relationship Between RFCs, ADRs, Specs & Code

```text
Issue ──► RFC (Proposal & Debate) ──► ADR (Decision Record) ──► Specification (`specs/`) ──► Implementation (`crates/`) ──► Conformance Tests (`conformance/`)
```

* **RFC**: Exploratory proposal and technical debate.
* **ADR**: Canonical historical decision record (`docs/01-adrs/`).
* **Specification**: Exact behavioral contracts (`specs/*.spec.md`).
* **Code**: Rust implementation (`crates/`).

---

# Decision Authority Matrix

| Scope | Approval Authority | Requirement |
|---|---|---|
| Bug Fix / Small Patch | Crate Maintainer | Passing tests & lints |
| New Module / Internal Feature | Module Lead | Specification update & tests |
| Public API / Transport Change | Architecture Review | RFC + Specification update |
| Protocol / Core Engine Change | Core Team | RFC + ADR + Spec + Conformance tests |
| Microkernel / Object Model | Core Team + Unanimous ADR | RFC + ADR + Spec + Conformance + Replay |
| Constitution Article Change | Major Version Release Only | Unanimous Core Team & Community RFC |

---

# 10 Core RFC Invariants

1. Major changes are reviewed systematically before implementation.
2. Architectural evolution is completely documented and transparent.
3. Historical proposals (accepted or rejected) remain permanently accessible.
4. Specifications evolve deliberately through formal consensus.
5. Backward compatibility & migration paths are explicit.
6. Security and permission implications are thoroughly audited.
7. Performance expectations and benchmarks are documented.
8. Deterministic event replay and `KernelClock` semantics are preserved.
9. Every accepted RFC has a traceable implementation path (`RFC -> ADR -> Spec -> Code`).
10. Architecture evolves intentionally rather than accidentally.

---

# Architecture Phase Completion Declaration

With this RFC process defined, the **Architectural Documentation Phase is Officially Complete**.

The project repository weight will now shift into formal specifications (`specs/`), executable test suites (`tests/`), Criterion benchmarks (`benchmarks/`), and Rust implementation crates (`crates/`):

```text
Repository Focus Ratio:
  [20% Documentation] ──► [30% Formal Specs] ──► [40% Rust Crates Implementation] ──► [10% Conformance & Benchmarks]
```
