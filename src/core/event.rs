use super::actor::Actor;
use super::event_kind::EventKind;
use super::ids::{EventId, ProjectId};
use super::importance::Importance;
use super::metadata::EventMetadata;
use super::payload::EventPayload;
use super::source::EventSource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Event {
    pub id: EventId,
    pub timestamp: String,
    pub actor: Actor,
    pub source: EventSource,
    pub project: ProjectId,
    pub kind: EventKind,
    #[serde(default)]
    pub parents: Vec<EventId>,
    pub payload: EventPayload,
    #[serde(default)]
    pub metadata: EventMetadata,
    pub importance: Importance,
}

impl Event {
    pub fn new(
        project: ProjectId,
        actor: Actor,
        source: EventSource,
        kind: EventKind,
        importance: Importance,
        payload: EventPayload,
    ) -> Self {
        let timestamp = chrono::Local::now().to_rfc3339();
        Self {
            id: EventId::new(),
            timestamp,
            actor,
            source,
            project,
            kind,
            parents: Vec::new(),
            payload,
            metadata: EventMetadata::new(),
            importance,
        }
    }

    pub fn with_parents(mut self, parents: Vec<EventId>) -> Self {
        self.parents = parents;
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.set(key, value);
        self
    }

    pub fn summary(&self) -> String {
        match &self.payload {
            EventPayload::Prompt(p) => format!("Prompt: {}", truncate(&p.prompt, 60)),
            EventPayload::Response(r) => format!("{} observed: {}", r.agent, truncate(&r.prompt, 50)),
            EventPayload::Commit(c) => format!("Commit {}: {}", &c.hash[..c.hash.len().min(7)], truncate(&c.message, 50)),
            EventPayload::Decision(d) => format!("Decision: {}", d.title),
            EventPayload::Shell(s) => format!("Shell ({})", truncate(&s.command, 40)),
            EventPayload::Build(b) => format!("Build {} ({})", if b.success { "passed" } else { "failed" }, b.tool),
            EventPayload::File(f) => format!("File {}: {}", f.action, f.path),
            EventPayload::Knowledge(k) => format!("Knowledge: {}", k.topic),
            EventPayload::Generic(g) => g.summary.clone(),
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max])
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::payload::PromptPayload;

    #[test]
    fn test_event_domain_model() {
        let ev = Event::new(
            ProjectId::new("test_project"),
            Actor::User,
            EventSource::Cli,
            EventKind::Prompt,
            Importance::High,
            EventPayload::Prompt(PromptPayload {
                prompt: "Refactor core model".into(),
                cwd: "/tmp".into(),
                terminal: None,
            }),
        );

        assert_eq!(ev.project.as_str(), "test_project");
        assert_eq!(ev.importance, Importance::High);
        assert!(ev.summary().contains("Refactor core model"));

        let json = serde_json::to_string(&ev).expect("serialization");
        let decoded: Event = serde_json::from_str(&json).expect("deserialization");
        assert_eq!(decoded.id, ev.id);
    }
}
