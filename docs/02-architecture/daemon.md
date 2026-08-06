# `docs/02-architecture/daemon.md`

# Purpose

The Matis Daemon (`matisd`) is the always-running background service responsible for observing engineering activity, processing events, maintaining memory, and serving context to clients.

Every interface, including the CLI, TUI, IDE plugins, AI tools, and APIs, communicates with the daemon.

The daemon is the product.

Everything else is a client.

---

# Philosophy

The daemon is the single source of runtime truth.

Instead of every client scanning repositories, reading logs, parsing Git history, and rebuilding context independently, all engineering knowledge flows through one continuously running service.

```text
AI

CLI

IDE

Browser

↓

matisd

↓

Engineering Memory
```

---

# Responsibilities

The daemon owns runtime orchestration.

It is responsible for:

* Event ingestion
* Event validation
* Event routing
* Memory maintenance
* Timeline updates
* Knowledge graph updates
* Context serving
* Project discovery
* Health monitoring
* Plugin lifecycle

It is **not** responsible for:

* Rendering UI
* Executing AI models
* Editing files
* Git operations
* Code generation

---

# Runtime Architecture

```text
                 +----------------------+
                 |      matisd          |
                 +----------+-----------+
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
  Capture Engine      Memory Engine      Query Engine
        │                   │                   │
        └──────────────┬────┴─────┬────────────┘
                       ▼          ▼
                Event Store   Knowledge Graph
                       │
                       ▼
                Local Context API
```

---

# Startup Sequence

```text
Boot

↓

Load Configuration

↓

Initialize Storage

↓

Recover Pending Events

↓

Discover Projects

↓

Start Event Bus

↓

Start Watchers

↓

Start Memory Engine

↓

Build Context Cache

↓

Accept Client Connections
```

The daemon should always recover cleanly after crashes.

---

# Internal Services

The daemon consists of independent services.

```text
Configuration

Project Manager

Capture Manager

Event Bus

Storage Manager

Timeline Manager

Memory Manager

Knowledge Graph

Search Index

Context Engine

Plugin Manager

API Server

Health Monitor
```

Every service communicates through the Event Bus.

---

# Project Discovery

The daemon continuously discovers repositories.

Sources include:

```text
Configured Workspaces

Recent Projects

Git Repositories

IDE Activity

Explicit Registration
```

Projects appear automatically without requiring manual import.

---

# Capture Lifecycle

```text
Filesystem

↓

Adapter

↓

Engineering Event

↓

Validation

↓

Event Bus

↓

Storage
```

Adapters never bypass the daemon.

---

# Event Processing Pipeline

```text
Receive Event

↓

Validate

↓

Assign Metadata

↓

Publish

↓

Persist

↓

Index

↓

Graph Update

↓

Memory Evaluation

↓

Context Cache Update
```

Every event follows the same path.

---

# Context Service

Clients never query storage directly.

Instead:

```text
Client

↓

Context API

↓

Context Engine

↓

Memory

↓

Knowledge Graph

↓

Timeline

↓

Result
```

This keeps retrieval logic centralized.

---

# Background Jobs

The daemon schedules maintenance tasks.

Examples:

```text
Memory Promotion

Graph Optimization

Index Rebuild

Archive Cleanup

Cache Refresh

Integrity Check

Plugin Updates

Snapshot Creation
```

These jobs run without interrupting foreground work.

---

# Resource Management

The daemon remains lightweight.

Target goals:

```text
Idle CPU          <1%
Idle Memory       <150 MB
Cold Start        <2 seconds
Context Retrieval <100 ms
Event Processing  <10 ms average
```

These are engineering targets.

---

# Client Interfaces

Every client communicates through the same API.

Examples:

```text
CLI

TUI

VS Code

JetBrains

Neovim

Claude Adapter

Gemini Adapter

Codex Adapter

REST

MCP

Future Plugins
```

Clients contain almost no business logic.

---

# Plugin System

The daemon exposes a plugin interface.

Plugins may contribute:
* Event producers
* Event consumers
* Context providers
* Search providers
* Visualizations

Plugins cannot bypass validation.

---

# Configuration Reload

Configuration changes should not require restarting the daemon.

Supported changes:

```text
Workspace Paths

Memory Policies

Capture Rules

Plugin Settings

Retention Policies

Logging Levels
```

Unsupported changes clearly indicate that a restart is required.

---

# Crash Recovery

The daemon must recover gracefully:

```text
Load Last Checkpoint

Replay Pending Events

Rebuild Missing Indexes

Resume Watchers

Resume Background Jobs
```

No engineering history should be lost because of a daemon crash.

---

# Multi-Project Scheduling

The daemon handles multiple active repositories.

Scheduler priorities:

```text
Active Repository       Highest
Recent Activity         High
Background Projects     Medium
Archived Projects       Low
```

This prevents inactive repositories from consuming resources.

---

# Local API

The daemon exposes a local interface:

```text
Submit Event

Query Timeline

Query Memory

Retrieve Context

Search Knowledge

Register Plugin

Health Status

Metrics

Configuration

Replay Events
```

Every client uses the same interface.

---

# Observability

The daemon continuously reports health.

Metrics:

```text
Uptime

CPU Usage

Memory Usage

Event Throughput

Queue Depth

Context Latency

Storage Size

Replay Progress

Background Jobs

Plugin Status
```

Observability is a first-class feature.

---

# Security

The daemon operates under least privilege:
* No outbound network access unless explicitly enabled.
* Local-first by default.
* Plugins run within defined permission boundaries.
* Sensitive event payloads are encrypted at rest when configured.
* Client authentication is required for remote interfaces.
* Audit logs are generated for administrative actions.

---

# Core Invariants

1. Only one active daemon instance manages a workspace unless clustering is explicitly configured.
2. Every event passes through validation before storage.
3. Clients never access storage directly.
4. Background jobs never block event ingestion.
5. Daemon restarts never lose acknowledged events.
6. Context retrieval remains available even while maintenance jobs run.
7. Plugins cannot violate core event invariants.
8. All runtime state can be reconstructed from persistent data.
9. Health information is always available.
10. The daemon remains independent of any specific AI provider.

---

# Future Evolution

* Multi-device synchronization.
* Team-shared engineering memory.
* Distributed event replication.
* Cluster mode for organizations.
* Remote project indexing.
* High-availability deployments.
* Enterprise authentication.
* Encrypted peer-to-peer memory sharing.

---

# Architecture Decision Candidates

* Should `matisd` use a single-process asynchronous runtime or multiple isolated worker processes?
* How should plugins be isolated: native Rust traits, WASM, or external processes?
* Should the local API default to Unix domain sockets / Windows named pipes, with TCP optional?
* How are long-running background jobs prioritized without affecting interactive latency?
* What is the strategy for zero-downtime daemon upgrades?
