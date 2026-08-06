use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskIntent {
    Continuation,
    Debugging,
    Optimization,
    KnowledgeRetrieval,
    FeatureDevelopment,
    Refactoring,
    General,
}

impl std::fmt::Display for TaskIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskIntent::Continuation => write!(f, "Continuation"),
            TaskIntent::Debugging => write!(f, "Debugging"),
            TaskIntent::Optimization => write!(f, "Optimization"),
            TaskIntent::KnowledgeRetrieval => write!(f, "KnowledgeRetrieval"),
            TaskIntent::FeatureDevelopment => write!(f, "FeatureDevelopment"),
            TaskIntent::Refactoring => write!(f, "Refactoring"),
            TaskIntent::General => write!(f, "General"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRequest {
    pub objective: String,
    pub project: String,
    pub repository: Option<String>,
    pub current_branch: Option<String>,
    pub current_files: Vec<String>,
    pub working_directory: Option<String>,
    pub requester: String,
    pub token_budget: usize,
    pub preferences: HashMap<String, String>,
}

impl ContextRequest {
    pub fn new(objective: impl Into<String>, project: impl Into<String>) -> Self {
        Self {
            objective: objective.into(),
            project: project.into(),
            repository: None,
            current_branch: None,
            current_files: Vec::new(),
            working_directory: None,
            requester: "user".into(),
            token_budget: 8000,
            preferences: HashMap::new(),
        }
    }

    pub fn with_token_budget(mut self, budget: usize) -> Self {
        self.token_budget = budget;
        self
    }

    pub fn with_branch(mut self, branch: impl Into<String>) -> Self {
        self.current_branch = Some(branch.into());
        self
    }

    pub fn with_files(mut self, files: Vec<String>) -> Self {
        self.current_files = files;
        self
    }
}
