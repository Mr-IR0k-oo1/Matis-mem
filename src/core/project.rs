use super::ids::ProjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoreProject {
    pub id: ProjectId,
    pub name: String,
    pub goal: String,
    pub constraints: Vec<String>,
    pub decisions: Vec<String>,
    pub notes: String,
}

impl CoreProject {
    pub fn new(name: impl Into<String>, goal: impl Into<String>) -> Self {
        let name_str = name.into();
        Self {
            id: ProjectId::new(&name_str),
            name: name_str,
            goal: goal.into(),
            constraints: Vec::new(),
            decisions: Vec::new(),
            notes: String::new(),
        }
    }
}
