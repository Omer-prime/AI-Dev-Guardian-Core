use common::{Issue, ScanResult, Severity};
use serde_json::json;

pub fn generate_report(issues: Vec<Issue>) -> ScanResult {
    // Keep scoring simple + deterministic for OSS.
    // SaaS can override / augment scoring later.
    let mut score: i32 = 100;

    for issue in &issues {
        let penalty: i32 = match issue.severity {
            Severity::Critical => 25,
            Severity::High => 15,
            Severity::Medium => 5,
            Severity::Low => 2,
        };

        score = score.saturating_sub(penalty);
    }

    ScanResult {
        issues,
        score: score.clamp(0, 100) as u8,
    }
}

fn sarif_level(severity: Severity) -> &'static str {
    // SARIF levels: "error" | "warning" | "note" | "none"
    match severity {
        Severity::Critical | Severity::High => "error",
        Severity::Medium => "warning",
        Severity::Low => "note",
    }
}

pub fn generate_sarif(report: &ScanResult) -> serde_json::Value {
    json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "AI Dev Guardian",
                    "informationUri": "https://aidevguardian.dev"
                }
            },
            "results": report.issues.iter().map(|issue| {
                json!({
                    "ruleId": issue.id,
                    "level": sarif_level(issue.severity),
                    "message": { "text": issue.description },
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": { "uri": issue.file },
                            "region": { "startLine": issue.line }
                        }
                    }]
                })
            }).collect::<Vec<_>>()
        }]
    })
}
