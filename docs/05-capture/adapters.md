# `docs/05-capture/adapters.md`

# Purpose

The Capture Layer is responsible for observing engineering activity across operating systems, developer tools, AI assistants, IDEs, terminals, browsers, and version control systems.

Its sole responsibility is converting external activity into standardized Engineering Events.

The Capture Layer never:
* stores events,
* analyzes events,
* creates memories,
* builds graphs,
* serves AI context.

It only observes and normalizes.

---

# Philosophy

Matis does not integrate with applications.

Matis integrates with **engineering activity**.

Whether an event originates from Claude, Gemini, Git, VS Code, Cursor, Neovim, Warp, or an unknown future AI should not matter.

Everything becomes an **Engineering Event**.

The Capture Layer hides implementation differences.

---

# High-Level Architecture

```text
            External World
┌─────────────────────────────────────┐
│  Claude CLI, Gemini CLI, Cursor,    │
│  Codex, Git, Shell, VS Code,        │
│  Browser, Filesystem, Docker, CI    │
└──────────────────┬──────────────────┘
                   │
                   ▼
       ┌──────────────────────┐
       │    Capture Layer     │
       │ (Engineering Sensors)│
       └───────────┬──────────┘
                   │
                   │  Normalize
                   │  Validate
                   │  Enrich
                   │  Publish
                   ▼
          Engineering Event
```

---

# Capture Pipeline

Every adapter follows the same lifecycle:

```text
Observe  ───►  Extract  ───►  Normalize  ───►  Validate  ───►  Enrich  ───►  Publish
```

No adapter is allowed to bypass this pipeline.

---

# Capture Principles

## Passive
Adapters observe. They never modify developer workflows.

---

## Read Only
Adapters never change files, never execute AI, never modify Git, never send prompts.

---

## Cross Platform
Adapters work across Linux, Windows, and macOS. OS-specific implementations expose the exact same interface.

---

## Local First
No captured information leaves the machine unless explicitly configured.

---

## Event Driven
Adapters never poll unnecessarily. Prefer filesystem notifications, Git hooks, OS events, IPC, sockets. Polling is a last resort.

---

# Adapter Interface

Every adapter implements the same contract:

```rust
pub trait CaptureAdapter {
    fn initialize(&mut self) -> Result<()>;
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn health(&self) -> HealthStatus;
    fn capabilities(&self) -> Vec<Capability>;
    fn version(&self) -> &str;
}
```

No adapter exposes storage or memory APIs.

---

# Adapter Categories (Sensors)

```text
Capture Layer (Sensors)
├── AI Sensors (Claude, Gemini, Codex, Cursor, Continue, OpenHands, Aider, Windsurf)
├── Development Sensors (Git, Shell, Filesystem, IDE, Terminal)
├── Infrastructure Sensors (Docker, Kubernetes, CI/CD, Cloud)
├── Research Sensors (Browser, Documentation, Package Managers)
└── Custom Sensors (Plugins, MCP Providers, User Adapters)
```

---

# AI Adapters
Supported providers: Claude, Gemini, Codex, Cursor, Continue, OpenHands, Aider, Cline, Windsurf, Custom MCP Clients.

All produce the same standardized event schema.

---

# Git Adapter
Captures repository evolution: Commit, Branch, Merge, Tag, Checkout, Reset, Rebase, Cherry Pick, Stash, Conflict, Push, Pull.

Git becomes another event producer.

---

# Shell Adapter
Captures meaningful engineering activity: `cargo build`, `cargo test`, `git commit`, `docker compose`, `kubectl apply`, `npm install`, `pnpm build`, `go test`.

Filters ignore noise (`pwd`, `clear`, `ls`, `cd`) unless explicitly enabled.

---

# IDE Adapter
Captures engineering context: Opened Project, Opened File, Closed File, Run Configuration, Debug Session, Breakpoint, Workspace Switch.

Not every cursor movement or keystroke. Only meaningful engineering events.

---

# Filesystem Adapter
Observes project changes: File Created, File Deleted, File Renamed, Directory Created, Configuration Changed, Binary Produced.

No file contents are interpreted here.

---

# Browser Adapter (Optional)
Captures engineering research: Opened Documentation, GitHub Issue, Stack Overflow, RFC, Rust Docs, MDN, API Reference.

Disabled by default.

---

# Container Adapter
Captures development environments: Docker Build, Container Start, Container Stop, Compose Up, Compose Down.

---

# CI Adapter
Captures automation: GitHub Actions, GitLab CI, Jenkins, Azure DevOps, Drone, Buildkite.

---

# Adapter Discovery & Capabilities

Adapters register dynamically and declare capabilities:

```text
Plugin Loaded  ───►  Capability Advertised  ───►  Validation  ───►  Registration  ───►  Ready
```

Consumers subscribe by capability, not by implementation.

---

# Event Enrichment

Adapters may enrich events with observable context:
* Repository
* Branch
* Working Directory
* Hostname
* OS
* Timestamp
* Tool Version
* Language
* Workspace

Adapters must not infer engineering knowledge — only observable context.

---

# Health Monitoring & Isolation

Every adapter reports state (`Running`, `Stopped`, `Paused`, `Failed`, `Recovering`).

A failing adapter never crashes Matis. Every adapter is isolated.

---

# Security

* Read-only access whenever possible.
* Explicit user consent for browser and shell capture.
* Sensitive paths excluded by default.
* No credential collection.
* Configurable redaction before event publication.

---

# Performance Targets

```text
Adapter Startup  <500 ms
Event Latency    <10 ms
Memory           <20 MB
CPU Idle         Near Zero
```

---

# Core Invariants

1. Every captured activity becomes an Engineering Event.
2. Adapters never bypass validation.
3. Adapters never persist data directly.
4. Adapters are independent and fault-isolated.
5. Every adapter implements the same lifecycle interface.
6. Captured events remain platform-neutral.
7. Capture is passive by default.
8. User privacy settings are enforced before publication.
9. Every event includes provenance identifying its adapter.
10. New adapters require no changes to existing adapters or consumers.

---

# Evolution Strategy: From Adapters to Sensors

Think of adapters as **Engineering Sensors**:

```text
Engineering Sensor  ───►  Observation  ───►  Normalization  ───►  Engineering Event
```

This terminology scales into the future for Code Review Sensors, Jira Sensors, Hardware Debugger Sensors, FPGA Build Sensors, or Telemetry Sensors.

---

# Architecture Decision Candidates

* Should sensors run in-process, as separate processes, or as sandboxed WASM modules?
* What is the capability negotiation protocol between the daemon and sensors?
* How are sensor permissions declared and enforced?
* Should sensors support hot loading and unloading without restarting `matisd`?
* What compatibility guarantees exist for third-party sensors across major versions?
