use std::process::Command;

fn zerum_bin() -> String {
    env!("CARGO_BIN_EXE_zerum").to_string()
}

fn run_check_format(format: &str) -> String {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/bad_project");
    let output = Command::new(zerum_bin())
        .args(["check", path, "--format", format])
        .output()
        .expect("run zerum check");
    String::from_utf8_lossy(&output.stdout)
        .replace(env!("CARGO_MANIFEST_DIR"), ".")
}

#[test]
fn human_report_snapshot_bad_project() {
    insta::assert_snapshot!("human_bad_project", run_check_format("human"));
}

#[test]
fn json_report_snapshot_bad_project() {
    let out = run_check_format("json");
    let _: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
    insta::assert_snapshot!("json_bad_project", out);
}

#[test]
fn sarif_report_snapshot_bad_project() {
    let out = run_check_format("sarif");
    let _: serde_json::Value = serde_json::from_str(&out).expect("valid SARIF JSON");
    insta::assert_snapshot!("sarif_bad_project", out);
}
