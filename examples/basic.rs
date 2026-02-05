use web_spec::{WebSpec, BrowserType, Automation};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let converter = WebSpec::new();
    
    let url = "https://example.com";
    println!("Converting {} to markdown...", url);
    
    let markdown = converter.from_url(url).await?;
    println!("\n{}", markdown);
    
    Ok(())
}

#[tokio::main]
#[allow(dead_code)]
async fn with_automation() -> Result<()> {
    let mut browser = web_spec::Browser::new(BrowserType::WebDriver).await?;
    
    browser.navigate_to("https://example.com").await?;
    let automation = Automation::new(&mut browser);
    automation.wait_for_element("h1", 5000).await?;
    
    let html = browser.get_html().await?;
    let markdown = web_spec::Converter::new().convert(&html)?;
    
    println!("{}", markdown);
    
    Ok(())
}
