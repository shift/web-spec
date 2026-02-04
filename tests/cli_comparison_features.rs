use serde_json::{json, Value};
/// CLI tests for result comparison feature
use std::fs;
use std::process::Command;

fn get_cargo_bin() -> String {
    format!("{}/target/debug/web2markdown", env!("CARGO_MANIFEST_DIR"))
}

/// Create a sample execution result JSON for testing
fn create_sample_result(feature_name: &str, status: &str, duration_ms: u64) -> String {
    let result = json!({
        "status": status,
        "timestamp": "2024-02-04T12:00:00Z",
        "duration_ms": duration_ms,
        "feature": {
            "name": feature_name,
            "file": "test.feature",
            "description": Value::Null
        },
        "scenarios": [
            {
                "name": "Test Scenario",
                "status": status,
                "duration_ms": duration_ms,
                "steps": [
                    {
                        "text": "I do something",
                        "keyword": "Given",
                        "status": status,
                        "duration_ms": duration_ms / 2,
                        "output": Value::Null,
                        "error": Value::Null
                    }
                ]
            }
        ],
        "summary": {
            "total_scenarios": 1,
            "passed_scenarios": if status == "passed" { 1 } else { 0 },
            "failed_scenarios": if status == "failed" { 1 } else { 0 },
            "skipped_scenarios": if status == "skipped" { 1 } else { 0 },
            "total_steps": 1,
            "passed_steps": if status == "passed" { 1 } else { 0 },
            "failed_steps": if status == "failed" { 1 } else { 0 },
            "skipped_steps": if status == "skipped" { 1 } else { 0 }
        }
    });
    result.to_string()
}

#[test]
fn test_compare_identical_results() {
    let bin = get_cargo_bin();
    let baseline_file = "/tmp/baseline_identical.json";
    let current_file = "/tmp/current_identical.json";

    let result = create_sample_result("Test Feature", "passed", 1000);
    fs::write(baseline_file, &result).expect("Failed to write baseline");
    fs::write(current_file, &result).expect("Failed to write current");

    let output = Command::new(&bin)
        .args(&[
            "compare",
            "--baseline",
            baseline_file,
            "--current",
            current_file,
            "--format",
            "text",
        ])
        .output()
        .expect("Failed to run compare command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Test Result Comparison Report"),
        "Output should contain report header"
    );
    assert!(stdout.contains("unchanged"), "Status should be unchanged");

    fs::remove_file(baseline_file).ok();
    fs::remove_file(current_file).ok();
}

#[test]
fn test_compare_regression_detection() {
    let bin = get_cargo_bin();
    let baseline_file = "/tmp/baseline_regression.json";
    let current_file = "/tmp/current_regression.json";

    let baseline = create_sample_result("Test Feature", "passed", 1000);
    let current = create_sample_result("Test Feature", "failed", 1000);

    fs::write(baseline_file, &baseline).expect("Failed to write baseline");
    fs::write(current_file, &current).expect("Failed to write current");

    let output = Command::new(&bin)
        .args(&[
            "compare",
            "--baseline",
            baseline_file,
            "--current",
            current_file,
            "--format",
            "text",
        ])
        .output()
        .expect("Failed to run compare command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("REGRESSION"),
        "Status should contain REGRESSION"
    );
    assert!(
        stdout.contains("Regressions"),
        "Output should contain regressions section"
    );
    assert!(
        stdout.contains("Regressions"),
        "Output should contain regressions section"
    );

    fs::remove_file(baseline_file).ok();
    fs::remove_file(current_file).ok();
}

#[test]
fn test_compare_improvement_detection() {
    let bin = get_cargo_bin();
    let baseline_file = "/tmp/baseline_improvement.json";
    let current_file = "/tmp/current_improvement.json";

    let baseline = create_sample_result("Test Feature", "passed", 2000);
    let current = create_sample_result("Test Feature", "passed", 1000);

    fs::write(baseline_file, &baseline).expect("Failed to write baseline");
    fs::write(current_file, &current).expect("Failed to write current");

    let output = Command::new(&bin)
        .args(&[
            "compare",
            "--baseline",
            baseline_file,
            "--current",
            current_file,
            "--format",
            "text",
        ])
        .output()
        .expect("Failed to run compare command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("IMPROVEMENT"),
        "Status should contain IMPROVEMENT"
    );
    assert!(
        stdout.contains("Improvements"),
        "Output should contain improvements section"
    );

    fs::remove_file(baseline_file).ok();
    fs::remove_file(current_file).ok();
}

