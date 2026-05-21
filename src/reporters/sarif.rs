use crate::core::Issue;
use crate::reporters::Reporter;
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
struct SarifLog {
    version: String,
    #[serde(rename = "$schema")]
    schema: String,
    runs: Vec<SarifRun>,
}

#[derive(Serialize)]
struct SarifRun {
    tool: SarifTool,
    results: Vec<SarifResult>,
}

#[derive(Serialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Serialize)]
struct SarifDriver {
    name: String,
    rules: Vec<SarifRule>,
}

#[derive(Serialize)]
struct SarifRule {
    id: String,
    name: String,
    short_description: SarifText,
}

#[derive(Serialize)]
struct SarifText {
    text: String,
}

#[derive(Serialize)]
struct SarifResult {
    rule_id: String,
    level: String,
    message: SarifText,
    locations: Vec<SarifLocation>,
}

#[derive(Serialize)]
struct SarifLocation {
    physical_location: SarifPhysicalLocation,
}

#[derive(Serialize)]
struct SarifPhysicalLocation {
    artifact_location: SarifArtifactLocation,
    region: SarifRegion,
}

#[derive(Serialize)]
struct SarifArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
struct SarifRegion {
    start_line: usize,
    start_column: usize,
}

pub struct SarifReporter;

impl Reporter for SarifReporter {
    fn report(&self, issues: &[Issue]) -> Result<String> {
        let mut rules = Vec::new();
        let mut results = Vec::new();
        for issue in issues {
            if !rules.iter().any(|r: &SarifRule| r.id == issue.id) {
                rules.push(SarifRule {
                    id: issue.id.clone(),
                    name: issue.title.clone(),
                    short_description: SarifText {
                        text: issue.title.clone(),
                    },
                });
            }
            results.push(SarifResult {
                rule_id: issue.id.clone(),
                level: severity_to_level(issue.severity),
                message: SarifText {
                    text: issue.message.clone(),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: issue.file.display().to_string(),
                        },
                        region: SarifRegion {
                            start_line: issue.line,
                            start_column: issue.column,
                        },
                    },
                }],
            });
        }
        let log = SarifLog {
            version: "2.1.0".to_string(),
            schema: "https://json.schemastore.org/sarif-2.1.0.json".to_string(),
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "zerum".to_string(),
                        rules,
                    },
                },
                results,
            }],
        };
        Ok(serde_json::to_string_pretty(&log)?)
    }
}

fn severity_to_level(severity: crate::core::Severity) -> String {
    use crate::core::Severity;
    match severity {
        Severity::Info | Severity::Low => "note",
        Severity::Medium => "warning",
        Severity::High | Severity::Critical => "error",
    }
    .to_string()
}
