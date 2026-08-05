use crate::parser::{line_col, ParsedFile};
use rustpython_ast::text_size::TextSize;
use rustpython_ast::{Arguments, ExceptHandler, Expr, Identifier, Mod, Pattern, Ranged, Stmt};

#[derive(Debug, Clone)]
pub struct FunctionMetrics {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub arg_count: usize,
    pub body_lines: usize,
    pub branch_count: usize,
    pub max_conditional_depth: usize,
    pub has_docstring: bool,
}

fn offset(start: TextSize) -> u32 {
    start.into()
}

fn count_args(args: &Arguments) -> usize {
    let mut n = args.posonlyargs.len() + args.args.len() + args.kwonlyargs.len();
    if args.vararg.is_some() {
        n += 1;
    }
    if args.kwarg.is_some() {
        n += 1;
    }
    n
}

/// True when the first meaningful statement is a string-literal expression (PEP 257 docstring).
pub fn body_starts_with_docstring(body: &[Stmt]) -> bool {
    match body.first() {
        Some(Stmt::Expr(e)) => matches!(
            e.value.as_ref(),
            Expr::Constant(c) if matches!(c.value, rustpython_ast::Constant::Str(_))
        ),
        _ => false,
    }
}

/// True when a module's first non-comment, non-blank statement is a string literal docstring.
pub fn module_has_docstring(parsed: &ParsedFile) -> bool {
    body_starts_with_docstring(module_body(&parsed.module))
}

pub fn collect_function_metrics(parsed: &ParsedFile) -> Vec<FunctionMetrics> {
    let mut out = Vec::new();
    walk_stmts(parsed, module_body(&parsed.module), &mut |stmt| {
        let (name, start, args, body) = match stmt {
            Stmt::FunctionDef(f) => (&f.name, f.start(), &f.args, f.body.as_slice()),
            Stmt::AsyncFunctionDef(f) => (&f.name, f.start(), &f.args, f.body.as_slice()),
            _ => return,
        };
        let (line, column) = line_col(&parsed.source, offset(start));
        out.push(FunctionMetrics {
            name: name.to_string(),
            line,
            column,
            arg_count: count_args(args),
            body_lines: stmt_line_span(parsed, body),
            branch_count: count_branches(body),
            max_conditional_depth: max_conditional_depth(body, 0),
            has_docstring: body_starts_with_docstring(body),
        });
    });
    out
}

/// Visits every expression in the module using [`walk_stmts`] for nesting (no duplicated control-flow descent).
pub fn walk_exprs_in_module(parsed: &ParsedFile, f: &mut impl FnMut(&Expr)) {
    walk_stmts(parsed, module_body(&parsed.module), &mut |stmt| {
        visit_stmt_exprs(stmt, f)
    });
}

pub fn walk_function_defs(
    parsed: &ParsedFile,
    f: &mut impl FnMut(&Arguments, TextSize, &Identifier),
) {
    walk_stmts(
        parsed,
        module_body(&parsed.module),
        &mut |stmt| match stmt {
            Stmt::FunctionDef(fd) => f(&fd.args, fd.start(), &fd.name),
            Stmt::AsyncFunctionDef(fd) => f(&fd.args, fd.start(), &fd.name),
            _ => {}
        },
    );
}

pub fn has_mutable_defaults(args: &Arguments) -> bool {
    args.posonlyargs
        .iter()
        .chain(args.args.iter())
        .chain(args.kwonlyargs.iter())
        .filter_map(|a| a.default.as_deref())
        .any(is_mutable_expr)
}

pub fn walk_stmts_for_try(parsed: &ParsedFile, f: &mut impl FnMut(&ExceptHandler)) {
    walk_stmts(parsed, module_body(&parsed.module), &mut |stmt| {
        let handlers = match stmt {
            Stmt::Try(t) => &t.handlers,
            Stmt::TryStar(t) => &t.handlers,
            _ => return,
        };
        for h in handlers {
            f(h);
        }
    });
}

pub fn walk_stmts_in_module(parsed: &ParsedFile, f: &mut impl FnMut(&Stmt)) {
    walk_stmts(parsed, module_body(&parsed.module), f);
}

/// Visits only top-level module statements (does not descend into functions/classes).
pub fn walk_module_level_stmts(parsed: &ParsedFile, f: &mut impl FnMut(&Stmt)) {
    for stmt in module_body(&parsed.module) {
        f(stmt);
    }
}

