# Matis Technical Strategy

# Purpose

The Technical Strategy defines the long-term engineering direction of Matis over a multi-year horizon.

Unlike the Implementation Roadmap (which defines *what gets built when*), the Technical Strategy specifies *why specific technical investments are made* and *which capabilities compound over time*.

---

# Strategic North Star

Every major technical investment must directly advance this core capability:

> **Any engineer should be able to resume any engineering effort, on any supported project, with complete, explainable, replayable engineering context.**

If a feature or optimization does not move the platform toward this North Star, its priority is deprioritized.

---

# The 5 Strategic Pillars

```text
 1. Engineering Reality     (Sensors, Event Capture, Identity, Replay Engine, Event Runtime)
 2. Engineering Memory      (Episodes Engine, Working & Semantic Memory, Knowledge Refinement)
 3. Engineering Reasoning   (Evidence Collector, Reasoning Engine, CIE, MQL AST)
 4. Engineering Intelligence (EIL Drift Detection, Trend Analysis, Health Scores, Feedback Loops)
 5. Engineering Platform    (ECP Protocol Standard, Plugin SDK Sandbox, Language Bindings)
```

---

# Multi-Generation 10-Year Progression

```text
Gen 1: Engineering Memory       (Sensors ──► Events ──► Event Store ──► Episodes ──► CIE Context)
  │
  ▼
Gen 2: Engineering Knowledge    (Working/Semantic Memory ──► Knowledge Graph ──► Reasoning Engine)
  │
  ▼
Gen 3: Engineering Intelligence (EIL Drift Detection ──► Health Scores ──► Proactive Recommendations)
  │
  ▼
Gen 4: Engineering Runtime      (ECP Standard Protocol ──► WASM Plugin SDK ──► Federation & Sync)
```

---

# Compounding vs Non-Compounding Investments

| High-Compounding Investments (PRIORITY) | Low-Compounding Investments (DEPRIORITIZED) |
|---|---|
| Deterministic Replay Engine (`KernelClock`) | UI Redesigns & Cosmetic Tweak Iterations |
| Canonical Object Model (`EngineeringObject`) | Temporary/Ad-hoc AI chat wrappers |
| Open Protocol Standard (`ECP`) | Proprietary SaaS Cloud Analytics dashboards |
| Formal Behavioral Specifications (`specs/*.spec.md`) | Single-provider AI tie-ins |
| WASM & Process Plugin Isolation (`matis-plugin-sdk`) | Unsubstantiated performance hacks without benchmarks |

---

# Living Architecture Dashboard & System Status

| Subsystem | Spec (`specs/`) | Crate (`crates/`) | Tests (`tests/`) | Conformance | Status |
|---|---|---|---|---|---|
| **Kernel / Microkernel** | `specs/kernel.spec.md` | `matis-kernel` | `tests/kernel/` | Pass | **Stable** |
| **Object Model (EOS)** | `specs/engineering-object.spec.md` | `matis-objects` | `tests/objects/` | Pass | **Stable** |
| **Event Runtime** | `specs/engineering-event.spec.md` | `matis-events` | `tests/replay/` | Pass | **Stable** |
| **Storage Engine** | `specs/storage.spec.md` | `matis-storage` | `tests/storage/` | Pass | **Stable** |
| **Episode Engine** | `specs/engineering-episode.spec.md` | `matis-episodes` | `tests/episodes/` | In Progress | **In Progress** |
| **Context Engine (CIE)** | `specs/engineering-context.spec.md` | `matis-context` | `tests/context/` | Pass | **Stable** |
| **Reasoning Engine** | `specs/reasoning.spec.md` | `matis-reasoning` | `tests/reasoning/` | Planned | **Phase 4** |
| **Memory Engine** | `specs/engineering-memory.spec.md` | `matis-memory` | `tests/memory/` | Planned | **Phase 5** |
| **Knowledge Graph** | `specs/graph.spec.md` | `matis-graph` | `tests/graph/` | Planned | **Phase 6** |
| **Intelligence (EIL)** | `specs/intelligence.spec.md` | `matis-intelligence` | `tests/intelligence/` | Planned | **Phase 7** |
| **Protocol (ECP)** | `specs/ecp-protocol.spec.md` | `matis-protocol` | `tests/protocol/` | Planned | **Phase 8** |

---

# Strategic Constraints & Non-Negotiables

1. Local-first remains the default; zero cloud dependence.
2. Engineering Events are immutable and append-only.
3. Event Replay remains 100% deterministic under `KernelClock`.
4. Engineering Objects (`EOS`) remain canonical across all transports.
5. Public trait boundaries remain stable; implementations remain replaceable.
6. Formal specifications (`specs/`) are authoritative over code.
