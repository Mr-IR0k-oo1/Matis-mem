use crate::core::{MemoryItem, Timeline};
use crate::memory::{EpisodicMemory, SemanticMemory, WorkingMemory};
use super::retrieval_planner::RetrievalWeights;

#[derive(Debug, Clone)]
pub struct MemoryCandidate {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tier_name: String,
    pub weight: f32,
}

pub struct MemorySelector;

impl MemorySelector {
    pub fn select_candidates(
        working: &WorkingMemory,
        episodic: &EpisodicMemory,
        semantic: &SemanticMemory,
        timeline: &Timeline,
        weights: &RetrievalWeights,
        project: &str,
    ) -> Vec<MemoryCandidate> {
        let mut candidates = Vec::new();

        // 1. Working Memory
        for item in &working.items {
            if item.project == project || project == "all" || project.is_empty() {
                candidates.push(MemoryCandidate {
                    id: item.id.clone(),
                    title: item.title.clone(),
                    content: item.content.clone(),
                    tier_name: "Working Memory".into(),
                    weight: weights.working_memory_weight,
                });
            }
        }

        // 2. Episodic Memory
        for item in &episodic.items {
            if item.project == project || project == "all" || project.is_empty() {
                candidates.push(MemoryCandidate {
                    id: item.id.clone(),
                    title: item.title.clone(),
                    content: item.content.clone(),
                    tier_name: "Episodic Memory".into(),
                    weight: weights.episodic_memory_weight,
                });
            }
        }

        // 3. Semantic Memory
        for item in &semantic.items {
            if item.project == project || project == "all" || project.is_empty() {
                candidates.push(MemoryCandidate {
                    id: item.id.clone(),
                    title: item.title.clone(),
                    content: item.content.clone(),
                    tier_name: "Semantic Memory".into(),
                    weight: weights.semantic_memory_weight,
                });
            }
        }

        // 4. Timeline Event Summaries
        for ev in timeline.events().iter().take(10) {
            if ev.project.as_str() == project || project == "all" || project.is_empty() {
                candidates.push(MemoryCandidate {
                    id: ev.id.to_string(),
                    title: format!("Event: {}", ev.kind),
                    content: ev.summary(),
                    tier_name: "Timeline Event".into(),
                    weight: weights.timeline_weight,
                });
            }
        }

        candidates
    }
}
