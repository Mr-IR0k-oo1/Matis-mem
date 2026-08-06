use crate::core::{CoreGraph, EdgeKind, NodeKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub graph: CoreGraph,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: CoreGraph::new(),
        }
    }

    pub fn add_file_dependency(&mut self, source_file: &str, target_file: &str) {
        let src_id = format!("file_{}", source_file);
        let tgt_id = format!("file_{}", target_file);

        self.graph.add_node(&src_id, source_file, NodeKind::File { path: source_file.to_string() });
        self.graph.add_node(&tgt_id, target_file, NodeKind::File { path: target_file.to_string() });
        self.graph.add_edge(&src_id, &tgt_id, EdgeKind::DependsOn);
    }
}
