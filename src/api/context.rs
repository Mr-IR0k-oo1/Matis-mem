use anyhow::Result;
use crate::core::{Event, EventPayload, MemoryItem};
use crate::data::{Knowledge, Project};

#[derive(Debug, Clone, Default)]
pub struct ContextRequestOptions {
    pub project_name: String,
    pub include_decisions: bool,
    pub include_commits: bool,
    pub include_knowledge: bool,
    pub query: Option<String>,
}

pub struct RichContextBuilder;

impl RichContextBuilder {
    pub fn build_context(
        project: Option<&Project>,
        events: &[Event],
        memories: &[MemoryItem],
        knowledge_items: &[Knowledge],
        opts: &ContextRequestOptions,
    ) -> Result<String> {
        let mut sections = Vec::new();

        // 1. Project overview
        if let Some(p) = project {
            sections.push(format!("[PROJECT OVERVIEW: {}]\nGoal: {}\nNotes: {}", p.name, p.goal, p.notes));
        }

        // 2. Recent Decisions
        if opts.include_decisions {
            let mut decisions = Vec::new();
            for ev in events {
                if let EventPayload::Decision(d) = &ev.payload {
                    decisions.push(format!("- Decision: {}\n  Rationale: {}", d.title, d.rationale));
                }
            }
            for mem in memories {
                if mem.title.starts_with("Decision:") {
                    decisions.push(format!("- {}\n  {}", mem.title, mem.content));
                }
            }
            if !decisions.is_empty() {
                sections.push(format!("[RECENT DECISIONS]\n{}", decisions.join("\n")));
            }
        }

        // 3. Relevant Commits
        if opts.include_commits {
            let mut commits = Vec::new();
            for ev in events {
                if let EventPayload::Commit(c) = &ev.payload {
                    commits.push(format!(
                        "- Commit {} ({}) : {}\n  Files: {}",
                        &c.hash[..c.hash.len().min(7)],
                        c.branch,
                        c.message,
                        c.files_changed.join(", ")
                    ));
                }
            }
            if !commits.is_empty() {
                sections.push(format!("[RECENT COMMITS]\n{}", commits.iter().take(5).cloned().collect::<Vec<_>>().join("\n")));
            }
        }

        // 4. Important Memories
        let mut important_mems = Vec::new();
        for mem in memories {
            important_mems.push(format!("[{}] {}: {}", mem.tier, mem.title, mem.content));
        }
        if !important_mems.is_empty() {
            sections.push(format!("[ENGINEERING MEMORIES]\n{}", important_mems.iter().take(5).cloned().collect::<Vec<_>>().join("\n")));
        }

        // 5. Knowledge items
        if opts.include_knowledge && !knowledge_items.is_empty() {
            let mut kn_strings = Vec::new();
            for k in knowledge_items {
                kn_strings.push(format!("- Topic: {}\n  Notes: {}", k.topic, k.notes.join("; ")));
            }
            sections.push(format!("[SEMANTIC KNOWLEDGE]\n{}", kn_strings.join("\n")));
        }

        Ok(sections.join("\n\n---\n\n"))
    }
}
