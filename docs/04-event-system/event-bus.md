# Event Bus

## Purpose

The Event Bus is the ingestion backbone of Matis.

Every subsystem that observes engineering activity publishes standardized Engineering Events into the Event Bus.

The Event Bus is responsible for transporting events from producers to consumers.

It does **not** interpret events.
It does **not** store events.
It does **not** build memories.

Its only responsibility is reliable event delivery.

---

## Philosophy

The Event Bus is intentionally dumb.

Every producer speaks one language.
Every consumer listens to one language.

```text
Capture -> Engineering Event -> Event Bus -> Consumers
```

Nothing communicates directly with storage.
Nothing communicates directly with memory.
Everything goes through the bus.

---

## Why an Event Bus?

**Without an Event Bus**: Everything knows about everything (Claude -> SQLite, Git -> SQLite, etc.). Dependencies explode.

**With an Event Bus**: Every subsystem becomes independent.

```text
Capture (Claude, Git, Shell, etc.)
      ↓
Engineering Event
      ↓
  Event Bus
      ↓
Consumers (Storage, Timeline, Memory, Graph, Search, API)
```

---

## Architecture

```text
                   Producers
 (Claude, Git, Shell, IDE, Filesystem, API, etc.)
                         │
                         ▼
          +---------------------------+
          |      Event Bus            |
          +---------------------------+
                         │
      ┌───────┬──────────┼───────────┬────────────┐
      ▼       ▼          ▼           ▼            ▼
   Storage Timeline    Memory      Graph        Search
```

---

## Design Principles

*   **Single Event Type**: Every producer emits `EngineeringEvent`. Nothing else.
*   **Append Only**: Consumers receive immutable copies; they never modify events.
*   **Producer Isolation**: A producer never knows who consumes its events.
*   **Consumer Isolation**: Consumers never know where events originated. Every consumer processes the same structure.

---

## Event Flow

1.  **Capture**: User action (e.g., git commit).
2.  **Normalize**: Adapter creates `EngineeringEvent`.
3.  **Publish**: Event is sent to the Event Bus.
4.  **Broadcast**: Event Bus delivers to all active consumers.
5.  **Process**: Storage persists, Memory evaluates, Graph links, etc.

---

## Delivery & Failure Handling

*   **States**: Received, Validated, Accepted, Delivered, Acknowledged, Failed.
*   **Isolation**: Subsystem failures remain isolated. If Memory fails, Storage can still succeed.
*   **Retries**: Individual consumers or producers can retry without affecting the whole system.
*   **Ordering**: Based on Timestamp -> Sequence Number -> Event ID.

---

## Event Replay

Because events are immutable, the Event Bus can replay history. This allows for:
*   Rebuilding memory engines after algorithm improvements.
*   Migrating data without loss.
*   Testing new consumers on historical data.

---

## Technical Features

*   **Filtering**: Consumers subscribe only to relevant event categories.
*   **Backpressure**: Buffering events if consumers are slow; prevents producer blocking.
*   **Priority**: Immediate processing for Critical events (e.g., Decisions, Releases).
*   **Versioning**: Schema versions ensure backward compatibility.
*   **Security**: Bus validates every event (schema, limits, checksum, source) before delivery.

---

## Core Invariants

1. Events are immutable after publication.
2. Producers never communicate directly with consumers.
3. Consumers process events independently.
4. Delivery failures are isolated.
5. Replay produces the same sequence of events.
6. Ordering is deterministic.
7. Schema versions remain backward compatible.
8. Validation occurs before delivery.
9. The Event Bus owns transport, not business logic.
10. Consumers may be added or removed without modifying producers.

---

## Future Extensions

*   Distributed event streaming across devices.
*   Team-shared event federation.
*   Remote synchronization with conflict resolution.
*   Real-time dashboards over WebSocket streams.
*   Time-travel debugging by replaying historical activity.

### Architecture Decision Candidates

*   **Transport**: In-process interface vs. distributed transport support.
*   **Guarantee**: At-least-once, at-most-once, or exactly-once delivery.
*   **Concurrency**: Synchronous vs. asynchronous consumers.
*   **Replay Strategy**: Reusing the bus pipeline vs. bypassing transport.
