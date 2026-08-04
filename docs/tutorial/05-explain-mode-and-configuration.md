# Explain mode and configuration (v0.2.0)

Zerum is designed to be **educational**: every finding should be understandable without external docs. v0.2.0 extends explain output and configuration beyond the Phase 1 ten-check baseline.

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

Every catalog rule implements `Check::explanation`, `false_positives`, `tradeoffs`, and `remediation`. Metadata (`CheckMetadata`) adds `safe_fixable` and examples for future autofix work (not implemented in v0.2.0).

## Configuration

`zerum init` writes a starter `zerum.toml`. Per-check sections support:

```toml
[checks.ZR001]
enabled = true
max_lines = 80

[checks.ZR401]
severity = "warning"
```

Supported overrides (see `src/config.rs`):

- `enabled` — skip the rule when `false`
- `severity` — remap issue severity for CI gates
- Thresholds — `max_lines`, `max_arguments`, `max_depth`, `max_methods`, etc.
- ZR207 architecture — `[[checks.ZR207.rules]]` with `from` layer path and `forbidden` module prefix

Discovery walks upward from the target path for `zerum.toml`, stopping at project roots (`.git`, `pyproject.toml`, or an existing config).

## Design decisions

| Decision | Rationale |
|----------|-----------|
| Severity override at analysis time | Same rule id, different CI policy per repo |
| No SARIF in v0.2.0 | Scope control; human + JSON only |
| Static explain text | Deterministic, offline, no LLM dependency |

## Alternatives considered

- **Embedding examples in TOML** — flexible but duplicates catalog; kept in Rust for now
- **Profile inheritance** — planned; not in v0.2.0

## Limitations

- `severity` strings must match supported variants; unknown values fall back to the rule default.
- Config schema is not validated with a separate JSON Schema file yet.
- Explain text is generic for many catalog rules; per-rule curated copy is incremental work.

## Testing configuration

`tests/config_override_tests.rs` verifies severity remapping. Rule precision tests use ephemeral temp dirs with inline Python sources.

## Related chapters

- [04 — Writing checks](04-writing-checks.md)
- [12 — Roadmap](12-roadmap.md)
