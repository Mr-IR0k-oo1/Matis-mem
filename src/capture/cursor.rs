use super::{CaptureAdapter, EventBus};
use crate::core::{
    Actor, Event, EventKind, EventPayload, EventSource, Importance, ProjectId, ResponsePayload,
};
use crate::data::AgentLog;

pub struct CursorCapture;

impl CursorCapture {
    pub fn parse_log(log: &AgentLog) -> Option<Event> {
        if log.agent != "vibe" && log.agent != "cursor" {
            return None;
        }
        Some(
            Event::new(
                ProjectId::new(&log.project),
                Actor::AI {
                    name: "cursor".into(),
                    provider: Some("anysphere".into()),
                },
                EventSource::Shim("cursor".into()),
                EventKind::Response,
                Importance::Medium,
                EventPayload::Response(ResponsePayload {
                    agent: "cursor".into(),
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

impl CaptureAdapter for CursorCapture {
    fn name(&self) -> &'static str {
        "cursor"
    }

    fn observe(&self, _bus: &EventBus) {}
}
