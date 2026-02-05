// Text output formatting for comparison results
use super::comparison::ComparisonResult;

/// Format comparison result as human-readable text
pub fn to_text_output(comparison: &ComparisonResult) -> String {
    let mut output = String::new();

    // Header
    output.push_str("=== Test Result Comparison Report ===\n\n");

    // Status
    output.push_str(&format!("Status: {}\n", comparison.status.to_uppercase()));
    output.push_str("\n");

    // Summary
    output.push_str("--- Summary ---\n");
    output.push_str(&format!(
        "Baseline: {}\n",
        comparison.summary.baseline_timestamp
    ));
    output.push_str(&format!(
        "Current:  {}\n",
        comparison.summary.current_timestamp
    ));
    output.push_str(&format!(
        "Scenarios Changed: {}\n",
        comparison.summary.scenario_changes_count
    ));
    output.push_str(&format!(
        "Step Performance Changes: {}\n",
        comparison.summary.step_changes_count
    ));
    output.push_str(&format!(
        "Regressions Detected: {}\n",
        comparison.summary.regression_count
    ));
    output.push_str(&format!(
        "Improvements Detected: {}\n\n",
        comparison.summary.improvement_count
    ));

    // Metrics
    output.push_str("--- Metrics Change ---\n");
    let metrics = &comparison.metrics_diff;
    output.push_str(&format!(
        "Passed Scenarios:  {:+} ({} → {})\n",
        metrics.passed_scenarios_diff, 0, metrics.passed_scenarios_diff
    ));
    output.push_str(&format!(
        "Failed Scenarios:  {:+} ({} → {})\n",
        metrics.failed_scenarios_diff, 0, metrics.failed_scenarios_diff
    ));
    output.push_str(&format!(
        "Skipped Scenarios: {:+} ({} → {})\n",
        metrics.skipped_scenarios_diff, 0, metrics.skipped_scenarios_diff
    ));
    output.push_str(&format!(
        "Passed Steps:      {:+} ({} → {})\n",
        metrics.passed_steps_diff, 0, metrics.passed_steps_diff
    ));
    output.push_str(&format!(
        "Failed Steps:      {:+} ({} → {})\n",
        metrics.failed_steps_diff, 0, metrics.failed_steps_diff
    ));
    output.push_str(&format!(
        "Skipped Steps:     {:+} ({} → {})\n",
        metrics.skipped_steps_diff, 0, metrics.skipped_steps_diff
    ));
    output.push_str(&format!(
        "Duration:          {:+}ms ({:.1}%)\n\n",
        metrics.duration_diff_ms, metrics.duration_change_percent
    ));

    // Regressions
    if !comparison.regressions.is_empty() {
        output.push_str("--- Regressions (CRITICAL) ---\n");
        for (idx, regression) in comparison.regressions.iter().enumerate() {
            output.push_str(&format!("  {}. {}\n", idx + 1, regression.description));
            output.push_str(&format!("     Severity: {}\n", regression.severity));
            output.push_str(&format!(
                "     Impact: {:.1} {}\n",
                regression.impact_value, regression.impact_unit
            ));
            if let Some(scenario) = &regression.scenario_name {
                output.push_str(&format!("     Scenario: {}\n", scenario));
            }
            if let Some(step) = &regression.step_text {
                output.push_str(&format!("     Step: {}\n", step));
            }
            output.push_str("\n");
        }
    }

    // Improvements
    if !comparison.improvements.is_empty() {
        output.push_str("--- Improvements ---\n");
        for (idx, improvement) in comparison.improvements.iter().enumerate() {
            output.push_str(&format!("  {}. {}\n", idx + 1, improvement.description));
            output.push_str(&format!(
                "     Value: {:.1} {}\n",
                improvement.improvement_value, improvement.improvement_unit
            ));
            if let Some(scenario) = &improvement.scenario_name {
                output.push_str(&format!("     Scenario: {}\n", scenario));
            }
            if let Some(step) = &improvement.step_text {
                output.push_str(&format!("     Step: {}\n", step));
            }
            output.push_str("\n");
        }
    }

    // Scenario changes
    if !comparison.scenario_changes.is_empty() {
        output.push_str("--- Scenario Changes ---\n");
        for scenario_change in &comparison.scenario_changes {
            output.push_str(&format!(
                "  {}: {} → {}\n",
                scenario_change.scenario_name,
                scenario_change.previous_status,
                scenario_change.current_status
            ));
            output.push_str(&format!(
                "     Duration: {}ms → {}ms\n",
                scenario_change.previous_duration_ms, scenario_change.current_duration_ms
            ));
            output.push_str(&format!(
                "     Change Type: {}\n\n",
                scenario_change.change_type
            ));
        }
    }

    // Step performance changes
    if !comparison.step_performance_changes.is_empty() {
        output.push_str("--- Step Performance Changes ---\n");
        for step_change in &comparison.step_performance_changes {
            let change_indicator = if step_change.is_regression {
                "↑"
            } else {
                "↓"
            };
            output.push_str(&format!(
                "  {} {} {:.1}% ({}x occurrence)\n",
                change_indicator,
                step_change.step_text,
                step_change.change_percent.abs(),
                step_change.occurrence_count
            ));
            output.push_str(&format!(
                "     Baseline: {:.1}ms → Current: {:.1}ms\n\n",
                step_change.baseline_avg_ms, step_change.current_avg_ms
            ));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::{
        ExecutionResult, ExecutionSummary, FeatureInfo, ScenarioResult, StepResult, compare_results,
    };

    fn create_test_result(name: &str, status: &str, duration_ms: u64) -> ExecutionResult {
        let feature = FeatureInfo {
            name: name.to_string(),
            file: Some("test.feature".to_string()),
            description: None,
        };
        let mut result = ExecutionResult::new(feature);
        result.status = status.to_string();
        result.duration_ms = duration_ms;

        let mut scenario = ScenarioResult {
            name: "Test Scenario".to_string(),
            status: status.to_string(),
            duration_ms,
            steps: Vec::new(),
        };

        let step = StepResult {
            text: "I do something".to_string(),
            keyword: "Given".to_string(),
            status: status.to_string(),
            duration_ms: duration_ms / 2,
            output: None,
            error: None,
        };

        scenario.steps.push(step);
        result.add_scenario(scenario);
        result.summary = ExecutionSummary {
            total_scenarios: 1,
            passed_scenarios: if status == "passed" { 1 } else { 0 },
            failed_scenarios: if status == "failed" { 1 } else { 0 },
            skipped_scenarios: if status == "skipped" { 1 } else { 0 },
            total_steps: 1,
            passed_steps: if status == "passed" { 1 } else { 0 },
            failed_steps: if status == "failed" { 1 } else { 0 },
            skipped_steps: if status == "skipped" { 1 } else { 0 },
        };
        result
    }

    #[test]
    fn test_comparison_text_output_format() {
        let baseline = create_test_result("Feature", "passed", 1000);
        let current = create_test_result("Feature", "passed", 1200);
        let comparison = compare_results(&baseline, &current);

        let text = to_text_output(&comparison);
        assert!(text.contains("=== Test Result Comparison Report ==="));
        assert!(text.contains("Status:"));
        assert!(text.contains("Summary"));
        assert!(text.contains("Metrics Change"));
    }

    #[test]
    fn test_comparison_text_shows_regressions() {
        let baseline = create_test_result("Feature", "passed", 1000);
        let mut current = create_test_result("Feature", "failed", 1000);
        current.status = "failed".to_string();

        let comparison = compare_results(&baseline, &current);
        let text = to_text_output(&comparison);
        assert!(text.contains("Regressions"));
    }

    #[test]
    fn test_comparison_text_shows_improvements() {
        let baseline = create_test_result("Feature", "passed", 2000);
        let current = create_test_result("Feature", "passed", 1000);

        let comparison = compare_results(&baseline, &current);
        let text = to_text_output(&comparison);
        assert!(text.contains("Improvements"));
    }
}
