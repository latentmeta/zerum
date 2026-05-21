use crate::core::ast_util::{is_print_call, walk_exprs_in_module};
use crate::core::{Category, Check, CheckContext, Issue, Severity};
use crate::parser::line_col;
use rustpython_ast::Ranged;

pub struct PrintDebugging;

impl Check for PrintDebugging {
    fn id(&self) -> &'static str {
        "ZR006"
    }

    fn name(&self) -> &'static str {
        "print-debugging"
    }

    fn category(&self) -> Category {
        Category::Warning
    }

    fn severity(&self) -> Severity {
        Severity::Low
    }

    fn explanation(&self) -> &'static str {
        "print() calls left in production code are noisy and usually indicate leftover debugging."
    }

    fn remediation(&self) -> &'static str {
        "Use structured logging (logging module) or remove the statement before merging."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }

        let mut issues = Vec::new();
        walk_exprs_in_module(ctx.parsed, &mut |expr| {
            if is_print_call(expr) {
                let (line, column) = line_col(&ctx.parsed.source, expr.start().into());
                issues.push(
                    Issue::deterministic(
                        self.id(),
                        self.name(),
                        "print() used for debugging",
                        ctx.file_path(),
                        line,
                        column,
                        self.severity(),
                        self.category(),
                    )
                    .with_guidance(self.explanation(), self.remediation()),
                );
            }
        });
        issues
    }
}
