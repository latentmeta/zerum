use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

fn run_json(source: &str) -> serde_json::Value {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("sample.py"), source).unwrap();
    let output = Command::new(zerum_bin())
        .args(["check", dir.path().to_str().unwrap(), "--format", "json"])
        .output()
        .expect("run zerum check");
    assert_eq!(output.status.code(), Some(1));
    serde_json::from_slice(&output.stdout).expect("valid json output")
}

fn has_id(out: &serde_json::Value, id: &str) -> bool {
    match out.as_array() {
        Some(items) => items
            .iter()
            .any(|i| i.get("id").and_then(|v| v.as_str()) == Some(id)),
        None => false,
    }
}

#[test]
fn zr010_todo_without_context_triggers_on_plain_todo() {
    let out = run_json("# TODO fix this\nx = 1\n");
    assert!(has_id(&out, "ZR010"));
}

#[test]
fn zr010_todo_with_owner_context_is_clean_for_rule() {
    let out = run_json("# TODO(team-platform): add retries\nx = 1\n");
    assert!(!has_id(&out, "ZR010"));
}

#[test]
fn zr406_assert_production_uses_ast_assert_stmt() {
    let out = run_json("def f(x):\n    assert x > 0\n    return x\n");
    assert!(has_id(&out, "ZR406"));
}

#[test]
fn zr407_eval_exec_detected_by_call_target() {
    let out = run_json("def f(code):\n    return eval(code)\n");
    assert!(has_id(&out, "ZR407"));
}

#[test]
fn zr414_len_comparison_detected() {
    let out = run_json("def f(xs):\n    return len(xs) > 0\n");
    assert!(has_id(&out, "ZR414"));
}

#[test]
fn zr414_truthy_check_does_not_trigger() {
    let out = run_json("def f(xs):\n    return bool(xs)\n");
    assert!(!has_id(&out, "ZR414"));
}

