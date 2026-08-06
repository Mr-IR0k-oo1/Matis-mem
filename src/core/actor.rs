use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Actor {
    User,
    AI {
        name: String,
        provider: Option<String>,
    },
    Git,
    Shell,
    Filesystem,
    Daemon,
    Ide {
        name: String,
    },
    Ci {
        system: String,
    },
    Custom(String),
}

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Actor::User => write!(f, "User"),
            Actor::AI { name, provider } => {
                if let Some(ref p) = provider {
                    write!(f, "AI({}/{})", p, name)
                } else {
                    write!(f, "AI({})", name)
                }
            }
            Actor::Git => write!(f, "Git"),
            Actor::Shell => write!(f, "Shell"),
            Actor::Filesystem => write!(f, "Filesystem"),
            Actor::Daemon => write!(f, "Daemon"),
            Actor::Ide { name } => write!(f, "IDE({})", name),
            Actor::Ci { system } => write!(f, "CI({})", system),
            Actor::Custom(s) => write!(f, "Actor({})", s),
        }
    }
}
