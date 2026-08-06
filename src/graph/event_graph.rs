use crate::core::{CoreGraph, EdgeKind, Event, EventPayload, NodeKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventGraph {
    pub graph: CoreGraph,
}

impl EventGraph {
    pub fn new() -> Self {
        Self {
            graph: CoreGraph::new(),
        }
    }

    pub fn build_from_events(&mut self, events: &[Event]) {
        for event in events {
            let event_node_id = format!("ev_{}", event.id);
            let actor_name = event.actor.to_string();
            let actor_node_id = format!("actor_{}", actor_name);

            // Add actor node
            self.graph.add_node(
                &actor_node_id,
                &actor_name,
                NodeKind::Person { name: actor_name.clone() },
            );

            // Add repo/project node
            let proj_name = event.project.as_str();
            let repo_node_id = format!("repo_{}", proj_name);
            self.graph.add_node(
                &repo_node_id,
                proj_name,
                NodeKind::Repository { name: proj_name.to_string() },
            );

            match &event.payload {
                EventPayload::Prompt(p) => {
                    self.graph.add_node(&event_node_id, &p.prompt, NodeKind::Prompt { id: event.id.to_string() });
                    self.graph.add_edge(&actor_node_id, &event_node_id, EdgeKind::Generated);
                    self.graph.add_edge(&event_node_id, &repo_node_id, EdgeKind::References);
                }
                EventPayload::Response(r) => {
                    let ai_node_id = format!("ai_{}", r.agent);
                    self.graph.add_node(&ai_node_id, &r.agent, NodeKind::AI { model: r.agent.clone() });
                    self.graph.add_node(&event_node_id, &r.prompt, NodeKind::Prompt { id: event.id.to_string() });
                    self.graph.add_edge(&ai_node_id, &event_node_id, EdgeKind::Generated);
                }
                EventPayload::Commit(c) => {
                    let commit_node_id = format!("commit_{}", c.hash);
                    self.graph.add_node(&commit_node_id, &c.message, NodeKind::Commit { hash: c.hash.clone() });
                    self.graph.add_edge(&actor_node_id, &commit_node_id, EdgeKind::Generated);
                    self.graph.add_edge(&commit_node_id, &repo_node_id, EdgeKind::Merged);

                    for file in &c.files_changed {
                        let file_node_id = format!("file_{}", file);
                        self.graph.add_node(&file_node_id, file, NodeKind::File { path: file.clone() });
                        self.graph.add_edge(&commit_node_id, &file_node_id, EdgeKind::Modified);
                    }
                }
                EventPayload::File(f) => {
                    let file_node_id = format!("file_{}", f.path);
                    self.graph.add_node(&file_node_id, &f.path, NodeKind::File { path: f.path.clone() });
                    self.graph.add_edge(&actor_node_id, &file_node_id, match f.action.as_str() {
                        "create" => EdgeKind::Generated,
                        _ => EdgeKind::Modified,
                    });
                }
                EventPayload::Decision(d) => {
                    let dec_node_id = format!("dec_{}", event.id);
                    self.graph.add_node(&dec_node_id, &d.title, NodeKind::Decision { id: event.id.to_string() });
                    self.graph.add_edge(&actor_node_id, &dec_node_id, EdgeKind::Generated);
                }
                _ => {}
            }
        }
    }
}
