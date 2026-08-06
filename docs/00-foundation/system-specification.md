# `docs/00-foundation/system-specification.md`

# Purpose

The Matis System Specification defines the complete conceptual model of the Engineering Memory Operating System.

It describes the system from first principles, independent of programming language, implementation details, or deployment model. This specification serves as the canonical high-level reference for all platform implementations.

---

# Vision & Core Philosophy

Software engineering is a continuous stream of decisions, experiments, failures, and discoveries. Traditional tools capture only fragments:
* Git captures source code diffs.
* IDEs capture raw editing.
* AI assistants capture conversations.
* CI captures builds.
* Issue trackers capture planning.

Matis captures **engineering itself**. Its purpose is to transform raw engineering activity into structured, reproducible engineering intelligence.

---

# Core Platform Principles

1. **Engineering First**: Models domain concepts (Events, Episodes, Memory, ADRs), not database rows or raw files.
2. **Local First & Private**: Knowledge belongs to the developer; local ownership by default.
3. **Immutable History**: Observed reality is append-only and never modified.
4. **Deterministic Intelligence**: Given identical events, the runtime produces identical Episodes, Memory, Knowledge, and Context.
5. **Explainability**: Every recommendation, memory, and context citation includes supporting evidence provenance.
6. **Extensibility**: Small, stable microkernel with growth through first-party modules and sandboxed plugins.

---

# Conceptual Reference Architecture

```text
                                Engineering Reality
                                         │
                                  ┌──────┴──────┐
                                  ▼             ▼
                              Sensors      External Systems
                                  │
                                  ▼
                            Engineering Events
                                  │
                             Event Runtime
                                  │
                                  ▼
                           Engineering Episodes
                                  │
                        ┌─────────┴─────────┐
                        ▼                   ▼
                  Memory Engine      Knowledge Graph
                        │                   │
                        └─────────┬─────────┘
                                  ▼
                           Reasoning Engine
                                  │
                                  ▼
                      Engineering Intelligence (EIL)
                                  │
                                  ▼
                      Context Intelligence (CIE)
                                  │
                                  ▼
                    CLI • IDE • MCP • REST • ECP Protocol • AI
```

---

# Runtime Layers & Canonical Objects

## Runtime Layers (0 to 7)
* **Layer 0 (Kernel)**: Identity, KernelClock, Scheduler, Service Registry, Security bounds.
* **Layer 1 (Sensors)**: Passive activity observation (AI, Git, Shell, IDE, Docker).
* **Layer 2 (Event Runtime)**: Append-only Event Store & Event Bus.
* **Layer 3 (Episodes)**: Unit of engineering work grouping events.
* **Layer 4 (Knowledge)**: Memory Engine & Knowledge Graph extraction.
* **Layer 5 (Reasoning)**: Reasoning Engine & evidence planning.
* **Layer 6 (Intelligence)**: EIL background trend & drift analysis.
* **Layer 7 (Interfaces)**: API front-doors & ECP transport stream adapters.

## The 7 Canonical Objects
`EngineeringEvent`, `EngineeringEpisode`, `EngineeringMemory`, `KnowledgeNode`, `Artifact`, `ContextBundle`, `Project`.

---

# System Invariants & Platform Guarantees

1. Every Engineering Event is immutable and append-only.
2. Every Engineering Object has a stable, globally unique `Identity`.
3. Every derived object has complete provenance back to source events.
4. Every recommendation and context bundle is fully explainable.
5. Every protocol (ECP) is versioned and transport-independent.
6. Every subsystem is replaceable behind stable kernel repository contracts.
7. Every event replay produces 100% deterministic results.
8. Every extension/plugin respects permission sandboxes.
9. Every engineering episode is reconstructable from immutable events.
10. Every public API is capability-based.

Violating any invariant is considered a platform bug.

---

# Platform Maturity Model & Execution Strategy

```text
Level 0: Idea  ──►  Level 1: Architecture (FREEZE)  ──►  Level 2: Formal Specs (*.spec.md)  ──►  Level 3: Reference Implementation  ──►  Level 4: Conformance Tests  ──►  Level 5: Ecosystem  ──►  Level 6: Standard
```

### Next Immediate Deliverables Roadmap
1. **Architecture Freeze**: Complete architecture phase.
2. **Formal Specifications (`specs/*.spec.md`)**: Field-level JSON/Binary schemas, invariants, state machines.
3. **ADRs**: Populate ADR-0001 through ADR-0015.
4. **Rust Workspace & Crate Boundaries**: Define clean Cargo workspace crate boundaries.
5. **Minimal Vertical Slice**: Sensor → Event → Event Store → Episode → Context API.
6. **Integration Verification & Replay**: Implement deterministic test suite and event replay engine.
