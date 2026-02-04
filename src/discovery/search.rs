// Search functionality for the step catalog
use super::catalog::StepInfo;

pub fn search_steps<'a>(steps: &'a [StepInfo], query: &str) -> Vec<&'a StepInfo> {
    let query_lower = query.to_lowercase();
    steps
        .iter()
        .filter(|step| {
            step.id.contains(&query_lower)
                || step.description.to_lowercase().contains(&query_lower)
                || step.category.to_lowercase().contains(&query_lower)
                || step
                    .aliases
                    .iter()
                    .any(|alias| alias.to_lowercase().contains(&query_lower))
                || step
                    .examples
                    .iter()
                    .any(|example| example.to_lowercase().contains(&query_lower))
        })
        .collect()
}

pub fn filter_by_category<'a>(steps: &'a [StepInfo], category: &str) -> Vec<&'a StepInfo> {
    steps
        .iter()
        .filter(|step| step.category.eq_ignore_ascii_case(category))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::catalog::*;

    #[test]
    fn test_search_by_id() {
        let catalog = crate::discovery::catalog::build_step_catalog();
        let results = search_steps(catalog.all_steps(), "click");
        assert!(!results.is_empty());
        assert!(results.iter().any(|s| s.id == "click"));
    }

    #[test]
    fn test_search_by_description() {
        let catalog = build_step_catalog();
        let results = search_steps(catalog.all_steps(), "navigate");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_filter_by_category() {
        let catalog = build_step_catalog();
        let results = filter_by_category(catalog.all_steps(), "Navigation");
        assert!(!results.is_empty());
        assert!(results.iter().all(|s| s.category == "Navigation"));
    }
}
