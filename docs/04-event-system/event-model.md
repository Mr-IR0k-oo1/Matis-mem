# Event Model

## Purpose

This document defines the canonical data model of Matis.

Every piece of information inside Matis is represented as an immutable Engineering Event.

This document is the source of truth for:

* Event structure
* Event lifecycle
* Event relationships
* Event identity
* Event immutability
* Event categories
* Parent-child relationships
* Storage guarantees

If this document changes, almost every subsystem of Matis changes.

---

## Philosophy

Traditional developer tools store state.

```text
Current branch
Current files
Current chat
Current project
```

State disappears.

Matis stores history.
History creates knowledge.
Knowledge creates intelligence.

Nothing is ever directly modified.
Everything is appended.
Exactly like Git.

---

## Fundamental Principle

> **Everything that happens becomes an immutable Engineering Event.**

Not just AI.
Not just Git.
Not just shell commands.
Everything.

**Examples**
```text
User asked Claude
      ↓
Claude generated code
      ↓
User rejected code
      ↓
Cargo build failed
      ↓
File modified
      ↓
Tests executed
      ↓
Commit created
      ↓
Deployment completed
```

Everything is one language.
Everything is an Event.

---

## Engineering Event

```rust
EngineeringEvent
{
    id: EventId,              // UUIDv7
    timestamp: DateTime<Utc>,
    actor: ActorId,
    source: SourceId,
    project: ProjectId,
    repository: RepositoryId,
    kind: EventKind,
    payload: Payload,
    parents: Vec<EventId>,    // Parent event references (DAG)
    metadata: Metadata,
    importance: Importance,
    confidence: f32,
    checksum: Blake3Hash
}
```

---

## Why One Event Type?

**Bad design**
Every subsystem invents its own storage (ClaudeSession, GitCommit, Prompt, BuildFailure, etc.). Eventually nothing connects.

**Instead**
Every subsystem speaks the same language: `EngineeringEvent` -> `Payload`.

---

## Event Lifecycle

```text
Captured -> Validated -> Normalized -> Assigned IDs -> Linked -> Stored -> Indexed -> Available for Queries -> Eventually Promoted
```

Events never skip stages.

---

## Event Identity

Every event receives a globally unique identifier.
**Recommended**: `UUIDv7`
**Reasons**: sortable, distributed, collision resistant, timestamp aware.

Event IDs never change.

---

## Immutability

Events cannot be modified. Never.
If something changes, create another event.

**Correct**
```text
Decision Made -> Decision Revised -> Decision Deprecated
```
History remains intact forever.

---

## Event Categories

High-level categories determine payload schemas:
`Prompt`, `Response`, `Conversation`, `Decision`, `Git`, `Filesystem`, `Shell`, `Build`, `Testing`, `Deployment`, `Issue`, `Knowledge`, `Memory`, `Research`, `Architecture`, `Security`, `Performance`, `System`.

---

## Payload Model

The Event structure never changes. Only payloads evolve.
* `PromptPayload`
* `ResponsePayload`
* `GitPayload`
* `BuildPayload`
* `DecisionPayload`
* `IssuePayload`
* `ShellPayload`
* `MemoryPayload`
* `KnowledgePayload`
* `DeploymentPayload`

---

## Parent Relationships

Git has parent commits. Events have parent events.

**Example**
```text
Prompt -> Claude Response -> User Edit -> Cargo Build -> Tests -> Commit
```

**Relationships**
`Commit` parents: `[Build Success]`
`Build Success` parents: `[Prompt, Response, User Edit]`

Everything becomes a Directed Acyclic Graph (DAG).

---

## Event Sources & Actors

**Source**: Where the event originated (e.g., Claude CLI, Gemini CLI, Git, Shell, Daemon).
**Actor**: Who performed the action (e.g., User, Claude, Gemini, Git, CI).

Actor is different from Source. A `Claude` Actor might originate from a `Claude CLI` Source.

---

## Metadata

Contains contextual information that never changes event meaning, only context:
`cwd`, `repository`, `branch`, `hostname`, `kernel`, `os`, `terminal`, `duration`, `latency`, `token usage`, `language`, `tool version`, `machine id`.

---

## Importance & Confidence

**Importance**: Controls memory promotion (Critical, High, Medium, Low, Temporary).
**Confidence**: Indicates certainty (1.0 for direct observation, 0.9 for detection, 0.7 for AI extraction, 0.4 for inference).

---

## Event Validation Rules

Every event must satisfy:
* Immutable
* Timestamp present
* Actor present
* Source present
* Project identified
* Payload validated
* Category defined
* Checksum verified
* Parent references valid

---

## Event Storage

Events are append-only. Never overwrite. Never update. Never delete.
**Implementation**: Append-only SQLite log or JSONL files.

---

## Core Invariants

1. Events are immutable.
2. Events are append-only.
3. Every event has a globally unique ID.
4. Every event has exactly one actor.
5. Every event has exactly one source.
6. Parent references form a directed acyclic graph.
7. Payload schemas are versioned.
8. Metadata cannot alter event meaning.
9. Historical events are never rewritten.
10. Derived memories never replace source events.

---

## Open Questions (ADR Candidates)

* Should event checksums cover only payloads or the entire serialized event?
* Is UUIDv7 sufficient, or should IDs be content-addressable like Git objects?
* Should parent references be mandatory for all events or optional for root events?
* How are cross-project relationships represented?
* Should large payloads (logs, diffs, transcripts) live inline or in a blob store?
* What retention policy applies to raw payloads versus derived semantic knowledge?
* How should sensitive data be redacted while preserving event integrity?
