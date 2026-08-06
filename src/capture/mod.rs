pub mod claude;
pub mod codex;
pub mod cursor;
pub mod filesystem;
pub mod gemini;
pub mod generic;
pub mod git;
pub mod shell;

use crate::core::Event;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub trait CaptureAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn observe(&self, bus: &EventBus);
}

#[derive(Clone)]
pub struct EventBus {
    tx: Sender<Event>,
    subscribers: Arc<Mutex<Vec<Sender<Event>>>>,
}

impl EventBus {
    pub fn new() -> (Self, Receiver<Event>) {
        let (tx, rx) = channel();
        (
            Self {
                tx,
                subscribers: Arc::new(Mutex::new(Vec::new())),
            },
            rx,
        )
    }

    pub fn publish(&self, event: Event) {
        let _ = self.tx.send(event.clone());
        if let Ok(subs) = self.subscribers.lock() {
            for sub in subs.iter() {
                let _ = sub.send(event.clone());
            }
        }
    }

    pub fn subscribe(&self) -> Receiver<Event> {
        let (tx, rx) = channel();
        if let Ok(mut subs) = self.subscribers.lock() {
            subs.push(tx);
        }
        rx
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new().0
    }
}
