//! Command handlers for CLI operations
use crate::discovery::catalog::build_step_catalog;
use crate::discovery::search::filter_by_category;
use crate::validation::feature::validate_feature;
use std::path::PathBuf;

/// List all available steps
pub fn handle_list_steps(
    category: Option<String>,
    search: Option<String>,
) -> Result<Vec<String>, String> {
    let catalog = build_step_catalog();
    let all_steps = catalog.all_steps();

    // Filter by category first if provided
    let filtered_refs: Vec<_> = if let Some(cat) = category {
        filter_by_category(all_steps, &cat)
    } else {
        all_steps.iter().collect()
    };

    // Then filter by search term if provided
    let results: Vec<_> = if let Some(q) = search {
        // Search within the filtered results
        let query_lower = q.to_lowercase();
        filtered_refs
            .into_iter()
            .filter(|step| {
                step.id.contains(&query_lower)
                    || step.description.to_lowercase().contains(&query_lower)
                    || step.category.to_lowercase().contains(&query_lower)
                    || step
                        .aliases
                        .iter()
                        .any(|alias| alias.to_lowercase().contains(&query_lower))
                    || step
                        .examples
                        .iter()
                        .any(|example| example.to_lowercase().contains(&query_lower))
            })
            .collect()
    } else {
        filtered_refs
    };

    let mut output = Vec::new();
    for step in results {
        output.push(format!(
            "[{}] {} - {}",
            step.category, step.id, step.description
        ));
    }

    Ok(output)
}

/// Search for steps matching a pattern
pub fn handle_search_steps(query: &str, category: Option<String>) -> Result<Vec<String>, String> {
    let catalog = build_step_catalog();
    let all_steps = catalog.all_steps();

    // Filter by category first if provided
    let filtered_refs: Vec<_> = if let Some(cat) = category {
        filter_by_category(all_steps, &cat)
    } else {
        all_steps.iter().collect()
    };

    // Then search within the filtered results
    let results: Vec<_> = search_steps_in_refs(&filtered_refs, query);

    let mut output = Vec::new();
    for step in results {
        output.push(format!(
            "[{}] {} - {}",
            step.category, step.id, step.description
        ));
        for alias in &step.aliases {
            output.push(format!("  Alias: {}", alias));
        }
    }

    Ok(output)
}

/// Helper function to search within a slice of references
fn search_steps_in_refs<'a>(
    steps: &[&'a crate::discovery::catalog::StepInfo],
    query: &str,
) -> Vec<&'a crate::discovery::catalog::StepInfo> {
    let query_lower = query.to_lowercase();
    steps
        .iter()
        .filter(|step| {
            step.id.contains(&query_lower)
                || step.description.to_lowercase().contains(&query_lower)
                || step.category.to_lowercase().contains(&query_lower)
                || step
                    .aliases
                    .iter()
                    .any(|alias| alias.to_lowercase().contains(&query_lower))
                || step
                    .examples
                    .iter()
                    .any(|example| example.to_lowercase().contains(&query_lower))
        })
        .copied()
        .collect()
}

/// Export step catalog as schema
pub fn handle_export_schema() -> Result<String, String> {
    let catalog = build_step_catalog();
    let schema = crate::discovery::schema::SchemaExport::from_catalog(&catalog);
    serde_json::to_string_pretty(&schema).map_err(|e| format!("Failed to serialize schema: {}", e))
}

/// Validate a feature file
pub fn handle_validate_feature(feature_path: &PathBuf) -> Result<String, String> {
    let path_str = feature_path
        .to_str()
        .ok_or_else(|| "Invalid path".to_string())?;
    let result = validate_feature(path_str)?;

    let mut output = String::new();
    if result.is_valid() {
        output.push_str("✓ Feature file is valid\n");
    } else {
        output.push_str(&format!(
            "✗ Feature file has {} errors:\n",
            result.error_count()
        ));
        for error in &result.errors {
            output.push_str(&format!("  - {}: {}\n", error.error_type, error.message));
            if !error.suggestions.is_empty() {
                output.push_str("    Suggestions:\n");
                for suggestion in &error.suggestions {
                    output.push_str(&format!("      * {}\n", suggestion));
                }
            }
        }
    }

    if !result.warnings.is_empty() {
        output.push_str(&format!("\n{} warning(s):\n", result.warning_count()));
        for warning in &result.warnings {
            output.push_str(&format!(
                "  ⚠ {}: {}\n",
                warning.warning_type, warning.message
            ));
        }
    }

    if result.is_valid() {
        Ok(output)
    } else {
        Err(output)
    }
}

