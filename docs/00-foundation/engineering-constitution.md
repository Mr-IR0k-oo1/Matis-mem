# The Engineering Constitution of Matis

**Version 1.0**

This document defines the permanent engineering principles of Matis. Every design decision, implementation, protocol, API, optimization, and future feature must be evaluated against these principles.

Architecture may evolve. Implementation may change. **This constitution does not.**

---

## Article I: Reality Is Immutable
Engineering history is reality. Reality cannot be rewritten. Engineering Events are immutable observations; corrections create new events and never modify existing ones. This enables replay, trust, auditing, and explainability.

## Article II: Engineering Before Software
Matis models engineering, not databases, APIs, or user interfaces. Engineering concepts always take precedence over implementation details. The platform vocabulary consists of Events, Episodes, Memory, Knowledge, and Context — never raw tables, endpoints, or SQL queries.

## Article III: Evidence Before Intelligence
Nothing may exist without evidence. Every recommendation, memory, lesson, relationship, insight, and prediction must trace back to Engineering Events. No exceptions.

## Article IV: Explainability Before Accuracy
An explainable answer with slightly lower confidence is preferable to an opaque answer with higher confidence. Every output should answer: *"Why do I believe this?"*

## Article V: Local Ownership
Engineering knowledge belongs to engineers. Cloud synchronization is optional; local storage is the default. Remote services must never become mandatory.

## Article VI: Determinism
Given identical Events, configuration, and algorithms, Matis produces 100% identical Episodes, Memory, Knowledge, and Context. Determinism is mandatory.

## Article VII: Simplicity
Every subsystem should have exactly one responsibility. Complexity is permitted only when it eliminates greater complexity elsewhere. Accidental complexity is a bug.

## Article VIII: Replaceability
Every implementation is replaceable behind stable kernel contracts. Storage changes, protocols evolve, reasoning improves — contracts remain intact.

## Article IX: Stability
The kernel changes slowly. Specifications change carefully. Plugins evolve rapidly. Innovation belongs at the edges; stability belongs at the center.

## Article X: Replayability
Every piece of derived knowledge must be reconstructable from immutable Engineering Events. Replay is the ultimate validation.

## Article XI: Layered Intelligence
Engineering intelligence grows strictly upward (`Events -> Episodes -> Memory -> Knowledge -> Reasoning -> Intelligence -> Context`). Higher layers never modify lower layers.

## Article XII: Minimal Kernel
The microkernel owns only identity, time, scheduling, capabilities, lifecycle, messaging, and security. Nothing else.

## Article XIII: Engineering Episodes Are First-Class
Engineering work is represented by Episodes, not raw prompts or commits. Episodes are the primary unit of engineering understanding.

## Article XIV: AI Is a Consumer
Artificial Intelligence consumes Engineering Intelligence — it never owns it. The platform remains fully functional without any LLM. No subsystem may assume an AI model exists.

## Article XV: Specifications Before Implementations
Architecture defines intent. Specifications define behavior. Code implements specifications. The order is never reversed.

## Article XVI: Evolution Through ADRs
Permanent architectural changes require an Architecture Decision Record (ADR). No silent architectural drift.

## Article XVII: Privacy By Default
Engineering history is private. Collection, sharing, and synchronization are explicit and opt-in. Privacy is a default behavior.

## Article XVIII: Protocol Independence
Engineering semantics never depend on HTTP, MCP, JSON, databases, or operating systems. Protocols transport meaning; they do not define it.

## Article XIX: Trust
Users should always trust Matis more after inspecting its reasoning than before. The platform earns trust through evidence, transparency, determinism, and reproducibility.

## Article XX: Long-Term Thinking
Every design decision should answer: *"Will this still make sense in ten years?"* Short-term convenience must not compromise long-term architecture.

---

# Constitutional Test (10 Checks)

Before merging any significant PR or architectural change, ask:
1. Does it preserve immutable history?
2. Can it be replayed deterministically?
3. Is every conclusion explainable with evidence provenance?
4. Does it preserve local ownership and default privacy?
5. Does it simplify the system?
6. Is it transport and backend independent?
7. Does it strengthen engineering intelligence?
8. Is there an ADR if it changes architecture?
9. Would a contributor understand it in five years?
10. Would removing this feature make the platform conceptually cleaner?

If several answers are "no", redesign the feature.

---

# Implementation Phase Roadmap

```text
Phase 0: Freeze Architecture, Constitution & ADRs (COMPLETED)
                          │
                          ▼
Phase 1: Formal Specifications (`specs/*.spec.md`)
(engineering-event.spec.md, engineering-episode.spec.md, engineering-memory.spec.md, ecp-protocol.spec.md)
                          │
                          ▼
Phase 2: Rust Workspace & Kernel (`crates/matis-kernel`)
                          │
                          ▼
Phase 3: Executable Vertical Slice
(Sensor ──► Event ──► Event Store ──► Episode Engine ──► Reasoning ──► Context API ──► CLI)
                          │
                          ▼
Phase 4: Replay, Memory Engine, Knowledge Graph & Distillation
                          │
                          ▼
Phase 5: Plugins, IDE Extensions, AI Adapters & ECP Transport
```
