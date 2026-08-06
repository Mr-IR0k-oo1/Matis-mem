#![allow(dead_code, unreachable_patterns)]
mod api;
mod capture;
mod config;
mod context;
mod core;
mod data;
mod error;
mod graph;
mod memory;
mod platform;
mod storage;
mod ui;
mod watcher;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};
use ui::app::App;

fn main() {
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--version" | "-V" => {
                println!("matis-mem v{} ({})", env!("CARGO_PKG_VERSION"), platform::os_name());
                return;
            }
            "--daemon" | "-d" => {
                config::init();
                if let Err(e) = run_daemon() {
                    eprintln!("matisd error: {}", e);
                    std::process::exit(1);
                }
                return;
            }
            "--help" | "-h" => {
                config::init();
                print_help();
                return;
            }
            _ => {}
        }
    }

    if !platform::is_tty() {
        eprintln!("matis-mem: requires an interactive terminal (or run with --daemon)");
        std::process::exit(1);
    }

    if let Err(e) = run() {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        eprintln!("matis-mem: {}", e);
        std::process::exit(1);
    }
}

fn run_daemon() -> Result<()> {
    config::ensure_dirs()?;
    println!("matisd: Local daemon engine running in background...");
    let event_store = storage::EventStore::new();
    let (bus, _rx) = capture::EventBus::new();

    if let Ok(rx) = watcher::log_watcher::start() {
        for evt in rx {
            if let watcher::log_watcher::WatchEvent::NewLog(log) = evt {
                let ev = capture::generic::GenericCapture::parse_log(&log);
                let _ = event_store.append(&ev);
                bus.publish(ev);
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    config::init();
    config::ensure_dirs()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let result = run_app(&mut terminal);

    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture);
    let _ = terminal.show_cursor();
    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new()?;
    let tick = Duration::from_millis(100);
    let mut last = Instant::now();

    loop {
        terminal.draw(|f| ui::render::render(f, &app))?;

        let timeout = tick.saturating_sub(last.elapsed());
        if crossterm::event::poll(timeout)? {
            let ev = crossterm::event::read()?;
            ui::events::handle(&ev, &mut app);
        }

        if last.elapsed() >= tick {
            app.tick();
            last = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn print_help() {
    println!("matis-mem v{} — {}", env!("CARGO_PKG_VERSION"), platform::os_name());
    println!("Terminal Engineering Memory & Context Layer");
    println!();
    println!("USAGE");
    println!("  matis-mem              Launch TUI interface");
    println!("  matis-mem --daemon     Launch local matisd daemon engine");
    println!("  matis-mem --version    Version + OS");
    println!("  matis-mem --help       This help");
    println!();
    println!("DATA:   {}", platform::data_dir_display());
    println!("EVENTS: {}", config::events_dir().display());
    println!("SHIMS:  {}", config::shims_dir().display());
    println!();
    println!("TABS");
    println!("  [1] TODAY     Daily engineering dashboard & prompt capture");
    println!("  [2] TIMELINE  Chronological engineering event stream");
    println!("  [3] MEMORY    Working, Episodic, and Semantic memory store");
    println!("  [4] GRAPH     Event, dependency, and knowledge network inspector");
    println!("  [5] SETTINGS  Passive capture shim installers and daemon options");
    println!();
    println!("GLOBAL KEYS");
    println!("  1-5 / Tab     Switch tabs");
    println!("  Ctrl+N        New project");
    println!("  q / Ctrl+C    Quit");
    println!();
    for line in platform::install_instructions() {
        println!("  {}", line);
    }
}
