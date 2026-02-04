// JSON output formatting
use super::result::ExecutionResult;

pub fn to_json_output(result: &ExecutionResult) -> Result<String, serde_json::Error> {
    serde_json::to_string(result)
}

pub fn to_json_output_pretty(result: &ExecutionResult) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::FeatureInfo;

    #[test]
    fn test_json_output() {
        let feature = FeatureInfo {
            name: "Test".to_string(),
            file: None,
            description: None,
        };
        let mut result = ExecutionResult::new(feature);
        result.update_status();

        let json = to_json_output(&result).expect("Failed to serialize");
        assert!(!json.is_empty());
        assert!(json.contains("status"));
    }

    #[test]
    fn test_json_output_pretty() {
        let feature = FeatureInfo {
            name: "Test".to_string(),
            file: None,
            description: None,
        };
        let result = ExecutionResult::new(feature);
        let json = to_json_output_pretty(&result).expect("Failed to serialize");
        assert!(json.contains("\n")); // Pretty print should have newlines
    }
}
