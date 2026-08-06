# Plugin SDK Specification (`specs/plugins/sdk.spec.md`)

## 1. Extension Taxonomy & Sandbox Isolation
* **Target Crate**: `crates/matis-plugin-sdk`

```text
                               Plugin Host (`matisd`)
                                         │
              ┌──────────────────────────┼──────────────────────────┐
              ▼                          ▼                          ▼
         Sensor Plugins           Consumer Plugins           Provider Plugins
       (Produce Events)           (Read Events)            (Extend Context)
```

## 2. Invariants & Permissions
1. Third-party plugins execute within strict WASM sandboxes or isolated worker processes.
2. Plugins MUST explicitly declare requested permissions in `plugin.toml`:
   - `filesystem.read`
   - `shell.observe`
   - `git.observe`
   - `network.outbound`
3. Plugins MUST NOT bypass kernel event validation or attempt direct storage access.
4. Plugin crashes MUST be fault-isolated — a failing plugin never terminates `matisd`.
