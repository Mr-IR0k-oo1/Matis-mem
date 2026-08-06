use super::request::TaskIntent;

#[derive(Debug, Clone)]
pub struct RetrievalWeights {
    pub working_memory_weight: f32,
    pub episodic_memory_weight: f32,
    pub semantic_memory_weight: f32,
    pub graph_weight: f32,
    pub timeline_weight: f32,
}

pub struct RetrievalPlanner;

impl RetrievalPlanner {
    pub fn plan(intent: TaskIntent) -> RetrievalWeights {
        match intent {
            TaskIntent::Continuation => RetrievalWeights {
                working_memory_weight: 0.9,
                episodic_memory_weight: 0.8,
                semantic_memory_weight: 0.6,
                graph_weight: 0.5,
                timeline_weight: 0.7,
            },
            TaskIntent::Debugging => RetrievalWeights {
                working_memory_weight: 0.8,
                episodic_memory_weight: 0.9,
                semantic_memory_weight: 0.5,
                graph_weight: 0.8,
                timeline_weight: 0.9,
            },
            TaskIntent::Optimization => RetrievalWeights {
                working_memory_weight: 0.6,
                episodic_memory_weight: 0.7,
                semantic_memory_weight: 0.9,
                graph_weight: 0.8,
                timeline_weight: 0.5,
            },
            TaskIntent::KnowledgeRetrieval => RetrievalWeights {
                working_memory_weight: 0.4,
                episodic_memory_weight: 0.5,
                semantic_memory_weight: 1.0,
                graph_weight: 0.9,
                timeline_weight: 0.4,
            },
            TaskIntent::FeatureDevelopment => RetrievalWeights {
                working_memory_weight: 0.8,
                episodic_memory_weight: 0.6,
                semantic_memory_weight: 0.8,
                graph_weight: 0.7,
                timeline_weight: 0.5,
            },
            TaskIntent::Refactoring => RetrievalWeights {
                working_memory_weight: 0.7,
                episodic_memory_weight: 0.6,
                semantic_memory_weight: 0.9,
                graph_weight: 1.0,
                timeline_weight: 0.4,
            },
            TaskIntent::General => RetrievalWeights {
                working_memory_weight: 0.7,
                episodic_memory_weight: 0.7,
                semantic_memory_weight: 0.7,
                graph_weight: 0.7,
                timeline_weight: 0.7,
            },
        }
    }
}
