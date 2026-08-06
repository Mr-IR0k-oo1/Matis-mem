use crate::core::{Event, EventPayload, Importance, MemoryItem, MemoryTier};
use super::episodic::EpisodicMemory;
use super::semantic::SemanticMemory;
use super::working::WorkingMemory;

pub struct MemoryPromotionEngine;

impl MemoryPromotionEngine {
    pub fn process_event(
        event: &Event,
        working: &mut WorkingMemory,
        episodic: &mut EpisodicMemory,
        semantic: &mut SemanticMemory,
    ) {
        let proj_str = event.project.as_str();
        match &event.payload {
            EventPayload::Prompt(p) => {
                working.push("Active Prompt", &p.prompt, proj_str);
            }
            EventPayload::Response(r) => {
                let item = MemoryItem::new(
                    format!("{} Interaction", r.agent),
                    &r.response,
                    MemoryTier::Episodic,
                    proj_str,
                )
                .with_source_event(event.id.as_str());

                if event.importance >= Importance::High {
                    semantic.store(item);
                } else {
                    episodic.add(item);
                }
            }
            EventPayload::Decision(d) => {
                let item = MemoryItem::new(
                    format!("Decision: {}", d.title),
                    &d.rationale,
                    MemoryTier::Semantic,
                    proj_str,
                )
                .with_source_event(event.id.as_str());

                semantic.store(item);
            }
            EventPayload::Commit(c) => {
                let item = MemoryItem::new(
                    format!("Commit {}", &c.hash[..c.hash.len().min(7)]),
                    &c.message,
                    MemoryTier::Episodic,
                    proj_str,
                )
                .with_source_event(event.id.as_str());

                episodic.add(item);
            }
            _ => {}
        }
    }
}
