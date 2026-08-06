# `docs/08-api/api.md`

# Purpose

The Matis API is the primary communication layer between external clients and the Matis daemon.

Every interaction with engineering memory flows through this interface.

The API exposes engineering intelligence, not storage internals.

Clients should never know:
* where events are stored,
* how memory is promoted,
* how graphs are built,
* how indexing works.

The API provides stable capabilities while allowing the internal architecture to evolve.

---

# Philosophy

The API is capability-based.

Clients ask for outcomes, never implementation details.

* **Bad**: `SELECT * FROM events WHERE ...`
* **Good**: `retrieve_context(ContextRequest { objective: "Continue OAuth work", ... })`

---

# High-Level Architecture

```text
               Applications (Claude, Gemini, Cursor, VS Code, CLI, TUI, Browser, MCP)
                                         │
                                         ▼
                            ┌─────────────────────────┐
                            │        MATIS API        │
                            └────────────┬────────────┘
                                         │
                                Intent Classification
                                         ▼
                             Context Intelligence Engine
                                         │
                         ┌───────────────┼───────────────┐
                         ▼               ▼               ▼
                      Memory           Graph          Timeline
                         │               │               │
                         └───────────────┼───────────────┘
                                         ▼
                                   Storage Layer
```

Clients never touch internal subsystems directly.

---

# API Design Principles

## Stable
Public APIs evolve slowly. Internal implementations evolve freely.

---

## Stateless
Requests contain sufficient context. No hidden client session state.

---

## Local First
Communication occurs locally. Remote access is optional.

---

## Versioned
Every endpoint belongs to an explicit API version (e.g. `/v1/`).

---

## Explainable
Every response contains provenance and rationale citations. Nothing appears without traceability.

---

# Capability API & Transport Abstraction

The API is structured as an **Internal Capability Interface**. Transports layer on top:

```text
                 Clients
                    │
                    ▼
          ┌───────────────────┐
          │ Transport Adapter │ (HTTP, MCP, CLI IPC, WebSocket, Unix Socket, Named Pipe)
          └─────────┬─────────┘
                    │
                    ▼
          ┌───────────────────┐
          │   Capability API  │ (Context, Memory, Timeline, Events, Projects, etc.)
          └─────────┬─────────┘
                    │
                    ▼
              Matis Daemon
```

The daemon exposes capabilities. Transports expose those capabilities. REST is one transport, MCP is another, CLI IPC is another.

---

# Communication Protocols

```text
Unix Domain Socket    (POSIX)
Windows Named Pipe    (Windows)
HTTP / REST          (Web / External)
WebSocket             (Streaming updates)
MCP                   (Model Context Protocol for AI tools)
CLI IPC               (Command-line tools)
```

Transport is an implementation detail; capability remains identical.

---

# API Domains

```text
Matis API
├── Context       (Context Intelligence Engine queries)
├── Timeline      (Chronological event stream queries & replays)
├── Events        (Publishing & reading raw immutable events)
├── Memory        (Working, Episodic, Semantic memory interactions)
├── Knowledge     (Semantic entity & ADR queries)
├── Search        (Cross-subsystem engineering search)
├── Projects      (Workspace & repository lifecycle management)
├── Plugins       (Sensor & consumer registration)
├── Health        (Daemon monitoring & telemetry)
├── Configuration (Runtime policy reloads)
└── Administration(Archiving, snapshots, backups)
```

---

# Context API

Primary endpoint.

Example Request:
```json
{
  "objective": "Continue OAuth implementation",
  "project": "auth_service",
  "current_branch": "feature/oauth",
  "token_budget": 8000
}
```

Example Response:
```json
{
  "intent": "continuation",
  "context_markdown": "# Active Work...",
  "token_estimate": 3200,
  "citations": [
    {
      "item_id": "mem_20260806_01",
      "title": "OAuth Architecture Decision",
      "explanation": "Selected from Semantic Memory: ADR-0004",
      "confidence": 0.98
    }
  ]
}
```

The client receives engineering context, not raw unparsed events.

---

# Memory API

Capabilities:
* Create Memory
* Pin Memory
* Forget Memory
* Archive Memory
* Annotate Memory
* Query Memory

Raw events remain immutable; memory actions record new memory items.

---

# Timeline API

Queries engineering history chronologically:
* What happened yesterday?
* Show activity before commit `X`.
* Show events between releases.
* Replay today's work.

---

# Event API

Operations:
* Publish Event
* Query Event
* Replay Events
* Subscribe (Stream)
* Validate

Consumers never modify events.

---

# Search & Knowledge API

Search spans events, memory, graph, decisions, and timeline.

Capabilities:
* Architecture decisions (ADRs)
* Engineering patterns & best practices
* Lessons learned & known constraints
* Prompt & commit search

---

# Project & Plugin API

Project lifecycle: `Register`, `Open`, `Close`, `Archive`, `Snapshot`, `Restore`, `List`.

Plugin API: `Register Sensors`, `Register Consumers`, `Provide Context`, `Extend Search`, `Add Commands`.

---

# Streaming & Subscriptions

Supported streams:
* Live Timeline Stream
* Raw Event Stream
* Context Updates
* Memory Updates
* Health & Telemetry Updates

Uses push subscriptions rather than polling.

---

# Error Model

Standard error structure:
```json
{
  "error": {
    "code": "INVALID_EVENT_PAYLOAD",
    "message": "Payload schema validation failed for event_kind 'git'",
    "cause": "Missing required field 'hash'",
    "suggestion": "Check Git capture sensor schema version",
    "trace_id": "trace_8f29d10a"
  }
}
```

---

# Rate Limiting & Performance Targets

Target metrics:
```text
Context Retrieval   <100 ms
Memory Query        <50 ms
Health Check        <10 ms
Timeline Query      <100 ms
Streaming Latency   <20 ms
```

---

# API Invariants

1. Clients never access storage directly.
2. All engineering data is traceable to source events.
3. Responses are deterministic for identical inputs.
4. API contracts remain versioned (`/v1/`).
5. Public interfaces remain transport-independent.
6. Sensitive information is redacted before transmission.
7. Event immutability is never violated.
8. Clients cannot bypass memory policies.
9. Every response contains sufficient provenance for explanation.
10. Internal implementation changes do not break public capabilities.

---

# Architecture Decision Candidates

* Should the primary internal API use Rust traits with transport adapters layered on top?
* Is MCP a first-class transport or implemented as a plugin?
* Should subscriptions use a unified event stream abstraction across WebSocket and local IPC?
* How are long-running operations represented: streaming, polling, or asynchronous jobs?
* What compatibility guarantees exist between daemon versions and client versions?
