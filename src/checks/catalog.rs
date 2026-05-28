use super::catalog_detectors::{self, CatalogDetector as PreciseDetector};
use crate::config::Config;
use crate::core::ast_util::{
    has_mutable_defaults, is_broad_except_handler, is_print_call, walk_exprs_in_module,
    walk_function_defs, walk_stmts_for_try, walk_stmts_in_module,
};
use crate::core::{Category, Check, CheckContext, CheckMetadata, Issue, Severity};
use crate::parser::line_col;
use rustpython_ast::{CmpOp, Constant, ExceptHandler, Expr, Ranged, Stmt};

#[derive(Debug, Clone, Copy)]
enum Detector {
    LongFunction(usize),
    TooManyArguments(usize),
    DeepNesting(usize),
    GodClass(usize),
    ComplexBooleanExpression(usize),
    MagicNumber,
    TodoWithoutContext,
    AssertProduction,
    DangerousEvalExec,
    LengthComparison,
    PatternAny(&'static [&'static str]),
    PatternComment(&'static [&'static str]),
    MissingModuleDocstring,
    MissingFunctionDocstring,
    BroadExcept,
    BareExcept,
    MutableDefaultArgument,
    PrintDebugging,
    ForbiddenArchitectureImport,
    Precise(PreciseDetector),
}

pub struct CatalogCheck {
    pub metadata: CheckMetadata,
    pub explanation: &'static str,
    pub remediation: &'static str,
    pub false_positives: &'static str,
    pub tradeoffs: &'static str,
    detector: Detector,
}

impl Check for CatalogCheck {
    fn metadata(&self) -> CheckMetadata {
        self.metadata
    }

