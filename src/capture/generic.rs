use super::{CaptureAdapter, EventBus};
use crate::core::{
    Actor, Event, EventKind, EventPayload, EventSource, Importance, ProjectId, ResponsePayload,
};
use crate::data::AgentLog;

pub struct GenericCapture;

impl GenericCapture {
    pub fn parse_log(log: &AgentLog) -> Event {
        Event::new(
            ProjectId::new(&log.project),
            Actor::AI {
                name: log.agent.clone(),
                provider: None,
            },
            EventSource::Shim(log.agent.clone()),
            EventKind::Response,
            Importance::Medium,
            EventPayload::Response(ResponsePayload {
                agent: log.agent.clone(),
                prompt: log.input.clone(),
                response: log.output.clone(),
                duration_ms: log.duration_ms,
                exit_code: log.exit_code,
            }),
        )
        .with_metadata("cwd", &log.cwd)
        .with_metadata("args", &log.args)
    }
}

impl CaptureAdapter for GenericCapture {
    fn name(&self) -> &'static str {
        "generic"
    }

    fn observe(&self, _bus: &EventBus) {}
}
