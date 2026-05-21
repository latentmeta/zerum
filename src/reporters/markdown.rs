use crate::core::Issue;
use crate::reporters::Reporter;
use anyhow::Result;
use std::fmt::Write as _;

pub struct MarkdownReporter;

impl Reporter for MarkdownReporter {
    fn report(&self, issues: &[Issue]) -> Result<String> {
        let mut out = String::from("| ID | Severity | File | Line | Message |\n");
        out.push_str("|----|----------|------|------|----------|\n");
        for issue in issues {
            writeln!(
                out,
                "| {} | {} | {} | {} | {} |",
                issue.id,
                issue.severity,
                issue.file.display(),
                issue.line,
                issue.message.replace('|', "\\|")
            )?;
        }
        Ok(out)
    }
}
