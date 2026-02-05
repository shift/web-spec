use crate::error::{Result, WebSpecError};
use thirtyfour::prelude::*;

#[cfg(feature = "chromiumoxide-backend")]
use chromiumoxide::{Browser as ChromiumBrowser, BrowserConfig, Page};
#[cfg(feature = "chromiumoxide-backend")]
use chromiumoxide::browser::HeadlessMode;
#[cfg(feature = "chromiumoxide-backend")]
use futures_util::StreamExt;

#[derive(Debug, Clone)]
pub enum BrowserType {
    WebDriver,
    #[cfg(feature = "chromiumoxide-backend")]
    Chromiumoxide,
}

pub struct Browser {
    _browser_type: BrowserType,
    driver: Option<WebDriver>,
    #[cfg(feature = "chromiumoxide-backend")]
    chromium: Option<ChromiumBrowser>,
    #[cfg(feature = "chromiumoxide-backend")]
    chromium_page: Option<Page>,
    #[cfg(feature = "chromiumoxide-backend")]
    #[allow(dead_code)]
    handler_task: Option<tokio::task::JoinHandle<()>>,
}

impl Browser {
    pub async fn new(browser_type: BrowserType) -> Result<Self> {
        let driver = match browser_type {
            BrowserType::WebDriver => {
                let caps = DesiredCapabilities::chrome();
                let driver = WebDriver::new("http://localhost:4444", caps).await?;
                Some(driver)
            }
            #[cfg(feature = "chromiumoxide-backend")]
            BrowserType::Chromiumoxide => {
                return Err(WebSpecError::Browser(
                    "Chromiumoxide backend needs to be initialized with Browser::new_chromiumoxide()".to_string(),
                ));
            }
        };

        Ok(Self {
            _browser_type: browser_type,
            driver,
            #[cfg(feature = "chromiumoxide-backend")]
            chromium: None,
            #[cfg(feature = "chromiumoxide-backend")]
            chromium_page: None,
            #[cfg(feature = "chromiumoxide-backend")]
            handler_task: None,
        })
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn new_chromiumoxide() -> Result<Self> {
        eprintln!("Creating browser config...");
        let config = BrowserConfig::builder()
            .no_sandbox()
            .headless_mode(HeadlessMode::New)
            .build()?;
        
        eprintln!("Launching chromium browser...");
        let (chromium, mut handler) = ChromiumBrowser::launch(config).await?;
        
        eprintln!("Starting event handler...");
        let handler_task = tokio::spawn(async move {
            eprintln!("Event handler loop started");
            while let Some(_event) = handler.next().await {
                // Just consume events
            }
            eprintln!("Event handler loop ended");
        });

        eprintln!("Creating new page...");
        let page = chromium.new_page("about:blank").await?;
        
        eprintln!("Page created successfully");
        Ok(Self {
            _browser_type: BrowserType::Chromiumoxide,
            driver: None,
            chromium: Some(chromium),
            chromium_page: Some(page),
            handler_task: Some(handler_task),
        })
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn new_chromiumoxide_with_path(chrome_path: &str) -> Result<Self> {
        eprintln!("Creating browser config...");
        let config = BrowserConfig::builder()
            .chrome_executable(chrome_path)
            .arg("--no-sandbox")
            .arg("--disable-dev-shm-usage")
            .arg("--disable-setuid-sandbox")
            .arg("--disable-gpu")
            .arg("--disable-software-rasterizer")
            .headless_mode(HeadlessMode::False)
            .window_size(1920, 1080)
            .build()?;
        
        eprintln!("Launching chromium browser...");
        let (chromium, mut handler) = ChromiumBrowser::launch(config).await?;
        
        eprintln!("Starting event handler...");
        let handler_task = tokio::spawn(async move {
            eprintln!("Event handler loop started");
            while let Some(_event) = handler.next().await {
                // Just consume events
            }
            eprintln!("Event handler loop ended");
        });

        eprintln!("Creating new page...");
        let page = chromium.new_page("about:blank").await?;
        
        eprintln!("Page created successfully");
        Ok(Self {
            _browser_type: BrowserType::Chromiumoxide,
            driver: None,
            chromium: Some(chromium),
            chromium_page: Some(page),
            handler_task: Some(handler_task),
        })
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn navigate_to(&mut self, url: &str) -> Result<()> {
        if let Some(driver) = &self.driver {
            driver.goto(url).await?;
        } else if let Some(page) = &self.chromium_page {
            page.goto(url).await?;
        } else {
            return Err(WebSpecError::Browser("No driver initialized".to_string()));
        }
        Ok(())
    }

    #[cfg(not(feature = "chromiumoxide-backend"))]
    pub async fn navigate_to(&mut self, url: &str) -> Result<()> {
        if let Some(driver) = &self.driver {
            driver.goto(url).await?;
        } else {
            return Err(WebSpecError::Browser("No driver initialized".to_string()));
        }
        Ok(())
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn wait_for_load(&mut self) -> Result<()> {
        if self.driver.is_some() || self.chromium_page.is_some() {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            Ok(())
        } else {
            Err(WebSpecError::Browser("No driver initialized".to_string()))
        }
    }

    #[cfg(not(feature = "chromiumoxide-backend"))]
    pub async fn wait_for_load(&mut self) -> Result<()> {
        if self.driver.is_some() {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            Ok(())
        } else {
            Err(WebSpecError::Browser("No driver initialized".to_string()))
        }
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn get_html(&self) -> Result<String> {
        if let Some(driver) = &self.driver {
            let result = driver.execute("return document.documentElement.outerHTML;", Vec::new()).await?;
            let html = result.json()
                .as_str()
                .ok_or_else(|| WebSpecError::Browser("Failed to get HTML".to_string()))?
                .to_string();
            Ok(html)
        } else if let Some(page) = &self.chromium_page {
            let html = page.evaluate("document.documentElement.outerHTML").await?.into_value()?;
            Ok(html)
        } else {
            Err(WebSpecError::Browser("No driver initialized".to_string()))
        }
    }

    #[cfg(not(feature = "chromiumoxide-backend"))]
    pub async fn get_html(&self) -> Result<String> {
        if let Some(driver) = &self.driver {
            let result = driver.execute("return document.documentElement.outerHTML;", Vec::new()).await?;
            let html = result.json()
                .as_str()
                .ok_or_else(|| WebSpecError::Browser("Failed to get HTML".to_string()))?
                .to_string();
            Ok(html)
        } else {
            Err(WebSpecError::Browser("No driver initialized".to_string()))
        }
    }

    pub fn driver(&self) -> Option<&WebDriver> {
        self.driver.as_ref()
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub fn chromium(&self) -> Option<&ChromiumBrowser> {
        self.chromium.as_ref()
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub fn chromium_page(&self) -> Option<&Page> {
        self.chromium_page.as_ref()
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        if let Some(driver) = self.driver.take() {
            tokio::spawn(async move {
                let _ = driver.quit().await;
            });
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "chromiumoxide-backend")]
    use super::*;

    #[cfg(feature = "chromiumoxide-backend")]
    #[test]
    fn test_chromiumoxide_browser_type_exists() {
        let _browser_type = BrowserType::Chromiumoxide;
    }

    #[cfg(feature = "chromiumoxide-backend")]
    #[tokio::test]
    #[ignore]
    async fn test_chromiumoxide_browser_creation() {
        let result = Browser::new_chromiumoxide().await;
        assert!(result.is_ok(), "Should successfully create Chromiumoxide browser");
    }
}
