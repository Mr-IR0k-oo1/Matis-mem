use super::{CaptureAdapter, EventBus};
use crate::core::{
    Actor, Event, EventKind, EventPayload, EventSource, Importance, ProjectId, ResponsePayload,
};
use crate::data::AgentLog;

pub struct GeminiCapture;

impl GeminiCapture {
    pub fn parse_log(log: &AgentLog) -> Option<Event> {
        if log.agent != "gemini" && log.agent != "gemini-cli" {
            return None;
        }
        Some(
            Event::new(
                ProjectId::new(&log.project),
                Actor::AI {
                    name: "gemini".into(),
                    provider: Some("google".into()),
                },
                EventSource::Shim("gemini".into()),
                EventKind::Response,
                Importance::Medium,
                EventPayload::Response(ResponsePayload {
                    agent: "gemini".into(),
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

impl CaptureAdapter for GeminiCapture {
    fn name(&self) -> &'static str {
        "gemini"
    }

    fn observe(&self, _bus: &EventBus) {}
}
