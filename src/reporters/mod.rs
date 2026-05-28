pub mod human;
pub mod json;

use crate::core::Issue;
use anyhow::Result;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReportKind {
    #[default]
    Human,
    Json,
}

pub trait Reporter {
    fn report(&self, issues: &[Issue]) -> Result<String>;
}

pub fn render(kind: ReportKind, issues: &[Issue]) -> Result<String> {
    match kind {
        ReportKind::Human => human::HumanReporter.report(issues),
        ReportKind::Json => json::JsonReporter.report(issues),
    }
}
