use async_trait::async_trait;
use crate::error::Result;

#[async_trait]
pub trait BrowserBackend: Send + Sync {
    async fn click(&self, selector: &str) -> Result<()>;
    async fn type_text(&self, selector: &str, text: &str) -> Result<()>;
    async fn clear_text(&self, selector: &str) -> Result<()>;
    async fn get_text(&self, selector: &str) -> Result<String>;
    async fn get_attribute(&self, selector: &str, attribute: &str) -> Result<String>;
    async fn is_visible(&self, selector: &str) -> Result<bool>;
    async fn is_enabled(&self, selector: &str) -> Result<bool>;
    async fn navigate_to(&self, url: &str) -> Result<()>;
    async fn wait_for_selector(&self, selector: &str, timeout_ms: u64) -> Result<()>;
    async fn execute_script(&self, script: &str, args: Vec<String>) -> Result<String>;
    async fn get_html(&self) -> Result<String>;
    async fn get_title(&self) -> Result<String>;
    async fn get_url(&self) -> Result<String>;
    async fn wait_for_load(&self) -> Result<()>;
}

#[cfg(feature = "chromiumoxide-backend")]
pub struct ChromiumoxideBackend {
    page: chromiumoxide::Page,
}

#[cfg(feature = "chromiumoxide-backend")]
impl ChromiumoxideBackend {
    pub fn new(page: chromiumoxide::Page) -> Self {
        Self { page }
    }
}

#[cfg(feature = "chromiumoxide-backend")]
#[async_trait]
impl BrowserBackend for ChromiumoxideBackend {
    async fn click(&self, selector: &str) -> Result<()> {
        let script = format!(
            "document.querySelector('{}').click()",
            escape_selector(selector)
        );
        self.page.evaluate(script.as_str()).await?;
        Ok(())
    }

    async fn type_text(&self, selector: &str, text: &str) -> Result<()> {
        let script = format!(
            "document.querySelector('{}').value = {}",
            escape_selector(selector),
            serde_json::to_string(text)?
        );
        self.page.evaluate(script.as_str()).await?;
        Ok(())
    }

    async fn clear_text(&self, selector: &str) -> Result<()> {
        let script = format!(
            "document.querySelector('{}').value = ''",
            escape_selector(selector)
        );
        self.page.evaluate(script.as_str()).await?;
        Ok(())
    }

    async fn get_text(&self, selector: &str) -> Result<String> {
        let script = format!(
            "document.querySelector('{}')?.textContent || ''",
            escape_selector(selector)
        );
        let result = self.page.evaluate(script.as_str()).await?;
        match result.into_value()? {
            Some(serde_json::Value::String(s)) => Ok(s),
            _ => Ok(String::new()),
        }
    }

    async fn get_attribute(&self, selector: &str, attribute: &str) -> Result<String> {
        let script = format!(
            "document.querySelector('{}')?.getAttribute('{}') || ''",
            escape_selector(selector),
            escape_selector(attribute)
        );
        let result = self.page.evaluate(script.as_str()).await?;
        match result.into_value()? {
            Some(serde_json::Value::String(s)) => Ok(s),
            _ => Ok(String::new()),
        }
    }

    async fn is_visible(&self, selector: &str) -> Result<bool> {
        let script = format!(
            "!!document.querySelector('{}')?.offsetParent",
            escape_selector(selector)
        );
        let result = self.page.evaluate(script.as_str()).await?;
        match result.into_value()? {
            Some(serde_json::Value::Bool(b)) => Ok(b),
            _ => Ok(false),
        }
    }

    async fn is_enabled(&self, selector: &str) -> Result<bool> {
        let script = format!(
            "!document.querySelector('{}')?.disabled",
            escape_selector(selector)
        );
        let result = self.page.evaluate(script.as_str()).await?;
        match result.into_value()? {
            Some(serde_json::Value::Bool(b)) => Ok(b),
            _ => Ok(true),
        }
    }

    async fn navigate_to(&self, url: &str) -> Result<()> {
        self.page.goto(url).await?;
        Ok(())
    }

    async fn wait_for_selector(&self, selector: &str, timeout_ms: u64) -> Result<()> {
        let script = format!(
            "!!document.querySelector('{}')",
            escape_selector(selector)
        );
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        tokio::time::timeout(
            std::time::Duration::from_millis(timeout_ms),
            async {
                loop {
                    interval.tick().await;
                    let result = self.page.evaluate(script.as_str()).await?;
                    match result.into_value()? {
                        Some(serde_json::Value::Bool(true)) => {
                            return Ok::<(), crate::error::WebSpecError>(());
                        }
                        _ => continue,
                    }
                }
            },
        )
        .await??;
        Ok(())
    }

