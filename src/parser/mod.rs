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

/// Map a byte offset to 1-based line and column.
///
/// Column is a byte offset within the line (not grapheme-aware).
#[must_use]
pub fn line_col(source: &str, offset: u32) -> (usize, usize) {
    let offset = floor_byte_offset(source, offset as usize);
    let before = &source[..offset];
    let line = before.matches('\n').count() + 1;
    let column = before
        .rfind('\n')
        .map_or(offset + 1, |idx| offset - idx);
    (line, column)
}

fn floor_byte_offset(source: &str, offset: usize) -> usize {
    let mut safe = offset.min(source.len());
    while safe > 0 && !source.is_char_boundary(safe) {
        safe -= 1;
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_handles_utf8_without_panic() {
        let source = "é = 1\n";
        // Byte index 1 is inside the first codepoint.
        let (line, col) = line_col(source, 1);
        assert_eq!(line, 1);
        assert_eq!(col, 1);
    }

    #[test]
    fn line_col_second_line() {
        let source = "a\nbc";
        let (line, col) = line_col(source, 3);
        assert_eq!(line, 2);
        assert_eq!(col, 2);
    }
}
