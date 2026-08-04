# Zerum

[![crates.io](https://img.shields.io/crates/v/zerum.svg)](https://crates.io/crates/zerum)
[![PyPI](https://img.shields.io/pypi/v/zerum.svg)](https://pypi.org/project/zerum/)
[![GitHub release](https://img.shields.io/github/v/release/latentmeta/zerum.svg)](https://github.com/latentmeta/zerum/releases)

**Zerum** is deterministic code governance for Python — *Credo for Python*.

**v0.4.1** ships ~**75** native checks (ZR001–ZR510), explainable findings, quiet **default** / full **strict** profiles, optional **Ruff** orchestration, and `human` / `json` output. Install via **PyPI**, **Homebrew**, **crates.io**, or GitHub Releases. No LLM in the core path.

Zerum is **not** a Ruff replacement. It focuses on maintainability, consistency, architecture boundaries, and deterministic AI-slop patterns.

---

## Install

Pick one. After install you get a `zerum` command on your `PATH`.

### Standalone binary

Download a release archive for your OS/arch from
[GitHub Releases](https://github.com/latentmeta/zerum/releases), extract, and put `zerum` on your `PATH`.

**Homebrew** (tap formula; after the formula is published):

```bash
brew install latentmeta/tap/zerum
```

**crates.io** (requires Rust **1.70+**):

```bash
cargo install zerum --locked
```

### Python (recommended for most Python projects)

No Rust toolchain required — prebuilt wheels:

```bash
pip install zerum
```

Isolated tool installs:

```bash
pipx install zerum
# or
uv tool install zerum
```

Verify:

```bash
zerum --version
zerum --help
```

---

## Quick start

```bash
cd your-python-project

# Optional: write a starter zerum.toml (default profile)
zerum init

# Run checks on the project
zerum check .

# Browse the catalog and learn a rule
zerum list-checks
zerum explain ZR001
```

Exit codes for `zerum check`:

| Code | Meaning |
|------|---------|
| 0 | No issues |
| 1 | Issues found |
| 2 | Operational error (bad path, CLI error, etc.) |

---

## Using Zerum

### Check a project

```bash
zerum check .
zerum check path/to/package
zerum check src/
```

Human-readable output is the default. Each finding includes rule id, location, explanation, and remediation.

### Profiles: default vs strict

With **no** `zerum.toml` (or `[profile] name = "default"`), Zerum uses the built-in **default** profile: noisy pattern heuristics are off so greenfield modules stay quieter.

Enable the full catalog:

```bash
zerum check . --profile strict
```

Or persist it:

```bash
zerum init --strict
# writes a strict starter config
```

```toml
[profile]
name = "strict"
```

Compare:

```bash
zerum check .                    # default — quieter
zerum check . --profile strict   # all ~75 rules
```

### Configuration (`zerum.toml`)

```bash
zerum init              # default template
zerum init --strict     # strict template
```

Common knobs:

```toml
[profile]
name = "default"

[checks.ZR001]
enabled = true
max_lines = 50

[checks.ZR401]
severity = "high"

# Architecture layers (ZR207)
[[checks.ZR207.rules]]
from = "app.domain"
forbidden = "app.infrastructure"

# Always run Ruff when you check (requires ruff on PATH)
# external_checkers = ["ruff"]
```

Custom profiles can inherit:

```toml
[profiles.team]
extends = "default"

[profiles.team.checks.ZR001]
max_lines = 60
```

```bash
zerum check . --profile team
```

Starter files in the repo: `zerum.toml.example`, `zerum.toml.strict.example`.

### Explain a rule

```bash
zerum explain ZR001
zerum explain ZR401
zerum explain ZR501
```

Shows category, severity, rationale, false positives, tradeoffs, examples, and remediation.

### List checks and external checkers

```bash
zerum list-checks
zerum list-checkers
```

`list-checks` prints the full ZR catalog. `list-checkers` shows external adapters (e.g. Ruff) and whether they are available on `PATH`.

### Optional Ruff orchestration

Zerum can run **Ruff** alongside native checks and merge findings:

```bash
# one-off
zerum check . --with-external ruff

# or persist in zerum.toml
# external_checkers = ["ruff"]
zerum check .
```

Requires `ruff` on `PATH`. External findings use ids like `EXT-RUFF`.

### Rule categories

| Range | Category |
|-------|----------|
| ZR001–015 | Readability |
| ZR101–110 | Consistency |
| ZR201–210 | Design |
| ZR301–315 | Refactor |
| ZR401–415 | Warning |
| ZR501–510 | AI (deterministic) |

---

## Output formats

```bash
zerum check . --format human    # default
zerum check . --format json
zerum check . --profile strict
zerum check . --with-external ruff
zerum list-checkers
```

| Format | When to use |
|--------|-------------|
| `human` | Terminal review — rule id, location, explanation, remediation |
| `json` | CI artifacts, scripts, and custom dashboards |

Optional external checkers (Ruff) are available from **v0.4.0**. Use the **default** profile for low noise on greenfield code; use `--profile strict` for full catalog coverage.

---

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

---

## Feature demos

Assume a project directory with some Python sources.

### 1. First pass (quiet default)

```bash
zerum check .
# exit 0 → clean under default profile
# exit 1 → findings printed to stdout
```

### 2. Full catalog

```bash
zerum check . --profile strict
```

Expect more findings on small modules (docstring / comment / heuristic rules).

### 3. Machine-readable report

```bash
zerum check . --format json > zerum-report.json
```

### 4. Learn why a finding fired

```bash
zerum check . --format human
# note a rule id, e.g. ZR003
zerum explain ZR003
```

### 5. Team config + architecture boundary

```bash
zerum init
# edit zerum.toml — set ZR207 rules for your layers
zerum check .
```

### 6. Zerum + Ruff in one command

```bash
zerum list-checkers
zerum check . --with-external ruff --format json
```

### 7. Upgrade later

```bash
pip install --upgrade zerum
# or: pipx upgrade zerum
# or: uv tool upgrade zerum
# or: brew upgrade zerum
zerum --version
```

---

## Add Zerum to CI/CD (Python project)

Fail the job when Zerum finds issues (`exit 1`). Use JSON if you want artifacts.

### GitHub Actions (pip)

```yaml
name: Zerum

on:
  pull_request:
  push:
    branches: [main]

jobs:
  zerum:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"

      - name: Install Zerum
        run: pip install zerum

      - name: Run Zerum
        run: zerum check . --format human

      # Optional: stricter gate + JSON artifact
      # - run: zerum check . --profile strict --format json > zerum.json
      # - uses: actions/upload-artifact@v4
      #   if: always()
      #   with:
      #     name: zerum-report
      #     path: zerum.json
```

### GitHub Actions (uv)

```yaml
- uses: astral-sh/setup-uv@v4
- run: uv tool install zerum
- run: zerum check .
```

### GitHub Actions (pipx)

```yaml
- uses: actions/setup-python@v5
  with:
    python-version: "3.12"
- run: pipx install zerum
- run: zerum check .
```

### Pre-commit

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: zerum
        name: zerum
        entry: zerum check .
        language: system
        pass_filenames: false
        types: [python]
```

Install Zerum on the machine (or in CI) before running pre-commit, e.g. `pipx install zerum`.

### GitLab CI

```yaml
zerum:
  image: python:3.12-slim
  script:
    - pip install zerum
    - zerum check .
```

### Tips for CI

- Start with **default** profile; move to `--profile strict` once the baseline is clean.
- Pin the version in CI: `pip install "zerum==0.4.1"`.
- Combine with Ruff only if `ruff` is installed in the job:  
  `zerum check . --with-external ruff`.
- Treat exit code `2` as infra failure; `1` as “findings to fix.”

---

## Changelog

See [CHANGELOG.md](CHANGELOG.md). Release notes: [v0.4.1](docs/RELEASE_v0.4.1.md).

---

## Building from source (contributors)

For hacking on Zerum itself — not required for normal use.

```bash
git clone https://github.com/latentmeta/zerum.git
cd zerum
cargo build --release
./target/release/zerum check path/to/python/project

# or run without installing
cargo run -- check path/to/python/project
cargo run -- explain ZR001
cargo run -- list-checks
cargo run -- init

# editable Python-env install via maturin
pip install "maturin>=1.7,<2.0"
maturin develop
```

Tests and lint:

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Packaging notes: [`packaging/`](packaging/) (PyPI, Homebrew). Config for multi-channel scaffolding: [`Sastri.toml`](Sastri.toml).

---

## License

MIT — see [LICENSE](LICENSE).
