use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum EventSource {
    Cli,
    Shim(String),
    Watcher(String),
    Neovim,
    Vscode,
    Daemon,
    Api,
    GitLib,
    ShellSession,
    Custom(String),
}

impl fmt::Display for EventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventSource::Cli => write!(f, "cli"),
            EventSource::Shim(s) => write!(f, "shim:{}", s),
            EventSource::Watcher(w) => write!(f, "watcher:{}", w),
            EventSource::Neovim => write!(f, "neovim"),
            EventSource::Vscode => write!(f, "vscode"),
            EventSource::Daemon => write!(f, "daemon"),
            EventSource::Api => write!(f, "api"),
            EventSource::GitLib => write!(f, "gitlib"),
            EventSource::ShellSession => write!(f, "shell_session"),
            EventSource::Custom(s) => write!(f, "source:{}", s),
        }
    }
}
