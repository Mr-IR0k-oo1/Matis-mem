use super::{CaptureAdapter, EventBus};
use crate::core::{
    Actor, BuildPayload, Event, EventKind, EventPayload, EventSource, Importance, ProjectId,
};

pub struct ShellCapture;

impl ShellCapture {
    pub fn create_shell_event(
        project: &str,
        command: &str,
        cwd: &str,
        output: &str,
        success: bool,
        duration_ms: u64,
    ) -> Event {
        let importance = if success {
            Importance::Medium
        } else {
            Importance::High
        };

        Event::new(
            ProjectId::new(project),
            Actor::Shell,
            EventSource::ShellSession,
            EventKind::Build,
            importance,
            EventPayload::Build(BuildPayload {
                tool: command.to_string(),
                success,
                output: output.to_string(),
                duration_ms,
            }),
        )
        .with_metadata("cwd", cwd)
    }
}

impl CaptureAdapter for ShellCapture {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn observe(&self, _bus: &EventBus) {}
}