    fn id(&self) -> &'static str {
        self.metadata.id
    }

    fn name(&self) -> &'static str {
        self.metadata.name
    }

    fn category(&self) -> Category {
        self.metadata.category
    }

    fn severity(&self) -> Severity {
        self.metadata.severity
    }

    fn explanation(&self) -> &'static str {
        self.explanation
    }

    fn remediation(&self) -> &'static str {
        self.remediation
    }

    fn false_positives(&self) -> &'static str {
        self.false_positives
    }

    fn tradeoffs(&self) -> &'static str {
        self.tradeoffs
    }

    fn run(&self, ctx: &CheckContext) -> Vec<Issue> {
        if !ctx.config.is_check_enabled(self.id()) {
            return Vec::new();
        }
        match self.detector {
            Detector::LongFunction(default) => {
                let max = ctx
                    .config
                    .check_config(self.id())
                    .max_lines
                    .unwrap_or(default);
                ctx.source_model()
                    .functions()
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
            Detector::TooManyArguments(default) => {
                let max = ctx
                    .config
                    .check_config(self.id())
                    .max_arguments
                    .unwrap_or(default);
                ctx.source_model()
                    .functions()
                    .into_iter()
                    .filter(|m| m.arg_count > max)
                    .map(|m| {
                        Issue::from_check(
                            self,
                            format!(
                                "function `{}` has {} arguments (max {})",
                                m.name, m.arg_count, max
                            ),
                            ctx.file_path(),
                            m.line,
                            m.column,
                        )
                    })
                    .collect()
            }
            Detector::DeepNesting(default) => {
                let max = ctx
                    .config
                    .check_config(self.id())
                    .max_depth
                    .unwrap_or(default);
                ctx.source_model()
                    .functions()
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
            Detector::GodClass(default) => {
                let max = ctx
                    .config
                    .check_config(self.id())
                    .max_methods
                    .unwrap_or(default);
                ctx.source_model()
                    .classes()
                    .into_iter()
                    .filter(|m| m.method_count > max)
                    .map(|m| {
                        Issue::from_check(
                            self,
                            format!(
                                "class `{}` defines {} methods (max {})",
                                m.name, m.method_count, max
                            ),
                            ctx.file_path(),
                            m.line,
                            m.column,
                        )
                    })
                    .collect()
            }
            Detector::ComplexBooleanExpression(max_ops) => {
                let mut issues = Vec::new();
                walk_exprs_in_module(ctx.parsed, &mut |expr| {
                    if let Expr::BoolOp(b) = expr {
                        let ops = b.values.len().saturating_sub(1);
                        if ops > max_ops {
                            let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                            issues.push(Issue::from_check(
                                self,
                                format!(
                                    "boolean expression has {} operators (max {})",
                                    ops, max_ops
                                ),
                                ctx.file_path(),
                                line,
                                col,
                            ));
                        }
                    }
                });
                issues
            }
            Detector::MagicNumber => {
                let mut issues = Vec::new();
                walk_exprs_in_module(ctx.parsed, &mut |expr| {
                    if let Expr::Constant(c) = expr {
                        if let Some(text) = numeric_constant_text(&c.value) {
                            if !is_magic_numeric_literal(&text) {
                                return;
                            }
                            let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                            issues.push(Issue::from_check(
                                self,
                                format!("magic numeric literal `{text}`"),
                                ctx.file_path(),
                                line,
                                col,
                            ));
                        }
                    }
                });
                issues
            }
            Detector::TodoWithoutContext => {
                let mut issues = Vec::new();
                for comment in ctx.source_model().comments() {
                    let upper = comment.text.to_ascii_uppercase();
                    if upper.contains("TODO")
                        && !upper.contains("TODO(")
                        && !upper.contains("TODO:")
                        && !upper.contains("TODO[")
                    {
                        issues.push(Issue::from_check(
                            self,
                            "TODO comment without owner/context (use TODO(owner): ...)",
                            ctx.file_path(),
                            comment.line,
                            comment.column,
                        ));
                    }
                }
                issues
            }
            Detector::AssertProduction => {
                let mut issues = Vec::new();
                walk_stmts_in_module(ctx.parsed, &mut |stmt| {
                    if let Stmt::Assert(a) = stmt {
                        let (line, col) = line_col(&ctx.parsed.source, a.start().into());
                        issues.push(Issue::from_check(
                            self,
                            "assert statement may be stripped in optimized mode",
                            ctx.file_path(),
                            line,
                            col,
                        ));
                    }
                });
                issues
            }
            Detector::DangerousEvalExec => {
                let mut issues = Vec::new();
                walk_exprs_in_module(ctx.parsed, &mut |expr| {
                    if let Expr::Call(c) = expr {
                        if let Expr::Name(n) = c.func.as_ref() {
                            if n.id.as_str() == "eval" || n.id.as_str() == "exec" {
                                let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                                issues.push(Issue::from_check(
                                    self,
                                    format!("dangerous dynamic execution via {}()", n.id),
                                    ctx.file_path(),
                                    line,
                                    col,
                                ));
                            }
                        }
                    }
                });
                issues
            }
            Detector::LengthComparison => {
                let mut issues = Vec::new();
                walk_exprs_in_module(ctx.parsed, &mut |expr| {
                    if let Expr::Compare(c) = expr {
                        if c.ops.is_empty() {
                            return;
                        }
                        let left_is_len = is_len_call(&c.left);
                        let right_is_len = c.comparators.iter().any(is_len_call);
                        let compares_zero_or_one = c
                            .comparators
                            .iter()
                            .any(|e| matches_constant_number(e, &["0", "1"]));
                        let op_suspicious = c.ops.iter().any(|op| {
                            matches!(
                                op,
                                CmpOp::Gt | CmpOp::Lt | CmpOp::Eq | CmpOp::GtE | CmpOp::LtE
                            )
                        });
                        if (left_is_len || right_is_len) && compares_zero_or_one && op_suspicious {
                            let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                            issues.push(Issue::from_check(
                                self,
                                "len(...) comparison can often be simplified to truthiness check",
                                ctx.file_path(),
                                line,
                                col,
                            ));
                        }
                    }
                });
                issues
            }
            Detector::PatternAny(patterns) => run_pattern_check(self, ctx, patterns, false),
            Detector::PatternComment(patterns) => run_pattern_comment_check(self, ctx, patterns),
            Detector::MissingModuleDocstring => {
                if has_module_docstring(ctx.source) {
                    Vec::new()
                } else {
                    vec![Issue::from_check(
                        self,
                        "module is missing a top-level docstring",
                        ctx.file_path(),
                        1,
                        1,
                    )]
                }
            }
            Detector::MissingFunctionDocstring => ctx
                .source_model()
                .functions()
                .into_iter()
                .filter_map(|m| {
                    let needle = format!("def {}", m.name);
                    let line_idx = ctx.source.lines().position(|l| l.contains(&needle))?;
                    let next = ctx
                        .source
                        .lines()
                        .nth(line_idx + 1)
                        .unwrap_or_default()
                        .trim();
                    if next.starts_with("\"\"\"") || next.starts_with("'''") {
                        None
                    } else {
                        Some(Issue::from_check(
                            self,
                            format!("function `{}` is missing a docstring", m.name),
                            ctx.file_path(),
                            m.line,
                            m.column,
                        ))
                    }
                })
                .collect(),
            Detector::BroadExcept => {
                let mut issues = Vec::new();
                walk_stmts_for_try(ctx.parsed, &mut |handler| {
                    if is_broad_except_handler(handler) {
                        let ExceptHandler::ExceptHandler(h) = handler;
                        let (line, col) = line_col(&ctx.parsed.source, h.start().into());
                        issues.push(Issue::from_check(
                            self,
                            "broad except handler catches Exception/BaseException",
                            ctx.file_path(),
                            line,
                            col,
                        ));
                    }
                });
                issues
            }
            Detector::BareExcept => {
                let mut issues = Vec::new();
                walk_stmts_for_try(ctx.parsed, &mut |handler| {
                    let ExceptHandler::ExceptHandler(h) = handler;
                    if h.type_.is_none() {
                        let (line, col) = line_col(&ctx.parsed.source, h.start().into());
                        issues.push(Issue::from_check(
                            self,
                            "bare except handler swallows all exceptions",
                            ctx.file_path(),
                            line,
                            col,
                        ));
                    }
                });
                issues
            }
            Detector::MutableDefaultArgument => {
                let mut issues = Vec::new();
                walk_function_defs(ctx.parsed, &mut |args, start, name| {
                    if has_mutable_defaults(args) {
                        let (line, col) = line_col(&ctx.parsed.source, start.into());
                        issues.push(Issue::from_check(
                            self,
                            format!("function `{name}` uses a mutable default argument"),
                            ctx.file_path(),
                            line,
                            col,
                        ));
                    }
                });
                issues
            }
            Detector::PrintDebugging => {
                let mut issues = Vec::new();
                walk_exprs_in_module(ctx.parsed, &mut |expr| {
                    if is_print_call(expr) {
                        let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                        issues.push(Issue::from_check(
                            self,
                            "print() used for debugging",
                            ctx.file_path(),
                            line,
                            col,
                        ));
                    }
                });
                issues
            }
            Detector::ForbiddenArchitectureImport => {
                run_forbidden_arch_import(self, ctx, ctx.config)
            }
            Detector::Precise(det) => catalog_detectors::run_detector(self, ctx, det),
        }
    }
}

fn run_pattern_check(
    check: &CatalogCheck,
    ctx: &CheckContext,
    patterns: &[&str],
    comments_only: bool,
) -> Vec<Issue> {
    let mut issues = Vec::new();
    for (idx, line) in ctx.source.lines().enumerate() {
        let haystack = if comments_only {
            match line.find('#') {
                Some(i) => &line[i..],
                None => continue,
            }
        } else {
            line
        };
        let upper = haystack.to_ascii_uppercase();
        for p in patterns {
            if upper.contains(&p.to_ascii_uppercase()) {
                let col = line
                    .to_ascii_uppercase()
                    .find(&p.to_ascii_uppercase())
                    .map_or(1, |c| c + 1);
                issues.push(Issue::from_check(
                    check,
                    format!("matches pattern `{p}`"),
                    ctx.file_path(),
                    idx + 1,
                    col,
                ));
                break;
            }
        }
    }
    issues
}

fn run_pattern_comment_check(
    check: &CatalogCheck,
    ctx: &CheckContext,
    patterns: &[&str],
) -> Vec<Issue> {
    let mut issues = Vec::new();
    for comment in ctx.source_model().comments() {
        let upper = comment.text.to_ascii_uppercase();
        for p in patterns {
            if upper.contains(&p.to_ascii_uppercase()) {
                issues.push(Issue::from_check(
                    check,
                    format!("matches pattern `{p}`"),
                    ctx.file_path(),
                    comment.line,
                    comment.column,
                ));
                break;
            }
        }
    }
    issues
}

fn has_module_docstring(source: &str) -> bool {
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        return trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''");
    }
    false
}

