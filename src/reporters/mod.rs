pub mod human;
pub mod json;
pub mod markdown;
pub mod sarif;

use crate::core::Issue;
use anyhow::Result;

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum ReportFormat {
    #[default]
    Human,
    Json,
    Markdown,
    Sarif,
}

pub trait Reporter {
    fn report(&self, issues: &[Issue]) -> Result<String>;
}

pub fn report(format: ReportFormat, issues: &[Issue]) -> Result<String> {
    match format {
        ReportFormat::Human => human::HumanReporter.report(issues),
        ReportFormat::Json => json::JsonReporter.report(issues),
        ReportFormat::Markdown => markdown::MarkdownReporter.report(issues),
        ReportFormat::Sarif => sarif::SarifReporter.report(issues),
    }
}
