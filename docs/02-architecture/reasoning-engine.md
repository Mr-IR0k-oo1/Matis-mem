# `docs/02-architecture/reasoning-engine.md`

# Purpose

The Reasoning Engine is responsible for transforming engineering intent into engineering understanding.

It orchestrates retrieval, graph traversal, episode analysis, memory selection, planning, constraint analysis, risk assessment, and context composition.

It does not generate code.

It prepares the highest quality engineering context for humans and AI systems.

---

# Philosophy

Searching is passive. Reasoning is active.

Given the request *"Continue OAuth implementation"*:

* **Search Engine**: Returns `JWT`, `OAuth`, `Authentication`, `middleware.rs`.
* **Reasoning Engine**: Returns active objective, previous implementations, architecture decisions, known bugs, recent build failures, active working files, pending TODOs, recommended next actions, and potential risks.

It understands engineering as structured units of work.

---

# Key Architectural Evolution: Engineering Episodes

Rather than treating millions of raw events as independent items, Matis organizes work into **Engineering Episodes**:

```text
                               Engineering Events
                                       │
                                       ▼
                                Episode Builder
                                       │
                                       ▼
                              Engineering Episodes
                                       │
            ┌──────────────────────────┼──────────────────────────┐
            ▼                          ▼                          ▼
        Timeline                     Memory                Knowledge Graph
            │                          │                          │
            └──────────────────────────┼──────────────────────────┘
                                       ▼
                              Reasoning Engine
                                       │
                                       ▼
                               Final AI Context
```

An **Engineering Episode** is a complete unit of engineering work (e.g. *Implement JWT Authentication*, *Migrate runtime to Tokio*, *Fix lifetime bug in parser*).

An Episode contains:
* Grouped Engineering Events
* AI conversations & responses
* Commits & file diffs
* Build/test outcomes
* Decisions & rationale
* Lessons learned & outcome status

Humans and staff engineers do not remember individual events; they remember *episodes*.

---

# Subsystem Architecture

```text
Reasoning Engine
├── Intent Compiler       (Classifies requests into Debugging, Continuation, Analysis, Refactoring, etc.)
├── Episode Builder       (Assembles raw events into cohesive Engineering Episodes)
├── Reasoning Planner     (Plans retrieval strategy across memory, graph, and episodes)
├── Evidence Collector    (Gathers immutable evidence from working memory, semantic memory, and graph)
├── Graph Traverser       (Traverses causal node/edge relationships)
├── Constraint Analyzer   (Evaluates persistent engineering constraints like offline-first, SQLite-only)
├── Conflict Detector     (Surfaces superseded decisions, conflicting constraints, and obsolete knowledge)
├── Risk Analyzer         (Flags breaking API changes, security impacts, or merge conflict risks)
├── Recommendation Engine (Generates next-step recommendations supported by evidence)
├── Context Composer      (Assembles final signal-dense engineering context)
└── Explanation Generator (Attaches explainability citations and confidence scores)
```

---

# Reasoning Pipeline

```text
Request  ──►  Intent  ──►  Episode Lookup  ──►  Plan  ──►  Evidence  ──►  Constraints  ──►  Conflicts  ──►  Risks  ──►  Recommendations  ──►  Context
```

Every stage produces traceable output.

---

# Intent & Planning Matrix

| Intent | Primary Retrieval Target | Key Risk Signals | Output Emphasis |
|---|---|---|---|
| **Debugging** | Build failures, recent diffs, episode history | Regression, breaking diffs | Error context, failure history, previous fixes |
| **Continuation** | Active Episode, working memory, recent commits | Stale context, open TODOs | Current objective, active branch, next steps |
| **Analysis** | Knowledge Graph, Semantic Memory, ADRs | Conflict, superseded decisions | Architectural rationale, graph connections |
| **Refactoring** | Dependency graph, test suites, API contracts | Breaking API, test failures | Impacted modules, constraints, safety checks |

---

# Conflict & Risk Detection

The engine actively surfaces engineering conflicts and risks:
* **Superseded Decisions**: e.g., `Decision V2: Axum` supersedes `Decision V1: Actix`.
* **Constraint Violations**: e.g., attempting a dynamic allocation in an embedded target.
* **Regression Risks**: e.g., modifying files with a history of recent build failures.

Nothing is silently swallowed or ignored.

---

# Recommendation Engine & Explainability

Recommendations cite supporting evidence:

```text
Recommendation: Implement refresh-token validation in claims.rs
Reason: Modified in previous 4 authentication commits within Episode ep_20260806_oauth.
Confidence: 0.94
Supporting Evidence:
  - 18 Engineering Events
  - 3 Architecture Decisions
  - 7 Commits on branch feature/auth
```

Confidence is never presented as absolute certainty.

---

# Learning & Performance Targets

* **Planning**: `<10 ms`
* **Reasoning**: `<50 ms`
* **Context Assembly**: `<100 ms`
* **Incremental Episode Update**: `<20 ms`

Reasoning improves as accepted/rejected recommendation feedback updates confidence scores.

---

# Core Invariants

1. Every conclusion is supported by immutable Engineering Events.
2. Recommendations are fully explainable with provenance.
3. Conflicting evidence is surfaced, never hidden.
4. Reasoning never mutates historical data.
5. Context respects configured token budgets.
6. User-confirmed knowledge overrides inferred knowledge.
7. Confidence is never presented as certainty.
8. Retrieval and reasoning remain deterministic for identical inputs.
9. AI providers receive the same reasoning output for the same request.
10. Engineering Episodes are the primary unit of historical reasoning.
