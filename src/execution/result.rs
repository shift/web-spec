// Execution result types
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub status: String,
    pub timestamp: String,
    pub duration_ms: u64,
    pub feature: FeatureInfo,
    pub scenarios: Vec<ScenarioResult>,
    pub summary: ExecutionSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureInfo {
    pub name: String,
    pub file: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    pub name: String,
    pub status: String, // "passed", "failed", "skipped"
    pub duration_ms: u64,
    pub steps: Vec<StepResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub text: String,
    pub keyword: String,
    pub status: String, // "passed", "failed", "skipped"
    pub duration_ms: u64,
    pub output: Option<String>,
    pub error: Option<ErrorInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionSummary {
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub skipped_scenarios: usize,
    pub total_steps: usize,
    pub passed_steps: usize,
    pub failed_steps: usize,
    pub skipped_steps: usize,
}

impl ExecutionResult {
    pub fn new(feature: FeatureInfo) -> Self {
        ExecutionResult {
            status: "pending".to_string(),
            timestamp: current_iso_timestamp(),
            duration_ms: 0,
            feature,
            scenarios: Vec::new(),
            summary: ExecutionSummary::new(),
        }
    }

    pub fn add_scenario(&mut self, scenario: ScenarioResult) {
        self.scenarios.push(scenario);
    }

    pub fn update_status(&mut self) {
        // Determine overall status
        if self.summary.failed_steps > 0 {
            self.status = "failed".to_string();
        } else if self.summary.passed_steps > 0 {
            self.status = "passed".to_string();
        } else {
            self.status = "skipped".to_string();
        }
    }
}

impl ExecutionSummary {
    pub fn new() -> Self {
        ExecutionSummary {
            total_scenarios: 0,
            passed_scenarios: 0,
            failed_scenarios: 0,
            skipped_scenarios: 0,
            total_steps: 0,
            passed_steps: 0,
            failed_steps: 0,
            skipped_steps: 0,
        }
    }

    pub fn add_scenario_result(&mut self, scenario: &ScenarioResult) {
        self.total_scenarios += 1;
        match scenario.status.as_str() {
            "passed" => self.passed_scenarios += 1,
            "failed" => self.failed_scenarios += 1,
            "skipped" => self.skipped_scenarios += 1,
            _ => {}
        }

        for step in &scenario.steps {
            self.total_steps += 1;
            match step.status.as_str() {
                "passed" => self.passed_steps += 1,
                "failed" => self.failed_steps += 1,
                "skipped" => self.skipped_steps += 1,
                _ => {}
            }
        }
    }
}

impl Default for ExecutionSummary {
    fn default() -> Self {
        Self::new()
    }
}

impl ScenarioResult {
    pub fn new(name: String) -> Self {
        ScenarioResult {
            name,
            status: "pending".to_string(),
            duration_ms: 0,
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: StepResult) {
        self.steps.push(step);
    }

    pub fn update_status(&mut self) {
        // Determine status based on steps
        if self.steps.iter().any(|s| s.status == "failed") {
            self.status = "failed".to_string();
        } else if self.steps.iter().all(|s| s.status == "skipped") {
            self.status = "skipped".to_string();
        } else if self.steps.iter().any(|s| s.status == "passed") {
            self.status = "passed".to_string();
        }
    }
}

impl StepResult {
    pub fn new(text: String, keyword: String) -> Self {
        StepResult {
            text,
            keyword,
            status: "pending".to_string(),
            duration_ms: 0,
            output: None,
            error: None,
        }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = status.into();
        self
    }

    pub fn with_output(mut self, output: impl Into<String>) -> Self {
        self.output = Some(output.into());
        self
    }

    pub fn with_error(mut self, error: ErrorInfo) -> Self {
        self.error = Some(error);
        self
    }
}

impl ErrorInfo {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        ErrorInfo {
            code: code.into(),
            message: message.into(),
            suggestions: Vec::new(),
        }
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}

fn current_iso_timestamp() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(_duration) => {
            // Format as ISO 8601: YYYY-MM-DDTHH:MM:SS.sssZ
            chrono::DateTime::<chrono::Utc>::from(SystemTime::now())
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string()
        }
        Err(_) => "unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result_creation() {
        let feature = FeatureInfo {
            name: "Test Feature".to_string(),
            file: None,
            description: None,
        };
        let result = ExecutionResult::new(feature);
        assert_eq!(result.status, "pending");
    }

    #[test]
    fn test_scenario_status_update() {
        let mut scenario = ScenarioResult::new("Test Scenario".to_string());
        scenario.add_step(
            StepResult::new("Step 1".to_string(), "Given".to_string()).with_status("passed"),
        );
        scenario.update_status();
        assert_eq!(scenario.status, "passed");
    }

    #[test]
    fn test_summary_calculation() {
        let mut summary = ExecutionSummary::new();
        let scenario = ScenarioResult {
            name: "Test".to_string(),
            status: "passed".to_string(),
            duration_ms: 100,
            steps: vec![StepResult {
                text: "Step 1".to_string(),
                keyword: "Given".to_string(),
                status: "passed".to_string(),
                duration_ms: 50,
                output: None,
                error: None,
            }],
        };
        summary.add_scenario_result(&scenario);
        assert_eq!(summary.total_scenarios, 1);
        assert_eq!(summary.total_steps, 1);
        assert_eq!(summary.passed_steps, 1);
    }
}
