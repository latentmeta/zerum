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
    for id in ["ZR002", "ZR411", "ZR404", "ZR405", "ZR501"] {
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
    assert!(stdout.contains("ZR207"));
}

#[test]
fn check_simple_project_clean_under_default_profile() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/simple_project");
    let output = Command::new(zerum_bin())
        .args(["check", path])
        .output()
        .expect("run zerum check");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn check_simple_project_finds_issues_under_strict_profile() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/simple_project");
    let output = Command::new(zerum_bin())
        .args(["check", path, "--profile", "strict"])
        .output()
        .expect("run zerum check strict");
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ZR"));
}

#[test]
fn check_writes_remediation_prompt_file() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/bad_project");
    let dir = tempfile::tempdir().expect("tempdir");
    let prompt = dir.path().join("fix.md");
    let output = Command::new(zerum_bin())
        .args([
            "check",
            path,
            "--remediation-prompt",
            prompt.to_str().unwrap(),
        ])
        .output()
        .expect("run zerum check --remediation-prompt");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Wrote remediation prompt"),
        "stderr={stderr}"
    );
    let body = std::fs::read_to_string(&prompt).expect("read prompt");
    assert!(body.contains("Zerum remediation prompt"));
    assert!(body.contains("ZR"));
    assert!(body.contains("## Findings by type"));
    assert!(body.contains("### Types (severity order)"));
    assert!(body.contains("## Instructions for the assistant"));
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
