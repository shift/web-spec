use std::process::Command;

/// Run CLI command and capture output
fn run_cli_command(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .args(&["run", "--features", "chromiumoxide-backend", "--"])
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute CLI command")
}

#[test]
fn test_list_steps_command() {
    let output = run_cli_command(&["list-steps"]);
    assert!(output.status.success(), "list-steps command failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.len() > 50, "Output should contain step information");
}

#[test]
fn test_list_steps_with_category_filter() {
    let output = run_cli_command(&["list-steps", "--category", "Navigation"]);
    assert!(
        output.status.success(),
        "list-steps with category filter failed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.len() > 0, "Output should contain results");
}

#[test]
fn test_search_steps_command() {
    let output = run_cli_command(&["search-steps", "click"]);
    assert!(output.status.success(), "search-steps command failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.len() > 0, "Output should contain search results");
}

#[test]
fn test_export_schema_command() {
    let output = run_cli_command(&["export-schema"]);
    assert!(output.status.success(), "export-schema command failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should be valid JSON
    let _: serde_json::Value = serde_json::from_str(&stdout).expect("Output should be valid JSON");
}

#[test]
fn test_export_schema_json_format() {
    let output = run_cli_command(&["export-schema", "--format", "json"]);
    assert!(
        output.status.success(),
        "export-schema with JSON format failed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let schema: serde_json::Value =
        serde_json::from_str(&stdout).expect("Output should be valid JSON");

    assert!(
        schema.get("metadata").is_some(),
        "Schema should have metadata field"
    );
    assert!(
        schema.get("steps").is_some(),
        "Schema should have steps field"
    );
}

#[test]
fn test_list_steps_json_output() {
    let output = run_cli_command(&["list-steps", "--format", "json"]);
    assert!(
        output.status.success(),
        "list-steps with JSON format failed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: serde_json::Value =
        serde_json::from_str(&stdout).expect("Output should be valid JSON");

    assert!(
        result.is_array() || result.is_object(),
        "JSON output should be array or object"
    );
}

#[test]
fn test_search_steps_json_output() {
    let output = run_cli_command(&["search-steps", "navigate", "--format", "json"]);
    assert!(
        output.status.success(),
        "search-steps with JSON format failed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: serde_json::Value =
        serde_json::from_str(&stdout).expect("Output should be valid JSON");

    assert!(
        result.is_array() || result.is_object(),
        "JSON output should be array or object"
    );
}
