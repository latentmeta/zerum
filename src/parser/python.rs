use crate::parser::{ParsedFile, PythonParser};
use anyhow::{Context, Result};
use rustpython_parser::{parse, Mode, ParseError};
use std::path::Path;

#[derive(Debug, Default)]
pub struct RustPythonParser;

impl PythonParser for RustPythonParser {
    fn parse_file(&self, source: &str, path: &Path) -> Result<ParsedFile> {
        let display = path.display().to_string();
        let module = parse(source, Mode::Module, &display).map_err(map_parse_error)?;
        Ok(ParsedFile {
            path: path.to_path_buf(),
            source: source.to_string(),
            module,
        })
    }
}

fn map_parse_error(err: ParseError) -> anyhow::Error {
    anyhow::Error::new(err).context("failed to parse Python source")
}
