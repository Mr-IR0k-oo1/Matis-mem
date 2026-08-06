# System Lifecycle

## Purpose
This document defines the phases of the Matis-mem lifecycle, from installation to daily operation.

## 1. Installation & Setup
1. **Binary Install**: `matis` binary is placed in the PATH.
2. **Init**: `matis init` sets up the configuration directory (`~/.config/matis-mem`).
3. **Shim Installation**: `matis shims install` creates symlinks/wrappers for `gemini-cli`, `claude`, etc.

## 2. Startup
1. **Daemon Launch**: `matis daemon start` (or auto-start on login).
2. **Project Discovery**: Daemon scans the current workspace for `.git` or `Cargo.toml`.
3. **Adapter Activation**: Watchers for Git and Filesystem are initialized.

## 3. Daily Operation (Capture-Process-Retrieve)
1. **Activity**: Developer uses tools (Shell, Git, AI).
2. **Capture**: Adapters publish events to the Bus.
3. **Storage**: Events are persisted to the Immutable Store.
4. **Induction**: Memory Engine updates Working Memory.
5. **Retrieval**: AI agents query memory via MCP for current task context.

## 4. Background (Promotion & Distillation)
1. **End-of-Session**: Daemon detects inactivity or project switch.
2. **Analysis**: Memory Engine scores events for importance.
3. **Promotion**: High-scoring clusters are promoted to Episodic Memory.
4. **Distillation**: (Periodic) AI summarizes episodic clusters into Semantic Knowledge.

## 5. Maintenance
1. **Indexing**: Background rebuild of FTS and Vector indices.
2. **Archiving**: Moving old episodic events to compressed storage.
3. **Backup**: Exporting periodic knowledge bundles.
