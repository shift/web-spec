// YAML output formatting
use super::result::ExecutionResult;

pub fn to_yaml_output(result: &ExecutionResult) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::FeatureInfo;

    #[test]
    fn test_yaml_output() {
        let feature = FeatureInfo {
            name: "Test".to_string(),
            file: None,
            description: None,
        };
        let mut result = ExecutionResult::new(feature);
        result.update_status();

        let yaml = to_yaml_output(&result).expect("Failed to serialize");
        assert!(!yaml.is_empty());
        assert!(yaml.contains("status"));
    }

    #[test]
    fn test_yaml_output_format() {
        let feature = FeatureInfo {
            name: "Test YAML".to_string(),
            file: Some("test.feature".to_string()),
            description: Some("A test feature".to_string()),
        };
        let result = ExecutionResult::new(feature);
        let yaml = to_yaml_output(&result).expect("Failed to serialize");

        // YAML should contain proper formatting
        assert!(yaml.contains("---") || yaml.contains("name:"));
        assert!(yaml.contains("Test YAML"));
    }
}
