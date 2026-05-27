# Zerum v0.2.0 — Deterministic Foundations

**Target release:** v0.2.0 (foundations milestone)  
**Repository:** [github.com/latentmeta/zerum](https://github.com/latentmeta/zerum)  
**License:** MIT

v0.2.0 expands Zerum from ten Phase 1 checks to a **~75-rule deterministic catalog** aimed at “Credo for Python”: explainable readability, consistency, design, refactor, warning, and AI-slop rules—without external linters, LLMs, plugins, or SARIF.

---

## Install

```bash
cargo install zerum --version 0.2.0
```

Or from source:

```bash
git clone https://github.com/latentmeta/zerum.git
cd zerum
cargo install --path .
```

Requires Rust **1.70+**.

---

## Highlights

### CLI (in scope)

| Command | Purpose |
|---------|---------|
| `zerum check <path>` | Run all enabled checks; exit **1** when issues exist |
| `zerum explain <id>` | Rule meaning, false positives, tradeoffs, remediation |
| `zerum list-checks` | List catalog rules ZR001–ZR510 |
| `zerum init` | Write starter `zerum.toml` |

**Removed / not in v0.2.0:** `review`, `list-checkers`, `--with-llm`, SARIF/Markdown reporters.

### Output formats

- `human` (default)
- `json`

### Rule catalog (~75 rules)

| Range | Category |
|-------|----------|
| ZR001–015 | Readability |
| ZR101–110 | Consistency |
| ZR201–210 | Design |
| ZR301–315 | Refactor |
| ZR401–415 | Warning |
| ZR501–510 | AI (deterministic slop patterns) |

AST-precise detectors (non-exhaustive): magic numbers, TODO context, `assert`/`eval`/`exec`, `len` comparisons, mutable defaults, broad/bare except, print debugging, commented-out code, identity `map`, else-after-return, service-class explosion, AI placeholder comments, and more. Many consistency rules remain intentional heuristics.

### Architecture

- **`SourceModel`** — semantic views over `ParsedFile` (functions, classes, imports, comments)
- **`CheckMetadata`** — ids, categories, severity, examples
- **Per-check config** — `enabled`, thresholds, `severity` override
- **Catalog registry** — `build_catalog()` replaces hand-wired Phase 1 registry

---

## Migration from v0.1.0

| v0.1.0 | v0.2.0 |
|--------|--------|
| ZR001 too-many-branches | ZR001 long-function (id remapped; review `zerum list-checks`) |
| `zerum review` | Use `zerum check`; gate in CI with exit code |
| `--format sarif` | Use `json` and convert externally if needed |
| 10 checks | ~75 checks; broader findings on small fixtures |

Re-run `zerum check` on your codebase and tune `zerum.toml` thresholds before enforcing in CI.

---

## Documentation

- Tutorial: chapters [04](tutorial/04-writing-checks.md) and [05](tutorial/05-explain-mode-and-configuration.md)
- Milestone review: [reviews/milestone-6-v0.2.0.md](../reviews/milestone-6-v0.2.0.md)

---

## Known limitations

- Heuristic rules (pattern-based) can false-positive; use `explain` and disable noisy ids.
- ZR207 forbidden imports require config rules and path segment matching.
- No autofix, plugins, or external checker orchestration in this release.
- Coverage toward the ≥70% goal is measured in development; not all rules have dedicated positive/negative tests yet.

---

## Contributors

See git history and [CHANGELOG.md](../CHANGELOG.md).
