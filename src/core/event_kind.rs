use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Prompt,
    Response,
    Decision,
    Git,
    Filesystem,
    Shell,
    Build,
    Test,
    Deployment,
    Issue,
    Knowledge,
    Memory,
    System,
}

impl fmt::Display for EventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventKind::Prompt => write!(f, "prompt"),
            EventKind::Response => write!(f, "response"),
            EventKind::Decision => write!(f, "decision"),
            EventKind::Git => write!(f, "git"),
            EventKind::Filesystem => write!(f, "filesystem"),
            EventKind::Shell => write!(f, "shell"),
            EventKind::Build => write!(f, "build"),
            EventKind::Test => write!(f, "test"),
            EventKind::Deployment => write!(f, "deployment"),
            EventKind::Issue => write!(f, "issue"),
            EventKind::Knowledge => write!(f, "knowledge"),
            EventKind::Memory => write!(f, "memory"),
            EventKind::System => write!(f, "system"),
        }
    }
}