/// Detects bare `print(...)` calls. Does not match `builtins.print` or renamed imports (Phase 1 scope).
pub fn is_print_call(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Call(c) if matches!(c.func.as_ref(), Expr::Name(n) if n.id.as_str() == "print")
    )
}

pub fn is_broad_except_handler(handler: &ExceptHandler) -> bool {
    let ExceptHandler::ExceptHandler(h) = handler;
    h.type_.is_none()
        || matches!(
            h.type_.as_deref(),
            Some(Expr::Name(n)) if n.id.as_str() == "Exception" || n.id.as_str() == "BaseException"
        )
}

/// `except BaseException:` only (ZR401). Bare and `Exception` are owned by ZR403 / ZR411.
pub fn is_base_exception_handler(handler: &ExceptHandler) -> bool {
    let ExceptHandler::ExceptHandler(h) = handler;
    matches!(
        h.type_.as_deref(),
        Some(Expr::Name(n)) if n.id.as_str() == "BaseException"
    )
}

pub fn is_mutable_expr(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::List(_)
            | Expr::Dict(_)
            | Expr::Set(_)
            | Expr::ListComp(_)
            | Expr::DictComp(_)
            | Expr::SetComp(_)
    )
}

fn module_body(module: &Mod) -> &[Stmt] {
    match module {
        Mod::Module(m) => &m.body,
        Mod::Interactive(m) => &m.body,
        Mod::Expression(_) => &[],
        Mod::FunctionType(_) => &[],
    }
}

fn walk_stmts(parsed: &ParsedFile, stmts: &[Stmt], visitor: &mut impl FnMut(&Stmt)) {
    for stmt in stmts {
        visitor(stmt);
        descend_stmt(parsed, stmt, visitor);
    }
}

fn descend_stmt(parsed: &ParsedFile, stmt: &Stmt, visitor: &mut impl FnMut(&Stmt)) {
    match stmt {
        Stmt::FunctionDef(f) => walk_stmts(parsed, &f.body, visitor),
        Stmt::AsyncFunctionDef(f) => walk_stmts(parsed, &f.body, visitor),
        Stmt::ClassDef(c) => walk_stmts(parsed, &c.body, visitor),
        Stmt::If(i) => {
            walk_stmts(parsed, &i.body, visitor);
            walk_stmts(parsed, &i.orelse, visitor);
        }
        Stmt::While(w) => {
            walk_stmts(parsed, &w.body, visitor);
            walk_stmts(parsed, &w.orelse, visitor);
        }
        Stmt::For(fr) => {
            walk_stmts(parsed, &fr.body, visitor);
            walk_stmts(parsed, &fr.orelse, visitor);
        }
        Stmt::AsyncFor(fr) => {
            walk_stmts(parsed, &fr.body, visitor);
            walk_stmts(parsed, &fr.orelse, visitor);
        }
        Stmt::With(w) => walk_stmts(parsed, &w.body, visitor),
        Stmt::AsyncWith(w) => walk_stmts(parsed, &w.body, visitor),
        Stmt::Try(t) => walk_try_stmts(
            parsed,
            t.body.as_slice(),
            &t.handlers,
            t.orelse.as_slice(),
            t.finalbody.as_slice(),
            visitor,
        ),
        Stmt::TryStar(t) => walk_try_stmts(
            parsed,
            t.body.as_slice(),
            &t.handlers,
            t.orelse.as_slice(),
            t.finalbody.as_slice(),
            visitor,
        ),
        Stmt::Match(m) => {
            for case in &m.cases {
                walk_stmts(parsed, &case.body, visitor);
            }
        }
        _ => {}
    }
}

fn walk_try_stmts(
    parsed: &ParsedFile,
    body: &[Stmt],
    handlers: &[ExceptHandler],
    orelse: &[Stmt],
    finalbody: &[Stmt],
    visitor: &mut impl FnMut(&Stmt),
) {
    walk_stmts(parsed, body, visitor);
    for h in handlers {
        let ExceptHandler::ExceptHandler(h) = h;
        walk_stmts(parsed, &h.body, visitor);
    }
    walk_stmts(parsed, orelse, visitor);
    walk_stmts(parsed, finalbody, visitor);
}

