# `docs/02-architecture/kernel.md`

# Purpose

The Matis Kernel is the immutable core of the Engineering Memory Operating System.

It provides the fundamental contracts that every subsystem depends on.

The kernel owns:
* Event lifecycle
* Identity
* Time & Scheduling
* Service orchestration
* Capability registration
* Security boundaries
* Internal messaging (IPC / Event Bus)

The kernel does **not** know about domain specifics like Claude, Gemini, Git, Cursor, Docker, VS Code, or specific database backends.

---

# Philosophy

A stable kernel enables an evolving ecosystem.

The kernel should change less than any other subsystem. Its responsibilities are intentionally minimal. Everything else is replaceable.

---

# Single-Process Microkernel Architecture

Matis employs an **in-process Microkernel Architecture Pattern**.

> **Note**: Microkernel architecture is an internal software design pattern, **not** a multi-process microservice deployment model. Matis runs as **one single, high-performance local process (`matisd`)**. The microkernel is an architectural boundary that isolates core infrastructure from domain engines.

```text
                                Applications
         (Claude, Cursor, VS Code, CLI, TUI, Browser, MCP, Plugins)
                                     │
                                     ▼
===========================================================================
                               MATIS KERNEL
===========================================================================
 Identity Manager  │  Kernel Clock     │  Scheduler      │  Lifecycle
 Capability Reg.   │  Service Registry │  Security / IPC │  Diagnostics
===========================================================================
                                     │
       ┌─────────────────────────────┼─────────────────────────────┐
       ▼                             ▼                             ▼
  Storage Engine               Memory Engine                Knowledge Graph
       │                             │                             │
       ▼                             ▼                             ▼
Reasoning / CIE               Episode Engine             Intelligence Layer
```

Everything depends on the kernel interface. The kernel depends on nothing.

---

# Kernel Responsibilities

## 1. Identity (`IdentityManager`)
Generates permanent, globally unique domain identifiers: `EventId`, `EpisodeId`, `MemoryId`, `ProjectId`, `ActorId`, `NodeId`.

## 2. Time & Deterministic Clock (`KernelClock`)
Defines time via `KernelClock` rather than system calls (`System::now()`). Enables deterministic replays, simulations, and testing.

## 3. Lifecycle (`LifecycleManager`)
Manages subsystem states: `Create -> Initialize -> Running -> Paused -> Stopping -> Stopped`.

## 4. Scheduling (`KernelScheduler`)
Manages asynchronous work execution across priority tiers:
* `Critical` (Immediate event persistence & safety)
* `Interactive` (Sub-100ms context retrieval & queries)
* `Background` (Graph traversals & episode building)
* `Maintenance` (Index rebuilds & snapshots)
* `Idle` (Memory promotion & distillation)

## 5. Capability & Service Registry
Services register capabilities (`Sensor`, `ContextProvider`, `MemoryProvider`, `Exporter`) dynamically. Subsystems query capabilities via the `ServiceRegistry` with zero tight coupling.

## 6. IPC & Event Bus (`KernelIPC`)
Provides internal messaging abstractions (channels, shared memory, IPC bounds) shielding business logic from transport details.

---

# Subsystem & Module Organization

```text
Kernel
├── Boot Manager          (Coordinates deterministic boot sequence)
├── Identity Manager      (Generates EventId, EpisodeId, ProjectId, etc.)
├── Clock Service         (Provides KernelClock for replayability)
├── Scheduler             (Priority queue execution: Critical to Idle)
├── Lifecycle Manager     (Enforces subsystem state transitions)
├── Capability Registry   (Dynamic capability discovery)
├── Service Registry      (Decoupled service lookup)
├── IPC Manager           (In-process event routing)
├── Configuration Manager (Owns workspace & system configs)
├── Security Manager      (Permissions, redaction, sandbox boundaries)
├── Diagnostics           (Health, metrics, tracing, crash recovery)
└── Shutdown Manager      (Flushes event store & safely stops subsystems)
```

---

# Boot & Shutdown Sequence

```text
Boot:     Clock ──► Config ──► Identity ──► Storage ──► Event Bus ──► Service Registry ──► Scheduler ──► Plugins/Sensors ──► API ──► Ready

Shutdown: Stop API ──► Stop Plugins ──► Flush Events ──► Flush Memory ──► Persist State ──► Shutdown Storage ──► Exit
```

---

# Failure Recovery

When a non-kernel subsystem or sensor encounters an error:

```text
Subsystem Error  ──►  Kernel Diagnostic Detector  ──►  Restart Subsystem  ──►  Replay Pending Events  ──►  Resume
```

The kernel remains alive and stable.

---

# Core Invariants

1. Every subsystem has a managed lifecycle.
2. Identity is globally unique and immutable.
3. Time is deterministic and replayable via `KernelClock`.
4. Scheduling is centralized under `KernelScheduler`.
5. Services communicate through registered capability interfaces.
6. Capabilities are dynamically discoverable.
7. Security policies are enforced before execution.
8. Kernel services are independent of specific engineering domains.
9. The kernel never depends on plugins or external tools.
10. System state is 100% recoverable after unexpected termination.
