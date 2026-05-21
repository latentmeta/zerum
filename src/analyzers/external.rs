//! External checker orchestration (Ruff, Mypy, etc.).
//!
//! TODO(phase-2): wire `ExternalAnalyzer` into the CLI and `integrations/`.

#![allow(dead_code)]

use crate::core::Issue;
use anyhow::Result;
use std::path::Path;

pub struct ExternalAnalyzer;

impl ExternalAnalyzer {
    pub fn run(_root: &Path) -> Result<Vec<Issue>> {
        Ok(Vec::new())
    }
}
