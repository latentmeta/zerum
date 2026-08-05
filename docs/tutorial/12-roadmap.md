# Roadmap

## Phase 1 (complete) — Foundation

- [x] Core issue model and `Check` trait
- [x] rustpython-parser integration
- [x] CLI: `check`, `explain`, `list-checks`, `init`
- [x] Reporters: human, JSON
- [x] Tutorial chapters 00–03

## v0.2.0 foundations (complete)

- [x] ~75-rule deterministic catalog (ZR001–ZR510)
- [x] `SourceModel`, `CheckMetadata`, severity override
- [x] AST-precise detector module (`catalog_detectors`)
- [x] Tutorial chapters 04–05
- [x] Published to crates.io

## v0.2.x / v0.4.0 hardening (complete)

- [x] Default profile (disable noisy heuristics)
- [x] Strict profile and `clean_project` fixture
- [x] Per-rule explain matrix tests
- [x] AST batch: ZR101, ZR306, ZR506
- [x] CI coverage gate ≥70%

## Phase 2 — Configuration (complete in v0.4.0)

- [x] `zerum.toml` schema validation
- [x] Profile inheritance (`[profiles.*]`)
- [x] Tutorial chapter 06

## Phase 3 — External tools (complete in v0.4.0)

- [x] `ExternalChecker` trait + Ruff JSON adapter
- [x] `list-checkers` CLI
- [x] Optional SARIF reporter (`--features sarif`)

## v0.4.1 — distribution (complete)

- [x] PyPI wheels via maturin (`pip` / `pipx` / `uv`)
- [x] Homebrew formula scaffold + Release `SHA256SUMS`
- [x] User-focused README and CI examples for Python projects

## v0.5.0 — remediation prompts (complete)

- [x] `zerum check --remediation-prompt` (deterministic; no LLM call)
- [x] Findings grouped by check type, ordered by severity
- [x] Source snippets + shared remediation per type

## Phase 4 — LLM layer (future)

- [ ] Opt-in `zerum check --with-llm`
- [ ] Provider trait, redaction, audit log
- [ ] Tutorial chapters 10–11
