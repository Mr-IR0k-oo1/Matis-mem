# Non-Goals

## Scope Boundaries
To keep Matis-mem focused and maintainable, the following are explicitly **not** within the project's scope:

### 1. Code Generation
Matis-mem records the engineering process; it does not generate code itself. It provides the *context* for other agents to generate code more effectively.

### 2. Version Control System (VCS)
Matis-mem is not a replacement for Git. It does not track file versions or manage branches. It observes Git activity and relates it to other engineering events.

### 3. Integrated Development Environment (IDE)
Matis-mem is not a code editor. It integrates with IDEs via plugins and shims to capture context, but it does not provide editing capabilities.

### 4. Build System
Matis-mem is not a build tool. It observes the output and results of build tools (like `cargo`, `npm`, `make`) to record outcomes and failures.

### 5. Task / Project Management
Matis-mem is not a ticket tracker. While it can reference issues in Jira or Linear, it is not designed for project planning, sprint management, or team resource allocation.

### 6. Cloud-First Platform
Matis-mem is designed as a local-first, privacy-preserving tool. A centralized cloud platform for hosting engineering memory is not an initial goal.

### 7. Real-Time Multi-User Collaboration
The primary focus is the "Personal Engineering Memory" of a single developer. Synchronizing memory across a team in real-time is a future consideration, not a core goal for v1.0.
