use crate::core::ast_util::collect_class_metrics;
use crate::core::{Category, Check, CheckContext, Issue, Severity};

const DEFAULT_MAX_METHODS: usize = 15;

pub struct GodClass;

impl Check for GodClass {
    fn id(&self) -> &'static str {
        "ZR008"
    }

    fn name(&self) -> &'static str {
        "god-class"
    }

    fn category(&self) -> Category {
        Category::Design
    }

    fn severity(&self) -> Severity {
        Severity::Medium
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
                Issue::deterministic(
                    self.id(),
                    self.name(),
                    format!(
                        "class `{}` defines {} methods (max {})",
                        m.name, m.method_count, max
                    ),
                    ctx.file_path(),
                    m.line,
                    m.column,
                    self.severity(),
                    self.category(),
                )
                .with_guidance(self.explanation(), self.remediation())
            })
            .collect()
    }
}
