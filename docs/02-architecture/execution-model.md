# `docs/02-architecture/execution-model.md`

# Purpose

The Execution Model defines how engineering activity flows through Matis from observation to intelligence.

It specifies the complete lifecycle of engineering work, interaction semantics between subsystems, priority scheduling, consistency guarantees, parallel execution boundaries, and failure recovery semantics.

Unlike subsystem documents, the Execution Model describes **system behavior as a unified runtime whole**.

---

# Philosophy: Event-Driven Transformations

Matis is an event-driven engineering runtime.

Subsystems do not call each other directly; execution is a pipeline of transformations:

```text
Reality  ──►  Observation  ──►  Engineering Event  ──►  Episode  ──►  Knowledge  ──►  Reasoning  ──►  Context
```

Reality is observed. Knowledge is derived. Intelligence is produced.

---

# The 8 Runtime Execution Stages

```text
  Stage 1: Capture       (Sensors observe external engineering activity)
     │
     ▼
  Stage 2: Validation    (Kernel checks schema, timestamp, identity, provenance, integrity)
     │
     ▼
  Stage 3: Persistence   (Appends validated Engineering Events to the Event Store)
     │
     ▼
  Stage 4: Organization  (Episode Engine groups events into Engineering Episodes)
     │
     ▼
  Stage 5: Knowledge     (Memory Engine & Knowledge Graph extract patterns, ADRs & relationship edges)
     │
     ▼
  Stage 6: Intelligence  (EIL performs background drift detection, health checks & trend analysis)
     │
     ▼
  Stage 7: Reasoning     (Reasoning Engine plans retrieval & builds evidence context bundles)
     │
     ▼
  Stage 8: Presentation  (API / Transport renders markdown, JSON, or ECP stream for AI/IDE/CLI)
```

No stage is ever bypassed. Derivation never precedes persistence.

---

# Execution Consistency Matrix

| Stage | Consistency Model | Latency Target | Execution Priority |
|---|---|---|---|
| **Event Store** | Strong (Synchronous append) | `<5 ms` | `Critical` |
| **Episode Assignment** | Eventual (Incremental) | `<20 ms` | `Background` |
| **Knowledge Graph** | Eventual (Incremental) | `<50 ms` | `Background` |
| **Memory Engine** | Eventual (Background) | `<100 ms` | `Idle` |
| **Intelligence Layer** | Eventual (Background) | `<500 ms` | `Idle` |
| **Reasoning / CIE** | Real-time (On-demand) | `<100 ms` | `Interactive` |

---

# Deterministic Replay & Recovery Engine

Failure recovery and testing rely on **Deterministic Event Replay**:

```text
System Crash / Recovery Initiated
               │
               ▼
   Read Last Valid Checkpoint
               │
               ▼
Replay Event Store (KernelClock)
               │
               ▼
  Rebuild Episode Store & Lineage
               │
               ▼
  Rebuild Knowledge Graph & Memory
               │
               ▼
  Resume Normal Runtime Execution
```

Given an identical Event Store, configuration, and algorithm, the runtime produces 100% identical Episodes, Memory, Knowledge, and Context.

---

# Concurrency, Backpressure & Incremental Deltas

* **Incremental Deltas**: The runtime processes event deltas rather than re-reading history.
* **Parallel Execution**: Independent stages (Graph updates, Search indexing, Analytics) execute concurrently after persistence acknowledgment.
* **Backpressure**: When background queues fill, sensors buffer in backpressure queues — capture never drops acknowledged events.

---

# Complete Runtime Diagram

```text
Reality
   │
   ▼
Sensors (Capture)
   │
   ▼
Event Bus
   │
   ▼
Event Store (Persistence)
   │
   ▼
Episode Engine (Organization)
   │
   ├────────────┐
   ▼            ▼
Memory      Knowledge Graph (Knowledge)
   │            │
   └──────┬─────┘
          ▼
 Distillation Engine
          ▼
 Engineering Intelligence (Intelligence)
          ▼
  Reasoning Engine (Reasoning)
          ▼
 Context Intelligence (CIE)
          ▼
 API / CLI / IDE / AI (Presentation)
```

---

# Core Invariants

1. Capture always precedes persistence.
2. Persistence always precedes derivation (derived data is never stored without source events).
3. Replay is 100% deterministic.
4. Event ordering is strictly preserved.
5. Independent execution stages execute concurrently when possible.
6. Subsystem failures never invalidate persisted event history.
7. Incremental execution produces the exact same result as full replay.
8. Background processing never blocks interactive context retrieval workflows.
9. Every execution stage is independently observable.
10. The runtime operates deterministically under `KernelClock`.

---

# Specifications Roadmap (`specs/`)

Platform architecture documentation is now complete. The next formal phase shifts from architectural design to precise executable specification files under `specs/`:

* `specs/engineering-event.spec.md`
* `specs/engineering-episode.spec.md`
* `specs/engineering-memory.spec.md`
* `specs/engineering-object.spec.md`
* `specs/engineering-context.spec.md`
* `specs/engineering-context-protocol.spec.md`
* `specs/event-bus.spec.md`
* `specs/replay.spec.md`
* `specs/execution.spec.md`
* `specs/sensor-sdk.spec.md`
* `specs/plugin-api.spec.md`
