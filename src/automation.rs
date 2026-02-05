use crate::browser::Browser;
use crate::error::{Result, WebSpecError};

#[cfg(feature = "chromiumoxide-backend")]
use chromiumoxide::page::ScreenshotParams;

#[cfg(feature = "chromiumoxide-backend")]
use chromiumoxide::Page;

#[cfg(feature = "webdriver")]
use thirtyfour::{prelude::*, WebElement};

pub struct Automation<'a> {
    browser: &'a mut Browser,
}

impl<'a> Automation<'a> {
    pub fn new(browser: &'a mut Browser) -> Self {
        Self { browser }
    }

    pub fn get_browser(&self) -> &Browser {
        self.browser
    }

    #[cfg(feature = "chromiumoxide-backend")]
    fn page(&self) -> Result<&Page> {
        self.browser
            .chromium_page()
            .ok_or_else(|| WebSpecError::Automation("No chromiumoxide page initialized".to_string()))
    }

    #[cfg(feature = "webdriver")]
    fn driver(&self) -> Result<&WebDriver> {
        self.browser
            .driver()
            .ok_or_else(|| WebSpecError::Automation("No WebDriver initialized".to_string()))
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn click(&self, selector: &str) -> Result<()> {
        let page = self.page()?;
        let escaped_selector = selector.replace('\\', "\\\\").replace('\'', "\\'");
        let script = format!("document.querySelector('{}').click()", escaped_selector);
        page.evaluate(script.as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn click(&self, selector: &str) -> Result<WebElement> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        element.click().await?;
        Ok(element)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn type_text(&self, selector: &str, text: &str) -> Result<()> {
        let page = self.page()?;
        let escaped_selector = selector.replace('\\', "\\\\").replace('\'', "\\'");
        let escaped_text = text.replace('\\', "\\\\").replace('\'', "\\'");
        let script = format!(
            "document.querySelector('{}').value = '{}'",
            escaped_selector,
            escaped_text
        );
        page.evaluate(script.as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn type_text(&self, selector: &str, text: &str) -> Result<WebElement> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        element.send_keys(text).await?;
        Ok(element)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn clear_text(&self, selector: &str) -> Result<()> {
        let page = self.page()?;
        let script = format!(
            "document.querySelector('{}').value = ''",
            selector.replace("'", "\\'")
        );
        page.evaluate(script.as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn clear_text(&self, selector: &str) -> Result<WebElement> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        element.clear().await?;
        Ok(element)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn select_option(&self, selector: &str, value: &str) -> Result<()> {
        let page = self.page()?;
        let script = format!(
            "document.querySelector('{}').value = '{}'",
            selector.replace("'", "\\'"),
            value.replace("'", "\\'")
        );
        page.evaluate(script.as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn select_option(&self, selector: &str, value: &str) -> Result<()> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        element.send_keys(value).await?;
        Ok(())
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn wait_for_element(&self, selector: &str, _timeout_ms: u64) -> Result<()> {
        let page = self.page()?;
        let script = format!(
            "!!document.querySelector('{}')",
            selector.replace("'", "\\'")
        );
        page.evaluate(script.as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn wait_for_element(&self, selector: &str, timeout_ms: u64) -> Result<WebElement> {
        let driver = self.driver()?;
        let element = driver
            .query(By::Css(selector))
            .wait(std::time::Duration::from_millis(timeout_ms), std::time::Duration::from_millis(100))
            .first()
            .await
            .map_err(|_| WebSpecError::Timeout)?;
        Ok(element)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn wait_for_element_visible(&self, selector: &str, _timeout_ms: u64) -> Result<bool> {
        let page = self.page()?;
        let script = format!(
            "!!document.querySelector('{}')",
            selector.replace("'", "\\'")
        );
        let value: serde_json::Value = page.evaluate(script.as_str()).await?.into_value()?;
        Ok(value.as_bool().unwrap_or(false))
    }

    #[cfg(feature = "webdriver")]
    pub async fn wait_for_element_visible(&self, selector: &str, timeout_ms: u64) -> Result<bool> {
        let driver = self.driver()?;
        let element = driver
            .query(By::Css(selector))
            .wait(std::time::Duration::from_millis(timeout_ms), std::time::Duration::from_millis(100))
            .first()
            .await;
        Ok(element.is_ok())
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn element_exists(&self, selector: &str) -> Result<bool> {
        let page = self.page()?;
        let script = format!(
            "!!document.querySelector('{}')",
            selector.replace("'", "\\'")
        );
        let value: serde_json::Value = page.evaluate(script.as_str()).await?.into_value()?;
        Ok(value.as_bool().unwrap_or(false))
    }

    #[cfg(feature = "webdriver")]
    pub async fn element_exists(&self, selector: &str) -> Result<bool> {
        let driver = self.driver()?;
        match driver.query(By::Css(selector)).first().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn element_visible(&self, selector: &str) -> Result<bool> {
        let page = self.page()?;
        let script = format!(
            r#"(() => {{
                const el = document.querySelector('{}');
                return el && el.offsetParent !== null;
            }})()"#,
            selector.replace("'", "\\'")
        );
        let value: serde_json::Value = page.evaluate(script.as_str()).await?.into_value()?;
        Ok(value.as_bool().unwrap_or(false))
    }

    #[cfg(feature = "webdriver")]
    pub async fn element_visible(&self, selector: &str) -> Result<bool> {
        let driver = self.driver()?;
        match driver.find(By::Css(selector)).await {
            Ok(element) => {
                let displayed = element.is_displayed().await.unwrap_or(false);
                Ok(displayed)
            }
            Err(_) => Ok(false),
        }
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn scroll_to_bottom(&self) -> Result<()> {
        let page = self.page()?;
        page.evaluate("window.scrollTo(0, document.body.scrollHeight)").await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn scroll_to_bottom(&self) -> Result<()> {
        let driver = self.driver()?;
        driver.execute("window.scrollTo(0, document.body.scrollHeight);", vec![]).await?;
        Ok(())
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn scroll_to_top(&self) -> Result<()> {
        let page = self.page()?;
        page.evaluate("window.scrollTo(0, 0)").await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn scroll_to_top(&self) -> Result<()> {
        let driver = self.driver()?;
        driver.execute("window.scrollTo(0, 0);", vec![]).await?;
        Ok(())
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn scroll_to_element(&self, selector: &str) -> Result<()> {
        let page = self.page()?;
        let script = format!(
            "document.querySelector('{}').scrollIntoView({{behavior: 'smooth', block: 'center'}})",
            selector.replace("'", "\\'")
        );
        page.evaluate(script.as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn scroll_to_element(&self, selector: &str) -> Result<WebElement> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        driver.execute(
            "arguments[0].scrollIntoView({behavior: 'smooth', block: 'center'});",
            vec![serde_json::to_value(&element)?]
        ).await?;
        Ok(element)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn scroll_by(&self, x: i64, y: i64) -> Result<()> {
        let page = self.page()?;
        page.evaluate(format!("window.scrollBy({}, {})", x, y).as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn scroll_by(&self, x: i64, y: i64) -> Result<()> {
        let driver = self.driver()?;
        driver.execute(
            &format!("window.scrollBy({}, {});", x, y),
            vec![]
        ).await?;
        Ok(())
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn execute_script(&self, script: &str) -> Result<()> {
        let page = self.page()?;
        page.evaluate(script).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn execute_script(&self, script: &str) -> Result<()> {
        let driver = self.driver()?;
        driver.execute(script, vec![]).await?;
        Ok(())
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn get_text(&self, selector: &str) -> Result<String> {
        let page = self.page()?;
        let script = format!(
            "document.querySelector('{}').textContent",
            selector.replace("'", "\\'")
        );
        let value: serde_json::Value = page.evaluate(script.as_str()).await?.into_value()?;
        if let Some(text) = value.as_str() {
            Ok(text.trim().to_string())
        } else {
            Ok(value.to_string())
        }
    }

    #[cfg(feature = "webdriver")]
    pub async fn get_text(&self, selector: &str) -> Result<String> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        let text = element.text().await?;
        Ok(text)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn get_attribute(&self, selector: &str, attribute: &str) -> Result<String> {
        let page = self.page()?;
        let script = format!(
            "document.querySelector('{}').getAttribute('{}')",
            selector.replace("'", "\\'"),
            attribute.replace("'", "\\'")
        );
        let value: serde_json::Value = page.evaluate(script.as_str()).await?.into_value()?;
        if let Some(attr) = value.as_str() {
            Ok(attr.trim().to_string())
        } else {
            Err(WebSpecError::Automation(format!("Attribute '{}' not found", attribute)))
        }
    }

    #[cfg(feature = "webdriver")]
    pub async fn get_attribute(&self, selector: &str, attribute: &str) -> Result<String> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        let attr = element.attr(attribute).await?.ok_or_else(|| 
            WebSpecError::Automation(format!("Attribute '{}' not found", attribute))
        )?;
        Ok(attr)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn get_html(&self, selector: &str) -> Result<String> {
        let page = self.page()?;
        let script = format!(
            "document.querySelector('{}').outerHTML",
            selector.replace("'", "\\'")
        );
        let value: serde_json::Value = page.evaluate(script.as_str()).await?.into_value()?;
        if let Some(html_str) = value.as_str() {
            Ok(html_str.to_string())
        } else {
            Err(WebSpecError::Automation("Failed to convert HTML to string".to_string()))
        }
    }

    #[cfg(feature = "webdriver")]
    pub async fn get_html(&self, selector: &str) -> Result<String> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        let html = driver.execute(
            "return arguments[0].outerHTML;",
            vec![serde_json::to_value(&element)?]
        ).await?;
        let html_value = html.json();
        if let Some(html_str) = html_value.as_str() {
            Ok(html_str.to_string())
        } else {
            Err(WebSpecError::Automation("Failed to convert HTML to string".to_string()))
        }
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn wait_for_load(&self) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn wait_for_load(&self) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        Ok(())
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn hover(&self, selector: &str) -> Result<()> {
        let page = self.page()?;
        let script = format!(
            r#"const el = document.querySelector('{}'); 
            const evt = new MouseEvent('mouseover', {{bubbles: true, cancelable: true}}); 
            el.dispatchEvent(evt);"#,
            selector.replace("'", "\\'")
        );
        page.evaluate(script.as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn hover(&self, selector: &str) -> Result<WebElement> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        driver.execute(
            "var evt = new MouseEvent('mouseover', {bubbles: true, cancelable: true}); arguments[0].dispatchEvent(evt);",
            vec![serde_json::to_value(&element)?]
        ).await?;
        Ok(element)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn right_click(&self, selector: &str) -> Result<()> {
        let page = self.page()?;
        let script = format!(
            r#"const el = document.querySelector('{}'); 
            const evt = new MouseEvent('contextmenu', {{bubbles: true, cancelable: true}}); 
            el.dispatchEvent(evt);"#,
            selector.replace("'", "\\'")
        );
        page.evaluate(script.as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn right_click(&self, selector: &str) -> Result<WebElement> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        driver.execute(
            "var evt = new MouseEvent('contextmenu', {bubbles: true, cancelable: true}); arguments[0].dispatchEvent(evt);",
            vec![serde_json::to_value(&element)?]
        ).await?;
        Ok(element)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn double_click(&self, selector: &str) -> Result<()> {
        let page = self.page()?;
        let script = format!(
            r#"const el = document.querySelector('{}'); 
            const evt = new MouseEvent('dblclick', {{bubbles: true, cancelable: true}}); 
            el.dispatchEvent(evt);"#,
            selector.replace("'", "\\'")
        );
        page.evaluate(script.as_str()).await?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn double_click(&self, selector: &str) -> Result<WebElement> {
        let driver = self.driver()?;
        let element = driver.find(By::Css(selector)).await?;
        driver.execute(
            "var evt = new MouseEvent('dblclick', {bubbles: true, cancelable: true}); arguments[0].dispatchEvent(evt);",
            vec![serde_json::to_value(&element)?]
        ).await?;
        Ok(element)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn get_all_links(&self) -> Result<Vec<String>> {
        let page = self.page()?;
        let result = page.evaluate(
            "Array.from(document.querySelectorAll('a[href]')).map(a => a.href)"
        ).await?;
        let value: serde_json::Value = result.into_value()?;
        if let Some(arr) = value.as_array() {
            let mut links = Vec::new();
            for item in arr {
                if let Some(s) = item.as_str() {
                    links.push(s.to_string());
                }
            }
            Ok(links)
        } else {
            Ok(Vec::new())
        }
    }

    #[cfg(feature = "webdriver")]
    pub async fn get_all_links(&self) -> Result<Vec<String>> {
        let driver = self.driver()?;
        let elements = driver.find_all(By::Css("a[href]")).await?;
        let mut links = Vec::new();
        for element in elements {
            if let Some(href) = element.attr("href").await? {
                links.push(href);
            }
        }
        Ok(links)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn get_all_images(&self) -> Result<Vec<String>> {
        let page = self.page()?;
        let result = page.evaluate(
            "Array.from(document.querySelectorAll('img[src]')).map(img => img.src)"
        ).await?;
        let value: serde_json::Value = result.into_value()?;
        if let Some(arr) = value.as_array() {
            let mut images = Vec::new();
            for item in arr {
                if let Some(s) = item.as_str() {
                    images.push(s.to_string());
                }
            }
            Ok(images)
        } else {
            Ok(Vec::new())
        }
    }

    #[cfg(feature = "webdriver")]
    pub async fn get_all_images(&self) -> Result<Vec<String>> {
        let driver = self.driver()?;
        let elements = driver.find_all(By::Css("img[src]")).await?;
        let mut images = Vec::new();
        for element in elements {
            if let Some(src) = element.attr("src").await? {
                images.push(src);
            }
        }
        Ok(images)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn get_all_headings(&self, level: u32) -> Result<Vec<String>> {
        let page = self.page()?;
        let script = format!("Array.from(document.querySelectorAll('h{}')).map(h => h.textContent)", level);
        let result = page.evaluate(script.as_str()).await?;
        let value: serde_json::Value = result.into_value()?;
        if let Some(arr) = value.as_array() {
            let mut headings = Vec::new();
            for item in arr {
                if let Some(s) = item.as_str() {
                    headings.push(s.trim().to_string());
                }
            }
            Ok(headings)
        } else {
            Ok(Vec::new())
        }
    }

    #[cfg(feature = "webdriver")]
    pub async fn get_all_headings(&self, level: u32) -> Result<Vec<String>> {
        let driver = self.driver()?;
        let selector = &format!("h{}", level);
        let elements = driver.find_all(By::Css(selector)).await?;
        let mut headings = Vec::new();
        for element in elements {
            let text = element.text().await?;
            headings.push(text);
        }
        Ok(headings)
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn count_elements(&self, selector: &str) -> Result<usize> {
        let page = self.page()?;
        let script = format!(
            "document.querySelectorAll('{}').length",
            selector.replace("'", "\\'")
        );
        let result = page.evaluate(script.as_str()).await?;
        let value: serde_json::Value = result.into_value()?;
        Ok(value.as_u64().unwrap_or(0) as usize)
    }

    #[cfg(feature = "webdriver")]
    pub async fn count_elements(&self, selector: &str) -> Result<usize> {
        let driver = self.driver()?;
        let elements = driver.find_all(By::Css(selector)).await?;
        Ok(elements.len())
    }

    #[cfg(feature = "chromiumoxide-backend")]
    pub async fn take_screenshot(&self, path: &str) -> Result<()> {
        let page = self.page()?;
        let screenshot = page.screenshot(ScreenshotParams::builder().build()).await?;
        std::fs::write(path, screenshot)?;
        Ok(())
    }

    #[cfg(feature = "webdriver")]
    pub async fn take_screenshot(&self, path: &str) -> Result<()> {
        let driver = self.driver()?;
        let screenshot = driver.screenshot_as_png().await?;
        std::fs::write(path, screenshot)?;
        Ok(())
    }

    pub async fn wait(&self, ms: u64) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
        Ok(())
    }
}
