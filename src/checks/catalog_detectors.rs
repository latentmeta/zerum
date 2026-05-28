//! AST-precise detector implementations used by the rule catalog.

use crate::core::ast_util::{
    is_broad_except_handler, is_mutable_expr, walk_exprs_in_module, walk_module_level_stmts,
    walk_stmts_for_try, walk_stmts_in_module,
};
use crate::core::{Check, CheckContext, Issue};
use crate::parser::line_col;
use rustpython_ast::{CmpOp, Constant, ExceptHandler, Expr, Operator, Ranged, Stmt};

pub fn run_detector(
    check: &dyn Check,
    ctx: &CheckContext,
    detector: CatalogDetector,
) -> Vec<Issue> {
    if !ctx.config.is_check_enabled(check.id()) {
        return Vec::new();
    }
    match detector {
        CatalogDetector::CommentedOutCode => detect_commented_out_code(check, ctx),
        CatalogDetector::AiPlaceholderInComment => detect_ai_placeholder_in_comment(check, ctx),
        CatalogDetector::GeneratedCommentPattern => detect_generated_comment_pattern(check, ctx),
        CatalogDetector::GeneratedDeadBranch => detect_generated_dead_branch(check, ctx),
        CatalogDetector::RedundantBooleanComparison => {
            detect_redundant_boolean_comparison(check, ctx)
        }
        CatalogDetector::DataclassWithoutBehavior => detect_dataclass_without_behavior(check, ctx),
        CatalogDetector::GlobalMutableAssignment => detect_global_mutable_assignment(check, ctx),
        CatalogDetector::ExceptHandlerOnlyPass { bare_only } => {
            detect_except_handler_only_pass(check, ctx, bare_only)
        }
        CatalogDetector::MixedStringQuotes => detect_mixed_string_quotes(check, ctx),
        CatalogDetector::ShortVariableName => detect_short_variable_names(check, ctx),
        CatalogDetector::TestAndShouldFunctionMix => detect_test_and_should_mix(check, ctx),
        CatalogDetector::DefensiveNamingSuffix => detect_defensive_naming_suffix(check, ctx),
        CatalogDetector::ServiceNamingExplosion => detect_service_naming_explosion(check, ctx),
        CatalogDetector::UnnecessaryElseAfterReturn => {
            detect_unnecessary_else_after_return(check, ctx)
        }
        CatalogDetector::IdentityPassthrough => detect_identity_passthrough(check, ctx),
        CatalogDetector::IdentityMap => detect_identity_map(check, ctx),
        CatalogDetector::BlanketExceptException => detect_blanket_except_exception(check, ctx),
        CatalogDetector::SilentFallbackReturnNone => detect_silent_fallback_return_none(check, ctx),
        CatalogDetector::GenericExceptionMessage => detect_generic_exception_message(check, ctx),
        CatalogDetector::CollapsibleIf => detect_collapsible_if(check, ctx),
        CatalogDetector::DuplicateBranchBody => detect_duplicate_branch_body(check, ctx),
        CatalogDetector::ManualStringJoinInLoop => detect_manual_string_join_in_loop(check, ctx),
        CatalogDetector::SortThenReverse => detect_sort_then_reverse(check, ctx),
        CatalogDetector::SortForTopK => detect_sort_for_top_k(check, ctx),
        CatalogDetector::QueryInLoop => detect_query_in_loop(check, ctx),
        CatalogDetector::SimplifiableIfExpression => detect_simplifiable_if_expression(check, ctx),
        CatalogDetector::ExtractableCondition => detect_extractable_condition(check, ctx),
        CatalogDetector::RepeatedTryExcept => detect_repeated_try_except(check, ctx),
        CatalogDetector::AmbiguousNoneReturn => detect_ambiguous_none_return(check, ctx),
        CatalogDetector::RejectNoneGuard => detect_reject_none_guard(check, ctx),
        CatalogDetector::FilterNonePattern => detect_filter_none_pattern(check, ctx),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CatalogDetector {
    CommentedOutCode,
    AiPlaceholderInComment,
    GeneratedCommentPattern,
    GeneratedDeadBranch,
    RedundantBooleanComparison,
    DataclassWithoutBehavior,
    GlobalMutableAssignment,
    ExceptHandlerOnlyPass { bare_only: bool },
    MixedStringQuotes,
    ShortVariableName,
    TestAndShouldFunctionMix,
    DefensiveNamingSuffix,
    ServiceNamingExplosion,
    UnnecessaryElseAfterReturn,
    IdentityPassthrough,
    IdentityMap,
    BlanketExceptException,
    SilentFallbackReturnNone,
    GenericExceptionMessage,
    CollapsibleIf,
    DuplicateBranchBody,
    ManualStringJoinInLoop,
    SortThenReverse,
    SortForTopK,
    QueryInLoop,
    SimplifiableIfExpression,
    ExtractableCondition,
    RepeatedTryExcept,
    AmbiguousNoneReturn,
    RejectNoneGuard,
    FilterNonePattern,
}

fn detect_commented_out_code(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let markers = ["def ", "class ", "return ", "import ", "raise "];
    let mut issues = Vec::new();
    for comment in ctx.source_model().comments() {
        let body = comment.text.trim_start_matches('#').trim();
        let lower = body.to_ascii_lowercase();
        if markers.iter().any(|m| lower.starts_with(m)) {
            issues.push(Issue::from_check(
                check,
                "comment appears to contain commented-out executable code",
                ctx.file_path(),
                comment.line,
                comment.column,
            ));
        }
    }
    issues
}

fn detect_ai_placeholder_in_comment(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let needles = ["TODO: AI", "FIXME: AI", "GENERATED BY AI"];
    let mut issues = Vec::new();
    for comment in ctx.source_model().comments() {
        let upper = comment.text.to_ascii_uppercase();
        if needles.iter().any(|n| upper.contains(n)) {
            issues.push(Issue::from_check(
                check,
                "AI placeholder marker found in comment",
                ctx.file_path(),
                comment.line,
                comment.column,
            ));
        }
    }
    issues
}

fn detect_generated_comment_pattern(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let needles = ["GENERATED BY", "AUTO-GENERATED"];
    let mut issues = Vec::new();
    for comment in ctx.source_model().comments() {
        let upper = comment.text.to_ascii_uppercase();
        if needles.iter().any(|n| upper.contains(n)) {
            issues.push(Issue::from_check(
                check,
                "generated-code marker in comment",
                ctx.file_path(),
                comment.line,
                comment.column,
            ));
        }
    }
    issues
}

fn detect_generated_dead_branch(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::If(i) = stmt {
            if is_constant_bool(&i.test, true) || is_constant_bool(&i.test, false) {
                let (line, col) = line_col(&ctx.parsed.source, i.start().into());
                issues.push(Issue::from_check(
                    check,
                    "constant boolean condition suggests generated or placeholder branch",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_redundant_boolean_comparison(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_exprs_in_module(ctx.parsed, &mut |expr| {
        if let Expr::Compare(c) = expr {
            if compare_uses_bool_constant(&c.left, true)
                || compare_uses_bool_constant(&c.left, false)
                || c.comparators.iter().any(|e| {
                    compare_uses_bool_constant(e, true) || compare_uses_bool_constant(e, false)
                })
            {
                let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                issues.push(Issue::from_check(
                    check,
                    "redundant comparison to True/False constant",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_dataclass_without_behavior(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::ClassDef(c) = stmt {
            if !has_dataclass_decorator(c) {
                return;
            }
            let only_pass = !c.body.is_empty() && c.body.iter().all(|s| matches!(s, Stmt::Pass(_)));
            if only_pass {
                let (line, col) = line_col(&ctx.parsed.source, c.start().into());
                issues.push(Issue::from_check(
                    check,
                    format!("dataclass `{}` has no behavior beyond pass", c.name),
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_global_mutable_assignment(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_module_level_stmts(ctx.parsed, &mut |stmt| {
        if let Stmt::Assign(a) = stmt {
            if is_mutable_expr(&a.value) {
                let (line, col) = line_col(&ctx.parsed.source, a.start().into());
                issues.push(Issue::from_check(
                    check,
                    "module-level assignment to mutable literal",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_except_handler_only_pass(
    check: &dyn Check,
    ctx: &CheckContext,
    bare_only: bool,
) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_for_try(ctx.parsed, &mut |handler| {
        let ExceptHandler::ExceptHandler(h) = handler;
        if bare_only && h.type_.is_some() {
            return;
        }
        if !bare_only && h.type_.is_none() {
            return;
        }
        if is_only_pass_stmts(&h.body) {
            let (line, col) = line_col(&ctx.parsed.source, h.start().into());
            let msg = if h.type_.is_none() {
                "bare except handler only contains pass"
            } else if is_broad_except_handler(handler) {
                "broad except handler only contains pass"
            } else {
                "except handler only contains pass"
            };
            issues.push(Issue::from_check(check, msg, ctx.file_path(), line, col));
        }
    });
    issues
}

fn detect_mixed_string_quotes(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut single = false;
    let mut double = false;
    walk_exprs_in_module(ctx.parsed, &mut |expr| {
        if let Expr::Constant(c) = expr {
            if matches!(c.value, Constant::Str(_)) {
                let start: usize = expr.start().into();
                let end: usize = expr.end().into();
                if start < end && end <= ctx.parsed.source.len() {
                    let slice = &ctx.parsed.source[start..end];
                    if slice.starts_with('\'') {
                        single = true;
                    }
                    if slice.starts_with('"') {
                        double = true;
                    }
                }
            }
        }
    });
    if single && double {
        vec![Issue::from_check(
            check,
            "module mixes single- and double-quoted string literals",
            ctx.file_path(),
            1,
            1,
        )]
    } else {
        Vec::new()
    }
}

fn detect_short_variable_names(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::Assign(a) = stmt {
            for target in &a.targets {
                if let Expr::Name(n) = target {
                    if n.id.as_str().len() <= 2 {
                        let (line, col) = line_col(&ctx.parsed.source, a.start().into());
                        issues.push(Issue::from_check(
                            check,
                            format!("very short variable name `{}`", n.id),
                            ctx.file_path(),
                            line,
                            col,
                        ));
                    }
                }
            }
        }
    });
    issues
}

fn detect_test_and_should_mix(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut test_style = false;
    let mut should_style = false;
    for f in ctx.source_model().functions() {
        if f.name.starts_with("test_") {
            test_style = true;
        }
        if f.name.starts_with("should_") {
            should_style = true;
        }
    }
    if test_style && should_style {
        vec![Issue::from_check(
            check,
            "module mixes test_* and should_* function naming styles",
            ctx.file_path(),
            1,
            1,
        )]
    } else {
        Vec::new()
    }
}

fn detect_defensive_naming_suffix(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let suffixes = ["Factory", "Builder", "Strategy"];
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::ClassDef(c) = stmt {
            for s in suffixes {
                if c.name.as_str().ends_with(s) {
                    let (line, col) = line_col(&ctx.parsed.source, c.start().into());
                    issues.push(Issue::from_check(
                        check,
                        format!("class `{}` uses defensive suffix `{s}`", c.name),
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

fn detect_service_naming_explosion(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let suffixes = ["Service", "Manager", "Handler"];
    let count = ctx
        .source_model()
        .classes()
        .into_iter()
        .filter(|c| suffixes.iter().any(|s| c.name.ends_with(s)))
        .count();
    if count >= 2 {
        vec![Issue::from_check(
            check,
            format!("module defines {count} service-style classes (Service/Manager/Handler)"),
            ctx.file_path(),
            1,
            1,
        )]
    } else {
        Vec::new()
    }
}

fn detect_unnecessary_else_after_return(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::If(i) = stmt {
            if !i.orelse.is_empty() && block_ends_with_return(&i.body) {
                let (line, col) = line_col(&ctx.parsed.source, i.start().into());
                issues.push(Issue::from_check(
                    check,
                    "if branch returns but is followed by else; else may be unnecessary",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_identity_passthrough(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        let (args, body, start, name) = match stmt {
            Stmt::FunctionDef(f) => (&f.args, f.body.as_slice(), f.start(), &f.name),
            Stmt::AsyncFunctionDef(f) => (&f.args, f.body.as_slice(), f.start(), &f.name),
            _ => return,
        };
        let param = single_positional_arg(args);
        let Some(param) = param else { return };
        let Some(Stmt::Return(r)) = last_meaningful_stmt(body) else {
            return;
        };
        let Some(Expr::Name(n)) = r.value.as_deref() else {
            return;
        };
        if n.id.as_str() != param {
            return;
        }
        if body.iter().filter(|s| !matches!(s, Stmt::Pass(_))).count() != 1 {
            return;
        }
        let (line, col) = line_col(&ctx.parsed.source, start.into());
        issues.push(Issue::from_check(
            check,
            format!("function `{name}` only returns its single argument"),
            ctx.file_path(),
            line,
            col,
        ));
    });
    issues
}

fn detect_identity_map(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_exprs_in_module(ctx.parsed, &mut |expr| {
        if let Expr::Call(c) = expr {
            if !is_map_call(&c.func) {
                return;
            }
            if c.args.first().is_some_and(is_identity_lambda) {
                let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                issues.push(Issue::from_check(
                    check,
                    "map(lambda x: x, ...) is an identity; use list(...) or copy directly",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_blanket_except_exception(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_for_try(ctx.parsed, &mut |handler| {
        let ExceptHandler::ExceptHandler(h) = handler;
        if matches!(
            h.type_.as_deref(),
            Some(Expr::Name(n)) if n.id.as_str() == "Exception"
        ) {
            let (line, col) = line_col(&ctx.parsed.source, h.start().into());
            issues.push(Issue::from_check(
                check,
                "except Exception catches nearly all errors; prefer specific exceptions",
                ctx.file_path(),
                line,
                col,
            ));
        }
    });
    issues
}

fn detect_silent_fallback_return_none(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_for_try(ctx.parsed, &mut |handler| {
        let ExceptHandler::ExceptHandler(h) = handler;
        if handler_returns_none_only(&h.body) {
            let (line, col) = line_col(&ctx.parsed.source, h.start().into());
            issues.push(Issue::from_check(
                check,
                "except handler returns None without logging or re-raising",
                ctx.file_path(),
                line,
                col,
            ));
        }
    });
    issues
}

fn detect_collapsible_if(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::If(outer) = stmt {
            if !outer.orelse.is_empty() {
                return;
            }
            if outer.body.len() != 1 {
                return;
            }
            if let Stmt::If(inner) = &outer.body[0] {
                if inner.orelse.is_empty() {
                    let (line, col) = line_col(&ctx.parsed.source, outer.start().into());
                    issues.push(Issue::from_check(
                        check,
                        "nested if without else can be collapsed with `and`",
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

fn detect_duplicate_branch_body(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::If(i) = stmt {
            if i.orelse.len() == 1 {
                if let Stmt::If(elif_branch) = &i.orelse[0] {
                    if elif_branch.orelse.is_empty()
                        && stmt_blocks_equal(ctx, &i.body, &elif_branch.body)
                    {
                        let (line, col) = line_col(&ctx.parsed.source, i.start().into());
                        issues.push(Issue::from_check(
                            check,
                            "if and elif branches have identical bodies",
                            ctx.file_path(),
                            line,
                            col,
                        ));
                    }
                }
            }
        }
    });
    issues
}

fn detect_manual_string_join_in_loop(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        let Stmt::For(f) = stmt else { return };
        for body_stmt in &f.body {
            if let Stmt::AugAssign(a) = body_stmt {
                if matches!(a.op, Operator::Add) {
                    let (line, col) = line_col(&ctx.parsed.source, a.start().into());
                    issues.push(Issue::from_check(
                        check,
                        "string concatenation with += in a loop; prefer `str.join`",
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

fn detect_sort_then_reverse(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    visit_stmt_blocks(ctx.parsed, &mut |stmts| {
        for pair in stmts.windows(2) {
            let (Stmt::Expr(e1), Stmt::Expr(e2)) = (&pair[0], &pair[1]) else {
                continue;
            };
            if is_method_call_named(&e1.value, "sort")
                && is_method_call_named(&e2.value, "reverse")
                && call_receivers_match(&e1.value, &e2.value)
            {
                let (line, col) = line_col(&ctx.parsed.source, e1.start().into());
                issues.push(Issue::from_check(
                    check,
                    "sort() followed by reverse(); use `sorted(..., reverse=True)`",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_sort_for_top_k(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_exprs_in_module(ctx.parsed, &mut |expr| {
        if let Expr::Subscript(s) = expr {
            if !is_sorted_call(&s.value) {
                return;
            }
            if slice_has_upper_bound(&s.slice) {
                let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                issues.push(Issue::from_check(
                    check,
                    "full sort then slice for top-k; consider `heapq.nlargest` or partial selection",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_simplifiable_if_expression(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_exprs_in_module(ctx.parsed, &mut |expr| {
        if let Expr::IfExp(i) = expr {
            let body_bool = expr_as_bool_constant(&i.body);
            let else_bool = expr_as_bool_constant(&i.orelse);
            if body_bool.is_some() && else_bool.is_some() && body_bool != else_bool {
                let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                issues.push(Issue::from_check(
                    check,
                    "if-expression with True/False branches can simplify to the condition",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_extractable_condition(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::If(i) = stmt {
            if bool_expression_complexity(&i.test) >= 3 {
                let (line, col) = line_col(&ctx.parsed.source, i.start().into());
                issues.push(Issue::from_check(
                    check,
                    "complex if condition may be clearer as a named boolean variable",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_repeated_try_except(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        let body = match stmt {
            Stmt::FunctionDef(f) => f.body.as_slice(),
            Stmt::AsyncFunctionDef(f) => f.body.as_slice(),
            _ => return,
        };
        let try_count = count_try_stmts(body);
        if try_count >= 2 {
            let (line, col) = line_col(&ctx.parsed.source, stmt.start().into());
            issues.push(Issue::from_check(
                check,
                format!("function contains {try_count} try/except blocks; consider consolidating"),
                ctx.file_path(),
                line,
                col,
            ));
        }
    });
    issues
}

fn detect_ambiguous_none_return(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        let body = match stmt {
            Stmt::FunctionDef(f) => f.body.as_slice(),
            Stmt::AsyncFunctionDef(f) => f.body.as_slice(),
            _ => return,
        };
        let mut explicit_none = false;
        let mut bare_return = false;
        collect_return_styles(body, &mut explicit_none, &mut bare_return);
        if explicit_none && bare_return {
            let (line, col) = line_col(&ctx.parsed.source, stmt.start().into());
            issues.push(Issue::from_check(
                check,
                "function mixes bare `return` and `return None`",
                ctx.file_path(),
                line,
                col,
            ));
        }
    });
    issues
}

fn detect_reject_none_guard(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::If(i) = stmt {
            if let Some(name) = is_not_none_test(&i.test) {
                if branch_returns_name(&i.body, name) {
                    let (line, col) = line_col(&ctx.parsed.source, i.start().into());
                    issues.push(Issue::from_check(
                        check,
                        format!("`if {name} is not None: return {name}` can often be simplified"),
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

fn detect_filter_none_pattern(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_exprs_in_module(ctx.parsed, &mut |expr| {
        if let Expr::ListComp(lc) = expr {
            if lc
                .generators
                .iter()
                .any(|g| g.ifs.iter().any(|cond| is_not_none_test(cond).is_some()))
            {
                let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                issues.push(Issue::from_check(
                    check,
                    "list comprehension filters `is not None`; consider filter(None, ...)",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
        if let Expr::GeneratorExp(ge) = expr {
            if ge
                .generators
                .iter()
                .any(|g| g.ifs.iter().any(|cond| is_not_none_test(cond).is_some()))
            {
                let (line, col) = line_col(&ctx.parsed.source, expr.start().into());
                issues.push(Issue::from_check(
                    check,
                    "generator filters `is not None`; consider filter(None, ...)",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::For(f) = stmt {
            if f.body
                .iter()
                .any(|s| matches!(s, Stmt::If(i) if is_not_none_test(&i.test).is_some()))
            {
                let (line, col) = line_col(&ctx.parsed.source, f.start().into());
                issues.push(Issue::from_check(
                    check,
                    "loop filters with `is not None`; consider filter(None, ...)",
                    ctx.file_path(),
                    line,
                    col,
                ));
            }
        }
    });
    issues
}

fn detect_query_in_loop(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        let Stmt::For(f) = stmt else { return };
        let mut found = false;
        walk_stmts_in_block(&f.body, &mut |inner| {
            if stmt_contains_db_call(inner) {
                found = true;
            }
        });
        if !found {
            return;
        }
        if found {
            let (line, col) = line_col(&ctx.parsed.source, f.start().into());
            issues.push(Issue::from_check(
                check,
                "database query or execute inside a loop may cause N+1 access",
                ctx.file_path(),
                line,
                col,
            ));
        }
    });
    issues
}

fn detect_generic_exception_message(check: &dyn Check, ctx: &CheckContext) -> Vec<Issue> {
    let generic = [
        "error",
        "failed",
        "something went wrong",
        "an error occurred",
    ];
    let mut issues = Vec::new();
    walk_stmts_in_module(ctx.parsed, &mut |stmt| {
        if let Stmt::Raise(r) = stmt {
            if let Some(Expr::Call(c)) = r.exc.as_deref() {
                if is_exception_call(&c.func) {
                    if let Some(msg) = first_string_arg(&c.args) {
                        if generic.iter().any(|g| msg.eq_ignore_ascii_case(g)) {
                            let (line, col) = line_col(&ctx.parsed.source, r.start().into());
                            issues.push(Issue::from_check(
                                check,
                                format!("generic exception message `{msg}`"),
                                ctx.file_path(),
                                line,
                                col,
                            ));
                        }
                    }
                }
            }
        }
    });
    issues
}

fn single_positional_arg(args: &rustpython_ast::Arguments) -> Option<&str> {
    if args.posonlyargs.len() + args.args.len() != 1 {
        return None;
    }
    args.posonlyargs
        .first()
        .map(|a| a.def.arg.as_str())
        .or_else(|| args.args.first().map(|a| a.def.arg.as_str()))
}

fn is_map_call(func: &Expr) -> bool {
    matches!(func, Expr::Name(n) if n.id.as_str() == "map")
}

fn is_identity_lambda(expr: &Expr) -> bool {
    if let Expr::Lambda(l) = expr {
        if l.args.args.len() != 1 {
            return false;
        }
        let param = l.args.args[0].def.arg.as_str();
        return matches!(
            l.body.as_ref(),
            Expr::Name(n) if n.id.as_str() == param
        );
    }
    false
}

fn handler_returns_none_only(body: &[Stmt]) -> bool {
    match last_meaningful_stmt(body) {
        Some(Stmt::Return(r)) => match r.value.as_deref() {
            None => true,
            Some(v) => is_none_constant(v),
        },
        _ => false,
    }
}

fn is_none_constant(expr: &Expr) -> bool {
    matches!(expr, Expr::Constant(c) if matches!(c.value, Constant::None))
}

fn is_exception_call(func: &Expr) -> bool {
    matches!(func, Expr::Name(n) if n.id.as_str() == "Exception")
}

fn first_string_arg(args: &[Expr]) -> Option<String> {
    args.first().and_then(|e| {
        if let Expr::Constant(c) = e {
            if let Constant::Str(s) = &c.value {
                return Some(s.to_string());
            }
        }
        None
    })
}

fn block_ends_with_return(stmts: &[Stmt]) -> bool {
    matches!(last_meaningful_stmt(stmts), Some(Stmt::Return(_)))
}

fn last_meaningful_stmt(stmts: &[Stmt]) -> Option<&Stmt> {
    stmts.iter().rev().find(|s| !matches!(s, Stmt::Pass(_)))
}

fn has_dataclass_decorator(class: &rustpython_ast::StmtClassDef) -> bool {
    class.decorator_list.iter().any(|d| {
        matches!(
            d,
            Expr::Name(n) if n.id.as_str() == "dataclass"
                || matches!(d, Expr::Attribute(a) if a.attr.as_str() == "dataclass")
        )
    })
}

fn is_only_pass_stmts(stmts: &[Stmt]) -> bool {
    !stmts.is_empty() && stmts.iter().all(|s| matches!(s, Stmt::Pass(_)))
}

fn is_constant_bool(expr: &Expr, value: bool) -> bool {
    matches!(
        expr,
        Expr::Constant(c) if matches!(c.value, Constant::Bool(b) if b == value)
    )
}

fn compare_uses_bool_constant(expr: &Expr, value: bool) -> bool {
    matches!(
        expr,
        Expr::Constant(c) if matches!(c.value, Constant::Bool(b) if b == value)
    )
}

fn stmt_blocks_equal(ctx: &CheckContext, a: &[Stmt], b: &[Stmt]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(left, right)| {
        let ls = stmt_source(ctx, left);
        let rs = stmt_source(ctx, right);
        ls.trim() == rs.trim()
    })
}

fn stmt_source(ctx: &CheckContext, stmt: &Stmt) -> String {
    let start: usize = stmt.start().into();
    let end: usize = stmt.end().into();
    if end <= ctx.parsed.source.len() {
        ctx.parsed.source[start..end].to_string()
    } else {
        String::new()
    }
}

fn is_method_call_named(expr: &Expr, method: &str) -> bool {
    if let Expr::Call(c) = expr {
        if let Expr::Attribute(a) = c.func.as_ref() {
            return a.attr.as_str() == method;
        }
    }
    false
}

fn call_receivers_match(left: &Expr, right: &Expr) -> bool {
    let (Expr::Call(l), Expr::Call(r)) = (left, right) else {
        return false;
    };
    match (l.func.as_ref(), r.func.as_ref()) {
        (Expr::Attribute(la), Expr::Attribute(ra)) => receivers_equal(&la.value, &ra.value),
        _ => false,
    }
}

fn receivers_equal(left: &Expr, right: &Expr) -> bool {
    match (left, right) {
        (Expr::Name(l), Expr::Name(r)) => l.id == r.id,
        (Expr::Attribute(l), Expr::Attribute(r)) => {
            l.attr == r.attr && receivers_equal(&l.value, &r.value)
        }
        _ => false,
    }
}

fn is_sorted_call(expr: &Expr) -> bool {
    if let Expr::Call(c) = expr {
        return matches!(c.func.as_ref(), Expr::Name(n) if n.id.as_str() == "sorted");
    }
    false
}

fn slice_has_upper_bound(slice: &Expr) -> bool {
    match slice {
        Expr::Slice(s) => s.upper.is_some(),
        _ => false,
    }
}

fn visit_stmt_blocks(parsed: &crate::parser::ParsedFile, visit: &mut impl FnMut(&[Stmt])) {
    fn walk(stmts: &[Stmt], visit: &mut impl FnMut(&[Stmt])) {
        visit(stmts);
        for stmt in stmts {
            match stmt {
                Stmt::FunctionDef(f) => walk(&f.body, visit),
                Stmt::AsyncFunctionDef(f) => walk(&f.body, visit),
                Stmt::ClassDef(c) => walk(&c.body, visit),
                Stmt::If(i) => {
                    walk(&i.body, visit);
                    walk(&i.orelse, visit);
                }
                Stmt::For(f) => walk(&f.body, visit),
                Stmt::While(w) => walk(&w.body, visit),
                Stmt::With(w) => walk(&w.body, visit),
                Stmt::Try(t) => {
                    walk(&t.body, visit);
                    for h in &t.handlers {
                        let ExceptHandler::ExceptHandler(h) = h;
                        walk(&h.body, visit);
                    }
                    walk(&t.orelse, visit);
                    walk(&t.finalbody, visit);
                }
                _ => {}
            }
        }
    }
    let body = match &parsed.module {
        rustpython_ast::Mod::Module(m) => m.body.as_slice(),
        rustpython_ast::Mod::Interactive(m) => m.body.as_slice(),
        _ => &[],
    };
    walk(body, visit);
}

fn walk_stmts_in_block(stmts: &[Stmt], visitor: &mut impl FnMut(&Stmt)) {
    for stmt in stmts {
        visitor(stmt);
        match stmt {
            Stmt::For(f) => walk_stmts_in_block(&f.body, visitor),
            Stmt::While(w) => walk_stmts_in_block(&w.body, visitor),
            Stmt::With(w) => walk_stmts_in_block(&w.body, visitor),
            Stmt::If(i) => {
                walk_stmts_in_block(&i.body, visitor);
                walk_stmts_in_block(&i.orelse, visitor);
            }
            Stmt::Try(t) => {
                walk_stmts_in_block(&t.body, visitor);
                for h in &t.handlers {
                    let ExceptHandler::ExceptHandler(h) = h;
                    walk_stmts_in_block(&h.body, visitor);
                }
                walk_stmts_in_block(&t.orelse, visitor);
                walk_stmts_in_block(&t.finalbody, visitor);
            }
            Stmt::FunctionDef(f) => walk_stmts_in_block(&f.body, visitor),
            Stmt::AsyncFunctionDef(f) => walk_stmts_in_block(&f.body, visitor),
            Stmt::ClassDef(c) => walk_stmts_in_block(&c.body, visitor),
            _ => {}
        }
    }
}

fn stmt_contains_db_call(stmt: &Stmt) -> bool {
    let mut found = false;
    visit_stmt_exprs(stmt, &mut |expr| {
        if expr_is_db_call(expr) {
            found = true;
        }
    });
    found
}

fn visit_stmt_exprs(stmt: &Stmt, f: &mut impl FnMut(&Expr)) {
    match stmt {
        Stmt::Expr(e) => walk_expr_shallow(&e.value, f),
        Stmt::Return(r) => {
            if let Some(v) = &r.value {
                walk_expr_shallow(v, f);
            }
        }
        Stmt::Assign(a) => {
            walk_expr_shallow(&a.value, f);
        }
        Stmt::AugAssign(a) => walk_expr_shallow(&a.value, f),
        _ => {}
    }
}

fn walk_expr_shallow(expr: &Expr, f: &mut impl FnMut(&Expr)) {
    f(expr);
    if let Expr::Call(c) = expr {
        for arg in &c.args {
            walk_expr_shallow(arg, f);
        }
    }
}

fn expr_as_bool_constant(expr: &Expr) -> Option<bool> {
    match expr {
        Expr::Constant(c) => match c.value {
            Constant::Bool(b) => Some(b),
            _ => None,
        },
        _ => None,
    }
}

fn bool_expression_complexity(expr: &Expr) -> usize {
    match expr {
        Expr::BoolOp(b) => b.values.len().saturating_sub(1) + 1,
        Expr::Compare(c) => {
            let mut n = c.comparators.len();
            if matches!(c.ops.first(), Some(CmpOp::Is | CmpOp::IsNot)) {
                n += 1;
            }
            n
        }
        Expr::UnaryOp(u) => bool_expression_complexity(&u.operand),
        _ => 1,
    }
}

fn count_try_stmts(stmts: &[Stmt]) -> usize {
    let mut count = 0;
    for stmt in stmts {
        if matches!(stmt, Stmt::Try(_)) {
            count += 1;
        }
        match stmt {
            Stmt::If(i) => {
                count += count_try_stmts(&i.body);
                count += count_try_stmts(&i.orelse);
            }
            Stmt::For(f) => count += count_try_stmts(&f.body),
            Stmt::While(w) => count += count_try_stmts(&w.body),
            Stmt::With(w) => count += count_try_stmts(&w.body),
            Stmt::Try(t) => {
                count += count_try_stmts(&t.body);
                for h in &t.handlers {
                    let ExceptHandler::ExceptHandler(h) = h;
                    count += count_try_stmts(&h.body);
                }
                count += count_try_stmts(&t.orelse);
                count += count_try_stmts(&t.finalbody);
            }
            _ => {}
        }
    }
    count
}

fn collect_return_styles(stmts: &[Stmt], explicit_none: &mut bool, bare_return: &mut bool) {
    for stmt in stmts {
        match stmt {
            Stmt::Return(r) => match r.value.as_deref() {
                None => *bare_return = true,
                Some(v) if is_none_constant(v) => *explicit_none = true,
                Some(_) => {}
            },
            Stmt::If(i) => {
                collect_return_styles(&i.body, explicit_none, bare_return);
                collect_return_styles(&i.orelse, explicit_none, bare_return);
            }
            Stmt::For(f) => collect_return_styles(&f.body, explicit_none, bare_return),
            Stmt::While(w) => collect_return_styles(&w.body, explicit_none, bare_return),
            Stmt::Try(t) => {
                collect_return_styles(&t.body, explicit_none, bare_return);
                for h in &t.handlers {
                    let ExceptHandler::ExceptHandler(h) = h;
                    collect_return_styles(&h.body, explicit_none, bare_return);
                }
                collect_return_styles(&t.orelse, explicit_none, bare_return);
            }
            _ => {}
        }
    }
}

fn is_not_none_test(expr: &Expr) -> Option<&str> {
    if let Expr::Compare(c) = expr {
        if matches!(c.ops.first(), Some(CmpOp::IsNot)) {
            if let Expr::Constant(v) = c.comparators.first()? {
                if matches!(v.value, Constant::None) {
                    if let Expr::Name(n) = c.left.as_ref() {
                        return Some(n.id.as_str());
                    }
                }
            }
        }
    }
    None
}

fn branch_returns_name(stmts: &[Stmt], name: &str) -> bool {
    matches!(
        last_meaningful_stmt(stmts),
        Some(Stmt::Return(r)) if matches!(
            r.value.as_deref(),
            Some(Expr::Name(n)) if n.id.as_str() == name
        )
    )
}

fn expr_is_db_call(expr: &Expr) -> bool {
    if let Expr::Call(c) = expr {
        if let Expr::Attribute(a) = c.func.as_ref() {
            return matches!(a.attr.as_str(), "query" | "execute");
        }
    }
    false
}
