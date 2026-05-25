# Zerum v0.1.0 — Phase 1 Foundation

**Release date:** 2026-05-25  
**Repository:** [github.com/latentmeta/zerum](https://github.com/latentmeta/zerum)  
**License:** MIT

First public release of **Zerum**: a Rust-native, deterministic-first code governance tool for Python. This version delivers the Phase 1 foundation—native checks, CLI, configuration, and reporters—without external checker orchestration or LLM review (planned for later phases).

Zerum is **not** a Ruff clone. It focuses on explainable findings (rationale + remediation), architectural boundaries, and CI-friendly exit semantics.

---

## Install

```bash
cargo install zerum
```

Or build from source:

```bash
git clone https://github.com/latentmeta/zerum.git
cd zerum
cargo install --path .
```

Requires Rust **1.70+** (see `rust-version` in `Cargo.toml` when publishing).

---

## Highlights

### Ten deterministic checks (ZR001–ZR010)

| ID | Check | Default severity |
|----|--------|------------------|
| ZR001 | too-many-branches | Medium |
| ZR002 | too-many-arguments | Low |
| ZR003 | long-function | Low |
| ZR004 | nested-conditionals | Medium |
| ZR005 | broad-except | High |
| ZR006 | print-debugging | Low |
| ZR007 | mutable-default-argument | High |
| ZR008 | god-class | Medium |
| ZR009 | ai-generated-placeholder-comment | Medium |
| ZR010 | forbidden-architecture-import | High |

Each check ships static **explanation** and **remediation** text. Thresholds are configurable per check in `zerum.toml` (see `zerum.toml.example`).

### CLI

| Command | Purpose |
|---------|---------|
| `zerum check <path>` | Run checks; exit **1** when issues are found |
| `zerum review <path>` | Same analysis; exit **0** on findings (for report-only CI steps) |
| `zerum explain <id>` | Print metadata and guidance for a check (e.g. `ZR001`) |
| `zerum list-checks` | List all built-in checks |
| `zerum list-checkers` | Placeholder list for Phase 2 external tools |
| `zerum init` | Write a starter `zerum.toml` |

Flags: `--format human|json|markdown|sarif` (default: human). `--with-llm` is accepted but not implemented in v0.1.0.

### Exit codes (`check`)

| Code | Meaning |
|------|---------|
| 0 | No issues |
| 1 | One or more issues found |
| 2 | Operational failure (no Python files, all files failed to parse/read, CLI error) |

### Reporters

- **Human** — line/column issues with explanation and remediation
- **JSON** — machine-readable issue list
- **Markdown** — summary suitable for logs or PR comments
- **SARIF 2.1.0** — CI integration (`ruleId` aligned with check ids)

### Configuration

- `zerum.toml` with per-check `enabled` flags and thresholds (`max_branches`, `max_arguments`, `max_lines`, `max_depth`, `max_methods`)
- **ZR010** architecture rules: `[[checks.ZR010.rules]]` with `from` (path layer) and `forbidden` (module prefix)
- `Config::discover` walks upward from the target path and stops at project roots (`.git`, `pyproject.toml`, or `zerum.toml`)

### Parser and analysis

- **rustpython-parser** 0.4 behind a `PythonParser` trait
- Shared AST walking: statement descent for metrics and imports; expression visits reuse the same tree (no duplicate control-flow walkers for `try` / `match` / loops)
- Parse failures are reported per file; if every file fails, the CLI exits **2**

---

## Notable behavior and scope (v0.1.0)

- **ZR003** counts function **body** lines only (not the `def` header).
- **ZR006** flags bare `print()` calls, not `builtins.print` or import aliases.
- **ZR009** scans **comment** text (`# …`) with word boundaries for tokens like `CHATGPT` / `COPILOT`; string literals are ignored.
- **ZR010** matches forbidden modules by exact name or `forbidden.` prefix; path layers use path **components**; relative imports (`from .x`) are not evaluated yet.
- Discovery skips `.git`, `__pycache__`, `venv`, `.venv`, and common cache dirs; walk errors propagate instead of being silently dropped.
- Columns in output are **byte offsets** within the line (documented in the tutorial).

---

## Documentation

Tutorial chapters under `docs/tutorial/`:

- [00 — Introduction](tutorial/00-introduction.md)
- [01 — Static analysis basics](tutorial/01-static-analysis-basics.md)
- [02 — Parsing Python in Rust](tutorial/02-parsing-python-in-rust.md)
- [03 — Building a rule engine](tutorial/03-building-a-rule-engine.md)
- [12 — Roadmap](tutorial/12-roadmap.md)

Publishing notes: [RELEASING.md](RELEASING.md)

---

## Testing

- **45** automated tests: per-check unit tests, CLI integration, exit-code behavior, discovery/config regressions, and insta snapshots (human / JSON / SARIF) on `tests/fixtures/bad_project`
- Fixtures: `simple_project`, `bad_project`, `arch_violation` (ZR010)

```bash
cargo test
```

---

## Stubs (Phase 2+)

The following are present as scaffolding only and are **not** wired to the CLI in v0.1.0:

- `integrations/` — external checker adapters (e.g. Ruff)
- `analyzers/external.rs`, `analyzers/llm.rs`, `llm/` — orchestration and LLM review

---

## What’s next (Phase 2)

- `ExternalChecker` trait and Ruff JSON adapter
- `zerum.toml` schema validation and richer profiles
- `list-checkers` probing installed tools
- Tutorial chapters 04–07

See the [roadmap](tutorial/12-roadmap.md) for Phases 3–4 (relative imports for ZR010, SARIF validation, opt-in LLM review).

---

## Contributors

Initial implementation and Phase 1 hardening by **thanos vassilakis**, maintained under [Latent Meta](https://github.com/latentmeta).

**Full changelog:** first release; no prior versions.
