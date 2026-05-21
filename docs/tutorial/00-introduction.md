# Zerum Tutorial — Introduction

Zerum is a **deterministic-first** code governance tool for Python. Phase 1 establishes the Rust project skeleton, core issue model, parser abstraction, rule engine, ten native checks (ZR001–ZR010), CLI, and reporters.

## What Phase 1 delivers

| Layer | Status in v0.1.0 |
|-------|------------------|
| Deterministic checks | All ZR001–ZR010 implemented |
| External orchestration | Stubs only (`integrations/`) |
| LLM review | Disabled stubs (`llm/`) |

## Architecture (Phase 1)

```text
CLI → Config + Discovery → DeterministicAnalyzer
                              ↓
                    PythonParser (rustpython-parser)
                              ↓
                    CheckRegistry → Check trait
                              ↓
                         Vec<Issue> → Reporter
```

## Repository map (created in Phase 1)

- `src/core/` — `Issue`, `Check`, `Severity`, `Category`, `CheckRegistry`, AST helpers
- `src/parser/` — `PythonParser` trait + `RustPythonParser`
- `src/checks/` — ZR001–ZR010 by category
- `src/analyzers/deterministic.rs` — runs enabled checks per file
- `src/reporters/` — human, JSON, Markdown, SARIF
- `tests/fixtures/` — `simple_project`, `bad_project`

## Running Zerum

```bash
cargo build
cargo run -- list-checks
cargo run -- check tests/fixtures/bad_project
cargo run -- explain ZR001
cargo run -- init   # writes zerum.toml from example
```

## Reviewer checklist

- [ ] `cargo test` passes
- [ ] `zerum check` on `bad_project` exits non-zero
- [ ] Each check id appears in `list-checks`
- [ ] Tutorial chapters 01–03 match the code layout

## Next phases

- **Phase 2** — TOML profiles, per-check tuning, external checker trait + Ruff adapter
- **Phase 3** — SARIF polish, CI recipes, architecture import rules in real monorepos
- **Phase 4** — Opt-in LLM review, audit trail, consensus

See `12-roadmap.md` (to be expanded) for the full sequence.
