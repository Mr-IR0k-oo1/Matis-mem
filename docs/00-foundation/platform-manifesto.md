# The Matis Platform Manifesto

## Why Matis Exists

Software engineering has accumulated hundreds of excellent tools:
* Git stores source code diffs.
* GitHub hosts repositories.
* VS Code edits files.
* Claude answers questions.
* Gemini explains code.
* Docker packages software.
* CI systems build projects.
* Issue trackers organize tasks.
* Documentation records decisions.

Each tool captures a fragment of engineering — none capture engineering itself.

Engineering is more than files. Engineering is thought, experimentation, failure, discovery, and memory.

Matis exists because engineering deserves its own operating system.

---

## The Problem: Engineering Amnesia

Today's engineering knowledge disappears.
An engineer asks an AI something — the answer disappears.
A difficult bug is solved — nobody remembers why.
A major architecture decision is made — six months later it is questioned again.
The same mistakes repeat. The same research is repeated. The same prompts are rewritten.

Engineering becomes amnesia. Matis exists to eliminate engineering amnesia.

---

## Our Belief

We believe engineering should become cumulative.
* Every solved problem should make the next problem easier.
* Every decision should become future context.
* Every lesson should improve future reasoning.
* Every project should become smarter over time.

Knowledge should compound, not disappear.

---

## What We Build

Matis is **not** an AI assistant, note-taking app, documentation tool, editor, search bar, or vector database.

Matis is an **Engineering Memory Operating System**. It observes engineering, remembers engineering, reasons about engineering, and improves engineering.

---

## Core Principles & Design Values

1. **Reality First**: Engineering Events are immutable observations. Everything else is derived.
2. **History Is Sacred**: Reality cannot be rewritten, only understood better. Append-only history.
3. **Knowledge Must Be Explainable**: Every memory, recommendation, relationship, and insight must have verifiable evidence provenance. Magic is unacceptable.
4. **Local Ownership**: Knowledge belongs to the engineer. Local-first by default; cloud is optional.
5. **Engineering Before AI**: AI is a consumer of engineering intelligence, not the source of truth.
6. **Stable Foundations**: Core contracts evolve slowly. Microkernel remains small; ecosystem grows.
7. **Simplicity Above Cleverness**: The simplest architecture that preserves correctness wins. Longevity over novelty.

When two implementations exist, prefer:
* correctness over convenience
* determinism over heuristics
* explainability over opacity
* composition over coupling
* specifications over assumptions
* replay over reconstruction
* evidence over confidence
* long-term maintainability over short-term speed

---

## The Engineering Contract (10 Invariants)

Every contributor agrees to protect these invariants:

1. Engineering history is immutable.
2. Engineering Objects are canonical primitives.
3. Replay is 100% deterministic.
4. Provenance is mandatory for every derived object.
5. Intelligence is explainable with evidence.
6. The microkernel remains minimal and domain-agnostic.
7. Extensions & plugins cannot weaken core security guarantees.
8. Local-first remains the default.
9. Architecture evolves through ADRs (`docs/01-adrs/`).
10. Simplicity is a platform feature.

If a feature violates these principles, the feature changes — the principles do not.

---

## The Final Goal & Execution Transition

The ambition is to make engineering itself persistent.

Just as Git permanently changed how source code is managed, Matis aims to permanently change how engineering knowledge is created, preserved, reasoned about, and reused.

```text
               Documentation Phase Completed
                            │
                            ▼
              Formal Specifications (`specs/*.spec.md`)
                            │
                            ▼
             ADRs (`docs/01-adrs/ADR-XXXX.md`)
                            │
                            ▼
              Rust Workspace & Crate Boundaries (`crates/`)
                            │
                            ▼
            Minimal Executable Vertical Slice
 (Sensor ──► Event ──► Event Store ──► Episode ──► Reasoning ──► Context API ──► CLI)
```
