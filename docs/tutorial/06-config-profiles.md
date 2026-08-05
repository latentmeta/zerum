# Configuration and profiles

Zerum reads `zerum.toml` from the project root (discovered by walking up from the checked path).

## Active profile

```toml
[profile]
name = "default"
```

Built-in profiles:

| Profile | Behavior |
|---------|----------|
| `default` | AST-precise and high-signal rules; noisy pattern heuristics disabled |
| `strict` | All ~75 catalog rules enabled |

Override on the CLI:

```bash
zerum check . --profile strict
```

## Per-check settings

```toml
[checks.ZR001]
enabled = true
max_lines = 80

[checks.ZR401]
severity = "critical"
```

Severity must be one of: `low`, `medium`, `high`, `critical`.

## Profile inheritance

Define layered profiles for teams:

```toml
[profile]
name = "team"

[profiles.base]
extends = "default"

[profiles.base.checks.ZR001]
max_lines = 40

[profiles.team]
extends = "base"

[profiles.team.checks.ZR401]
severity = "high"
```

Top-level `[checks.ZR###]` entries override the active profile chain.

## External checkers

```toml
external_checkers = ["ruff"]
```

Or per invocation:

```bash
zerum check . --with-external ruff
zerum list-checkers
```

External findings use ids like `EXT-RUFF-E501` (tool + rule code) and `confidence: tool_reported`.

## Schema validation

Invalid check ids (not matching `ZR###`) or unknown severity values fail at config load with a clear error.
