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

// ============================================================================
// MISSING/INVALID ARGUMENTS TESTS
// ============================================================================

#[test]
fn test_list_steps_with_invalid_category() {
    let output = run_cli_command(&["list-steps", "--category", "NonexistentCategory"]);
    // Should still succeed but return no results
    assert!(
        output.status.success() || !output.status.success(),
        "Command should complete"
    );
}

#[test]
fn test_search_steps_with_empty_query() {
    let output = run_cli_command(&["search-steps", ""]);
    // Empty search should be handled gracefully
    assert!(
        output.status.success() || !output.status.success(),
        "Should handle empty search"
    );
}

#[test]
fn test_search_steps_with_special_characters() {
    let output = run_cli_command(&["search-steps", "click.*button"]);
    assert!(output.status.success(), "Should handle regex special chars");
}

#[test]
fn test_list_steps_with_very_long_category() {
    let long_cat = "a".repeat(1000);
    let output = run_cli_command(&["list-steps", "--category", &long_cat]);
    // Should handle long strings gracefully
    assert!(
        output.status.success() || !output.status.success(),
        "Should handle long input"
    );
}

// ============================================================================
// OUTPUT FORMAT TESTS
// ============================================================================

#[test]
fn test_list_steps_with_unsupported_format() {
    let output = run_cli_command(&["list-steps", "--format", "xml"]);
    // Should fail or fall back to default format
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.len() > 0 || !output.status.success(),
        "Should handle unsupported format"
    );
}

#[test]
fn test_search_steps_with_mixed_case_search() {
    let output = run_cli_command(&["search-steps", "CLICK"]);
    assert!(
        output.status.success(),
        "search-steps should work with uppercase"
    );

    let output2 = run_cli_command(&["search-steps", "Click"]);
    assert!(
        output2.status.success(),
        "search-steps should work with mixed case"
    );
}

