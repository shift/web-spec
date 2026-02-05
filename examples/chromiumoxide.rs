#[cfg(feature = "chromiumoxide-backend")]
use web_spec::Browser;

#[cfg(feature = "chromiumoxide-backend")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://news.ycombinator.com/news";

    println!("=== Testing Chromiumoxide Backend ===");
    println!("Creating Chromiumoxide browser...");
    let mut browser = Browser::new_chromiumoxide().await?;

    println!("Navigating to {}...", url);
    browser.navigate_to(url).await?;

    println!("Waiting for page load...");
    browser.wait_for_load().await?;

    println!("Extracting HTML...");
    let html = browser.get_html().await?;

    println!("Converting to markdown...");
    let converter = web_spec::Converter::new();
    let markdown = converter.convert(&html)?;

    println!("\n=== Markdown Output (first 2000 chars) ===");
    println!("{}", &markdown.chars().take(2000).collect::<String>());

    Ok(())
}

#[cfg(not(feature = "chromiumoxide-backend"))]
fn main() {
    eprintln!("This example requires the 'chromiumoxide-backend' feature.");
    eprintln!("Run with: cargo run --example chromiumoxide --features chromiumoxide-backend");
}
