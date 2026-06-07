mod defaults;

use crate::core::Severity;
use anyhow::{bail, Context, Result};
use defaults::is_disabled_in_default_profile;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub use defaults::DEFAULT_PROFILE_DISABLED;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub profile: ActiveProfile,
    #[serde(default)]
    pub profiles: HashMap<String, ProfileDefinition>,
    #[serde(default)]
    pub checks: HashMap<String, CheckConfig>,
    /// External checker ids to run (e.g. `ruff`).
    #[serde(default)]
    pub external_checkers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActiveProfile {
    #[serde(default = "default_profile_name")]
    pub name: String,
}

impl Default for ActiveProfile {
    fn default() -> Self {
        Self {
            name: default_profile_name(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProfileDefinition {
    #[serde(default)]
    pub extends: Option<String>,
    #[serde(default)]
    pub checks: HashMap<String, CheckConfig>,
}

fn default_profile_name() -> String {
    "default".to_string()
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CheckConfig {
    #[serde(default)]
    pub enabled: Option<bool>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct ForbiddenImportRule {
    pub from: String,
    pub forbidden: String,
}

/// Resolved per-check settings after profile merge and built-in defaults.
#[derive(Debug, Clone)]
pub struct ResolvedCheckConfig {
    pub enabled: bool,
    pub max_branches: Option<usize>,
    pub max_arguments: Option<usize>,
    pub max_lines: Option<usize>,
    pub max_depth: Option<usize>,
    pub max_methods: Option<usize>,
    pub rules: Vec<ForbiddenImportRule>,
    pub severity: Option<Severity>,
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
        let config: Config =
            toml::from_str(&raw).with_context(|| format!("parse config {}", path.display()))?;
        config.validate()?;
        Ok(config)
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

    pub fn validate(&self) -> Result<()> {
        let valid_severities = ["low", "medium", "high", "critical"];
        for (id, cfg) in &self.checks {
            if !id.starts_with("ZR") || id.len() != 5 {
                bail!("unknown check key `{id}` in zerum.toml: expected catalog id like ZR001");
            }
            if let Some(sev) = &cfg.severity {
                let s = sev.to_string().to_ascii_lowercase();
                if !valid_severities.contains(&s.as_str()) {
                    bail!("invalid severity `{s}` for {id}: use low, medium, high, or critical");
                }
            }
        }
        for (name, profile) in &self.profiles {
            if let Some(ext) = &profile.extends {
                if ext == name {
                    bail!("profile `{name}` cannot extend itself");
                }
            }
            for (id, cfg) in &profile.checks {
                if let Some(sev) = &cfg.severity {
                    let s = sev.to_string().to_ascii_lowercase();
                    if !valid_severities.contains(&s.as_str()) {
                        bail!("invalid severity `{s}` for profile `{name}` check {id}");
                    }
                }
            }
        }
        self.validate_profile_chain()?;
        Ok(())
    }

    fn validate_profile_chain(&self) -> Result<()> {
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        self.visit_profile_chain(&self.profile.name, &mut visiting, &mut visited)
    }

    fn visit_profile_chain(
        &self,
        name: &str,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) -> Result<()> {
        if visited.contains(name) {
            return Ok(());
        }
        if !visiting.insert(name.to_string()) {
            bail!("profile inheritance cycle detected at `{name}`");
        }
        if let Some(def) = self.profiles.get(name) {
            if let Some(parent) = &def.extends {
                if !self.profiles.contains_key(parent) && parent != "default" && parent != "strict"
                {
                    bail!("profile `{name}` extends unknown profile `{parent}`");
                }
                self.visit_profile_chain(parent, visiting, visited)?;
            }
        }
        visiting.remove(name);
        visited.insert(name.to_string());
        Ok(())
    }

    pub fn is_check_enabled(&self, id: &str) -> bool {
        self.resolved_check_config(id).enabled
    }

    pub fn check_config(&self, id: &str) -> ResolvedCheckConfig {
        self.resolved_check_config(id)
    }

    fn resolved_check_config(&self, id: &str) -> ResolvedCheckConfig {
        let mut enabled = self.builtin_enabled_for_profile(id);
        let mut merged = CheckConfig::default();

        for def in self.profile_chain_defs().into_iter().flatten() {
            if let Some(cfg) = def.checks.get(id) {
                merge_check_config(&mut merged, cfg);
                if let Some(e) = cfg.enabled {
                    enabled = e;
                }
            }
        }

        if let Some(cfg) = self.checks.get(id) {
            merge_check_config(&mut merged, cfg);
            if let Some(e) = cfg.enabled {
                enabled = e;
            }
        }

        ResolvedCheckConfig {
            enabled,
            max_branches: merged.max_branches,
            max_arguments: merged.max_arguments,
            max_lines: merged.max_lines,
            max_depth: merged.max_depth,
            max_methods: merged.max_methods,
            rules: merged.rules,
            severity: merged.severity,
        }
    }

    fn builtin_enabled_for_profile(&self, id: &str) -> bool {
        match self.profile.name.as_str() {
            "strict" => true,
            "default" => !is_disabled_in_default_profile(id),
            _ => !is_disabled_in_default_profile(id),
        }
    }

    fn profile_chain_defs(&self) -> Vec<Option<&ProfileDefinition>> {
        let mut chain = Vec::new();
        let mut current = Some(self.profile.name.as_str());
        let mut seen = HashSet::new();
        while let Some(name) = current {
            if !seen.insert(name.to_string()) {
                break;
            }
            if name == "strict" {
                chain.push(None);
                current = Some("default");
                continue;
            }
            if name == "default" {
                chain.push(None);
                break;
            }
            if let Some(def) = self.profiles.get(name) {
                chain.push(Some(def));
                current = def.extends.as_deref();
            } else {
                break;
            }
        }
        chain.reverse();
        chain
    }
}

fn merge_check_config(target: &mut CheckConfig, source: &CheckConfig) {
    if source.max_branches.is_some() {
        target.max_branches = source.max_branches;
    }
    if source.max_arguments.is_some() {
        target.max_arguments = source.max_arguments;
    }
    if source.max_lines.is_some() {
        target.max_lines = source.max_lines;
    }
    if source.max_depth.is_some() {
        target.max_depth = source.max_depth;
    }
    if source.max_methods.is_some() {
        target.max_methods = source.max_methods;
    }
    if source.severity.is_some() {
        target.severity = source.severity;
    }
    if !source.rules.is_empty() {
        target.rules = source.rules.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn default_profile_disables_noisy_heuristics() {
        let config = Config::default();
        assert!(!config.is_check_enabled("ZR110"));
        assert!(!config.is_check_enabled("ZR506"));
        assert!(config.is_check_enabled("ZR001"));
        assert!(config.is_check_enabled("ZR105"));
    }

    #[test]
    fn strict_profile_enables_all_rules() {
        let config = Config {
            profile: ActiveProfile {
                name: "strict".to_string(),
            },
            ..Default::default()
        };
        assert!(config.is_check_enabled("ZR110"));
        assert!(config.is_check_enabled("ZR506"));
    }

    #[test]
    fn explicit_enable_overrides_default_profile() {
        let mut config = Config::default();
        config.checks.insert(
            "ZR110".to_string(),
            CheckConfig {
                enabled: Some(true),
                ..Default::default()
            },
        );
        assert!(config.is_check_enabled("ZR110"));
    }

    #[test]
    fn profile_inheritance_merges_checks() {
        let mut profiles = HashMap::new();
        let mut base_checks = HashMap::new();
        base_checks.insert(
            "ZR001".to_string(),
            CheckConfig {
                max_lines: Some(40),
                ..Default::default()
            },
        );
        profiles.insert(
            "base".to_string(),
            ProfileDefinition {
                extends: None,
                checks: base_checks,
            },
        );
        let mut team_checks = HashMap::new();
        team_checks.insert(
            "ZR001".to_string(),
            CheckConfig {
                max_lines: Some(60),
                ..Default::default()
            },
        );
        profiles.insert(
            "team".to_string(),
            ProfileDefinition {
                extends: Some("base".to_string()),
                checks: team_checks,
            },
        );
        let config = Config {
            profile: ActiveProfile {
                name: "team".to_string(),
            },
            profiles,
            ..Default::default()
        };
        assert_eq!(config.check_config("ZR001").max_lines, Some(60));
    }

    #[test]
    fn invalid_check_id_rejected_on_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("zerum.toml");
        fs::write(&path, "[checks.bad]\nenabled = true\n").unwrap();
        let err = Config::load(&path).unwrap_err();
        assert!(err.to_string().contains("unknown check key"));
    }

    #[test]
    fn discover_stops_at_project_root_without_config() {
        let outer = tempdir().unwrap();
        let project = outer.path().join("myapp");
        fs::create_dir_all(project.join("src")).unwrap();
        fs::write(project.join("pyproject.toml"), "[project]\nname = \"x\"\n").unwrap();
        fs::write(
            outer.path().join("zerum.toml"),
            "[profile]\nname = \"default\"\n",
        )
        .unwrap();

        let config = Config::discover(&project.join("src")).unwrap();
        assert!(config.checks.is_empty());
        assert!(!config.is_check_enabled("ZR110"));
    }
}
