# `docs/02-architecture/storage.md`

# Purpose

The Storage Layer is responsible for persisting every piece of engineering knowledge managed by Matis.

It provides durable, immutable, queryable, and recoverable storage for engineering events and their derived knowledge.

Storage is an implementation detail.

The rest of the system should never know whether data is stored in SQLite, PostgreSQL, RocksDB, or another backend.

---

# Philosophy

Storage should answer one question:

> **Where does information live?**

It should never answer:

* What does this mean?
* Should this become memory?
* How is this related?

Those belong to higher layers.

---

# Storage Architecture

```text
                   Engineering Events
                           │
                           ▼
                   Storage Abstraction
                           │
       ┌───────────┬────────────┬────────────┐
       ▼           ▼            ▼            ▼
  Event Store  Blob Store   Search Index  Graph Store
       │           │            │            │
       └───────────┴────────────┴────────────┘
                           │
                     Backup & Archive
```

Every storage engine has one responsibility.

---

# Storage Principles

The Storage Layer follows these principles:

## Immutable
Raw events are never modified.

---

## Append Only
Historical records are never overwritten.

---

## Recoverable
Every index can be rebuilt from raw events.

---

## Traceable
Every derived object references its source events.

---

## Replaceable
No subsystem depends on a specific database.

---

# Storage Components

## Event Store
The canonical source of truth.

Stores:
* Engineering Events
* Event metadata
* Event relationships
* Event timestamps

The Event Store never contains derived knowledge.

---

## Blob Store
Stores large objects.

Examples:
```text
AI transcripts

Git patches

Large logs

Crash dumps

Attachments

Screenshots

Artifacts
```

Events reference blobs. Events never embed large binary data.

---

## Search Index
Optimized for fast retrieval.

Indexes:
```text
Projects

Actors

Repositories

Files

Commits

Prompts

Responses

Knowledge

Tags
```

The Search Index can always be rebuilt.

---

## Graph Store
Stores relationships.

Examples:
```text
Prompt  ───generated───►  Response  ───modified───►  File  ───committed───►  Commit
```

Graph storage contains edges, not raw events.

---

## Memory Store
Stores semantic knowledge.

Examples:
```text
Architecture Decisions

Lessons

Patterns

Constraints

Milestones

Recommendations
```

Everything here references Engineering Events.

---

# Storage Responsibilities

| Component | Stores | Never Stores |
|---|---|---|
| **Event Store** | Immutable events | Derived summaries |
| **Blob Store** | Large binary/text payloads | Relationships |
| **Graph Store** | Nodes and edges | Raw transcripts |
| **Memory Store** | Semantic knowledge | Temporary state |
| **Search Index** | Optimized lookup structures | Canonical data |

Every storage engine has one job.

---

# Data Flow

```text
Capture

↓

Engineering Event

↓

Event Store

↓

Graph Builder

↓

Graph Store

↓

Memory Engine

↓

Memory Store

↓

Search Index
```

Notice: Everything starts from the Event Store.

---

# Event Storage

Events are append-only.

```text
Event  ───►  Serialize  ───►  Checksum  ───►  Compress  ───►  Persist
```

Once persisted, the event never changes.

---

# Blob Storage

Large payloads remain outside the Event Store.

```text
Prompt  ───►  50 KB transcript  ───►  Blob  ───►  Blob ID  ───►  Event references Blob ID
```

Benefits:
* smaller event records
* faster indexing
* easier caching

---

# Indexing

Indexes are derived, never primary.

Examples:
```text
Project Index

Repository Index

Prompt Index

Commit Index

Decision Index

Knowledge Index
```

Delete the index, rebuild it — nothing is lost.

---

# Relationship Storage

Relationships are stored separately.

```text
Prompt  ───generated───►  Response
```

becomes:

```text
Edge { from: Prompt, to: Response, type: "generated" }
```

Graph reconstruction remains deterministic.

---

# Storage Isolation

Every storage engine has a repository interface.

```text
Storage Interface
├── EventRepository
├── BlobRepository
├── MemoryRepository
├── GraphRepository
└── SearchRepository
```

Business logic never sees raw database details or SQL.

---

# Snapshots

The Storage Layer supports snapshots:
* Daily
* Weekly
* Before Release
* Manual
* Before Distillation

Snapshots enable recovery and historical analysis.

---

# Archiving

Old information moves to archives:

```text
Recent Events  ───►  Archive  ───►  Compressed  ───►  Read-only
```

Archives remain searchable.

---

# Backup Strategy

Supports:
* Incremental
* Full
* Encrypted
* Offline
* Cloud
* External Drive

Backups include Events, Graph, Memory, and Configuration. Indexes may be regenerated.

---

# Data Integrity

Every stored object has:
* Checksum
* Version
* Timestamp
* Origin
* Schema Version

Corruption is detectable.

---

# Storage Versioning

Storage schema evolves. Events do not.

Migration affects:
* Indexes
* Tables
* Caches
* Graph
* Memory Objects

Historical events remain unchanged.

---

# Performance Goals

Target metrics:

```text
Insert             <5 ms
Query              <50 ms
Context Retrieval  <100 ms
Startup            <2 s
Recovery           Linear with event count
```

Optimize only after measuring.

---

# Security

Storage is local-first.

Requirements:
* Encryption at rest (optional)
* File integrity verification
* Permission-aware access
* Secret redaction
* Secure deletion for ephemeral data
* Backup encryption support

---

# Core Invariants

1. Engineering Events are immutable.
2. Event Store is the canonical source of truth.
3. Every derived object references source events.
4. Search indexes are rebuildable.
5. Graph relationships are deterministic.
6. Large payloads live in the Blob Store.
7. Storage engines remain interchangeable through repository interfaces.
8. Backups preserve provenance and integrity.
9. Storage failures never silently discard acknowledged events.
10. Every stored object has a verifiable schema version.

---

# Recommended Physical Layout

```text
~/.matis/
├── config/
│   └── config.toml
├── storage/
│   ├── events/
│   │   └── events.jsonl / events.db
│   ├── blobs/
│   │   ├── 5d/
│   │   └── 8a/
│   ├── graph/
│   │   └── graph.db
│   ├── memory/
│   │   └── memory.db
│   ├── search/
│   │   └── index/
│   ├── cache/
│   ├── archive/
│   └── snapshots/
├── logs/
├── plugins/
└── workspace/
```

The directory layout separates **canonical data** (events), **derived data** (graph, memory, search), and **ephemeral data** (cache).

---

# Architecture Decision Candidates

* Should the Event Store initially use SQLite with an append-only event table, or a purpose-built log-structured store?
* Should the Graph Store be implemented as relational edge tables first, with an abstraction that allows migration to a graph database later?
* What size threshold moves payloads from inline storage to the Blob Store?
* Should archives remain queryable in-place or require explicit mounting?
* How should schema migrations be versioned to guarantee replay compatibility?
