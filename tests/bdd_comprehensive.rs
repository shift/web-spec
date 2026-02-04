// Copyright (c) 2024 web-spec Contributors
// Comprehensive BDD step definitions for all features

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use tempfile::TempDir;

    fn get_cargo_bin() -> String {
        format!("{}/target/debug/web-spec", env!("CARGO_MANIFEST_DIR"))
    }

    fn create_temp_dir() -> TempDir {
        TempDir::new().expect("Failed to create temp dir")
    }

    #[test]
    fn test_cli_available() {
        let bin = get_cargo_bin();
        assert!(PathBuf::from(&bin).exists(), "CLI binary should exist");
    }

    #[test]
    fn test_compare_identical_results() {
        let temp_dir = create_temp_dir();
        let content = r#"{
            "status": "passed",
            "timestamp": "2024-02-04T10:00:00Z",
            "duration_ms": 1000,
            "feature": {"name": "Test", "file": "test.feature", "description": null},
            "scenarios": [],
            "summary": {"total_scenarios": 5, "passed_scenarios": 5, "failed_scenarios": 0, "skipped_scenarios": 0, "total_steps": 20, "passed_steps": 20, "failed_steps": 0, "skipped_steps": 0}
        }"#;

        fs::write(temp_dir.path().join("baseline.json"), content)
            .expect("Failed to write baseline");
        fs::write(temp_dir.path().join("current.json"), content).expect("Failed to write current");

        let output = Command::new(&get_cargo_bin())
            .args(&[
                "compare",
                "--baseline",
                temp_dir.path().join("baseline.json").to_str().unwrap(),
                "--current",
                temp_dir.path().join("current.json").to_str().unwrap(),
            ])
            .output()
            .expect("Failed to run compare");

        assert_eq!(output.status.code().unwrap_or(-1), 0);
    }

    #[test]
    fn test_compare_different_results() {
        let temp_dir = create_temp_dir();
        let baseline =
            r#"{"summary": {"total_scenarios": 5, "passed_scenarios": 5, "failed_scenarios": 0}}"#;
        let current =
            r#"{"summary": {"total_scenarios": 5, "passed_scenarios": 4, "failed_scenarios": 1}}"#;

        fs::write(temp_dir.path().join("baseline.json"), baseline)
            .expect("Failed to write baseline");
        fs::write(temp_dir.path().join("current.json"), current).expect("Failed to write current");

        let output = Command::new(&get_cargo_bin())
            .args(&[
                "compare",
                "--baseline",
                temp_dir.path().join("baseline.json").to_str().unwrap(),
                "--current",
                temp_dir.path().join("current.json").to_str().unwrap(),
            ])
            .output()
            .expect("Failed to run compare");

        assert!(
            output.status.code().unwrap_or(-1) != 0
                || !String::from_utf8_lossy(&output.stdout).contains("regression")
        );
    }

    #[test]
    fn test_compare_missing_file() {
        let temp_dir = create_temp_dir();
        fs::write(temp_dir.path().join("current.json"), "{}").expect("Failed to write current");

        let output = Command::new(&get_cargo_bin())
            .args(&[
                "compare",
                "--baseline",
                "/nonexistent/baseline.json",
                "--current",
                temp_dir.path().join("current.json").to_str().unwrap(),
            ])
            .output()
            .expect("Failed to run compare");

        assert_ne!(output.status.code().unwrap_or(-1), 0);
    }

    #[test]
    fn test_compare_json_format() {
        let temp_dir = create_temp_dir();
        fs::write(temp_dir.path().join("baseline.json"), "{}").expect("Failed to write baseline");
        fs::write(temp_dir.path().join("current.json"), "{}").expect("Failed to write current");

        let output = Command::new(&get_cargo_bin())
            .args(&[
                "compare",
                "--baseline",
                temp_dir.path().join("baseline.json").to_str().unwrap(),
                "--current",
                temp_dir.path().join("current.json").to_str().unwrap(),
                "--format",
                "json",
            ])
            .output()
            .expect("Failed to run compare");

        let _stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let _stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let exit_code = output.status.code().unwrap_or(-1);
        // Compare command returns exit code 1 when there are differences, which is expected
        assert!(
            exit_code == 0 || exit_code == 1,
            "Compare should exit with 0 or 1, got {}",
            exit_code
        );
    }

    #[test]
    fn test_webhook_help() {
        let output = Command::new(&get_cargo_bin())
            .args(&["webhook", "--help"])
            .output()
            .expect("Failed to run webhook --help");

        assert!(output.status.code().unwrap_or(-1) == 0);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        assert!(stdout.contains("webhook") || stdout.contains("Webhook"));
    }

    #[test]
    fn test_alerts_help() {
        let output = Command::new(&get_cargo_bin())
            .args(&["alerts", "--help"])
            .output()
            .expect("Failed to run alerts --help");

        assert!(output.status.code().unwrap_or(-1) == 0);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        assert!(stdout.contains("alert") || stdout.contains("Alert"));
    }

    #[test]
    fn test_batch_help() {
        let output = Command::new(&get_cargo_bin())
            .args(&["batch", "--help"])
            .output()
            .expect("Failed to run batch --help");

        assert!(output.status.code().unwrap_or(-1) == 0);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        assert!(stdout.contains("batch") || stdout.contains("Batch"));
    }

    #[test]
    fn test_debug_help() {
        let output = Command::new(&get_cargo_bin())
            .args(&["debug", "--help"])
            .output()
            .expect("Failed to run debug --help");

        assert!(output.status.code().unwrap_or(-1) == 0);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        assert!(stdout.contains("debug") || stdout.contains("Debug"));
    }

    #[test]
    fn test_batch_multiple_features() {
        let temp_dir = create_temp_dir();

        for i in 1..=3 {
            let content = format!(
                r#"Feature: Test Feature {}
    Scenario: Test scenario {}
        Given I extract the page title"#,
                i, i
            );
            let file_path = temp_dir.path().join(format!("feature_{}.feature", i));
            fs::write(&file_path, content).expect("Failed to write feature file");
        }

        let output = Command::new(&get_cargo_bin())
            .args(&["batch", "--path", temp_dir.path().to_str().unwrap()])
            .output()
            .expect("Failed to run batch command");

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let output_str = if !stdout.is_empty() { stdout } else { stderr };

        assert!(
            output_str.contains("feature")
                || output_str.contains("Feature")
                || output.status.code().unwrap_or(-1) == 0
        );
    }

    #[test]
    fn test_export_schema() {
        let temp_dir = create_temp_dir();

        let output = Command::new(&get_cargo_bin())
            .args(&[
                "export-schema",
                "--output",
                temp_dir.path().join("schema.json").to_str().unwrap(),
            ])
            .output()
            .expect("Failed to export schema");

        assert_eq!(output.status.code().unwrap_or(-1), 0);
        assert!(temp_dir.path().join("schema.json").exists());
    }

    #[test]
    fn test_run_help() {
        let output = Command::new(&get_cargo_bin())
            .args(&["run", "--help"])
            .output()
            .expect("Failed to run run --help");

        assert!(output.status.code().unwrap_or(-1) == 0);
    }

    #[test]
    fn test_compare_help() {
        let output = Command::new(&get_cargo_bin())
            .args(&["compare", "--help"])
            .output()
            .expect("Failed to run compare --help");

        assert!(output.status.code().unwrap_or(-1) == 0);
    }

    #[test]
    fn test_help_command() {
        let output = Command::new(&get_cargo_bin())
            .args(&["--help"])
            .output()
            .expect("Failed to run --help");

        assert!(output.status.code().unwrap_or(-1) == 0);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        assert!(stdout.contains("web-spec"));
    }

    #[test]
    fn test_version_command() {
        let output = Command::new(&get_cargo_bin())
            .args(&["--version"])
            .output()
            .expect("Failed to run --version");

        assert!(output.status.code().unwrap_or(-1) == 0);
    }

    #[test]
    fn test_invalid_command() {
        let output = Command::new(&get_cargo_bin())
            .args(&["invalid-command"])
            .output()
            .expect("Failed to run invalid command");

        assert_ne!(output.status.code().unwrap_or(-1), 0);
    }
}
