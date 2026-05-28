# Test coverage (v0.2.0)

The prompt target is **≥70%** line coverage for the deterministic core.

## How to measure

```bash
cargo tarpaulin --out Stdout -- --test-threads=1
```

Requires [cargo-tarpaulin](https://github.com/xd009642/tarpaulin).

## Latest measurement

**2026-05-27:** `cargo tarpaulin` → **84.93%** line coverage (1876/2209 lines). Exceeds the ≥70% prompt target.

| Area | Notes |
|------|--------|
| `src/core/` | Registry, config, AST helpers — well covered by unit + integration tests |
| `src/checks/catalog.rs` | Catalog wiring; detector branches covered incrementally via `rule_precision_tests` |
| `src/checks/catalog_detectors.rs` | AST paths covered per rule with positive/negative temp-file tests |
| `src/cli.rs` | CLI smoke via `tests/cli_*` |
| `src/reporters/` | Reporter unit tests + snapshots |

Coverage is tracked toward 70% as more per-rule precision tests land. Heuristic-only rules contribute less branch coverage until converted to AST detectors or given fixture tests.

## CI recommendation (post–v0.2.0)

Add a non-blocking tarpaulin job reporting line coverage, then make it a required gate once stable above 70%.
