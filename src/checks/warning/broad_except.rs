use crate::core::ast_util::walk_stmts_for_try;
use crate::core::{Category, Check, CheckContext, Issue, Severity};
use crate::parser::line_col;
use rustpython_ast::{ExceptHandler, Ranged};

pub struct BroadExcept;

impl Check for BroadExcept {
    fn id(&self) -> &'static str {
        "ZR005"
    }

    fn name(&self) -> &'static str {
        "broad-except"
    }

    fn category(&self) -> Category {
        Category::Warning
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn explanation(&self) -> &'static str {
        "Catching Exception or using bare except hides bugs and makes failures silent."
    }

    fn remediation(&self) -> &'static str {
        "Catch specific exception types and re-raise or log unexpected errors."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }

        let mut issues = Vec::new();
        walk_stmts_for_try(ctx.parsed, &mut |handler| {
            if crate::core::ast_util::is_broad_except_handler(handler) {
                let ExceptHandler::ExceptHandler(h) = handler;
                let (line, column) = line_col(&ctx.parsed.source, h.start().into());
                issues.push(Issue::from_check(
                    self,
                    "broad or bare except handler swallows errors",
                    ctx.file_path(),
                    line,
                    column,
                ));
            }
        });
        issues
    }
}
