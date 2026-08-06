use crate::core::{MemoryItem, MemoryTier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub items: Vec<MemoryItem>,
}

impl EpisodicMemory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, mut item: MemoryItem) {
        item.tier = MemoryTier::Episodic;
        self.items.push(item);
    }
}
