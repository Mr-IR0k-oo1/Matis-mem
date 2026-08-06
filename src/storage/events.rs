use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::config::data_dir;
use crate::core::Event;

pub fn events_dir() -> PathBuf {
    data_dir().join("events")
}

#[derive(Clone)]
pub struct EventStore {
    base_dir: PathBuf,
    lock: Arc<Mutex<()>>,
}

impl EventStore {
    pub fn new() -> Self {
        Self {
            base_dir: events_dir(),
            lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn with_dir(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            lock: Arc::new(Mutex::new(())),
        }
    }

    fn daily_file(&self, iso_ts: &str) -> PathBuf {
        let dt = DateTime::parse_from_rfc3339(iso_ts)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let year = dt.format("%Y").to_string();
        let month = dt.format("%m").to_string();
        let day = dt.format("%d").to_string();

        self.base_dir.join(year).join(month).join(day).join("events.jsonl")
    }

    pub fn append(&self, event: &Event) -> Result<()> {
        let _guard = self.lock.lock().unwrap();
        let file_path = self.daily_file(&event.timestamp);
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)
            .with_context(|| format!("failed to open event log: {}", file_path.display()))?;

        let line = serde_json::to_string(event)?;
        writeln!(file, "{}", line)?;
        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        if !self.base_dir.exists() {
            return Ok(events);
        }

        self.visit_dir(&self.base_dir, &mut events)?;
        events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(events)
    }

    fn visit_dir(&self, dir: &PathBuf, events: &mut Vec<Event>) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)?.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                self.visit_dir(&path, events)?;
            } else if path.file_name().and_then(|s| s.to_str()) == Some("events.jsonl") {
                let file = std::fs::File::open(&path)?;
                let reader = BufReader::new(file);
                for line in reader.lines().filter_map(|l| l.ok()) {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if let Ok(ev) = serde_json::from_str::<Event>(&line) {
                        events.push(ev);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Actor, EventKind, EventPayload, EventSource, Importance};

    #[test]
    fn test_event_store_append_and_read() {
        let temp_dir = std::env::temp_dir().join(format!("matis_test_events_{}", chrono::Local::now().timestamp_subsec_nanos()));
        let store = EventStore::with_dir(temp_dir.clone());

        let ev1 = Event::new(
            crate::core::ProjectId::new("proj_a"),
            Actor::User,
            EventSource::Cli,
            EventKind::Prompt,
            Importance::Medium,
            EventPayload::Prompt(crate::core::PromptPayload {
                prompt: "First prompt".into(),
                cwd: "/home".into(),
                terminal: None,
            }),
        );

        let ev2 = Event::new(
            crate::core::ProjectId::new("proj_a"),
            Actor::Git,
            EventSource::GitLib,
            EventKind::Git,
            Importance::High,
            EventPayload::Commit(crate::core::CommitPayload {
                hash: "abc123456789".into(),
                branch: "main".into(),
                message: "Initial commit".into(),
                files_changed: vec!["src/main.rs".into()],
                insertions: 10,
                deletions: 0,
            }),
        );

        store.append(&ev1).expect("append ev1 failed");
        store.append(&ev2).expect("append ev2 failed");

        let read_events = store.read_all().expect("read_all failed");
        assert_eq!(read_events.len(), 2);

        let _ = std::fs::remove_dir_all(temp_dir);
    }
}

