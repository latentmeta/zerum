# Changelog

All notable changes to [Zerum](https://github.com/latentmeta/zerum) are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **PyPI distribution** via [maturin](https://www.maturin.rs/) bin bindings (`pyproject.toml`): `pip install zerum` / `uv tool install zerum` / `pipx install zerum` installs the Rust CLI with no Rust toolchain required
- GitHub Actions: maturin packaging smoke on PRs; `pypi-release.yml` builds Linux/macOS/Windows wheels + sdist, Trusted Publishing to PyPI, attach assets to GitHub Release
- Packaging docs under [`docs/packaging/`](docs/packaging/)

### Planned

- Opt-in LLM layer (`--with-llm`)
- Autofix pipeline

## [0.4.0] - 2026-06-07

Post–v0.2.0 hardening: default profile, AST detectors, config profiles, external Ruff orchestration.

### Added

- **Default profile:** noisy pattern heuristics disabled when no `zerum.toml` (see `src/config/defaults.rs`)
- **Strict profile:** `zerum check --profile strict` and `zerum init --strict`
- **Profile inheritance:** `[profiles.*]` with `extends` in `zerum.toml`
- **Config validation:** invalid check ids and severity values fail at load
- **AST detectors:** ZR101 inconsistent function naming, ZR306 repeated literal, ZR506 empty wrapper
- **Curated explain text** for tier-1 rules (ZR001, ZR003, ZR005, ZR010, ZR207, ZR401–404, ZR406–407, ZR414, ZR501, ZR504)
- **`ExternalChecker` trait** and Ruff JSON adapter (`--with-external ruff`, `external_checkers` config)
- **`list-checkers`** CLI command
- **`sarif` feature flag** for optional SARIF output (`cargo build --features sarif`)
- **Tests:** `catalog_rule_matrix`, `clean_project` fixture, `list-checkers` CLI test
- **Docs:** [tutorial 06](docs/tutorial/06-config-profiles.md), [DISTRO_CLOSURE.md](docs/DISTRO_CLOSURE.md)

### Changed

- Removed unwired legacy check modules under `src/checks/{readability,design,...}/`
- CI coverage job fails below **70%** line coverage
- `zerum.toml.example` documents default vs strict and external checkers

Release notes: [docs/RELEASE_v0.4.0.md](docs/RELEASE_v0.4.0.md)

## [0.2.0] - 2026-05-27

Deterministic foundations release — ~75 native checks, catalog registry, SourceModel, human/json-only CLI.

### Added

- **Catalog:** `build_catalog()` with ZR001–ZR015, ZR101–ZR110, ZR201–ZR210, ZR301–ZR315, ZR401–ZR415, ZR501–ZR510
- **AST detectors:** `catalog_detectors` for TODO context, magic numbers, `len` compares, assert/eval/exec, except/pass patterns, identity map, else-after-return, AI placeholders, and more
- **SourceModel:** semantic views (functions, classes, imports, comments) over `ParsedFile`
- **CheckMetadata:** false positives, tradeoffs, examples on every rule
- **Config:** per-check `severity` override
- **Tests:** `rule_precision_tests`, `catalog_fixture_tests`, category fixtures, config override tests
- **Docs:** tutorial chapters 04–05, `docs/RELEASE_v0.2.0.md`, milestone review

### Changed

- Registry driven by catalog instead of ten hand-written checks
- CLI limited to `check`, `explain`, `list-checks`, `init`
- Reporters limited to `human` and `json`
- Check id semantics realigned (e.g. ZR001 is `long-function` in v0.2.0)

### Removed

- `zerum review`, `list-checkers`, `--with-llm` (stubs removed from user-facing path)
- SARIF and Markdown reporters from CLI

### Known limitations

- Many consistency/refactor rules remain pattern-based heuristics
- Broad findings on minimal fixtures; tune `zerum.toml` before strict CI gates
- Architecture rule ZR207 requires explicit `[[checks.ZR207.rules]]` configuration

## [0.1.0] - 2026-05-25

First public release — Phase 1 foundation. Deterministic Python governance in Rust; not a general-purpose linter like Ruff.

### Added

- **Core:** `Issue` model with severity, category, confidence, and per-check explanation/remediation; `Check` trait and `CheckRegistry`; `IssueBuilder` / `Issue::from_check`
- **Parser:** `PythonParser` trait and `RustPythonParser` (rustpython-parser 0.4); byte-safe `line_col` mapping
- **Checks (ZR001–ZR010):**
  - ZR001 too-many-branches
  - ZR002 too-many-arguments
  - ZR003 long-function
  - ZR004 nested-conditionals
  - ZR005 broad-except
  - ZR006 print-debugging
  - ZR007 mutable-default-argument
  - ZR008 god-class
  - ZR009 ai-generated-placeholder-comment
  - ZR010 forbidden-architecture-import
- **CLI:** `check`, `review`, `explain`, `init`, `list-checks`, `list-checkers` (placeholder)
- **Reporters:** human, JSON, Markdown, SARIF 2.1.0 (`--format`)
- **Config:** `zerum.toml` / `zerum.toml.example`; per-check thresholds; ZR010 `[[checks.ZR010.rules]]`; `Config::discover` with project-root anchoring
- **Discovery:** Python file walk with skip dirs (`.git`, `__pycache__`, venvs, caches); walk errors surfaced to callers
- **Exit codes:** `0` clean, `1` issues (`check`), `2` operational failure; `review` exits `0` when issues are found
- **Docs:** tutorial chapters 00–03 and 12 (roadmap); `docs/RELEASING.md` for crates.io publish
- **Tests:** 45 tests — per-check unit tests, CLI integration, exit-code tests, insta snapshots (human/JSON/SARIF); fixtures `simple_project`, `bad_project`, `arch_violation`

### Changed

- N/A (initial release)

### Fixed

- N/A (initial release)

### Known limitations (v0.1.0)

- ZR003 measures function **body** lines only (excludes the `def` header).
- ZR006 matches bare `print()` only, not `builtins.print` or aliases.
- ZR009 scans `#` comments only; string literals are ignored; token matching uses word boundaries.
- ZR010 does not evaluate relative imports; module match uses exact name or `forbidden.` prefix.
- `--with-llm` is accepted but not implemented.
- `integrations/`, `analyzers/external.rs`, `analyzers/llm.rs`, and `llm/` are stubs only.

[Unreleased]: https://github.com/latentmeta/zerum/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/latentmeta/zerum/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/latentmeta/zerum/releases/tag/v0.1.0
