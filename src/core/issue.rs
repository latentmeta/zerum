use crate::core::{Category, Check, Severity};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
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
    pub explanation: Option<Cow<'static, str>>,
    pub remediation: Option<Cow<'static, str>>,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
    pub category: Category,
    pub source: String,
    pub confidence: ConfidenceKind,
}

/// Builds a deterministic [`Issue`] without a long positional argument list.
#[derive(Debug)]
pub struct IssueBuilder {
    id: String,
    title: String,
    message: String,
    file: PathBuf,
    line: usize,
    column: usize,
    severity: Severity,
    category: Category,
    explanation: Option<&'static str>,
    remediation: Option<&'static str>,
}

impl Issue {
    pub fn builder(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
        file: PathBuf,
        line: usize,
        column: usize,
    ) -> IssueBuilder {
        IssueBuilder {
            id: id.into(),
            title: title.into(),
            message: message.into(),
            file,
            line,
            column,
            severity: Severity::Medium,
            category: Category::Design,
            explanation: None,
            remediation: None,
        }
    }

    pub fn from_check(
        check: &dyn Check,
        message: impl Into<String>,
        file: PathBuf,
        line: usize,
        column: usize,
    ) -> Self {
        Self::builder(check.id(), check.name(), message, file, line, column)
            .severity(check.severity())
            .category(check.category())
            .guidance(check.explanation(), check.remediation())
            .build()
    }
}

impl IssueBuilder {
    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    pub fn guidance(mut self, explanation: &'static str, remediation: &'static str) -> Self {
        self.explanation = Some(explanation);
        self.remediation = Some(remediation);
        self
    }

    pub fn build(self) -> Issue {
        Issue {
            id: self.id,
            title: self.title,
            message: self.message,
            explanation: self.explanation.map(Cow::Borrowed),
            remediation: self.remediation.map(Cow::Borrowed),
            file: self.file,
            line: self.line,
            column: self.column,
            severity: self.severity,
            category: self.category,
            source: "zerum".to_string(),
            confidence: ConfidenceKind::Deterministic,
        }
    }
}
