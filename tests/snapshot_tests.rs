use std::process::Command;

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

#[test]
fn human_report_snapshot_bad_project() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/bad_project");
    let output = Command::new(zerum_bin())
        .args(["check", path, "--format", "human"])
        .output()
        .expect("run zerum check");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let normalized = stdout.replace(env!("CARGO_MANIFEST_DIR"), ".");
    insta::assert_snapshot!("human_bad_project", normalized);
}
