use crate::core::ast_util::collect_class_metrics;
use crate::core::{Check, CheckContext, Issue};

const DEFAULT_MAX_METHODS: usize = 15;

pub struct GodClass;

impl Check for GodClass {
    fn id(&self) -> &'static str {
        "ZR008"
    }

    fn name(&self) -> &'static str {
        "god-class"
    }

    fn category(&self) -> crate::core::Category {
        crate::core::Category::Design
    }

    fn severity(&self) -> crate::core::Severity {
        crate::core::Severity::Medium
    }

    fn explanation(&self) -> &'static str {
        "Classes with many methods often violate the single-responsibility principle and become change magnets."
    }

    fn remediation(&self) -> &'static str {
        "Split the class by responsibility, extract mixins or collaborators, and group related behavior."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }
        let max = ctx
            .config
            .check_config(self.id())
            .max_methods
            .unwrap_or(DEFAULT_MAX_METHODS);

        collect_class_metrics(ctx.parsed)
            .into_iter()
            .filter(|m| m.method_count > max)
            .map(|m| {
                Issue::from_check(
                    self,
                    format!(
                        "class `{}` defines {} methods (max {})",
                        m.name, m.method_count, max
                    ),
                    ctx.file_path(),
                    m.line,
                    m.column,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::parser::{PythonParser, RustPythonParser};
    use std::path::Path;

    fn method(n: usize) -> String {
        format!("    def m{n}(self): pass\n")
    }

    #[test]
    fn flags_god_class() {
        let mut source = String::from("class Big:\n");
        for i in 0..16 {
            source.push_str(&method(i));
        }
        let path = Path::new("t.py");
        let parsed = RustPythonParser.parse_file(&source, path).unwrap();
        let config = Config::default();
        let ctx = CheckContext {
            path,
            source: &source,
            parsed: &parsed,
            config: &config,
        };
        let issues = GodClass.run(&ctx);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id, "ZR008");
    }
}
