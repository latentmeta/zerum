//! LLM-assisted review — explicitly opt-in.
//!
//! TODO(phase-4): wire `LlmAnalyzer` into `zerum check --with-llm`.

#![allow(dead_code)]

use crate::core::Issue;
use anyhow::Result;
use std::path::Path;

pub struct LlmAnalyzer;

impl LlmAnalyzer {
    pub fn run(_root: &Path) -> Result<Vec<Issue>> {
        Ok(Vec::new())
    }
}
