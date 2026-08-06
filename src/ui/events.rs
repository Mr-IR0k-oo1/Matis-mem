use crossterm::event::{Event as CEvent, KeyCode, KeyEvent, KeyModifiers};

use crate::core::{Actor, Event, EventKind, EventPayload, EventSource, Importance};
use crate::data::Project;
use crate::memory::MemoryPromotionEngine;
use crate::ui::app::{App, ConfirmAction, Focus, Popup, Tab};

pub fn handle(event: &CEvent, app: &mut App) {
    if let CEvent::Key(key) = event {
        if handle_popup(key, app) {
            return;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            app.should_quit = true;
            return;
        }

        match key.code {
            KeyCode::Char('q') if app.focus != Focus::PromptInput => app.should_quit = true,
            KeyCode::Char('1') => app.tab = Tab::Today,
            KeyCode::Char('2') => app.tab = Tab::Timeline,
            KeyCode::Char('3') => app.tab = Tab::Memory,
            KeyCode::Char('4') => app.tab = Tab::Graph,
            KeyCode::Char('5') => app.tab = Tab::Settings,
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    app.tab = app.tab.prev();
                } else {
                    app.tab = app.tab.next();
                }
            }
            _ => match app.tab {
                Tab::Today => handle_today_keys(key, app),
                Tab::Timeline => handle_timeline_keys(key, app),
                Tab::Memory => handle_memory_keys(key, app),
                Tab::Graph => handle_graph_keys(key, app),
                Tab::Settings => handle_settings_keys(key, app),
            },
        }
    }
}

fn handle_today_keys(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Up => {
            if app.project_idx > 0 {
                app.project_idx -= 1;
                app.refresh_projects();
            }
        }
        KeyCode::Down => {
            if !app.projects.is_empty() && app.project_idx < app.projects.len() - 1 {
                app.project_idx += 1;
                app.refresh_projects();
            }
        }
        KeyCode::Enter => {
            if !app.prompt.trim().is_empty() {
                let prompt_text = app.prompt.trim().to_string();
                app.prompt.clear();

                let event = Event::new(
                    crate::core::ProjectId::new(app.active_project_name()),
                    Actor::User,
                    EventSource::Cli,
                    EventKind::Prompt,
                    Importance::Medium,
                    EventPayload::Prompt(crate::core::PromptPayload {
                        prompt: prompt_text,
                        cwd: std::env::current_dir()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                        terminal: None,
                    }),
                );

                let _ = app.event_store.append(&event);
                app.timeline.add(event.clone());
                MemoryPromotionEngine::process_event(
                    &event,
                    &mut app.working_memory,
                    &mut app.episodic_memory,
                    &mut app.semantic_memory,
                );
                app.event_graph.build_from_events(&[event]);
                app.set_status("Event captured and added to Timeline & Memory", false);
            }
        }
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.popup = Popup::NewProject {
                name_buf: String::new(),
                goal_buf: String::new(),
                field: 0,
            };
        }
        KeyCode::Char(c) => {
            app.prompt.push(c);
        }
        KeyCode::Backspace => {
            app.prompt.pop();
        }
        _ => {}
    }
}

fn handle_timeline_keys(key: &KeyEvent, app: &mut App) {
    let len = app.timeline.events().len();
    match key.code {
        KeyCode::Up => {
            if app.timeline_idx > 0 {
                app.timeline_idx -= 1;
            }
        }
        KeyCode::Down => {
            if len > 0 && app.timeline_idx < len - 1 {
                app.timeline_idx += 1;
            }
        }
        _ => {}
    }
}

fn handle_memory_keys(key: &KeyEvent, app: &mut App) {
    let len = app.episodic_memory.items.len();
    match key.code {
        KeyCode::Up => {
            if app.memory_idx > 0 {
                app.memory_idx -= 1;
            }
        }
        KeyCode::Down => {
            if len > 0 && app.memory_idx < len - 1 {
                app.memory_idx += 1;
            }
        }
        _ => {}
    }
}

fn handle_graph_keys(_key: &KeyEvent, _app: &mut App) {}

fn handle_settings_keys(key: &KeyEvent, app: &mut App) {
    let len = app.shim_statuses.len();
    match key.code {
        KeyCode::Up => {
            if app.shim_idx > 0 {
                app.shim_idx -= 1;
            }
        }
        KeyCode::Down => {
            if len > 0 && app.shim_idx < len - 1 {
                app.shim_idx += 1;
            }
        }
        KeyCode::Char('i') => {
            app.popup = Popup::Confirm {
                message: "Install all passive capture shims?".into(),
                on_yes: ConfirmAction::InstallShims,
            };
        }
        KeyCode::Char('u') => {
            app.popup = Popup::Confirm {
                message: "Uninstall all passive capture shims?".into(),
                on_yes: ConfirmAction::UninstallShims,
            };
        }
        _ => {}
    }
}

fn handle_popup(key: &KeyEvent, app: &mut App) -> bool {
    match app.popup {
        Popup::None => false,
        Popup::Confirm { ref on_yes, .. } => {
            let action = on_yes.clone();
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    app.popup = Popup::None;
                    match action {
                        ConfirmAction::InstallShims => {
                            if let Ok((i, a, _)) = crate::watcher::shim::install_all() {
                                app.set_status(&format!("Installed {} shims ({} existing)", i, a), false);
                                app.refresh_shims();
                            }
                        }
                        ConfirmAction::UninstallShims => {
                            if let Ok(u) = crate::watcher::shim::uninstall_all() {
                                app.set_status(&format!("Uninstalled {} shims", u), false);
                                app.refresh_shims();
                            }
                        }
                        ConfirmAction::DeleteProject(name) => {
                            let _ = Project::delete(&name);
                            app.refresh_projects();
                        }
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    app.popup = Popup::None;
                }
                _ => {}
            }
            true
        }
        Popup::NewProject {
            ref mut name_buf,
            ref mut goal_buf,
            ref mut field,
        } => {
            match key.code {
                KeyCode::Tab => *field = (*field + 1) % 2,
                KeyCode::Enter => {
                    if !name_buf.trim().is_empty() {
                        let p = Project::new(name_buf.trim(), goal_buf.trim());
                        let _ = p.save();
                        app.refresh_projects();
                        app.set_status(&format!("Created project: {}", p.name), false);
                    }
                    app.popup = Popup::None;
                }
                KeyCode::Esc => app.popup = Popup::None,
                KeyCode::Char(c) => {
                    if *field == 0 {
                        name_buf.push(c);
                    } else {
                        goal_buf.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if *field == 0 {
                        name_buf.pop();
                    } else {
                        goal_buf.pop();
                    }
                }
                _ => {}
            }
            true
        }
        _ => {
            if key.code == KeyCode::Esc {
                app.popup = Popup::None;
                true
            } else {
                false
            }
        }
    }
}
