// Batch execution support for running multiple features
use crate::execution::result::ExecutionResult;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BatchConfig {
    pub parallel: bool,
    pub max_workers: usize,
    pub timeout_seconds: u64,
    pub continue_on_failure: bool,
    pub output_format: String,
}

impl Default for BatchConfig {
    fn default() -> Self {
        BatchConfig {
            parallel: true,
            max_workers: num_cpus::get(),
            timeout_seconds: 300,
            continue_on_failure: true,
            output_format: "text".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BatchResult {
    pub total_features: usize,
    pub passed_features: usize,
    pub failed_features: usize,
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub total_duration_ms: u64,
    pub results: Vec<FeatureResult>,
    pub errors: Vec<BatchError>,
}

#[derive(Debug, Clone)]
pub struct FeatureResult {
    pub name: String,
    pub path: PathBuf,
    pub status: String,
    pub scenarios_passed: usize,
    pub scenarios_failed: usize,
    pub duration_ms: u64,
    pub result: Option<ExecutionResult>,
}

#[derive(Debug, Clone)]
pub struct BatchError {
    pub path: PathBuf,
    pub error: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct BatchProgress {
    completed: Arc<Mutex<usize>>,
    total: Arc<Mutex<usize>>,
    start_time: Instant,
    results: Arc<Mutex<Vec<FeatureResult>>>,
    errors: Arc<Mutex<Vec<BatchError>>>,
}

impl BatchProgress {
    pub fn new(total: usize) -> Self {
        BatchProgress {
            completed: Arc::new(Mutex::new(0)),
            total: Arc::new(Mutex::new(total)),
            start_time: Instant::now(),
            results: Arc::new(Mutex::new(Vec::new())),
            errors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn increment_completed(&self) {
        let mut completed = self.completed.lock().unwrap();
        *completed += 1;
    }

    pub fn get_progress(&self) -> f32 {
        let completed = *self.completed.lock().unwrap();
        let total = *self.total.lock().unwrap();
        if total == 0 {
            return 0.0;
        }
        completed as f32 / total as f32
    }

    pub fn get_elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn add_result(&self, result: FeatureResult) {
        self.results.lock().unwrap().push(result);
    }

    pub fn add_error(&self, error: BatchError) {
        self.errors.lock().unwrap().push(error);
    }

    pub fn collect_results(&self) -> Vec<FeatureResult> {
        self.results.lock().unwrap().clone()
    }

    pub fn collect_errors(&self) -> Vec<BatchError> {
        self.errors.lock().unwrap().clone()
    }
}

pub struct BatchExecutor {
    config: BatchConfig,
    progress: Option<BatchProgress>,
}

impl BatchExecutor {
    pub fn new() -> Self {
        BatchExecutor {
            config: BatchConfig::default(),
            progress: None,
        }
    }

    pub fn with_config(config: BatchConfig) -> Self {
        BatchExecutor {
            config,
            progress: None,
        }
    }

    pub fn execute(
        &mut self,
        paths: &[PathBuf],
        executor: &(impl Fn(&PathBuf) -> Result<ExecutionResult, String> + Sync + Send),
    ) -> BatchResult {
        let total = paths.len();
        self.progress = Some(BatchProgress::new(total));

        let start_time = Instant::now();

        let results: Vec<FeatureResult> = if self.config.parallel && total > 1 {
            // Optimize parallel execution with work-stealing configuration
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(self.config.max_workers)
                .build()
                .unwrap();

            pool.install(|| {
                paths
                    .par_iter()
                    .with_max_len(1) // Process one file per thread to maximize parallelism
                    .map(|path| self.execute_feature(path, executor))
                    .collect()
            })
        } else {
            paths
                .iter()
                .map(|path| self.execute_feature(path, executor))
                .collect()
        };

        let duration_ms = start_time.elapsed().as_millis() as u64;

        // Calculate aggregates
        let passed_features = results.iter().filter(|r| r.status == "passed").count();
        let failed_features = results.iter().filter(|r| r.status == "failed").count();
        let total_scenarios: usize = results
            .iter()
            .map(|r| r.scenarios_passed + r.scenarios_failed)
            .sum();
        let passed_scenarios: usize = results.iter().map(|r| r.scenarios_passed).sum();
        let failed_scenarios: usize = results.iter().map(|r| r.scenarios_failed).sum();

        // Collect errors
        let errors: Vec<BatchError> = results
            .iter()
            .filter(|r| r.result.is_none())
            .map(|r| BatchError {
                path: r.path.clone(),
                error: format!("Feature execution failed"),
                timestamp: chrono::Local::now().to_rfc3339(),
            })
            .collect();

        BatchResult {
            total_features: total,
            passed_features,
            failed_features,
            total_scenarios,
            passed_scenarios,
            failed_scenarios,
            total_duration_ms: duration_ms,
            results,
            errors,
        }
    }

    fn execute_feature(
        &self,
        path: &PathBuf,
        executor: &impl Fn(&PathBuf) -> Result<ExecutionResult, String>,
    ) -> FeatureResult {
        let name = path
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let start_time = Instant::now();

        let result = executor(path);

        let duration_ms = start_time.elapsed().as_millis() as u64;

        match result {
            Ok(exec_result) => {
                let scenarios_passed = exec_result.summary.passed_scenarios;
                let scenarios_failed = exec_result.summary.failed_scenarios;

                if let Some(progress) = &self.progress {
                    progress.increment_completed();
                }

                FeatureResult {
                    name,
                    path: path.clone(),
                    status: exec_result.status.clone(),
                    scenarios_passed,
                    scenarios_failed,
                    duration_ms,
                    result: Some(exec_result),
                }
            }
            Err(_e) => {
                if let Some(progress) = &self.progress {
                    progress.increment_completed();
                }

                FeatureResult {
                    name,
                    path: path.clone(),
                    status: "failed".to_string(),
                    scenarios_passed: 0,
                    scenarios_failed: 0,
                    duration_ms,
                    result: None,
                }
            }
        }
    }

    pub fn discover_features(path: &str, _pattern: &str) -> Result<Vec<PathBuf>, String> {
        let mut paths = Vec::new();

        let base_path = Path::new(path);
        if base_path.is_file() {
            if let Some(ext) = base_path.extension() {
                if ext == "feature" {
                    return Ok(vec![base_path.to_path_buf()]);
                }
            }
            return Err("Path is not a feature file".to_string());
        }

        if base_path.is_dir() {
            for entry in walkdir::WalkDir::new(path).max_depth(10).follow_links(true) {
                match entry {
                    Ok(entry) => {
                        if entry.file_type().is_file() {
                            if let Some(ext) = entry.path().extension() {
                                if ext == "feature" {
                                    paths.push(entry.path().to_path_buf());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not access entry: {}", e);
                    }
                }
            }
        }

        paths.sort();
        Ok(paths)
    }

    pub fn format_result(&self, result: &BatchResult, format: &str) -> String {
        match format {
            "json" => self.format_json(result),
            "yaml" => self.format_yaml(result),
            _ => self.format_text(result),
        }
    }

    fn format_text(&self, result: &BatchResult) -> String {
        let mut output = String::new();

        output.push_str("=== Batch Execution Summary ===\n\n");
        output.push_str(&format!(
            "Features:  {} total, {} passed, {} failed\n",
            result.total_features, result.passed_features, result.failed_features
        ));
        output.push_str(&format!(
            "Scenarios: {} total, {} passed, {} failed\n",
            result.total_scenarios, result.passed_scenarios, result.failed_scenarios
        ));
        output.push_str(&format!("Duration:  {}ms\n\n", result.total_duration_ms));

        output.push_str("=== Feature Results ===\n");
        for feature in &result.results {
            let status_icon = if feature.status == "passed" {
                "✓"
            } else {
                "✗"
            };
            output.push_str(&format!(
                "{} {} - {} ({}ms)\n",
                status_icon, feature.name, feature.status, feature.duration_ms
            ));
            if feature.scenarios_failed > 0 {
                output.push_str(&format!(
                    "    Failed: {}/{} scenarios\n",
                    feature.scenarios_failed,
                    feature.scenarios_passed + feature.scenarios_failed
                ));
            }
        }

        if !result.errors.is_empty() {
            output.push_str("\n=== Errors ===\n");
            for error in &result.errors {
                output.push_str(&format!("✗ {} - {}\n", error.path.display(), error.error));
            }
        }

        output
    }

    fn format_json(&self, result: &BatchResult) -> String {
        let json = serde_json::json!({
            "batch_summary": {
                "total_features": result.total_features,
                "passed_features": result.passed_features,
                "failed_features": result.failed_features,
                "total_scenarios": result.total_scenarios,
                "passed_scenarios": result.passed_scenarios,
                "failed_scenarios": result.failed_scenarios,
                "duration_ms": result.total_duration_ms,
            },
            "features": result.results.iter().map(|f| serde_json::json!({
                "name": f.name,
                "path": f.path.to_string_lossy(),
                "status": f.status,
                "scenarios_passed": f.scenarios_passed,
                "scenarios_failed": f.scenarios_failed,
                "duration_ms": f.duration_ms,
            })).collect::<Vec<_>>(),
            "errors": result.errors.iter().map(|e| serde_json::json!({
                "path": e.path.to_string_lossy(),
                "error": e.error,
            })).collect::<Vec<_>>(),
        });
        serde_json::to_string_pretty(&json).unwrap_or_default()
    }

    fn format_yaml(&self, result: &BatchResult) -> String {
        let yaml = serde_yaml::to_value(&serde_json::json!({
            "batch_summary": {
                "total_features": result.total_features,
                "passed_features": result.passed_features,
                "failed_features": result.failed_features,
                "total_scenarios": result.total_scenarios,
                "passed_scenarios": result.passed_scenarios,
                "failed_scenarios": result.failed_scenarios,
                "duration_ms": result.total_duration_ms,
            },
            "features": result.results.iter().map(|f| serde_yaml::to_value(&serde_json::json!({
                "name": f.name,
                "path": f.path.to_string_lossy(),
                "status": f.status,
                "scenarios_passed": f.scenarios_passed,
                "scenarios_failed": f.scenarios_failed,
                "duration_ms": f.duration_ms,
            })).unwrap()).collect::<Vec<_>>(),
        }))
        .unwrap();
        serde_yaml::to_string(&yaml).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_mock_result(path: &str, status: &str) -> ExecutionResult {
        ExecutionResult {
            status: status.to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
            duration_ms: 100,
            feature: crate::execution::FeatureInfo {
                name: path.to_string(),
                file: Some(path.to_string()),
                description: Some("Test feature".to_string()),
            },
            scenarios: vec![],
            summary: crate::execution::ExecutionSummary {
                total_scenarios: 5,
                passed_scenarios: if status == "passed" { 5 } else { 3 },
                failed_scenarios: if status == "failed" { 2 } else { 0 },
                skipped_scenarios: 0,
                total_steps: 20,
                passed_steps: if status == "passed" { 20 } else { 18 },
                failed_steps: if status == "failed" { 2 } else { 0 },
                skipped_steps: 0,
            },
        }
    }

    #[test]
    fn test_batch_config_defaults() {
        let config = BatchConfig::default();
        assert!(config.parallel);
        assert_eq!(config.max_workers, num_cpus::get());
        assert_eq!(config.timeout_seconds, 300);
        assert!(config.continue_on_failure);
    }

    #[test]
    fn test_batch_config_custom() {
        let config = BatchConfig {
            parallel: false,
            max_workers: 4,
            timeout_seconds: 600,
            continue_on_failure: true,
            output_format: "json".to_string(),
        };

        assert!(!config.parallel);
        assert_eq!(config.max_workers, 4);
        assert_eq!(config.timeout_seconds, 600);
        assert!(config.continue_on_failure);
    }

    #[test]
    fn test_batch_executor_creation() {
        let executor = BatchExecutor::new();
        assert!(executor.config.parallel);
    }

    #[test]
    fn test_batch_executor_with_config() {
        let config = BatchConfig {
            parallel: false,
            max_workers: 2,
            ..Default::default()
        };
        let executor = BatchExecutor::with_config(config.clone());
        assert!(!executor.config.parallel);
        assert_eq!(executor.config.max_workers, 2);
    }

    #[test]
    fn test_feature_result_creation() {
        let result = FeatureResult {
            name: "Test Feature".to_string(),
            path: PathBuf::from("test.feature"),
            status: "passed".to_string(),
            scenarios_passed: 5,
            scenarios_failed: 0,
            duration_ms: 100,
            result: None,
        };

        assert_eq!(result.name, "Test Feature");
        assert_eq!(result.status, "passed");
        assert_eq!(result.scenarios_passed, 5);
    }

    #[test]
    fn test_batch_error_creation() {
        let error = BatchError {
            path: PathBuf::from("test.feature"),
            error: "Execution failed".to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
        };

        assert_eq!(error.error, "Execution failed");
    }

    #[test]
    fn test_batch_progress_creation() {
        let progress = BatchProgress::new(10);
        assert_eq!(progress.get_progress(), 0.0);
    }

    #[test]
    fn test_batch_progress_increment() {
        let progress = BatchProgress::new(10);
        progress.increment_completed();
        assert_eq!(progress.get_progress(), 0.1);
    }

    #[test]
    fn test_format_text_output() {
        let config = BatchConfig::default();
        let executor = BatchExecutor::with_config(config);

        let result = BatchResult {
            total_features: 2,
            passed_features: 1,
            failed_features: 1,
            total_scenarios: 10,
            passed_scenarios: 8,
            failed_scenarios: 2,
            total_duration_ms: 500,
            results: vec![
                FeatureResult {
                    name: "Feature 1".to_string(),
                    path: PathBuf::from("feature1.feature"),
                    status: "passed".to_string(),
                    scenarios_passed: 5,
                    scenarios_failed: 0,
                    duration_ms: 200,
                    result: None,
                },
                FeatureResult {
                    name: "Feature 2".to_string(),
                    path: PathBuf::from("feature2.feature"),
                    status: "failed".to_string(),
                    scenarios_passed: 3,
                    scenarios_failed: 2,
                    duration_ms: 300,
                    result: None,
                },
            ],
            errors: vec![],
        };

        let output = executor.format_result(&result, "text");
        eprintln!("ACTUAL OUTPUT:\n{}", output);
        assert!(output.contains("Batch Execution Summary"));
        assert!(output.contains("Features:"));
        assert!(output.contains("2 total, 1 passed, 1 failed"));
        assert!(output.contains("✓ Feature 1"));
        assert!(output.contains("✗ Feature 2"));
    }

    #[test]
    fn test_format_json_output() {
        let config = BatchConfig::default();
        let executor = BatchExecutor::with_config(config);

        let result = BatchResult {
            total_features: 1,
            passed_features: 1,
            failed_features: 0,
            total_scenarios: 5,
            passed_scenarios: 5,
            failed_scenarios: 0,
            total_duration_ms: 100,
            results: vec![FeatureResult {
                name: "Test".to_string(),
                path: PathBuf::from("test.feature"),
                status: "passed".to_string(),
                scenarios_passed: 5,
                scenarios_failed: 0,
                duration_ms: 100,
                result: None,
            }],
            errors: vec![],
        };

        let output = executor.format_result(&result, "json");
        assert!(output.contains("\"total_features\""));
        assert!(output.contains("\"batch_summary\""));
    }

    #[test]
    fn test_batch_execute_single_feature() {
        let mut executor = BatchExecutor::new();
        let paths = vec![PathBuf::from("test.feature")];

        let result = executor.execute(&paths, &|path| {
            Ok(create_mock_result(path.to_str().unwrap(), "passed"))
        });

        assert_eq!(result.total_features, 1);
        assert_eq!(result.passed_features, 1);
        assert_eq!(result.failed_features, 0);
    }

    #[test]
    fn test_batch_execute_multiple_features() {
        let mut executor = BatchExecutor::new();
        let paths = vec![
            PathBuf::from("feature1.feature"),
            PathBuf::from("feature2.feature"),
            PathBuf::from("feature3.feature"),
        ];

        let result = executor.execute(&paths, &|path| {
            if path.to_str().unwrap().contains("feature2") {
                Err("Simulated failure".to_string())
            } else {
                Ok(create_mock_result(path.to_str().unwrap(), "passed"))
            }
        });

        assert_eq!(result.total_features, 3);
        assert_eq!(result.passed_features, 2);
        assert_eq!(result.failed_features, 1);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_discover_features_from_file() {
        let temp_dir = std::env::temp_dir();
        let test_feature = temp_dir.join("test_batch.feature");
        fs::write(&test_feature, "Feature: Test\nScenario: Test").unwrap();

        let paths =
            BatchExecutor::discover_features(test_feature.to_str().unwrap(), "*.feature").unwrap();

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], test_feature);

        let _ = fs::remove_file(test_feature);
    }
}
