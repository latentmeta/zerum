# Zerum

<p align="center">
  <img src="https://raw.githubusercontent.com/latentmeta/zerum/main/docs/zerum-icon-02.jpg" alt="Zerum — Rust crab administering analysis extract to a Python snake" width="640" />
</p>

<p align="center">
  <strong>Zerum</strong> — deterministic code governance for Python · <em>Credo for Python</em>
</p>

<p align="center">
  <a href="https://crates.io/crates/zerum"><img src="https://img.shields.io/crates/v/zerum.svg" alt="crates.io" /></a>
  <a href="https://pypi.org/project/zerum/"><img src="https://img.shields.io/pypi/v/zerum.svg" alt="PyPI" /></a>
  <a href="https://github.com/latentmeta/zerum/releases"><img src="https://img.shields.io/github/v/release/latentmeta/zerum.svg" alt="GitHub release" /></a>
  <a href="https://github.com/latentmeta/zerum/actions/workflows/ci.yml"><img src="https://github.com/latentmeta/zerum/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="https://docs.rs/zerum"><img src="https://docs.rs/zerum/badge.svg" alt="docs.rs" /></a>
  <a href="https://github.com/latentmeta/zerum/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT license" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.70%2B-orange.svg" alt="Rust 1.70+" /></a>
  <a href="https://www.python.org/"><img src="https://img.shields.io/badge/python-3.9%2B%20(wheels)-blue.svg" alt="Python 3.9+ wheels" /></a>
</p>

**v0.5.0** ships ~**75** native checks (ZR001–ZR510), explainable findings, quiet **default** / full **strict** profiles, optional **Ruff** orchestration, `human` / `json` output, and **`--remediation-prompt`** — a deterministic markdown brief for LLM/editor agents (no model call in Zerum). Install via **PyPI**, **Homebrew**, **crates.io**, or GitHub Releases.

Zerum is **not** a Ruff replacement. It focuses on maintainability, consistency, architecture boundaries, and deterministic AI-slop patterns.

Full-resolution artwork: [`docs/zerum-icon-01.png`](https://github.com/latentmeta/zerum/blob/main/docs/zerum-icon-01.png).

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

# Write a remediation prompt for an LLM / editor agent
zerum check . --remediation-prompt

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

### Remediation prompt (v0.5.0)

Zerum can save a **deterministic** markdown prompt from its findings — grouped by check type, ordered by severity (critical → info), with shared remediation text and source snippets. Zerum does **not** call an LLM; you paste the file into Cursor, ChatGPT, or another agent.

```bash
zerum check . --remediation-prompt
# → zerum-remediation-prompt.md

zerum check . --remediation-prompt fixes.md
zerum check . --profile strict --remediation-prompt fixes.md
```

Typical workflow:

1. `zerum check . --remediation-prompt fixes.md`
2. Open `fixes.md` in your editor / agent and ask it to apply the remediations
3. Re-run `zerum check .` until clean

The prompt includes:

- Goal and constraints (minimal edits, preserve APIs)
- Summary counts and a type index ordered by severity
- Findings **grouped by check id**, shared explanation/remediation once per type
- Per-occurrence location, message, and source context
- Instructions to re-run Zerum after edits

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
zerum check . --remediation-prompt fixes.md
zerum list-checkers
```

| Output | When to use |
|--------|-------------|
| `human` | Terminal review — rule id, location, explanation, remediation |
| `json` | CI artifacts, scripts, and custom dashboards |
| `--remediation-prompt` | Markdown brief for an LLM/editor agent (file on disk; still prints `human`/`json` to stdout) |

Optional external checkers (Ruff) are available from **v0.4.0**. Use the **default** profile for low noise on greenfield code; use `--profile strict` for full catalog coverage.

---

## Tutorial

Educational material lives under [`docs/tutorial/`](https://github.com/latentmeta/zerum/tree/main/docs/tutorial/):

- [00 — Introduction](https://github.com/latentmeta/zerum/blob/main/docs/tutorial/00-introduction.md)
- [01 — Static analysis basics](https://github.com/latentmeta/zerum/blob/main/docs/tutorial/01-static-analysis-basics.md)
- [02 — Parsing Python in Rust](https://github.com/latentmeta/zerum/blob/main/docs/tutorial/02-parsing-python-in-rust.md)
- [03 — Building a rule engine](https://github.com/latentmeta/zerum/blob/main/docs/tutorial/03-building-a-rule-engine.md)
- [04 — Writing checks](https://github.com/latentmeta/zerum/blob/main/docs/tutorial/04-writing-checks.md)
- [05 — Explain mode and configuration](https://github.com/latentmeta/zerum/blob/main/docs/tutorial/05-explain-mode-and-configuration.md)
- [06 — Config and profiles](https://github.com/latentmeta/zerum/blob/main/docs/tutorial/06-config-profiles.md)
- [12 — Roadmap](https://github.com/latentmeta/zerum/blob/main/docs/tutorial/12-roadmap.md)

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

### 7. Save a remediation prompt for an LLM / editor agent

```bash
zerum check . --remediation-prompt
# → zerum-remediation-prompt.md

zerum check . --remediation-prompt fixes.md
```

Findings are **grouped by type** and **sorted by severity** (critical first).

### 8. Upgrade later

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
        run: pip install "zerum==0.5.0"

      - name: Run Zerum
        run: zerum check . --format human

      # Optional: remediation prompt artifact for reviewers / agents
      # - run: zerum check . --remediation-prompt zerum-remediation-prompt.md || true
      # - uses: actions/upload-artifact@v4
      #   if: failure()
      #   with:
      #     name: zerum-remediation-prompt
      #     path: zerum-remediation-prompt.md
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
    - pip install "zerum==0.5.0"
    - zerum check .
```

### Tips for CI

- Start with **default** profile; move to `--profile strict` once the baseline is clean.
- Pin the version in CI: `pip install "zerum==0.5.0"`.
- Combine with Ruff only if `ruff` is installed in the job:  
  `zerum check . --with-external ruff`.
- Treat exit code `2` as infra failure; `1` as “findings to fix.”
- Optionally upload `--remediation-prompt` output as a CI artifact when the check fails.

---

## Changelog

See [CHANGELOG.md](https://github.com/latentmeta/zerum/blob/main/CHANGELOG.md). Release notes: [v0.5.0](https://github.com/latentmeta/zerum/blob/main/docs/RELEASE_v0.5.0.md).

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
cargo run -- check path/to/project --remediation-prompt /tmp/fixes.md

# editable Python-env install via maturin
pip install "maturin>=1.7,<2.0"
maturin develop
```

Tests and lint:

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Coverage (rustup cargo; asdf shims break `cargo +toolchain`):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo +1.97.1 tarpaulin \
  --engine llvm --all-targets --all-features --follow-exec \
  --out Stdout --fail-under 70 -- --test-threads=1
```

Packaging notes: [`packaging/`](https://github.com/latentmeta/zerum/tree/main/packaging) (PyPI, Homebrew). Config for multi-channel scaffolding: [`Sastri.toml`](https://github.com/latentmeta/zerum/blob/main/Sastri.toml).

---

## License

MIT — see [LICENSE](https://github.com/latentmeta/zerum/blob/main/LICENSE).
