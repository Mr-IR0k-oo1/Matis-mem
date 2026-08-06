use super::compression_engine::CompressedItem;
use super::request::TaskIntent;

#[derive(Debug, Clone)]
pub struct ContextCitation {
    pub item_id: String,
    pub title: String,
    pub explanation: String,
    pub confidence: f32,
}

pub struct CitationGenerator;

impl CitationGenerator {
    pub fn generate_citations(
        items: &[CompressedItem],
        intent: TaskIntent,
    ) -> Vec<ContextCitation> {
        items
            .iter()
            .map(|item| {
                let explanation = match intent {
                    TaskIntent::Continuation => format!(
                        "Selected from {} because it captures active working state.",
                        item.tier_name
                    ),
                    TaskIntent::Debugging => format!(
                        "Selected from {} due to relevance to recent error/event history.",
                        item.tier_name
                    ),
                    TaskIntent::Optimization => format!(
                        "Selected from {} to provide architectural performance context.",
                        item.tier_name
                    ),
                    TaskIntent::KnowledgeRetrieval => format!(
                        "Selected from {} as high-confidence semantic knowledge.",
                        item.tier_name
                    ),
                    _ => format!("Retrieved from {} based on keyword relevance.", item.tier_name),
                };

                ContextCitation {
                    item_id: item.original_id.clone(),
                    title: item.title.clone(),
                    explanation,
                    confidence: 0.95,
                }
            })
            .collect()
    }
}
