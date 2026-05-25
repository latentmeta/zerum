use std::process::Command;

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

#[test]
fn list_checks_lists_all_phase1_ids() {
    let output = Command::new(zerum_bin())
        .arg("list-checks")
        .output()
        .expect("run zerum list-checks");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for id in [
        "ZR001", "ZR002", "ZR003", "ZR004", "ZR005", "ZR006", "ZR007", "ZR008", "ZR009", "ZR010",
    ] {
        assert!(stdout.contains(id), "missing {id} in list-checks output");
    }
}

#[test]
fn check_bad_project_fails_with_expected_checks() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/bad_project");
    let output = Command::new(zerum_bin())
        .args(["check", path])
        .output()
        .expect("run zerum check");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    for id in ["ZR001", "ZR002", "ZR005", "ZR006", "ZR007", "ZR009"] {
        assert!(stdout.contains(id), "expected {id} in output");
    }
}

#[test]
fn check_arch_violation_flags_zr010() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/arch_violation");
    let output = Command::new(zerum_bin())
        .args(["check", path])
        .output()
        .expect("run zerum check arch_violation");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ZR010"));
}

#[test]
fn check_simple_project_passes() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/simple_project");
    let output = Command::new(zerum_bin())
        .args(["check", path])
        .output()
        .expect("run zerum check");
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No issues found"));
}

#[test]
fn explain_zr001_includes_metadata() {
    let output = Command::new(zerum_bin())
        .args(["explain", "ZR001"])
        .output()
        .expect("run zerum explain");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("too-many-branches"));
    assert!(stdout.contains("category:"));
    assert!(stdout.contains("severity:"));
    assert!(stdout.contains("Remediation:"));
}