/// Export validation result as JSON
pub fn handle_validate_feature_json(feature_path: &PathBuf) -> Result<String, String> {
    let path_str = feature_path
        .to_str()
        .ok_or_else(|| "Invalid path".to_string())?;
    let result = validate_feature(path_str)?;

    // Create a JSON representation of the validation result
    let json = serde_json::json!({
        "valid": result.is_valid(),
        "file": path_str,
        "error_count": result.error_count(),
        "warning_count": result.warning_count(),
        "errors": result.errors,
        "warnings": result.warnings,
    });

    Ok(serde_json::to_string(&json).map_err(|e| e.to_string())?)
}

/// Export validation result as YAML
pub fn handle_validate_feature_yaml(feature_path: &PathBuf) -> Result<String, String> {
    let path_str = feature_path
        .to_str()
        .ok_or_else(|| "Invalid path".to_string())?;
    let result = validate_feature(path_str)?;

    // Create a YAML representation of the validation result
    let yaml_data = serde_yaml::to_value(&serde_json::json!({
        "valid": result.is_valid(),
        "file": path_str,
        "error_count": result.error_count(),
        "warning_count": result.warning_count(),
        "errors": result.errors,
        "warnings": result.warnings,
    }))
    .map_err(|e| format!("YAML serialization error: {}", e))?;

    serde_yaml::to_string(&yaml_data).map_err(|e| format!("YAML error: {}", e))
}

/// Export validation result as TAP (Test Anything Protocol)
pub fn handle_validate_feature_tap(feature_path: &PathBuf) -> Result<String, String> {
    let path_str = feature_path
        .to_str()
        .ok_or_else(|| "Invalid path".to_string())?;
    let result = validate_feature(path_str)?;

    // Create a TAP representation of the validation result
    // TAP format: version, plan, test results
    let mut tap_output = String::from("TAP version 13\n");

    // In TAP, we represent validation as a single test (validation test)
    tap_output.push_str("1..1\n");

    if result.is_valid() {
        tap_output.push_str("ok 1 - Feature validation passed\n");
    } else {
        tap_output.push_str("not ok 1 - Feature validation failed\n");
        tap_output.push_str("  ---\n");
        tap_output.push_str(&format!("  message: |\n    File: {}\n", path_str));
        tap_output.push_str(&format!("    Errors: {}\n", result.error_count()));
        tap_output.push_str(&format!("    Warnings: {}\n", result.warning_count()));

        if !result.errors.is_empty() {
            tap_output.push_str("    Error details:\n");
            for error in &result.errors {
                tap_output.push_str(&format!("      - {:?}\n", error));
            }
        }

        tap_output.push_str("  ...\n");
    }

    Ok(tap_output)
}