fn run_forbidden_arch_import(
    check: &CatalogCheck,
    ctx: &CheckContext,
    config: &Config,
) -> Vec<Issue> {
    let rules = &config.check_config(check.id()).rules;
    if rules.is_empty() {
        return Vec::new();
    }
    let mut issues = Vec::new();
    for import in ctx.source_model().imports() {
        for rule in rules {
            if path_contains_layer(ctx.path, &rule.from)
                && (import.module == rule.forbidden
                    || import.module.starts_with(&format!("{}.", rule.forbidden)))
            {
                issues.push(Issue::from_check(
                    check,
                    format!(
                        "module `{module}` must not be imported from `{from}` layer",
                        module = import.module,
                        from = rule.from
                    ),
                    ctx.file_path(),
                    import.line,
                    import.column,
                ));
            }
        }
    }
    issues
}

fn path_contains_layer(path: &std::path::Path, layer: &str) -> bool {
    let parts: Vec<&str> = layer.split('.').collect();
    let components: Vec<&str> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    components
        .windows(parts.len())
        .any(|w| w == parts.as_slice())
}

fn is_magic_numeric_literal(text: &str) -> bool {
    let cleaned = text.trim();
    if let Some(stripped) = cleaned.strip_prefix('-') {
        return is_magic_numeric_literal(stripped);
    }
    !matches!(
        cleaned,
        "0" | "1" | "2" | "10" | "0.0" | "1.0" | "2.0" | "10.0"
    ) && cleaned.chars().all(|c| c.is_ascii_digit() || c == '.')
}

