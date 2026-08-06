use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Importance {
    Temporary = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl fmt::Display for Importance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Importance::Temporary => write!(f, "temporary"),
            Importance::Low => write!(f, "low"),
            Importance::Medium => write!(f, "medium"),
            Importance::High => write!(f, "high"),
            Importance::Critical => write!(f, "critical"),
        }
    }
}
