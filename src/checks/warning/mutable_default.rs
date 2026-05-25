use crate::core::ast_util::{has_mutable_defaults, walk_function_defs};
use crate::core::{Category, Check, CheckContext, Issue, Severity};
use crate::parser::line_col;

pub struct MutableDefaultArgument;

impl Check for MutableDefaultArgument {
    fn id(&self) -> &'static str {
        "ZR007"
    }

    fn name(&self) -> &'static str {
        "mutable-default-argument"
    }

    fn category(&self) -> Category {
        Category::Warning
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn explanation(&self) -> &'static str {
        "Mutable default arguments are shared across calls and cause subtle state bugs."
    }

    fn remediation(&self) -> &'static str {
        "Use None as the default and initialize the mutable value inside the function body."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }

        let mut issues = Vec::new();
        walk_function_defs(ctx.parsed, &mut |args, start, name| {
            if has_mutable_defaults(args) {
                let (line, column) = line_col(&ctx.parsed.source, start.into());
                issues.push(Issue::from_check(
                    self,
                    format!("function `{name}` uses a mutable default argument"),
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
    fn flags_list_default() {
        let source = "def f(items=[]):\n    return items\n";
        let path = Path::new("t.py");
        let parsed = RustPythonParser.parse_file(source, path).unwrap();
        let config = Config::default();
        let ctx = CheckContext {
            path,
            source,
            parsed: &parsed,
            config: &config,
        };
        assert_eq!(MutableDefaultArgument.run(&ctx).len(), 1);
    }

    #[test]
    fn none_default_is_clean() {
        let source = "def f(items=None):\n    return items\n";
        let path = Path::new("t.py");
        let parsed = RustPythonParser.parse_file(source, path).unwrap();
        let config = Config::default();
        let ctx = CheckContext {
            path,
            source,
            parsed: &parsed,
            config: &config,
        };
        assert!(MutableDefaultArgument.run(&ctx).is_empty());
    }
}
