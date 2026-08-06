# Kernel Scheduler Specification (`specs/runtime/scheduler.spec.md`)

## 1. Priority Tiers & Contracts
* **Target Crate**: `crates/matis-kernel`

```text
Priority Tier 1: Critical     (Event Store persistence, crash recovery)      ──► Sync / Immediate
Priority Tier 2: Interactive  (Context retrieval `matis continue`, MQL)      ──► Target <100ms
Priority Tier 3: Background   (Episode grouping, Knowledge Graph updates)     ──► Target <20ms
Priority Tier 4: Maintenance  (Index rebuilds, snapshot creation, log purge)  ──► Scheduled
Priority Tier 5: Idle         (Memory promotion, background distillation)      ──► Idle CPU
```

## 2. Invariants
1. `Critical` events are persisted to storage before acknowledgment.
2. `Interactive` queries pre-empt `Idle` distillation background tasks.
3. Background queues feature backpressure limits to prevent memory bloat under event storms.
