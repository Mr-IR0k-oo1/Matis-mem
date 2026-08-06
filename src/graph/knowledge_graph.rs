use crate::core::{CoreGraph, EdgeKind, NodeKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub graph: CoreGraph,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            graph: CoreGraph::new(),
        }
    }

    pub fn link_decision_to_file(&mut self, decision_id: &str, decision_title: &str, file_path: &str) {
        let dec_id = format!("dec_{}", decision_id);
        let file_id = format!("file_{}", file_path);

        self.graph.add_node(&dec_id, decision_title, NodeKind::Decision { id: decision_id.to_string() });
        self.graph.add_node(&file_id, file_path, NodeKind::File { path: file_path.to_string() });
        self.graph.add_edge(&dec_id, &file_id, EdgeKind::References);
    }
}
