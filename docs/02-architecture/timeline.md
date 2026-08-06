# Timeline Engine

## Purpose
The Timeline Engine provides a linear, chronological view of engineering activity across all actors and sources.

## Philosophy
Context is often temporal. Knowing that a build failed *immediately after* an AI suggested a library change is critical. The Timeline Engine reconstructs the "Stream of Consciousness" of a project.

## Responsibilities
- **Chronological Sequencing**: Ordering events from disparate sources into a single stream.
- **Sessionization**: Grouping events into discrete "work sessions" or "episodes."
- **Marker Injection**: Identifying significant milestones (e.g., "Feature X started," "Regression fixed").
- **Time-Travel**: Providing the state of the project at any point in history.

## Data Model
The Timeline is a view over the Event Store:
- **Event Stream**: The raw sequence of events.
- **Episode**: A collection of events with a start/end time and a primary goal.
- **Checkpoint**: A snapshot of the knowledge graph and state at a specific timestamp.

## Retrieval Patterns
- "What was I doing at 3 PM last Tuesday?"
- "Show me the 10 events leading up to this commit."
- "Summarize the activity of the last 24 hours."

## Future Work
- Visual timeline "scrubber" in the TUI.
- Automatic session naming using AI distillation.
