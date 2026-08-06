use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "category", content = "data", rename_all = "snake_case")]
pub enum EventPayload {
    Prompt(PromptPayload),
    Response(ResponsePayload),
    Commit(CommitPayload),
    Decision(DecisionPayload),
    Shell(ShellPayload),
    Build(BuildPayload),
    File(FilePayload),
    Knowledge(KnowledgePayload),
    Generic(GenericPayload),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptPayload {
    pub prompt: String,
    pub cwd: String,
    pub terminal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponsePayload {
    pub agent: String,
    pub prompt: String,
    pub response: String,
    pub duration_ms: u64,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitPayload {
    pub hash: String,
    pub branch: String,
    pub message: String,
    pub files_changed: Vec<String>,
    pub insertions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecisionPayload {
    pub title: String,
    pub rationale: String,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShellPayload {
    pub command: String,
    pub success: bool,
    pub output: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildPayload {
    pub tool: String,
    pub success: bool,
    pub output: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FilePayload {
    pub path: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgePayload {
    pub topic: String,
    pub notes: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenericPayload {
    pub summary: String,
    pub details: String,
}
