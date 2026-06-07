# Zerum v0.2.0 — Deterministic Foundations

**Release date:** 2026-05-27  
**Repository:** [github.com/latentmeta/zerum](https://github.com/latentmeta/zerum)  
**crates.io:** [zerum 0.2.0](https://crates.io/crates/zerum/0.2.0)  
**License:** MIT

Zerum v0.2.0 is the **foundations milestone**: a Rust-native, deterministic-first governance tool for Python—*Credo for Python*. It expands the Phase 1 ten-check prototype into a **75-rule catalog** with explainable findings across readability, consistency, design, refactor, warnings, and AI-slop patterns.

This release deliberately does **not** include external checker orchestration, LLM review, plugins, autofix, or SARIF/Markdown reporters. Tune `zerum.toml` before turning on strict CI gates; heuristic rules can be noisy on small modules.

---

## Install

```bash
cargo install zerum --version 0.2.0
```

From source:

```bash
git clone https://github.com/latentmeta/zerum.git
git checkout v0.2.0
cargo install --path .
```

Requires Rust **1.70+** (`rust-version` in `Cargo.toml`).

Verify:

```bash
zerum list-checks | wc -l    # expect 75
zerum explain ZR001
zerum check path/to/python/project
```

---

## What's new

### ~75 native checks (ZR001–ZR510)

Single catalog registry (`build_catalog()`) replaces the hand-wired Phase 1 registry. Every rule supports:

```bash
zerum explain ZR###
```

Output includes rule meaning, false positives, tradeoffs, examples, and remediation.

| Range | Category | Count |
|-------|----------|------:|
| ZR001–015 | Readability | 15 |
| ZR101–110 | Consistency | 10 |
| ZR201–210 | Design | 10 |
| ZR301–315 | Refactor | 15 |
| ZR401–415 | Warning | 15 |
| ZR501–510 | AI (deterministic slop) | 10 |

### AST-precise detectors (subset)

Implemented in `catalog_detectors` and built-in structural detectors (non-exhaustive):

| Area | Examples |
|------|----------|
| Readability | Magic numbers (ZR006), commented-out code (ZR009), TODO without context (ZR010), short names (ZR005) |
| Consistency | Mixed string quotes (ZR105), test/`should_` mix (ZR106) |
| Design | Dataclass without behavior (ZR205), service naming explosion (ZR209) |
| Refactor | Duplicate branch body (ZR301), collapsible if (ZR302), else after return (ZR303), redundant bool compare (ZR304), identity map/passthrough (ZR310–311), manual join in loop (ZR314), sort+reverse (ZR315), … |
| Warning | Empty/bare/broad except patterns (ZR401–403, 408, 411), assert in production (ZR406), eval/exec (ZR407), `len` comparisons (ZR414), query-in-loop (ZR412), global mutable (ZR409), … |
| AI slop | Placeholder comments (ZR501), generated comment patterns (ZR502), generic exception messages (ZR504), dead branches (ZR507), defensive suffixes (ZR508) |

Many **consistency** and **design** rules remain **pattern-based heuristics** by design in v0.2.0; use `explain` and `enabled = false` for noisy ids.

### SourceModel and metadata

- **`SourceModel`** — semantic views over the parser: functions, classes, imports, comments.
- **`CheckMetadata`** — id, name, category, severity, `safe_fixable`, examples on every catalog entry.
- **Per-check config** — `enabled`, thresholds (`max_lines`, `max_arguments`, `max_depth`, `max_methods`), and **`severity` override**.

### CLI (scope for v0.2.0)

| Command | Purpose |
|---------|---------|
| `zerum check <path>` | Run enabled checks; exit **1** when issues exist |
| `zerum explain <id>` | Full guidance for a rule (e.g. `ZR401`) |
| `zerum list-checks` | Print all catalog rules |
| `zerum init` | Write starter `zerum.toml` from `zerum.toml.example` |

**Exit codes (`check`):** `0` clean · `1` issues found · `2` operational error (no files, parse failures, CLI error).

### Reporters

- `human` (default) — issues with explanation and remediation
- `json` — machine-readable issue list for CI

**Not in v0.2.0:** `review`, `list-checkers`, `--with-llm`, `--format markdown|sarif`.

### Configuration example

```toml
[checks.ZR001]
enabled = true
max_lines = 80

[checks.ZR401]
severity = "high"

[checks.ZR207]
enabled = true

[[checks.ZR207.rules]]
from = "app.domain"
forbidden = "app.infrastructure"
```

### Tests and quality

- Integration: `rule_precision_tests`, `catalog_fixture_tests`, category fixtures, snapshot tests (`bad_project`), config override tests.
- Registry integrity tests (unique ZR ids, metadata completeness).
- Development coverage **≥70%** line target documented in `docs/coverage.md` (~85% measured at release hardening).

### Documentation

- Tutorial: [04 — Writing checks](tutorial/04-writing-checks.md), [05 — Explain mode and configuration](tutorial/05-explain-mode-and-configuration.md)
- Milestone review: [reviews/milestone-6-v0.2.0.md](../reviews/milestone-6-v0.2.0.md)
- Publishing: [docs/RELEASING.md](RELEASING.md)

---

## Full rule catalog

### Readability (ZR001–ZR015)

| ID | Name |
|----|------|
| ZR001 | long-function |
| ZR002 | too-many-arguments |
| ZR003 | deep-nesting |
| ZR004 | complex-boolean-expression |
| ZR005 | unclear-variable-name |
| ZR006 | magic-number |
| ZR007 | missing-module-docstring |
| ZR008 | missing-function-docstring |
| ZR009 | commented-out-code |
| ZR010 | todo-without-context |
| ZR011 | narrator-docstring |
| ZR012 | boilerplate-docstring |
| ZR013 | step-comment |
| ZR014 | narrator-comment |
| ZR015 | obvious-comment |

### Consistency (ZR101–ZR110)

| ID | Name |
|----|------|
| ZR101 | inconsistent-function-naming |
| ZR102 | inconsistent-class-naming |
| ZR103 | inconsistent-constant-naming |
| ZR104 | inconsistent-import-style |
| ZR105 | mixed-quote-style |
| ZR106 | inconsistent-test-naming |
| ZR107 | inconsistent-private-prefix |
| ZR108 | duplicate-naming-pattern |
| ZR109 | mixed-collection-style |
| ZR110 | mixed-return-style |

### Design (ZR201–ZR210)

| ID | Name |
|----|------|
| ZR201 | god-class |
| ZR202 | too-many-instance-variables |
| ZR203 | too-many-public-methods |
| ZR204 | feature-envy |
| ZR205 | dataclass-without-behavior |
| ZR206 | circular-import |
| ZR207 | forbidden-architecture-import |
| ZR208 | layer-violation |
| ZR209 | service-object-explosion |
| ZR210 | excessive-indirection |

### Refactor (ZR301–ZR315)

| ID | Name |
|----|------|
| ZR301 | duplicate-branch-body |
| ZR302 | collapsible-if |
| ZR303 | unnecessary-else-after-return |
| ZR304 | redundant-boolean-comparison |
| ZR305 | simplifiable-if-expression |
| ZR306 | repeated-literal |
| ZR307 | long-parameter-list |
| ZR308 | extractable-condition |
| ZR309 | repeated-try-except |
| ZR310 | identity-passthrough |
| ZR311 | identity-map |
| ZR312 | reject-none |
| ZR313 | filter-none |
| ZR314 | manual-string-join |
| ZR315 | sort-then-reverse |

### Warning (ZR401–ZR415)

| ID | Name |
|----|------|
| ZR401 | broad-except |
| ZR402 | empty-except |
| ZR403 | bare-except |
| ZR404 | mutable-default-argument |
| ZR405 | print-debugging |
| ZR406 | assert-production |
| ZR407 | dangerous-eval-exec |
| ZR408 | silent-exception-swallowing |
| ZR409 | global-mutable-state |
| ZR410 | ambiguous-none-return |
| ZR411 | blanket-except |
| ZR412 | query-in-loop |
| ZR413 | silent-fallback |
| ZR414 | length-comparison |
| ZR415 | sort-for-top-k |

### AI slop (ZR501–ZR510)

Inspired by deterministic Elixir [ex_slop](https://hex.pm/packages/ex_slop)-style checks; no LLM in the pipeline.

| ID | Name |
|----|------|
| ZR501 | placeholder-generated-code |
| ZR502 | generated-comment-pattern |
| ZR503 | excessive-narration |
| ZR504 | generic-exception-message |
| ZR505 | boilerplate-parameter-docs |
| ZR506 | empty-wrapper-function |
| ZR507 | generated-dead-branch |
| ZR508 | defensive-overengineering |
| ZR509 | excessive-abstraction |
| ZR510 | generic-utility-explosion |

---

## Migration from v0.1.0

| v0.1.0 | v0.2.0 |
|--------|--------|
| 10 checks (ZR001–ZR010) | **75** checks (ZR001–ZR510); ids **remapped** |
| ZR001 `too-many-branches` | ZR001 `long-function` |
| ZR003 `long-function` | ZR003 `deep-nesting` |
| ZR004 `nested-conditionals` | ZR004 `complex-boolean-expression` |
| ZR005 `broad-except` | ZR401 `broad-except` (warning category) |
| ZR006 `print-debugging` | ZR405 `print-debugging` |
| ZR007 `mutable-default-argument` | ZR404 `mutable-default-argument` |
| ZR008 `god-class` | ZR201 `god-class` (design category) |
| ZR009 `ai-generated-placeholder-comment` | ZR501 `placeholder-generated-code` |
| ZR010 `forbidden-architecture-import` | ZR207 `forbidden-architecture-import` |
| `zerum review` (exit 0 on findings) | Use `zerum check` only; gate CI on exit **1** |
| `--format sarif\|markdown` | **`human`** and **`json`** only |
| `list-checkers`, `--with-llm` stubs | **Removed** from CLI |

**Recommended upgrade path**

1. `cargo install zerum --version 0.2.0`
2. `zerum list-checks` and remap any documented ZR ids in CI configs.
3. `zerum init` or merge new keys into existing `zerum.toml`.
4. Run `zerum check .` locally; disable or raise thresholds for noisy rules before enforcing in CI.

---

## Known limitations

- **Heuristic rules** (pattern/comment matching) may false-positive on minimal or generated code.
- **ZR207** requires explicit `[[checks.ZR207.rules]]` and path-segment layer matching; not a full import-graph boundary system.
- **No autofix** — findings are advisory; use your formatter/linter for fixes.
- **No external tools** — Ruff, Bandit, Mypy, etc. are not orchestrated in this release (planned post–v0.2.0).
- **Not all rules** have dedicated positive/negative tests in v0.2.0; catalog and fixture coverage grow in later patches.
- Zerum is **not** a replacement for Ruff, Black, or security scanners; it complements them with governance-style rules.

---

## Out of scope (planned later)

- `ExternalChecker` trait and Ruff/Bandit adapters
- Default/strict profiles and noisy-rule defaults
- SARIF reporter and `list-checkers`
- Opt-in LLM layer (`--with-llm`)
- Plugins, WASM, dynamic rule loading, autofix

See [CHANGELOG.md](../CHANGELOG.md) and [tutorial/12-roadmap.md](tutorial/12-roadmap.md).

---

## Contributors

Thanks to everyone who landed the v0.2.0 foundations milestone. See git history for `v0.2.0` and [CHANGELOG.md](../CHANGELOG.md).

**Full diff from v0.1.0:** [compare v0.1.0...v0.2.0](https://github.com/latentmeta/zerum/compare/v0.1.0...v0.2.0)
