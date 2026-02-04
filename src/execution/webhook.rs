// Webhook notification system for test execution results
use crate::execution::result::ExecutionResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub name: String,
    pub events: Vec<WebhookEvent>,
    pub headers: HashMap<String, String>,
    pub retry_count: u32,
    pub timeout_seconds: u64,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        WebhookConfig {
            url: String::new(),
            name: "default".to_string(),
            events: vec![WebhookEvent::Completion, WebhookEvent::Failure],
            headers: HashMap::new(),
            retry_count: 3,
            timeout_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WebhookEvent {
    Start,
    Completion,
    Failure,
    Success,
    Regression,
    Improvement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub event: String,
    pub timestamp: String,
    pub feature: String,
    pub status: String,
    pub summary: ExecutionSummaryPayload,
    pub comparison: Option<ComparisonPayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionSummaryPayload {
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub total_steps: usize,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonPayload {
    pub baseline_status: String,
    pub current_status: String,
    pub regressions: u32,
    pub improvements: u32,
}

#[derive(Debug, Clone)]
pub struct WebhookManager {
    configs: Vec<WebhookConfig>,
    client: reqwest::blocking::Client,
}

impl WebhookManager {
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        WebhookManager {
            configs: Vec::new(),
            client,
        }
    }

    pub fn add_config(&mut self, config: WebhookConfig) {
        self.configs.push(config);
    }

    pub fn from_config_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let configs: Vec<WebhookConfig> = serde_yaml::from_str(&content)?;

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Ok(WebhookManager { configs, client })
    }

    pub fn notify_start(&self, result: &ExecutionResult) -> Vec<Result<(), WebhookError>> {
        let payload = self.create_payload(result, WebhookEvent::Start);
        self.send_to_webhooks(&payload, &WebhookEvent::Start)
    }

    pub fn notify_completion(&self, result: &ExecutionResult) -> Vec<Result<(), WebhookError>> {
        let payload = self.create_payload(result, WebhookEvent::Completion);
        self.send_to_webhooks(&payload, &WebhookEvent::Completion)
    }

    pub fn notify_failure(&self, result: &ExecutionResult) -> Vec<Result<(), WebhookError>> {
        let payload = self.create_payload(result, WebhookEvent::Failure);
        self.send_to_webhooks(&payload, &WebhookEvent::Failure)
    }

    pub fn notify_success(&self, result: &ExecutionResult) -> Vec<Result<(), WebhookError>> {
        let payload = self.create_payload(result, WebhookEvent::Success);
        self.send_to_webhooks(&payload, &WebhookEvent::Success)
    }

    fn create_payload(&self, result: &ExecutionResult, event: WebhookEvent) -> WebhookPayload {
        WebhookPayload {
            event: format!("{:?}", event),
            timestamp: chrono::Local::now().to_rfc3339(),
            feature: result.feature.name.clone(),
            status: result.status.clone(),
            summary: ExecutionSummaryPayload {
                total_scenarios: result.summary.total_scenarios,
                passed_scenarios: result.summary.passed_scenarios,
                failed_scenarios: result.summary.failed_scenarios,
                total_steps: result.summary.total_steps,
                duration_ms: result.duration_ms,
            },
            comparison: None,
        }
    }

    fn send_to_webhooks(
        &self,
        payload: &WebhookPayload,
        event: &WebhookEvent,
    ) -> Vec<Result<(), WebhookError>> {
        let mut results = Vec::new();

        for config in &self.configs {
            if !config.events.contains(event) {
                continue;
            }

            let result = self.send_webhook(config, payload);
            results.push(result);
        }

        results
    }

    fn send_webhook(
        &self,
        config: &WebhookConfig,
        payload: &WebhookPayload,
    ) -> Result<(), WebhookError> {
        let payload_json = serde_json::to_string(payload)
            .map_err(|e| WebhookError::Serialization(e.to_string()))?;

        let mut last_error = None;

        for attempt in 0..config.retry_count {
            // Create request for each attempt
            let mut request = self
                .client
                .post(&config.url)
                .body(payload_json.clone())
                .header("Content-Type", "application/json");

            for (key, value) in &config.headers {
                request = request.header(key, value);
            }

            match request.send() {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        return Ok(());
                    } else {
                        let body = response.text().unwrap_or_default();
                        last_error = Some(WebhookError::HttpError(status.as_u16(), body));
                    }
                }
                Err(e) => {
                    last_error = Some(WebhookError::Request(e.to_string()));
                }
            }

            if attempt < config.retry_count - 1 {
                std::thread::sleep(Duration::from_millis(500 * (attempt + 1) as u64));
            }
        }

        Err(last_error.unwrap_or(WebhookError::Unknown))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("HTTP error: {0} - {1}")]
    HttpError(u16, String),
    #[error("Request error: {0}")]
    Request(String),
    #[error("Unknown error")]
    Unknown,
}

