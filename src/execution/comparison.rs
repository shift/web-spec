// Result comparison for detecting regressions and improvements
use super::result::ExecutionResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comprehensive comparison between two execution results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    /// Overall comparison status: "regression", "improvement", "unchanged"
    pub status: String,

    /// Summary of the comparison
    pub summary: ComparisonSummary,

    /// Changes in overall metrics
    pub metrics_diff: MetricsDifference,

    /// Changes per scenario
    pub scenario_changes: Vec<ScenarioChange>,

    /// Performance changes per step
    pub step_performance_changes: Vec<StepPerformanceChange>,

    /// Detected regressions
    pub regressions: Vec<RegressionItem>,

    /// Detected improvements
    pub improvements: Vec<ImprovementItem>,
}

/// Summary of comparison results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonSummary {
    /// Baseline timestamp
    pub baseline_timestamp: String,

    /// Current timestamp
    pub current_timestamp: String,

    /// Number of scenarios with status changes
    pub scenario_changes_count: usize,

    /// Number of step performance changes
    pub step_changes_count: usize,

    /// Number of detected regressions
    pub regression_count: usize,

    /// Number of detected improvements
    pub improvement_count: usize,
}

/// Differences in overall metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsDifference {
    /// Change in passed scenarios
    pub passed_scenarios_diff: i32,

    /// Change in failed scenarios
    pub failed_scenarios_diff: i32,

    /// Change in skipped scenarios
    pub skipped_scenarios_diff: i32,

    /// Change in passed steps
    pub passed_steps_diff: i32,

    /// Change in failed steps
    pub failed_steps_diff: i32,

    /// Change in skipped steps
    pub skipped_steps_diff: i32,

    /// Change in total duration (ms)
    pub duration_diff_ms: i64,

    /// Percentage change in duration
    pub duration_change_percent: f64,
}

/// Change in a scenario between baseline and current
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioChange {
    /// Scenario name
    pub scenario_name: String,

    /// Previous status
    pub previous_status: String,

    /// Current status
    pub current_status: String,

    /// Duration in baseline (ms)
    pub previous_duration_ms: u64,

    /// Duration in current (ms)
    pub current_duration_ms: u64,

    /// Change type: "status_changed", "duration_improved", "duration_regressed", "new", "removed"
    pub change_type: String,
}

/// Performance change for a specific step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepPerformanceChange {
    /// Step text
    pub step_text: String,

    /// Average duration in baseline (ms)
    pub baseline_avg_ms: f64,

    /// Average duration in current (ms)
    pub current_avg_ms: f64,

    /// Percentage change
    pub change_percent: f64,

    /// Whether this is a regression (duration increased)
    pub is_regression: bool,

    /// Number of times this step appears
    pub occurrence_count: usize,
}

/// Detected regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionItem {
    /// Description of the regression
    pub description: String,

    /// Severity: "critical", "high", "medium", "low"
    pub severity: String,

    /// Related scenario name (if applicable)
    pub scenario_name: Option<String>,

    /// Related step text (if applicable)
    pub step_text: Option<String>,

    /// Impact metric
    pub impact_value: f64,

    /// Impact unit ("ms", "count", "%")
    pub impact_unit: String,
}

/// Detected improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementItem {
    /// Description of the improvement
    pub description: String,

    /// Related scenario name (if applicable)
    pub scenario_name: Option<String>,

    /// Related step text (if applicable)
    pub step_text: Option<String>,

    /// Improvement value
    pub improvement_value: f64,

    /// Improvement unit ("ms", "count", "%")
    pub improvement_unit: String,
}

