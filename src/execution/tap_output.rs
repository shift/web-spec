//! TAP (Test Anything Protocol) output format
//!
//! TAP is a simple text-based format for test results that's widely used in
//! CI/CD systems. Each test result is represented as a single line:
//! - "ok N test description" for passing tests
//! - "not ok N test description" for failing tests
//! - "1..N" as the first line indicating total number of tests

use crate::execution::ExecutionResult;

/// Convert ExecutionResult to TAP format
///
/// TAP format specification:
/// - Version line: "TAP version 13"
/// - Plan line: "1..N" where N is total number of tests
/// - Test lines: "ok/not ok N description"
/// - Diagnostic lines: "# ..."
pub fn to_tap_output(result: &ExecutionResult) -> String {
    let mut output = String::new();

    // TAP version
    output.push_str("TAP version 13\n");

    // Calculate total tests (one per scenario)
    let total_tests = result.scenarios.len();

    // Plan line (must be before test lines)
    output.push_str(&format!("1..{}\n", total_tests));

    // Add diagnostic info if requested (optional)
    if let Some(ref file) = result.feature.file {
        output.push_str(&format!("# File: {}\n", file));
    }

    // Test lines (one per scenario)
    let mut test_number = 1;
    for scenario in &result.scenarios {
        let is_passed = scenario.status == "passed";
        let status = if is_passed { "ok" } else { "not ok" };
        output.push_str(&format!("{} {} {}\n", status, test_number, scenario.name));

        // Add diagnostic info for failures
        if !is_passed && !scenario.steps.is_empty() {
            // Check if any step failed
            if let Some(failed_step) = scenario.steps.iter().find(|s| s.status != "passed") {
                output.push_str("  ---\n");
                output.push_str(&format!(
                    "  message: |\n    Step failed: {}\n",
                    failed_step.text
                ));
                output.push_str("  ...\n");
            }
        }

        test_number += 1;
    }

    output
}

/// Parse TAP format to extract pass/fail counts
pub fn parse_tap_output(tap_text: &str) -> TapSummary {
    let mut passed = 0;
    let mut failed = 0;
    let mut total = 0;
    let mut version = "13".to_string();

    for line in tap_text.lines() {
        let trimmed = line.trim();

        // Parse version
        if trimmed.starts_with("TAP version") {
            version = trimmed
                .split_whitespace()
                .nth(2)
                .unwrap_or("13")
                .to_string();
        }

        // Parse plan (1..N)
        if trimmed.starts_with("1..") {
            if let Ok(n) = trimmed[3..].parse::<usize>() {
                total = n;
            }
        }

        // Parse test results
        if trimmed.starts_with("ok ") {
            passed += 1;
        } else if trimmed.starts_with("not ok ") {
            failed += 1;
        }
    }

    TapSummary {
        version,
        total,
        passed,
        failed,
    }
}

/// Summary statistics from TAP output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TapSummary {
    pub version: String,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

impl TapSummary {
    pub fn success(&self) -> bool {
        self.failed == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::{
        ExecutionResult, ExecutionSummary, FeatureInfo, ScenarioResult, StepResult,
    };

    fn create_test_result() -> ExecutionResult {
        ExecutionResult {
            status: "failed".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            duration_ms: 150,
            feature: FeatureInfo {
                name: "Test Feature".to_string(),
                file: Some("test.feature".to_string()),
                description: None,
            },
            scenarios: vec![
                ScenarioResult {
                    name: "Scenario 1".to_string(),
                    status: "passed".to_string(),
                    duration_ms: 100,
                    steps: vec![],
                },
                ScenarioResult {
                    name: "Scenario 2".to_string(),
                    status: "failed".to_string(),
                    duration_ms: 50,
                    steps: vec![StepResult {
                        text: "I click on button".to_string(),
                        keyword: "Given".to_string(),
                        status: "failed".to_string(),
                        duration_ms: 50,
                        output: None,
                        error: None,
                    }],
                },
            ],
            summary: ExecutionSummary {
                total_scenarios: 2,
                passed_scenarios: 1,
                failed_scenarios: 1,
                skipped_scenarios: 0,
                total_steps: 1,
                passed_steps: 0,
                failed_steps: 1,
                skipped_steps: 0,
            },
        }
    }

    #[test]
    fn test_tap_output_format() {
        let result = create_test_result();
        let tap = to_tap_output(&result);

        assert!(tap.contains("TAP version 13"));
        assert!(tap.contains("1..2"));
        assert!(tap.contains("ok 1 Scenario 1"));
        assert!(tap.contains("not ok 2 Scenario 2"));
    }

    #[test]
    fn test_tap_output_with_errors() {
        let result = create_test_result();
        let tap = to_tap_output(&result);

        // Should include error diagnostic for failed scenario
        assert!(tap.contains("---"));
        assert!(tap.contains("message:"));
    }

    #[test]
    fn test_parse_tap_output() {
        let tap_text = r#"TAP version 13
1..2
ok 1 Scenario 1
not ok 2 Scenario 2
  ---
  message: Step failed
  ...
"#;
        let summary = parse_tap_output(tap_text);

        assert_eq!(summary.version, "13");
        assert_eq!(summary.total, 2);
        assert_eq!(summary.passed, 1);
        assert_eq!(summary.failed, 1);
    }

    #[test]
    fn test_tap_summary_success() {
        let success = TapSummary {
            version: "13".to_string(),
            total: 2,
            passed: 2,
            failed: 0,
        };
        assert!(success.success());

        let failure = TapSummary {
            version: "13".to_string(),
            total: 2,
            passed: 1,
            failed: 1,
        };
        assert!(!failure.success());
    }
}
