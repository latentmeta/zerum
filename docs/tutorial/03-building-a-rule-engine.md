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

`CheckRegistry::new()` registers all Phase 1 checks. `DeterministicAnalyzer` iterates the registry and respects `config.is_check_enabled(id)`.

Adding a check:

1. Implement `Check` in `src/checks/...`
2. Register in `src/core/registry.rs`
3. Add unit test and fixture coverage
4. Document in tutorial chapter 04

## Configuration hook

`Config::check_config("ZR001")` returns per-check settings (e.g. `max_branches`). Missing entries default to enabled with built-in thresholds.

## Design choices

| Choice | Rationale |
|--------|-----------|
| `Arc<dyn Check>` | Simple v0.1.0; few checks, no dynamic plugins yet |
| Shared `ast_util` | One AST walk for metrics; avoids duplicated visitors |
| Stable issue ids | `zerum explain ZR001` and SARIF `ruleId` stay aligned |

## Alternatives considered

- **Full visitor codegen** — deferred; manual walks are clearer for teaching
- **Plugin `.so` loading** — explicitly out of scope for v0.1.0 per project prompt

## Next chapter

`04-writing-checks.md` will walk through implementing a new check end-to-end.
