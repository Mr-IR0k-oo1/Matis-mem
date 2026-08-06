use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum NodeKind {
    Prompt { id: String },
    Commit { hash: String },
    File { path: String },
    Issue { id: String },
    Decision { id: String },
    AI { model: String },
    Person { name: String },
    Repository { name: String },
}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Prompt { id } => write!(f, "Prompt({})", id),
            NodeKind::Commit { hash } => write!(f, "Commit({})", &hash[..hash.len().min(7)]),
            NodeKind::File { path } => write!(f, "File({})", path),
            NodeKind::Issue { id } => write!(f, "Issue({})", id),
            NodeKind::Decision { id } => write!(f, "Decision({})", id),
            NodeKind::AI { model } => write!(f, "AI({})", model),
            NodeKind::Person { name } => write!(f, "Person({})", name),
            NodeKind::Repository { name } => write!(f, "Repo({})", name),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Generated,
    Modified,
    Reviewed,
    Fixed,
    Introduced,
    Merged,
    DerivedFrom,
    References,
    DependsOn,
}

impl std::fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeKind::Generated => write!(f, "generated"),
            EdgeKind::Modified => write!(f, "modified"),
            EdgeKind::Reviewed => write!(f, "reviewed"),
            EdgeKind::Fixed => write!(f, "fixed"),
            EdgeKind::Introduced => write!(f, "introduced"),
            EdgeKind::Merged => write!(f, "merged"),
            EdgeKind::DerivedFrom => write!(f, "derived_from"),
            EdgeKind::References => write!(f, "references"),
            EdgeKind::DependsOn => write!(f, "depends_on"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub kind: NodeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub kind: EdgeKind,
    pub weight: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoreGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl CoreGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: impl Into<String>, label: impl Into<String>, kind: NodeKind) -> String {
        let id_str = id.into();
        if !self.nodes.iter().any(|n| n.id == id_str) {
            self.nodes.push(Node {
                id: id_str.clone(),
                label: label.into(),
                kind,
            });
        }
        id_str
    }

    pub fn add_edge(&mut self, from: impl Into<String>, to: impl Into<String>, kind: EdgeKind) {
        let from_str = from.into();
        let to_str = to.into();
        if !self.edges.iter().any(|e| e.from == from_str && e.to == to_str && e.kind == kind) {
            self.edges.push(Edge {
                from: from_str,
                to: to_str,
                kind,
                weight: 1.0,
            });
        }
    }
}
