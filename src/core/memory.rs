use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MemoryTier {
    Working,
    Episodic,
    Semantic,
}

impl std::fmt::Display for MemoryTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryTier::Working => write!(f, "Working"),
            MemoryTier::Episodic => write!(f, "Episodic"),
            MemoryTier::Semantic => write!(f, "Semantic"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tier: MemoryTier,
    pub project: String,
    pub tags: Vec<String>,
    pub importance_score: f32,
    pub created_at: String,
    pub updated_at: String,
    pub source_event_id: Option<String>,
}

impl MemoryItem {
    pub fn new(
        title: impl Into<String>,
        content: impl Into<String>,
        tier: MemoryTier,
        project: impl Into<String>,
    ) -> Self {
        let ts = chrono::Local::now().to_rfc3339();
        let id = format!("mem_{}", chrono::Local::now().format("%Y%m%d_%H%M%S_%3f"));
        Self {
            id,
            title: title.into(),
            content: content.into(),
            tier,
            project: project.into(),
            tags: Vec::new(),
            importance_score: 1.0,
            created_at: ts.clone(),
            updated_at: ts,
            source_event_id: None,
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_source_event(mut self, event_id: impl Into<String>) -> Self {
        self.source_event_id = Some(event_id.into());
        self
    }
}
