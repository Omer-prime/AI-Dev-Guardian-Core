use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FixSuggestion {
    pub message: String,
    pub replacement_example: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Issue {
    pub id: String,
    pub description: String,
    pub file: String,
    pub line: usize,
    pub severity: Severity,

    // Enterprise-grade metadata
    pub cwe: String,
    pub category: String,
    pub confidence: u8,

    pub fix: Option<FixSuggestion>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanResult {
    pub issues: Vec<Issue>,
    pub score: u8,
}