fn is_len_call(expr: &Expr) -> bool {
    if let Expr::Call(c) = expr {
        if let Expr::Name(n) = c.func.as_ref() {
            return n.id.as_str() == "len";
        }
    }
    false
}

fn matches_constant_number(expr: &Expr, allowed: &[&str]) -> bool {
    if let Expr::Constant(c) = expr {
        if let Some(txt) = numeric_constant_text(&c.value) {
            return allowed.contains(&txt.as_str());
        }
    }
    false
}

fn numeric_constant_text(c: &Constant) -> Option<String> {
    match c {
        Constant::Int(i) => Some(i.to_string()),
        Constant::Float(f) => Some(f.to_string()),
        _ => None,
    }
}

struct RuleDocs {
    explanation: &'static str,
    remediation: &'static str,
    false_positives: &'static str,
    tradeoffs: &'static str,
    examples: &'static str,
}

const GENERIC_DOCS: RuleDocs = RuleDocs {
    explanation: "This finding highlights maintainability risk in deterministic analysis.",
    remediation: "Refactor for clarity and consistency with project conventions.",
    false_positives: "Heuristic rule: validate in project context before enforcing strictly.",
    tradeoffs: "Stricter thresholds can improve consistency but may increase review churn.",
    examples: "See `zerum explain <ID>` for examples.",
};

fn rule(
    id: &'static str,
    name: &'static str,
    category: Category,
    severity: Severity,
    detector: Detector,
) -> Box<dyn Check> {
    rule_with_docs(
        id,
        name,
        category,
        severity,
        detector,
        docs_for(id).unwrap_or(GENERIC_DOCS),
    )
}

fn rule_with_docs(
    id: &'static str,
    name: &'static str,
    category: Category,
    severity: Severity,
    detector: Detector,
    docs: RuleDocs,
) -> Box<dyn Check> {
    Box::new(CatalogCheck {
        metadata: CheckMetadata {
            id,
            name,
            category,
            severity,
            safe_fixable: false,
            examples: docs.examples,
        },
        explanation: docs.explanation,
        remediation: docs.remediation,
        false_positives: docs.false_positives,
        tradeoffs: docs.tradeoffs,
        detector,
    })
}

