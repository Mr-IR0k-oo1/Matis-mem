# `docs/07-query-engine/retrieval-engine.md`

# Purpose

The Retrieval Engine is responsible for selecting, ranking, assembling, and explaining engineering knowledge in response to a query.

Unlike traditional search systems, the Retrieval Engine does not retrieve raw documents. It retrieves **engineering understanding**.

Its goal is to provide the smallest amount of information that enables the best engineering decision.

---

# Philosophy

Developers rarely need more information — they need the **right information**.

Given a question, the Retrieval Engine prioritizes:
* Relevance
* Correctness
* Traceability
* Freshness
* Signal Density

Not document count.

---

# Subsystem Architectural Evolution: The Reasoning Engine

To prevent scattering planning, query parsing, retrieval, and explanation across disconnected modules, the Query Engine and Retrieval Engine are unified into the **Reasoning Engine**:

```text
                               Reasoning Engine
 ┌──────────────────────────────────────────────────────────────────────────┐
 │                                                                          │
 │  Intent Compiler  ──►  MQL Parser  ──►  Retrieval Planner                │
 │                                                  │                       │
 │                                                  ▼                       │
 │  Candidate Generator ◄── Graph Traverser ◄── Memory Selector             │
 │          │                                                               │
 │          ▼                                                               │
 │   Ranking Engine  ──►  Compression Engine  ──►  Context Composer          │
 │                                                      │                   │
 │                                                      ▼                   │
 │                                            Explanation Generator         │
 └──────────────────────────────────────────────────────────────────────────┘
```

The Reasoning Engine forms the central intelligence layer of Matis.

---

# Retrieval Pipeline

Every request follows a strict pipeline:

```text
Receive Query  ──►  Understand Intent  ──►  Generate Candidates  ──►  Rank Candidates  ──►  Filter  ──►  Compress  ──►  Assemble Context  ──►  Explain Selection  ──►  Return Result
```

No stage is skipped.

---

# Retrieval Sources & Priority Tiering

1. **Working Memory**: Current engineering state (Priority: **Very High**)
2. **Semantic Memory**: Permanent engineering knowledge & ADRs (Priority: **Very High**)
3. **Knowledge Graph**: Relationship edges & causal links (Priority: **High**)
4. **Timeline**: Chronological engineering history (Priority: **Medium**)
5. **Event Store**: Raw canonical engineering events (Priority: **Low**)
6. **Archives**: Compressed historical event logs (Priority: **Lowest**)

---

# Candidate Generation & Ranking Signals

Candidates are scored by the `RankingEngine` using:
* **Query Similarity**: Keyword & semantic vector alignment.
* **Importance**: Low, Medium, High, Critical.
* **Confidence**:
  * *Observed*: `1.0`
  * *Derived*: `0.92`
  * *AI Inference*: `0.71`
* **Freshness**:
  * *Current Branch*: Fresh
  * *Yesterday's Build*: Recent
  * *Architecture Decision*: Stable
* **Context Signals**: Active repository, branch, open files, active objective, project constraints.

---

# Progressive Compression Pipeline

To strictly honor token budgets:

```text
Raw Event  ──►  Summary  ──►  Knowledge Object  ──►  Reference  ──►  Citation
```

Meaning is preserved while redundancy is removed.

---

# Conflict Resolution & Evolution Tracking

When conflicting decisions exist (e.g. `Decision A: Use SQLite` superseded by `Decision B: Move to PostgreSQL`), the engine retrieves:
* Current active decision
* Superseded previous decision
* Rationale and event trail causing the change

Historical evolution remains transparent and traceable.

---

# Retrieval Modes

* **Fast Mode**: Sub-20ms lookup via Working Memory & Semantic Memory.
* **Balanced Mode** (Default): Sub-100ms retrieval via Working Memory, Semantic Memory, Knowledge Graph, and Timeline.
* **Deep Mode**: Comprehensive search including raw events, deep graph traversal, and archives.

---

# Explainability & Citations

Every retrieved item includes:
```json
{
  "item_id": "mem_20260806_01",
  "selection_reason": "Referenced by 7 recent commits on active branch",
  "confidence": 0.98,
  "source_events": ["ev_20260806_1001", "ev_20260806_1002"]
}
```

Users never wonder *"Why was this included?"*.

---

# Core Invariants

1. Every retrieved item originates from immutable Engineering Events.
2. Ranking is deterministic for identical inputs and data.
3. Confidence scores are preserved throughout the pipeline.
4. Progressive compression never fabricates information.
5. User-pinned knowledge overrides automatic ranking.
6. Retrieval remains storage-independent.
7. Explainability accompanies every result.
8. Sensitive information is filtered before output.
9. Context budgets are strictly enforced.
10. Retrieval never mutates Engineering Events.
