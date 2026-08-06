use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::core::EventKind;
use crate::ui::app::{App, Focus, Popup, Tab};
use crate::ui::theme;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header & Tabs
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Footer / Status bar
        ])
        .split(f.size());

    render_header(f, app, chunks[0]);

    match app.tab {
        Tab::Today => render_today_tab(f, app, chunks[1]),
        Tab::Timeline => render_timeline_tab(f, app, chunks[1]),
        Tab::Memory => render_memory_tab(f, app, chunks[1]),
        Tab::Graph => render_graph_tab(f, app, chunks[1]),
        Tab::Settings => render_settings_tab(f, app, chunks[1]),
    }

    render_footer(f, app, chunks[2]);
    render_popups(f, app);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(0)])
        .split(area);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border(false))
        .style(Style::default().bg(theme::BG));
    let title = Paragraph::new(Span::styled(" MATIS-MEM v0.3 ", theme::accent())).block(title_block);
    f.render_widget(title, header_chunks[0]);

    let tabs = vec![
        Tab::Today.label(),
        Tab::Timeline.label(),
        Tab::Memory.label(),
        Tab::Graph.label(),
        Tab::Settings.label(),
    ];

    let tab_idx = match app.tab {
        Tab::Today => 0,
        Tab::Timeline => 1,
        Tab::Memory => 2,
        Tab::Graph => 3,
        Tab::Settings => 4,
    };

    let tabs_widget = Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL).border_style(theme::border(false)))
        .highlight_style(Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD))
        .select(tab_idx);

    f.render_widget(tabs_widget, header_chunks[1]);
}

fn render_today_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(26), Constraint::Min(0)])
        .split(area);

    // Left: Projects List
    let p_items: Vec<ListItem> = app
        .projects
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let style = if i == app.project_idx {
                theme::selected()
            } else {
                theme::normal()
            };
            ListItem::new(format!(" {} ", name)).style(style)
        })
        .collect();

    let p_list = List::new(p_items)
        .block(
            Block::default()
                .title(" [1] Projects ")
                .borders(Borders::ALL)
                .border_style(theme::border(app.focus == Focus::Projects)),
        );
    f.render_widget(p_list, chunks[0]);

    // Right: Today's Dashboard & Prompt
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)])
        .split(chunks[1]);

    // Project Info
    let p_info = if let Some(ref p) = app.active_project {
        format!("Project: {}\nGoal: {}\nNotes: {}", p.name, p.goal, p.notes)
    } else {
        "No active project selected.".to_string()
    };

    let info_block = Paragraph::new(p_info)
        .block(Block::default().title(" Active Project ").borders(Borders::ALL).border_style(theme::border(false)))
        .wrap(Wrap { trim: true });
    f.render_widget(info_block, right_chunks[0]);

    // Prompt Box & Activity
    let prompt_block = Paragraph::new(app.prompt.as_str())
        .block(
            Block::default()
                .title(" Capture Prompt / Note (Press Enter) ")
                .borders(Borders::ALL)
                .border_style(theme::border(app.focus == Focus::PromptInput)),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(prompt_block, right_chunks[1]);
}

