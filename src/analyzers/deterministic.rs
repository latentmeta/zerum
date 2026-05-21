use crate::config::Config;
use crate::core::{CheckContext, CheckRegistry, Issue};
use crate::parser::{PythonParser, RustPythonParser};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub struct DeterministicAnalyzer {
    registry: CheckRegistry,
    parser: RustPythonParser,
}

#[derive(Debug, Default)]
pub struct AnalyzeResult {
    pub issues: Vec<Issue>,
    pub file_errors: Vec<FileError>,
}

#[derive(Debug, Clone)]
pub struct FileError {
    pub path: PathBuf,
    pub message: String,
}

impl Default for DeterministicAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl DeterministicAnalyzer {
    pub fn new() -> Self {
        Self {
            registry: CheckRegistry::new(),
            parser: RustPythonParser,
        }
    }

    pub fn registry(&self) -> &CheckRegistry {
        &self.registry
    }

    pub fn analyze_file(&self, path: &Path, config: &Config) -> Result<Vec<Issue>> {
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("read {}", path.display()))?;
        let parsed = self
            .parser
            .parse_file(&source, path)
            .with_context(|| format!("parse {}", path.display()))?;
        let ctx = CheckContext {
            path,
            source: &source,
            parsed: &parsed,
            config,
        };
        let mut issues = Vec::new();
        for check in self.registry.all() {
            if config.is_check_enabled(check.id()) {
                issues.extend(check.run(&ctx));
            }
        }
        issues.sort_by(|a, b| {
            (
                &a.file,
                a.line,
                a.column,
                &a.id,
                &a.message,
            )
                .cmp(&(&b.file, b.line, b.column, &b.id, &b.message))
        });
        Ok(issues)
    }

    pub fn analyze_paths(&self, paths: &[PathBuf], config: &Config) -> AnalyzeResult {
        let mut result = AnalyzeResult::default();
        for path in paths {
            match self.analyze_file(path, config) {
                Ok(mut issues) => result.issues.append(&mut issues),
                Err(err) => {
                    eprintln!("warning: {err:#}");
                    result.file_errors.push(FileError {
                        path: path.clone(),
                        message: format!("{err:#}"),
                    });
                }
            }
        }
        result.issues.sort_by(|a, b| {
            (
                &a.file,
                a.line,
                a.column,
                &a.id,
                &a.message,
            )
                .cmp(&(&b.file, b.line, b.column, &b.id, &b.message))
        });
        result
    }
}
