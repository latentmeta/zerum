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

**Tradeoff:** Parser errors skip a file with a warning; we do not ship a full type checker in v0.1.0.

## AST version notes (0.4)

`rustpython-ast` 0.4 uses **tuple-style** enum variants (`Stmt::FunctionDef(f)`), not struct variants. Function parameters use `Arguments` with per-argument `ArgWithDefault`—not legacy `defaults` / `kw_defaults` vectors.

## Line and column

Byte offsets from `Ranged::start()` are converted via `line_col(source, offset)` in `parser/mod.rs` (1-based line, 1-based column).

## Files to read

- `src/parser/python.rs` — `RustPythonParser`
- `src/core/ast_util.rs` — walks and metrics

## Limitations

- Python only in Phase 1
- No type information; rules are syntactic
- Async variants (`AsyncFunctionDef`, etc.) are handled explicitly
