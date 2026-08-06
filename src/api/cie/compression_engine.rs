use super::ranking_engine::ScoredCandidate;

#[derive(Debug, Clone)]
pub struct CompressedItem {
    pub title: String,
    pub body: String,
    pub tier_name: String,
    pub compression_level: &'static str,
    pub estimated_tokens: usize,
    pub original_id: String,
}

pub struct CompressionEngine;

impl CompressionEngine {
    pub fn compress_to_budget(
        scored: Vec<ScoredCandidate>,
        token_budget: usize,
    ) -> Vec<CompressedItem> {
        let mut result = Vec::new();
        let mut used_tokens = 0usize;

        for item in scored {
            let body = &item.candidate.content;
            let est_tokens = est_token_count(body);

            if used_tokens + est_tokens <= token_budget {
                result.push(CompressedItem {
                    title: item.candidate.title.clone(),
                    body: body.clone(),
                    tier_name: item.candidate.tier_name.clone(),
                    compression_level: "Full",
                    estimated_tokens: est_tokens,
                    original_id: item.candidate.id.clone(),
                });
                used_tokens += est_tokens;
            } else {
                // Progressive Compression: Summarize
                let summary_body = summarize(body, 120);
                let summary_tokens = est_token_count(&summary_body);

                if used_tokens + summary_tokens <= token_budget {
                    result.push(CompressedItem {
                        title: item.candidate.title.clone(),
                        body: summary_body,
                        tier_name: item.candidate.tier_name.clone(),
                        compression_level: "Summary",
                        estimated_tokens: summary_tokens,
                        original_id: item.candidate.id.clone(),
                    });
                    used_tokens += summary_tokens;
                } else {
                    // Compress to Reference only
                    let ref_body = format!("[Reference to ID: {}]", item.candidate.id);
                    let ref_tokens = est_token_count(&ref_body);
                    if used_tokens + ref_tokens <= token_budget {
                        result.push(CompressedItem {
                            title: item.candidate.title.clone(),
                            body: ref_body,
                            tier_name: item.candidate.tier_name.clone(),
                            compression_level: "Reference",
                            estimated_tokens: ref_tokens,
                            original_id: item.candidate.id.clone(),
                        });
                        used_tokens += ref_tokens;
                    } else {
                        break;
                    }
                }
            }
        }

        result
    }
}

fn est_token_count(text: &str) -> usize {
    // Standard rule of thumb: ~4 chars per token
    (text.len() / 4).max(1)
}

fn summarize(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}…", &text[..max_len])
    } else {
        text.to_string()
    }
}