fn render_timeline_tab(f: &mut Frame, app: &App, area: Rect) {
    let events = app.timeline.events();
    let items: Vec<ListItem> = events
        .iter()
        .enumerate()
        .map(|(i, ev)| {
            let style = if i == app.timeline_idx {
                theme::selected()
            } else {
                theme::normal()
            };
            let prefix = match ev.kind {
                EventKind::Prompt => "PROMPT",
                EventKind::Response => "AI_OBS",
                EventKind::Git => "GIT   ",
                EventKind::Filesystem => "FILE  ",
                EventKind::Build => "BUILD ",
                EventKind::Decision => "DECIDE",
                EventKind::Shell => "SHELL ",
                EventKind::Test => "TEST  ",
                EventKind::Deployment => "DEPLOY",
                EventKind::Issue => "ISSUE ",
                EventKind::Knowledge => "KNOW  ",
                EventKind::Memory => "MEM   ",
                EventKind::System => "SYS   ",
            };
            let line = format!(" [{}] {} | [{}] {}", &ev.timestamp[..16], prefix, ev.project, ev.summary());
            ListItem::new(line).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Engineering Event Timeline ")
                .borders(Borders::ALL)
                .border_style(theme::border(app.focus == Focus::TimelineList)),
        );
    f.render_widget(list, area);
}

fn render_memory_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
        .split(area);

    // Working Memory
    let working_items: Vec<ListItem> = app
        .working_memory
        .items
        .iter()
        .map(|item| ListItem::new(format!("• {}", item.title)))
        .collect();
    let w_list = List::new(working_items)
        .block(Block::default().title(" Working Memory ").borders(Borders::ALL).border_style(theme::border(false)));
    f.render_widget(w_list, chunks[0]);

    // Episodic Memory
    let episodic_items: Vec<ListItem> = app
        .episodic_memory
        .items
        .iter()
        .map(|item| ListItem::new(format!("• {}", item.title)))
        .collect();
    let e_list = List::new(episodic_items)
        .block(Block::default().title(" Episodic Memory ").borders(Borders::ALL).border_style(theme::border(false)));
    f.render_widget(e_list, chunks[1]);

    // Semantic Memory
    let semantic_items: Vec<ListItem> = app
        .semantic_memory
        .items
        .iter()
        .map(|item| ListItem::new(format!("• {}", item.title)))
        .collect();
    let s_list = List::new(semantic_items)
        .block(Block::default().title(" Semantic Memory ").borders(Borders::ALL).border_style(theme::border(false)));
    f.render_widget(s_list, chunks[2]);
}

fn render_graph_tab(f: &mut Frame, app: &App, area: Rect) {
    let nodes_count = app.event_graph.graph.nodes.len();
    let edges_count = app.event_graph.graph.edges.len();

    let text = format!(
        "Event & Knowledge Graph Engine\n\nNodes tracked: {}\nEdges tracked: {}\n\nNode samples:\n{}",
        nodes_count,
        edges_count,
        app.event_graph
            .graph
            .nodes
            .iter()
            .take(10)
            .map(|n| format!("- [{}] {}", n.kind, n.label))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let block = Paragraph::new(text)
        .block(
            Block::default()
                .title(" Graph Engine Inspector ")
                .borders(Borders::ALL)
                .border_style(theme::border(app.focus == Focus::GraphView)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(block, area);
}

fn render_settings_tab(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .shim_statuses
        .iter()
        .map(|s| {
            let status_str = if s.installed { "INSTALLED" } else { "NOT INSTALLED" };
            let line = format!(" {:<12} | {:<14} | Real bin exists: {}", s.name, status_str, s.real_exists);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Passive Capture Shims & Daemon Settings ")
                .borders(Borders::ALL)
                .border_style(theme::border(app.focus == Focus::ShimList)),
        );
    f.render_widget(list, area);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let status_text = if let Some((ref msg, is_err, _)) = app.status {
        let style = if is_err { theme::err() } else { theme::ok() };
        Span::styled(format!(" Status: {} ", msg), style)
    } else {
        Span::styled(" [1-5] Tabs | [Tab] Switch Focus | [Enter] Select/Submit | [q] Quit ", theme::dim())
    };

    let p = Paragraph::new(status_text).style(Style::default().bg(theme::SURFACE));
    f.render_widget(p, area);
}

fn render_popups(f: &mut Frame, app: &App) {
    if let Popup::Confirm { ref message, .. } = app.popup {
        let area = centered_rect(50, 20, f.size());
        f.render_widget(Clear, area);
        let block = Paragraph::new(format!("\n {}\n\n (y/n)", message))
            .block(Block::default().title(" Confirm ").borders(Borders::ALL).border_style(theme::accent()));
        f.render_widget(block, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
