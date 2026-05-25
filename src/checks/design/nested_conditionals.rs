use crate::core::ast_util::collect_function_metrics;
use crate::core::{Category, Check, CheckContext, Issue, Severity};

const DEFAULT_MAX_DEPTH: usize = 3;

pub struct NestedConditionals;

impl Check for NestedConditionals {
    fn id(&self) -> &'static str {
        "ZR004"
    }

    fn name(&self) -> &'static str {
        "nested-conditionals"
    }

    fn category(&self) -> Category {
        Category::Design
    }

    fn severity(&self) -> Severity {
        Severity::Medium
    }

    fn explanation(&self) -> &'static str {
        "Deeply nested conditionals increase cognitive load and hide the happy path."
    }

    fn remediation(&self) -> &'static str {
        "Flatten with guard clauses, extract nested blocks into named functions, or restructure control flow."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }
        let max = ctx
            .config
            .check_config(self.id())
            .max_depth
            .unwrap_or(DEFAULT_MAX_DEPTH);

        collect_function_metrics(ctx.parsed)
            .into_iter()
            .filter(|m| m.max_conditional_depth > max)
            .map(|m| {
                Issue::from_check(
                    self,
                    format!(
                        "function `{}` has conditional nesting depth {} (max {})",
                        m.name, m.max_conditional_depth, max
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

    #[test]
    fn flags_deep_nesting() {
        let source = r#"
def deep():
    if a:
        if b:
            if c:
                if d:
                    pass
"#;
        let path = Path::new("t.py");
        let parsed = RustPythonParser.parse_file(source, path).unwrap();
        let config = Config::default();
        let ctx = CheckContext {
            path,
            source,
            parsed: &parsed,
            config: &config,
        };
        let issues = NestedConditionals.run(&ctx);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id, "ZR004");
    }

    #[test]
    fn async_function_depth_counted() {
        let source = r#"
async def deep():
    if a:
        if b:
            if c:
                if d:
                    pass
"#;
        let path = Path::new("t.py");
        let parsed = RustPythonParser.parse_file(source, path).unwrap();
        let config = Config::default();
        let ctx = CheckContext {
            path,
            source,
            parsed: &parsed,
            config: &config,
        };
        assert_eq!(NestedConditionals.run(&ctx).len(), 1);
    }
}
