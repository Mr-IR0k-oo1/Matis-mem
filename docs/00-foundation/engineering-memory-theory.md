# Engineering Memory Theory (EMT)

## Abstract

Software engineering is fundamentally a continuous knowledge creation process. Current developer tools primarily capture static engineering artifacts (source code, commits, pull requests, CI logs, issue tasks). These artifacts represent the outcomes of engineering rather than the engineering process itself.

This document introduces **Engineering Memory Theory (EMT)**, a foundational conceptual framework in which software engineering is represented as an evolving, multi-tiered sequence of observations, episodes, memories, knowledge graph nodes, and explainable context. The goal of EMT is to preserve engineering understanding rather than merely preserving engineering artifacts. Matis serves as the reference implementation of EMT.

---

# 1. Motivation & The Core Hypothesis

Modern engineering generates vast streams of fragmented data across tools (prompts, code diffs, review comments, build logs, stack trace failures). Reconstructing human intent requires manual archaeology across disparate systems.

### Core Hypothesis
$$\text{Engineering Productivity Constraint} = \text{Context Reconstruction Overhead} \gg \text{Code Generation Overhead}$$

Developers repeatedly spend up to 40% of their attention answering:
* *Why was this designed this way?*
* *What failed during previous attempts?*
* *Which architectural alternatives were rejected?*
* *What assumptions still hold true?*

These questions persist because engineering memory is uncaptured and ephemeral.

---

# 2. The 6-Tier EMT Cognitive Memory Hierarchy

Similar to human cognitive architecture, EMT structures engineering memory into six progressive abstraction tiers:

```text
                                Engineering Intelligence
                                           ▲
                                           │  (Reasoning Engine)
                                Engineering Context
                                           ▲
                                           │  (Dynamic CIE Assembly)
                                   Knowledge Graph
                                           ▲
                                           │  (Graph Traverser)
                                   Semantic Memory
                                           ▲
                                           │  (Knowledge Refinement Loop)
                                 Engineering Episodes
                                           ▲
                                           │  (Episode Builder)
                                  Engineering Events
                                           ▲
                                           │  (Sensors & Observation)
                                   Engineering Reality
```

1. **Reality**: Physical developer actions (prompts, code edits, terminal commands, builds).
2. **Events**: Immutable, append-only facts (`EngineeringEvent`).
3. **Episodes**: Coherent units of engineering work (`EngineeringEpisode`).
4. **Semantic Memory**: Distilled permanent knowledge, ADRs, patterns, lessons, constraints (`EngineeringMemory`).
5. **Knowledge Graph**: Causal relationship edges connecting nodes (`KnowledgeNode`).
6. **Context**: Transient, signal-dense context bundles assembled on-demand for AIs or engineers (`ContextBundle`).

---

# 3. Deterministic Replay Equation & Fundamental Formula

The core mathematical identity of Engineering Memory Theory is:

$$\text{Engineering Intelligence} \equiv \text{Replay}\Big(\text{EventStore}_{\text{immutable}}, \text{MemoryEngine}, \text{KnowledgeGraph}, \text{ReasoningEngine}\Big)$$

Where deterministic event log replay guarantees:

$$\text{Replay}(\text{EventStore}) \equiv \text{Original}(\text{EventStore})$$

Replay validation ensures scientific reproducibility across independent implementations.

---

# 4. Measurable Predictions & Open Research Directions

EMT posits five empirically testable predictions:
1. Context retrieval via `matis continue` will reduce task-resume latency by $\ge 70\%$.
2. Repeated architectural debates over superseded ADRs will drop to near zero.
3. Cross-project pattern reuse will increase as semantic memory compounds across workspaces.
4. AI assistant output consistency will improve due to higher signal-to-noise prompt context.
5. Long-lived repositories will retain structural continuity despite engineer rotation.

### Key Research Questions
* *How can Episode boundaries be detected automatically with $>95\%$ precision?*
* *What quantitative heuristics best trigger memory promotion from Episodic to Semantic Memory?*
* *How can graph traversals optimize token budget allocations under severe context window constraints?*

---

# 5. Theoretical Conclusion

Engineering Memory Theory proposes that software engineering should be modeled as an evolving knowledge system rather than a collection of disconnected artifacts.

Matis is the reference implementation designed to prove EMT in production code.
