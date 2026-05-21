use std::fs;
use std::process::Command;
use tempfile::tempdir;
use zerum::cli::{EXIT_ERROR, EXIT_ISSUES};

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

#[test]
fn all_parse_failures_exit_error() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("broken.py"), "def (((: pass\n").unwrap();

    let output = Command::new(zerum_bin())
        .args(["check", dir.path().to_str().unwrap()])
        .output()
        .expect("run zerum check");

    assert_eq!(
        output.status.code(),
        Some(i32::from(EXIT_ERROR)),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn issues_found_exit_issues() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/bad_project");
    let output = Command::new(zerum_bin())
        .args(["check", path])
        .output()
        .expect("run zerum check");

    assert_eq!(output.status.code(), Some(i32::from(EXIT_ISSUES)));
}
