use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub profile: ProfileConfig,
    #[serde(default)]
    pub checks: HashMap<String, CheckConfig>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProfileConfig {
    #[serde(default = "default_profile_name")]
    pub name: String,
}

fn default_profile_name() -> String {
    "default".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub max_branches: Option<usize>,
    #[serde(default)]
    pub max_arguments: Option<usize>,
    #[serde(default)]
    pub max_lines: Option<usize>,
    #[serde(default)]
    pub max_depth: Option<usize>,
    #[serde(default)]
    pub max_methods: Option<usize>,
    #[serde(default)]
    pub rules: Vec<ForbiddenImportRule>,
}

fn default_enabled() -> bool {
    true
}

impl Default for CheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_branches: None,
            max_arguments: None,
            max_lines: None,
            max_depth: None,
            max_methods: None,
            rules: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForbiddenImportRule {
    pub from: String,
    pub forbidden: String,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("read config {}", path.display()))?;
        toml::from_str(&raw).with_context(|| format!("parse config {}", path.display()))
    }

    pub fn discover(start: &Path) -> Result<Self> {
        let mut dir = start.to_path_buf();
        if start.is_file() {
            dir = start
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| PathBuf::from("."));
        }
        loop {
            let candidate = dir.join("zerum.toml");
            if candidate.is_file() {
                return Self::load(&candidate);
            }
            if !dir.pop() {
                break;
            }
        }
        Ok(Self::default())
    }

    pub fn is_check_enabled(&self, id: &str) -> bool {
        self.checks
            .get(id)
            .map(|c| c.enabled)
            .unwrap_or(true)
    }

    pub fn check_config(&self, id: &str) -> CheckConfig {
        self.checks.get(id).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_config_default_is_enabled() {
        let cfg = CheckConfig::default();
        assert!(cfg.enabled);
    }

    #[test]
    fn missing_check_entry_defaults_to_enabled() {
        let config = Config::default();
        assert!(config.is_check_enabled("ZR999"));
        assert!(config.check_config("ZR999").enabled);
    }
}
