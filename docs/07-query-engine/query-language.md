# `docs/07-query-engine/query-language.md`

# Purpose

The Matis Query Language (MQL) provides a unified interface for querying engineering history, knowledge, relationships, and context.

Unlike SQL, MQL is not designed to retrieve rows.
Unlike GraphQL, MQL is not designed to retrieve object graphs.
Unlike vector search, MQL is not designed to retrieve similar text.

MQL retrieves **engineering understanding**.

---

# Philosophy

Developers do not think in databases. Developers think in questions:
* *Why did this change?*
* *Who introduced this bug?*
* *Which prompt generated this file?*
* *Which AI solved this problem previously?*
* *What decisions affect authentication?*
* *What happened before this regression?*

MQL answers engineering questions, not storage questions.

---

# Two-Layer Query Architecture

MQL separates intent compilation from execution using an intermediate AST (Abstract Syntax Tree):

```text
               User / AI Request (Natural Language or Structured)
                                       │
                                       ▼
                             ┌───────────────────┐
                             │  Intent Compiler  │
                             └─────────┬─────────┘
                                       │
                                       ▼
                             ┌───────────────────┐
                             │      MQL AST      │ (Intermediate Representation)
                             └─────────┬─────────┘
                                       │
                                       ▼
                             ┌───────────────────┐
                             │   Query Planner   │
                             └─────────┬─────────┘
                                       │
                                       ▼
  ┌─────────────────┬──────────────────┼──────────────────┬─────────────────┐
  ▼                 ▼                  ▼                  ▼                 ▼
Working           Semantic          Knowledge          Timeline           Event
Memory             Memory             Graph             Stream            Store
  │                 │                  │                  │                 │
  └─────────────────┴──────────────────┼──────────────────┴─────────────────┘
                                       ▼
                            Context Intelligence Engine
                                       │
                                       ▼
                                 Query Result
```

* **Intent Compiler**: Translates free-form human language or client commands into a structured query tree (`MQL AST`).
* **MQL AST**: Stable, deterministic intermediate representation.
* **Planner**: Optimizes execution order (searching fast/near memory before deep storage).
* **Execution Engine**: Interacts with Knowledge Graph, Memory Engine, Timeline, and Event Store.

---

# Query Categories & MQL Syntax Examples

## 1. Timeline Queries
```text
timeline today
timeline yesterday
timeline last-week
timeline between release-v1 release-v2
```

## 2. Project Queries
```text
project current
project matis
project quantrix
```

## 3. Memory & Decision Queries
```text
memory authentication
memory architecture
decision oauth
decision sqlite
```

## 4. Prompt, Commit & File Queries
```text
prompt "oauth"
prompt generated middleware.rs
commit latest
commit affecting auth/
file middleware.rs
file Cargo.toml
```

## 5. Graph Relationship Queries
```text
why middleware.rs
how auth.rs
influenced oauth
parents commit abc123
children decision oauth
related middleware.rs
```

## 6. Context Queries
```text
continue authentication
```

---

# MQL AST Structure

```rust
pub enum MqlQuery {
    Timeline(TimelineQuery),
    Memory(MemoryQuery),
    Decision(DecisionQuery),
    Graph(GraphQuery),
    Context(ContextQuery),
    Search(SearchQuery),
    Composite(Vec<MqlQuery>),
}
```

Every query, regardless of origin, compiles down to an `MqlQuery` AST.

---

# Search Strategy & Query Optimization Hierarchy

The MQL Planner evaluates sources in order of signal density and proximity:

1. **Working Memory** (Current branch, active prompt, file session)
2. **Semantic Memory** (ADRs, constraints, distilled patterns)
3. **Knowledge Graph** (Causal links & relationship edges)
4. **Timeline** (Chronological event stream)
5. **Event Store** (Canonical raw immutable events)
6. **Archives** (Read-only historical event logs)

Searching near-memory first guarantees sub-50ms responses for most queries without full database scans.

---

# Output Formats

```text
Human Text      (Formatted CLI/TUI output)
Markdown        (Rich engineering documentation)
JSON            (Machine-readable API output)
Tree / Graph    (Node and edge relationship representation)
Timeline Stream (Chronological event list)
Context Bundle  (Packed prompt for LLMs)
```

---

# Query Invariants

1. Queries never modify Engineering Events.
2. Results are deterministic for identical data and inputs.
3. Every answer is traceable to source events.
4. Graph traversals preserve relationship direction.
5. Context generation respects token budgets.
6. Natural language queries compile into deterministic MQL ASTs.
7. Archived data remains queryable unless explicitly disabled.
8. Sensitive information is redacted before presentation.
9. Query execution is independent of storage implementation.
10. Every result can explain why it was returned (provenance citations).

---

# Example MQL Session Output

```text
> continue authentication

Context
─────────────────────────────────────────────────────────────
Current Branch
  feature/auth

Architecture Decision
  Tower middleware (ADR-0004)

Recent Decision
  JWT authentication with short expiration

Known Bug
  Token expiration parsing in claims.rs

Recent Commits
  8 commits on branch feature/auth

Related Files
  middleware.rs
  claims.rs
  jwt.rs

Citations & Provenance
  - ADR-0004 referenced by 7 recent commits (Confidence: 0.98)
  - Bug logged in Event ev_20260806_1029 (Confidence: 0.95)

Suggested Next Step
  Implement refresh-token validation in jwt.rs
```
