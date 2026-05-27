use anyhow::{Context, Result};
use crate::core::Severity;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

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
    #[serde(default)]
    pub severity: Option<Severity>,
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
            severity: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForbiddenImportRule {
    pub from: String,
    pub forbidden: String,
}

fn default_check_config() -> &'static CheckConfig {
    static DEFAULT: OnceLock<CheckConfig> = OnceLock::new();
    DEFAULT.get_or_init(CheckConfig::default)
}

fn is_project_root(dir: &Path) -> bool {
    dir.join(".git").exists()
        || dir.join("pyproject.toml").is_file()
        || dir.join("zerum.toml").is_file()
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
            if is_project_root(&dir) {
                break;
            }
            if !dir.pop() {
                break;
            }
        }
        Ok(Self::default())
    }

    pub fn is_check_enabled(&self, id: &str) -> bool {
        self.check_config(id).enabled
    }

    pub fn check_config(&self, id: &str) -> &CheckConfig {
        match self.checks.get(id) {
            Some(cfg) => cfg,
            None => default_check_config(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

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

    #[test]
    fn check_config_returns_shared_default_without_clone() {
        let config = Config::default();
        let a = config.check_config("ZR999") as *const CheckConfig;
        let b = config.check_config("ZR888") as *const CheckConfig;
        assert_eq!(a, b);
    }

    #[test]
    fn discover_stops_at_project_root_without_config() {
        let outer = tempdir().unwrap();
        let project = outer.path().join("myapp");
        fs::create_dir_all(project.join("src")).unwrap();
        fs::write(project.join("pyproject.toml"), "[project]\nname = \"x\"\n").unwrap();
        fs::write(outer.path().join("zerum.toml"), "name = \"wrong\"\n").unwrap();

        let config = Config::discover(&project.join("src")).unwrap();
        assert!(config.checks.is_empty());
    }
}
