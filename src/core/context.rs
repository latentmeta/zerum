use crate::config::Config;
use crate::parser::{ParsedFile, SourceModel};
use std::path::{Path, PathBuf};

pub struct CheckContext<'a> {
    pub path: &'a Path,
    pub source: &'a str,
    pub parsed: &'a ParsedFile,
    pub config: &'a Config,
    model: SourceModel<'a>,
}

impl<'a> CheckContext<'a> {
    pub fn new(
        path: &'a Path,
        source: &'a str,
        parsed: &'a ParsedFile,
        config: &'a Config,
    ) -> Self {
        Self {
            path,
            source,
            parsed,
            config,
            model: SourceModel::from_parsed(parsed),
        }
    }

    pub fn file_path(&self) -> PathBuf {
        self.path.to_path_buf()
    }

    pub fn source_model(&self) -> &SourceModel<'a> {
        &self.model
    }
}
