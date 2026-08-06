# Capture Layer & Adapters

## Purpose
The Capture Layer is the entry point for all external reality. It uses Adapters to normalize diverse data sources into `EngineeringEvents`.

## Adapter Types

### 1. CLI Wrappers (Shims)
Wraps existing tools to capture input/output.
- **AI Shims**: Intercept `gemini-cli`, `claude`, `amp` transcripts.
- **Shell Shims**: Log command history and return codes (filtered for noise).

### 2. Watchers
Background processes that observe system changes.
- **Git Watcher**: Detects commits, branch switches, and merges.
- **Filesystem Watcher**: Detects significant file modifications or project structure changes.

### 3. Plugin-Based
Direct integrations with host applications.
- **IDE Plugins**: Captures file navigation, edits, and tool usage.
- **Browser Extensions**: (Future) Captures research and documentation lookups.

## Normalization Process
1. **Intercept**: The raw data (e.g., a JSON log from Claude) is received.
2. **Map**: Fields are mapped to the canonical `EngineeringEvent` schema.
3. **Enrich**: Metadata like `cwd`, `git_branch`, and `project_id` are added.
4. **Publish**: The event is sent to the Event Bus.

## Noise Reduction
Adapters are responsible for filtering "junk" events.
- Ignore `ls`, `cd`, `clear`.
- Ignore `node_modules`, `.git`, and build artifacts.
- Debounce frequent filesystem events.

## Core Invariants
1. Adapters must never block the primary tool's execution.
2. Adapters must fail silently to the user but log errors to the daemon.
3. Every captured artifact must be attributed to an `Actor` and `Source`.
