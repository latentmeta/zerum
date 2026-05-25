use crate::core::ast_util::collect_function_metrics;
use crate::core::{Category, Check, CheckContext, Issue, Severity};

const DEFAULT_MAX: usize = 8;

pub struct TooManyBranches;

impl Check for TooManyBranches {
    fn id(&self) -> &'static str {
        "ZR001"
    }

    fn name(&self) -> &'static str {
        "too-many-branches"
    }

    fn category(&self) -> Category {
        Category::Design
    }

    fn severity(&self) -> Severity {
        Severity::Medium
    }

    fn explanation(&self) -> &'static str {
        "Functions with many branches are harder to test and reason about. Each branch is a separate path through the function."
    }

    fn remediation(&self) -> &'static str {
        "Extract helper functions, use early returns, or replace nested conditionals with polymorphism or lookup tables."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }
        let max = ctx
            .config
            .check_config(self.id())
            .max_branches
            .unwrap_or(DEFAULT_MAX);

        collect_function_metrics(ctx.parsed)
            .into_iter()
            .filter(|m| m.branch_count > max)
            .map(|m| {
                Issue::from_check(
                    self,
                    format!(
                        "function `{}` has {} branches (max {})",
                        m.name, m.branch_count, max
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

    fn run_check(source: &str) -> Vec<Issue> {
        let parser = RustPythonParser;
        let path = Path::new("sample.py");
        let parsed = parser.parse_file(source, path).unwrap();
        let config = Config::default();
        let check = TooManyBranches;
        let ctx = CheckContext {
            path,
            source,
            parsed: &parsed,
            config: &config,
        };
        check.run(&ctx)
    }

    #[test]
    fn flags_branchy_function() {
        let source = r#"
def branchy(x):
    if x > 0:
        pass
    elif x > 1:
        pass
    elif x > 2:
        pass
    elif x > 3:
        pass
    elif x > 4:
        pass
    elif x > 5:
        pass
    elif x > 6:
        pass
    elif x > 7:
        pass
    elif x > 8:
        pass
    while x:
        for i in range(3):
            if i:
                pass
"#;
        let issues = run_check(source);
        assert!(!issues.is_empty());
        assert!(issues.iter().all(|i| i.id == "ZR001"));
    }

    #[test]
    fn async_function_branches_counted() {
        let source = r#"
async def branchy(x):
    if x:
        pass
    elif x > 1:
        pass
    elif x > 2:
        pass
    elif x > 3:
        pass
    elif x > 4:
        pass
    elif x > 5:
        pass
    elif x > 6:
        pass
    elif x > 7:
        pass
    elif x > 8:
        pass
"#;
        assert!(!run_check(source).is_empty());
    }
}
