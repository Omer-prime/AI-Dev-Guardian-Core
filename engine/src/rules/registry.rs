use super::{HardcodedSecretRule, SqlInjectionRule, SecurityRule};

pub struct RuleRegistry;

impl RuleRegistry {
    pub fn all_rules() -> Vec<Box<dyn SecurityRule>> {
        vec![
            Box::new(HardcodedSecretRule),
            Box::new(SqlInjectionRule),
        ]
    }

    pub fn filter_rules(
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
    ) -> Vec<Box<dyn SecurityRule>> {
        let rules = Self::all_rules();

        rules
            .into_iter()
            .filter(|rule| {
                let id = rule.id().to_string();

                if let Some(ref include_list) = include {
                    if !include_list.contains(&id) {
                        return false;
                    }
                }

                if let Some(ref exclude_list) = exclude {
                    if exclude_list.contains(&id) {
                        return false;
                    }
                }

                true
            })
            .collect()
    }
}