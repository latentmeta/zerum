use zerum::core::{Category, ConfidenceKind, Issue, Severity};
use zerum::reporters::{ReportFormat, report};
use std::path::PathBuf;

fn sample_issue() -> Issue {
    Issue {
        id: "ZR001".into(),
        title: "too-many-branches".into(),
        message: "function `f` has 12 branches".into(),
        explanation: Some("branches are hard".into()),
        remediation: Some("refactor".into()),
        file: PathBuf::from("app.py"),
        line: 10,
        column: 1,
        severity: Severity::Medium,
        category: Category::Design,
        source: "zerum".into(),
        confidence: ConfidenceKind::Deterministic,
    }
}

#[test]
fn json_reporter_emits_issue_id() {
    let out = report(ReportFormat::Json, &[sample_issue()]).unwrap();
    assert!(out.contains("\"id\": \"ZR001\""));
}

#[test]
fn sarif_reporter_emits_version() {
    let out = report(ReportFormat::Sarif, &[sample_issue()]).unwrap();
    assert!(out.contains("\"version\": \"2.1.0\""));
    assert!(out.contains("ZR001"));
}
