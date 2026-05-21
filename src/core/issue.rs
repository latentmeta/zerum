use crate::core::{Category, Severity};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceKind {
    Deterministic,
    ToolReported,
    LlmInferred,
    ConsensusInferred,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub message: String,
    pub explanation: Option<String>,
    pub remediation: Option<String>,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
    pub category: Category,
    pub source: String,
    pub confidence: ConfidenceKind,
}

impl Issue {
    pub fn deterministic(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
        file: PathBuf,
        line: usize,
        column: usize,
        severity: Severity,
        category: Category,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            message: message.into(),
            explanation: None,
            remediation: None,
            file,
            line,
            column,
            severity,
            category,
            source: "zerum".to_string(),
            confidence: ConfidenceKind::Deterministic,
        }
    }

    pub fn with_guidance(mut self, explanation: impl Into<String>, remediation: impl Into<String>) -> Self {
        self.explanation = Some(explanation.into());
        self.remediation = Some(remediation.into());
        self
    }
}
