use crate::core::ast_util::collect_function_metrics;
use crate::core::{Category, Check, CheckContext, Issue, Severity};

const DEFAULT_MAX_LINES: usize = 50;

pub struct LongFunction;

impl Check for LongFunction {
    fn id(&self) -> &'static str {
        "ZR003"
    }

    fn name(&self) -> &'static str {
        "long-function"
    }

    fn category(&self) -> Category {
        Category::Readability
    }

    fn severity(&self) -> Severity {
        Severity::Low
    }

    fn explanation(&self) -> &'static str {
        "Long functions mix multiple responsibilities and are harder to test and review."
    }

    fn remediation(&self) -> &'static str {
        "Extract cohesive blocks into private helpers with descriptive names."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }
        let max = ctx
            .config
            .check_config(self.id())
            .max_lines
            .unwrap_or(DEFAULT_MAX_LINES);

        collect_function_metrics(ctx.parsed)
            .into_iter()
            .filter(|m| m.body_lines > max)
            .map(|m| {
                Issue::deterministic(
                    self.id(),
                    self.name(),
                    format!(
                        "function `{}` spans ~{} lines (max {})",
                        m.name, m.body_lines, max
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
