use crate::core::Event;
use crate::storage::EventStore;
use anyhow::Result;

pub struct QueryEngine {
    store: EventStore,
}

impl QueryEngine {
    pub fn new(store: EventStore) -> Self {
        Self { store }
    }

    pub fn query_events(&self, query: &str) -> Result<Vec<Event>> {
        let events = self.store.read_all()?;
        let q_lower = query.to_lowercase();
        let matched = events
            .into_iter()
            .filter(|e| {
                e.summary().to_lowercase().contains(&q_lower)
                    || e.project.as_str().to_lowercase().contains(&q_lower)
            })
            .collect();
        Ok(matched)
    }
}
