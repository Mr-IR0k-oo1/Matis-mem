# Philosophy

## Overview
**Purpose**: To establish the immutable laws that govern the design and evolution of Matis-mem.

**Responsibilities**:
- Provide a decision-making framework for contributors.
- Ensure architectural consistency.
- Protect user privacy and data ownership.

## Core Principles

### Everything is an Event
We do not store "current state" as the primary source of truth. We store a sequence of immutable events. The "state" is a projection of the event log.

### Everything is Immutable
Once an event is captured, it is never modified or deleted (except for explicit privacy/purging requests). This ensures the integrity of the engineering timeline.

### Memory beats Logging
Logs are for machines; memory is for context. Matis doesn't just record what happened; it attempts to understand and relate what happened.

### Relationships beat Folders
Hierarchy is a lie in complex systems. We use a knowledge graph to link events, commits, and prompts based on their semantic and temporal relationships, not their location on disk.

### Knowledge beats History
History is a list of things that happened. Knowledge is the understanding of *why* they happened. Matis promotes raw events into distilled knowledge.

### AI is a Consumer
The AI is not the master; it is a consumer of the context Matis provides. Matis's primary job is to feed high-signal context to agents.

### Humans remain the Authority
While Matis is autonomous in capture, the human developer is the final arbiter of what constitutes "Knowledge."

### Local First / Privacy by Default
Your engineering reasoning is your most valuable IP. Matis runs locally, stores data locally, and never leaks context to the cloud without explicit instruction.

## Composable & Explainable
The system should be built from small, focused adapters and engines. Every suggestion or context injection provided by Matis must be traceable back to the source events.

## Future Work
- Formalizing the "Philosophy Audit" for new PRs.
