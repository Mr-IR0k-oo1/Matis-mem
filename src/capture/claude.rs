use super::{CaptureAdapter, EventBus};
use crate::core::{
    Actor, Event, EventKind, EventPayload, EventSource, Importance, ProjectId, ResponsePayload,
};
use crate::data::AgentLog;

pub struct ClaudeCapture;

impl ClaudeCapture {
    pub fn parse_log(log: &AgentLog) -> Option<Event> {
        if log.agent != "claude" {
            return None;
        }
        Some(
            Event::new(
                ProjectId::new(&log.project),
                Actor::AI {
                    name: "claude".into(),
                    provider: Some("anthropic".into()),
                },
                EventSource::Shim("claude".into()),
                EventKind::Response,
                Importance::Medium,
                EventPayload::Response(ResponsePayload {
                    agent: "claude".into(),
                    prompt: log.input.clone(),
                    response: log.output.clone(),
                    duration_ms: log.duration_ms,
                    exit_code: log.exit_code,
                }),
            )
            .with_metadata("cwd", &log.cwd)
            .with_metadata("args", &log.args),
        )
    }
}

impl CaptureAdapter for ClaudeCapture {
    fn name(&self) -> &'static str {
        "claude"
    }

    fn observe(&self, _bus: &EventBus) {}
}
