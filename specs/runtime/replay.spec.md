# Replay Engine Formal Specification (`specs/runtime/replay.spec.md`)

## 1. Mathematical Determinism Property
Given an immutable Event Store $\mathcal{S}$ and configuration $\mathcal{C}$:

$$\text{Replay}(\mathcal{S}, \mathcal{C}) \equiv \text{Original}(\mathcal{S}, \mathcal{C})$$

Replaying an event log MUST reconstruct 100% identical Episodes, Working Memory, Semantic Memory, and Knowledge Graph nodes/edges.

## 2. Replay Algorithm Contract

```rust
pub trait ReplayEngine {
    fn reset_state(&mut self) -> Result<()>;
    fn step_event(&mut self, event: &EngineeringEvent) -> Result<ReplayStepOutcome>;
    fn replay_log(&mut self, log_path: &Path) -> Result<ReplaySummary>;
}
```

1. **Step 1**: Wipe transient memory/graph state (or initialize fresh in-memory test state).
2. **Step 2**: Initialize `KernelClock` to first event timestamp.
3. **Step 3**: Replay events topologically according to DAG parent linkages (`parents: Vec<EventId>`).
4. **Step 4**: Verify calculated checksums of generated Episode models against golden test fixtures.
