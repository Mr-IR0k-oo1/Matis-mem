# Kernel Behavioral Specification (`specs/runtime/kernel.spec.md`)

## 1. Status & Invariants
* **Specification Version**: 1.0.0
* **Status**: Normative Specification
* **Target Crate**: `crates/matis-kernel`

### Core Invariants
1. `matis-kernel` operates as a single-process microkernel (`matisd`).
2. The Kernel depends on zero external domain crates; domain engines depend on Kernel trait abstractions.
3. Time is strictly governed by `KernelClock` for 100% deterministic replayability.
4. Identity (`EventId`, `EpisodeId`, `MemoryId`, `ProjectId`, `ActorId`) is globally unique and immutable.

---

## 2. Kernel Module Contracts

```rust
pub trait KernelRuntime {
    fn identity(&self) -> &IdentityManager;
    fn clock(&self) -> &KernelClock;
    fn scheduler(&self) -> &KernelScheduler;
    fn lifecycle(&self) -> &LifecycleManager;
    fn service_registry(&self) -> &ServiceRegistry;
}
```

### Module Responsibilities
- `IdentityManager`: Generates UUIDv7 / ULID time-ordered unique identifiers.
- `KernelClock`: Abstract clock providing ISO-8601 timestamps with simulation / step replay controls (`step()`, `now()`).
- `KernelScheduler`: 5-tier priority task scheduler (`Critical`, `Interactive`, `Background`, `Maintenance`, `Idle`).
- `LifecycleManager`: Enforces state machine (`Created -> Initialized -> Running -> Paused -> Stopping -> Stopped`).
- `ServiceRegistry`: Decoupled capability lookup for context, memory, graph, and sensor providers.
