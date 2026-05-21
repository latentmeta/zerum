# Roadmap

## Phase 1 (current) — Foundation

- [x] Core issue model and `Check` trait
- [x] rustpython-parser integration
- [x] ZR001–ZR010 checks
- [x] CLI: `check`, `list-checks`, `explain`, `init`
- [x] Reporters: human, JSON, Markdown, SARIF
- [x] Tutorial chapters 00–03

## Phase 2 — Configuration and external tools

- [ ] Load `zerum.toml` profiles reliably in CI
- [ ] `ExternalChecker` trait + Ruff JSON adapter
- [ ] `zerum list-checkers` shows availability
- [ ] Tutorial 05–07

## Phase 3 — Architecture and reporters

- [ ] Stronger ZR010 path matching (module vs file path)
- [ ] SARIF validation against schema
- [ ] Tutorial 08–09

## Phase 4 — LLM layer

- [ ] Opt-in `zerum check --with-llm`
- [ ] Provider trait, redaction, audit log
- [ ] Consensus and disagreement reporting
- [ ] Tutorial 10–11
