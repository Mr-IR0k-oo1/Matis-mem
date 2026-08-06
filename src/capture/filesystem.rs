use super::{CaptureAdapter, EventBus};
use crate::core::{
    Actor, Event, EventKind, EventPayload, EventSource, FilePayload, Importance, ProjectId,
};

pub struct FilesystemCapture;

impl FilesystemCapture {
    pub fn create_file_event(project: &str, path: &str, action: &str) -> Event {
        Event::new(
            ProjectId::new(project),
            Actor::Filesystem,
            EventSource::Watcher("filesystem".into()),
            EventKind::Filesystem,
            Importance::Low,
            EventPayload::File(FilePayload {
                path: path.to_string(),
                action: action.to_string(),
            }),
        )
    }
}

impl CaptureAdapter for FilesystemCapture {
    fn name(&self) -> &'static str {
        "filesystem"
    }

    fn observe(&self, _bus: &EventBus) {}
}
