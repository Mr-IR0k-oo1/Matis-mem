# Architecture Decision Records (ADRs)

# Purpose

Architecture Decision Records (ADRs) capture the reasoning behind major architectural choices made throughout the lifecycle of Matis.

Unlike design documents, ADRs explain **why** a decision was made, what alternatives were considered, and the consequences of adopting that decision.

ADRs form the permanent long-term engineering memory of the project.

---

# Philosophy

* **Code**: Explains *how*.
* **Documentation**: Explains *what*.
* **Architecture**: Explains *how it fits together*.
* **ADRs**: Explain *why*.

Without ADRs, architectural intent disappears as contributors change and the system evolves.

---

# ADR Principles

Every ADR must be:
* Immutable after acceptance.
* Versioned.
* Linked to supporting evidence (events, benchmarks, episodes).
* Traceable to discussions and Engineering Episodes.
* Explainable to future contributors.

New decisions **supersede** old decisions. They never rewrite history.

---

# Lifecycle

```text
Proposed  ──►  Review  ──►  Accepted  ──►  Implemented  ──►  Superseded  ──►  Archived
```

Historical decisions remain 100% accessible.

---

# Repository Structure (`docs/01-adrs/`)

```text
docs/
└── 01-adrs/
    ├── README.md                           (This document)
    ├── ADR-0001-engineering-events.md      (Events are immutable and append-only)
    ├── ADR-0002-episodes.md                (Episodes are first-class units of work)
    ├── ADR-0003-local-first.md             (Local-first architecture & privacy)
    ├── ADR-0004-object-model.md            (Unified Engineering Object hierarchy)
    ├── ADR-0005-microkernel.md             (Single-process microkernel design)
    ├── ADR-0006-ecp-protocol.md            (Engineering Context Protocol - ECP)
    ├── ADR-0007-storage-abstraction.md     (Storage repository interfaces)
    ├── ADR-0008-sensors.md                 (Passive sensor / adapter pipeline)
    ├── ADR-0009-capability-api.md          (Capability-based API abstraction)
    ├── ADR-0010-reasoning-engine.md        (Unified Reasoning Engine & CIE)
    ├── ADR-0011-distillation-refinement.md (Knowledge Refinement Loop)
    ├── ADR-0012-plugin-sandbox.md          (Fault-isolated plugin system)
    ├── ADR-0013-mql-ast.md                 (Two-layer MQL AST & intent compiler)
    ├── ADR-0014-deterministic-clock.md     (KernelClock for replayability)
    └── ADR-0015-replay-driven-recovery.md  (Deterministic event replay recovery)
```

Numbers never change. Titles may evolve.

---

# ADR Template (`docs/01-adrs/template.md`)

```markdown
# ADR-XXXX: [Short Title]

## Status
[Draft | Proposed | Accepted | Implemented | Deprecated | Superseded | Rejected | Archived]

## Date
YYYY-MM-DD

## Authors
[Author Names / Handle]

## Context
What problem are we trying to solve? Why is this decision required now?

## Decision
What is the explicit architectural choice we are making?

## Alternatives Considered
- **Option A**: [Description & reason rejected]
- **Option B**: [Description & reason rejected]

## Consequences
- **Positive**: [Benefits gained]
- **Negative**: [Tradeoffs & overhead accepted]
- **Neutral**: [Architectural implications]

## Evidence & Traceability
- **Supporting Events / Episodes**: [Episode ID / Event IDs]
- **Benchmarks / Prototypes**: [Results / Metrics]

## Related ADRs & Episodes
- Supersedes: [ADR-XXXX]
- Related: [ADR-YYYY], [Episode-ZZZZ]
```

---

# 10 Core Invariants

1. Architectural decisions are never lost or deleted.
2. Every decision has explicit reasoning and tradeoff analysis.
3. Superseded decisions remain permanently accessible.
4. Decisions are linked to engineering evidence and supporting episodes.
5. Architectural evolution is 100% traceable.
6. Every implementation choice can reference a canonical decision.
7. Historical context is preserved.
8. Tradeoffs are explicitly documented, not implied.
9. ADRs are immutable after entering the `Accepted` state.
10. System architecture can be reconstructed from the ADR history alone.

---

# Next Immediate Execution Steps (Roadmap)

With the core architecture fully specified across `docs/00-foundation`, `docs/01-vision`, `docs/02-architecture`, `docs/03-memory`, `docs/05-capture`, `docs/07-query-engine`, and `docs/08-api`, development shifts to execution:

1. **Formal Specifications (`*.spec.md`)**: Define exact JSON/Binary schemas, field types, validation rules, state machines, and error conditions (`engineering-event.spec.md`, `engineering-episode.spec.md`, `engineering-memory.spec.md`, `engineering-context.spec.md`, `engineering-protocol.spec.md`).
2. **ADR Population**: Commit ADR-0001 through ADR-0015.
3. **Crate Boundaries**: Define Rust workspace crates (`matis-kernel`, `matis-events`, `matis-episodes`, `matis-memory`, `matis-graph`, `matis-reasoning`, `matis-api`, `matis-storage`, `matis-sensors`).
4. **Executable Invariant Tests**: Implement integration unit/property tests for object immutability, replayability, and token budgets.
5. **Implementation Execution**: Incrementally code engines against formal specifications.
