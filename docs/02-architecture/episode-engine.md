# `docs/02-architecture/episode-engine.md`

# Purpose

The Episode Engine groups individual Engineering Events into meaningful units of engineering work called **Engineering Episodes**.

An Engineering Episode represents a complete engineering activity with a beginning, evolution, and outcome.

Instead of thinking in isolated prompts or commits, Matis thinks in engineering stories:
* *Implement JWT Authentication*
* *Fix Rust Lifetime Error*
* *Migrate to Axum*
* *Build Context Engine*
* *Optimize Memory Retrieval*

Episodes become the primary unit of engineering understanding across Matis.

---

# Philosophy

Humans do not remember individual actions like `14:32 cargo build`.

People remember: *"That week we migrated the authentication system."*

* Git remembers **commits**.
* Matis remembers **engineering work**.

---

# Core Architectural Shift: The Episode-Centric Platform

The entire Matis architecture revolves around **Units of Engineering Work (Episodes)** rather than isolated event logs:

```text
                               Engineering Events
                                       │
                                       ▼
                                Episode Engine
                                       │
            ┌──────────────────────────┼──────────────────────────┐
            ▼                          ▼                          ▼
        Timeline                Knowledge Graph             Memory Engine
            │                          │                          │
            └──────────────────────────┼──────────────────────────┘
                                       ▼
                                Reasoning Engine
                                       │
                                       ▼
                         Context Intelligence Engine (CIE)
                                       │
                                       ▼
                               AI / Humans / IDEs
```

Git's core abstraction is the **commit**. Matis's defining abstraction is the **Engineering Episode**.

---

# Episode Lifecycle

```text
Detected  ──►  Growing  ──►  Active  ──►  Completed  ──►  Distilled  ──►  Archived
```

Episodes never disappear; their state evolves.

---

# Episode Model Structure

```rust
pub struct EngineeringEpisode {
    pub id: EpisodeId,
    pub title: String,
    pub objective: String,
    pub project: ProjectId,
    pub repository: Option<RepositoryId>,
    pub participants: Vec<ActorId>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub status: EpisodeStatus,
    pub events: Vec<EventId>,
    pub decisions: Vec<EventId>,
    pub commits: Vec<String>,
    pub files: Vec<String>,
    pub memories: Vec<MemoryId>,
    pub outcome: EpisodeOutcome,
    pub lessons: Vec<String>,
    pub confidence: f32,
}
```

Episodes never duplicate raw event payloads — they hold stable references (`EventId`, `MemoryId`, `ProjectId`).

---

# Episode Operations & Relationships

## Episode Growth, Splitting & Merging
* **Growth**: New events matching an active objective/branch are attached automatically.
* **Splitting**: When work diverges (e.g. *OAuth* split into *JWT* vs *Session Cookies*), the engine creates child episodes while preserving lineage.
* **Merging**: When parallel optimization branches converge, a parent episode represents the merged work.

## Episode Status & Outcomes
* **Status**: `Draft`, `Active`, `Paused`, `Blocked`, `Review`, `Completed`, `Archived`.
* **Outcomes**: `Success`, `PartialSuccess`, `Failure`, `Abandoned`, `Superseded`.

## Directed Episode Graph
```text
Episode A (OAuth)  ───depends_on───►  Episode B (User DB Migration)
```

Relationships: `depends_on`, `implements`, `supersedes`, `duplicates`, `blocks`, `relates_to`, `caused_by`, `extends`, `contains`.

---

# Subsystem Architecture

```text
Episode Engine
├── Episode Detector    (Automatic detection via branch creation, prompts, issue assignments)
├── Episode Builder     (Assembles and updates episode domain models)
├── Event Grouper       (Assigns events to active episode streams)
├── Timeline Builder    (Constructs chronological episode narratives)
├── Relationship Builder(Builds graph edges between episodes)
├── Status Tracker      (Tracks Active/Blocked/Completed transitions)
├── Metrics Collector   (Calculates duration, event count, build failures, diff size)
├── Replay Engine       (Enables engineering replay time-machine functionality)
├── Episode Store       (Persists episode metadata and lineage)
└── Episode API         (Exposes episode queries to Reasoning Engine & CIE)
```

---

# Replay & AI Integration

When an AI assistant asks *"Continue authentication work"*, Matis returns the **Authentication Episode** containing:
* Objective & active branch
* Architecture decisions & rationale
* Recent commits & changed files
* Known bugs & build failures
* Lessons learned & suggested next step

Using **Engineering Replay**, developers can step through every prompt, commit, build, and decision of an episode from inception to release.

---

# Core Invariants

1. Every episode is composed of immutable Engineering Events.
2. Episodes never duplicate source event data.
3. Every episode has traceable provenance.
4. Episode reconstruction from events is deterministic.
5. Episode relationships preserve engineering history.
6. Users may override automatic episode grouping.
7. Completed episodes remain immutable except for metadata and annotations.
8. Memory distillation operates on episodes rather than individual events whenever possible.
9. Episode identifiers remain stable.
10. Episodes can always be rebuilt from the Event Store.
