use crate::config::Config;
use crate::core::{CheckContext, CheckRegistry, Issue};
use crate::parser::{PythonParser, RustPythonParser};
use anyhow::{Context, Result};
use std::path::Path;

pub struct DeterministicAnalyzer {
    registry: CheckRegistry,
    parser: RustPythonParser,
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
            ).cmp(&(
                &b.file,
                b.line,
                b.column,
                &b.id,
                &b.message,
            ))
        });
        Ok(issues)
    }

    pub fn analyze_paths(&self, paths: &[std::path::PathBuf], config: &Config) -> Result<Vec<Issue>> {
        let mut all = Vec::new();
        for path in paths {
            match self.analyze_file(path, config) {
                Ok(mut issues) => all.append(&mut issues),
                Err(err) => {
                    eprintln!("warning: {err:#}");
                }
            }
        }
        Ok(all)
    }
}
