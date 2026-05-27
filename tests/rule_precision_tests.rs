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

#[test]
fn zr304_redundant_boolean_compare_triggers() {
    let out = run_json("def f(flag):\n    return flag == True\n");
    assert!(has_id(&out, "ZR304"));
}

#[test]
fn zr304_identity_compare_is_clean() {
    let out = run_json("def f(flag):\n    return flag\n");
    assert!(!has_id(&out, "ZR304"));
}

#[test]
fn zr009_commented_out_def_in_comment_triggers() {
    let out = run_json("# def old():\n#     pass\nx = 1\n");
    assert!(has_id(&out, "ZR009"));
}

#[test]
fn zr009_plain_comment_is_clean() {
    let out = run_json("# keep this note for reviewers\nx = 1\n");
    assert!(!has_id(&out, "ZR009"));
}

#[test]
fn zr501_ai_placeholder_comment_triggers() {
    let out = run_json("# TODO: AI implement\nx = 1\n");
    assert!(has_id(&out, "ZR501"));
}

#[test]
fn zr507_constant_false_branch_triggers() {
    let out = run_json("def f():\n    if False:\n        return 1\n");
    assert!(has_id(&out, "ZR507"));
}

#[test]
fn zr409_module_level_mutable_triggers() {
    let out = run_json("CACHE = []\n");
    assert!(has_id(&out, "ZR409"));
}

#[test]
fn zr409_function_local_mutable_is_clean() {
    let out = run_json("def f():\n    xs = []\n    return xs\n");
    assert!(!has_id(&out, "ZR409"));
}

#[test]
fn zr408_typed_except_pass_triggers() {
    let out = run_json(
        "def f():\n    try:\n        return 1\n    except ValueError:\n        pass\n",
    );
    assert!(has_id(&out, "ZR408"));
}

#[test]
fn zr402_bare_except_pass_triggers() {
    let out = run_json("def f():\n    try:\n        return 1\n    except:\n        pass\n");
    assert!(has_id(&out, "ZR402"));
}

#[test]
fn zr303_else_after_return_triggers() {
    let out = run_json(
        "def f(x):\n    if x:\n        return 1\n    else:\n        return 2\n",
    );
    assert!(has_id(&out, "ZR303"));
}

#[test]
fn zr310_identity_passthrough_triggers() {
    let out = run_json("def echo(x):\n    return x\n");
    assert!(has_id(&out, "ZR310"));
}

#[test]
fn zr311_identity_map_triggers() {
    let out = run_json("def f(xs):\n    return list(map(lambda x: x, xs))\n");
    assert!(has_id(&out, "ZR311"));
}

#[test]
fn zr411_except_exception_triggers() {
    let out = run_json(
        "def f():\n    try:\n        return 1\n    except Exception:\n        return 0\n",
    );
    assert!(has_id(&out, "ZR411"));
}

#[test]
fn zr413_silent_fallback_triggers() {
    let out = run_json(
        "def f():\n    try:\n        return 1\n    except ValueError:\n        return None\n",
    );
    assert!(has_id(&out, "ZR413"));
}

#[test]
fn zr504_generic_exception_message_triggers() {
    let out = run_json("def f():\n    raise Exception(\"error\")\n");
    assert!(has_id(&out, "ZR504"));
}

