# Contributing to Matis

Thank you for your interest in contributing to Matis — the reference implementation of the Engineering Knowledge Standard (EKS) and Engineering Memory Operating System!

## The 6 Golden Rules of Contribution

1. **No Code Without a Spec**: Every PR must reference a behavioral specification in `specs/`.
2. **No Public API Without Docs**: All public trait methods and structs require complete Rustdoc (`///`).
3. **No Architecture Change Without an ADR**: Foundational changes require an accepted ADR in `docs/01-adrs/`.
4. **No Performance PR Without Benchmarks**: Optimization claims require pre/post `cargo bench` metrics.
5. **No Merge Without Passing Replay Tests**: Event replay tests (`tests/replay/`) must pass 100%.
6. **No Mutable Events**: `EngineeringEvent` instances are 100% append-only and immutable.

## Local Development Pipeline

```bash
# 1. Code formatting check
cargo fmt --check

# 2. Linting check (zero warnings allowed)
cargo clippy --workspace --all-targets -- -D warnings

# 3. Unit, integration & replay test suite
cargo test --workspace

# 4. Run Criterion benchmarks
cargo bench --workspace
```
