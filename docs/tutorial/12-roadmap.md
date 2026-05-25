# Roadmap

## Phase 1 (complete) — Foundation

- [x] Core issue model and `Check` trait
- [x] rustpython-parser integration
- [x] ZR001–ZR010 checks with unit tests and fixtures
- [x] CLI: `check`, `review`, `list-checks`, `explain`, `init`
- [x] Reporters: human, JSON, Markdown, SARIF (snapshot tests on bad_project)
- [x] Exit codes 0 / 1 / 2; `review` exits 0 on findings
- [x] Tutorial chapters 00–03

## Phase 2 — Configuration and external tools

- [ ] Per-check schema validation in `zerum.toml`
- [ ] `ExternalChecker` trait + Ruff JSON adapter
- [ ] `zerum list-checkers` reports tool availability
- [ ] Tutorial chapters 04–07

## Phase 3 — Architecture and reporters

- [ ] Relative-import handling for ZR010
- [ ] SARIF validation against schema
- [ ] Tutorial chapters 08–09

## Phase 4 — LLM layer

- [ ] Opt-in `zerum check --with-llm`
- [ ] Provider trait, redaction, audit log
- [ ] Consensus and disagreement reporting
- [ ] Tutorial chapters 10–11
