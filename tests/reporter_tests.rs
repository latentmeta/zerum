use std::borrow::Cow;
use std::path::PathBuf;
use zerum::core::{Category, ConfidenceKind, Issue, Severity};
use zerum::reporters::{render, ReportKind};

fn sample_issue() -> Issue {
    Issue {
        id: "ZR001".into(),
        title: "too-many-branches".into(),
        message: "function `f` has 12 branches".into(),
        explanation: Some(Cow::Borrowed("branches are hard")),
        remediation: Some(Cow::Borrowed("refactor")),
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
    let out = render(ReportKind::Json, &[sample_issue()]).unwrap();
    assert!(out.contains("\"id\": \"ZR001\""));
}

#[test]
fn human_reporter_emits_issue_id() {
    let out = render(ReportKind::Human, &[sample_issue()]).unwrap();
    assert!(out.contains("ZR001"));
}
