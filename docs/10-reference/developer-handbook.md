# Matis Developer Handbook

# Purpose

The Developer Handbook defines the daily engineering workflow for contributors to the Matis reference implementation.

It explains how to add new features, modify existing systems, debug issues, review changes, and maintain architectural integrity so any contributor can clone the repository, read this document, and become productive immediately.

---

# Repository Orientation

Every directory has a single clear responsibility:

```text
docs/        ──► Concepts, Architecture & ADRs
specs/       ──► Formal Behavioral Specifications (*.spec.md)
crates/      ──► Rust Implementation Workspace (18 crates)
tests/       ──► Verification & Replay Test Suites
conformance/ ──► Official Conformance Certification Suite
benchmarks/  ──► Criterion Latency & Allocation Benchmarks
examples/    ──► Demos, Sensor Examples & CLI Samples
plugins/     ──► Installed Third-Party Sensor Binaries
tools/       ──► Replay & Diagnostic Tooling (`matis-replay`, `matis-doctor`)
fixtures/    ──► Test Event Logs & Golden Outputs
```

---

# Developer Workflow & Command Pipeline

Before starting any task:
1. **Locate Specification**: Read the relevant specification in `specs/`.
2. **Check ADRs**: Read `docs/01-adrs/` for foundational choices.
3. **Locate Crate**: Find the single owning crate in `crates/`.

### Local Execution Pipeline
```bash
# 1. Code formatting check
cargo fmt --check

# 2. Linting check (zero warnings allowed)
cargo clippy --workspace --all-targets -- -D warnings

# 3. Unit, integration & replay test suite
cargo test --workspace

# 4. Run Criterion benchmarks
cargo bench --workspace

# 5. Run Conformance test suite
cargo test --test conformance
```

---

# Step-by-Step Feature Implementation Guide

1. **Step 1 (Owning Crate)**: Identify the exact crate (`crates/matis-[feature]`). Never leak logic into unrelated crates.
2. **Step 2 (Read Contracts)**: Review `specs/*.spec.md` and crate public traits (`src/traits.rs`).
3. **Step 3 (Write Tests First)**: Write unit and replay tests verifying the required contract.
4. **Step 4 (Implement)**: Write code satisfying the specification.
5. **Step 5 (Verify)**: Run `cargo fmt && cargo clippy && cargo test && cargo bench`.

---

# Adding a New Engineering Event Schema

To add a new event variant:
1. Update `EventKind` enum in `crates/matis-objects/src/event_kind.rs`.
2. Implement schema validation in `crates/matis-events/src/validation.rs`.
3. Add serialization property tests (`proptest`) in `crates/matis-events/tests/`.
4. Update Event Specification (`specs/engineering-event.spec.md`).
5. Update Replay Test Suite (`tests/replay/`) to verify deterministic state reconstruction.

---

# Debugging & Replay Investigation Strategy

Always debug from **reality upward**, never from presentation downward:

```text
Engineering Event Store (events.jsonl)
                  │
                  ▼
          Episode Engine
                  │
                  ▼
        Memory & Knowledge Graph
                  │
                  ▼
           Reasoning Engine
                  │
                  ▼
       Context Intelligence (CIE)
                  │
                  ▼
           CLI / Presentation
```

When unexpected behavior occurs:
1. Do **not** start by debugging CLI/TUI logic.
2. Run `matis-replay` tool to replay the event log:
   ```bash
   cargo run --bin matis-replay -- --event-log path/to/events.jsonl
   ```
3. Inspect diffs in generated Episode, Memory, and Graph states.

---

# The 10 Core Contributor Rules

1. Never guess architectural intent — read `docs/01-adrs/`.
2. Never guess behavior — read `specs/*.spec.md`.
3. Never skip replay validation tests (`tests/replay/`).
4. Never introduce a new abstraction without proving the existing one insufficient.
5. Never optimize code without pre/post Criterion benchmark metrics (`cargo bench`).
6. Never duplicate engineering data — store references to `Identity` keys.
7. Never bypass layer boundaries (Kernel depends on nothing; CLI depends on API).
8. Never merge failing conformance tests (`conformance/`).
9. Never weaken deterministic replay or `KernelClock` guarantees.
10. Leave the architecture simpler than you found it.
