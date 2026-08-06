# Matis Implementation Playbook

# Purpose

The Implementation Playbook defines the engineering rules, patterns, conventions, and implementation standards for the Matis reference implementation workspace.

Its purpose is to ensure that independently developed crates and modules exhibit consistent architecture, behavior, performance, and maintainability.

Specifications define **what**; the playbook defines **how**.

---

# Engineering Philosophy & Priority Order

Every contributor must optimize in this exact priority order:

1. **Correctness**: Zero mathematical or contract errors.
2. **Determinism**: 100% reproducible outputs under `KernelClock`.
3. **Simplicity**: Single responsibility per crate/module; prefer explicit pure functions.
4. **Security**: Sandboxed plugins, local-first privacy, secret redaction.
5. **Performance**: Honor performance budgets without compromising correctness.
6. **Maintainability**: Clear trait boundaries, complete documentation, zero cyclic dependencies.

---

# The 6 Golden Rules of Implementation

1. **Rule 1**: No implementation without a specification in `specs/`.
2. **Rule 2**: No public API module without complete documentation (`///`).
3. **Rule 3**: No architectural change without an accepted ADR in `docs/01-adrs/`.
4. **Rule 4**: No performance optimization PR without pre/post Criterion benchmarks.
5. **Rule 5**: No merge without passing deterministic event replay tests.
6. **Rule 6**: **No mutable Engineering Events. Ever.**

---

# Standard Crate Layout & Module Guidelines

Every crate in `crates/` follows a uniform structure:

```text
crates/matis-[feature]/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs       (Exposes `pub mod api;` only)
│   ├── api.rs       (Public capability trait implementations)
│   ├── traits.rs    (Internal and public trait definitions)
│   ├── types.rs     (Crate-specific domain types & errors)
│   ├── error.rs     (Structured error definitions with code, cause & suggestion)
│   └── service.rs   (Core engine or service logic)
└── tests/
    └── integration_tests.rs
```

---

# Structured Error Handling & Logging

## Error Envelope
All crate errors must return a machine-readable envelope:
```rust
pub struct MatisError {
    pub category: ErrorCategory,
    pub code: &'static str,
    pub message: String,
    pub cause: Option<String>,
    pub suggestion: Option<String>,
    pub trace_id: Option<String>,
}
```

## Logging Standard
Structured tracing log entries only:
```text
tracing::info!(
    component = "episode_engine",
    operation = "episode_assignment",
    duration_ms = 4.2,
    trace_id = "tr_8f910a",
    result = "success"
);
```

---

# Testing Pyramid & Performance Budgets

## Testing Pyramid
```text
  Conformance Tests (`conformance/`)      [Mandatory]
  Replay Tests (`tests/replay/`)          [Mandatory]
  Integration Tests (`tests/integration`) [Mandatory]
  Unit Tests (`src/**/tests`)             [Mandatory]
  Property Tests (`proptest`)              [Mandatory for Object & Event serializers]
```

## Performance Budgets
* **Event Capture**: `<10 ms`
* **Context Generation (CIE)**: `<100 ms`
* **Replay Speed**: Linear with event count ($\mathcal{O}(N)$)
* **Cold Start (`matisd`)**: `<2 s`
* **Memory Promotion**: Background idle priority

---

# Final Repository Convergence Directive

With this playbook established, the documentation phase is complete. Every top-level repository directory maps to concrete executable artifacts:

```text
docs/        ──► Concepts, Architecture & ADRs
specs/       ──► Formal Contracts & Schemas (*.spec.md)
crates/      ──► Rust Implementation (`matis-kernel`, `matis-events`, etc.)
tests/       ──► Verification & Deterministic Replay Suites
benchmarks/  ──► Criterion Latency & Memory Benchmarks
examples/    ──► Sample Integrations & Sensor Demos
tools/       ──► Replay & Diagnostic Tooling Binaries
```
