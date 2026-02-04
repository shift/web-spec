// Feature-level validation
use super::errors::{ValidationError, ValidationResult, ValidationWarning};
use crate::discovery::catalog::build_step_catalog;
use std::fs;

pub fn validate_feature(feature_file_path: &str) -> Result<ValidationResult, String> {
    // Read file
    let content =
        fs::read_to_string(feature_file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    validate_feature_content(&content)
}

pub fn validate_feature_content(content: &str) -> Result<ValidationResult, String> {
    let mut result = ValidationResult::new();
    let catalog = build_step_catalog();

    // Basic syntax checks
    if !content.to_uppercase().contains("FEATURE:") {
        result.add_error(ValidationError::new(
            "MISSING_FEATURE",
            "Feature file must start with 'Feature:' declaration",
        ));
    }

    if !content.to_uppercase().contains("SCENARIO:") {
        result.add_warning(ValidationWarning::new(
            "NO_SCENARIOS",
            "Feature file contains no scenarios",
        ));
    }

    // Parse and validate each step
    let lines: Vec<&str> = content.lines().collect();
    let mut step_number = 0;

    for (_line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Check if this is a step line (starts with Given, When, Then, And, But)
        if is_step_line(trimmed) {
            step_number += 1;
            let step_text = extract_step_text(trimmed);

            // Validate this step
            if let Err(error) =
                crate::validation::step::validate_step(step_text, step_number, &catalog)
            {
                // Add line number context
                result.add_error(error);
            }
        }
    }

    Ok(result)
}

fn is_step_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("Given ")
        || trimmed.starts_with("When ")
        || trimmed.starts_with("Then ")
        || trimmed.starts_with("And ")
        || trimmed.starts_with("But ")
}

fn extract_step_text(line: &str) -> &str {
    let trimmed = line.trim();

    // Remove the keyword (Given, When, Then, And, But)
    if let Some(pos) = trimmed.find(' ') {
        &trimmed[pos + 1..]
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_feature() {
        let feature = r#"
Feature: Login
  Scenario: Valid Login
    Given I navigate to "https://example.com"
    When I click on "button.login"
    Then I should see "Welcome"
"#;

        let result = validate_feature_content(feature).unwrap();
        assert!(result.is_valid(), "Valid feature should pass validation");
    }

    #[test]
    fn test_validate_invalid_step() {
        let feature = r#"
Feature: Login
  Scenario: Invalid
    Given I foobarbaz something
"#;

        let result = validate_feature_content(feature).unwrap();
        assert!(!result.is_valid(), "Invalid step should fail validation");
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_validate_missing_feature() {
        let feature = r#"
Scenario: No feature
    Given something
"#;

        let result = validate_feature_content(feature).unwrap();
        assert!(!result.is_valid(), "Missing Feature should fail");
    }
}