/// Expression literals on a statement node. Child statement bodies are reached via [`walk_stmts`].
fn visit_stmt_exprs(stmt: &Stmt, f: &mut impl FnMut(&Expr)) {
    match stmt {
        Stmt::FunctionDef(fd) => fd.decorator_list.iter().for_each(|d| walk_expr(d, f)),
        Stmt::AsyncFunctionDef(fd) => fd.decorator_list.iter().for_each(|d| walk_expr(d, f)),
        Stmt::ClassDef(cd) => cd.decorator_list.iter().for_each(|d| walk_expr(d, f)),
        Stmt::If(i) => walk_expr(&i.test, f),
        Stmt::While(w) => walk_expr(&w.test, f),
        Stmt::For(fr) => walk_expr(&fr.iter, f),
        Stmt::AsyncFor(fr) => walk_expr(&fr.iter, f),
        Stmt::With(w) => visit_with_items(&w.items, f),
        Stmt::AsyncWith(w) => visit_with_items(&w.items, f),
        Stmt::Try(t) => visit_try_handler_type_exprs(&t.handlers, f),
        Stmt::TryStar(t) => visit_try_handler_type_exprs(&t.handlers, f),
        Stmt::Match(m) => {
            walk_expr(&m.subject, f);
            for case in &m.cases {
                walk_pattern(&case.pattern, f);
                if let Some(g) = &case.guard {
                    walk_expr(g, f);
                }
            }
        }
        Stmt::Assign(a) => {
            a.targets.iter().for_each(|t| walk_expr(t, f));
            walk_expr(&a.value, f);
        }
        Stmt::AnnAssign(a) => {
            walk_expr(&a.target, f);
            walk_expr(&a.annotation, f);
            if let Some(v) = &a.value {
                walk_expr(v, f);
            }
        }
        Stmt::AugAssign(a) => {
            walk_expr(&a.target, f);
            walk_expr(&a.value, f);
        }
        Stmt::Return(r) => {
            if let Some(v) = &r.value {
                walk_expr(v, f);
            }
        }
        Stmt::Expr(e) => walk_expr(&e.value, f),
        Stmt::Raise(r) => {
            if let Some(e) = &r.exc {
                walk_expr(e, f);
            }
            if let Some(c) = &r.cause {
                walk_expr(c, f);
            }
        }
        Stmt::Assert(a) => {
            walk_expr(&a.test, f);
            if let Some(m) = &a.msg {
                walk_expr(m, f);
            }
        }
        Stmt::Delete(d) => d.targets.iter().for_each(|t| walk_expr(t, f)),
        // Other stmt kinds (pass, break, import, type alias when present) carry no inline exprs.
        _ => {}
    }
}

fn visit_with_items(items: &[rustpython_ast::WithItem], f: &mut impl FnMut(&Expr)) {
    for item in items {
        walk_expr(&item.context_expr, f);
        if let Some(v) = &item.optional_vars {
            walk_expr(v, f);
        }
    }
}

fn visit_try_handler_type_exprs(handlers: &[ExceptHandler], f: &mut impl FnMut(&Expr)) {
    for h in handlers {
        let ExceptHandler::ExceptHandler(h) = h;
        if let Some(ty) = &h.type_ {
            walk_expr(ty, f);
        }
    }
}

fn walk_pattern(p: &Pattern, f: &mut impl FnMut(&Expr)) {
    match p {
        Pattern::MatchValue(v) => walk_expr(&v.value, f),
        Pattern::MatchSequence(s) => s.patterns.iter().for_each(|p| walk_pattern(p, f)),
        Pattern::MatchMapping(m) => m.keys.iter().for_each(|e| walk_expr(e, f)),
        Pattern::MatchClass(c) => {
            walk_expr(&c.cls, f);
            c.kwd_patterns.iter().for_each(|p| walk_pattern(p, f));
        }
        _ => {}
    }
}

