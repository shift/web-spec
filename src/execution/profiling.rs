//! Profiling and performance analysis for test execution
//!
//! Provides detailed timing information, bottleneck identification,
//! and performance metrics for test execution and individual steps.

use crate::execution::{ExecutionResult, ScenarioResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance profiling metrics for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingMetrics {
    pub total_duration_ms: u64,
    pub scenarios: Vec<ScenarioMetrics>,
    pub slowest_steps: Vec<SlowestStepInfo>,
    pub bottleneck_analysis: BottleneckAnalysis,
}

/// Metrics for a single scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioMetrics {
    pub name: String,
    pub duration_ms: u64,
    pub step_count: usize,
    pub passed: bool,
    pub steps: Vec<StepMetrics>,
    pub slowest_step: Option<SlowestStepInfo>,
}

/// Metrics for a single step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepMetrics {
    pub text: String,
    pub duration_ms: u64,
    pub percentage: f32,
    pub status: String,
}

/// Information about a slow step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlowestStepInfo {
    pub text: String,
    pub total_ms: u64,
    pub calls: usize,
    pub average_ms: u64,
}

/// Analysis of execution bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckAnalysis {
    pub top_bottleneck: Option<String>,
    pub suggestions: Vec<String>,
    pub slow_scenario: Option<String>,
}

/// Generate profiling metrics from an execution result
pub fn analyze_execution(result: &ExecutionResult) -> ProfilingMetrics {
    let total_duration = result.duration_ms;

    // Analyze each scenario
    let mut scenarios = Vec::new();
    let mut all_steps = Vec::new();

    for scenario in &result.scenarios {
        let metrics = analyze_scenario(scenario, total_duration);
        all_steps.extend(metrics.steps.iter().map(|s| SlowestStepInfo {
            text: s.text.clone(),
            total_ms: s.duration_ms,
            calls: 1,
            average_ms: s.duration_ms,
        }));
        scenarios.push(metrics);
    }

    // Combine duplicate steps and count calls
    let slowest_steps = aggregate_slowest_steps(all_steps);

    // Analyze bottlenecks
    let bottleneck_analysis = analyze_bottlenecks(&slowest_steps, &scenarios);

    ProfilingMetrics {
        total_duration_ms: total_duration,
        scenarios,
        slowest_steps,
        bottleneck_analysis,
    }
}

/// Analyze a single scenario for performance metrics
fn analyze_scenario(scenario: &ScenarioResult, _total_duration: u64) -> ScenarioMetrics {
    let duration = scenario.duration_ms;
    let mut steps = Vec::new();
    let mut slowest_step: Option<SlowestStepInfo> = None;
    let mut max_step_duration = 0;

    for step in &scenario.steps {
        let percentage = if duration > 0 {
            (step.duration_ms as f32 / duration as f32) * 100.0
        } else {
            0.0
        };

        steps.push(StepMetrics {
            text: step.text.clone(),
            duration_ms: step.duration_ms,
            percentage,
            status: step.status.clone(),
        });

        // Track slowest step
        if step.duration_ms > max_step_duration {
            max_step_duration = step.duration_ms;
            slowest_step = Some(SlowestStepInfo {
                text: step.text.clone(),
                total_ms: step.duration_ms,
                calls: 1,
                average_ms: step.duration_ms,
            });
        }
    }

    ScenarioMetrics {
        name: scenario.name.clone(),
        duration_ms: duration,
        step_count: scenario.steps.len(),
        passed: scenario.status == "passed",
        steps,
        slowest_step,
    }
}

/// Aggregate and combine duplicate steps
fn aggregate_slowest_steps(steps: Vec<SlowestStepInfo>) -> Vec<SlowestStepInfo> {
    let mut map: HashMap<String, SlowestStepInfo> = HashMap::new();

    for step in steps {
        map.entry(step.text.clone())
            .and_modify(|e| {
                e.total_ms += step.total_ms;
                e.calls += 1;
                e.average_ms = e.total_ms / e.calls as u64;
            })
            .or_insert(step);
    }

    // Sort by total duration (descending)
    let mut result: Vec<_> = map.into_values().collect();
    result.sort_by(|a, b| b.total_ms.cmp(&a.total_ms));
    result.truncate(10); // Keep top 10
    result
}