/// Handle validate command with HTML output
pub fn handle_validate_feature_html(feature_path: &PathBuf) -> Result<String, String> {
    let path_str = feature_path
        .to_str()
        .ok_or_else(|| "Invalid path".to_string())?;
    let validation_result = validate_feature(path_str)?;

    // Create an ExecutionResult-like structure for HTML rendering
    // Since validation doesn't execute scenarios, we'll create a simple HTML report
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("  <meta charset=\"UTF-8\">\n");
    html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("  <title>Validation Report - web-spec</title>\n");
    html.push_str("  <style>\n");
    html.push_str("    * { margin: 0; padding: 0; box-sizing: border-box; }\n");
    html.push_str("    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; background-color: #f5f7fa; color: #2c3e50; line-height: 1.6; }\n");
    html.push_str("    .container { max-width: 1200px; margin: 0 auto; padding: 0 20px; }\n");
    html.push_str("    .header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 40px 0; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1); }\n");
    html.push_str("    .header h1 { font-size: 2.5em; margin-bottom: 10px; }\n");
    html.push_str("    main { padding: 40px 0; }\n");
    html.push_str("    .validation-report { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); }\n");
    html.push_str("    .report-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; padding-bottom: 20px; border-bottom: 2px solid #ecf0f1; }\n");
    html.push_str("    .report-header h2 { font-size: 1.8em; }\n");
    html.push_str("    .status-badge { display: inline-block; padding: 6px 12px; border-radius: 20px; font-weight: bold; font-size: 0.9em; }\n");
    html.push_str("    .badge-valid { background-color: #d4edda; color: #155724; }\n");
    html.push_str("    .badge-invalid { background-color: #f8d7da; color: #721c24; }\n");
    html.push_str("    .file-info { background-color: #f8f9fa; border-left: 4px solid #667eea; padding: 15px; margin-bottom: 20px; border-radius: 4px; }\n");
    html.push_str("    .file-label { font-size: 0.85em; color: #7f8c8d; margin-bottom: 4px; }\n");
    html.push_str("    .file-path { font-weight: 600; word-break: break-all; }\n");
    html.push_str("    .errors-section, .warnings-section { margin-top: 20px; }\n");
    html.push_str(
        "    .errors-section h3, .warnings-section h3 { font-size: 1.2em; margin-bottom: 12px; }\n",
    );
    html.push_str("    .error-list, .warning-list { list-style: none; padding: 0; }\n");
    html.push_str("    .error-item, .warning-item { padding: 12px; margin-bottom: 10px; border-left: 4px solid #e74c3c; background-color: #fef2f2; border-radius: 4px; }\n");
    html.push_str("    .warning-item { border-left-color: #f39c12; background-color: #fffbf0; }\n");
    html.push_str("    .error-message, .warning-message { font-weight: 600; color: #2c3e50; margin-bottom: 6px; }\n");
    html.push_str("    .error-text, .warning-text { font-size: 0.9em; color: #555; }\n");
    html.push_str("    .summary-stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-top: 20px; }\n");
    html.push_str("    .stat-card { background-color: #f8f9fa; padding: 15px; border-radius: 4px; border-left: 4px solid #667eea; }\n");
    html.push_str("    .stat-label { font-size: 0.85em; color: #7f8c8d; margin-bottom: 6px; }\n");
    html.push_str("    .stat-value { font-size: 1.8em; font-weight: bold; }\n");
    html.push_str("    .stat-value.valid { color: #27ae60; }\n");
    html.push_str("    .stat-value.invalid { color: #e74c3c; }\n");
    html.push_str("    .footer { background-color: #2c3e50; color: #ecf0f1; text-align: center; padding: 20px 0; margin-top: 40px; }\n");
    html.push_str("  </style>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");

    html.push_str("  <header class=\"header\">\n");
    html.push_str("    <div class=\"container\">\n");
    html.push_str("      <h1>Feature Validation Report</h1>\n");
    html.push_str("    </div>\n");
    html.push_str("  </header>\n");

    html.push_str("  <main class=\"container\">\n");
    html.push_str("    <div class=\"validation-report\">\n");

    // Header
    let valid_class = if validation_result.is_valid() {
        "badge-valid"
    } else {
        "badge-invalid"
    };
    let status_text = if validation_result.is_valid() {
        "VALID"
    } else {
        "INVALID"
    };
    html.push_str("      <div class=\"report-header\">\n");
    html.push_str("        <h2>Validation Result</h2>\n");
    html.push_str(&format!(
        "        <span class=\"status-badge {}\">{}</span>\n",
        valid_class, status_text
    ));
    html.push_str("      </div>\n");

    // File info
    html.push_str("      <div class=\"file-info\">\n");
    html.push_str("        <div class=\"file-label\">FILE</div>\n");
    html.push_str(&format!(
        "        <div class=\"file-path\">{}</div>\n",
        escape_html_for_attr(path_str)
    ));
    html.push_str("      </div>\n");

    // Summary stats
    html.push_str("      <div class=\"summary-stats\">\n");
    let status_class = if validation_result.is_valid() {
        "valid"
    } else {
        "invalid"
    };
    html.push_str(&format!("        <div class=\"stat-card\">\n"));
    html.push_str("          <div class=\"stat-label\">VALIDATION STATUS</div>\n");
    html.push_str(&format!(
        "          <div class=\"stat-value {}\">{}</div>\n",
        status_class, status_text
    ));
    html.push_str("        </div>\n");
    html.push_str("        <div class=\"stat-card\">\n");
    html.push_str("          <div class=\"stat-label\">ERRORS</div>\n");
    html.push_str(&format!(
        "          <div class=\"stat-value invalid\">{}</div>\n",
        validation_result.error_count()
    ));
    html.push_str("        </div>\n");
    html.push_str("        <div class=\"stat-card\">\n");
    html.push_str("          <div class=\"stat-label\">WARNINGS</div>\n");
    html.push_str(&format!(
        "          <div class=\"stat-value\">{}$</div>\n",
        validation_result.warning_count()
    ));
    html.push_str("        </div>\n");
    html.push_str("      </div>\n");

    // Errors
    if !validation_result.errors.is_empty() {
        html.push_str("      <div class=\"errors-section\">\n");
        html.push_str("        <h3>Errors</h3>\n");
        html.push_str("        <ul class=\"error-list\">\n");
        for error in &validation_result.errors {
            html.push_str("          <li class=\"error-item\">\n");
            html.push_str(&format!(
                "            <div class=\"error-message\">{}</div>\n",
                escape_html_for_attr(&error.message)
            ));
            html.push_str(&format!(
                "            <div class=\"error-text\">{}</div>\n",
                escape_html_for_attr(&error.error_type)
            ));
            html.push_str("          </li>\n");
        }
        html.push_str("        </ul>\n");
        html.push_str("      </div>\n");
    }

    // Warnings
    if !validation_result.warnings.is_empty() {
        html.push_str("      <div class=\"warnings-section\">\n");
        html.push_str("        <h3>Warnings</h3>\n");
        html.push_str("        <ul class=\"warning-list\">\n");
        for warning in &validation_result.warnings {
            html.push_str("          <li class=\"warning-item\">\n");
            html.push_str(&format!(
                "            <div class=\"warning-message\">{}</div>\n",
                escape_html_for_attr(&warning.message)
            ));
            html.push_str("          </li>\n");
        }
        html.push_str("        </ul>\n");
        html.push_str("      </div>\n");
    }

    html.push_str("    </div>\n");
    html.push_str("  </main>\n");

    html.push_str("  <footer class=\"footer\">\n");
    html.push_str("    <div class=\"container\">\n");
    html.push_str("      <p>Generated by web-spec | Test Anything Protocol</p>\n");
    html.push_str("    </div>\n");
    html.push_str("  </footer>\n");

    html.push_str("</body>\n");
    html.push_str("</html>\n");

    Ok(html)
}