/// Slack-specific webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackWebhookPayload {
    pub text: String,
    pub channel: Option<String>,
    pub username: Option<String>,
    pub icon_emoji: Option<String>,
    pub attachments: Option<Vec<SlackAttachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackAttachment {
    pub color: String,
    pub title: String,
    pub text: String,
    pub fields: Vec<SlackField>,
    pub footer: Option<String>,
    pub ts: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackField {
    pub title: String,
    pub value: String,
    pub short: bool,
}

impl SlackWebhookPayload {
    pub fn from_execution_result(result: &ExecutionResult) -> Self {
        let color = match result.status.as_str() {
            "passed" => "good",
            "failed" => "danger",
            _ => "warning",
        };

        let attachment = SlackAttachment {
            color: color.to_string(),
            title: format!("Test Execution: {}", result.feature.name),
            text: format!(
                "Status: *{}*\nScenarios: {} passed, {} failed",
                result.status, result.summary.passed_scenarios, result.summary.failed_scenarios
            ),
            fields: vec![
                SlackField {
                    title: "Duration".to_string(),
                    value: format!("{}ms", result.duration_ms),
                    short: true,
                },
                SlackField {
                    title: "Scenarios".to_string(),
                    value: format!(
                        "{}/{}",
                        result.summary.passed_scenarios, result.summary.total_scenarios
                    ),
                    short: true,
                },
            ],
            footer: Some("web-spec".to_string()),
            ts: Some(chrono::Local::now().timestamp()),
        };

        SlackWebhookPayload {
            text: format!(
                "Test execution completed: {} - {}",
                result.feature.name,
                if result.status == "passed" {
                    "All tests passed"
                } else {
                    "Some tests failed"
                }
            ),
            channel: None,
            username: Some("web-spec-bot".to_string()),
            icon_emoji: Some(":rocket:".to_string()),
            attachments: Some(vec![attachment]),
        }
    }
}

/// Discord webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordWebhookPayload {
    pub content: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub embeds: Option<Vec<DiscordEmbed>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<i32>,
    pub fields: Option<Vec<DiscordField>>,
    pub footer: Option<DiscordFooter>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

impl DiscordWebhookPayload {
    pub fn from_execution_result(result: &ExecutionResult) -> Self {
        let color = match result.status.as_str() {
            "passed" => 0x00FF00, // Green
            "failed" => 0xFF0000, // Red
            _ => 0xFFFF00,        // Yellow
        };

        let embed = DiscordEmbed {
            title: Some(format!("Test Execution: {}", result.feature.name)),
            description: Some(format!(
                "**Status:** {}",
                if result.status == "passed" {
                    ":white_check_mark: All tests passed"
                } else {
                    ":x: Some tests failed"
                }
            )),
            color: Some(color),
            fields: Some(vec![
                DiscordField {
                    name: "Duration".to_string(),
                    value: format!("{}ms", result.duration_ms),
                    inline: Some(true),
                },
                DiscordField {
                    name: "Scenarios".to_string(),
                    value: format!(
                        "{}/{} passed",
                        result.summary.passed_scenarios, result.summary.total_scenarios
                    ),
                    inline: Some(true),
                },
                DiscordField {
                    name: "Failed".to_string(),
                    value: format!("{}", result.summary.failed_scenarios),
                    inline: Some(true),
                },
            ]),
            footer: Some(DiscordFooter {
                text: "web-spec".to_string(),
                icon_url: None,
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        };

        DiscordWebhookPayload {
            content: None,
            username: Some("web-spec".to_string()),
            avatar_url: None,
            embeds: Some(vec![embed]),
        }
    }
}

/// Microsoft Teams webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsWebhookPayload {
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "themeColor")]
    pub theme_color: String,
    pub summary: String,
    pub sections: Vec<TeamsSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsSection {
    #[serde(rename = "activityTitle")]
    pub activity_title: String,
    #[serde(rename = "activitySubtitle")]
    pub activity_subtitle: String,
    #[serde(rename = "activityImage")]
    pub activity_image: Option<String>,
    pub facts: Vec<TeamsFact>,
    pub markdown: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsFact {
    pub name: String,
    pub value: String,
}

impl TeamsWebhookPayload {
    pub fn from_execution_result(result: &ExecutionResult) -> Self {
        let color = match result.status.as_str() {
            "passed" => "0076D7", // Green-blue
            "failed" => "D13438", // Red
            _ => "FFB900",        // Yellow
        };

        let status_text = if result.status == "passed" {
            "All tests passed"
        } else {
            "Some tests failed"
        };

        let section = TeamsSection {
            activity_title: format!("Test Execution: {}", result.feature.name),
            activity_subtitle: status_text.to_string(),
            activity_image: None,
            facts: vec![
                TeamsFact {
                    name: "Duration".to_string(),
                    value: format!("{}ms", result.duration_ms),
                },
                TeamsFact {
                    name: "Scenarios".to_string(),
                    value: format!(
                        "{}/{}",
                        result.summary.passed_scenarios, result.summary.total_scenarios
                    ),
                },
                TeamsFact {
                    name: "Passed".to_string(),
                    value: format!("{}", result.summary.passed_scenarios),
                },
                TeamsFact {
                    name: "Failed".to_string(),
                    value: format!("{}", result.summary.failed_scenarios),
                },
            ],
            markdown: true,
        };

        TeamsWebhookPayload {
            type_field: "MessageCard".to_string(),
            context: "http://schema.org/extensions".to_string(),
            theme_color: color.to_string(),
            summary: format!("Test Execution: {} - {}", result.feature.name, status_text),
            sections: vec![section],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::result::{ExecutionResult, ExecutionSummary, FeatureInfo};

    fn create_test_result(status: &str) -> ExecutionResult {
        ExecutionResult {
            status: status.to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
            duration_ms: 1000,
            feature: FeatureInfo {
                name: "Test Feature".to_string(),
                file: Some("test.feature".to_string()),
                description: Some("Test description".to_string()),
            },
            scenarios: vec![],
            summary: ExecutionSummary {
                total_scenarios: 5,
                passed_scenarios: 4,
                failed_scenarios: 1,
                skipped_scenarios: 0,
                total_steps: 20,
                passed_steps: 18,
                failed_steps: 2,
                skipped_steps: 0,
            },
        }
    }

    #[test]
    fn test_webhook_manager_creation() {
        let manager = WebhookManager::new();
        assert!(manager.configs.is_empty());
    }

    #[test]
    fn test_webhook_config_defaults() {
        let config = WebhookConfig::default();
        assert!(config.url.is_empty());
        assert_eq!(config.name, "default");
        assert_eq!(config.retry_count, 3);
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn test_webhook_config_custom() {
        let mut config = WebhookConfig::default();
        config.url = "https://hooks.example.com/webhook".to_string();
        config.name = "slack".to_string();
        config.events = vec![WebhookEvent::Completion];
        config.retry_count = 5;

        assert_eq!(config.url, "https://hooks.example.com/webhook");
        assert_eq!(config.name, "slack");
        assert_eq!(config.events, vec![WebhookEvent::Completion]);
        assert_eq!(config.retry_count, 5);
    }

    #[test]
    fn test_slack_payload_from_result() {
        let result = create_test_result("passed");
        let slack = SlackWebhookPayload::from_execution_result(&result);

        assert!(slack.text.contains("All tests passed"));
        assert_eq!(slack.username, Some("web-spec-bot".to_string()));
        assert!(slack.attachments.is_some());
        let attachments = slack.attachments.unwrap();
        assert_eq!(attachments.len(), 1);
        assert_eq!(attachments[0].color, "good");
    }

    #[test]
    fn test_slack_payload_failed() {
        let result = create_test_result("failed");
        let slack = SlackWebhookPayload::from_execution_result(&result);

        assert!(slack.text.contains("Some tests failed"));
        assert_eq!(slack.attachments.unwrap()[0].color, "danger");
    }

    #[test]
    fn test_webhook_event_enum() {
        assert_eq!(WebhookEvent::Start, WebhookEvent::Start);
        assert_eq!(WebhookEvent::Completion, WebhookEvent::Completion);
        assert_ne!(WebhookEvent::Success, WebhookEvent::Failure);
    }

    #[test]
    fn test_execution_payload_creation() {
        let result = create_test_result("passed");
        let manager = WebhookManager::new();

        let payload = manager.create_payload(&result, WebhookEvent::Completion);

        assert_eq!(payload.event, "Completion");
        assert_eq!(payload.feature, "Test Feature");
        assert_eq!(payload.status, "passed");
        assert_eq!(payload.summary.total_scenarios, 5);
        assert_eq!(payload.summary.passed_scenarios, 4);
    }

    #[test]
    fn test_webhook_error_display() {
        let error = WebhookError::HttpError(404, "Not Found".to_string());
        assert!(error.to_string().contains("404"));

        let error2 = WebhookError::Serialization("test".to_string());
        assert!(error2.to_string().contains("Serialization"));
    }
}
