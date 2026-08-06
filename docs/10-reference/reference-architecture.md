# Matis Reference Architecture

# Purpose

The Matis Reference Architecture defines the canonical implementation structure for the official Rust reference workspace.

While specifications define behavioral contracts (`specs/`), the Reference Architecture defines crate boundaries, module organization, public trait exports, dependency direction, ownership rules, and build ordering for `matisd` and `matis`.

---

# Workspace Structure (`crates/`)

```text
matis/
├── Cargo.toml
├── Cargo.lock
├── crates/
│   ├── matis-kernel/            (Identity, KernelClock, Scheduler, ServiceRegistry, Diagnostics)
│   ├── matis-objects/           (Canonical primitive structs: Event, Episode, Memory, GraphNode, Artifact, Context)
│   ├── matis-events/            (Event validation, event pipelines, schemas)
│   ├── matis-storage/           (EventStore, BlobStore, Repository traits, SQLite/RocksDB/Postgres implementations)
│   ├── matis-event-bus/         (Publish/subscribe, priority event channels, queue backpressure)
│   ├── matis-episodes/          (Episode Engine, automatic grouping, narrative builder)
│   ├── matis-memory/            (Working & Semantic Memory stores, Knowledge Refinement Loop)
│   ├── matis-graph/             (Knowledge Graph, node/edge relationships, graph traversals)
│   ├── matis-reasoning/         (Reasoning Engine, evidence collector, constraint analyzer)
│   ├── matis-context/           (Context Intelligence Engine [CIE], progressive compression, citations)
│   ├── matis-intelligence/      (Engineering Intelligence Layer [EIL], drift detection, health analytics)
│   ├── matis-protocol/          (Engineering Context Protocol [ECP] framing, serializers)
│   ├── matis-api/               (Capability API, HTTP/MCP/WebSocket transport adapters)
│   ├── matis-daemon/            (matisd runtime process orchestration, service assembly)
│   ├── matis-cli/               (matis CLI front-end, interactive TUI, user commands)
│   ├── matis-plugin-sdk/        (Third-party Sensor & Consumer plugin traits, WASM/process sandboxing)
│   ├── matis-config/            (Workspace configuration, policy loaders)
│   └── matis-utils/             (Cross-cutting crypto, checksums, time formatting)
├── specs/                       (Formal behavioral specifications: *.spec.md)
├── docs/                        (Architecture, ADRs, foundation specifications)
├── tests/                       (Unit, integration, replay, property tests)
├── conformance/                 (Compliance certification test suite)
├── benchmarks/                  (Criterion benchmarks for latency & memory)
├── examples/                    (Sample sensors, plugins, and CLI integrations)
├── tools/                       (Development & replay tooling binaries)
├── plugins/                     (Installed sensors & plugin binaries)
└── fixtures/                    (Test event logs, sample episodes, golden outputs)
```

---

# Strict Inward Dependency Rules & Layer Diagram

Dependencies **must strictly point inward / downward**. Higher layers depend on lower layers — **never the reverse**.

```text
                               ┌─────────────────┐
                               │   matis-cli     │ (User UX / CLI Interface)
                               └────────┬────────┘
                                        │
                               ┌────────▼────────┐
                               │   matis-api     │ (Capability API & Transports)
                               └────────┬────────┘
                                        │
                               ┌────────▼────────┐
                               │ matis-reasoning │ (Reasoning Engine / Evidence Collector)
                               └────────┬────────┘
                                        │
                               ┌────────▼────────┐
                               │  matis-context  │ (Context Intelligence Engine [CIE])
                               └────────┬────────┘
                                        │
                      ┌─────────────────┴─────────────────┐
                      ▼                                   ▼
              matis-episodes                         matis-memory / matis-graph
                      │                                   │
                      └─────────────────┬─────────────────┘
                                        │
                               ┌────────▼────────┐
                               │  matis-events   │ (Event Pipeline & Replay)
                               └────────┬────────┘
                                        │
                               ┌────────▼────────┐
                               │  matis-storage  │ (Event & Blob Repositories)
                               └────────┬────────┘
                                        │
                               ┌────────▼────────┐
                               │  matis-kernel   │ (Identity, Clock, Scheduler, ServiceRegistry)
                               └─────────────────┘
```

### Prohibited Dependencies (Enforced by CI)
* `matis-kernel` depending on ANY other matis crate (Kernel is 100% self-contained).
* `matis-storage` depending on `matis-reasoning`, `matis-episodes`, or `matis-memory`.
* `matis-events` depending on `matis-api` or `matis-cli`.
* `matis-episodes` depending on `matis-api` or `matis-cli`.
* `matis-graph` depending on `matis-api`.

---

# Ownership Matrix

| Crate | Canonical Responsibility | Public API Boundary (`pub mod api`) |
|---|---|---|
| `matis-kernel` | Identity, `KernelClock`, Scheduler, ServiceRegistry | `pub mod kernel;` |
| `matis-objects` | 7 Canonical Primitive Structs & Identifiers | `pub mod objects;` |
| `matis-events` | Validation, schemas, replay pipeline | `pub mod events;` |
| `matis-storage` | Persistence repository traits & db implementations | `pub mod storage;` |
| `matis-event-bus` | Channels, pub/sub queues, backpressure | `pub mod bus;` |
| `matis-episodes` | Episode Engine, episode detection & narrative | `pub mod episodes;` |
| `matis-memory` | Working/Semantic memory stores & refinement | `pub mod memory;` |
| `matis-graph` | Knowledge Graph nodes, edges & traversals | `pub mod graph;` |
| `matis-reasoning` | Evidence collection, planning, constraint checks | `pub mod reasoning;` |
| `matis-context` | CIE, token budget optimizer, compression, citations | `pub mod context;` |
| `matis-intelligence`| EIL drift detection, trend analysis, health metrics | `pub mod intelligence;` |
| `matis-protocol` | ECP framing, message definitions, serialization | `pub mod protocol;` |
| `matis-api` | Capability API, HTTP, MCP, WebSocket front doors | `pub mod api;` |
| `matis-daemon` | `matisd` runtime assembly & boot/shutdown sequence | `pub mod daemon;` |
| `matis-cli` | `matis` binary, UX commands, interactive TUI | Binary crate (`main.rs`) |
| `matis-plugin-sdk` | Sensor & plugin traits, WASM/process sandbox | `pub mod plugin;` |

---

# Build Order Sequence

To ensure zero circular dependency locks, crates must be implemented and built in this exact order:

```text
matis-utils ──► matis-kernel ──► matis-objects ──► matis-config ──► matis-events ──► matis-storage ──► matis-event-bus ──► matis-episodes ──► matis-memory & matis-graph ──► matis-context ──► matis-reasoning ──► matis-intelligence ──► matis-protocol ──► matis-plugin-sdk ──► matis-api ──► matis-daemon ──► matis-cli
```

---

# Architecture Freeze Declaration (v1.0)

With this Reference Architecture established, the high-level architecture is **FROZEN (v1.0)**.

Future changes require:
1. Formal Specification update under `specs/`
2. Accepted Architecture Decision Record (ADR) under `docs/01-adrs/`
3. Conformance Test update under `conformance/`
