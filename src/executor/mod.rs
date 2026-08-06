#![allow(dead_code)]
//! Legacy executor subsystem retained for backward compatibility.

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub provider: String,
}