    async fn execute_script(&self, script: &str, _args: Vec<String>) -> Result<String> {
        let result = self.page.evaluate(script).await?;
        match result.into_value()? {
            Some(serde_json::Value::String(s)) => Ok(s),
            Some(serde_json::Value::Number(n)) => Ok(n.to_string()),
            Some(serde_json::Value::Bool(b)) => Ok(b.to_string()),
            _ => Ok(String::new()),
        }
    }

    async fn get_html(&self) -> Result<String> {
        self.page.content().await.map_err(Into::into)
    }

    async fn get_title(&self) -> Result<String> {
        Ok(self.page.get_title().await.unwrap_or_default())
    }

    async fn get_url(&self) -> Result<String> {
        Ok(self.page.url().await.unwrap_or_default())
    }

    async fn wait_for_load(&self) -> Result<()> {
        self.page.wait_for_navigation().await?;
        Ok(())
    }
}

#[cfg(feature = "webdriver")]
pub struct WebDriverBackend {
    driver: std::sync::Arc<thirtyfour::WebDriver>,
}

#[cfg(feature = "webdriver")]
impl WebDriverBackend {
    pub fn new(driver: std::sync::Arc<thirtyfour::WebDriver>) -> Self {
        Self { driver }
    }
}

#[cfg(feature = "webdriver")]
#[async_trait]
impl BrowserBackend for WebDriverBackend {
    async fn click(&self, selector: &str) -> Result<()> {
        let element = self.driver.find(thirtyfour::By::Css(selector)).await?;
        element.click().await?;
        Ok(())
    }

    async fn type_text(&self, selector: &str, text: &str) -> Result<()> {
        let element = self.driver.find(thirtyfour::By::Css(selector)).await?;
        element.clear().await?;
        element.send_keys(text).await?;
        Ok(())
    }

    async fn clear_text(&self, selector: &str) -> Result<()> {
        let element = self.driver.find(thirtyfour::By::Css(selector)).await?;
        element.clear().await?;
        Ok(())
    }

    async fn get_text(&self, selector: &str) -> Result<String> {
        let element = self.driver.find(thirtyfour::By::Css(selector)).await?;
        Ok(element.text().await?)
    }

    async fn get_attribute(&self, selector: &str, attribute: &str) -> Result<String> {
        let element = self.driver.find(thirtyfour::By::Css(selector)).await?;
        Ok(element.attr(attribute).await?.unwrap_or_default())
    }

    async fn is_visible(&self, selector: &str) -> Result<bool> {
        let element = self.driver.find(thirtyfour::By::Css(selector)).await?;
        Ok(element.is_displayed().await?)
    }

    async fn is_enabled(&self, selector: &str) -> Result<bool> {
        let element = self.driver.find(thirtyfour::By::Css(selector)).await?;
        Ok(element.is_enabled().await?)
    }

    async fn navigate_to(&self, url: &str) -> Result<()> {
        self.driver.goto(url).await?;
        Ok(())
    }

    async fn wait_for_selector(&self, selector: &str, timeout_ms: u64) -> Result<()> {
        let condition = thirtyfour::element::Located(thirtyfour::By::Css(selector));
        thirtyfour::WebDriver::wait(self.driver.as_ref(), std::time::Duration::from_millis(timeout_ms))
            .until(condition)
            .await?;
        Ok(())
    }

    async fn execute_script(&self, script: &str, args: Vec<String>) -> Result<String> {
        use thirtyfour::ScriptArgs;
        let script_args: Vec<thirtyfour::WebDriverToElementConvert> =
            args.into_iter().map(|a| a.into()).collect();
        let result = self.driver.execute(script, ScriptArgs(script_args)).await?;
        result.json().map(|j| j.to_string()).ok_or_else(||
            crate::error::WebSpecError::Automation("Script returned no result".to_string()))
    }

    async fn get_html(&self) -> Result<String> {
        Ok(self.driver.source().await?)
    }

    async fn get_title(&self) -> Result<String> {
        Ok(self.driver.title().await?)
    }

    async fn get_url(&self) -> Result<String> {
        Ok(self.driver.current_url().await?)
    }

    async fn wait_for_load(&self) -> Result<()> {
        thirtyfour::WebDriver::wait(self.driver.as_ref(), std::time::Duration::from_secs(30))
            .until(thirtyfour::script::document_complete())
            .await?;
        Ok(())
    }
}

fn escape_selector(selector: &str) -> String {
    selector.replace('\\', "\\\\").replace('\'', "\\'")
}
