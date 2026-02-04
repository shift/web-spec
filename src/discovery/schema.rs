// Schema export functionality
use super::catalog::StepCatalog;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaExport {
    pub metadata: SchemaMetadata,
    pub categories: Vec<String>,
    pub steps: Vec<ExportedStepInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub version: String,
    pub generated_at: String,
    pub total_steps: usize,
    pub total_categories: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedStepInfo {
    pub id: String,
    pub pattern: String,
    pub aliases: Vec<String>,
    pub category: String,
    pub description: String,
    pub parameters: Vec<ExportedParameterInfo>,
    pub examples: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedParameterInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub required: bool,
    pub description: String,
}

impl SchemaExport {
    pub fn from_catalog(catalog: &StepCatalog) -> Self {
        let steps = catalog
            .all_steps()
            .iter()
            .map(|step| ExportedStepInfo {
                id: step.id.clone(),
                pattern: step.pattern.clone(),
                aliases: step.aliases.clone(),
                category: step.category.clone(),
                description: step.description.clone(),
                parameters: step
                    .parameters
                    .iter()
                    .map(|p| ExportedParameterInfo {
                        name: p.name.clone(),
                        param_type: p.param_type.clone(),
                        required: p.required,
                        description: p.description.clone(),
                    })
                    .collect(),
                examples: step.examples.clone(),
            })
            .collect();

        SchemaExport {
            metadata: SchemaMetadata {
                version: "0.1.0".to_string(),
                generated_at: chrono::Local::now()
                    .format("%Y-%m-%dT%H:%M:%SZ")
                    .to_string(),
                total_steps: catalog.total_steps(),
                total_categories: catalog.categories.len(),
            },
            categories: catalog.categories.clone(),
            steps,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_export() {
        let catalog = crate::discovery::catalog::build_step_catalog();
        let schema = SchemaExport::from_catalog(&catalog);
        assert!(!schema.steps.is_empty());
        assert!(!schema.categories.is_empty());
    }

    #[test]
    fn test_schema_to_json() {
        let catalog = crate::discovery::catalog::build_step_catalog();
        let schema = SchemaExport::from_catalog(&catalog);
        let json = schema.to_json().expect("Failed to serialize");
        assert!(!json.is_empty());
        assert!(json.contains("metadata"));
    }
}
