# Matis Conformance Specification

# Purpose

The Matis Conformance Specification defines the mandatory behavioral guarantees that every implementation of the Matis platform must satisfy.

Conformance ensures that independent implementations remain compatible while allowing implementation freedom. An implementation is considered **Matis-compatible** only if it satisfies every required conformance rule.

The specification is the product; the reference implementation (`matisd`) is one compliant implementation of the specification.

---

# Conformance Tiers

```text
                               ┌────────────────────────────────┐
                               │  Extended Conformance Tier     │
                               │  (Federation, Sync, Clusters)  │
                               └───────────────┬────────────────┘
                                               │
                               ┌───────────────▼────────────────┐
                               │  Standard Conformance Tier     │
                               │  (Memory, Graph, Distillation) │
                               └───────────────┬────────────────┘
                                               │
                               ┌───────────────▼────────────────┐
                               │  Core Conformance Tier         │
                               │  (Events, Replay, Episodes, CIE)│
                               └────────────────────────────────┘
```

1. **Core Conformance Tier**: Minimum required implementation. Must support immutable Events, Event Store persistence, deterministic Replay, Episode grouping, CIE context retrieval, and CLI output.
2. **Standard Conformance Tier**: Complete local runtime. Must support Working & Semantic Memory, Knowledge Graph, Distillation Engine, Reasoning Engine, EIL analysis, and Plugin Host.
3. **Extended Conformance Tier**: Enterprise/Federated capabilities. Multi-user synchronization, remote APIs, cluster runtime, organizational memory.

---

# Mandatory Behavioral Rules

## 1. Event Immutability & Persistence
* Engineering Events are 100% append-only and immutable.
* Event Store implementations (SQLite, RocksDB, Postgres, etc.) may vary, but event ordering, identity, checksums, and provenance semantics must remain 100% identical.

## 2. Replay Determinism Property
Given identical source events, configuration, and algorithm version:

$$\text{Replay}(\text{EventStore}) \equiv \text{Original}(\text{EventStore})$$

Replaying an Event Store MUST produce identical Episodes, Memory, Knowledge Graph nodes/edges, and Context bundles.

## 3. Canonical Object Contracts
All 7 canonical object types (`Event`, `Episode`, `Memory`, `Knowledge`, `Artifact`, `Context`, `Project`) must implement:
* Stable `Identity` keys (`UUIDv7` / `ULID`)
* Additive `schema_version`
* Cryptographic `checksum`
* Mandatory `ProvenanceMetadata` (creator, evidence, confidence, generation method)

## 4. ECP Protocol Compliance
Any network or IPC transport implementation must pass the Engineering Context Protocol (ECP) test suite, covering handshake version negotiation, capability discovery, structured errors, replay streams, and push subscriptions.

---

# Conformance Test Suite & Repository Layout (`conformance/`)

```text
conformance/
├── core/
│   ├── event_immutability_tests.rs
│   ├── replay_determinism_tests.rs
│   ├── episode_grouping_tests.rs
│   └── context_budget_tests.rs
├── standard/
│   ├── memory_refinement_tests.rs
│   ├── graph_traversal_tests.rs
│   ├── distillation_tests.rs
│   └── plugin_sandbox_tests.rs
├── protocol/
│   ├── ecp_handshake_tests.rs
│   ├── ecp_streaming_tests.rs
│   └── ecp_error_tests.rs
└── fixtures/
    ├── sample_event_logs/
    └── reference_episodes/
```

Certification (`Core Certified`, `Standard Certified`, `Extended Certified`) requires 100% pass rates across the official `conformance/` test suite.

---

# Core Invariants

1. Immutable Engineering Events.
2. 100% deterministic event replay.
3. Stable Engineering Object identities (`Identity`).
4. Complete provenance metadata for all derived data.
5. Fully explainable context and reasoning citations.
6. Protocol compatibility across ECP transports.
7. Reference-based object relationships (zero data duplication).
8. Replayable platform state from cold storage.
9. Versioned schemas with backward compatibility within major versions.
10. The specification is the product; the reference binary is an implementation.
