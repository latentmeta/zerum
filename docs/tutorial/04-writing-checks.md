# Writing checks (v0.2.0)

v0.2.0 ships roughly **75 deterministic rules** from a single catalog (`src/checks/catalog.rs`) instead of ten hand-written modules. New rules should prefer **AST-precise** detectors over line-pattern heuristics.

## Catalog + detectors

| Piece | Role |
|-------|------|
| `build_catalog()` | Registers every `ZR###` rule with metadata and a `Detector` variant |
| `CatalogCheck` | Implements `Check`; dispatches to metrics, AST walks, or `catalog_detectors` |
| `catalog_detectors.rs` | Shared AST logic for rules that need precision (TODO context, `len` compares, identity `map`, etc.) |
| `SourceModel` | Semantic views: functions, classes, imports, comments |

Adding a rule:

1. Choose a detector: reuse `LongFunction`, `BroadExcept`, or add a `CatalogDetector` variant.
2. Register with `rule("ZR###", "kebab-name", Category::…, Severity::…, detector)`.
3. Add **positive and negative** tests in `tests/rule_precision_tests.rs` or a category fixture.
4. Run `cargo test` and update snapshots only when output changes are intentional.

## AST-precise vs heuristic

**Prefer AST** when the rule has a clear syntactic definition:

- `Stmt::Assert`, `Stmt::Return`, `Expr::Compare` with `len()`
- `except` handlers with only `pass`
- `if False:` / `if True:` branches

**Heuristics** (`PatternAny` / `PatternComment`) remain for stylistic consistency rules (ZR101–110) where full semantic analysis is expensive and false positives are acceptable at low severity.

## Example: identity passthrough (ZR310)

```rust
// Simplified: one positional arg, body is only `return <same name>`
walk_stmts_in_module(ctx.parsed, &mut |stmt| {
    if let Stmt::FunctionDef(f) = stmt {
        // … match single arg and Stmt::Return(Expr::Name(…))
    }
});
```

## Design decisions

| Decision | Rationale |
|----------|-----------|
| Central catalog | One place for ids, categories, and explain metadata; avoids registry drift |
| `catalog_detectors` module | Keeps `catalog.rs` readable; AST helpers stay testable in isolation |
| `SourceModel` boundary | Checks do not re-parse; metrics and comments are cached per file |
| No plugins in v0.2.0 | Compile-time registry only; dynamic loading is explicitly out of scope |

## Alternatives considered

- **Per-category crates/modules** — clearer ownership but harder to grep; deferred until rule count stabilizes
- **Macro-generated rule tables** — less readable for tutorials; rejected for v0.2.0

## Complexity and limitations

- Walkers in `ast_util` do not model all Python 3.12 syntax equally; edge cases may be missed.
- Pattern rules scan source text and can match string literals or comments incorrectly.
- Architecture rule ZR207 requires `[[checks.ZR207.rules]]` in discovered `zerum.toml`; path matching uses directory segments.

## Tradeoffs

Stricter AST rules reduce noise but cost implementation time. Heuristic rules ship faster and teach conventions, but need `zerum explain` and fixture tests so users can judge fit.

## Related chapters

- [05 — Explain mode and configuration](05-explain-mode-and-configuration.md)
- [03 — Building a rule engine](03-building-a-rule-engine.md)
