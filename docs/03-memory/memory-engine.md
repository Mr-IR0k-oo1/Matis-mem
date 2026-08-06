# Memory Engine

## Purpose

The Memory Engine transforms raw engineering events into persistent engineering knowledge.

Unlike traditional logging systems, Matis does not treat every event equally.

Some events are temporary.
Some become permanent.
Some evolve into architectural knowledge.

The Memory Engine decides:

* What should be remembered
* What should be forgotten
* What should be summarized
* What should be promoted
* What should be retrieved

It is the intelligence layer of Matis.

---

## Philosophy

Humans don't remember everything.

They remember:
* Important decisions
* Repeated patterns
* Big failures
* Successful solutions
* Lessons learned

Matis should behave the same way.
Not every shell command deserves immortality.

---

## Memory Architecture

```text
                    Engineering Events
                           │
                           ▼
                   Memory Processing
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 ▼
   Working Memory    Episodic Memory   Semantic Memory
         │                 │                 │
         └─────────────────┼─────────────────┘
                           ▼
                  Context Retrieval Engine
```

Memory is a pipeline.
Not a database.

---

## Three Memory Layers

### Working Memory

**Purpose**: Current engineering context.
Contains everything needed to continue active work.

**Examples**:
* Current branch
* Today's prompts
* Current AI sessions
* Open terminals
* Recent builds
* Recent commits
* Current TODOs
* Recent errors

**Characteristics**:
* Fast
* Volatile
* High detail
* Frequently updated
* Project-specific

**Typical lifetime**: Hours to several days.

---

### Episodic Memory

**Purpose**: Complete engineering history.
Contains every captured engineering event.

**Examples**:
* Prompt history
* Responses
* Git commits
* Shell activity
* Architecture discussions
* Test executions
* Deployments
* Bug investigations

**Characteristics**:
* Append-only
* Chronological
* Immutable
* Replayable

**Retention**: Configurable. Default: 90 Days.
This layer acts as the source material for long-term learning.

---

### Semantic Memory

**Purpose**: Permanent engineering knowledge. Not conversations. Knowledge.

**Examples**:
* Architecture Decisions
* Coding Patterns
* Successful Prompt Templates
* Performance Improvements
* Security Practices
* Project Constraints
* Recurring Problems
* Known Solutions

**Characteristics**:
* Compressed
* Structured
* Searchable
* Long-term
* Cross-project

---

## Memory Lifecycle

```text
Capture -> Normalize -> Store Event -> Working Memory -> Importance Analysis -> Promotion Decision -> Semantic Extraction -> Knowledge Graph -> Retrieval
```

No shortcuts. Every memory begins as an event.

---

## Memory Promotion

Promotion determines whether information deserves permanent storage.
Promotion is based on evidence, not time alone.

### Promotion Factors

* **Importance**: Architecture decisions (Very High) vs. `pwd` (Very Low).
* **Frequency**: Repeated ideas become stronger memories (e.g., Rust ownership patterns).
* **Outcome**: Successful results increase importance (Prompt -> Merged Commit -> Production).
* **User Feedback**: Explicit actions (Pinned, Starred, ADR) cause immediate promotion.
* **AI Confidence**: Extracted knowledge carries confidence (Observed 1.0, Summarized 0.85, Inferred 0.55).

---

## Memory Scoring

Every event receives a score:
* Architecture Decision: +100
* Merged to Main: +80
* Release Tag: +100
* Repeated Pattern: +60
* Bug Fix: +50
* Failed Build: +15
* Temporary Command: -40
* Cursor Movement: -100

Score determines promotion eligibility.

---

## Forgetting

Not everything should survive.
**Examples**: `pwd`, `ls`, `clear`, repeated build output, abandoned experiments.
Forgetting reduces storage growth and improves retrieval quality.

---

## Distillation

Distillation converts thousands of events into meaningful knowledge. It never deletes original events; it creates derived knowledge.

**Example Input**: 124 Prompts, 19 Commits, 3 Deployments.
**Example Output**: "Project migrated from Axum 0.8 to 0.9. Reason: Performance. Outcome: Successful."

---

## Knowledge Objects

Semantic memory stores structured knowledge, not free-form text.
**Example**: Decision, Reason, Evidence, Consequences, Alternatives, Related Events, Confidence, Last Verified.

---

## Retrieval

Retrieval does not search events directly. It builds context.
**Example**: User asks "Continue authentication work."
**Returned context**: Current branch, Auth ADR, Recent commits, Known bugs, Open TODOs, Relevant prompt history.

---

## Memory Decay

Importance decreases over time unless reinforced.
* **Temporary Debugging**: High Today -> Low Next Week -> Expired.
* **Architecture Decision**: Referenced frequently -> Importance Increases.

---

## Cross-Project Learning

Knowledge is not isolated. Matis can recommend optimizations learned from Project A when working on Project B. Experience compounds.

---

## User-Controlled Memory

Users always retain authority.
**Operations**: Pin, Archive, Forget, Restore, Merge, Split, Annotate.
No AI may permanently alter semantic memory without explicit approval.

---

## Memory Invariants

1. Every memory originates from one or more Engineering Events.
2. Raw events remain immutable.
3. Semantic memory is derived, never primary.
4. Promotion never modifies historical events.
5. Retrieval never invents facts.
6. Confidence is preserved.
7. User actions override automatic promotion.
8. Knowledge remains traceable to source events.
9. Forgetting affects derived views, not immutable history.
10. Every retrieved memory must be explainable.

---

## Future Extensions

* Adaptive promotion based on developer habits.
* Team-wide shared memory with provenance tracking.
* Federated memories across multiple devices.
* AI-generated architecture summaries.
* Automatic ADR generation.
* Temporal reasoning (reconstructing state before a regression).

### Architecture Decision Recommendation

**Should semantic memory be fully regenerated from immutable events whenever the distillation algorithm improves, or should promoted knowledge be stored as immutable derived objects with version history?**

The first approach favors reproducibility. The second favors historical continuity. Deciding this early will influence the storage layer and retrieval engine.
