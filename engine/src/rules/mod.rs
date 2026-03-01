use common::{Issue, Severity, FixSuggestion};
use regex::Regex;
pub mod registry;

pub trait SecurityRule: Send + Sync {
    fn id(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn cwe(&self) -> &'static str;
    fn category(&self) -> &'static str;
    fn confidence(&self) -> u8;
    fn check(&self, file_path: &str, content: &str) -> Vec<Issue>;
}
/* ================================
   Hardcoded Secret Rule
================================ */

pub struct HardcodedSecretRule;

impl SecurityRule for HardcodedSecretRule {
    fn id(&self) -> &'static str {
        "HARD_CODED_SECRET"
    }

    fn description(&self) -> &'static str {
        "Detects hardcoded API keys, tokens, or passwords in source code."
    }

    fn cwe(&self) -> &'static str {
        "CWE-798" // Use of Hard-coded Credentials
    }

    fn category(&self) -> &'static str {
        "Sensitive Data Exposure"
    }

    fn confidence(&self) -> u8 {
        90
    }

    fn check(&self, file_path: &str, content: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        let secret_patterns = vec![
            r#"(?i)api[_-]?key\s*=\s*["'][^"']+["']"#,
            r#"(?i)secret\s*=\s*["'][^"']+["']"#,
            r#"(?i)password\s*=\s*["'][^"']+["']"#,
            r#"(?i)token\s*=\s*["'][^"']+["']"#,
        ];

        for pattern in secret_patterns {
            let re = Regex::new(pattern).unwrap();

            for (line_number, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    issues.push(Issue {
                        id: self.id().to_string(),
                        description: format!("Hardcoded secret detected: {}", line.trim()),
                        file: file_path.to_string(),
                        line: line_number + 1,
                        severity: Severity::High,
                        cwe: self.cwe().to_string(),
                        category: self.category().to_string(),
                        confidence: self.confidence(),
                        fix: Some(FixSuggestion {
                            message: "Move secret to environment variable and load via process.env or os.getenv.".to_string(),
                            replacement_example: Some("const apiKey = process.env.API_KEY;".to_string()),
                        }),
                    });
                }
            }
        }

        issues
    }
}

/* ================================
   SQL Injection Rule
================================ */

pub struct SqlInjectionRule;

impl SecurityRule for SqlInjectionRule {
    fn id(&self) -> &'static str {
        "SQL_INJECTION"
    }

    fn description(&self) -> &'static str {
        "Detects possible SQL injection via string concatenation."
    }

    fn cwe(&self) -> &'static str {
        "CWE-89" // SQL Injection
    }

    fn category(&self) -> &'static str {
        "Injection"
    }

    fn confidence(&self) -> u8 {
        80
    }

    fn check(&self, file_path: &str, content: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        let patterns = vec![
            r#"(?i)select\s+.*\+.*"#,
            r#"(?i)insert\s+.*\+.*"#,
            r#"(?i)update\s+.*\+.*"#,
            r#"(?i)delete\s+.*\+.*"#,
        ];

        for pattern in patterns {
            let re = Regex::new(pattern).unwrap();

            for (line_number, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    issues.push(Issue {
                        id: self.id().to_string(),
                        description: format!("Possible SQL injection: {}", line.trim()),
                        file: file_path.to_string(),
                        line: line_number + 1,
                        severity: Severity::Critical,
                        cwe: self.cwe().to_string(),
                        category: self.category().to_string(),
                        confidence: self.confidence(),
                        fix: Some(FixSuggestion {
                            message: "Use parameterized queries instead of string concatenation.".to_string(),
                            replacement_example: Some(
                                "db.query('SELECT * FROM users WHERE id = ?', [userId]);"
                                    .to_string(),
                            ),
                        }),
                    });
                }
            }
        }

        issues
    }
}