/// Escape HTML characters for safe display
fn escape_html_for_attr(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Handle compare command
pub fn handle_compare_results(
    baseline_path: &PathBuf,
    current_path: &PathBuf,
) -> Result<String, String> {
    use std::fs;

    // Read baseline result
    let baseline_json = fs::read_to_string(baseline_path)
        .map_err(|e| format!("Failed to read baseline file: {}", e))?;

    let baseline: crate::execution::ExecutionResult = serde_json::from_str(&baseline_json)
        .map_err(|e| format!("Failed to parse baseline JSON: {}", e))?;

    // Read current result
    let current_json = fs::read_to_string(current_path)
        .map_err(|e| format!("Failed to read current file: {}", e))?;

    let current: crate::execution::ExecutionResult = serde_json::from_str(&current_json)
        .map_err(|e| format!("Failed to parse current JSON: {}", e))?;

    // Compare results
    let comparison = crate::execution::compare_results(&baseline, &current);

    // Return as JSON
    serde_json::to_string(&comparison).map_err(|e| format!("Failed to serialize comparison: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_steps() {
        let result = handle_list_steps(None, None);
        assert!(result.is_ok());
        let steps = result.unwrap();
        assert!(!steps.is_empty());
    }

    #[test]
    fn test_search_steps() {
        let result = handle_search_steps("click", None);
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_export_schema() {
        let result = handle_export_schema();
        assert!(result.is_ok());
        let schema_json = result.unwrap();
        assert!(!schema_json.is_empty());
        // Verify it's valid JSON
        assert!(schema_json.contains("\"steps\""));
    }
}
