use crate::core::ast_util::collect_function_metrics;
use crate::core::{Check, CheckContext, Issue};

const DEFAULT_MAX_LINES: usize = 50;

pub struct LongFunction;

impl Check for LongFunction {
    fn id(&self) -> &'static str {
        "ZR003"
    }

    fn name(&self) -> &'static str {
        "long-function"
    }

    fn category(&self) -> crate::core::Category {
        crate::core::Category::Readability
    }

    fn severity(&self) -> crate::core::Severity {
        crate::core::Severity::Low
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
                Issue::from_check(
                    self,
                    format!(
                        "function `{}` spans ~{} lines (max {})",
                        m.name, m.body_lines, max
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
    fn flags_long_function() {
        let source = format!("def long_fn():\n{}\n", "    x = 1\n".repeat(55));
        let path = Path::new("t.py");
        let parsed = RustPythonParser.parse_file(&source, path).unwrap();
        let config = Config::default();
        let ctx = CheckContext {
            path,
            source: &source,
            parsed: &parsed,
            config: &config,
        };
        let issues = LongFunction.run(&ctx);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id, "ZR003");
    }

    #[test]
    fn short_function_is_clean() {
        let source = "def short():\n    return 1\n";
        let path = Path::new("t.py");
        let parsed = RustPythonParser.parse_file(source, path).unwrap();
        let config = Config::default();
        let ctx = CheckContext {
            path,
            source,
            parsed: &parsed,
            config: &config,
        };
        assert!(LongFunction.run(&ctx).is_empty());
    }
}
