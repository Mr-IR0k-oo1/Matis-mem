# `docs/00-foundation/engineering-object-specification.md`

# Purpose

The Engineering Object Specification (EOS) defines the immutable contract for every engineering object inside Matis.

It is the lowest-level semantic specification of the platform. Every subsystem, protocol, plugin, storage engine, API, and runtime component depends on this specification.

The EOS is independent of:
* Programming language
* Storage backend
* Serialization format
* Network protocol
* Operating system
* AI provider

It defines **meaning and contract**, not transient implementation.

---

# Philosophy: The Engineering Universe

Everything inside Matis belongs to a strict 4-step transform:

```text
Reality  ──►  Observation  ──►  Engineering Object  ──►  Engineering Intelligence
```

Reality produces observations.
Observations become Engineering Objects.
Engineering Objects become Engineering Intelligence.

---

# The Canonical Base Envelope: `EngineeringObject`

Every Engineering Object shares the exact same base envelope contract:

```rust
pub struct BaseEngineeringObject {
    pub id: Identity,
    pub object_type: ObjectType,
    pub schema_version: u32,
    pub created_at: String,
    pub creator: ActorId,
    pub provenance: ProvenanceMetadata,
    pub metadata: KeyValueMetadata,
    pub references: Vec<Identity>,
    pub checksum: String,
}
```

Every specialized object type (`Event`, `Episode`, `Memory`, `Knowledge`, `Artifact`, `Context`, `Project`) wraps or extends this structure.

---

# Object Taxonomy (The 7 Canonical Object Types)

1. **`Event`**: Smallest immutable observed fact (prompt, response, commit, build, file change).
2. **`Episode`**: Unit of engineering work (objective, timeline, commits, decisions, outcome).
3. **`Memory`**: Reusable engineering knowledge (ADR, constraint, pattern, lesson, milestone).
4. **`Knowledge`**: Node/edge relationship entity connecting objects in a directed graph.
5. **`Artifact`**: External binary/text content stored in Blob Store (patch, transcript, screenshot).
6. **`Context`**: Transient, on-demand prompt context bundle assembled for AIs or humans.
7. **`Project`**: Root scope entity defining repository, workspace, and team boundaries.

---

# Mandatory Contract Rules & Provenance Model

## 1. Identity & Immutability
Identity (`Identity`) is globally unique, permanent, and immutable.

## 2. Reference-Based Linking
Objects reference related objects strictly via `Identity` keys (`EventId`, `EpisodeId`, `MemoryId`, `ArtifactId`) — payloads are **never duplicated**.

## 3. Provenance & Verification
Every object includes `ProvenanceMetadata`:
* `created_by`: `ActorId`
* `created_at`: Timestamp
* `supporting_evidence`: List of `Identity` keys
* `confidence`: `f32` (`0.0` – `1.0`)
* `generation_method`: `Observed` | `Derived` | `AI_Inferred` | `User_Annotated`

## 4. Integrity & Versioning
Every object contains a cryptographic checksum (`checksum`) and `schema_version`. Schema evolution is additive; mutation is strictly prohibited. Superseded objects point to their successors (`superseded_by`).

---

# Subsystem Architecture Dependence

```text
Engineering Object Specification (EOS)  [docs/00-foundation/engineering-object-specification.md]
                  │
                  ▼
              Kernel                    [docs/02-architecture/kernel.md]
                  │
                  ▼
            Event System                [docs/01-vision/event-model.md]
                  │
                  ▼
           Episode Engine               [docs/02-architecture/episode-engine.md]
                  │
                  ▼
            Memory Engine               [docs/03-memory/architecture.md]
                  │
                  ▼
          Knowledge Graph               [docs/02-architecture/knowledge-graph.md]
                  │
                  ▼
          Reasoning Engine              [docs/02-architecture/reasoning-engine.md]
                  │
                  ▼
      Engineering Intelligence          [docs/02-architecture/engineering-intelligence.md]
                  │
                  ▼
            Protocol (ECP)              [docs/02-architecture/matis-protocol.md]
                  │
                  ▼
         Clients & Consumers            [CLI, TUI, VS Code, MCP, API]
```

---

# Architectural Roadmap: From Architecture to Formal Specifications

Moving forward, platform development progresses from high-level architecture documents to precise formal specifications detailing exact JSON/Binary schemas, field types, validation rules, state machines, and error conditions:

1. `docs/00-foundation/specs/engineering-event.spec.md`
2. `docs/00-foundation/specs/engineering-episode.spec.md`
3. `docs/00-foundation/specs/engineering-memory.spec.md`
4. `docs/00-foundation/specs/engineering-context.spec.md`
5. `docs/00-foundation/specs/engineering-protocol.spec.md`

---

# Architecture Decision Candidates

* **Identifier Encoding**: Use UUIDv7 or ULID for time-ordered, sortable, collision-free object IDs.
* **Integrity Enforcement**: Mandatory SHA-256 checksums on all serialized `BaseEngineeringObject` envelopes.
* **Schema Evolution Protocol**: Semantic versioning on `schema_version` fields with strict forward/backward deserialization compatibility.
