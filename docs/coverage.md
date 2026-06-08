# Test coverage (v0.4.0)

The prompt target is **≥70%** line coverage for the deterministic core.

## How to measure

```bash
cargo tarpaulin --out Stdout -- --test-threads=1
```

Requires [cargo-tarpaulin](https://github.com/xd009642/tarpaulin).

Match CI locally (Linux uses the **llvm** engine so integration tests and subprocess `zerum` invocations are counted):

```bash
rustup component add llvm-tools-preview
cargo tarpaulin \
  --engine llvm \
  --all-targets \
  --all-features \
  --follow-exec \
  --out Stdout \
  --fail-under 70 \
  -- \
  --test-threads=1
```

## Latest measurement

**2026-06-07:** `cargo tarpaulin` → **~85%** line coverage (2256/2651 lines). Exceeds the ≥70% target.

| Area | Notes |
|------|--------|
| `src/core/` | Registry, config, AST helpers — well covered by unit + integration tests |
| `src/checks/catalog.rs` | Catalog wiring; detector branches covered via `rule_precision_tests` |
| `src/checks/catalog_detectors.rs` | AST paths covered per rule with positive/negative temp-file tests |
| `src/cli.rs` | CLI via `tests/cli_*` (subprocess; needs `--follow-exec` in CI) |
| `src/reporters/` | Reporter unit tests + snapshots |

## CI

The coverage job in [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) enforces **≥70%** with:

- `--engine llvm` — reliable on `ubuntu-latest` (default ptrace often reports ~20% because integration tests spawn the `zerum` binary)
- `--all-targets` / `--all-features` — same surface as the main test job
- `--follow-exec` — attribute coverage when tests run `zerum` as a subprocess

Do **not** lower `--fail-under` to ~20% to green CI; that only hides mis-measured coverage.