fn docs_for(id: &str) -> Option<RuleDocs> {
    Some(match id {
        "ZR001" => RuleDocs {
            explanation: "Long functions are harder to test, review, and reuse.",
            remediation: "Extract helpers or split responsibilities into smaller functions.",
            false_positives: "Generated code, DSL builders, or single-purpose scripts.",
            tradeoffs: "Lower thresholds catch sprawl early but may conflict with local cohesion.",
            examples: "A 120-line handler doing validation, IO, and formatting in one block.",
        },
        "ZR003" => RuleDocs {
            explanation: "Deep nesting obscures control flow and error handling paths.",
            remediation: "Use guard clauses, early returns, or extract nested blocks into functions.",
            false_positives: "State machines or parser combinator style code with intentional depth.",
            tradeoffs: "Flattening can scatter related logic if over-applied.",
            examples: "Four levels of if/for inside one function.",
        },
        "ZR005" => RuleDocs {
            explanation: "Very short names reduce readability for maintainers.",
            remediation: "Rename to descriptive snake_case identifiers.",
            false_positives: "Loop indices, math conventions (x, y), or well-known abbreviations in scope.",
            tradeoffs: "Verbose names help newcomers but can clutter tight algorithms.",
            examples: "`def f(a, b):` where names do not convey intent.",
        },
        "ZR010" => RuleDocs {
            explanation: "TODOs without owner or context tend to linger indefinitely.",
            remediation: "Add owner, ticket id, or concrete next step; remove stale TODOs.",
            false_positives: "Template files or internal scratch scripts.",
            tradeoffs: "Strict TODO hygiene improves tracking but adds friction for spikes.",
            examples: "`# TODO fix this` with no ticket or owner.",
        },
        "ZR207" => RuleDocs {
            explanation: "Layer violations let infrastructure concerns leak into domain code.",
            remediation: "Move imports behind interfaces or invert dependencies per your architecture rules.",
            false_positives: "Monorepos with unconventional folder layouts; adjust `[[checks.ZR207.rules]]`.",
            tradeoffs: "Strict layering improves testability but requires upfront boundary design.",
            examples: "Domain module importing `app.infrastructure.db`.",
        },
        "ZR401" => RuleDocs {
            explanation: "Broad except handlers hide bugs and make failures hard to diagnose.",
            remediation: "Catch specific exception types and re-raise or wrap with context.",
            false_positives: "Top-level CLI entrypoints that must report any failure.",
            tradeoffs: "Narrow exceptions improve signal but need maintenance when APIs change.",
            examples: "`except Exception:` around business logic.",
        },
        "ZR402" => RuleDocs {
            explanation: "Empty except blocks swallow errors silently.",
            remediation: "Log, handle, or re-raise; remove no-op handlers.",
            false_positives: "Intentional suppression with documented rationale in comments.",
            tradeoffs: "Explicit handling adds noise if every rare edge case is logged.",
            examples: "`except ValueError: pass`",
        },
        "ZR403" => RuleDocs {
            explanation: "Bare except catches system exits and keyboard interrupts.",
            remediation: "Specify `except Exception` at minimum, preferably concrete types.",
            false_positives: "Legacy compatibility shims (still discouraged).",
            tradeoffs: "Bare except is convenient for quick scripts but unsafe in libraries.",
            examples: "`except:` with no exception type.",
        },
        "ZR404" => RuleDocs {
            explanation: "Mutable defaults are shared across calls and cause subtle bugs.",
            remediation: "Use `None` as default and assign a new list/dict inside the function.",
            false_positives: "Immutable defaults (None, int, str, tuple) are fine.",
            tradeoffs: "The None-sentinel pattern is idiomatic but slightly more verbose.",
            examples: "`def f(items=[]):`",
        },
        "ZR406" => RuleDocs {
            explanation: "Assert statements are stripped when Python runs with `-O`.",
            remediation: "Use explicit validation and raise proper exceptions in production paths.",
            false_positives: "Test modules and type-checking-only blocks.",
            tradeoffs: "Asserts are concise for invariants but unsafe as production guards.",
            examples: "`assert user.is_active` in application code.",
        },
        "ZR407" => RuleDocs {
            explanation: "`eval` and `exec` execute arbitrary code and are major security risks.",
            remediation: "Use safe parsers, AST literals, or restricted DSLs instead.",
            false_positives: "Controlled REPL tooling with explicit operator approval.",
            tradeoffs: "Dynamic execution is flexible but almost never worth the risk in apps.",
            examples: "`eval(user_input)`",
        },
        "ZR414" => RuleDocs {
            explanation: "Comparing `len(x) > 0` is less idiomatic than truthiness for collections.",
            remediation: "Use `if xs:` or `if not xs:` when semantics match.",
            false_positives: "Cases where falsy values differ from empty (custom containers).",
            tradeoffs: "Truthiness is shorter; length checks can be clearer for non-bool emptiness.",
            examples: "`if len(items) > 0:`",
        },
        "ZR501" => RuleDocs {
            explanation: "AI-generated placeholders often ship without real implementation.",
            remediation: "Replace placeholders with real logic or remove dead scaffolding.",
            false_positives: "Legitimate docs mentioning AI tooling in comments.",
            tradeoffs: "Catching slop early reduces review load; patterns may miss subtle cases.",
            examples: "`# TODO: AI generated` in production modules.",
        },
        "ZR504" => RuleDocs {
            explanation: "Generic exception messages make production debugging harder.",
            remediation: "Include actionable context while avoiding sensitive data in messages.",
            false_positives: "User-facing messages intentionally generic for security.",
            tradeoffs: "Detailed errors help engineers but may leak internals if exposed raw.",
            examples: "`raise Exception(\"Something went wrong\")`",
        },
        _ => return None,
    })
}

