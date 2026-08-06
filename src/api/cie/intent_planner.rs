use super::request::{ContextRequest, TaskIntent};

pub struct IntentPlanner;

impl IntentPlanner {
    pub fn classify_intent(req: &ContextRequest) -> TaskIntent {
        let obj_lower = req.objective.to_lowercase();

        if obj_lower.contains("fix")
            || obj_lower.contains("bug")
            || obj_lower.contains("error")
            || obj_lower.contains("fail")
            || obj_lower.contains("panic")
            || obj_lower.contains("issue")
        {
            TaskIntent::Debugging
        } else if obj_lower.contains("continue")
            || obj_lower.contains("resume")
            || obj_lower.contains("pick up")
            || obj_lower.contains("last work")
        {
            TaskIntent::Continuation
        } else if obj_lower.contains("optimize")
            || obj_lower.contains("speed up")
            || obj_lower.contains("benchmark")
            || obj_lower.contains("perf")
            || obj_lower.contains("memory leak")
        {
            TaskIntent::Optimization
        } else if obj_lower.contains("explain")
            || obj_lower.contains("how does")
            || obj_lower.contains("what is")
            || obj_lower.contains("architecture")
            || obj_lower.contains("understand")
        {
            TaskIntent::KnowledgeRetrieval
        } else if obj_lower.contains("refactor")
            || obj_lower.contains("clean up")
            || obj_lower.contains("rename")
            || obj_lower.contains("restructure")
        {
            TaskIntent::Refactoring
        } else if obj_lower.contains("add")
            || obj_lower.contains("implement")
            || obj_lower.contains("build")
            || obj_lower.contains("create")
            || obj_lower.contains("feature")
        {
            TaskIntent::FeatureDevelopment
        } else {
            TaskIntent::General
        }
    }
}