/// Compare two execution results
pub fn compare_results(baseline: &ExecutionResult, current: &ExecutionResult) -> ComparisonResult {
    let mut scenario_changes = Vec::new();
    let mut regressions = Vec::new();
    let mut improvements = Vec::new();

    // Calculate metrics differences
    let metrics_diff = calculate_metrics_diff(baseline, current);

    // Build scenario map for current results
    let current_scenarios: HashMap<String, &crate::execution::ScenarioResult> = current
        .scenarios
        .iter()
        .map(|s| (s.name.clone(), s))
        .collect();

    let baseline_scenarios: HashMap<String, &crate::execution::ScenarioResult> = baseline
        .scenarios
        .iter()
        .map(|s| (s.name.clone(), s))
        .collect();

    // Compare scenarios
    for (scenario_name, baseline_scenario) in &baseline_scenarios {
        if let Some(current_scenario) = current_scenarios.get(scenario_name) {
            let change = compare_scenarios(baseline_scenario, current_scenario);

            // Detect regressions in status
            if baseline_scenario.status == "passed" && current_scenario.status == "failed" {
                regressions.push(RegressionItem {
                    description: format!(
                        "Scenario '{}' changed from passed to failed",
                        scenario_name
                    ),
                    severity: "critical".to_string(),
                    scenario_name: Some(scenario_name.clone()),
                    step_text: None,
                    impact_value: 1.0,
                    impact_unit: "count".to_string(),
                });
            }

            // Detect duration improvements
            if current_scenario.duration_ms < baseline_scenario.duration_ms {
                let improvement_ms =
                    baseline_scenario.duration_ms as i64 - current_scenario.duration_ms as i64;
                let improvement_percent =
                    (improvement_ms as f64 / baseline_scenario.duration_ms as f64) * 100.0;
                improvements.push(ImprovementItem {
                    description: format!(
                        "Scenario '{}' duration improved by {:.1}%",
                        scenario_name, improvement_percent
                    ),
                    scenario_name: Some(scenario_name.clone()),
                    step_text: None,
                    improvement_value: improvement_ms as f64,
                    improvement_unit: "ms".to_string(),
                });
            }
            // Detect duration regressions
            else if current_scenario.duration_ms > baseline_scenario.duration_ms {
                let regression_ms =
                    current_scenario.duration_ms as i64 - baseline_scenario.duration_ms as i64;
                let regression_percent =
                    (regression_ms as f64 / baseline_scenario.duration_ms as f64) * 100.0;
                if regression_percent > 10.0 {
                    // Only flag significant regressions (>10%)
                    regressions.push(RegressionItem {
                        description: format!(
                            "Scenario '{}' duration regressed by {:.1}%",
                            scenario_name, regression_percent
                        ),
                        severity: if regression_percent > 50.0 {
                            "high"
                        } else {
                            "medium"
                        }
                        .to_string(),
                        scenario_name: Some(scenario_name.clone()),
                        step_text: None,
                        impact_value: regression_ms as f64,
                        impact_unit: "ms".to_string(),
                    });
                }
            }

            scenario_changes.push(change);
        } else {
            // Scenario removed
            scenario_changes.push(ScenarioChange {
                scenario_name: scenario_name.clone(),
                previous_status: baseline_scenario.status.clone(),
                current_status: "removed".to_string(),
                previous_duration_ms: baseline_scenario.duration_ms,
                current_duration_ms: 0,
                change_type: "removed".to_string(),
            });
        }
    }

    // Detect new scenarios
    for (scenario_name, current_scenario) in &current_scenarios {
        if !baseline_scenarios.contains_key(scenario_name) {
            scenario_changes.push(ScenarioChange {
                scenario_name: scenario_name.clone(),
                previous_status: "new".to_string(),
                current_status: current_scenario.status.clone(),
                previous_duration_ms: 0,
                current_duration_ms: current_scenario.duration_ms,
                change_type: "new".to_string(),
            });
        }
    }

    // Analyze step performance
    let step_performance_changes =
        analyze_step_performance(baseline, current, &mut regressions, &mut improvements);

    // Determine overall status
    let overall_status = if !regressions.is_empty() {
        "regression".to_string()
    } else if !improvements.is_empty() {
        "improvement".to_string()
    } else {
        "unchanged".to_string()
    };

    ComparisonResult {
        status: overall_status,
        summary: ComparisonSummary {
            baseline_timestamp: baseline.timestamp.clone(),
            current_timestamp: current.timestamp.clone(),
            scenario_changes_count: scenario_changes.len(),
            step_changes_count: step_performance_changes.len(),
            regression_count: regressions.len(),
            improvement_count: improvements.len(),
        },
        metrics_diff,
        scenario_changes,
        step_performance_changes,
        regressions,
        improvements,
    }
}

/// Calculate differences in metrics
fn calculate_metrics_diff(
    baseline: &ExecutionResult,
    current: &ExecutionResult,
) -> MetricsDifference {
    let baseline_summary = &baseline.summary;
    let current_summary = &current.summary;

    let passed_scenarios_diff =
        current_summary.passed_scenarios as i32 - baseline_summary.passed_scenarios as i32;
    let failed_scenarios_diff =
        current_summary.failed_scenarios as i32 - baseline_summary.failed_scenarios as i32;
    let skipped_scenarios_diff =
        current_summary.skipped_scenarios as i32 - baseline_summary.skipped_scenarios as i32;
    let passed_steps_diff =
        current_summary.passed_steps as i32 - baseline_summary.passed_steps as i32;
    let failed_steps_diff =
        current_summary.failed_steps as i32 - baseline_summary.failed_steps as i32;
    let skipped_steps_diff =
        current_summary.skipped_steps as i32 - baseline_summary.skipped_steps as i32;
    let duration_diff_ms = current.duration_ms as i64 - baseline.duration_ms as i64;
    let duration_change_percent = if baseline.duration_ms > 0 {
        (duration_diff_ms as f64 / baseline.duration_ms as f64) * 100.0
    } else {
        0.0
    };

    MetricsDifference {
        passed_scenarios_diff,
        failed_scenarios_diff,
        skipped_scenarios_diff,
        passed_steps_diff,
        failed_steps_diff,
        skipped_steps_diff,
        duration_diff_ms,
        duration_change_percent,
    }
}

