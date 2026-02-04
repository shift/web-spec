// Validation error types
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub step_number: Option<usize>,
    pub step_text: Option<String>,
    pub error_type: String,
    pub message: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub step_number: Option<usize>,
    pub step_text: Option<String>,
    pub warning_type: String,
    pub message: String,
}

impl ValidationResult {
    pub fn new() -> Self {
        ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.valid = false;
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationError {
    pub fn new(error_type: impl Into<String>, message: impl Into<String>) -> Self {
        ValidationError {
            step_number: None,
            step_text: None,
            error_type: error_type.into(),
            message: message.into(),
            suggestions: Vec::new(),
        }
    }

    pub fn with_step(mut self, step_number: usize, step_text: String) -> Self {
        self.step_number = Some(step_number);
        self.step_text = Some(step_text);
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions.extend(suggestions);
        self
    }
}

impl ValidationWarning {
    pub fn new(warning_type: impl Into<String>, message: impl Into<String>) -> Self {
        ValidationWarning {
            step_number: None,
            step_text: None,
            warning_type: warning_type.into(),
            message: message.into(),
        }
    }

    pub fn with_step(mut self, step_number: usize, step_text: String) -> Self {
        self.step_number = Some(step_number);
        self.step_text = Some(step_text);
        self
    }
}
