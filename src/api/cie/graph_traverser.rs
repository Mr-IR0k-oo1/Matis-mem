use crate::graph::{DependencyGraph, EventGraph, KnowledgeGraph};

#[derive(Debug, Clone)]
pub struct GraphContextCandidate {
    pub id: String,
    pub title: String,
    pub detail: String,
    pub relationship: String,
}

pub struct GraphTraverser;

impl GraphTraverser {
    pub fn traverse(
        event_graph: &EventGraph,
        _dep_graph: &DependencyGraph,
        _knowledge_graph: &KnowledgeGraph,
        current_files: &[String],
    ) -> Vec<GraphContextCandidate> {
        let mut candidates = Vec::new();

        for node in &event_graph.graph.nodes {
            for file in current_files {
                if node.label.contains(file) || file.contains(&node.label) {
                    candidates.push(GraphContextCandidate {
                        id: node.id.clone(),
                        title: format!("Graph Node: {}", node.label),
                        detail: format!("Kind: {}", node.kind),
                        relationship: "Active file connection".into(),
                    });
                }
            }
        }

        candidates
    }
}
