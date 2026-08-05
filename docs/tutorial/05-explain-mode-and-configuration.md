# Explain mode and configuration

Zerum is designed to be **educational**: every finding should be understandable without external docs.

## Explain mode

```bash
zerum explain ZR404
```

Output includes:

| Field | Purpose |
|-------|---------|
| Rule name + id | Stable identifier for CI and `zerum.toml` |
| Category / severity | How the finding is grouped and filtered |
| Explanation | What was detected |
| False positives | When the rule may not apply |
| Tradeoffs | Cost of enabling or tightening the rule |
| Remediation | Concrete improvement steps |

Every catalog rule implements `Check::explanation`, `false_positives`, `tradeoffs`, and `remediation`. Metadata (`CheckMetadata`) adds `safe_fixable` and examples for future autofix work. Tier-1 rules have curated copy; other rules use AST-precise or heuristic fallbacks (not a single generic "heuristic" claim for AST rules).

## Configuration

`zerum init` writes a starter `zerum.toml`. Per-check sections support:

```toml
[checks.ZR001]
enabled = true
max_lines = 80

[checks.ZR401]
severity = "high"
```

Supported overrides (see `src/config/`):

- `enabled` — skip the rule when `false`
- `severity` — remap issue severity for CI gates (`low`, `medium`, `high`, `critical`)
- Thresholds — `max_lines`, `max_arguments`, `max_depth`, `max_methods`, etc.
- ZR207 architecture — `[[checks.ZR207.rules]]` with `from` layer path and `forbidden` module prefix

Discovery walks upward from the target path for `zerum.toml`, stopping at project roots (`.git`, `pyproject.toml`, or an existing config).

Custom profiles may `extends = "strict"` or `extends = "default"`; built-in enablement follows the inheritance chain.

## Design decisions

| Decision | Rationale |
|----------|-----------|
| Severity override at analysis time | Same rule id, different CI policy per repo |
| SARIF behind `sarif` feature (on by default) | Optional for consumers that need code-scanning integration |
| Static explain text | Deterministic, offline, no LLM dependency |

## Alternatives considered

- **Embedding examples in TOML** — flexible but duplicates catalog; kept in Rust for now

## Limitations

- Invalid severity values fail at config load (no silent fallback).
- Config schema is not validated with a separate JSON Schema file yet.
- Explain text is curated for tier-1 rules; remaining rules use precise/heuristic fallbacks.
- Comment scanners treat `#` as a comment start and may mis-handle `#` inside string literals.

## Testing configuration

`tests/config_override_tests.rs` verifies severity remapping. Rule precision tests use ephemeral temp dirs with inline Python sources.

## Related chapters

- [04 — Writing checks](04-writing-checks.md)
- [06 — Config and profiles](06-config-profiles.md)
- [12 — Roadmap](12-roadmap.md)
