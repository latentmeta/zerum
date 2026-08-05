//! Per-rule contract tests: explain works for every catalog id; triggers where defined.

use std::fs;
use std::process::Command;
use tempfile::tempdir;
use zerum::core::CheckRegistry;

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

fn all_rule_ids() -> Vec<String> {
    let registry = CheckRegistry::new();
    registry.iter().map(|c| c.id().to_string()).collect()
}

fn run_check_json(source: &str, config_toml: Option<&str>) -> serde_json::Value {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("sample.py"), source).unwrap();
    if let Some(toml) = config_toml {
        fs::write(dir.path().join("zerum.toml"), toml).unwrap();
    }
    let output = Command::new(zerum_bin())
        .args(["check", dir.path().to_str().unwrap(), "--format", "json"])
        .output()
        .expect("run zerum check");
    serde_json::from_slice(&output.stdout).unwrap_or(serde_json::Value::Array(vec![]))
}

fn has_id(out: &serde_json::Value, id: &str) -> bool {
    out.as_array()
        .map(|items| {
            items
                .iter()
                .any(|i| i.get("id").and_then(|v| v.as_str()) == Some(id))
        })
        .unwrap_or(false)
}

/// Minimal snippets that should trigger a single rule (strict profile where needed).
fn trigger_snippet(id: &str) -> Option<String> {
    Some(match id {
        "ZR001" => {
            let mut s = String::from("def long_fn():\n");
            s.push_str(&"    x=1\n".repeat(60));
            s
        }
        "ZR010" => "# TODO fix\nx=1\n".to_string(),
        "ZR101" => "def snake_one():\n    pass\ndef camelCase():\n    pass\n".to_string(),
        "ZR304" => "def f(flag):\n    return flag == True\n".to_string(),
        "ZR306" => "a = \"ERROR\"\nb = \"ERROR\"\nc = \"ERROR\"\n".to_string(),
        "ZR401" => "try:\n    1/0\nexcept BaseException:\n    pass\n".to_string(),
        "ZR411" => "try:\n    1/0\nexcept Exception:\n    pass\n".to_string(),
        "ZR404" => "def f(xs=[]):\n    return xs\n".to_string(),
        "ZR406" => "def f(x):\n    assert x\n".to_string(),
        "ZR407" => "def f(c):\n    return eval(c)\n".to_string(),
        "ZR414" => "def f(xs):\n    return len(xs) > 0\n".to_string(),
        "ZR501" => "# TODO: AI generated\nx=1\n".to_string(),
        "ZR506" => "def wrap():\n    return other()\n".to_string(),
        _ => return None,
    })
}

#[test]
fn catalog_has_seventy_five_rules() {
    let ids = all_rule_ids();
    assert_eq!(ids.len(), 75);
}

#[test]
fn every_rule_explain_succeeds() {
    for id in all_rule_ids() {
        let output = Command::new(zerum_bin())
            .args(["explain", &id])
            .output()
            .expect("run explain");
        assert!(
            output.status.success(),
            "explain {} failed: {}",
            id,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn curated_rules_have_non_generic_explanation() {
    for id in ["ZR001", "ZR401", "ZR010"] {
        let output = Command::new(zerum_bin())
            .args(["explain", id])
            .output()
            .unwrap();
        let text = String::from_utf8_lossy(&output.stdout);
        assert!(
            !text.contains("maintainability risk in deterministic analysis"),
            "{id}"
        );
    }
}

#[test]
fn trigger_snippets_fire_under_strict_profile() {
    let strict = "[profile]\nname = \"strict\"\n";
    for id in all_rule_ids() {
        let Some(snippet) = trigger_snippet(&id) else {
            continue;
        };
        let out = run_check_json(&snippet, Some(strict));
        assert!(has_id(&out, &id), "expected {id} to trigger");
    }
}

#[test]
fn clean_module_passes_default_profile() {
    let output = Command::new(zerum_bin())
        .args(["check", "tests/fixtures/clean_project", "--format", "json"])
        .output()
        .expect("run check");
    assert_eq!(output.status.code(), Some(0), "{:?}", output.stderr);
}

#[test]
fn default_profile_disables_zr110_on_minimal_code() {
    let out = run_check_json("def f():\n    return 1\n", None);
    assert!(!has_id(&out, "ZR110"));
}
