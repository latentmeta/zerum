//! Trait for orchestrating external linters and type checkers.

use crate::core::{Category, ConfidenceKind, Issue, Severity};
use anyhow::Result;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ExternalFinding {
    pub tool: String,
    pub code: String,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
}

pub trait ExternalChecker: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn is_available(&self) -> bool;
    fn run(&self, root: &Path) -> Result<Vec<ExternalFinding>>;
}

impl ExternalFinding {
    pub fn into_issue(self) -> Issue {
        let title = format!("{}:{}", self.tool, self.code);
        let id = format!("EXT-{}-{}", self.tool.to_uppercase(), self.code);
        Issue::builder(
            id,
            title,
            self.message,
            self.file.into(),
            self.line,
            self.column,
        )
        .category(Category::Warning)
        .severity(self.severity)
        .build()
        .with_source(self.tool)
    }
}

trait IssueSource {
    fn with_source(self, source: String) -> Issue;
}

impl IssueSource for Issue {
    fn with_source(mut self, source: String) -> Issue {
        self.source = source;
        self.confidence = ConfidenceKind::ToolReported;
        self
    }
}

pub fn builtin_checkers() -> Vec<Box<dyn ExternalChecker>> {
    vec![Box::new(super::ruff::RuffChecker)]
}

pub fn find_checker(id: &str) -> Option<Box<dyn ExternalChecker>> {
    builtin_checkers().into_iter().find(|c| c.id() == id)
}
