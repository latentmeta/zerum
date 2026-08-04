# Building a rule engine

The rule engine is intentionally small: a **registry** of types implementing one **trait**.

## Check trait

```rust
pub trait Check: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn category(&self) -> Category;
    fn severity(&self) -> Severity;
    fn explanation(&self) -> &'static str;
    fn remediation(&self) -> &'static str;
    fn run(&self, ctx: &CheckContext) -> Vec<Issue>;
}
```

`CheckContext` carries the file path, source, parsed AST, and `Config`.

## Registry

`CheckRegistry::new()` calls `build_catalog()` (~75 rules in v0.2.0). `DeterministicAnalyzer` iterates the registry and respects `config.is_check_enabled(id)`.

Adding a check:

1. Add a `Detector` variant in `src/checks/catalog.rs` or `catalog_detectors.rs`
2. Register with `rule(...)` in `build_catalog()`
3. Add unit tests and fixture coverage
4. See [04 — Writing checks](04-writing-checks.md) for AST vs heuristic guidance

## Configuration hook

`Config::check_config("ZR001")` returns per-check settings (e.g. `max_branches`). Missing entries default to enabled with built-in thresholds. `Config::discover` walks upward from the target path and stops at a project root (`.git`, `pyproject.toml`, or `zerum.toml`) if no config file is found.

## Design choices

| Choice | Rationale |
|--------|-----------|
| `Box<dyn Check>` | Stateless catalog entries; no dynamic plugins in v0.2.0 |
| `SourceModel` | Semantic metrics and comments without duplicating parse walks |
| Shared `ast_util` | `walk_stmts` descends statements; `walk_exprs_in_module` reuses it |
| Stable issue ids | `zerum explain ZR001` stays aligned with JSON `id` field |

## Alternatives considered

- **Full visitor codegen** — deferred; manual walks are clearer for teaching
- **Plugin `.so` loading** — explicitly out of scope for v0.1.0 per project prompt

## Related chapters

- [04 — Writing checks](04-writing-checks.md)
- [05 — Explain mode and configuration](05-explain-mode-and-configuration.md)
