# Zerum

[![crates.io](https://img.shields.io/crates/v/zerum.svg)](https://crates.io/crates/zerum)
[![docs.rs](https://docs.rs/zerum/badge.svg)](https://docs.rs/zerum)

**Zerum** is a Rust-native, deterministic-first code governance tool for Python — *Credo for Python*.

v0.2.0 delivers roughly **75 native checks** (ZR001–ZR510) with explainable findings, `human` and `json` output, and no dependency on external linters or LLMs.

Zerum is **not** a Ruff replacement. It focuses on maintainability, consistency, architecture boundaries, and deterministic AI-slop patterns.

## Install

```bash
cargo install zerum
```

Requires Rust **1.70+** (see `rust-version` in `Cargo.toml`).

## Quick start

```bash
cargo build
cargo run -- list-checks
cargo run -- check path/to/python/project
cargo run -- explain ZR001
cargo run -- init
```

`zerum init` writes `zerum.toml` from `zerum.toml.example`.

## Exit codes (`check`)

| Code | Meaning |
|------|---------|
| 0 | No issues |
| 1 | Issues found |
| 2 | Operational error (missing path, parse/read failure on all files, CLI error) |

## Rule categories (v0.2.0)

| Range | Category |
|-------|----------|
| ZR001–015 | Readability |
| ZR101–110 | Consistency |
| ZR201–210 | Design |
| ZR301–315 | Refactor |
| ZR401–415 | Warning |
| ZR501–510 | AI (deterministic) |

Run `zerum list-checks` for the full catalog. Use `zerum explain ZR###` for rationale, false positives, tradeoffs, and remediation.

## Output formats

```bash
cargo run -- check . --format human    # default
cargo run -- check . --format json
cargo run -- check . --profile strict
cargo run -- check . --with-external ruff
cargo run -- list-checkers
```

Optional external checkers (Ruff) and SARIF (`--features sarif`) are available from **v0.4.0**. Use the **default profile** for low noise on greenfield code; use `--profile strict` for full catalog coverage.

## Tutorial

Educational material lives under [`docs/tutorial/`](docs/tutorial/):

- [00 — Introduction](docs/tutorial/00-introduction.md)
- [01 — Static analysis basics](docs/tutorial/01-static-analysis-basics.md)
- [02 — Parsing Python in Rust](docs/tutorial/02-parsing-python-in-rust.md)
- [03 — Building a rule engine](docs/tutorial/03-building-a-rule-engine.md)
- [04 — Writing checks](docs/tutorial/04-writing-checks.md)
- [05 — Explain mode and configuration](docs/tutorial/05-explain-mode-and-configuration.md)
- [06 — Config and profiles](docs/tutorial/06-config-profiles.md)
- [12 — Roadmap](docs/tutorial/12-roadmap.md)

## Development

CI runs lint, tests, coverage, and regression suites on push/PR (see `.github/workflows/ci.yml`).

```bash
cargo test
cargo clippy -- -D warnings
cargo run -- check tests/fixtures/bad_project
cargo run -- check tests/fixtures/arch_violation
```

Category fixtures: `consistency_project`, `refactor_project`, `design_project`, `ai_slop_project`, `warning_project`.

## Changelog

See [CHANGELOG.md](CHANGELOG.md). Release notes: [v0.1.0](docs/RELEASE_v0.1.0.md) · [v0.2.0](docs/RELEASE_v0.2.0.md). Publishing: [docs/RELEASING.md](docs/RELEASING.md).

## License

MIT — see [LICENSE](LICENSE).
