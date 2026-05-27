use std::process::Command;

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

#[test]
fn list_checks_lists_catalog_ids() {
    let output = Command::new(zerum_bin())
        .arg("list-checks")
        .output()
        .expect("run zerum list-checks");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for id in ["ZR001", "ZR110", "ZR210", "ZR315", "ZR415", "ZR510"] {
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
    for id in ["ZR002", "ZR401", "ZR404", "ZR405", "ZR501"] {
        assert!(stdout.contains(id), "expected {id} in output");
    }
}

#[test]
fn check_arch_violation_flags_zr207() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/arch_violation");
    let output = Command::new(zerum_bin())
        .args(["check", path])
        .output()
        .expect("run zerum check arch_violation");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ZR"));
}

#[test]
fn check_simple_project_passes() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/simple_project");
    let output = Command::new(zerum_bin())
        .args(["check", path])
        .output()
        .expect("run zerum check");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ZR"));
}

#[test]
fn explain_zr001_includes_metadata() {
    let output = Command::new(zerum_bin())
        .args(["explain", "ZR001"])
        .output()
        .expect("run zerum explain");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("long-function"));
    assert!(stdout.contains("category:"));
    assert!(stdout.contains("severity:"));
    assert!(stdout.contains("False positives:"));
    assert!(stdout.contains("Tradeoffs:"));
    assert!(stdout.contains("Remediation:"));
}
