# `docs/07-query-engine/context-builder.md`

# Purpose

The **Context Intelligence Engine (CIE)** is the intelligence gateway of Matis.

Its responsibility is not to retrieve raw data.

Its responsibility is to construct the **minimum complete engineering context** required for an AI or developer to perform a task.

Instead of returning raw logs, files, or unstructured conversations, the Context Intelligence Engine returns understanding.

It answers:

* What is happening?
* Why is it happening?
* What has already been tried?
* What should not be repeated?
* What constraints exist?
* What knowledge matters right now?

---

# Philosophy

Large Language Models have limited context windows.

Developers have limited attention.

Neither should waste capacity reading irrelevant history.

The Context Intelligence Engine maximizes **signal density**.

Every token returned improves the probability of a correct engineering decision.

---

# High-Level Architecture

```text
                  User / AI Request
                          │
                          ▼
                 Intent Classifier
                          │
                          ▼
                  Retrieval Planner
                          │
          ┌───────────────┼────────────────┐
          │               │                │
          ▼               ▼                ▼
   Timeline Query   Knowledge Graph   Memory Engine
          │               │                │
          └───────────────┼────────────────┘
                          ▼
               Context Assembly Engine
                          ▼
               Relevance Ranking Engine
                          ▼
               Token Budget Optimizer
                          ▼
                 Progressive Compression
                          ▼
               Citation & Explanation Generator
                          ▼
                 Final AI Context
```

The Context Intelligence Engine never queries one subsystem directly.

It orchestrates them all.

---

# Internal Subsystem Structure

```text
Context Intelligence Engine (CIE)
├── Intent Planner        (Classifies task intent: Debugging, Feature, Refactoring, Continuation, etc.)
├── Retrieval Planner     (Determines target source weights and priority across memory tiers & graph)
├── Graph Traverser       (Discovers file dependencies, decisions, and related entities)
├── Memory Selector       (Extracts candidates from Working, Episodic, and Semantic memory)
├── Ranking Engine        (Scores candidates by similarity, recency, importance, and confidence)
├── Compression Engine    (Progressive compression to strictly respect token budget constraints)
├── Context Composer      (Assembles clean, signal-dense markdown context)
└── Citation Generator    (Attaches explainability citations: "Why am I seeing this?")
```

---

# Inputs

The Context Intelligence Engine accepts a request object.

```rust
pub struct ContextRequest {
    pub objective: String,
    pub project: String,
    pub repository: Option<String>,
    pub current_branch: Option<String>,
    pub current_files: Vec<String>,
    pub working_directory: Option<String>,
    pub requester: String,
    pub token_budget: usize,
    pub preferences: HashMap<String, String>,
}
```

The request contains intent, not implementation.

---

# Intent Classification

Every request is first classified by the `IntentPlanner`.

Example:

> "Continue previous work" → **Continuation**

> "Fix authentication bug" → **Debugging**

> "Improve performance" → **Optimization**

> "Explain this architecture" → **KnowledgeRetrieval**

> "Build new feature" → **FeatureDevelopment**

> "Refactor module" → **Refactoring**

Intent determines what context is retrieved and how sources are weighted.

---

# Context Sources

The Context Intelligence Engine combines multiple knowledge sources:

1. **Working Memory**: Current activity (current branch, active tasks, recent prompts, recent builds).
2. **Episodic Memory**: Recent engineering history (previous attempts, build failures, discussions, recent commits).
3. **Semantic Memory**: Permanent knowledge (architecture decisions, constraints, best practices, patterns).
4. **Knowledge Graph**: Relationship discovery (related files, related issues, similar features, previous implementations).
5. **Timeline**: Temporal reconstruction (what happened yesterday, what happened before the bug).

---

# Relevance Ranking

Not all knowledge is equally useful. Each candidate receives a score based on:
- Similarity
- Recency
- Importance
- Confidence
- User annotations
- Success history
- Frequency of reuse

Higher-ranked knowledge is included first.

---

# Token Budget & Progressive Compression

The Context Intelligence Engine enforces strict token budgets.

When space is constrained, the `CompressionEngine` applies progressive compression:

```text
Raw Event → Summary → Knowledge Object → Decision Only → Reference
```

Meaning is preserved before detail is removed.

---

# Explainability & Citations

Every piece of context includes an explanation:

> **Why am I seeing this?**

Example:
```text
[Authentication Decision]
Reason: Referenced by 7 recent commits.
Confidence: 0.98
Source: ADR-0008
```

---

# Core Invariants

1. Context is assembled dynamically, never hardcoded.
2. Every context item is traceable to source events.
3. Token budgets are strictly respected.
4. Higher-relevance knowledge replaces lower-relevance knowledge first.
5. Context never invents facts.
6. User-pinned knowledge is always prioritized.
7. Sensitive information is filtered before output.
8. Retrieved context is deterministic for the same inputs and event history.
9. Every context item includes an explanation for why it was selected.
10. Context generation is independent of any specific AI provider.
