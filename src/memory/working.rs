use crate::core::{MemoryItem, MemoryTier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkingMemory {
    pub items: Vec<MemoryItem>,
}

impl WorkingMemory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, title: impl Into<String>, content: impl Into<String>, project: impl Into<String>) -> MemoryItem {
        let item = MemoryItem::new(title, content, MemoryTier::Working, project);
        self.items.push(item.clone());
        item
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
