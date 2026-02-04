use web_spec::{Browser, Converter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://news.ycombinator.com/news";
    
    println!("=== Testing WebDriver Backend ===");
    println!("Navigating to {}...", url);
    
    let mut browser = Browser::new(web_spec::BrowserType::WebDriver).await?;
    browser.navigate_to(url).await?;
    browser.wait_for_load().await?;
    
    println!("Extracting HTML...");
    let html = browser.get_html().await?;
    
    println!("Converting to markdown...");
    let converter = Converter::new();
    let markdown = converter.convert(&html)?;
    
    println!("\n=== Markdown Output (first 2000 chars) ===");
    println!("{}", &markdown.chars().take(2000).collect::<String>());
    
    Ok(())
}
