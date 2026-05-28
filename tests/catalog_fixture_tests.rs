use std::path::PathBuf;
use std::process::Command;

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

fn fixture_root(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn run_check(fixture: &str) -> String {
    let path = fixture_root(fixture);
    let output = Command::new(zerum_bin())
        .args(["check", path.to_str().unwrap()])
        .output()
        .expect("run zerum check");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn contains_id(out: &str, id: &str) -> bool {
    out.contains(id)
}

#[test]
fn consistency_fixture_flags_mixed_test_naming() {
    let out = run_check("consistency_project");
    assert!(contains_id(&out, "ZR106"));
}

#[test]
fn refactor_fixture_flags_redundant_boolean_compare() {
    let out = run_check("refactor_project");
    assert!(contains_id(&out, "ZR304"));
    assert!(contains_id(&out, "ZR303"));
    assert!(contains_id(&out, "ZR302"));
    assert!(contains_id(&out, "ZR315"));
}

#[test]
fn design_fixture_flags_service_object_explosion() {
    let out = run_check("design_project");
    assert!(contains_id(&out, "ZR209"));
}

#[test]
fn ai_fixture_flags_placeholder_and_dead_branch() {
    let out = run_check("ai_slop_project");
    assert!(contains_id(&out, "ZR501"));
    assert!(contains_id(&out, "ZR507"));
    assert!(contains_id(&out, "ZR508"));
}

#[test]
fn warning_fixture_flags_silent_exception_swallowing() {
    let out = run_check("warning_project");
    assert!(contains_id(&out, "ZR408"));
}
