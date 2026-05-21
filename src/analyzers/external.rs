use crate::core::Issue;
use anyhow::Result;
use std::path::Path;

/// External checker orchestration (Ruff, Mypy, etc.) — Phase 2.
pub struct ExternalAnalyzer;

impl ExternalAnalyzer {
    pub fn run(_root: &Path) -> Result<Vec<Issue>> {
        Ok(Vec::new())
    }
}
