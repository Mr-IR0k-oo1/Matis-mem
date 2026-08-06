use anyhow::Result;
use crate::capture::claude::ClaudeCapture;
use crate::capture::codex::CodexCapture;
use crate::capture::cursor::CursorCapture;
use crate::capture::gemini::GeminiCapture;
use crate::capture::generic::GenericCapture;
use crate::core::{
    Actor, Event, EventKind, EventPayload, EventSource, Importance, ProjectId, ResponsePayload,
};
use crate::data::{AgentLog, Session};
use crate::storage::events::EventStore;

pub struct ArchiveMigrator;

impl ArchiveMigrator {
    pub fn migrate_legacy_data(store: &EventStore) -> Result<usize> {
        let mut count = 0;

        // 1. Ingest recent agent logs
        if let Ok(logs) = AgentLog::recent(500) {
            for log in logs {
                let event = ClaudeCapture::parse_log(&log)
                    .or_else(|| GeminiCapture::parse_log(&log))
                    .or_else(|| CursorCapture::parse_log(&log))
                    .or_else(|| CodexCapture::parse_log(&log))
                    .unwrap_or_else(|| GenericCapture::parse_log(&log));

                if store.append(&event).is_ok() {
                    count += 1;
                }
            }
        }

        // 2. Ingest legacy sessions
        let sessions_dir = crate::config::sessions_dir();
        if sessions_dir.exists() {
            if let Ok(projects) = std::fs::read_dir(&sessions_dir) {
                for p_entry in projects.filter_map(|e| e.ok()) {
                    if p_entry.path().is_dir() {
                        let project_name = p_entry.file_name().to_string_lossy().to_string();
                        if let Ok(sessions) = Session::last_n(&project_name, 100) {
                            for s in sessions {
                                let event = Event::new(
                                    ProjectId::new(&s.project),
                                    Actor::AI {
                                        name: s.model.clone(),
                                        provider: None,
                                    },
                                    EventSource::Cli,
                                    EventKind::Response,
                                    Importance::Medium,
                                    EventPayload::Response(ResponsePayload {
                                        agent: s.model,
                                        prompt: s.prompt,
                                        response: s.response,
                                        duration_ms: s.duration_ms,
                                        exit_code: 0,
                                    }),
                                );
                                if store.append(&event).is_ok() {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(count)
    }
}
