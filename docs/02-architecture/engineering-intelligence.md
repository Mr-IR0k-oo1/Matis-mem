# `docs/02-architecture/engineering-intelligence.md`

# Purpose

The Engineering Intelligence Layer (EIL) continuously analyzes engineering activity to discover insights, predict problems, identify opportunities, and improve future engineering decisions.

Unlike the Reasoning Engine, which reacts to immediate context requests, the Engineering Intelligence Layer operates continuously in the background.

It transforms engineering history into active engineering intelligence.

---

# Philosophy

* Git stores **history**.
* Matis stores **knowledge**.
* The Intelligence Layer creates **wisdom**.

Engineering intelligence means answering questions that nobody explicitly asked:
* *Why are builds getting slower?*
* *Which AI prompt patterns consistently succeed?*
* *Which architectural decisions repeatedly cause regressions?*
* *Which project areas have become risky or under-documented?*

---

# Complete Platform Intelligence Pipeline

Memory is not the destination — it is one stage in a continuous intelligence pipeline:

```text
                                    Reality
                                       │
                                       ▼
                               Engineering Events
                                       │
                                       ▼
                              Engineering Episodes
                                       │
                                       ▼
                               Engineering Memory
                                       │
                                       ▼
                                Knowledge Graph
                                       │
                                       ▼
                                Reasoning Engine
                                       │
                                       ▼
                         Engineering Intelligence (EIL)
                                       │
                                       ▼
                         Context Intelligence Engine (CIE)
                                       │
                                       ▼
                                Humans & AI / IDEs
```

---

# Responsibilities

The Engineering Intelligence Layer is responsible for:
* Trend analysis (build stability, compile times, technical debt growth)
* Pattern recognition across episodes and projects
* Architectural drift detection
* Recommendation generation (suggesting next actions, warning of risks)
* Knowledge quality analysis (identifying documentation gaps & low confidence)
* Engineering health assessment
* AI workflow effectiveness measurement (accepted vs rejected suggestions)

It is **not** responsible for:
* Event capture or storage
* Serving raw context or formatting markdown
* Executing AI models or editing files

---

# Intelligence Domains & Insight Types

## Intelligence Domains
1. **Engineering Health**: Technical debt, build stability, deployment reliability, architectural drift, test coverage.
2. **AI Effectiveness**: Accepted vs rejected suggestions, prompt efficiency, revision count, context utilization.
3. **Knowledge Quality**: Outdated knowledge, conflicting ADRs, documentation gaps, low confidence scores.
4. **Project Intelligence**: Architecture evolution, feature velocity, bug density, component stability.
5. **Developer Intelligence (Local & Private)**: Common failure patterns, preferred workflows, personal learning progress (never telemetry/surveillance).

## Insight Types
* **Observation**: e.g., *"Authentication changes have increased 40% over the last 3 weeks."*
* **Pattern**: e.g., *"JWT-related prompts are usually followed by token validation bugs in claims.rs."*
* **Recommendation**: e.g., *"Add authentication integration tests before merging PR."*
* **Warning**: e.g., *"Recent commits bypass existing architectural constraint: offline-first."*
* **Opportunity**: e.g., *"Parser optimization from Project A can be applied to Project B."*

---

# Subsystem Architecture

```text
Engineering Intelligence Layer (EIL)
├── Trend Analyzer             (Tracks build times, test stability, debt growth over time)
├── Pattern Analyzer           (Identifies recurring prompt/commit/bug patterns across episodes)
├── Architecture Drift Detector(Detects divergence from recorded ADRs and project constraints)
├── Knowledge Quality Analyzer (Finds documentation gaps, outdated decisions, and low-confidence items)
├── AI Effectiveness Analyzer  (Measures prompt efficiency, revision counts, and suggestion acceptance)
├── Engineering Health Engine  (Calculates multidimensional project health scores)
├── Recommendation Engine      (Generates actionable recommendations supported by evidence)
├── Insight Generator          (Produces structured Observations, Warnings, and Opportunities)
├── Confidence Manager         (Adjusts confidence scores dynamically based on outcomes)
└── Learning Feedback Engine   (Incorporates user feedback on accepted/rejected advice)
```

---

# Architectural Drift Detection

One of EIL's most critical analytical functions is detecting when current code diverges from recorded Architecture Decision Records (ADRs):

```text
Recorded Architecture Decision (ADR-0002): SQLite, Offline-First, Event-Driven
                                      vs
Current Diff Activity: Remote REST Calls, Redis Caching, Mutable State
                                      │
                                      ▼
                        Warning: Architectural Drift
```

---

# Cross-Project Compound Learning

Validated patterns and lessons discovered in one repository are packaged into cross-project semantic memory:

```text
Project A (Validated Optimization Pattern)  ──►  Cross-Project Semantic Memory  ──►  Project B (Suggested Automatically)
```

Engineering experience compounds across an entire organization or developer workspace.

---

# Performance & Core Invariants

* **Idle Execution**: Runs incrementally during background idle periods or after episode completion without interrupting interactive work.
* **Local-First & Private**: No analytics leave the machine; all telemetry remains private to the local developer.

### Core Invariants
1. Every insight is derived from immutable Engineering Events.
2. Every recommendation is explainable and traceable.
3. Confidence accompanies every inferred insight (`0.0` – `1.0`).
4. Intelligence never modifies source history or raw events.
5. User feedback influences confidence scores, not historical facts.
6. Project intelligence remains isolated unless cross-project sharing is explicitly enabled.
7. Recommendations respect security and privacy policies.
8. Engineering health scores are reproducible from the same data.
9. Cross-project learning preserves provenance chains.
10. Intelligence augments engineering decisions rather than replacing developer authority.
