# Terminal User Interface (TUI)

## Purpose
The TUI is the "Dashboard" for the developer, providing a high-speed way to interact with engineering memory without leaving the terminal.

## Layout
- **Timeline View**: A vertical stream of recent events.
- **Knowledge Browser**: A searchable tree of distilled knowledge and ADRs.
- **Graph Explorer**: A relationship-focused view of how artifacts are linked.
- **Agent Feed**: Live logs from active AI sessions.
- **Shims Manager**: UI for installing/uninstalling CLI wrappers.

## Interactions
- **Filtering**: Instant filtering by actor, source, or importance.
- **Promotion**: One-key promotion of a log entry to Semantic Knowledge.
- **Querying**: A "Command Bar" for semantic search (e.g., `/search "why did we use redis?"`).
- **Details Pane**: Deep-dive into event payloads, diffs, and metadata.

## Design Principles
- **Keyboard First**: Every action mapped to a shortcut.
- **High Information Density**: Minimize whitespace, maximize context.
- **Performance**: Instant rendering using `ratatui`.
- **Visual Cues**: Color-coding by event type (Green for success, Red for failure, Blue for AI).

## Future Work
- Interactive graph visualization using ASCII/Unicode art.
- Integrated prompt runner with context auto-injection.
