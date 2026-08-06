# Goals and Non-Goals

## Goals
- **Context Persistence**: Ensure no engineering reasoning is lost between sessions.
- **Autonomous Capture**: Ingest data silently from existing tools (Git, Shell, AI CLIs).
- **AI Empowerment**: Provide high-fidelity, relationship-rich context to AI agents.
- **Zero Friction**: Require minimal manual documentation from the developer.
- **Privacy First**: Keep all engineering IP local and encrypted.
- **Tool Agnostic**: Support any AI tool, IDE, or shell via a standard adapter model.

## Non-Goals
- **Replacing Git**: Matis supplements Git; it does not replace it as the source of code truth.
- **Project Management**: Matis is not a replacement for Jira or Linear (though it can link to them).
- **Real-time Collaboration**: The initial focus is on individual developer memory, not real-time multi-user editing.
- **Code Execution**: Matis observes and records work; it does not execute code itself (this is the job of the agents).
- **Cloud SaaS**: Matis is a local-first tool, not a centralized cloud platform.
