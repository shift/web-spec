pub mod automation;
pub mod browser;
pub mod cli;
pub mod converter;
pub mod discovery;
pub mod error;
pub mod execution;
pub mod validation;

pub use automation::Automation;
pub use browser::{Browser, BrowserType};
pub use converter::Converter;
pub use discovery::{StepCatalog, catalog::build_step_catalog};
pub use error::{Result, WebSpecError};
pub use execution::{ExecutionResult, ExecutionSummary, ScenarioResult, StepResult};
pub use validation::{ValidationResult, validate_feature};

#[derive(Debug, Clone)]
pub struct WebSpec {
    browser_type: BrowserType,
}

impl WebSpec {
    pub fn new() -> Self {
        Self {
            browser_type: BrowserType::WebDriver,
        }
    }

    pub fn with_browser(browser_type: BrowserType) -> Self {
        Self { browser_type }
    }

    pub async fn from_url(&self, url: &str) -> Result<String> {
        let mut browser = Browser::new(self.browser_type.clone()).await?;
        browser.navigate_to(url).await?;
        browser.wait_for_load().await?;

        let html = browser.get_html().await?;

        let converter = Converter::new();
        let markdown = converter.convert(&html)?;

        Ok(markdown)
    }
}

impl Default for WebSpec {
    fn default() -> Self {
        Self::new()
    }
}
