# `docs/02-architecture/object-model.md`

# Purpose

The Object Model defines the canonical entities of the Engineering Memory Operating System.

Every subsystem stores, references, transmits, and reasons about these core objects.

No subsystem may invent ad-hoc core objects. Everything derives from this unified primitive hierarchy.

---

# Philosophy: Radically Simple Primitives

Git achieved longevity by relying on just four immutable object types (`Blob`, `Tree`, `Commit`, `Tag`).

Matis achieves simplicity by unifying all engineering data into a single top-level primitive: **Engineering Object**.

```text
                                Engineering Object
                                         │
        ┌───────────┬────────────┬───────┴───┬───────────┬───────────┐
        ▼           ▼            ▼           ▼           ▼           ▼
      Event      Episode      Memory     Knowledge    Context     Artifact
```

---

# Core Object Hierarchy & Upward Evolution

Information evolves strictly upward — lower layers never depend on higher layers:

```text
Reality
  │
  ▼
Engineering Event   (The smallest immutable observed fact)
  │
  ▼
Engineering Episode (Groups events into an engineering story/activity)
  │
  ▼
Engineering Memory  (Stores reusable engineering understanding & ADRs)
  │
  ▼
Knowledge           (Connects memories and episodes in a relationship graph)
  │
  ▼
Context             (Transient, on-demand bundle assembled for AIs/Humans)
```

Nothing skips layers. Evolution is strictly directional.

---

# The Canonical Engineering Objects

Only these 6 core variants and `Identity` exist in the kernel:

1. **`EngineeringEvent`**: Smallest immutable fact (`Prompt`, `Response`, `Commit`, `Build`, `FileModified`).
2. **`EngineeringEpisode`**: Canonical story of work (`title`, `events`, `commits`, `decisions`, `outcome`).
3. **`EngineeringMemory`**: Reusable understanding (`Constraint`, `Lesson`, `Pattern`, `ADR`, `Milestone`).
4. **`KnowledgeNode`**: Graph entity connecting episodes, memories, decisions, and files with directed edges.
5. **`ContextBundle`**: Transient context packed for AI/developer consumption (discarded after use).
6. **`Artifact`**: External binary/text output stored in Blob Store (`SourceFile`, `Binary`, `Patch`, `Screenshot`).
7. **`Identity`**: Permanent, immutable identifiers (`EventId`, `EpisodeId`, `MemoryId`, `ProjectId`, `ArtifactId`).

---

# Subsystem Object Ownership

Each subsystem has exclusive ownership of exactly one object type:

| Subsystem | Owns | Immutable? | Storage Location |
|---|---|---|---|
| **Capture Layer** | `EngineeringEvent` | Yes | Event Store (`events.jsonl`) |
| **Episode Engine** | `EngineeringEpisode` | Yes (Metadata evolves) | Episode Store |
| **Memory Engine** | `EngineeringMemory` | Yes (Superseded over edit) | Memory Store (`memory.db`) |
| **Knowledge Graph** | `KnowledgeNode` | Yes | Graph Store (`graph.db`) |
| **Reasoning Engine / CIE** | `ContextBundle` | No (Transient) | Generated on-demand |
| **Blob Store** | `Artifact` | Yes | Blob Store (`blobs/`) |

---

# Reference-Based Model (Zero Data Duplication)

Higher-level objects reference lower-level objects by `Identity`; data is **never copied**:

```text
EngineeringEpisode {
    id: EpisodeId,
    title: "Implement JWT Auth",
    events: [EventId("ev_101"), EventId("ev_102")],
    memories: [MemoryId("mem_201")],
    ...
}
```

---

# Lifecycle & Provenance

* **Event**: Observed ──► Validated ──► Stored ──► Referenced Forever
* **Episode**: Detected ──► Growing ──► Completed ──► Archived
* **Memory**: Extracted ──► Reviewed ──► Promoted ──► Referenced ──► Superseded
* **Knowledge**: Created ──► Linked ──► Expanded ──► Verified
* **Context**: Requested ──► Built ──► Consumed ──► Discarded

Every object carries complete **provenance metadata**: `Origin`, `SupportingObjects`, `CreationTime`, `Version`, `Checksum`, and `Confidence`.

---

# Recommended Crate Layout (`crates/`)

The object model lives directly inside the kernel crate so every subsystem depends on the exact same canonical types:

```text
crates/
├── matis-kernel/
│   └── object/
│       ├── event.rs
│       ├── episode.rs
│       ├── memory.rs
│       ├── knowledge.rs
│       ├── context.rs
│       ├── artifact.rs
│       └── identity.rs
├── matis-events/
├── matis-episodes/
├── matis-memory/
├── matis-graph/
├── matis-reasoning/
├── matis-api/
└── matis-storage/
```

---

# Core Invariants

1. Every object has an immutable, globally unique `Identity`.
2. Every object has traceable provenance back to source events.
3. Events remain the sole canonical source of truth.
4. Episodes organize events via references, never duplication.
5. Memory is derived from episodes or events.
6. Knowledge links objects without mutating them.
7. Context is transient, reproducible, and never permanently stored.
8. Artifacts are referenced via Blob IDs, never embedded inline.
9. Objects evolve only through new derived objects (supersedence), never in-place mutation.
10. Every object supports binary, JSON, and MessagePack serialization with verifiable schema versions.
