# Zerum

**Repository:** [github.com/latentmeta/zerum](https://github.com/latentmeta/zerum)

**Zerum** is a Rust-native, deterministic-first code governance platform for Python.

It combines:

1. **Native deterministic checks** (ZR001–ZR010) — complexity, design, warnings, AI placeholders, architecture imports
2. **External checker orchestration** (planned) — Ruff, Mypy, Bandit, and others
3. **Auditable LLM-assisted review** (planned, opt-in only)

Zerum is **not** a Ruff clone. It focuses on explainable governance, maintainability, and architectural boundaries.

## Install

```bash
cargo install zerum
```

Requires a [Rust toolchain](https://rustup.rs/) (see `rust-version` in `Cargo.toml`).

## Quick start

```bash
zerum list-checks
zerum check path/to/python/project
zerum review path/to/python/project   # same analysis, exits 0 when issues exist
zerum explain ZR001
zerum init
```

From a git checkout:

```bash
git clone https://github.com/latentmeta/zerum.git
cd zerum
cargo build
cargo run -- check path/to/python/project
```

`zerum init` writes `zerum.toml` from `zerum.toml.example`. You can also copy the example file manually.

## Exit codes (`check`)

| Code | Meaning |
|------|---------|
| 0 | No issues |
| 1 | Issues found |
| 2 | Operational error (missing path, parse/read failure on all files, CLI error) |

`zerum review` uses the same analysis and reporters but exits **0** when issues are found (only **2** on operational failure).

## Checks (v0.1.0)

| ID | Name |
|----|------|
| ZR001 | too-many-branches |
| ZR002 | too-many-arguments |
| ZR003 | long-function |
| ZR004 | nested-conditionals |
| ZR005 | broad-except |
| ZR006 | print-debugging |
| ZR007 | mutable-default-argument |
| ZR008 | god-class |
| ZR009 | ai-generated-placeholder-comment |
| ZR010 | forbidden-architecture-import |

## Output formats

```bash
cargo run -- check . --format human    # default
cargo run -- check . --format json
cargo run -- check . --format sarif
cargo run -- check . --format markdown
```

## Tutorial

Educational material lives under [`docs/tutorial/`](docs/tutorial/):

- [00 — Introduction](docs/tutorial/00-introduction.md)
- [01 — Static analysis basics](docs/tutorial/01-static-analysis-basics.md)
- [02 — Parsing Python in Rust](docs/tutorial/02-parsing-python-in-rust.md)
- [03 — Building a rule engine](docs/tutorial/03-building-a-rule-engine.md)
- [12 — Roadmap](docs/tutorial/12-roadmap.md)

## Development

```bash
cargo test
cargo run -- check tests/fixtures/bad_project        # exits 1
cargo run -- check tests/fixtures/arch_violation     # ZR010, exits 1
cargo run -- check tests/fixtures/simple_project     # exits 0
```

## License

MIT — see [LICENSE](LICENSE).
