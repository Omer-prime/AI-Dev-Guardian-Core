use common::{Issue, ScanResult};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error as SqlxError;


// Helper function to handle database connection pooling and avoid re-initializing the pool
async fn get_db_pool() -> Result<sqlx::PgPool, SqlxError> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/security_scans") // Replace with your credentials
        .await
}

// Function to save scan report into the database
pub async fn save_report_to_db(report: &ScanResult) -> Result<(), SqlxError> {
    let pool = get_db_pool().await?;

    // Insert each issue into the database
    for issue in &report.issues {
        let severity = format!("{:?}", issue.severity);

        sqlx::query(
            r#"
            INSERT INTO scans (file_path, issue_id, severity, description, line)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&issue.file)
        .bind(&issue.id)
        .bind(severity) // Convert Severity to text safely
        .bind(&issue.description)
        .bind(issue.line as i32) // Explicitly cast `line` to a valid SQL type (e.g., `i32`)
        .execute(&pool)
        .await?;
    }

    Ok(())
}

// Function to generate a report with a security score based on issue severity
pub fn generate_report(issues: Vec<Issue>) -> ScanResult {
    let mut score = 100;

    for issue in &issues {
        score -= match issue.severity {
            common::Severity::Critical => 25,
            common::Severity::High => 15,
            common::Severity::Medium => 5,
            common::Severity::Low => 2,
        };
    }

    if score < 0 {
        score = 0;
    }

    ScanResult {
        issues,
        score: score as u8,
    }
}

// Function to generate SARIF-compliant report for GitHub/CI compatibility
pub fn generate_sarif(report: &ScanResult) -> serde_json::Value {
    json!({
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
                    "level": format!("{:?}", issue.severity).to_lowercase(),
                    "message": {
                        "text": issue.description
                    },
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": {
                                "uri": issue.file
                            },
                            "region": {
                                "startLine": issue.line
                            }
                        }
                    }]
                })
            }).collect::<Vec<_>>()
        }]
    })
}