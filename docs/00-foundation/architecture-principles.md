# Matis Architecture Principles

# Purpose

The Architecture Principles define the practical engineering philosophy that guides everyday design decisions within Matis.

Unlike the Constitution (which defines non-negotiable platform values), these principles provide practical heuristics for choosing between competing technical implementations.

---

# The 20 Architectural Principles

1. **Reality Before Interpretation**: Events are observed facts; episodes and memories are derived. Never fabricate events to fit an abstraction.
2. **Build Upwards**: `Events -> Episodes -> Memory -> Knowledge -> Reasoning -> Intelligence -> Context`. Dependencies strictly point downward; higher layers never re-define lower layers.
3. **One Responsibility**: Every crate and module owns exactly one domain concept. If a module owns two concepts, split it.
4. **Derive, Don't Duplicate**: Derived entities reference lower-layer `Identity` keys — data payloads are never duplicated.
5. **Stable Contracts**: Interfaces and traits outlive concrete implementations. Every trait should outlive at least three internal engine rewrites.
6. **Explicit Over Implicit**: Hidden mutations, background magic, and implicit global state are strictly forbidden.
7. **Immutable History**: Events are append-only. Corrections create compensating events, never inline edits.
8. **Explainability**: Every memory, recommendation, relationship, and citation must include supporting evidence provenance (`Why do I believe this?`).
9. **Composition Over Inheritance**: Rust traits, composition, and capability definitions over deep struct coupling.
10. **Protocol Before Transport**: Semantics belong to the Engineering Context Protocol (ECP); HTTP, MCP, WebSockets, and Unix Sockets are mere transport front-doors.
11. **Kernel Minimalism**: The microkernel owns only identity, time (`KernelClock`), priority scheduling, capability registry, service registry, lifecycle, and security. Zero domain logic.
12. **Specifications Before Code**: `Specification (specs/) -> Tests (tests/) -> Implementation (crates/)`. Never code without a specification.
13. **Replay Is Truth**: Event replay is the ultimate validation. If replay produces different state without an explicit algorithm change, it is a defect.
14. **Simplicity Compounds**: Prefer three small, predictable subsystems over one clever, monolithic subsystem.
15. **Optimize Last**: Optimization order: Correctness ──► Determinism ──► Simplicity ──► Security ──► Maintainability ──► Performance.
16. **Replaceability**: Storage backends, Graph engines, Retrieval algorithms, and Plugins must be 100% replaceable behind stable trait interfaces.
17. **Local-First**: Assume offline execution, zero network, zero cloud, zero telemetry by default.
18. **Engineering Objects Are Sacred**: The 7 canonical object types are strictly governed. Adding a core object requires an RFC, ADR, and Specification.
19. **Events Are Forever**: Derived data may evolve or be rebuilt; raw Engineering Events persist permanently in the append-only Event Store.
20. **Build for Decades**: Every design choice must answer: *"Will this still make sense ten years from now?"*

---

# Decision Evaluation Framework

When evaluating competing pull requests or technical designs, score them in this exact order:

```text
1. Preserves immutable history?
2. Preserves replay determinism under KernelClock?
3. Reduces conceptual complexity?
4. Improves explainability & evidence provenance?
5. Maintains subsystem replaceability?
6. Strengthens public trait contracts?
7. Simplifies implementation code?
8. Improves performance without compromising items 1–7?
```

The first criterion that differs determines the superior engineering choice.

---

# Architectural Smells (CI & Review Flags)

Treat as severe architectural defects during review:
* Circular crate or module dependencies
* Mutable `EngineeringEvent` mutations or deletions
* Duplicate object models across crates
* Public APIs exposing storage handles or database specifics (e.g. SQLite structs)
* Hidden background global mutability
* Cross-layer reverse imports (e.g. Storage depending on Reasoning)
* Features without formal specifications under `specs/`
* Optimization claims without Criterion benchmark data

---

# Architectural Documentation Freeze Declaration

With these Architecture Principles committed, the **Documentation Phase is officially complete and FROZEN**.

The repository transitions completely into formal specifications (`specs/`), executable test suites (`tests/`), Criterion performance benchmarks (`benchmarks/`), and production Rust workspace crates (`crates/`):

```text
                                DOCUMENTATION FREEZE (v1.0)
                                            │
                                            ▼
                           Formal Specifications (`specs/*.spec.md`)
                                            │
                                            ▼
                         Crate Trait Interfaces & Workspace Setup
                                            │
                                            ▼
                           Executable Vertical Slice Implementation
```
