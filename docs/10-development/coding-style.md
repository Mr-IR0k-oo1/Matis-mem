# Coding Style & Conventions

## Language
Matis-mem is written in **Rust**.

## Core Principles
1. **Safety First**: No `unsafe` code unless absolutely necessary for platform shims (and then, wrapped strictly).
2. **Explicit Errors**: Use `anyhow` for application-level errors and `thiserror` for library-level errors.
3. **Async Everywhere**: Use `tokio` for all I/O and background tasks.
4. **Data Immutability**: Prefer immutable data structures. Once an `EngineeringEvent` is created, it should never be modified.
5. **Trait-Based Design**: Define clear interfaces for Storage, Adapters, and Consumers to allow for easy extensibility.

## Formatting
- Use standard `cargo fmt`.
- Limit line length to 100 characters where possible.
- Use explicit imports (avoid `use crate::*`).

## Naming
- **Events**: `SnakeCase` for types (e.g., `git.commit`, `ai.prompt`).
- **Structs**: `PascalCase`.
- **Variables/Functions**: `snake_case`.

## Documentation
- Document every public module, struct, and function.
- Include "Why" in comments, not just "What."
- Maintain the `docs/` architecture book alongside code changes.
