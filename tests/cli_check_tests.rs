use std::process::Command;

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

#[test]
fn list_checks_includes_zr001() {
    let output = Command::new(zerum_bin())
        .arg("list-checks")
        .output()
        .expect("run zerum list-checks");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ZR001"));
    assert!(stdout.contains("ZR010"));
}

#[test]
fn check_bad_project_fails() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/bad_project");
    let output = Command::new(zerum_bin())
        .args(["check", path])
        .output()
        .expect("run zerum check");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ZR002"));
    assert!(stdout.contains("ZR006"));
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
fn explain_zr001() {
    let output = Command::new(zerum_bin())
        .args(["explain", "ZR001"])
        .output()
        .expect("run zerum explain");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("too-many-branches"));
}
