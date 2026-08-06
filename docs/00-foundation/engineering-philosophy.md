# Matis Engineering Philosophy

# Purpose

The Engineering Philosophy defines the intellectual foundation of Matis.

It explains how engineering decisions are evaluated, how complexity is controlled, and what values the platform prioritizes above all else. Unlike transient software implementations, this philosophy is timeless and endures across rewrites and major versions.

---

# The 10 Enduring Principles

1. **Reality Is More Valuable Than Opinion**: Matis trusts observations (`EngineeringEvent`), not assumptions or unverified heuristics. Interpretations evolve; observed facts remain permanent.
2. **Complexity Is a Cost**: Every abstraction must justify its existence. If removing a feature simplifies the platform without losing core capability, the feature should not exist.
3. **Time Is a First-Class Dimension**: Engineering is not static snapshots. History, context, and decisions exist in time. Observed history is never discarded.
4. **Knowledge Must Compound**: A solved problem should never become an unsolved problem again. Every solved issue and recorded ADR makes future engineering easier.
5. **Systems Should Explain Themselves**: Opacity is a defect. Every recommendation, citation, relationship, and insight must explain why it exists and what evidence supports it.
6. **Humans Own Decisions**: Matis assists; engineers decide. Recommendations are advisory. Engineering responsibility remains with human developers.
7. **Local Knowledge Has Priority**: The local machine contains the richest context. Offline execution is a strict requirement, not an enhancement.
8. **Architecture Is an Investment**: Good architecture slows down the first month to accelerate the next ten years.
9. **Replay Is the Ultimate Test**: Replay reconstructs reality. If event log replay fails or produces non-deterministic results, the system has lost self-trust.
10. **Small Foundations, Large Ecosystems**: The microkernel remains small and domain-agnostic (`matis-kernel`). Platform growth happens outward via plugins and ECP protocol streams.

---

# The 4 Canonical Sources of Truth

Everything inside Matis derives strictly from these four sources:

```text
Reality  ──►  Engineering Events  ──►  Specifications (`specs/`)  ──►  Conformance Tests (`conformance/`)
```

---

# Decision Heuristics & Acceptable Debt

When choosing between technical designs, prefer the option that:
* Preserves immutable event history
* Requires fewer concepts
* Produces deterministic behavior under `KernelClock`
* Is simpler to explain and replay

### Acceptable vs Unacceptable Technical Debt
* **Acceptable Debt**: Temporary implementation shortcuts, benchmarked performance tradeoffs, deferred non-v1 optimizations.
* **Unacceptable Debt**: Architectural ambiguity, multiple sources of truth, broken replay determinism, missing formal specifications, undefined object ownership.

---

# Architecture Complete (v1.0) & Documentation Freeze

With this document committed, the **Architecture Phase is Officially FROZEN (v1.0)**.

The project center of gravity shifts 100% to:
1. **Formal Specifications (`specs/*.spec.md`)**
2. **Executable Test Suites (`tests/`) & Replay Verification**
3. **Rust Workspace Implementation (`crates/`)**
4. **Criterion Latency & Allocation Benchmarks (`benchmarks/`)**
