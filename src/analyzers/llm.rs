use crate::core::Issue;
use anyhow::Result;
use std::path::Path;

/// LLM-assisted review — explicitly opt-in, Phase 4+.
pub struct LlmAnalyzer;

impl LlmAnalyzer {
    pub fn run(_root: &Path) -> Result<Vec<Issue>> {
        Ok(Vec::new())
    }
}
