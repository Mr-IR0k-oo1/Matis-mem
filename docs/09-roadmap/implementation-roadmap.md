# Matis Implementation Roadmap

# Purpose

The Implementation Roadmap defines the step-by-step execution strategy for building Matis.

Its goal is to maximize validated learning while minimizing architectural risk. Progress is measured by working, demonstrable software — never by documentation.

---

# Philosophy: Vertical Slices Over Horizontal Layers

Build **vertically**, not horizontally:

```text
  BAD (Horizontal):   100% Storage  ──►  100% Memory  ──►  100% Protocol  ──►  0% Working Product
  GOOD (Vertical):    Sensor ──► Event ──► Storage ──► Episode ──► Context ──► CLI ──► Working Product
```

Every milestone produces a working, testable product that deepens the vertical slice.

---

# The 10 Implementation Phases

```text
Phase 0: Foundation         (ADRs, Specs, Workspace, CI, Lints, Format)         ──► v0.0.1
Phase 1: Runtime Kernel     (Identity, KernelClock, Scheduler, Service Registry) ──► v0.1.0 (matisd boot)
Phase 2: Event Runtime     (Event Store, Validation, Event Bus, Replay)          ──► v0.2.0 (Deterministic replay)
Phase 3: Episode Engine     (Automatic Episode Detection, Builder, Timeline)     ──► v0.3.0 (Episode stories)
Phase 4: Context Engine     (Reasoning Engine, CIE, MQL, `matis continue` CLI)   ──► v0.4.0 (Useful context)
Phase 5: Memory Engine      (Working & Semantic Memory, Knowledge Refinement)    ──► v0.5.0 (Persistent memory)
Phase 6: Knowledge Graph    (Node/Edge Graph, Traversal, Graph Search)           ──► v0.6.0 (Causal graph)
Phase 7: Intelligence (EIL) (Background Distillation, Drift Detection, Health)   ──► v0.7.0 (Active intelligence)
Phase 8: Plugin Platform    (Plugin SDK, WASM/Process Isolation, Capabilities)   ──► v0.8.0 (Ecosystem SDK)
Phase 9: Official Sensors   (Claude, Gemini, Git, Shell, IDE, Docker Sensors)    ──► v0.9.0 (Full workflows)
Phase 10: Stable Platform   (Cross-platform production release, docs, ECP std)   ──► v1.0.0 (Stable release)
```

---

# Traceability Chain (Idea ──► Spec ──► Tests ──► Code)

Every architectural concept must maintain a strict 1-to-1 traceability chain:

```text
Architecture Doc (docs/02-architecture/feature.md)
                      │
                      ▼
Formal Specification (specs/feature.spec.md)
                      │
                      ▼
Executable Test Suite (tests/feature/)
                      │
                      ▼
Rust Crate Implementation (crates/matis-feature/)
```

---

# Definition of Done (DoD) & Quality Gates

A phase or feature is complete **only** when:
1. Implementation exists and compiles (`cargo check`, `cargo build`).
2. Test suite passes (`cargo test`) including deterministic replay tests.
3. Formal specification (`specs/*.spec.md`) is satisfied.
4. Performance gates pass (`<2s` cold start, `<100ms` context retrieval, `<10ms` capture).
5. Code formatting & linting pass (`cargo fmt`, `cargo clippy`).
6. Relevant ADRs (`docs/01-adrs/`) are updated.

---

# Anti-Goals for v1.0

Do **not** build during v1.0:
* Cloud synchronization & team SaaS dashboards
* Enterprise SAML/OIDC SSO
* Distributed multi-node clustering
* Mobile applications
* Proprietary ML ranking models
* Fancy web dashboards

Focus strictly on a rock-solid, local-first Engineering Memory Operating System CLI and runtime daemon.
