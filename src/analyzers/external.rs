//! External checker orchestration.

use crate::config::Config;
use crate::core::Issue;
use crate::integrations::{find_checker, ExternalFinding};
use anyhow::Result;
use std::path::Path;

pub struct ExternalAnalyzer {
    checker_ids: Vec<String>,
}

impl ExternalAnalyzer {
    pub fn from_config(config: &Config) -> Self {
        Self {
            checker_ids: config.external_checkers.clone(),
        }
    }

    pub fn with_checkers(ids: Vec<String>) -> Self {
        Self { checker_ids: ids }
    }

    pub fn run(&self, root: &Path) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        for id in &self.checker_ids {
            let Some(checker) = find_checker(id) else {
                continue;
            };
            if !checker.is_available() {
                continue;
            }
            for finding in checker.run(root)? {
                issues.push(map_finding(finding));
            }
        }
        Ok(issues)
    }
}

fn map_finding(f: ExternalFinding) -> Issue {
    f.into_issue()
}