fn walk_expr(expr: &Expr, f: &mut impl FnMut(&Expr)) {
    f(expr);
    match expr {
        Expr::BoolOp(b) => b.values.iter().for_each(|v| walk_expr(v, f)),
        Expr::NamedExpr(n) => {
            walk_expr(&n.target, f);
            walk_expr(&n.value, f);
        }
        Expr::BinOp(b) => {
            walk_expr(&b.left, f);
            walk_expr(&b.right, f);
        }
        Expr::UnaryOp(u) => walk_expr(&u.operand, f),
        Expr::Lambda(l) => walk_expr(&l.body, f),
        Expr::IfExp(i) => {
            walk_expr(&i.test, f);
            walk_expr(&i.body, f);
            walk_expr(&i.orelse, f);
        }
        Expr::Dict(d) => {
            for (k, v) in d.keys.iter().zip(&d.values) {
                if let Some(k) = k {
                    walk_expr(k, f);
                }
                walk_expr(v, f);
            }
        }
        Expr::Set(s) => s.elts.iter().for_each(|e| walk_expr(e, f)),
        Expr::ListComp(lc) => walk_comprehension(lc.elt.as_ref(), &lc.generators, f),
        Expr::SetComp(sc) => walk_comprehension(sc.elt.as_ref(), &sc.generators, f),
        Expr::GeneratorExp(g) => walk_comprehension(g.elt.as_ref(), &g.generators, f),
        Expr::DictComp(dc) => {
            walk_expr(&dc.key, f);
            walk_expr(&dc.value, f);
            for gen in &dc.generators {
                walk_expr(&gen.iter, f);
                gen.ifs.iter().for_each(|i| walk_expr(i, f));
            }
        }
        Expr::Await(a) => walk_expr(&a.value, f),
        Expr::Yield(y) => {
            if let Some(v) = &y.value {
                walk_expr(v, f);
            }
        }
        Expr::YieldFrom(y) => walk_expr(&y.value, f),
        Expr::Compare(c) => {
            walk_expr(&c.left, f);
            c.comparators.iter().for_each(|e| walk_expr(e, f));
        }
        Expr::Call(call) => {
            walk_expr(&call.func, f);
            call.args.iter().for_each(|a| walk_expr(a, f));
            call.keywords.iter().for_each(|kw| walk_expr(&kw.value, f));
        }
        Expr::FormattedValue(fv) => walk_expr(&fv.value, f),
        Expr::Attribute(a) => walk_expr(&a.value, f),
        Expr::Subscript(s) => {
            walk_expr(&s.value, f);
            walk_expr(&s.slice, f);
        }
        Expr::Starred(s) => walk_expr(&s.value, f),
        Expr::List(l) => l.elts.iter().for_each(|e| walk_expr(e, f)),
        Expr::Tuple(t) => t.elts.iter().for_each(|e| walk_expr(e, f)),
        Expr::Slice(sl) => {
            if let Some(l) = &sl.lower {
                walk_expr(l, f);
            }
            if let Some(u) = &sl.upper {
                walk_expr(u, f);
            }
            if let Some(s) = &sl.step {
                walk_expr(s, f);
            }
        }
        _ => {}
    }
}

fn walk_comprehension(
    elt: &Expr,
    generators: &[rustpython_ast::Comprehension],
    f: &mut impl FnMut(&Expr),
) {
    walk_expr(elt, f);
    for gen in generators {
        walk_expr(&gen.iter, f);
        gen.ifs.iter().for_each(|i| walk_expr(i, f));
    }
}

/// Line span of executable statements in a function body (excludes nested def/class headers).
/// Does not count the `def`/`async def` header line — only body statements (B9).
fn stmt_line_span(parsed: &ParsedFile, body: &[Stmt]) -> usize {
    let mut min_line = usize::MAX;
    let mut max_line = 0usize;
    for stmt in body {
        if matches!(
            stmt,
            Stmt::FunctionDef(_) | Stmt::AsyncFunctionDef(_) | Stmt::ClassDef(_)
        ) {
            continue;
        }
        let (start_line, _) = line_col(&parsed.source, offset(stmt.start()));
        let (end_line, _) = line_col(&parsed.source, offset(stmt.end()));
        min_line = min_line.min(start_line);
        max_line = max_line.max(end_line);
    }
    if min_line == usize::MAX {
        return 0;
    }
    max_line.saturating_sub(min_line).saturating_add(1)
}

