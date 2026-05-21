use crate::core::Issue;
use crate::reporters::Reporter;
use anyhow::Result;
use std::fmt::Write as _;

pub struct HumanReporter;

impl Reporter for HumanReporter {
    fn report(&self, issues: &[Issue]) -> Result<String> {
        if issues.is_empty() {
            return Ok("No issues found.\n".to_string());
        }
        let mut out = String::new();
        for issue in issues {
            writeln!(
                out,
                "[{}] {} {}:{}:{} — {}",
                issue.severity,
                issue.id,
                issue.file.display(),
                issue.line,
                issue.column,
                issue.message
            )?;
            if let Some(explanation) = &issue.explanation {
                writeln!(out, "  explanation: {explanation}")?;
            }
            if let Some(remediation) = &issue.remediation {
                writeln!(out, "  remediation: {remediation}")?;
            }
        }
        Ok(out)
    }
}
