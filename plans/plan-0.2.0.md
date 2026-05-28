# Zerum v0.2.0 plan (prompt-driven revision)

**Target:** `0.2.0`  
**Repository:** https://github.com/latentmeta/zerum  
**Prompt source:** `prompts/prompt-0.2.0.md`

This plan is revised to follow the prompt exactly: Zerum v0.2.0 is a **deterministic, native, educational** release (Credo-for-Python direction), with **no external checker execution, no SARIF, no LLM, no plugins, no autofix**.

---

## 1) Product boundaries (hard scope)

### Required commands

- `zerum check .`
- `zerum check path/`
- `zerum explain ZR001` (and every implemented rule)
- `zerum list-checks`
- `zerum init`

### Output formats

- `human`
- `json`

### Explicitly out of scope for 0.2.0

- SARIF output
- External checker execution (Ruff/Mypy/etc.)
- `--with-llm` behavior
- Plugins / dynamic loading / WASM
- Autofix

---

## 2) v0.2.0 objectives

1. Establish deterministic “Credo for Python” foundations with broad native rule coverage.
2. Expand Zerum from 10 rules to approximately **75 deterministic rules**.
3. Introduce metadata-rich rule definitions and excellent explain-mode output.
4. Strengthen parser abstraction and check authoring ergonomics.
5. Deliver educational assets (tutorial chapters + review notes) per milestone.

---

## 3) Architecture requirements (from prompt)

### Parser layer

- Keep `rustpython-parser`, but do not leak parser internals.
- Introduce a semantic wrapper (`SourceModel` concept) and drive checks from semantic structures:
  - functions
  - classes
  - imports
  - comments

### Core model and traits

- Keep/enhance `Issue` with deterministic, explainable fields.
- Add `CheckMetadata` with:
  - id, name, category, severity
  - `safe_fixable`
  - `examples`
- Evolve `Check` trait so each rule exposes metadata + run logic.

### Configuration behavior

- Per-rule:
  - enable/disable
  - severity override
  - rule-specific thresholds/settings
- `zerum init` generates a valid starter `zerum.toml`.

---

## 4) Rule inventory target (Layer 2 scope)

Target: **~75 deterministic rules** distributed as:

- Readability: 15 (`ZR001`–`ZR015`)
- Consistency: 10 (`ZR101`–`ZR110`)
- Design: 10 (`ZR201`–`ZR210`)
- Refactor: 15 (`ZR301`–`ZR315`)
- Warning: 15 (`ZR401`–`ZR415`)
- AI Slop (deterministic): 10 (`ZR501`–`ZR510`)

### Implementation strategy

- **Must-have for 0.2.0 GA:** all rule IDs exist, run deterministically, and support `explain`.
- **Quality bar per rule:** at least one positive test and one negative/baseline test.
- Use phased maturity labels internally:
  - M1: implemented + tested
  - M2: tuned for lower false positives

---

## 5) Milestone plan

## Milestone 0 — Plan + docs alignment

Deliver:
- This revised plan
- Tutorial skeleton for:
  - `04_rule_design.md`
  - `05_explain_mode.md`
- Decision log for metadata schema and rule naming conventions

Output report:
- What was built
- Open questions
- Technical debt
- Recommended next step

## Milestone 1 — Rule engine evolution

Deliver:
- `CheckMetadata` implementation and registry updates
- `Check` trait updates (metadata + run)
- Config severity overrides + enable/disable support wired consistently
- `list-checks` shows metadata cleanly

Tests:
- registry integrity + unique IDs
- metadata presence for all registered checks
- config override behavior

## Milestone 2 — Parser wrapper and shared check utilities

Deliver:
- semantic wrapper layer for parser output (SourceModel-like API)
- comment extraction utilities for AI-slop/readability checks
- shared utility helpers for naming/style checks

Tests:
- parser wrapper unit tests
- Unicode + location stability checks

## Milestone 3 — Readability + Warning expansions

Deliver:
- Complete readability (`ZR001`–`ZR015`)
- Complete warning (`ZR401`–`ZR415`)
- Explain entries for every added rule

Tests:
- unit tests for all new readability/warning rules
- fixtures with expected issue sets
- snapshot coverage for `human` and `json`

## Milestone 4 — Consistency + Refactor expansions

Deliver:
- Complete consistency (`ZR101`–`ZR110`)
- Complete refactor (`ZR301`–`ZR315`)

Tests:
- naming/style fixture matrix
- refactor opportunity fixtures
- false-positive regression tests

## Milestone 5 — Design + AI Slop expansions

Deliver:
- Complete design (`ZR201`–`ZR210`)
- Complete AI slop (`ZR501`–`ZR510`) as deterministic heuristics
- Explain mode polish: tradeoffs and false-positive notes per rule

Tests:
- architecture-oriented fixtures for design rules
- slop-pattern fixtures for AI category

## Milestone 6 — v0.2.0 release hardening

Deliver:
- docs refresh (README + tutorial chapters 00–05 updates)
- release notes `docs/RELEASE_v0.2.0.md`
- changelog update

Quality gates:
- `cargo test` green
- `cargo clippy -- -D warnings` green
- `cargo publish --dry-run` green
- Test coverage target documented and measured toward **>=70%**

---

## 6) Explain mode specification (0.2.0)

Each rule explanation output should include:

- Rule meaning
- Why it matters
- Examples
- False positives
- Tradeoffs
- Suggested remediation

Acceptance:
- `zerum explain <RULE_ID>` works for every rule in registry.

---

## 7) Testing strategy

Required test types (prompt-aligned):

- unit tests
- fixture tests
- snapshot tests
- reporter tests

Minimum per-rule test contract:

- 1 positive detection case
- 1 negative clean case
- explain output smoke test for rule metadata presence

Project-level tests:

- CLI smoke (`check`, `list-checks`, `explain`, `init`)
- config overrides (enable/disable/severity)
- reporter parity (`human` and `json`)

---

## 8) Risks and mitigations

- **Risk:** 75-rule target could become shallow.
  - **Mitigation:** enforce per-rule minimum test contract; defer low-confidence heuristics.
- **Risk:** false positives reduce trust.
  - **Mitigation:** prioritize explain quality + threshold tuning + regression fixtures.
- **Risk:** parser coupling complexity.
  - **Mitigation:** strict semantic wrapper boundary and shared utility layer.

---

## 9) Definition of done (v0.2.0)

v0.2.0 is done when:

1. Zerum ships deterministic native rules across all six categories (~75 total).
2. Only `human` and `json` outputs are present and stable.
3. Explain mode is complete for every rule and includes tradeoffs/false-positive guidance.
4. Config supports enable/disable and severity overrides reliably.
5. Educational deliverables (code, tests, docs, tutorial updates, review notes) are complete for each milestone.
6. No external checker execution, no SARIF, no LLM behavior is introduced in this release.

