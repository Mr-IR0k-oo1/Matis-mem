# CLI Suite Behavioral Specification (`specs/api/cli.spec.md`)

## 1. Subcommand Matrix
* **Target Crate**: `crates/matis-cli`

```text
matis init              Initialize workspace config & local `.matis/` directory
matis start / stop      Control background `matisd` daemon process
matis status            Check daemon health, event throughput & memory stats
matis continue          Retrieve instant engineering context for active objective
matis replay            Execute deterministic event log replay validation
matis episode           List, inspect, split, merge, or close Engineering Episodes
matis memory            Inspect, review, pin, or annotate semantic memory & ADRs
matis doctor            Diagnostic integrity verification & event log repair
```

## 2. Invariants
1. `matis continue` MUST execute in `<100ms`.
2. Output formats support `--format human` (default markdown), `--format json`, and `--format tree`.
3. CLI subcommands invoke kernel capabilities via `matis-api` transport adapters — zero direct database queries.
