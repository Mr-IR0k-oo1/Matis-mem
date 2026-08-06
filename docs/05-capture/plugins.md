# `docs/05-capture/plugins.md`

# Purpose

The Plugin System allows Matis to be extended without modifying the core platform.

Plugins add capabilities. They never modify core architecture.

Everything outside the core daemon should ideally be a plugin.

The daemon remains small; the ecosystem grows.

---

# Philosophy

Core should only contain functionality that every installation requires.

Everything else belongs in plugins or first-party modules.

Examples:
```text
Claude Integration

Gemini Integration

VS Code

JetBrains

Slack

Jira

Docker

GitHub

Notion

Custom Sensors
```

None of these belong directly in the daemon core.

---

# Module vs Plugin Architecture Hierarchy

To maintain high architectural stability, Matis organizes extensions into a three-tier hierarchy:

```text
Core Daemon (Immutable contracts, Event Bus, Memory Engine, CIE, Storage Interfaces)
  │
  ├── First-Party Modules (Versioned packages maintained with Matis: Git, Claude, Gemini, Docker)
  │
  └── Third-Party Plugins (Community extensions, Jira, Linear, Slack, Notion, Internal Tools)
```

Where:
* **Core**: Defines immutable event contracts, storage repositories, memory models, and context generation.
* **Modules**: First-party, versioned packages maintained alongside Matis.
* **Plugins**: Third-party extensions running within defined permission sandboxes.

---

# Architecture Diagram

```text
                                matisd
                                   │
              ┌────────────────────┼────────────────────┐
              │                    │                    │
        Core Services          Modules             Plugin Host
              │                    │                    │
              ▼                    ▼                    ▼
     Event Bus / Storage      Git / Claude         Sensors / Consumers
```

---

# Plugin Categories

## 1. Sensors (Producers)
Observe engineering activity (Claude, Gemini, Git, Shell, Filesystem, Browser, Docker). Produce Engineering Events. Nothing else.

---

## 2. Consumers
Consume Engineering Events (Analytics, Metrics, Notifications, Logging, Exporters, Webhooks). Never produce events directly.

---

## 3. Providers
Provide services (Context Providers, Memory Providers, Search Providers, Graph Providers). Clients call providers.

---

## 4. UI Plugins
Extend interfaces (Timeline View, Graph View, Dashboard, Theme, Widgets).

---

## 5. Tool Plugins
Provide CLI commands (`matis benchmark`, `matis doctor`, `matis graph`, `matis export`, `matis replay`).

---

# Plugin Lifecycle

Every plugin follows a strict lifecycle:

```text
Discover  ──►  Load  ──►  Validate  ──►  Initialize  ──►  Register  ──►  Running  ──►  Paused  ──►  Stopped  ──►  Unload
```

No plugin skips lifecycle stages.

---

# Plugin Manifest

Every plugin contains a manifest:

```toml
[plugin]
name = "claude-sensor"
version = "0.3.0"
author = "Matis Team"
description = "Claude CLI & API passive observation sensor"
min_api_version = "0.3.0"
capabilities = ["sensor.ai.claude", "event.producer"]
permissions = ["filesystem.read", "shell.observe"]
```

The daemon refuses invalid or unsigned plugins.

---

# Permissions & Sandboxing

Plugins explicitly request permissions:
* Filesystem Read / Write
* Shell Observation
* Git Observation
* Browser Observation
* Network
* Clipboard
* Notifications

Plugins execute in isolation (in-process trait boundaries, separate processes, or WASM sandboxing) to prevent daemon crashes.

---

# Hot Reload & Event Flow

Plugins communicate only through stable interfaces (Event Bus, Capability API, Service Registry).

Hot reload sequence:
```text
Unload  ──►  Replace Binary  ──►  Validate  ──►  Load  ──►  Resume
```

---

# Recommended SDK Layout (`plugin-sdk/`)

```text
plugin-sdk/
├── core/
│   ├── traits/
│   ├── types/
│   └── errors/
├── sensor/
│   └── sensor.rs
├── consumer/
│   └── consumer.rs
├── provider/
│   └── provider.rs
├── context/
│   ├── request.rs
│   └── response.rs
├── events/
│   └── event.rs
├── permissions/
│   └── permissions.rs
├── manifest/
│   └── manifest.rs
└── testing/
    └── harness.rs
```

The SDK is the single dependency required for third-party plugin development.

---

# Core Invariants

1. Plugins cannot violate Engineering Event invariants.
2. Plugins cannot modify immutable events.
3. Plugins communicate only through public APIs.
4. Plugin failures are fault-isolated.
5. Permissions are explicit and least-privilege.
6. Capability registration is deterministic.
7. Plugin upgrades preserve compatibility contracts where possible.
8. The daemon remains fully functional without optional plugins.
9. Plugin APIs are versioned independently of transport protocols.
10. Every plugin action is observable and auditable.
