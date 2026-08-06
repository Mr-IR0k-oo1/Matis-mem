# Architecture

## Overview
**Purpose**: To provide a high-level map of the Matis-mem system and its data flow.

**Responsibilities**:
- Define the primary subsystems.
- Map the lifecycle of data from capture to retrieval.
- Establish the boundaries between components.

## Architecture Map

```text
[ Sources ] -> [ Adapters ] -> [ Normalizer ]
                                     |
                                     v
[ Retrieval ] <- [ Engines ] <- [ Event Bus ] -> [ Storage ]
      |              |                               |
      v              v                               v
[ Clients ]     [ Promotion ]                 [ Event Store ]
(TUI/API)       (Graph/Memory)                (Vector/Graph)
```

## Core Components

### 1. Capture Layer
The surface area of the system. Adapters for Shell (shims), Git (hooks), IDEs, and AI Agents (MCP/Logs) intercept data.

### 2. Event Bus
The central nervous system. Every captured action is normalized into a standard Event format and broadcast to listeners.

### 3. Storage Layer
- **Event Store (Immutable)**: The raw, sequential log of everything (SQLite/Flat files).
- **Knowledge Graph**: The relationship model (Nodes: Commit, Prompt, File; Edges: derived_from, references).
- **Vector Index**: For semantic similarity searches.

### 4. Memory Engine
Manages the lifecycle of context. It moves data through **Working Memory** (immediate), **Episodic Memory** (recent sessions), and **Semantic Memory** (permanent knowledge).

### 5. Retrieval & API
Provides context to the outside world. This includes the TUI for humans and the MCP/REST API for AI agents.

## Data Flow
1. **Capture**: A user runs a git commit.
2. **Normalize**: The Git Adapter creates a `GIT_COMMIT` event.
3. **Broadcast**: The Event Bus sends it to Storage and the Memory Engine.
4. **Relate**: The Knowledge Graph creates an edge between the new commit and the last `AI_RESPONSE` that suggested the change.
5. **Retrieve**: When the user starts a new prompt, the Query Engine finds the commit and its reasoning to inject as context.

## Future Work
- Decentralized event synchronization for multi-device setups.
- Real-time visualization of the event bus.
