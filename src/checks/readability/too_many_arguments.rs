use crate::core::ast_util::collect_function_metrics;
use crate::core::{Category, Check, CheckContext, Issue, Severity};

const DEFAULT_MAX_ARGS: usize = 5;

pub struct TooManyArguments;

impl Check for TooManyArguments {
    fn id(&self) -> &'static str {
        "ZR002"
    }

    fn name(&self) -> &'static str {
        "too-many-arguments"
    }

    fn category(&self) -> Category {
        Category::Readability
    }

    fn severity(&self) -> Severity {
        Severity::Low
    }

    fn explanation(&self) -> &'static str {
        "Functions with many parameters are difficult to call correctly and often indicate missing abstractions."
    }

    fn remediation(&self) -> &'static str {
        "Group related parameters into a dataclass or config object, or split the function."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }
        let max = ctx
            .config
            .check_config(self.id())
            .max_arguments
            .unwrap_or(DEFAULT_MAX_ARGS);

        collect_function_metrics(ctx.parsed)
            .into_iter()
            .filter(|m| m.arg_count > max)
            .map(|m| {
                Issue::deterministic(
                    self.id(),
                    self.name(),
                    format!(
                        "function `{}` has {} arguments (max {})",
                        m.name, m.arg_count, max
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