fn count_branches(stmts: &[Stmt]) -> usize {
    let mut n = 0;
    for stmt in stmts {
        match stmt {
            Stmt::If(i) => {
                n += 1;
                n += count_branches(&i.body);
                n += count_branches(&i.orelse);
            }
            Stmt::While(w) => {
                n += 1;
                n += count_branches(&w.body);
                n += count_branches(&w.orelse);
            }
            Stmt::For(fr) => {
                n += 1;
                n += count_branches(&fr.body);
                n += count_branches(&fr.orelse);
            }
            Stmt::AsyncFor(fr) => {
                n += 1;
                n += count_branches(&fr.body);
                n += count_branches(&fr.orelse);
            }
            Stmt::Try(t) => {
                n += 1 + t.handlers.len();
                n += count_branches(&t.body);
                for h in &t.handlers {
                    let ExceptHandler::ExceptHandler(h) = h;
                    n += count_branches(&h.body);
                }
                n += count_branches(&t.orelse);
                n += count_branches(&t.finalbody);
            }
            Stmt::TryStar(t) => {
                n += 1 + t.handlers.len();
                n += count_branches(&t.body);
                for h in &t.handlers {
                    let ExceptHandler::ExceptHandler(h) = h;
                    n += count_branches(&h.body);
                }
                n += count_branches(&t.orelse);
                n += count_branches(&t.finalbody);
            }
            Stmt::Match(m) => {
                n += m.cases.len();
                for case in &m.cases {
                    n += count_branches(&case.body);
                }
            }
            Stmt::With(w) => n += count_branches(&w.body),
            Stmt::AsyncWith(w) => n += count_branches(&w.body),
            _ => {}
        }
    }
    n
}

fn max_conditional_depth(stmts: &[Stmt], depth: usize) -> usize {
    let mut max = depth;
    for stmt in stmts {
        let nested = match stmt {
            Stmt::If(i) => {
                let d = depth + 1;
                max_conditional_depth(&i.body, d).max(max_conditional_depth(&i.orelse, d))
            }
            Stmt::While(w) => max_conditional_depth(&w.body, depth + 1),
            Stmt::For(fr) => max_conditional_depth(&fr.body, depth + 1),
            Stmt::AsyncFor(fr) => max_conditional_depth(&fr.body, depth + 1),
            Stmt::Try(t) => depth_try(
                t.body.as_slice(),
                &t.handlers,
                t.orelse.as_slice(),
                t.finalbody.as_slice(),
                depth,
            ),
            Stmt::TryStar(t) => depth_try(
                t.body.as_slice(),
                &t.handlers,
                t.orelse.as_slice(),
                t.finalbody.as_slice(),
                depth,
            ),
            Stmt::Match(m) => m
                .cases
                .iter()
                .map(|c| max_conditional_depth(&c.body, depth + 1))
                .max()
                .unwrap_or(depth),
            Stmt::With(w) => max_conditional_depth(&w.body, depth),
            Stmt::AsyncWith(w) => max_conditional_depth(&w.body, depth),
            _ => depth,
        };
        max = max.max(nested);
    }
    max
}

fn depth_try(
    body: &[Stmt],
    handlers: &[ExceptHandler],
    orelse: &[Stmt],
    finalbody: &[Stmt],
    depth: usize,
) -> usize {
    let d = depth + 1;
    let mut m = max_conditional_depth(body, d);
    for h in handlers {
        let ExceptHandler::ExceptHandler(h) = h;
        m = m.max(max_conditional_depth(&h.body, d));
    }
    m = m.max(max_conditional_depth(orelse, d));
    m.max(max_conditional_depth(finalbody, d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{PythonParser, RustPythonParser};
    use std::path::Path;

    #[test]
    fn nested_function_depth_not_attributed_to_outer() {
        let source = r#"
def outer():
    if a:
        if b:
            if c:
                pass

    def inner():
        if e:
            if f:
                if g:
                    if h:
                        pass
"#;
        let parsed = RustPythonParser
            .parse_file(source, Path::new("t.py"))
            .unwrap();
        let metrics = collect_function_metrics(&parsed);
        let outer = metrics.iter().find(|m| m.name == "outer").unwrap();
        let inner = metrics.iter().find(|m| m.name == "inner").unwrap();
        assert_eq!(outer.max_conditional_depth, 3);
        assert_eq!(inner.max_conditional_depth, 4);
    }

    #[test]
    fn walk_exprs_finds_print_inside_nested_function() {
        let source = "def outer():\n    def inner():\n        print('x')\n";
        let parsed = RustPythonParser
            .parse_file(source, Path::new("t.py"))
            .unwrap();
        let mut hits = 0;
        walk_exprs_in_module(&parsed, &mut |expr| {
            if is_print_call(expr) {
                hits += 1;
            }
        });
        assert_eq!(hits, 1);
    }
}
