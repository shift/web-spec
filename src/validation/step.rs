// Step-level validation
use super::errors::ValidationError;
use crate::discovery::catalog::StepCatalog;
use regex::Regex;

pub fn validate_step(
    step_text: &str,
    step_number: usize,
    catalog: &StepCatalog,
) -> Result<(), ValidationError> {
    // Try to match against any registered step pattern (including aliases)
    for step_info in catalog.all_steps() {
        // Check main pattern
        if let Ok(regex) = Regex::new(&step_info.pattern) {
            if regex.is_match(step_text) {
                return Ok(());
            }
        }

        // Check aliases
        for alias in &step_info.aliases {
            if let Ok(regex) = Regex::new(alias) {
                if regex.is_match(step_text) {
                    return Ok(());
                }
            }
        }
    }

    // If no match found, try to find similar steps with enhanced suggestions
    let suggestions = find_similar_steps_with_details(step_text, catalog);

    let mut error = ValidationError::new(
        "UNKNOWN_STEP",
        format!("Step '{}' does not match any registered pattern", step_text),
    )
    .with_step(step_number, step_text.to_string());

    // Add multiple suggestions
    if !suggestions.is_empty() {
        error = error.with_suggestion(format!("Did you mean: {}?", suggestions.join(" or ")));
    }

    // Add helpful hints
    if step_text.to_lowercase().contains("click") && !suggestions.contains(&"click".to_string()) {
        error = error.with_suggestion("For clicking elements, try: 'I click on \"selector\"' or 'I click the \"button\" button'");
    }
    if step_text.to_lowercase().contains("type") && !suggestions.contains(&"type".to_string()) {
        error = error
            .with_suggestion("For typing into fields, try: 'I type \"text\" into \"selector\"'");
    }
    if step_text.to_lowercase().contains("should") && !suggestions.contains(&"should".to_string()) {
        error = error.with_suggestion("For assertions, try: 'the element \"selector\" should be visible' or 'the page should contain \"text\"'");
    }

    if suggestions.is_empty() {
        error =
            error.with_suggestion("Run 'web-spec list-steps' to see all available step patterns");
        error = error.with_suggestion(
            "Visit the documentation at docs/GETTING_STARTED.md for common step examples",
        );
    }

    Err(error)
}

fn find_similar_steps_with_details(step_text: &str, catalog: &StepCatalog) -> Vec<String> {
    let words: Vec<&str> = step_text.split_whitespace().collect();

    let mut matches: Vec<_> = catalog
        .all_steps()
        .iter()
        .filter_map(|step| {
            let step_words: Vec<&str> = step.description.split_whitespace().collect();
            let matches = words
                .iter()
                .filter(|w| {
                    step_words
                        .iter()
                        .any(|sw| sw.to_lowercase() == w.to_lowercase())
                })
                .count();

            if matches >= 2 {
                Some((step.id.clone(), matches))
            } else {
                None
            }
        })
        .collect();

    // Sort by match count (descending) to show most relevant first
    matches.sort_by(|a, b| b.1.cmp(&a.1));

    matches.into_iter().take(3).map(|(id, _)| id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_known_step() {
        let catalog = crate::discovery::catalog::build_step_catalog();
        let result = validate_step("I click on \"button.login\"", 1, &catalog);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_unknown_step() {
        let catalog = crate::discovery::catalog::build_step_catalog();
        let result = validate_step("I foobarbaz on \"button\"", 1, &catalog);
        assert!(result.is_err());
    }
}
