//! Output formatting utilities
use std::fs;
use std::path::PathBuf;

/// Format output based on requested format
pub fn format_output(content: String, format: &str, pretty: bool) -> Result<String, String> {
    match format {
        "text" | "txt" => Ok(content),
        "json" => {
            // Simple JSON wrapping for text content
            let json = serde_json::json!({
                "status": "success",
                "data": content
            });

            let result = if pretty {
                serde_json::to_string_pretty(&json)
            } else {
                serde_json::to_string(&json)
            };

            result.map_err(|e| e.to_string())
        }
        "yaml" | "yml" => {
            // Simple YAML wrapping for text content
            let yaml_content = format!(
                "status: success\ndata: |\n{}",
                content
                    .lines()
                    .map(|line| format!("  {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            Ok(yaml_content)
        }
        "tap" => {
            // TAP format is plain text, no wrapping needed
            Ok(content)
        }
        "html" => {
            // HTML format is already complete, no wrapping needed
            Ok(content)
        }
        _ => Err(format!(
            "Unsupported output format: {}. Supported formats: text, json, yaml, tap, html",
            format
        )),
    }
}

/// Write output to file or stdout
pub fn write_output(content: String, output_path: Option<PathBuf>) -> Result<(), String> {
    if let Some(path) = output_path {
        fs::write(&path, content).map_err(|e| format!("Failed to write output file: {}", e))?;
        println!("Output written to: {}", path.display());
    } else {
        println!("{}", content);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_text() {
        let result = format_output("hello world".to_string(), "text", false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn test_format_json() {
        let result = format_output("hello world".to_string(), "json", false);
        assert!(result.is_ok());
        let json_str = result.unwrap();
        assert!(json_str.contains("success"));
        assert!(json_str.contains("hello world"));
    }

    #[test]
    fn test_format_yaml() {
        let result = format_output("hello world".to_string(), "yaml", false);
        assert!(result.is_ok());
        let yaml_str = result.unwrap();
        assert!(yaml_str.contains("status: success"));
        assert!(yaml_str.contains("hello world"));
    }

    #[test]
    fn test_format_yml_alias() {
        let result = format_output("test data".to_string(), "yml", false);
        assert!(result.is_ok());
        let yaml_str = result.unwrap();
        assert!(yaml_str.contains("status: success"));
    }

    #[test]
    fn test_unsupported_format() {
        let result = format_output("test".to_string(), "xml", false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Supported formats"));
    }

    #[test]
    fn test_format_tap() {
        let result = format_output("TAP version 13\n1..1\nok 1 test".to_string(), "tap", false);
        assert!(result.is_ok());
        let tap_str = result.unwrap();
        assert!(tap_str.contains("TAP version 13"));
    }

    #[test]
    fn test_format_html() {
        let html_content = "<!DOCTYPE html>\n<html>\n<body>Test</body>\n</html>".to_string();
        let result = format_output(html_content.clone(), "html", false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("<!DOCTYPE html>"));
        assert_eq!(output, html_content);
    }
}
