# `docs/03-memory/distillation-engine.md`

# Purpose

The Distillation Engine continuously transforms engineering history into reusable engineering knowledge.

Unlike summarization systems, the Distillation Engine does not compress raw conversations.

It discovers:
* patterns,
* decisions,
* lessons,
* constraints,
* architectural evolution,
* engineering habits.

Its goal is to reduce millions of Engineering Events into a compact set of high-value knowledge objects while preserving 100% complete traceability.

---

# Philosophy

History is expensive. Knowledge is cheap.

The Event Store should grow forever.
Semantic Memory should grow slowly.

Distillation is the process that converts raw history into high-signal knowledge.

---

# High-Level Architecture: Knowledge Refinement Loop

Rather than a static one-way promotion pipeline, the Distillation Engine operates as a **Continuous Knowledge Refinement Loop**:

```text
                       Engineering Events
                               │
                               ▼
                      Candidate Selection
                               │
                               ▼
                     Relationship Analysis
                               │
                               ▼
                       Pattern Detection
                               │
                               ▼
                      Decision Extraction
                               │
                               ▼
                      Confidence Scoring
                               │
                               ▼
                       Human Review Queue
                               │
                               ▼
                        Semantic Memory
                               │
                               ▼
                          Used by AI
                               │
                               ▼
                     User Feedback & Outcome
                               │
                               ▼
                      Confidence Updated
                               │
                               ▼
                       Knowledge Refined
```

Knowledge is dynamic. If a pattern repeatedly leads to successful builds and merges, its confidence rises. If a recommendation leads to test failures or is superseded, its confidence drops or it is marked as superseded.

---

# Responsibilities

The Distillation Engine is responsible for:
* detecting recurring prompt/code/build patterns,
* identifying Architecture Decision Records (ADRs) and choices,
* extracting engineering lessons from failures and successes,
* discovering project constraints (e.g. offline-first, SQLite-only, Windows compatibility),
* recognizing successful engineering workflows,
* identifying obsolete knowledge,
* proposing memory promotion.

It is **not** responsible for:
* retrieving context for prompt assembly,
* ranking search results,
* modifying raw Engineering Events,
* answering user queries directly.

---

# Distillation Subsystem Architecture

```text
Distillation Engine
├── Candidate Selector       (Selects candidate events: ADRs, PR merges, build failures, refactors)
├── Episode Builder          (Groups events into engineering episodes by project/branch/goal)
├── Pattern Detector         (Detects repeated prompt/code/fix sequences)
├── Decision Extractor       (Identifies architecture choices and evidence)
├── Constraint Extractor     (Discovers persistent project constraints)
├── Lesson Extractor         (Transforms failure-to-success iterations into lessons)
├── Confidence Scorer        (Computes initial confidence based on supporting event count)
├── Human Review Queue       (Presents proposed knowledge objects for human approval)
├── Knowledge Builder        (Creates structured Decision/Pattern/Constraint/Lesson objects)
├── Promotion Manager        (Manages memory promotion from Episodic → Semantic Memory)
└── Knowledge Refinement Loop(Updates confidence based on usage outcomes and user feedback)
```

---

# Structured Knowledge Objects

Distillation produces explicit, structured domain objects instead of free-form text summaries:

1. **Decision**: Architectural choices (`title`, `rationale`, `alternatives`, `evidence`).
2. **Pattern**: Recurring problem-solving sequences (`name`, `trigger`, `sequence`, `outcome`).
3. **Constraint**: Persistent engineering invariants (`description`, `scope`, `origin`).
4. **Lesson**: Extracted insights from build/test/implementation failures (`issue`, `failed_attempts`, `solution`).
5. **Milestone**: Major project markers (`version`, `commits`, `key_events`).
6. **Habit / Workflow**: Observed developer preferences and routines.

---

# Pattern Confidence & Human Review

Every distilled object receives a confidence score (`0.0` – `1.0`):
* **Signals**: Supporting event count, independent confirmations, human verification, successful build/merge outcomes, time consistency.
* **Human Validation**: Users remain the final authority. Distilled candidates enter a Review Queue where human approval converts proposed candidates into permanent Semantic Memory.

---

# Obsolescence & Supersedence

Knowledge is never overwritten or deleted. When a decision or constraint changes:

```text
Decision V2  ───supersedes───►  Decision V1
```

Historical lineage and provenance remain 100% intact.

---

# Scheduling & Performance

* **Idle Execution**: Background distillation runs during CPU idle periods, after major PR merges, before releases, or on demand.
* **Non-Blocking**: Distillation never blocks interactive context retrieval or event ingestion.
* **Incremental Processing**: Prefers sub-100ms incremental processing over full database replays.

---

# Core Invariants

1. Engineering Events remain immutable.
2. Distilled knowledge always references supporting event evidence.
3. Human approval overrides automatic promotion.
4. Confidence scores accompany every extracted object.
5. Knowledge evolution preserves historical lineage (supersedence over deletion).
6. Distillation is deterministic for identical inputs and algorithms.
7. Incremental distillation produces the same result as full replay.
8. No distilled object exists without provenance.
9. Obsolete knowledge is superseded, not rewritten.
10. Distillation failures never corrupt persistent data or event capture.
