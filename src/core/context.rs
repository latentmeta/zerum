use crate::config::Config;
use crate::parser::{ParsedFile, SourceModel};
use std::path::{Path, PathBuf};

pub struct CheckContext<'a> {
    pub path: &'a Path,
    pub source: &'a str,
    pub parsed: &'a ParsedFile,
    pub config: &'a Config,
}

impl<'a> CheckContext<'a> {
    pub fn file_path(&self) -> PathBuf {
        self.path.to_path_buf()
    }

    pub fn source_model(&self) -> SourceModel<'a> {
        self.parsed.source_model()
    }
}
