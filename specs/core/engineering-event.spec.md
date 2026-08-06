# Engineering Event Formal Behavioral Specification (`specs/core/engineering-event.spec.md`)

## 1. Specification Status & Invariants

* **Specification Version**: 1.0.0
* **Status**: Normative / Core Specification
* **Target Crate**: `crates/matis-events` & `crates/matis-objects`

### Core Invariants
1. `EngineeringEvent` instances are 100% append-only and immutable once validated and stored.
2. Every `EngineeringEvent` must carry a stable, unique `EventId` (UUIDv7 or ULID).
3. Every `EngineeringEvent` must contain cryptographic checksum verification (`SHA-256`).
4. Schema evolution is strictly additive; existing event payload fields MUST NOT be deleted or mutated in-place.

---

## 2. Field Schema & Data Contract

```rust
pub struct EngineeringEventSpec {
    pub id: EventId,
    pub project_id: ProjectId,
    pub session_id: Option<SessionId>,
    pub actor: Actor,
    pub source: EventSource,
    pub kind: EventKind,
    pub importance: Importance,
    pub timestamp: String, // ISO-8601 UTC string under KernelClock
    pub payload: EventPayload,
    pub metadata: EventMetadata,
    pub parents: Vec<EventId>,
    pub checksum: String,
}
```

### Required Fields & Validation Rules
| Field | Type | Validation Rule |
|---|---|---|
| `id` | `EventId` | Must be a valid 128-bit UUIDv7 or ULID string. Cannot be empty. |
| `project_id` | `ProjectId` | Must reference an active registered project identity. |
| `actor` | `Actor` | Must specify `actor_type` (`User`, `AI`, `System`, `Tool`) and name. |
| `source` | `EventSource` | Must identify provenance sensor (`Git`, `Shell`, `Claude`, `Filesystem`). |
| `kind` | `EventKind` | Must match valid enum (`GitCommit`, `Prompt`, `BuildResult`, etc.). |
| `importance` | `Importance` | Priority score: `Low`, `Medium`, `High`, `Critical`. |
| `timestamp` | `String` | ISO-8601 UTC timestamp provided by `KernelClock`. |
| `parents` | `Vec<EventId>` | Parent DAG linkages (0 for root events). Must not contain self-references. |
| `checksum` | `String` | Hexadecimal SHA-256 digest of canonical serialized payload. |

---

## 3. State Machine & Event Validation Lifecycle

```text
  Observation Captured by Sensor
                │
                ▼
  In-Memory Draft Event
                │
                ▼
  Validate Schema & Parent References
                │
         ┌──────┴──────┐
         ▼             ▼
      Pass           Fail  ──► Drop Event & Log Error Diagnostic
         │
         ▼
  Compute SHA-256 Checksum
                │
                ▼
  Append to Event Store (events.jsonl / events.db)
                │
                ▼
  Publish to Kernel Event Bus (Critical Priority Tier)
```

---

## 4. Replay & Determinism Contract

Replaying an `EventStore` MUST satisfy:

$$\text{Replay}(\text{EventStore}) \equiv \text{Original}(\text{EventStore})$$

When an event stream is replayed:
1. Event timestamps and parents MUST be evaluated in strict DAG topological order.
2. Identical event payloads MUST generate identical `EpisodeId` assignments in `crates/matis-episodes`.
