use crate::core::ast_util::{
    collect_class_metrics, collect_function_metrics, collect_import_modules,
};
use crate::parser::ParsedFile;

#[derive(Debug, Clone)]
pub struct FunctionView {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub arg_count: usize,
    pub body_lines: usize,
    pub branch_count: usize,
    pub max_conditional_depth: usize,
}

#[derive(Debug, Clone)]
pub struct ClassView {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub method_count: usize,
}

#[derive(Debug, Clone)]
pub struct ImportView {
    pub module: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct CommentView {
    pub text: String,
    pub line: usize,
    pub column: usize,
}

pub struct SourceModel<'a> {
    parsed: &'a ParsedFile,
}

impl<'a> SourceModel<'a> {
    pub fn from_parsed(parsed: &'a ParsedFile) -> Self {
        Self { parsed }
    }

    pub fn functions(&self) -> Vec<FunctionView> {
        collect_function_metrics(self.parsed)
            .into_iter()
            .map(|m| FunctionView {
                name: m.name,
                line: m.line,
                column: m.column,
                arg_count: m.arg_count,
                body_lines: m.body_lines,
                branch_count: m.branch_count,
                max_conditional_depth: m.max_conditional_depth,
            })
            .collect()
    }

    pub fn classes(&self) -> Vec<ClassView> {
        collect_class_metrics(self.parsed)
            .into_iter()
            .map(|m| ClassView {
                name: m.name,
                line: m.line,
                column: m.column,
                method_count: m.method_count,
            })
            .collect()
    }

    pub fn imports(&self) -> Vec<ImportView> {
        collect_import_modules(self.parsed)
            .into_iter()
            .map(|(module, line, column)| ImportView {
                module,
                line,
                column,
            })
            .collect()
    }

    pub fn comments(&self) -> Vec<CommentView> {
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
    }
}
