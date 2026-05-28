use std::process::Command;

#[test]
fn list_checkers_includes_ruff() {
    let output = Command::new(env!("CARGO_BIN_EXE_zerum"))
        .arg("list-checkers")
        .output()
        .expect("run list-checkers");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ruff"));
}
