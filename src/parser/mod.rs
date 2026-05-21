mod python;

use anyhow::Result;
use rustpython_ast::Mod;
use std::path::{Path, PathBuf};

pub use python::RustPythonParser;

pub trait PythonParser: Send + Sync {
    fn parse_file(&self, source: &str, path: &Path) -> Result<ParsedFile>;
}

#[derive(Debug, Clone)]
pub struct ParsedFile {
    pub path: PathBuf,
    pub source: String,
    pub module: Mod,
}

#[must_use]
pub fn line_col(source: &str, offset: u32) -> (usize, usize) {
    let offset = offset as usize;
    let safe = offset.min(source.len());
    let before = &source[..safe];
    let line = before.matches('\n').count() + 1;
    let column = before
        .rfind('\n')
        .map(|idx| safe - idx)
        .unwrap_or(safe + 1);
    (line, column)
}
