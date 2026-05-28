//! Ruff JSON adapter.

use super::external_checker::{ExternalChecker, ExternalFinding};
use crate::core::Severity;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

pub struct RuffChecker;

impl ExternalChecker for RuffChecker {
    fn id(&self) -> &'static str {
        "ruff"
    }

    fn name(&self) -> &'static str {
        "Ruff"
    }

    fn is_available(&self) -> bool {
        Command::new("ruff")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn run(&self, root: &Path) -> Result<Vec<ExternalFinding>> {
        let output = Command::new("ruff")
            .args(["check", "--output-format", "json"])
            .arg(root)
            .output()
            .context("spawn ruff check")?;

        if !output.status.success() && output.stdout.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("not found") || stderr.contains("No such file") {
                return Ok(Vec::new());
            }
        }

        let items: Vec<RuffDiagnostic> = if output.stdout.is_empty() {
            Vec::new()
        } else {
            serde_json::from_slice(&output.stdout).context("parse ruff JSON output")?
        };

        Ok(items
            .into_iter()
            .map(|d| {
                let severity = map_ruff_severity(&d.code);
                ExternalFinding {
                    tool: "ruff".to_string(),
                    code: d.code,
                    message: d.message,
                    file: d.filename,
                    line: d.location.row.max(1),
                    column: d.location.column.max(1),
                    severity,
                }
            })
            .collect())
    }
}

fn map_ruff_severity(code: &str) -> Severity {
    if code.starts_with('E') || code.starts_with('F') {
        Severity::High
    } else {
        Severity::Medium
    }
}

#[derive(Debug, Deserialize)]
struct RuffDiagnostic {
    code: String,
    message: String,
    filename: String,
    location: RuffLocation,
}

#[derive(Debug, Deserialize)]
struct RuffLocation {
    row: usize,
    column: usize,
}

pub fn available() -> bool {
    RuffChecker.is_available()
}
