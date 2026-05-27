# Zerum Tutorial — Introduction

Zerum is a **deterministic-first** code governance tool for Python — *Credo for Python*.

## What v0.2.0 delivers

| Layer | Status in v0.2.0 |
|-------|------------------|
| Deterministic checks | ~75 rules (ZR001–ZR510) via catalog |
| Explain mode | False positives, tradeoffs, remediation per rule |
| Reporters | `human` and `json` only |
| External orchestration | Out of scope |
| LLM review | Out of scope |

## Architecture (v0.2.0)

```text
CLI → Config + Discovery → DeterministicAnalyzer
                              ↓
                    PythonParser → SourceModel
                              ↓
                    CheckRegistry (build_catalog)
                              ↓
                         Vec<Issue> → Reporter (human | json)
```

## Repository map (created in Phase 1)

- `src/core/` — `Issue`, `Check`, `Severity`, `Category`, `CheckRegistry`, AST helpers
- `src/parser/` — `PythonParser` trait + `RustPythonParser`
- `src/checks/catalog.rs` — full rule catalog
- `src/checks/catalog_detectors.rs` — AST-precise detectors
- `src/analyzers/deterministic.rs` — runs enabled checks per file
- `src/reporters/` — human, JSON
- `tests/fixtures/` — `simple_project`, `bad_project`, `arch_violation` (ZR010)

## Running Zerum

```bash
cargo build
cargo run -- list-checks
cargo run -- check tests/fixtures/bad_project   # exits 1 when issues exist
cargo run -- explain ZR001
cargo run -- init
```

## Reviewer checklist

- [ ] `cargo test` passes
- [ ] `zerum check` on `bad_project` exits 1; `zerum review` exits 0 with the same output
- [ ] Each check id appears in `list-checks`
- [ ] Tutorial chapters 01–03 match the code layout

## Next phases

- **Phase 2** — external checker trait + Ruff adapter
- **Phase 3** — stronger architecture rules, SARIF validation
- **Phase 4** — opt-in LLM review, audit trail, consensus

See [12 — Roadmap](12-roadmap.md) for the full sequence.