/// Compare individual scenarios
fn compare_scenarios(
    baseline: &crate::execution::ScenarioResult,
    current: &crate::execution::ScenarioResult,
) -> ScenarioChange {
    let change_type = if baseline.status != current.status {
        "status_changed".to_string()
    } else if current.duration_ms < baseline.duration_ms {
        "duration_improved".to_string()
    } else if current.duration_ms > baseline.duration_ms {
        "duration_regressed".to_string()
    } else {
        "unchanged".to_string()
    };

    ScenarioChange {
        scenario_name: baseline.name.clone(),
        previous_status: baseline.status.clone(),
        current_status: current.status.clone(),
        previous_duration_ms: baseline.duration_ms,
        current_duration_ms: current.duration_ms,
        change_type,
    }
}

/// Analyze step performance changes
fn analyze_step_performance(
    baseline: &ExecutionResult,
    current: &ExecutionResult,
    regressions: &mut Vec<RegressionItem>,
    improvements: &mut Vec<ImprovementItem>,
) -> Vec<StepPerformanceChange> {
    let mut step_changes = Vec::new();

    // Build step maps
    let mut baseline_step_times: HashMap<String, Vec<u64>> = HashMap::new();
    let mut current_step_times: HashMap<String, Vec<u64>> = HashMap::new();

    // Collect step timings from baseline
    for scenario in &baseline.scenarios {
        for step in &scenario.steps {
            baseline_step_times
                .entry(step.text.clone())
                .or_insert_with(Vec::new)
                .push(step.duration_ms);
        }
    }

    // Collect step timings from current
    for scenario in &current.scenarios {
        for step in &scenario.steps {
            current_step_times
                .entry(step.text.clone())
                .or_insert_with(Vec::new)
                .push(step.duration_ms);
        }
    }

    // Compare step performance
    for (step_text, baseline_times) in &baseline_step_times {
        if let Some(current_times) = current_step_times.get(step_text) {
            let baseline_avg =
                baseline_times.iter().sum::<u64>() as f64 / baseline_times.len() as f64;
            let current_avg = current_times.iter().sum::<u64>() as f64 / current_times.len() as f64;
            let change_percent = ((current_avg - baseline_avg) / baseline_avg) * 100.0;
            let is_regression = current_avg > baseline_avg;

            if change_percent.abs() > 5.0 {
                // Only track significant changes (>5%)
                let change = StepPerformanceChange {
                    step_text: step_text.clone(),
                    baseline_avg_ms: baseline_avg,
                    current_avg_ms: current_avg,
                    change_percent,
                    is_regression,
                    occurrence_count: current_times.len(),
                };

                if is_regression && change_percent > 10.0 {
                    regressions.push(RegressionItem {
                        description: format!(
                            "Step '{}' duration regressed by {:.1}%",
                            step_text, change_percent
                        ),
                        severity: if change_percent > 50.0 {
                            "high"
                        } else {
                            "medium"
                        }
                        .to_string(),
                        scenario_name: None,
                        step_text: Some(step_text.clone()),
                        impact_value: current_avg - baseline_avg,
                        impact_unit: "ms".to_string(),
                    });
                } else if !is_regression && change_percent.abs() > 10.0 {
                    improvements.push(ImprovementItem {
                        description: format!(
                            "Step '{}' duration improved by {:.1}%",
                            step_text,
                            change_percent.abs()
                        ),
                        scenario_name: None,
                        step_text: Some(step_text.clone()),
                        improvement_value: baseline_avg - current_avg,
                        improvement_unit: "ms".to_string(),
                    });
                }

                step_changes.push(change);
            }
        }
    }

    step_changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::{ExecutionSummary, FeatureInfo, ScenarioResult, StepResult};

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
    fn test_compare_identical_results() {
        let baseline = create_test_result("Feature", "passed", 1000);
        let current = create_test_result("Feature", "passed", 1000);

        let comparison = compare_results(&baseline, &current);
        assert_eq!(comparison.status, "unchanged");
        assert_eq!(comparison.summary.regression_count, 0);
        assert_eq!(comparison.summary.improvement_count, 0);
    }

    #[test]
    fn test_detect_regression_status_change() {
        let baseline = create_test_result("Feature", "passed", 1000);
        let mut current = create_test_result("Feature", "failed", 1000);
        current.status = "failed".to_string();

        let comparison = compare_results(&baseline, &current);
        assert_eq!(comparison.status, "regression");
        assert!(comparison.summary.regression_count > 0);
    }

    #[test]
    fn test_detect_duration_improvement() {
        let baseline = create_test_result("Feature", "passed", 2000);
        let current = create_test_result("Feature", "passed", 1000);

        let comparison = compare_results(&baseline, &current);
        assert_eq!(comparison.status, "improvement");
        assert!(comparison.summary.improvement_count > 0);
    }

    #[test]
    fn test_metrics_difference_calculation() {
        let baseline = create_test_result("Feature", "passed", 1000);
        let current = create_test_result("Feature", "passed", 1500);

        let comparison = compare_results(&baseline, &current);
        assert!(comparison.metrics_diff.duration_diff_ms > 0);
        assert!(comparison.metrics_diff.duration_change_percent > 0.0);
    }
}
