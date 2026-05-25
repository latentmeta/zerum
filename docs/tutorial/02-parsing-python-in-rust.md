# Parsing Python in Rust

Phase 1 uses **rustpython-parser** behind a small abstraction so the backend can change later.

## Parser trait

```rust
pub trait PythonParser: Send + Sync {
    fn parse_file(&self, source: &str, path: &Path) -> Result<ParsedFile>;
}
```

`ParsedFile` holds the source text and a `Mod` AST root. Checks must not depend on parser internals—only on `ParsedFile` and helpers in `core/ast_util.rs`.

## Why rustpython-parser?

- Pure Rust (no Python runtime in the CLI)
- Produces a Python 3 AST aligned with `rustpython-ast`
- Good enough for governance rules on `.py` files

**Tradeoff:** A file that fails to parse is skipped with a warning on stderr. If every file fails, the CLI exits with code 2.

## AST version notes (0.4)

`rustpython-ast` 0.4 uses **tuple-style** enum variants (`Stmt::FunctionDef(f)`), not struct variants. Function parameters use `Arguments` with per-argument `ArgWithDefault`.

## Line and column

Byte offsets from `Ranged::start()` are converted via `line_col(source, offset)` in `parser/mod.rs`. Line numbers are 1-based. Column is a **byte offset** within the line (not grapheme-aware).

## Files to read

- `src/parser/python.rs` — `RustPythonParser`
- `src/core/ast_util.rs` — walks and metrics

## Limitations

- Python only in Phase 1
- No type information; rules are syntactic
- Async variants (`AsyncFunctionDef`, etc.) are handled explicitly in metrics and walks
- Discovery surfaces walk errors (missing roots, permission failures) instead of silently skipping them
