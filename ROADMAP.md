# Matis Project Roadmap

Matis is built vertically, focusing on shipping a complete, working local-first Engineering Memory Runtime (`matis continue`).

## Execution Path (Idea ──► Specs ──► Crates ──► Vertical Slice)

```text
Phase 0: Foundation & Specifications   [COMPLETED] (Architecture Freeze & Initial Specs)
Phase 1: Workspace Bootstrap & Kernel  [IN PROGRESS] (crates/matis-kernel & matis-objects)
Phase 2: Event Runtime & Replay        [PLANNED] (crates/matis-events, matis-storage, deterministic replay)
Phase 3: Episode Engine                [PLANNED] (crates/matis-episodes, automatic episode detection)
Phase 4: Context Engine & CLI          [PLANNED] (crates/matis-context, matis-reasoning, `matis continue`)
Phase 5: Memory Engine                 [PLANNED] (crates/matis-memory, Knowledge Refinement Loop)
Phase 6: Knowledge Graph               [PLANNED] (crates/matis-graph, causal node/edge traversals)
Phase 7: Intelligence (EIL)            [PLANNED] (crates/matis-intelligence, drift detection)
Phase 8: Plugin SDK                    [PLANNED] (crates/matis-plugin-sdk, WASM sandbox)
Phase 9: Official Sensors              [PLANNED] (Claude, Gemini, Git, Shell, IDE Sensors)
Phase 10: v1.0 Production Release      [PLANNED] (Cross-platform stable v1.0.0 release)
```

## Traceability Mandate
Every architectural concept maps 1-to-1:
$$\text{Doc (\texttt{docs/})} \Longrightarrow \text{Spec (\texttt{specs/})} \Longrightarrow \text{Crate (\texttt{crates/})} \Longrightarrow \text{Test (\texttt{tests/})}$$
