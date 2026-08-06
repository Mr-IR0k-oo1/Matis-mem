pub mod citation_generator;
pub mod compression_engine;
pub mod context_composer;
pub mod graph_traverser;
pub mod intent_planner;
pub mod memory_selector;
pub mod ranking_engine;
pub mod request;
pub mod retrieval_planner;

pub use citation_generator::{CitationGenerator, ContextCitation};
pub use compression_engine::{CompressedItem, CompressionEngine};
pub use context_composer::{AssembledContext, ContextComposer};
pub use graph_traverser::{GraphContextCandidate, GraphTraverser};
pub use intent_planner::IntentPlanner;
pub use memory_selector::{MemoryCandidate, MemorySelector};
pub use ranking_engine::{RankingEngine, ScoredCandidate};
pub use request::{ContextRequest, TaskIntent};
pub use retrieval_planner::{RetrievalPlanner, RetrievalWeights};

use crate::core::Timeline;
use crate::graph::{DependencyGraph, EventGraph, KnowledgeGraph};
use crate::memory::{EpisodicMemory, SemanticMemory, WorkingMemory};

pub struct ContextIntelligenceEngine;

impl ContextIntelligenceEngine {
    pub fn build_context(
        req: &ContextRequest,
        working: &WorkingMemory,
        episodic: &EpisodicMemory,
        semantic: &SemanticMemory,
        timeline: &Timeline,
        event_graph: &EventGraph,
        dep_graph: &DependencyGraph,
        knowledge_graph: &KnowledgeGraph,
    ) -> AssembledContext {
        // 1. Intent Classification
        let intent = IntentPlanner::classify_intent(req);

        // 2. Retrieval Planning
        let weights = RetrievalPlanner::plan(intent);

        // 3. Memory Candidate Selection
        let candidates = MemorySelector::select_candidates(
            working,
            episodic,
            semantic,
            timeline,
            &weights,
            &req.project,
        );

        // 4. Graph Traversal
        let graph_candidates = GraphTraverser::traverse(
            event_graph,
            dep_graph,
            knowledge_graph,
            &req.current_files,
        );

        // 5. Relevance Ranking
        let scored = RankingEngine::rank(candidates, &req.objective);

        // 6. Token Budget Optimization & Compression
        let compressed = CompressionEngine::compress_to_budget(scored, req.token_budget);

        // 7. Citation & Explanation Generation
        let citations = CitationGenerator::generate_citations(&compressed, intent);

        // 8. Context Composition
        ContextComposer::compose(req, intent, &compressed, &graph_candidates, citations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cie_full_pipeline() {
        let req = ContextRequest::new("Fix JWT expiration bug", "auth_project")
            .with_token_budget(4000)
            .with_branch("feature/auth");

        let mut working = WorkingMemory::new();
        working.push("Active Prompt", "Fixing JWT expiration issue", "auth_project");

        let episodic = EpisodicMemory::new();
        let semantic = SemanticMemory::new();
        let timeline = Timeline::new();
        let event_graph = EventGraph::new();
        let dep_graph = DependencyGraph::new();
        let knowledge_graph = KnowledgeGraph::new();

        let ctx = ContextIntelligenceEngine::build_context(
            &req,
            &working,
            &episodic,
            &semantic,
            &timeline,
            &event_graph,
            &dep_graph,
            &knowledge_graph,
        );

        assert_eq!(ctx.intent, TaskIntent::Debugging);
        assert!(ctx.text.contains("auth_project"));
        assert!(ctx.text.contains("Fix JWT expiration bug"));
    }
}
