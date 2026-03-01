use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GuardianConfig {
    pub rules: Option<RuleConfig>,
}

#[derive(Debug, Deserialize)]
pub struct RuleConfig {
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}