/// Identify bottlenecks and generate suggestions
fn analyze_bottlenecks(
    slowest_steps: &[SlowestStepInfo],
    scenarios: &[ScenarioMetrics],
) -> BottleneckAnalysis {
    let mut suggestions = Vec::new();
    let mut top_bottleneck: Option<String> = None;
    let mut slow_scenario: Option<String> = None;

    // Find slowest step
    if let Some(slowest) = slowest_steps.first() {
        top_bottleneck = Some(slowest.text.clone());

        // Generate suggestions based on step type
        if slowest.text.contains("navigate") || slowest.text.contains("wait") {
            suggestions.push(format!(
                "The '{}' step takes {}ms - consider reducing wait times or optimizing navigation",
                slowest.text, slowest.total_ms
            ));
        }
        if slowest.text.contains("click") {
            suggestions.push(
                "Click operations are slow - verify element selectors and page responsiveness"
                    .to_string(),
            );
        }
    }

    // Find slowest scenario
    if let Some(slow) = scenarios.iter().max_by_key(|s| s.duration_ms) {
        slow_scenario = Some(slow.name.clone());
        suggestions.push(format!(
            "Scenario '{}' is the slowest at {}ms",
            slow.name, slow.duration_ms
        ));
    }

    // Add general suggestions
    if slowest_steps.len() > 1 {
        let second_slowest = &slowest_steps[1];
        if let Some(ref first) = top_bottleneck {
            let diff = slowest_steps[0].total_ms - second_slowest.total_ms;
            if diff > 1000 {
                suggestions.push(format!(
                    "Step '{}' is {}ms slower than the next slowest step - investigate this outlier",
                    first, diff
                ));
            }
        }
    }

    BottleneckAnalysis {
        top_bottleneck,
        suggestions,
        slow_scenario,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::{ExecutionSummary, FeatureInfo, StepResult};

    fn create_test_result() -> ExecutionResult {
        ExecutionResult {
            status: "passed".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            duration_ms: 1000,
            feature: FeatureInfo {
                name: "Test".to_string(),
                file: Some("test.feature".to_string()),
                description: None,
            },
            scenarios: vec![
                ScenarioResult {
                    name: "Scenario 1".to_string(),
                    status: "passed".to_string(),
                    duration_ms: 600,
                    steps: vec![
                        StepResult {
                            text: "I navigate to".to_string(),
                            keyword: "Given".to_string(),
                            status: "passed".to_string(),
                            duration_ms: 400,
                            output: None,
                            error: None,
                        },
                        StepResult {
                            text: "I click on button".to_string(),
                            keyword: "When".to_string(),
                            status: "passed".to_string(),
                            duration_ms: 200,
                            output: None,
                            error: None,
                        },
                    ],
                },
                ScenarioResult {
                    name: "Scenario 2".to_string(),
                    status: "passed".to_string(),
                    duration_ms: 400,
                    steps: vec![
                        StepResult {
                            text: "I navigate to".to_string(),
                            keyword: "Given".to_string(),
                            status: "passed".to_string(),
                            duration_ms: 300,
                            output: None,
                            error: None,
                        },
                        StepResult {
                            text: "I type text".to_string(),
                            keyword: "When".to_string(),
                            status: "passed".to_string(),
                            duration_ms: 100,
                            output: None,
                            error: None,
                        },
                    ],
                },
            ],
            summary: ExecutionSummary {
                total_scenarios: 2,
                passed_scenarios: 2,
                failed_scenarios: 0,
                skipped_scenarios: 0,
                total_steps: 4,
                passed_steps: 4,
                failed_steps: 0,
                skipped_steps: 0,
            },
        }
    }

    #[test]
    fn test_analyze_execution() {
        let result = create_test_result();
        let metrics = analyze_execution(&result);

        assert_eq!(metrics.total_duration_ms, 1000);
        assert_eq!(metrics.scenarios.len(), 2);
        assert!(!metrics.slowest_steps.is_empty());
    }

    #[test]
    fn test_scenario_metrics() {
        let result = create_test_result();
        let metrics = analyze_execution(&result);

        let scenario1 = &metrics.scenarios[0];
        assert_eq!(scenario1.name, "Scenario 1");
        assert_eq!(scenario1.duration_ms, 600);
        assert_eq!(scenario1.step_count, 2);
    }

    #[test]
    fn test_slowest_steps_aggregation() {
        let result = create_test_result();
        let metrics = analyze_execution(&result);

        // Should have aggregated "I navigate to" which appears twice
        let nav_step = metrics
            .slowest_steps
            .iter()
            .find(|s| s.text.contains("navigate"));
        assert!(nav_step.is_some());

        if let Some(step) = nav_step {
            assert_eq!(step.calls, 2);
            assert_eq!(step.total_ms, 700);
        }
    }

    #[test]
    fn test_bottleneck_analysis() {
        let result = create_test_result();
        let metrics = analyze_execution(&result);

        assert!(metrics.bottleneck_analysis.top_bottleneck.is_some());
        assert!(!metrics.bottleneck_analysis.suggestions.is_empty());
    }
}
