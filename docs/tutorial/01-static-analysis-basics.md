# Static analysis basics

Static analysis inspects source **without executing it**. Zerum’s deterministic layer answers: “Does this code violate a policy we can prove from syntax alone?”

## Deterministic vs probabilistic

| Kind | Source in Zerum | Trust model |
|------|-----------------|-------------|
| Deterministic | Native ZR* checks | Reproducible; same input → same issues |
| Tool-reported | External checkers (Phase 2) | Normalized from Ruff, Mypy, etc. |
| LLM-inferred | Optional review (Phase 4) | Never replaces deterministic truth |

`ConfidenceKind` on every `Issue` records this distinction for auditors.

## What Zerum checks in Phase 1

Governance-focused rules (complexity, design, warnings, AI placeholders, architecture imports)—not a full style linter. Compare to Ruff: Zerum explains *why* a rule exists and targets maintainability and boundaries.

## Issue shape

Each finding includes:

- **id** — e.g. `ZR002`
- **location** — file, line, column
- **severity** and **category**
- **explanation** and **remediation** (deterministic checks always populate these)

## Files to read

- `src/core/issue.rs` — data model
- `src/core/check.rs` — `Check` trait
- `src/analyzers/deterministic.rs` — orchestration loop

## Exercise

Run `zerum check tests/fixtures/bad_project` and map each reported id to the pattern in `messy.py`.
