# Context Intelligence Engine (CIE) Specification (`specs/reasoning/context-builder.spec.md`)

## 1. Specification Status & Budget Contract
* **Specification Version**: 1.0.0
* **Target Crate**: `crates/matis-context`

CIE constructs signal-dense Markdown context bundles from memory tiers, timeline events, and knowledge graphs while strictly enforcing caller-specified token budgets (`token_budget`).

## 2. Pipeline Stages

```text
  ContextRequest (Objective, Project, Branch, Files, TokenBudget)
                         │
                         ▼
        1. IntentPlanner (Continuation, Debugging, Optimization, etc.)
                         │
                         ▼
        2. RetrievalPlanner (Assigns memory & graph weights)
                         │
                         ▼
        3. MemorySelector & GraphTraverser (Extracts candidates)
                         │
                         ▼
        4. RankingEngine (Computes relevance & confidence scores)
                         │
                         ▼
        5. CompressionEngine (Progressive compression to token budget)
                         │
                         ▼
        6. CitationGenerator (Generates explainability citations)
                         │
                         ▼
        7. ContextComposer (Formats Markdown bundle)
```

## 3. Progressive Compression Rules
* `Full`: Complete payload body included when budget allows.
* `Summary`: Truncated summary (120 chars) when budget is constrained.
* `Reference`: ID reference link (`[Reference ID: mem_101]`) when budget is exhausted.
