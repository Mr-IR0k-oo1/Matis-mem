# `docs/02-architecture/matis-protocol.md`

# Purpose: Engineering Context Protocol (ECP)

The **Engineering Context Protocol (ECP)** (formerly referred to as the Matis Protocol) defines the universal communication language of the Engineering Memory Operating System.

Every interaction inside Matis is represented using ECP.

The protocol is independent of:
* operating system
* transport (Unix Domain Socket, Named Pipe, HTTP, WebSocket, MCP)
* programming language
* AI provider
* storage engine
* deployment model

ECP is the canonical, open standard representation of engineering intelligence. Protocols outlive products — ECP is designed to become an ecosystem standard for engineering memory exchange.

---

# Philosophy: Engineering Runtime

Matis is not merely an application or memory manager — it is an **Engineering Runtime**:

```text
                               Engineering Runtime
                                        │
                                  Kernel / ECP
                                        │
             ┌──────────────────────────┼──────────────────────────┐
             ▼                          ▼                          ▼
        Event Bus                   Scheduler                Identity / Time
             ▼                          ▼                          ▼
         Episodes                   Knowledge                    Memory
             ▼                          ▼                          ▼
         Reasoning                 Intelligence                 Context
                                        │
                                        ▼
                            Humans • AI • IDEs • Tools
```

Everything communicates through one language: **Engineering Objects**.

---

# Why an Open Native Protocol (ECP)?

Without ECP, every application and client speaks a different protocol (REST, custom JSON, SQL, etc.).

With ECP:
```text
All Clients (Claude, Gemini, Cursor, VS Code, CLI, TUI, MCP)
                          │
                          ▼
             Engineering Context Protocol (ECP)
                          │
                          ▼
                     Matis Daemon
```

One universal language. One stable contract.

---

# Protocol Objects & Message Types

## Protocol Objects
Nothing crosses subsystem or transport boundaries except canonical ECP objects:
`Event`, `Episode`, `Memory`, `Knowledge`, `Context`, `Project`, `Repository`, `Query`, `Response`, `Capability`, `Health`, `Configuration`.

## Message Types
* `Command` (Instruction execution)
* `Query` (MQL AST requests)
* `Event` (Immutable Engineering Event transmission)
* `Response` (Structured ECP responses with citations)
* `Notification` (Asynchronous alerts)
* `Subscription` (Push stream registration)
* `Heartbeat` (Keepalive & health telemetry)
* `Snapshot` (State portability & backups)
* `Replay` (First-class engineering replay controls)
* `Handshake` (Version & capability negotiation)

---

# ECP Protocol Stack

```text
Engineering Context Protocol (ECP)
├── Object Model            (Canonical Event, Episode, Memory, Context definitions)
├── Message Definitions     (Typed Command, Query, Event, Response payloads)
├── Capability Negotiation  (Dynamic discovery & capability advertising)
├── Session Management      (Transport-independent session establishment)
├── Version Negotiation     (Handshake & backward-compatibility validation)
├── Compression Engine      (Progressive payload compression & Blob referencing)
├── Security & Redaction    (Token redaction, signatures, checksums, TLS/mTLS)
├── Streaming & Subscriptions(Push stream subscriptions for live events & context)
├── Replay & Time-Machine   (First-class episode replay & deterministic ordering)
└── Diagnostics             (Correlated error envelopes & telemetry)
```

---

# Serialization & Transport Independence

ECP separates **meaning** from **byte encoding** and **transport**:

```text
Engineering Layer  ──►  ECP Object  ──►  Serialization (JSON, MessagePack, CBOR, Protobuf)  ──►  Transport Layer
```

Supported Transports:
* Unix Domain Socket (POSIX IPC)
* Windows Named Pipe (Windows IPC)
* TCP / Local RPC
* HTTP / REST (Adapter layer)
* WebSocket (Full-duplex streaming)
* Model Context Protocol (MCP Adapter)
* Shared Memory (High-throughput in-process messaging)

Large binary payloads (logs, patches, screenshots) are stored in the Blob Store and referenced by Blob ID in ECP messages — large transcripts never bloat wire messages.

---

# Core Invariants

1. ECP messages are immutable once transmitted.
2. Every message has a declared schema version.
3. ECP objects remain transport-independent and language-neutral.
4. Protocol evolution is backward-compatible (`v1` endpoints remain supported).
5. Capabilities and versions are negotiated during initial handshake.
6. Every message contains a unique correlation identifier (`trace_id`).
7. Replay preserves original event ordering and provenance.
8. Security metadata, signatures, and redaction rules are explicit.
9. Engineering semantics never depend on serialization format.
10. ECP protocol contracts remain independent of any specific AI provider or storage backend.
