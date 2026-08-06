# Engineering Episode Formal Behavioral Specification (`specs/core/engineering-episode.spec.md`)

## 1. Specification Status & Invariants

* **Specification Version**: 1.0.0
* **Status**: Normative / Core Specification
* **Target Crate**: `crates/matis-episodes`

### Core Invariants
1. An `EngineeringEpisode` represents a coherent unit of engineering work (*story of work*).
2. Episodes reference `EventId`, `MemoryId`, and `ProjectId` keys — episodes **NEVER** duplicate raw event payloads.
3. Every episode has a deterministic lifecycle (`Detected -> Growing -> Active -> Completed -> Distilled -> Archived`).
4. Replaying an `EventStore` MUST reconstruct identical episodes and parent/child relationship linkages.

---

## 2. Field Schema & Data Contract

```rust
pub struct EngineeringEpisodeSpec {
    pub id: EpisodeId,
    pub title: String,
    pub objective: String,
    pub project_id: ProjectId,
    pub repository_id: Option<RepositoryId>,
    pub participants: Vec<ActorId>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub status: EpisodeStatus,
    pub events: Vec<EventId>,
    pub decisions: Vec<EventId>,
    pub commits: Vec<String>,
    pub files: Vec<String>,
    pub memories: Vec<MemoryId>,
    pub outcome: EpisodeOutcome,
    pub lessons: Vec<String>,
    pub confidence: f32,
}
```

---

## 3. Episode State Machine

```text
  Branch Created / New Objective Prompt
                    │
                    ▼
           Status: Detected
                    │
                    ▼
    Events Attached ──► Status: Active / Growing
                    │
           ┌────────┴────────┐
           ▼                 ▼
  Work Paused/Blocked    PR Merged / Release Created
           │                 │
           ▼                 ▼
    Status: Blocked    Status: Completed
                             │
                             ▼
                    Status: Distilled (Knowledge Refinement Loop)
                             │
                             ▼
                    Status: Archived
```

---

## 4. Episode Detection & Grouping Rules

An event $E$ is attached to Episode $\mathcal{EP}$ if:
1. $E.\text{branch} == \mathcal{EP}.\text{branch}$, OR
2. $E.\text{files} \cap \mathcal{EP}.\text{files} \neq \emptyset$ within a 30-minute working session window, OR
3. $E.\text{parents}$ reference events already belonging to $\mathcal{EP}$.
