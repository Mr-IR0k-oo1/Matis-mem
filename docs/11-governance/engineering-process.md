# Matis Engineering Process & Governance

# Purpose

The Engineering Process defines how Matis is designed, implemented, reviewed, tested, and evolved.

It ensures that architectural quality and stability scale with contributors. The process is designed to preserve correctness, maintainability, and long-term architectural integrity over rapid, unvetted feature creep.

---

# Philosophy & Rule Zero

Software quality is not created during code review — it is created by a disciplined engineering process. The process makes incorrect implementations difficult and correct implementations natural.

### Rule Zero
Before writing code, every contributor must answer:
> **"What specific engineering problem does this solve?"**

If the problem cannot be clearly explained and traced back to a specification, implementation does not begin.

---

# End-to-End Traceability Chain

Every contribution must participate in a 100% complete traceability chain:

```text
Issue ──► ADR (if architectural) ──► Specification (`specs/`) ──► Crate Interface ──► Implementation (`crates/`) ──► Tests (`tests/`) ──► Benchmarks (`benchmarks/`) ──► Documentation ──► Release
```

Nothing lives in isolation.

---

# Definitions of Ready & Done

## Definition of Ready (DoR)
Work begins only when:
1. Requirements and problem statement are clearly defined.
2. Formal specification exists under `specs/`.
3. Crate dependencies & trait boundaries are resolved.
4. Acceptance criteria and performance gates are explicit.

## Definition of Done (DoD)
A task or PR is complete only when:
```text
Implementation (`crates/`)        [✓]
Tests (`tests/` & Unit)           [✓]
Replay Verification               [✓]
Conformance Suite                 [✓]
Criterion Benchmarks              [✓]
Documentation (`docs/` & API)     [✓]
Code Review Approval              [✓]
```

---

# Design & Code Review Guidelines

## Mandatory Design Reviews
Architectural review is strictly required when altering:
* Microkernel (`matis-kernel`)
* Object Model (`matis-objects`)
* Protocol (`matis-protocol`)
* Storage interfaces (`matis-storage`)
* Replay Engine & `KernelClock`
* Public Trait Boundaries (`pub mod api`)

## Constructive Actionable Reviewing
Reviews focus on: Correctness ──► Simplicity ──► Security ──► Performance ──► Maintainability.

Generic reviews like *"Looks good"* are prohibited. Reviews must cite specific platform invariants (e.g. *"Replay invariant violated in episode_engine.rs:L140"* or *"Public API leaks internal SQLite handle"*).

---

# Testing & Performance Policy

* **Testing Stack**: Every crate requires Unit Tests, Integration Tests, Property Tests, Replay Tests, and Conformance Tests.
* **Performance Work**: Optimization claims must include pre-change and post-change Criterion benchmark results. *No benchmark = no optimization claims.*
* **Dependency Policy**: Adding an external crate requires explicit justification regarding security, license, maintenance, and size impact.

---

# 10 Core Governance Invariants

1. Every architectural change is documented via an ADR (`docs/01-adrs/`).
2. Every implementation traces back to a formal specification (`specs/`).
3. Every specification has matching conformance tests (`conformance/`).
4. Every performance claim is backed by benchmarks (`benchmarks/`).
5. Every release passes 100% of deterministic event replay tests.
6. Every external dependency is explicitly justified.
7. Every public API is fully documented with examples and failure modes.
8. Every breaking change is intentional and versioned.
9. Every code review enforces platform invariants.
10. Every merged feature strengthens architectural clarity rather than expanding bloat.

---

# Code-to-Docs Ratio Target

With governance established, the documentation phase is complete. The repository now shifts focus to executable specifications, test suites, and production Rust code:

```text
Target Repository Weight:
  docs/       [████████]       (20%)
  specs/      [██████████]     (25%)
  crates/     [██████████████] (35%)
  tests/      [████████]       (20%)
```
