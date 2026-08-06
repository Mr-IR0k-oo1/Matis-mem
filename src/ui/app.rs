#![allow(dead_code)]
use anyhow::Result;
use std::sync::mpsc::Receiver;
use std::time::Instant;

use crate::core::{MemoryTier, Timeline};
use crate::data::{Knowledge, Project};
use crate::graph::EventGraph;
use crate::memory::{EpisodicMemory, MemoryPromotionEngine, SemanticMemory, WorkingMemory};
use crate::storage::EventStore;
use crate::watcher::{log_watcher, ShimStatus};



// ── Tabs ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Today,
    Timeline,
    Memory,
    Graph,
    Settings,
}

impl Tab {
    pub fn label(&self) -> &'static str {
        match self {
            Tab::Today => "[1] TODAY",
            Tab::Timeline => "[2] TIMELINE",
            Tab::Memory => "[3] MEMORY",
            Tab::Graph => "[4] GRAPH",
            Tab::Settings => "[5] SETTINGS",
        }
    }

    pub fn next(&self) -> Tab {
        match self {
            Tab::Today => Tab::Timeline,
            Tab::Timeline => Tab::Memory,
            Tab::Memory => Tab::Graph,
            Tab::Graph => Tab::Settings,
            Tab::Settings => Tab::Today,
        }
    }

    pub fn prev(&self) -> Tab {
        match self {
            Tab::Today => Tab::Settings,
            Tab::Timeline => Tab::Today,
            Tab::Memory => Tab::Timeline,
            Tab::Graph => Tab::Memory,
            Tab::Settings => Tab::Graph,
        }
    }
}

// ── Focus ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Focus {
    Projects,
    PromptInput,
    TimelineList,
    MemoryList,
    GraphView,
    ShimList,
}

// ── Popup ─────────────────────────────────────────────────────────────────────

pub enum Popup {
    None,
    NewProject {
        name_buf: String,
        goal_buf: String,
        field: usize,
    },
    AddMemory {
        title_buf: String,
        content_buf: String,
        tier: MemoryTier,
        error: Option<String>,
    },
    Confirm {
        message: String,
        on_yes: ConfirmAction,
    },
    Output {
        title: String,
        lines: Vec<String>,
        scroll: usize,
    },
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    DeleteProject(String),
    InstallShims,
    UninstallShims,
}

// ── App State ─────────────────────────────────────────────────────────────────

pub struct App {
    pub tab: Tab,
    pub focus: Focus,
    pub popup: Popup,

    // Projects
    pub projects: Vec<String>,
    pub project_idx: usize,
    pub active_project: Option<Project>,
    pub project_list_state: ratatui::widgets::ListState,

    // Engine & Stores
    pub event_store: EventStore,
    pub timeline: Timeline,
    pub timeline_idx: usize,
    pub timeline_list_state: ratatui::widgets::ListState,

    // Memory
    pub working_memory: WorkingMemory,
    pub episodic_memory: EpisodicMemory,
    pub semantic_memory: SemanticMemory,
    pub memory_idx: usize,
    pub memory_list_state: ratatui::widgets::ListState,

    // Graph
    pub event_graph: EventGraph,

    // Prompt & Input
    pub prompt: String,
    pub cursor: usize,

    // Shims & Watchers
    pub shim_statuses: Vec<ShimStatus>,
    pub shim_idx: usize,
    pub shim_list_state: ratatui::widgets::ListState,
    pub watch_rx: Option<Receiver<log_watcher::WatchEvent>>,

    // Knowledge
    pub knowledge_topics: Vec<String>,

    // Status
    pub status: Option<(String, bool, Instant)>,
    pub should_quit: bool,
}

fn make_list_state(idx: usize) -> ratatui::widgets::ListState {
    let mut s = ratatui::widgets::ListState::default();
    s.select(Some(idx));
    s
}

impl App {
    pub fn new() -> Result<Self> {
        let projects = Project::list().unwrap_or_default();
        let active_project = projects.first().and_then(|n| Project::load(n).ok());
        let project_list_state = make_list_state(0);

        let event_store = EventStore::new();
        // Migrate legacy logs into event store automatically
        let _ = crate::storage::ArchiveMigrator::migrate_legacy_data(&event_store);

        let events = event_store.read_all().unwrap_or_default();
        let timeline = Timeline::from_events(events.clone());
        let timeline_list_state = make_list_state(0);

        let mut working_memory = WorkingMemory::new();
        let mut episodic_memory = EpisodicMemory::new();
        let mut semantic_memory = SemanticMemory::new();

        for ev in &events {
            MemoryPromotionEngine::process_event(
                ev,
                &mut working_memory,
                &mut episodic_memory,
                &mut semantic_memory,
            );
        }
        let memory_list_state = make_list_state(0);

        let mut event_graph = EventGraph::new();
        event_graph.build_from_events(&events);

        let shim_statuses = crate::watcher::shim::status();
        let shim_list_state = make_list_state(0);

        let watch_rx = log_watcher::start().ok();
        let knowledge_topics = Knowledge::list().unwrap_or_default();

        Ok(Self {
            tab: Tab::Today,
            focus: Focus::PromptInput,
            popup: Popup::None,

            projects,
            project_idx: 0,
            active_project,
            project_list_state,

            event_store,
            timeline,
            timeline_idx: 0,
            timeline_list_state,

            working_memory,
            episodic_memory,
            semantic_memory,
            memory_idx: 0,
            memory_list_state,

            event_graph,

            prompt: String::new(),
            cursor: 0,

            shim_statuses,
            shim_idx: 0,
            shim_list_state,
            watch_rx,

            knowledge_topics,

            status: None,
            should_quit: false,
        })
    }

    pub fn tick(&mut self) {
        let mut new_logs = Vec::new();
        if let Some(ref rx) = self.watch_rx {
            while let Ok(evt) = rx.try_recv() {
                if let log_watcher::WatchEvent::NewLog(log) = evt {
                    new_logs.push(log);
                }
            }
        }

        for log in new_logs {
            let ev = crate::capture::generic::GenericCapture::parse_log(&log);
            let _ = self.event_store.append(&ev);
            self.timeline.add(ev.clone());
            MemoryPromotionEngine::process_event(
                &ev,
                &mut self.working_memory,
                &mut self.episodic_memory,
                &mut self.semantic_memory,
            );
            self.event_graph.build_from_events(&[ev]);
            self.set_status(&format!("Captured event from agent: {}", log.agent), false);
        }
    }

    pub fn set_status(&mut self, msg: &str, is_err: bool) {
        self.status = Some((msg.to_string(), is_err, Instant::now()));
    }

    pub fn refresh_projects(&mut self) {
        self.projects = Project::list().unwrap_or_default();
        if self.project_idx >= self.projects.len() && !self.projects.is_empty() {
            self.project_idx = self.projects.len() - 1;
        }
        self.active_project = self.projects.get(self.project_idx).and_then(|n| Project::load(n).ok());
        self.project_list_state.select(Some(self.project_idx));
    }

    pub fn active_project_name(&self) -> &str {
        self.active_project.as_ref().map(|p| p.name.as_str()).unwrap_or("global")
    }

    pub fn refresh_shims(&mut self) {
        self.shim_statuses = crate::watcher::shim::status();
    }
}
