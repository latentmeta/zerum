use crate::core::ast_util::collect_import_modules;
use crate::core::{Category, Check, CheckContext, Issue, Severity};
use std::path::Path;

pub struct ForbiddenArchitectureImport;

impl Check for ForbiddenArchitectureImport {
    fn id(&self) -> &'static str {
        "ZR010"
    }

    fn name(&self) -> &'static str {
        "forbidden-architecture-import"
    }

    fn category(&self) -> Category {
        Category::Architecture
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn explanation(&self) -> &'static str {
        "Importing across architectural boundaries couples layers and makes refactors risky."
    }

    fn remediation(&self) -> &'static str {
        "Introduce interfaces in the domain layer or move shared code to an allowed dependency direction."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }
        let rules = ctx.config.check_config(self.id()).rules;
        if rules.is_empty() {
            return Vec::new();
        }

        let file = ctx.path;
        let mut issues = Vec::new();
        for (module, line, column) in collect_import_modules(ctx.parsed) {
            if module.is_empty() {
                continue;
            }
            for rule in &rules {
                if path_contains_layer(file, &rule.from)
                    && module_matches_forbidden(&module, &rule.forbidden)
                {
                    issues.push(
                        Issue::deterministic(
                            self.id(),
                            self.name(),
                            format!(
                                "module `{module}` must not be imported from `{from}` layer",
                                from = rule.from
                            ),
                            ctx.file_path(),
                            line,
                            column,
                            self.severity(),
                            self.category(),
                        )
                        .with_guidance(self.explanation(), self.remediation()),
                    );
                }
            }
        }
        issues
    }
}

fn path_contains_layer(path: &Path, layer: &str) -> bool {
    let layer_parts: Vec<&str> = layer.split('.').collect();
    if layer_parts.is_empty() {
        return false;
    }
    let components: Vec<&str> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    components
        .windows(layer_parts.len())
        .any(|window| window == layer_parts.as_slice())
}

fn module_matches_forbidden(module: &str, forbidden: &str) -> bool {
    module == forbidden || module.starts_with(&format!("{forbidden}."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::parser::{PythonParser, RustPythonParser};
    use std::path::Path;

    #[test]
    fn flags_forbidden_import_in_domain_layer() {
        let source = "from app.infrastructure.db import connect\n";
        let path = Path::new("app/domain/service.py");
        let parsed = RustPythonParser.parse_file(source, path).unwrap();
        let mut config = Config::default();
        config.checks.insert(
            "ZR010".into(),
            crate::config::CheckConfig {
                enabled: true,
                rules: vec![crate::config::ForbiddenImportRule {
                    from: "app.domain".into(),
                    forbidden: "app.infrastructure".into(),
                }],
                ..Default::default()
            },
        );
        let ctx = CheckContext {
            path,
            source,
            parsed: &parsed,
            config: &config,
        };
        let issues = ForbiddenArchitectureImport.run(&ctx);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id, "ZR010");
    }

    #[test]
    fn does_not_match_prefix_extension() {
        assert!(!module_matches_forbidden(
            "app.infrastructure_extra",
            "app.infrastructure"
        ));
    }
}
