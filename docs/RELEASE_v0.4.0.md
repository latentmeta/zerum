# Zerum v0.4.0 — Profiles, orchestration, and trust defaults

**Release date:** 2026-06-07  
**Repository:** [github.com/latentmeta/zerum](https://github.com/latentmeta/zerum)  
**License:** MIT

v0.4.0 hardens the v0.2.0 foundations release: **quieter default profile**, **config profiles with inheritance**, **optional Ruff orchestration**, more **AST-precise** rules, and **curated explain** text for high-traffic checks. Still deterministic-first—no LLM in the core path.

---

## Install

```bash
cargo install zerum --version 0.4.0
```

From source:

```bash
git clone https://github.com/latentmeta/zerum.git
git checkout v0.4.0
cargo install --path .
# Optional SARIF reporter:
cargo install --path . --features sarif
```

Requires Rust **1.70+**.

Verify:

```bash
zerum list-checks | wc -l          # 75
zerum check tests/fixtures/clean_project    # exit 0 (default profile)
zerum check . --profile strict
zerum list-checkers
zerum explain ZR401
```

---

## Highlights

### Default and strict profiles

Without a `zerum.toml`, Zerum now uses the built-in **`default`** profile: noisy pattern heuristics are **off** so greenfield modules stay quiet. Enable everything with:

```bash
zerum check . --profile strict
```

Or persist in config:

```toml
[profile]
name = "strict"
```

Starter files: `zerum.toml.example`, `zerum.toml.strict.example`.  
`zerum init --strict` writes the strict template.

Disabled-by-default ids are listed in `src/config/defaults.rs` (e.g. ZR007/008 docstrings, ZR102–104 consistency heuristics, ZR503/505 AI patterns).

### Configuration and profiles

- **`[profiles.*]` inheritance** — `extends = "base"` with merged per-check overrides.
- **Validation at load** — invalid check keys (`ZR###` only), severity enum, profile cycle detection.
- **`external_checkers = ["ruff"]`** in `zerum.toml` for persistent external tooling.

See [tutorial 06 — Config and profiles](tutorial/06-config-profiles.md).

### External checkers (Ruff)

```bash
zerum check . --with-external ruff
zerum list-checkers
```

- `ExternalChecker` trait + Ruff JSON adapter (`integrations/ruff.rs`).
- External findings use ids like `EXT-RUFF` with `confidence: tool_reported`.
- Requires `ruff` on `PATH` for the adapter to run.

### Rule quality

| Change | Rules |
|--------|--------|
| AST-precise (new or improved) | ZR101 inconsistent function naming, ZR306 repeated literal, ZR506 empty wrapper |
| Curated explain text | ZR001, ZR003, ZR005, ZR010, ZR207, ZR401–404, ZR406–407, ZR414, ZR501, ZR504 |

### CLI additions

| Flag / command | Purpose |
|----------------|---------|
| `check --profile <name>` | Override `[profile].name` |
| `check --with-external ruff` | Run Ruff alongside native catalog |
| `init --strict` | Write strict `zerum.toml` template |
| `list-checkers` | Show external adapters and availability |

### Optional SARIF

```bash
cargo build --release --features sarif
zerum check . --format sarif
```

SARIF is behind a **feature flag** to keep default builds lean.

### Tests and CI

- `catalog_rule_matrix` — 75 rules, explain smoke, strict-profile triggers.
- `clean_project` fixture — passes under default profile.
- CI coverage job: **fail under 70%** line coverage (tarpaulin).

### Cleanup

- Removed unwired legacy modules under `src/checks/{readability,design,warning,architecture,ai}/`; catalog is the single source of truth.

---

## Migration from v0.2.0

| v0.2.0 | v0.4.0 |
|--------|--------|
| All checks enabled when no `zerum.toml` | **Default profile** disables ~25 noisy heuristics |
| No `--profile` | `--profile strict` restores full catalog |
| No external tools | `--with-external ruff`, `external_checkers` config |
| No `list-checkers` | `list-checkers` restored |
| `src/config.rs` | `src/config/mod.rs` + `defaults.rs` |
| Generic explain on many rules | Tier-1 rules have curated explain copy |

**If CI suddenly passes with fewer findings:** you are likely on the default profile. Compare:

```bash
zerum check .                    # default
zerum check . --profile strict   # all 75 rules
```

Explicitly enable rules in `zerum.toml` if you relied on heuristic ids that are now default-off.

**If you need v0.2.0 behavior without config:** use `--profile strict` or set `[profile] name = "strict"`.

---

## What's unchanged

- **75 rules** (ZR001–ZR510), same id semantics as v0.2.0.
- Core CLI: `check`, `explain`, `list-checks`, `init`.
- Reporters: `human`, `json` (SARIF optional via feature).
- No LLM, autofix, or plugins.

---

## GitHub release and crates.io

1. Ensure tag points at a commit with `version = "0.4.0"` in `Cargo.toml` (see [RELEASING.md](RELEASING.md)).
2. `git tag -a v0.4.0 -m "Zerum v0.4.0" upstream/main && git push upstream v0.4.0`
3. Confirm **Release** workflow on Actions; attach notes from this file.
4. `cargo publish` from clean tree.

---

## Known limitations

- Ruff adapter pins to JSON output format; version drift may require doc updates.
- Many rules still use pattern heuristics under `--profile strict` only.
- Bandit/Semgrep/import-linter not yet wired (planned v0.6.0).
- Per-rule explain text incomplete outside tier-1 set.

---

## Full changelog

See [CHANGELOG.md](../CHANGELOG.md#040---2026-06-07).

**Compare:** [v0.2.0...v0.4.0](https://github.com/latentmeta/zerum/compare/v0.2.0...v0.4.0)
