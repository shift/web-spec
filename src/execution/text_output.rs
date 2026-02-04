// Text output formatting for execution results
use super::result::ExecutionResult;

/// Format execution result as human-readable text
pub fn to_text_output(result: &ExecutionResult) -> String {
    let mut output = String::new();

    // Header
    output.push_str("=== Execution Report ===\n\n");

    // Feature info
    output.push_str(&format!("Feature: {}\n", result.feature.name));
    if let Some(file) = &result.feature.file {
        output.push_str(&format!("File: {}\n", file));
    }
    if let Some(desc) = &result.feature.description {
        output.push_str(&format!("Description: {}\n", desc));
    }
    output.push_str(&format!("Status: {}\n", result.status));
    output.push_str(&format!("Duration: {}ms\n", result.duration_ms));
    output.push_str(&format!("Timestamp: {}\n\n", result.timestamp));

    // Scenarios
    output.push_str(&format!("Scenarios: {}\n", result.scenarios.len()));
    for (idx, scenario) in result.scenarios.iter().enumerate() {
        output.push_str(&format!(
            "\n  {}. {} [{}]\n",
            idx + 1,
            scenario.name,
            scenario.status
        ));
        output.push_str(&format!("     Duration: {}ms\n", scenario.duration_ms));

        // Steps
        for (step_idx, step) in scenario.steps.iter().enumerate() {
            let symbol = match step.status.as_str() {
                "passed" => "✓",
                "failed" => "✗",
                "skipped" => "⊘",
                _ => "?",
            };
            output.push_str(&format!(
                "     {} {}. {} {}\n",
                symbol,
                step_idx + 1,
                step.keyword,
                step.text
            ));

            if let Some(error) = &step.error {
                output.push_str(&format!("        Error: {}\n", error.message));
                if !error.suggestions.is_empty() {
                    output.push_str("        Suggestions:\n");
                    for suggestion in &error.suggestions {
                        output.push_str(&format!("          - {}\n", suggestion));
                    }
                }
            }

            if let Some(out) = &step.output {
                if !out.is_empty() {
                    output.push_str(&format!("        Output: {}\n", out));
                }
            }
        }
    }

    // Summary
    output.push_str("\n=== Summary ===\n");
    output.push_str(&format!(
        "Scenarios: {} passed, {} failed, {} skipped (total: {})\n",
        result.summary.passed_scenarios,
        result.summary.failed_scenarios,
        result.summary.skipped_scenarios,
        result.summary.total_scenarios
    ));
    output.push_str(&format!(
        "Steps: {} passed, {} failed, {} skipped (total: {})\n",
        result.summary.passed_steps,
        result.summary.failed_steps,
        result.summary.skipped_steps,
        result.summary.total_steps
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::FeatureInfo;

    #[test]
    fn test_text_output_formatting() {
        let feature = FeatureInfo {
            name: "Test Feature".to_string(),
            file: Some("test.feature".to_string()),
            description: Some("A test feature".to_string()),
        };
        let result = ExecutionResult::new(feature);
        let text = to_text_output(&result);

        assert!(text.contains("=== Execution Report ==="));
        assert!(text.contains("Feature: Test Feature"));
        assert!(text.contains("File: test.feature"));
        assert!(text.contains("Description: A test feature"));
        assert!(text.contains("=== Summary ==="));
    }
}
