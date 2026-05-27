use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

#[test]
fn severity_override_applies_to_matching_check() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(
        root.join("zerum.toml"),
        r#"
[checks.ZR001]
enabled = true
severity = "critical"
max_lines = 1
"#,
    )
    .unwrap();
    fs::write(root.join("example.py"), "def long_fn():\n    a=1\n    b=2\n    c=3\n").unwrap();

    let output = Command::new(zerum_bin())
        .args(["check", root.to_str().unwrap(), "--format", "json"])
        .output()
        .expect("run zerum check");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"id\": \"ZR001\""), "stdout: {stdout}");
    assert!(
        stdout.contains("\"severity\": \"critical\""),
        "stdout: {stdout}"
    );
}

