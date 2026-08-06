# `docs/02-architecture/knowledge-graph.md`

# Purpose

The Knowledge Graph is the intelligence layer of Matis.

While the Event Store records **what happened**, the Knowledge Graph explains **how everything is connected**.

It transforms millions of independent Engineering Events into a connected graph that can answer complex engineering questions.

Without the Knowledge Graph, Matis is an event logger.

With it, Matis becomes an engineering reasoning engine.

---

# Philosophy

Engineering is not a sequence of unrelated actions.

Everything has relationships.

A prompt produces code.

Code modifies files.

Files become commits.

Commits implement features.

Features resolve issues.

Issues influence architecture.

Architecture affects future prompts.

Matis should model these relationships explicitly.

---

# Knowledge Hierarchy

```text
Raw Events

↓

Relationships

↓

Timeline

↓

Knowledge Graph

↓

Engineering Intelligence

↓

AI Context
```

Events are facts.

Relationships create meaning.

---

# Core Principle

Everything inside Matis is either:

* A Node
* An Edge

Nothing else.

---

# Graph Model

```text
             ┌────────────┐
             │   Prompt   │
             └─────┬──────┘
                   │generated
                   ▼
             ┌────────────┐
             │ AI Reply   │
             └─────┬──────┘
                   │created
                   ▼
             ┌────────────┐
             │ Rust File  │
             └─────┬──────┘
                   │committed
                   ▼
             ┌────────────┐
             │ Git Commit │
             └─────┬──────┘
                   │released
                   ▼
             ┌────────────┐
             │  Release   │
             └────────────┘
```

Everything is connected.

---

# Node Types

The graph stores domain objects instead of raw logs.

## Project

Represents a software project.

Examples:
```text
Quantrix

Matis

Astraeus

Website
```

---

## Repository

Git repository.

Stores:
* remote
* branch
* tags
* owner

---

## Actor

Represents an entity performing work.

Examples:
```text
User

Claude

Gemini

Cursor

Cargo

Git

Daemon
```

---

## Prompt

Represents a user request.

Examples:
```text
Implement OAuth

Fix lifetime errors

Optimize parser
```

---

## Response

Represents generated output.

May include:
* explanation
* patch
* implementation
* review

---

## Decision

Architecture decisions.

Examples:
```text
Use SQLite

Switch to Axum

Replace Tokio runtime

Adopt Event Bus
```

---

## File

Represents source files.

Examples:
```text
main.rs

event.rs

Cargo.toml

README.md
```

---

## Commit

Git commits.

Metadata:
```text
hash

branch

author

timestamp

message
```

---

## Build

Compilation.

Stores:
```text
Success

Failure

Duration

Errors
```

---

## Test

Test execution.

Stores:
```text
Passed

Failed

Coverage

Duration
```

---

## Issue

Bug reports.

Feature requests.

Tasks.

---

## Deployment

Release activity.

Examples:
```text
Docker

GitHub Release

Production

Development
```

---

## Memory

Semantic knowledge.

Examples:
```text
Pattern

Constraint

Lesson

Best Practice
```

---

# Relationship Types

Relationships carry meaning.

## created

```text
User  ───created───►  Prompt
```

---

## generated

```text
Prompt  ───generated───►  AI Response
```

---

## modified

```text
Response  ───modified───►  File
```

---

## committed

```text
File  ───committed_in───►  Commit
```

---

## implements

```text
Commit  ───implements───►  Issue
```

---

## resolves

```text
Commit  ───resolves───►  Bug
```

---

## depends_on

```text
Feature  ───depends_on───►  Decision
```

---

## derived_from

```text
Memory  ───derived_from───►  Events
```

---

## references

```text
Prompt  ───references───►  Decision
```

---

## reviewed_by

```text
Commit  ───reviewed_by───►  Gemini
```

---

## supersedes

```text
Decision  ───supersedes───►  Older Decision
```

---

## influenced_by

One of the most important edges.

```text
Architecture  ───influenced_by───►  Prompt
```

This preserves engineering reasoning.

---

# Graph Construction

Graph generation occurs continuously.

```text
Engineering Event

↓

Relationship Extractor

↓

Node Creation

↓

Edge Creation

↓

Knowledge Graph

↓

Search Index
```

The graph is never manually edited.

---

# Relationship Discovery

Relationships are discovered using multiple strategies.

## Direct Observation

Example:
```text
Commit  ───contains───►  File
```

Confidence: `1.0`

---

## Metadata

Example:
```text
Prompt  ───mentions───►  Cargo.toml
```

Confidence: `0.95`

---

## AI Extraction

Example:
```text
Prompt  ───influenced───►  Architecture
```

Confidence: `0.80` (Always marked as inferred)

---

## User Annotation

Example:
```text
Decision  ───caused───►  Performance Improvement
```

Confidence: `1.0` (User input overrides inference)

---

# Graph Evolution

Graphs never rewrite history.

Example: Instead of updating a decision node in-place:

```text
Decision V2  ───supersedes───►  Decision V1
```

Historical context remains intact.

---

# Cross-Project Graph

One of Matis's most powerful capabilities.

```text
Project A  ───►  Created Pattern  ───►  Semantic Memory  ───►  Project B (Suggested Automatically)
```

Experience compounds across repositories.

---

# Example Query

User asks:
> Why did we migrate to Axum?

Graph traversal:
```text
Decision  ──►  Evidence  ──►  Prompt  ──►  Discussion  ──►  Commit  ──►  Performance Benchmark  ──►  Deployment
```

Instead of searching text, Matis traverses relationships.

---

# AI Context Generation

The graph is the primary source for AI context.

Example request:
> Continue authentication work.

Returned context:
```text
Current Branch

Relevant Decisions

Related Files

Open Issues

Recent Prompts

Successful Implementations

Known Failures

Pending Tasks
```

This is dramatically more useful than replaying raw chat history.

---

# Graph Integrity Rules

The graph must satisfy these invariants:

1. Every node originates from one or more Engineering Events.
2. Every edge has a defined relationship type.
3. Relationships are directional.
4. Inferred edges include confidence scores.
5. User-confirmed edges override inferred edges.
6. Historical relationships are never deleted.
7. Every node is traceable back to source events.
8. Cycles are allowed only where semantically meaningful; provenance chains remain acyclic.
9. Graph reconstruction from immutable events must be deterministic.
10. Every graph query must be explainable by showing supporting events.

---

# Future Extensions

* Root cause analysis for regressions.
* Automatic Architecture Decision Record (ADR) generation.
* Dependency impact analysis before refactoring.
* Engineering timeline visualization.
* Team collaboration graphs.
* Cross-repository expertise mapping.
* AI agent specialization based on historical success.
* Recommendation engines for prompts, libraries, and architectural patterns.

---

# Architecture Decision Candidates

Before implementation, document these as ADRs:

* Should the Knowledge Graph be rebuilt entirely from immutable events, or incrementally updated?
* Which graph storage should be the default: relational tables, embedded graph structures, or a dedicated graph database?
* Should inferred relationships require explicit user confirmation before becoming semantic memory?
* How are deleted files, renamed repositories, and rewritten Git history represented without breaking provenance?
