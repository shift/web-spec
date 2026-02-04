// Performance alerts and monitoring system
use crate::execution::result::{ScenarioResult, StepResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub name: String,
    pub enabled: bool,
    pub thresholds: Vec<AlertThreshold>,
    pub notifications: Vec<AlertNotification>,
}

impl Default for AlertConfig {
    fn default() -> Self {
        AlertConfig {
            name: "default".to_string(),
            enabled: true,
            thresholds: vec![
                AlertThreshold {
                    name: "slow_scenario".to_string(),
                    metric: AlertMetric::ScenarioDurationMs,
                    operator: AlertOperator::GreaterThan,
                    value: 30000.0, // 30 seconds
                    severity: AlertSeverity::Warning,
                    message: "Scenario exceeded {:.1}s duration".to_string(),
                },
                AlertThreshold {
                    name: "very_slow_scenario".to_string(),
                    metric: AlertMetric::ScenarioDurationMs,
                    operator: AlertOperator::GreaterThan,
                    value: 60000.0, // 60 seconds
                    severity: AlertSeverity::Critical,
                    message: "Scenario exceeded {:.1}s duration".to_string(),
                },
                AlertThreshold {
                    name: "slow_step".to_string(),
                    metric: AlertMetric::StepDurationMs,
                    operator: AlertOperator::GreaterThan,
                    value: 10000.0, // 10 seconds
                    severity: AlertSeverity::Warning,
                    message: "Step exceeded {:.1}s duration".to_string(),
                },
                AlertThreshold {
                    name: "high_failure_rate".to_string(),
                    metric: AlertMetric::FailureRatePercent,
                    operator: AlertOperator::GreaterThan,
                    value: 10.0, // 10%
                    severity: AlertSeverity::Warning,
                    message: "Failure rate exceeded {:.1}%".to_string(),
                },
            ],
            notifications: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThreshold {
    pub name: String,
    pub metric: AlertMetric,
    pub operator: AlertOperator,
    pub value: f64,
    pub severity: AlertSeverity,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertMetric {
    ScenarioDurationMs,
    StepDurationMs,
    FailureRatePercent,
    TotalDurationMs,
    ScenariosPerSecond,
    StepsPerSecond,
    MemoryUsageMb,
    Custom { key: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertOperator {
    GreaterThan,
    LessThan,
    EqualTo,
    NotEqualTo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotification {
    pub channel: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct PerformanceAlert {
    pub timestamp: String,
    pub severity: AlertSeverity,
    pub threshold_name: String,
    pub message: String,
    pub metric: String,
    pub value: f64,
    pub threshold_value: f64,
    pub feature: Option<String>,
    pub scenario: Option<String>,
    pub step: Option<String>,
}

impl std::fmt::Display for PerformanceAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}] {}: {} (value: {:.2}, threshold: {:.2})",
            self.severity, self.threshold_name, self.message, self.value, self.threshold_value
        )
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    start_time: Instant,
    scenario_durations: Vec<Duration>,
    step_durations: Vec<Duration>,
    step_count: usize,
    scenario_count: usize,
    failed_scenarios: usize,
    skipped_scenarios: usize,
    custom_metrics: HashMap<String, f64>,
    alerts: Vec<PerformanceAlert>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        PerformanceMonitor {
            start_time: Instant::now(),
            scenario_durations: Vec::new(),
            step_durations: Vec::new(),
            step_count: 0,
            scenario_count: 0,
            failed_scenarios: 0,
            skipped_scenarios: 0,
            custom_metrics: HashMap::new(),
            alerts: Vec::new(),
        }
    }

    pub fn record_scenario(&mut self, scenario: &ScenarioResult) {
        self.scenario_count += 1;
        let duration = Duration::from_millis(scenario.duration_ms);
        self.scenario_durations.push(duration);

        if scenario.status == "failed" {
            self.failed_scenarios += 1;
        } else if scenario.status == "skipped" {
            self.skipped_scenarios += 1;
        }

        self.step_count += scenario.steps.len();
        for step in &scenario.steps {
            self.step_durations
                .push(Duration::from_millis(step.duration_ms));
        }
    }

    pub fn record_step(&mut self, step: &StepResult) {
        self.step_count += 1;
        self.step_durations
            .push(Duration::from_millis(step.duration_ms));
    }

    pub fn set_metric(&mut self, key: &str, value: f64) {
        self.custom_metrics.insert(key.to_string(), value);
    }

    pub fn evaluate_thresholds(&mut self, config: &AlertConfig) -> Vec<PerformanceAlert> {
        if !config.enabled {
            return Vec::new();
        }

        let mut alerts = Vec::new();

        for threshold in &config.thresholds {
            if !self.evaluate_threshold(threshold) {
                continue;
            }

            let alert = PerformanceAlert {
                timestamp: chrono::Local::now().to_rfc3339(),
                severity: threshold.severity.clone(),
                threshold_name: threshold.name.clone(),
                message: format!("{}", threshold.message),
                metric: format!("{:?}", threshold.metric),
                value: self.get_metric_value(&threshold.metric),
                threshold_value: threshold.value,
                feature: None,
                scenario: None,
                step: None,
            };

            alerts.push(alert.clone());
            self.alerts.push(alert);
        }

        alerts
    }

    fn evaluate_threshold(&self, threshold: &AlertThreshold) -> bool {
        let value = self.get_metric_value(&threshold.metric);

        match threshold.operator {
            AlertOperator::GreaterThan => value > threshold.value,
            AlertOperator::LessThan => value < threshold.value,
            AlertOperator::EqualTo => (value - threshold.value).abs() < f64::EPSILON,
            AlertOperator::NotEqualTo => (value - threshold.value).abs() >= f64::EPSILON,
        }
    }

    fn get_metric_value(&self, metric: &AlertMetric) -> f64 {
        match metric {
            AlertMetric::ScenarioDurationMs => {
                if self.scenario_durations.is_empty() {
                    0.0
                } else {
                    self.scenario_durations.iter().sum::<Duration>().as_millis() as f64
                        / self.scenario_durations.len() as f64
                }
            }
            AlertMetric::StepDurationMs => {
                if self.step_durations.is_empty() {
                    0.0
                } else {
                    self.step_durations.iter().sum::<Duration>().as_millis() as f64
                        / self.step_durations.len() as f64
                }
            }
            AlertMetric::FailureRatePercent => {
                if self.scenario_count == 0 {
                    0.0
                } else {
                    (self.failed_scenarios as f64 / self.scenario_count as f64) * 100.0
                }
            }
            AlertMetric::TotalDurationMs => self.start_time.elapsed().as_millis() as f64,
            AlertMetric::ScenariosPerSecond => {
                let elapsed_sec = self.start_time.elapsed().as_secs_f64();
                if elapsed_sec == 0.0 {
                    0.0
                } else {
                    self.scenario_count as f64 / elapsed_sec
                }
            }
            AlertMetric::StepsPerSecond => {
                let elapsed_sec = self.start_time.elapsed().as_secs_f64();
                if elapsed_sec == 0.0 {
                    0.0
                } else {
                    self.step_count as f64 / elapsed_sec
                }
            }
            AlertMetric::MemoryUsageMb => 0.0, // Would require sysinfo crate
            AlertMetric::Custom { key } => self.custom_metrics.get(key).copied().unwrap_or(0.0),
        }
    }

    pub fn get_summary(&self) -> PerformanceSummary {
        let total_duration = self.start_time.elapsed();

        PerformanceSummary {
            total_duration_ms: total_duration.as_millis() as u64,
            scenario_count: self.scenario_count,
            scenarios_passed: self.scenario_count - self.failed_scenarios - self.skipped_scenarios,
            scenarios_failed: self.failed_scenarios,
            scenarios_skipped: self.skipped_scenarios,
            step_count: self.step_count,
            avg_scenario_duration_ms: if self.scenario_durations.is_empty() {
                0.0
            } else {
                self.scenario_durations.iter().sum::<Duration>().as_millis() as f64
                    / self.scenario_durations.len() as f64
            },
            avg_step_duration_ms: if self.step_durations.is_empty() {
                0.0
            } else {
                self.step_durations.iter().sum::<Duration>().as_millis() as f64
                    / self.step_durations.len() as f64
            },
            max_scenario_duration_ms: self
                .scenario_durations
                .iter()
                .max()
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
            max_step_duration_ms: self
                .step_durations
                .iter()
                .max()
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
            failure_rate_percent: if self.scenario_count == 0 {
                0.0
            } else {
                (self.failed_scenarios as f64 / self.scenario_count as f64) * 100.0
            },
            alerts_generated: self.alerts.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub total_duration_ms: u64,
    pub scenario_count: usize,
    pub scenarios_passed: usize,
    pub scenarios_failed: usize,
    pub scenarios_skipped: usize,
    pub step_count: usize,
    pub avg_scenario_duration_ms: f64,
    pub avg_step_duration_ms: f64,
    pub max_scenario_duration_ms: u64,
    pub max_step_duration_ms: u64,
    pub failure_rate_percent: f64,
    pub alerts_generated: usize,
}

impl std::fmt::Display for PerformanceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Duration: {}ms | Scenarios: {} ({} passed, {} failed, {} skipped) | Steps: {} | Avg Scenario: {:.1}ms | Avg Step: {:.1}ms | Failure Rate: {:.1}% | Alerts: {}",
            self.total_duration_ms,
            self.scenario_count,
            self.scenarios_passed,
            self.scenarios_failed,
            self.scenarios_skipped,
            self.step_count,
            self.avg_scenario_duration_ms,
            self.avg_step_duration_ms,
            self.failure_rate_percent,
            self.alerts_generated
        )
    }
}

pub struct AlertManager {
    configs: Vec<AlertConfig>,
}

impl AlertManager {
    pub fn new() -> Self {
        AlertManager {
            configs: Vec::new(),
        }
    }

    pub fn add_config(&mut self, config: AlertConfig) {
        self.configs.push(config);
    }

    pub fn from_config_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let configs: Vec<AlertConfig> = serde_yaml::from_str(&content)?;
        Ok(AlertManager { configs })
    }

    pub fn evaluate(&self, monitor: &mut PerformanceMonitor) -> Vec<PerformanceAlert> {
        let mut all_alerts = Vec::new();

        for config in &self.configs {
            let alerts = monitor.evaluate_thresholds(config);
            all_alerts.extend(alerts);
        }

        all_alerts
    }

    pub fn format_alerts(&self, alerts: &[PerformanceAlert], format: &str) -> String {
        match format {
            "json" => self.format_json(alerts),
            "yaml" => self.format_yaml(alerts),
            _ => self.format_text(alerts),
        }
    }

    fn format_text(&self, alerts: &[PerformanceAlert]) -> String {
        if alerts.is_empty() {
            return "No performance alerts triggered".to_string();
        }

        let mut output = String::new();
        output.push_str("=== Performance Alerts ===\n\n");

        let mut critical_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;

        for alert in alerts {
            match alert.severity {
                AlertSeverity::Critical => critical_count += 1,
                AlertSeverity::Warning => warning_count += 1,
                AlertSeverity::Info => info_count += 1,
            }

            output.push_str(&format!("{}\n", alert));
            if let Some(ref scenario) = alert.scenario {
                output.push_str(&format!("  Scenario: {}\n", scenario));
            }
            output.push('\n');
        }

        output.push_str(&format!(
            "Summary: {} critical, {} warning, {} info\n",
            critical_count, warning_count, info_count
        ));

        output
    }

    fn format_json(&self, alerts: &[PerformanceAlert]) -> String {
        let json = serde_json::json!({
            "alerts": alerts.iter().map(|a| serde_json::json!({
                "timestamp": a.timestamp,
                "severity": format!("{:?}", a.severity),
                "threshold": a.threshold_name,
                "message": a.message,
                "metric": a.metric,
                "value": a.value,
                "threshold_value": a.threshold_value,
            })).collect::<Vec<_>>(),
            "count": alerts.len(),
        });
        serde_json::to_string_pretty(&json).unwrap_or_default()
    }

    fn format_yaml(&self, alerts: &[PerformanceAlert]) -> String {
        let value: serde_yaml::Value = serde_yaml::to_value(&serde_json::json!({
            "alerts": alerts.iter().map(|a| serde_yaml::to_value(&serde_json::json!({
                "timestamp": a.timestamp,
                "severity": format!("{:?}", a.severity),
                "threshold": a.threshold_name,
                "message": a.message,
                "metric": a.metric,
                "value": a.value,
                "threshold_value": a.threshold_value,
            })).unwrap()).collect::<Vec<_>>(),
            "count": alerts.len(),
        }))
        .unwrap();
        serde_yaml::to_string(&value).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_config_defaults() {
        let config = AlertConfig::default();
        assert!(config.enabled);
        assert_eq!(config.name, "default");
        assert!(!config.thresholds.is_empty());
    }

    #[test]
    fn test_alert_threshold_defaults() {
        let threshold = AlertThreshold {
            name: "slow_scenario".to_string(),
            metric: AlertMetric::ScenarioDurationMs,
            operator: AlertOperator::GreaterThan,
            value: 30000.0,
            severity: AlertSeverity::Warning,
            message: "Scenario exceeded {:.1}s duration".to_string(),
        };

        assert_eq!(threshold.value, 30000.0);
        assert_eq!(threshold.severity, AlertSeverity::Warning);
    }

    #[test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        assert_eq!(monitor.scenario_count, 0);
        assert_eq!(monitor.step_count, 0);
        assert!(monitor.alerts.is_empty());
    }

    #[test]
    fn test_performance_monitor_scenario_recording() {
        let mut monitor = PerformanceMonitor::new();

        let scenario = ScenarioResult {
            name: "Test".to_string(),
            status: "passed".to_string(),
            duration_ms: 5000,
            steps: vec![],
        };

        monitor.record_scenario(&scenario);
        assert_eq!(monitor.scenario_count, 1);
        assert_eq!(monitor.scenario_durations.len(), 1);
    }

    #[test]
    fn test_performance_monitor_failure_tracking() {
        let mut monitor = PerformanceMonitor::new();

        let passed = ScenarioResult {
            name: "Passed".to_string(),
            status: "passed".to_string(),
            duration_ms: 1000,
            steps: vec![],
        };

        let failed = ScenarioResult {
            name: "Failed".to_string(),
            status: "failed".to_string(),
            duration_ms: 2000,
            steps: vec![],
        };

        monitor.record_scenario(&passed);
        monitor.record_scenario(&failed);

        let summary = monitor.get_summary();
        assert_eq!(summary.scenario_count, 2);
        assert_eq!(summary.scenarios_passed, 1);
        assert_eq!(summary.scenarios_failed, 1);
        assert!((summary.failure_rate_percent - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_alert_severity_ordering() {
        assert_eq!(AlertSeverity::Info, AlertSeverity::Info);
        assert_eq!(AlertSeverity::Warning, AlertSeverity::Warning);
        assert_eq!(AlertSeverity::Critical, AlertSeverity::Critical);
    }

    #[test]
    fn test_performance_summary_display() {
        let summary = PerformanceSummary {
            total_duration_ms: 5000,
            scenario_count: 10,
            scenarios_passed: 8,
            scenarios_failed: 1,
            scenarios_skipped: 1,
            step_count: 50,
            avg_scenario_duration_ms: 500.0,
            avg_step_duration_ms: 100.0,
            max_scenario_duration_ms: 2000,
            max_step_duration_ms: 500,
            failure_rate_percent: 10.0,
            alerts_generated: 2,
        };

        let display = format!("{}", summary);
        eprintln!("ACTUAL: {}", display);
        assert!(display.contains("Duration: 5000ms"));
        assert!(display.contains("Scenarios: 10"));
        assert!(display.contains("Failure Rate: 10.0%"));
    }

    #[test]
    fn test_alert_manager_creation() {
        let manager = AlertManager::new();
        assert!(manager.configs.is_empty());
    }

    #[test]
    fn test_alert_format_text_empty() {
        let manager = AlertManager::new();
        let alerts = Vec::new();
        let output = manager.format_alerts(&alerts, "text");
        assert_eq!(output, "No performance alerts triggered");
    }

    #[test]
    fn test_alert_format_json() {
        let manager = AlertManager::new();
        let alert = PerformanceAlert {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            severity: AlertSeverity::Warning,
            threshold_name: "slow_scenario".to_string(),
            message: "Scenario exceeded 30s duration".to_string(),
            metric: "ScenarioDurationMs".to_string(),
            value: 35000.0,
            threshold_value: 30000.0,
            feature: None,
            scenario: Some("Test Scenario".to_string()),
            step: None,
        };

        let output = manager.format_alerts(&[alert], "json");
        assert!(output.contains("slow_scenario"));
        assert!(output.contains("Warning"));
    }

    #[test]
    fn test_evaluate_thresholds_with_disabled_config() {
        let mut monitor = PerformanceMonitor::new();
        let mut config = AlertConfig::default();
        config.enabled = false;

        // Add some scenarios to trigger thresholds
        let scenario = ScenarioResult {
            name: "Slow".to_string(),
            status: "passed".to_string(),
            duration_ms: 45000,
            steps: vec![],
        };
        monitor.record_scenario(&scenario);

        let alerts = monitor.evaluate_thresholds(&config);
        assert!(alerts.is_empty());
    }

    #[test]
    fn test_metric_custom_key() {
        let mut monitor = PerformanceMonitor::new();
        monitor.set_metric("memory_usage", 256.5);
        monitor.set_metric("cpu_usage", 75.0);

        let custom = AlertMetric::Custom {
            key: "memory_usage".to_string(),
        };

        let value = monitor.get_metric_value(&custom);
        assert!((value - 256.5).abs() < 0.01);
    }

    #[test]
    fn test_empty_scenario_durations() {
        let monitor = PerformanceMonitor::new();
        let summary = monitor.get_summary();

        assert_eq!(summary.avg_scenario_duration_ms, 0.0);
        assert_eq!(summary.avg_step_duration_ms, 0.0);
        assert_eq!(summary.failure_rate_percent, 0.0);
    }
}
