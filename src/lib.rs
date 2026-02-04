pub mod browser;
pub mod automation;
pub mod converter;
pub mod error;
pub mod discovery;
pub mod validation;
pub mod execution;
pub mod cli;

pub use browser::{Browser, BrowserType};
pub use automation::Automation;
pub use converter::Converter;
pub use error::{Result, Web2MarkdownError};
pub use discovery::{StepCatalog, catalog::build_step_catalog};
pub use validation::{validate_feature, ValidationResult};
pub use execution::{ExecutionResult, ExecutionSummary, ScenarioResult, StepResult};



#[derive(Debug, Clone)]
pub struct Web2Markdown {
    browser_type: BrowserType,
}

impl Web2Markdown {
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

impl Default for Web2Markdown {
    fn default() -> Self {
        Self::new()
    }
}
