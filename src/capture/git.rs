use super::{CaptureAdapter, EventBus};
use crate::core::{
    Actor, CommitPayload, Event, EventKind, EventPayload, EventSource, Importance, ProjectId,
};

pub struct GitCapture;

impl GitCapture {
    pub fn create_commit_event(
        project: &str,
        hash: &str,
        branch: &str,
        message: &str,
        files: Vec<String>,
        insertions: usize,
        deletions: usize,
    ) -> Event {
        Event::new(
            ProjectId::new(project),
            Actor::Git,
            EventSource::GitLib,
            EventKind::Git,
            Importance::High,
            EventPayload::Commit(CommitPayload {
                hash: hash.to_string(),
                branch: branch.to_string(),
                message: message.to_string(),
                files_changed: files,
                insertions,
                deletions,
            }),
        )
    }
}

impl CaptureAdapter for GitCapture {
    fn name(&self) -> &'static str {
        "git"
    }

    fn observe(&self, _bus: &EventBus) {}
}