#[test]
fn test_compare_json_output_format() {
    let bin = get_cargo_bin();
    let baseline_file = "/tmp/baseline_json.json";
    let current_file = "/tmp/current_json.json";

    let result = create_sample_result("Test Feature", "passed", 1000);
    fs::write(baseline_file, &result).expect("Failed to write baseline");
    fs::write(current_file, &result).expect("Failed to write current");

    let output = Command::new(&bin)
        .args(&[
            "compare",
            "--baseline",
            baseline_file,
            "--current",
            current_file,
            "--format",
            "json",
        ])
        .output()
        .expect("Failed to run compare command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"status\""),
        "JSON output should contain status field"
    );
    assert!(
        stdout.contains("\"summary\""),
        "JSON output should contain summary"
    );

    fs::remove_file(baseline_file).ok();
    fs::remove_file(current_file).ok();
}

#[test]
fn test_compare_yaml_output_format() {
    let bin = get_cargo_bin();
    let baseline_file = "/tmp/baseline_yaml.json";
    let current_file = "/tmp/current_yaml.json";

    let result = create_sample_result("Test Feature", "passed", 1000);
    fs::write(baseline_file, &result).expect("Failed to write baseline");
    fs::write(current_file, &result).expect("Failed to write current");

    let output = Command::new(&bin)
        .args(&[
            "compare",
            "--baseline",
            baseline_file,
            "--current",
            current_file,
            "--format",
            "yaml",
        ])
        .output()
        .expect("Failed to run compare command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("status:"),
        "YAML output should contain status field"
    );
    assert!(
        stdout.contains("summary:"),
        "YAML output should contain summary"
    );

    fs::remove_file(baseline_file).ok();
    fs::remove_file(current_file).ok();
}

#[test]
fn test_compare_to_file_output() {
    let bin = get_cargo_bin();
    let baseline_file = "/tmp/baseline_tofile.json";
    let current_file = "/tmp/current_tofile.json";
    let output_file = "/tmp/comparison_report.txt";

    let result = create_sample_result("Test Feature", "passed", 1000);
    fs::write(baseline_file, &result).expect("Failed to write baseline");
    fs::write(current_file, &result).expect("Failed to write current");

    let _ = fs::remove_file(output_file);

    let output = Command::new(&bin)
        .args(&[
            "compare",
            "--baseline",
            baseline_file,
            "--current",
            current_file,
            "--format",
            "text",
            "--output",
            output_file,
        ])
        .output()
        .expect("Failed to run compare command");

    assert!(output.status.success(), "Command should succeed");
    assert!(
        fs::metadata(output_file).is_ok(),
        "Output file should exist"
    );

    let content = fs::read_to_string(output_file).expect("Failed to read output file");
    assert!(
        content.contains("Test Result Comparison Report"),
        "File should contain report"
    );

    fs::remove_file(baseline_file).ok();
    fs::remove_file(current_file).ok();
    fs::remove_file(output_file).ok();
}

#[test]
fn test_compare_missing_baseline_file() {
    let bin = get_cargo_bin();
    let baseline_file = "/tmp/nonexistent_baseline.json";
    let current_file = "/tmp/current_missing.json";

    let result = create_sample_result("Test Feature", "passed", 1000);
    fs::write(current_file, &result).expect("Failed to write current");

    let output = Command::new(&bin)
        .args(&[
            "compare",
            "--baseline",
            baseline_file,
            "--current",
            current_file,
        ])
        .output()
        .expect("Failed to run compare command");

    assert!(
        !output.status.success(),
        "Command should fail with missing file"
    );

    fs::remove_file(current_file).ok();
}

#[test]
fn test_compare_invalid_json_file() {
    let bin = get_cargo_bin();
    let baseline_file = "/tmp/invalid_baseline.json";
    let current_file = "/tmp/invalid_current.json";

    fs::write(baseline_file, "not valid json").expect("Failed to write baseline");
    fs::write(current_file, "not valid json").expect("Failed to write current");

    let output = Command::new(&bin)
        .args(&[
            "compare",
            "--baseline",
            baseline_file,
            "--current",
            current_file,
        ])
        .output()
        .expect("Failed to run compare command");

    assert!(
        !output.status.success(),
        "Command should fail with invalid JSON"
    );

    fs::remove_file(baseline_file).ok();
    fs::remove_file(current_file).ok();
}

#[test]
fn test_compare_pretty_json_output() {
    let bin = get_cargo_bin();
    let baseline_file = "/tmp/baseline_pretty.json";
    let current_file = "/tmp/current_pretty.json";

    let result = create_sample_result("Test Feature", "passed", 1000);
    fs::write(baseline_file, &result).expect("Failed to write baseline");
    fs::write(current_file, &result).expect("Failed to write current");

    let output = Command::new(&bin)
        .args(&[
            "compare",
            "--baseline",
            baseline_file,
            "--current",
            current_file,
            "--format",
            "json",
            "--pretty",
        ])
        .output()
        .expect("Failed to run compare command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Pretty JSON should have newlines
    assert!(
        stdout.lines().count() > 1,
        "Pretty JSON should be multiline"
    );

    fs::remove_file(baseline_file).ok();
    fs::remove_file(current_file).ok();
}
