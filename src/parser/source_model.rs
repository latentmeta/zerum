use crate::core::ast_util::walk_stmts_in_module;
use crate::parser::{line_col, ParsedFile};
use rustpython_ast::{Expr, Ranged, Stmt};
use std::cell::OnceCell;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct FunctionView {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub arg_count: usize,
    pub body_lines: usize,
    pub branch_count: usize,
    pub max_conditional_depth: usize,
    pub has_docstring: bool,
}

#[derive(Debug, Clone)]
pub struct ClassView {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub method_count: usize,
    pub public_method_count: usize,
    pub instance_var_count: usize,
}

#[derive(Debug, Clone)]
pub struct ImportView {
    pub module: String,
    pub line: usize,
    pub column: usize,
    pub is_from: bool,
}

#[derive(Debug, Clone)]
pub struct CommentView {
    pub text: String,
    pub line: usize,
    pub column: usize,
}

pub struct SourceModel<'a> {
    parsed: &'a ParsedFile,
    functions: OnceCell<Vec<FunctionView>>,
    classes: OnceCell<Vec<ClassView>>,
    imports: OnceCell<Vec<ImportView>>,
    comments: OnceCell<Vec<CommentView>>,
}

impl<'a> SourceModel<'a> {
    pub fn from_parsed(parsed: &'a ParsedFile) -> Self {
        Self {
            parsed,
            functions: OnceCell::new(),
            classes: OnceCell::new(),
            imports: OnceCell::new(),
            comments: OnceCell::new(),
        }
    }

    pub fn functions(&self) -> &[FunctionView] {
        self.functions.get_or_init(|| {
            crate::core::ast_util::collect_function_metrics(self.parsed)
                .into_iter()
                .map(|m| FunctionView {
                    name: m.name,
                    line: m.line,
                    column: m.column,
                    arg_count: m.arg_count,
                    body_lines: m.body_lines,
                    branch_count: m.branch_count,
                    max_conditional_depth: m.max_conditional_depth,
                    has_docstring: m.has_docstring,
                })
                .collect()
        })
    }

    pub fn classes(&self) -> &[ClassView] {
        self.classes
            .get_or_init(|| collect_class_views(self.parsed))
    }

    pub fn imports(&self) -> &[ImportView] {
        self.imports.get_or_init(|| collect_imports(self.parsed))
    }

    pub fn comments(&self) -> &[CommentView] {
        self.comments.get_or_init(|| {
            self.parsed
                .source
                .lines()
                .enumerate()
                .filter_map(|(idx, line)| {
                    let hash = line.find('#')?;
                    Some(CommentView {
                        text: line[hash..].to_string(),
                        line: idx + 1,
                        column: hash + 1,
                    })
                })
                .collect()
        })
    }
}

fn collect_imports(parsed: &ParsedFile) -> Vec<ImportView> {
    let mut out = Vec::new();
    walk_stmts_in_module(parsed, &mut |stmt| match stmt {
        Stmt::Import(i) => {
            for alias in &i.names {
                let (line, column) = line_col(&parsed.source, alias.start().into());
                out.push(ImportView {
                    module: alias.name.to_string(),
                    line,
                    column,
                    is_from: false,
                });
            }
        }
        Stmt::ImportFrom(i) => {
            let module = i.module.as_ref().map(|m| m.to_string()).unwrap_or_default();
            for alias in &i.names {
                let (line, column) = line_col(&parsed.source, alias.start().into());
                let full = if module.is_empty() {
                    alias.name.to_string()
                } else {
                    format!("{module}.{}", alias.name)
                };
                out.push(ImportView {
                    module: full,
                    line,
                    column,
                    is_from: true,
                });
            }
        }
        _ => {}
    });
    out
}

fn collect_class_views(parsed: &ParsedFile) -> Vec<ClassView> {
    let mut out = Vec::new();
    walk_stmts_in_module(parsed, &mut |stmt| {
        if let Stmt::ClassDef(c) = stmt {
            let (line, column) = line_col(&parsed.source, c.start().into());
            let mut method_count = 0;
            let mut public_method_count = 0;
            let mut instance_vars = HashSet::new();
            for s in &c.body {
                match s {
                    Stmt::FunctionDef(f) => {
                        method_count += 1;
                        if !f.name.as_str().starts_with('_') {
                            public_method_count += 1;
                        }
                        if f.name.as_str() == "__init__" {
                            for body_stmt in &f.body {
                                if let Stmt::Assign(a) = body_stmt {
                                    for target in &a.targets {
                                        if let Expr::Attribute(attr) = target {
                                            if matches!(
                                                attr.value.as_ref(),
                                                Expr::Name(n) if n.id.as_str() == "self"
                                            ) {
                                                instance_vars.insert(attr.attr.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Stmt::AsyncFunctionDef(f) => {
                        method_count += 1;
                        if !f.name.as_str().starts_with('_') {
                            public_method_count += 1;
                        }
                    }
                    _ => {}
                }
            }
            out.push(ClassView {
                name: c.name.to_string(),
                line,
                column,
                method_count,
                public_method_count,
                instance_var_count: instance_vars.len(),
            });
        }
    });
    out
}
