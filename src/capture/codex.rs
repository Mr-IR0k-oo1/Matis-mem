use super::{CaptureAdapter, EventBus};
use crate::core::{
    Actor, Event, EventKind, EventPayload, EventSource, Importance, ProjectId, ResponsePayload,
};
use crate::data::AgentLog;

pub struct CodexCapture;

impl CodexCapture {
    pub fn parse_log(log: &AgentLog) -> Option<Event> {
        if log.agent != "codex" && log.agent != "copilot" {
            return None;
        }
        Some(
            Event::new(
                ProjectId::new(&log.project),
                Actor::AI {
                    name: "codex".into(),
                    provider: Some("openai".into()),
                },
                EventSource::Shim("codex".into()),
                EventKind::Response,
                Importance::Medium,
                EventPayload::Response(ResponsePayload {
                    agent: "codex".into(),
                    prompt: log.input.clone(),
                    response: log.output.clone(),
                    duration_ms: log.duration_ms,
                    exit_code: log.exit_code,
                }),
            )
            .with_metadata("cwd", &log.cwd),
        )
    }
}

impl CaptureAdapter for CodexCapture {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn observe(&self, _bus: &EventBus) {}
}
