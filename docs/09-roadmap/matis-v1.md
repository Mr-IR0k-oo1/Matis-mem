# Matis v1.0 Product Definition & Scope

# Purpose

This document defines the exact scope, boundaries, performance budgets, and quality gates for **Matis v1.0**.

Version 1.0 is the smallest complete implementation that validates the Engineering Memory Operating System architecture. Every feature included in v1.0 must directly contribute to enabling a single engineer to recover complete, explainable engineering context instantly. Everything else is explicitly deferred.

---

# Product Definition & Mission Statement

> **"Matis helps engineers resume complex work instantly by remembering engineering history better than humans do."**

Matis v1.0 is a **local-first Engineering Memory Runtime**. It captures engineering activity via passive sensors, organizes events into Engineering Episodes, preserves history in an append-only Event Store, and produces explainable context via `matis continue`.

---

# Core User Story: The `matis continue` Moment

```text
Developer  ──►  Works Normally  ──►  Sensors Capture Events  ──►  Episodes Created  ──►  Laptop Closed
                                                                                               │
                                                                                               ▼
Developer Productive  ◄──  Explainable Context Returned  ◄──  `matis continue`  ◄──  Developer Returns Next Day
```

When the user runs `matis continue`, the system correctly answers:
1. *What am I working on?*
2. *What happened recently?*
3. *Why was this built?*
4. *Which files matter?*
5. *Which decisions matter?*
6. *What should I do next?*

---

# Included Scope for v1.0

* **Microkernel**: `IdentityManager`, `KernelClock`, Priority Scheduler, ServiceRegistry.
* **Event Runtime**: Append-only Event Store (SQLite), Validation, Event Bus, Deterministic Replay.
* **Episode Engine**: Automatic Episode Detection, Narrative Builder, Episode Timeline, Episode References.
* **Context Engine (CIE)**: Token Budget Optimizer, Progressive Compression (`Full -> Summary -> Reference`), Selection Rationale Citations.
* **Memory Engine**: Working Memory & Semantic Memory, Manual Review Queue (`matis memory`).
* **Official Sensors**: Filesystem, Git, Shell, Claude CLI, Gemini CLI, Codex.
* **Core CLI Suite (`matis`)**:
  - `matis init` (Initialize workspace)
  - `matis start` / `matis stop` / `matis status` (Daemon control)
  - `matis continue` (Retrieve instant engineering context)
  - `matis replay` (Run deterministic event replay)
  - `matis episode` (List/inspect active engineering work)
  - `matis memory` (Manage persistent knowledge & ADRs)
  - `matis doctor` (Diagnostic integrity check)

---

# Explicitly Excluded Non-v1 Features

The following are strictly **out of scope** for v1.0:
* Cloud synchronization & SaaS web dashboards
* Team collaboration & multi-tenant organization memory
* Distributed multi-node clustering & federation
* Marketplace & third-party plugin repository
* Mobile applications
* Machine-learning ranking models
* Enterprise SAML/OIDC SSO
* Billing & hosted infrastructure

---

# v1.0 Performance Targets & Quality Gates

| Operation | Budget Target | Conformance Validation |
|---|---|---|
| **Daemon Startup (`matisd`)** | `<2 s` | `conformance/core/startup_tests.rs` |
| **Event Capture Latency** | `<10 ms` | `conformance/core/capture_tests.rs` |
| **Context Retrieval (`matis continue`)**| `<100 ms` | `conformance/core/context_budget_tests.rs` |
| **Event Log Replay** | $\mathcal{O}(N)$ Linear | `conformance/core/replay_determinism_tests.rs` |
| **Shutdown (`matis stop`)** | `<1 s` | `conformance/core/shutdown_tests.rs` |

### Release Quality Gates
1. 100% pass rate on `conformance/core/` test suite.
2. Zero event corruption under abnormal termination tests (`matis doctor` passes).
3. Replay produces 100% deterministic Episode and Memory state.
4. `cargo fmt`, `cargo clippy -D warnings`, and `cargo bench` gates pass cleanly.

---

# Release Milestones

```text
Alpha (v0.2.0)  ──►  Beta (v0.5.0)  ──►  Release Candidate (v0.9.0)  ──►  Production v1.0.0
 (Kernel, Events,      (Episodes, Replay,      (Memory, Plugins,           (Stable Local
  CLI Suite)            CIE Context)            Stability Gates)            Engineering Runtime)
```
