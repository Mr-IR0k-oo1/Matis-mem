use super::event::Event;
use super::ids::ProjectId;
use super::importance::Importance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimelineFilter {
    pub project: Option<ProjectId>,
    pub min_importance: Option<Importance>,
    pub query: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct Timeline {
    events: Vec<Event>,
}

impl Timeline {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn from_events(events: Vec<Event>) -> Self {
        let mut t = Self { events };
        t.sort();
        t
    }

    pub fn add(&mut self, event: Event) {
        self.events.push(event);
        self.sort();
    }

    pub fn sort(&mut self) {
        self.events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn filter(&self, filter: &TimelineFilter) -> Vec<Event> {
        self.events
            .iter()
            .filter(|e| {
                if let Some(ref proj) = filter.project {
                    if &e.project != proj && proj.as_str() != "all" && !proj.as_str().is_empty() {
                        return false;
                    }
                }
                if let Some(ref min_imp) = filter.min_importance {
                    if e.importance < *min_imp {
                        return false;
                    }
                }
                if let Some(ref q) = filter.query {
                    let q_lower = q.to_lowercase();
                    let matches_summary = e.summary().to_lowercase().contains(&q_lower);
                    let matches_project = e.project.as_str().to_lowercase().contains(&q_lower);
                    if !matches_summary && !matches_project {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .take(filter.limit.unwrap_or(usize::MAX))
            .collect()
    }
}
