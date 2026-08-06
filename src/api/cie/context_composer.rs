use super::citation_generator::ContextCitation;
use super::compression_engine::CompressedItem;
use super::graph_traverser::GraphContextCandidate;
use super::request::{ContextRequest, TaskIntent};

#[derive(Debug, Clone)]
pub struct AssembledContext {
    pub text: String,
    pub intent: TaskIntent,
    pub token_estimate: usize,
    pub citations: Vec<ContextCitation>,
}

pub struct ContextComposer;

impl ContextComposer {
    pub fn compose(
        req: &ContextRequest,
        intent: TaskIntent,
        items: &[CompressedItem],
        graph_nodes: &[GraphContextCandidate],
        citations: Vec<ContextCitation>,
    ) -> AssembledContext {
        let mut sections = Vec::new();

        // Header & Objective
        sections.push(format!(
            "# Context Intelligence Engine\n\n**Project:** {}\n**Intent:** {}\n**Objective:** {}\n",
            req.project, intent, req.objective
        ));

        // Active State
        if let Some(ref branch) = req.current_branch {
            sections.push(format!("## Active Work\n- **Branch:** {}\n", branch));
        }

        if !req.current_files.is_empty() {
            sections.push(format!(
                "- **Active Files:** {}\n",
                req.current_files.join(", ")
            ));
        }

        // Relevant Memory & Events
        if !items.is_empty() {
            let mut mem_text = String::from("## Relevant Engineering Context\n\n");
            for item in items {
                mem_text.push_str(&format!(
                    "### [{}] {}\n*Compression Level: {}\n\n{}\n\n",
                    item.tier_name, item.title, item.compression_level, item.body
                ));
            }
            sections.push(mem_text);
        }

        // Graph Connections
        if !graph_nodes.is_empty() {
            let mut graph_text = String::from("## Graph Connections\n\n");
            for g in graph_nodes {
                graph_text.push_str(&format!("- **{}**: {}\n", g.title, g.detail));
            }
            sections.push(graph_text);
        }

        // Explainability & Citations
        if !citations.is_empty() {
            let mut cite_text = String::from("## Citations & Selection Rationale\n\n");
            for c in &citations {
                cite_text.push_str(&format!(
                    "- **{}** (`{}`)\n  Rationale: {} (Confidence: {:.2})\n",
                    c.title, c.item_id, c.explanation, c.confidence
                ));
            }
            sections.push(cite_text);
        }

        let text = sections.join("\n---\n\n");
        let token_estimate = (text.len() / 4).max(1);

        AssembledContext {
            text,
            intent,
            token_estimate,
            citations,
        }
    }
}
