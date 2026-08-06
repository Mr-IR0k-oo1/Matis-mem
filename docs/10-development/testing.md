# Testing Strategy

## Philosophy
Matis-mem is a high-integrity system. We must trust our engineering memory.

## 1. Unit Tests
- Every core logic component (Event normalization, scoring, graph traversal) must have exhaustive unit tests.
- Use `mockall` or trait injection to test components in isolation.

## 2. Integration Tests
- Verify the flow from `Event Bus` -> `Storage` -> `Memory`.
- Test SQLite schema migrations and index rebuilding.
- Use temporary directories for all file-based storage tests.

## 3. End-to-End (E2E) Tests
- Scripted interactions with CLI shims and TUI.
- Verification of MCP responses using a mock AI agent.
- Testing daemon startup/shutdown and adapter lifecycle.

## 4. Property-Based Testing
- Use `proptest` for event serialization and payload validation to ensure schema robustness.

## 5. Architectural Invariant Testing
- Automated checks to ensure the "10 Core Invariants" of the Event Bus and Memory Engine are never violated.
