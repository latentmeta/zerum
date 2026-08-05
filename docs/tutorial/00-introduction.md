# Zerum Tutorial — Introduction

Zerum is a **deterministic-first** code governance tool for Python — *Credo for Python*.

## What current Zerum delivers (v0.5.0)

| Layer | Status |
|-------|--------|
| Deterministic checks | ~75 rules (ZR001–ZR510) via catalog |
| Explain mode | False positives, tradeoffs, remediation per rule (curated text for tier-1 rules; AST-precise fallback for others) |
| Reporters | `human`, `json`, and optional `sarif` (default feature) |
| Remediation prompts | `zerum check --remediation-prompt` (deterministic; no model call) |
| External orchestration | Optional Ruff via `--with-external ruff` |
| LLM review | Out of scope until Phase 4 |

## Architecture

```text
CLI → Config + Discovery → DeterministicAnalyzer
                              ↓
                    PythonParser → SourceModel (cached views)
                              ↓
                    CheckRegistry (build_catalog)
                              ↓
                         Vec<Issue> → Reporter (human | json | sarif)
                                      + optional remediation prompt file
```

## Repository map

- `src/core/` — `Issue`, `Check`, `Severity`, `Category`, `CheckRegistry`, AST helpers
- `src/parser/` — `PythonParser` trait + `RustPythonParser` + `SourceModel`
- `src/checks/catalog.rs` — full rule catalog
- `src/checks/catalog_detectors.rs` — AST-precise detectors
- `src/analyzers/deterministic.rs` — runs enabled checks per file
- `src/reporters/` — human, JSON, SARIF, remediation prompt
- `tests/fixtures/` — `simple_project`, `bad_project`, `arch_violation`, category fixtures

## Running Zerum

Prefer the installed binary (PyPI / Homebrew / crates.io / Releases):

```bash
zerum list-checks
zerum check path/to/project
zerum explain ZR001
zerum check . --remediation-prompt
```

From a source checkout:

```bash
cargo build
cargo run -- list-checks
cargo run -- check tests/fixtures/bad_project   # exits 1 when issues exist
cargo run -- explain ZR001
cargo run -- init
```

## Reviewer checklist

- [ ] `cargo test` passes
- [ ] `zerum check` on `bad_project` exits 1
- [ ] Each check id appears in `list-checks`
- [ ] Tutorial chapters match the current CLI (`check`, `explain`, `init`, `list-checks`, `list-checkers`)

## Next phases

- **Phase 4** — opt-in LLM review, audit trail, consensus

See [12 — Roadmap](12-roadmap.md) for the full sequence.
