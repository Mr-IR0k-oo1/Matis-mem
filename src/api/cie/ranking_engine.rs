use super::memory_selector::MemoryCandidate;

#[derive(Debug, Clone)]
pub struct ScoredCandidate {
    pub candidate: MemoryCandidate,
    pub score: f32,
    pub confidence: f32,
}

pub struct RankingEngine;

impl RankingEngine {
    pub fn rank(candidates: Vec<MemoryCandidate>, query: &str) -> Vec<ScoredCandidate> {
        let q_lower = query.to_lowercase();
        let terms: Vec<&str> = q_lower.split_whitespace().collect();

        let mut scored: Vec<ScoredCandidate> = candidates
            .into_iter()
            .map(|cand| {
                let text_lower = format!("{} {}", cand.title, cand.content).to_lowercase();
                let mut match_count = 0usize;
                for term in &terms {
                    if text_lower.contains(term) {
                        match_count += 1;
                    }
                }

                let term_score = if !terms.is_empty() {
                    (match_count as f32) / (terms.len() as f32)
                } else {
                    0.5
                };

                let score = cand.weight * 0.6 + term_score * 0.4;
                let confidence = (score * 0.95).min(0.99);

                ScoredCandidate {
                    candidate: cand,
                    score,
                    confidence,
                }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }
}
