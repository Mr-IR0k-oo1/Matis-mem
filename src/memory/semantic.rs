use crate::core::{MemoryItem, MemoryTier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SemanticMemory {
    pub items: Vec<MemoryItem>,
}

impl SemanticMemory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn store(&mut self, mut item: MemoryItem) {
        item.tier = MemoryTier::Semantic;
        self.items.push(item);
    }
}
