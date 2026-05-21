use crate::core::ast_util::collect_import_modules;
use crate::core::{Category, Check, CheckContext, Issue, Severity};

pub struct ForbiddenArchitectureImport;

impl Check for ForbiddenArchitectureImport {
    fn id(&self) -> &'static str {
        "ZR010"
    }

    fn name(&self) -> &'static str {
        "forbidden-architecture-import"
    }

    fn category(&self) -> Category {
        Category::Architecture
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn explanation(&self) -> &'static str {
        "Importing across architectural boundaries couples layers and makes refactors risky."
    }

    fn remediation(&self) -> &'static str {
        "Introduce interfaces in the domain layer or move shared code to an allowed dependency direction."
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }
        let rules = ctx.config.check_config(self.id()).rules;
        if rules.is_empty() {
            return Vec::new();
        }

        let file = ctx.file_path().display().to_string();
        let mut issues = Vec::new();
        for (module, line, column) in collect_import_modules(ctx.parsed) {
            for rule in &rules {
                if file.contains(&rule.from.replace('.', std::path::MAIN_SEPARATOR_STR))
                    && module.starts_with(&rule.forbidden)
                {
                    issues.push(
                        Issue::deterministic(
                            self.id(),
                            self.name(),
                            format!(
                                "module `{}` must not import `{}` from `{}` context",
                                module, rule.forbidden, rule.from
                            ),
                            ctx.file_path(),
                            line,
                            column,
                            self.severity(),
                            self.category(),
                        )
                        .with_guidance(self.explanation(), self.remediation()),
                    );
                }
            }
        }
        issues
    }
}
