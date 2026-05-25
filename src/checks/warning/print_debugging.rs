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
        "Use structured logging (logging module) or remove the statement before merging. \
         Phase 1 only flags bare `print()` calls, not `builtins.print` or import aliases."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }

        let mut issues = Vec::new();
        walk_exprs_in_module(ctx.parsed, &mut |expr| {
            if is_print_call(expr) {
                let (line, column) = line_col(&ctx.parsed.source, expr.start().into());
                issues.push(Issue::from_check(
                    self,
                    "print() used for debugging",
                    ctx.file_path(),
                    line,
                    column,
                ));
            }
        });
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::parser::{PythonParser, RustPythonParser};
    use std::path::Path;

    #[test]
    fn flags_print_call() {
        let source = "def f():\n    print('debug')\n";
        let path = Path::new("t.py");
        let parsed = RustPythonParser.parse_file(source, path).unwrap();
        let config = Config::default();
        let ctx = CheckContext {
            path,
            source,
            parsed: &parsed,
            config: &config,
        };
        assert_eq!(PrintDebugging.run(&ctx).len(), 1);
    }

    #[test]
    fn ignores_builtins_print_attribute() {
        let source = "def f():\n    builtins.print('debug')\n";
        let path = Path::new("t.py");
        let parsed = RustPythonParser.parse_file(source, path).unwrap();
        let config = Config::default();
        let ctx = CheckContext {
            path,
            source,
            parsed: &parsed,
            config: &config,
        };
        assert!(PrintDebugging.run(&ctx).is_empty());
    }
}