pub fn build_catalog() -> Vec<Box<dyn Check>> {
    vec![
        rule(
            "ZR001",
            "long-function",
            Category::Readability,
            Severity::Low,
            Detector::LongFunction(50),
        ),
        rule(
            "ZR002",
            "too-many-arguments",
            Category::Readability,
            Severity::Low,
            Detector::TooManyArguments(5),
        ),
        rule(
            "ZR003",
            "deep-nesting",
            Category::Readability,
            Severity::Medium,
            Detector::DeepNesting(3),
        ),
        rule(
            "ZR004",
            "complex-boolean-expression",
            Category::Readability,
            Severity::Low,
            Detector::ComplexBooleanExpression(3),
        ),
        rule(
            "ZR005",
            "unclear-variable-name",
            Category::Readability,
            Severity::Low,
            Detector::Precise(PreciseDetector::ShortVariableName),
        ),
        rule(
            "ZR006",
            "magic-number",
            Category::Readability,
            Severity::Low,
            Detector::MagicNumber,
        ),
        rule(
            "ZR007",
            "missing-module-docstring",
            Category::Readability,
            Severity::Low,
            Detector::MissingModuleDocstring,
        ),
        rule(
            "ZR008",
            "missing-function-docstring",
            Category::Readability,
            Severity::Low,
            Detector::MissingFunctionDocstring,
        ),
        rule(
            "ZR009",
            "commented-out-code",
            Category::Readability,
            Severity::Low,
            Detector::Precise(PreciseDetector::CommentedOutCode),
        ),
        rule(
            "ZR010",
            "todo-without-context",
            Category::Readability,
            Severity::Low,
            Detector::TodoWithoutContext,
        ),
        rule(
            "ZR011",
            "narrator-docstring",
            Category::Readability,
            Severity::Low,
            Detector::PatternAny(&["this function", "this module"]),
        ),
        rule(
            "ZR012",
            "boilerplate-docstring",
            Category::Readability,
            Severity::Low,
            Detector::PatternAny(&["auto-generated", "boilerplate"]),
        ),
        rule(
            "ZR013",
            "step-comment",
            Category::Readability,
            Severity::Low,
            Detector::PatternComment(&["step 1", "step 2", "step 3"]),
        ),
        rule(
            "ZR014",
            "narrator-comment",
            Category::Readability,
            Severity::Low,
            Detector::PatternComment(&["now we", "then we"]),
        ),
        rule(
            "ZR015",
            "obvious-comment",
            Category::Readability,
            Severity::Low,
            Detector::PatternComment(&["increment x", "set value"]),
        ),
        rule(
            "ZR101",
            "inconsistent-function-naming",
            Category::Consistency,
            Severity::Low,
            Detector::Precise(PreciseDetector::InconsistentFunctionNaming),
        ),
        rule(
            "ZR102",
            "inconsistent-class-naming",
            Category::Consistency,
            Severity::Low,
            Detector::PatternAny(&["class ", "_"]),
        ),
        rule(
            "ZR103",
            "inconsistent-constant-naming",
            Category::Consistency,
            Severity::Low,
            Detector::PatternAny(&[" = \"", " = '"]),
        ),
        rule(
            "ZR104",
            "inconsistent-import-style",
            Category::Consistency,
            Severity::Low,
            Detector::PatternAny(&["import ", "from "]),
        ),
        rule(
            "ZR105",
            "mixed-quote-style",
            Category::Consistency,
            Severity::Low,
            Detector::Precise(PreciseDetector::MixedStringQuotes),
        ),
        rule(
            "ZR106",
            "inconsistent-test-naming",
            Category::Consistency,
            Severity::Low,
            Detector::Precise(PreciseDetector::TestAndShouldFunctionMix),
        ),
        rule(
            "ZR107",
            "inconsistent-private-prefix",
            Category::Consistency,
            Severity::Low,
            Detector::PatternAny(&["def _", "def __"]),
        ),
        rule(
            "ZR108",
            "duplicate-naming-pattern",
            Category::Consistency,
            Severity::Low,
            Detector::PatternAny(&["manager", "service", "helper"]),
        ),
        rule(
            "ZR109",
            "mixed-collection-style",
            Category::Consistency,
            Severity::Low,
            Detector::PatternAny(&["[]", "{}"]),
        ),
        rule(
            "ZR110",
            "mixed-return-style",
            Category::Consistency,
            Severity::Low,
            Detector::PatternAny(&["return", "yield"]),
        ),
        rule(
            "ZR201",
            "god-class",
            Category::Design,
            Severity::Medium,
            Detector::GodClass(15),
        ),
        rule(
            "ZR202",
            "too-many-instance-variables",
            Category::Design,
            Severity::Medium,
            Detector::PatternAny(&["self.", "self.", "self."]),
        ),
        rule(
            "ZR203",
            "too-many-public-methods",
            Category::Design,
            Severity::Medium,
            Detector::PatternAny(&["def ", "class "]),
        ),
        rule(
            "ZR204",
            "feature-envy",
            Category::Design,
            Severity::Medium,
            Detector::PatternAny(&["other.", ".get_", ".set_"]),
        ),
        rule(
            "ZR205",
            "dataclass-without-behavior",
            Category::Design,
            Severity::Low,
            Detector::Precise(PreciseDetector::DataclassWithoutBehavior),
        ),
        rule(
            "ZR206",
            "circular-import",
            Category::Design,
            Severity::High,
            Detector::PatternAny(&["import app.", "from app."]),
        ),
        rule(
            "ZR207",
            "forbidden-architecture-import",
            Category::Design,
            Severity::High,
            Detector::ForbiddenArchitectureImport,
        ),
        rule(
            "ZR208",
            "layer-violation",
            Category::Design,
            Severity::High,
            Detector::PatternAny(&["infrastructure", "domain"]),
        ),
        rule(
            "ZR209",
            "service-object-explosion",
            Category::Design,
            Severity::Low,
            Detector::Precise(PreciseDetector::ServiceNamingExplosion),
        ),
        rule(
            "ZR210",
            "excessive-indirection",
            Category::Design,
            Severity::Low,
            Detector::PatternAny(&["delegate(", "wrapper("]),
        ),
        rule(
            "ZR301",
            "duplicate-branch-body",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::DuplicateBranchBody),
        ),
        rule(
            "ZR302",
            "collapsible-if",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::CollapsibleIf),
        ),
        rule(
            "ZR303",
            "unnecessary-else-after-return",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::UnnecessaryElseAfterReturn),
        ),
        rule(
            "ZR304",
            "redundant-boolean-comparison",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::RedundantBooleanComparison),
        ),
        rule(
            "ZR305",
            "simplifiable-if-expression",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::SimplifiableIfExpression),
        ),
        rule(
            "ZR306",
            "repeated-literal",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::RepeatedLiteral),
        ),
        rule(
            "ZR307",
            "long-parameter-list",
            Category::Refactor,
            Severity::Low,
            Detector::TooManyArguments(7),
        ),
        rule(
            "ZR308",
            "extractable-condition",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::ExtractableCondition),
        ),
        rule(
            "ZR309",
            "repeated-try-except",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::RepeatedTryExcept),
        ),
        rule(
            "ZR310",
            "identity-passthrough",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::IdentityPassthrough),
        ),
        rule(
            "ZR311",
            "identity-map",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::IdentityMap),
        ),
        rule(
            "ZR312",
            "reject-none",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::RejectNoneGuard),
        ),
        rule(
            "ZR313",
            "filter-none",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::FilterNonePattern),
        ),
        rule(
            "ZR314",
            "manual-string-join",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::ManualStringJoinInLoop),
        ),
        rule(
            "ZR315",
            "sort-then-reverse",
            Category::Refactor,
            Severity::Low,
            Detector::Precise(PreciseDetector::SortThenReverse),
        ),
        rule(
            "ZR401",
            "broad-except",
            Category::Warning,
            Severity::High,
            Detector::BroadExcept,
        ),
        rule(
            "ZR402",
            "empty-except",
            Category::Warning,
            Severity::High,
            Detector::Precise(PreciseDetector::ExceptHandlerOnlyPass { bare_only: true }),
        ),
        rule(
            "ZR403",
            "bare-except",
            Category::Warning,
            Severity::High,
            Detector::BareExcept,
        ),
        rule(
            "ZR404",
            "mutable-default-argument",
            Category::Warning,
            Severity::High,
            Detector::MutableDefaultArgument,
        ),
        rule(
            "ZR405",
            "print-debugging",
            Category::Warning,
            Severity::Low,
            Detector::PrintDebugging,
        ),
        rule(
            "ZR406",
            "assert-production",
            Category::Warning,
            Severity::Medium,
            Detector::AssertProduction,
        ),
        rule(
            "ZR407",
            "dangerous-eval-exec",
            Category::Warning,
            Severity::High,
            Detector::DangerousEvalExec,
        ),
        rule(
            "ZR408",
            "silent-exception-swallowing",
            Category::Warning,
            Severity::High,
            Detector::Precise(PreciseDetector::ExceptHandlerOnlyPass { bare_only: false }),
        ),
        rule(
            "ZR409",
            "global-mutable-state",
            Category::Warning,
            Severity::Medium,
            Detector::Precise(PreciseDetector::GlobalMutableAssignment),
        ),
        rule(
            "ZR410",
            "ambiguous-none-return",
            Category::Warning,
            Severity::Medium,
            Detector::Precise(PreciseDetector::AmbiguousNoneReturn),
        ),
        rule(
            "ZR411",
            "blanket-except",
            Category::Warning,
            Severity::High,
            Detector::Precise(PreciseDetector::BlanketExceptException),
        ),
        rule(
            "ZR412",
            "query-in-loop",
            Category::Warning,
            Severity::Medium,
            Detector::Precise(PreciseDetector::QueryInLoop),
        ),
        rule(
            "ZR413",
            "silent-fallback",
            Category::Warning,
            Severity::Medium,
            Detector::Precise(PreciseDetector::SilentFallbackReturnNone),
        ),
        rule(
            "ZR414",
            "length-comparison",
            Category::Warning,
            Severity::Low,
            Detector::LengthComparison,
        ),
        rule(
            "ZR415",
            "sort-for-top-k",
            Category::Warning,
            Severity::Low,
            Detector::Precise(PreciseDetector::SortForTopK),
        ),
        rule(
            "ZR501",
            "placeholder-generated-code",
            Category::Ai,
            Severity::Medium,
            Detector::Precise(PreciseDetector::AiPlaceholderInComment),
        ),
        rule(
            "ZR502",
            "generated-comment-pattern",
            Category::Ai,
            Severity::Medium,
            Detector::Precise(PreciseDetector::GeneratedCommentPattern),
        ),
        rule(
            "ZR503",
            "excessive-narration",
            Category::Ai,
            Severity::Low,
            Detector::PatternComment(&["this function does", "in this step"]),
        ),
        rule(
            "ZR504",
            "generic-exception-message",
            Category::Ai,
            Severity::Low,
            Detector::Precise(PreciseDetector::GenericExceptionMessage),
        ),
        rule(
            "ZR505",
            "boilerplate-parameter-docs",
            Category::Ai,
            Severity::Low,
            Detector::PatternAny(&[":param ", ":return:"]),
        ),
        rule(
            "ZR506",
            "empty-wrapper-function",
            Category::Ai,
            Severity::Low,
            Detector::Precise(PreciseDetector::EmptyWrapperFunction),
        ),
        rule(
            "ZR507",
            "generated-dead-branch",
            Category::Ai,
            Severity::Low,
            Detector::Precise(PreciseDetector::GeneratedDeadBranch),
        ),
        rule(
            "ZR508",
            "defensive-overengineering",
            Category::Ai,
            Severity::Low,
            Detector::Precise(PreciseDetector::DefensiveNamingSuffix),
        ),
        rule(
            "ZR509",
            "excessive-abstraction",
            Category::Ai,
            Severity::Low,
            Detector::PatternAny(&["Abstract", "Base", "Interface"]),
        ),
        rule(
            "ZR510",
            "generic-utility-explosion",
            Category::Ai,
            Severity::Low,
            Detector::PatternAny(&["utils", "helpers", "common"]),
        ),
    ]
}
