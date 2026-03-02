use super::{HardcodedSecretRule, SecurityRule, SqlInjectionRule};

pub struct RuleRegistry;

impl RuleRegistry {
    pub fn all_rules() -> Vec<Box<dyn SecurityRule>> {
        vec![Box::new(HardcodedSecretRule), Box::new(SqlInjectionRule)]
    }

    pub fn filter_rules(
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
    ) -> Vec<Box<dyn SecurityRule>> {
        Self::all_rules()
            .into_iter()
            .filter(|rule| {
                let id = rule.id().to_string();

                // If include list is provided, keep only those IDs
                if let Some(ref include_list) = include
                    && !include_list.contains(&id)
                {
                    return false;
                }

                // If exclude list is provided, drop those IDs
                if let Some(ref exclude_list) = exclude
                    && exclude_list.contains(&id)
                {
                    return false;
                }

                true
            })
            .collect()
    }
}