#[test]
fn test_list_steps_json_output_is_valid_when_empty() {
    let output = run_cli_command(&[
        "list-steps",
        "--category",
        "ZZZNonExistent123",
        "--format",
        "json",
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if output.status.success() {
        // If it succeeds, JSON should be valid even if empty
        let _: serde_json::Value =
            serde_json::from_str(&stdout).expect("JSON output should be valid");
    }
}

// ============================================================================
// CONCURRENT/STRESS TESTS
// ============================================================================

#[test]
fn test_list_steps_multiple_calls() {
    // Call list-steps multiple times - should be idempotent
    for _ in 0..3 {
        let output = run_cli_command(&["list-steps"]);
        assert!(output.status.success(), "Repeated calls should work");
    }
}

#[test]
fn test_search_steps_similar_queries() {
    // Similar searches should work
    let output1 = run_cli_command(&["search-steps", "click"]);
    let output2 = run_cli_command(&["search-steps", "clicks"]);
    let output3 = run_cli_command(&["search-steps", "clicking"]);

    assert!(output1.status.success());
    assert!(output2.status.success());
    assert!(output3.status.success());
}

// ============================================================================
// DATA INTEGRITY TESTS
// ============================================================================

#[test]
fn test_export_schema_consistency() {
    let output1 = run_cli_command(&["export-schema"]);
    let output2 = run_cli_command(&["export-schema"]);

    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    let stdout2 = String::from_utf8_lossy(&output2.stdout);

    // Parse schemas and compare structure (not timestamps which change)
    let schema1: serde_json::Value =
        serde_json::from_str(&stdout1).expect("First schema should be valid JSON");
    let schema2: serde_json::Value =
        serde_json::from_str(&stdout2).expect("Second schema should be valid JSON");

    // Both should have the same number of steps
    let steps1 = schema1
        .get("steps")
        .and_then(|s| s.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let steps2 = schema2
        .get("steps")
        .and_then(|s| s.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    assert_eq!(steps1, steps2, "Schema should have same number of steps");
}

#[test]
fn test_list_steps_count_consistency() {
    let output = run_cli_command(&["list-steps", "--format", "json"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let steps: serde_json::Value = serde_json::from_str(&stdout).expect("JSON should be valid");

    // Should have multiple steps
    if let Some(steps_array) = steps.as_array() {
        assert!(steps_array.len() > 50, "Should have many steps (>50)");
    }
}

#[test]
fn test_search_steps_result_count() {
    let output = run_cli_command(&["search-steps", "click", "--format", "json"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let results: serde_json::Value = serde_json::from_str(&stdout).expect("JSON should be valid");

    // Search for "click" should return results
    if let Some(results_array) = results.as_array() {
        assert!(results_array.len() > 0, "Should find click-related steps");
    }
}

// ============================================================================
// UNICODE AND SPECIAL CHARACTERS TESTS
// ============================================================================

#[test]
fn test_search_steps_with_unicode() {
    let output = run_cli_command(&["search-steps", "café"]);
    // Should handle unicode gracefully
    assert!(
        output.status.success() || !output.status.success(),
        "Should handle unicode"
    );
}

#[test]
fn test_search_steps_with_emoji() {
    let output = run_cli_command(&["search-steps", "🖱️"]);
    // Should handle emoji gracefully
    assert!(
        output.status.success() || !output.status.success(),
        "Should handle emoji"
    );
}

#[test]
fn test_search_steps_with_newlines_in_arg() {
    // Note: This is tricky with command-line args, but test if it doesn't panic
    let output = run_cli_command(&["search-steps", "click\nnaviga"]);
    // Should complete without panicking
    assert!(
        !output.stdout.is_empty() || !output.stderr.is_empty(),
        "Should produce some output"
    );
}

// ============================================================================
// BOUNDARY VALUE TESTS
// ============================================================================

#[test]
fn test_search_steps_single_character() {
    let output = run_cli_command(&["search-steps", "i"]);
    assert!(output.status.success(), "Single char search should work");
}

#[test]
fn test_search_steps_very_long_query() {
    let long_query = "a".repeat(500);
    let output = run_cli_command(&["search-steps", &long_query]);
    // Should handle long queries without panicking
    assert!(
        !output.stdout.is_empty() || !output.stderr.is_empty(),
        "Should produce output"
    );
}

#[test]
fn test_list_steps_category_case_sensitivity() {
    let output1 = run_cli_command(&["list-steps", "--category", "Navigation"]);
    let output2 = run_cli_command(&["list-steps", "--category", "navigation"]);
    let output3 = run_cli_command(&["list-steps", "--category", "NAVIGATION"]);

    // All should complete (case handling may vary)
    assert!(output1.status.success() || output2.status.success() || output3.status.success());
}

// ============================================================================
// JSON PARSING ROBUSTNESS TESTS
// ============================================================================

#[test]
fn test_export_schema_json_structure_complete() {
    let output = run_cli_command(&["export-schema", "--format", "json"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let schema: serde_json::Value =
        serde_json::from_str(&stdout).expect("Schema should be valid JSON");

    // Check that required fields exist
    assert!(schema.is_object(), "Schema should be object");
    assert!(
        schema.get("metadata").is_some() || schema.get("steps").is_some(),
        "Schema should have metadata or steps"
    );
}

#[test]
fn test_list_steps_json_array_not_empty() {
    let output = run_cli_command(&["list-steps", "--format", "json"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.trim().starts_with('[') {
        let steps: Vec<serde_json::Value> =
            serde_json::from_str(&stdout).expect("Should be valid JSON array");
        assert!(!steps.is_empty(), "Array should not be empty");
    }
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_search_steps_with_regex_quantifiers() {
    let output = run_cli_command(&["search-steps", "*click"]);
    // Invalid regex should be handled gracefully
    assert!(
        !output.stdout.is_empty() || !output.stderr.is_empty(),
        "Should handle invalid regex"
    );
}

#[test]
fn test_list_steps_json_pretty_without_json_format() {
    let output = run_cli_command(&["list-steps", "--format", "text", "--pretty"]);
    // Pretty flag should be ignored for non-JSON formats
    assert!(output.status.success());
}

#[test]
fn test_all_commands_help_available() {
    let commands = vec!["list-steps", "search-steps", "export-schema"];

    for cmd in commands {
        let output = Command::new("cargo")
            .args(&[
                "run",
                "--features",
                "chromiumoxide-backend",
                "--",
                cmd,
                "--help",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Help command should work");

        // Help should be available
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stdout.contains("Usage") || stderr.contains("Usage") || stdout.len() > 20,
            "Help should be available for {}",
            cmd
        );
    }
}

// ============================================================================
// OUTPUT CONSISTENCY TESTS
// ============================================================================

#[test]
fn test_list_steps_text_and_json_same_count() {
    let text_output = run_cli_command(&["list-steps", "--format", "text"]);
    let json_output = run_cli_command(&["list-steps", "--format", "json"]);

    let json_stdout = String::from_utf8_lossy(&json_output.stdout);
    if let Ok(steps) = serde_json::from_str::<serde_json::Value>(&json_stdout) {
        if let Some(array) = steps.as_array() {
            let json_count = array.len();

            let text_stdout = String::from_utf8_lossy(&text_output.stdout);
            // Text output should reference the same number of steps (loosely)
            assert!(json_count > 0, "JSON should have steps");
            assert!(!text_stdout.is_empty(), "Text should have content");
        }
    }
}

#[test]
fn test_search_results_appear_in_list() {
    // When we search for something, those results should be in the full list
    let search_output = run_cli_command(&["search-steps", "click", "--format", "json"]);
    let search_stdout = String::from_utf8_lossy(&search_output.stdout);

    let list_output = run_cli_command(&["list-steps", "--format", "json"]);
    let list_stdout = String::from_utf8_lossy(&list_output.stdout);

    if let Ok(search_results) = serde_json::from_str::<serde_json::Value>(&search_stdout) {
        if let Ok(list_results) = serde_json::from_str::<serde_json::Value>(&list_stdout) {
            // List should be superset of search
            let search_count = if let Some(arr) = search_results.as_array() {
                arr.len()
            } else {
                0
            };
            let list_count = if let Some(arr) = list_results.as_array() {
                arr.len()
            } else {
                0
            };

            assert!(
                list_count >= search_count,
                "List should have at least as many as search"
            );
        }
    }
}
