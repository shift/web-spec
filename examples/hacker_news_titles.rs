use regex::Regex;
#[cfg(feature = "chromiumoxide-backend")]
use web_spec::Browser;

#[cfg(feature = "chromiumoxide-backend")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://news.ycombinator.com/news";
    let chrome_path =
        "/nix/store/b8w6xj2isq4m120sz2gg4hb5gxhjmzca-chromium-143.0.7499.40/bin/chromium";

    println!("=== Hacker News Post Title Extractor ===");
    println!("Using real browser (visible window): {}", chrome_path);
    println!("Navigating to {}...\n", url);

    let start = std::time::Instant::now();

    println!("Initializing browser (you should see a window open)...");
    let mut browser = Browser::new_chromiumoxide_with_path(chrome_path).await?;
    println!(
        "Browser initialized in {:.2}s",
        start.elapsed().as_secs_f64()
    );

    println!("Navigating to {}...", url);
    let nav_start = std::time::Instant::now();
    browser.navigate_to(url).await?;
    println!("Navigated in {:.2}s", nav_start.elapsed().as_secs_f64());

    println!("Waiting for page load...");
    browser.wait_for_load().await?;
    println!("Page loaded");

    let html = browser.get_html().await?;
    println!("HTML extracted ({} bytes)", html.len());

    let titles = extract_hacker_news_titles(&html);
    println!("Extracted {} post titles", titles.len());

    println!("\nTop 10 Hacker News Posts:\n");
    for (i, title) in titles.iter().take(10).enumerate() {
        println!("{}. {}", i + 1, title);
    }

    println!("\nExtracted {} total posts", titles.len());
    println!("Total time: {:.2}s", start.elapsed().as_secs_f64());

    println!("\nBrowser window will remain open for 5 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    Ok(())
}

#[allow(dead_code)]
fn extract_hacker_news_titles(html: &str) -> Vec<String> {
    let title_pattern = Regex::new(r#"<span class="titleline"><a[^>]*>([^<]+)</a>"#).unwrap();

    title_pattern
        .captures_iter(html)
        .filter_map(|cap| cap.get(1))
        .map(|title| {
            title
                .as_str()
                .replace("&amp;", "&")
                .replace("&#x27;", "'")
                .replace("&quot;", "\"")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .trim()
                .to_string()
        })
        .take(30)
        .collect()
}

#[cfg(not(feature = "chromiumoxide-backend"))]
fn main() {
    eprintln!("Error: hacker_news_titles example requires the 'chromiumoxide-backend' feature.");
    eprintln!(
        "Please run with: cargo run --example hacker_news_titles --features chromiumoxide-backend"
    );
    std::process::exit(1);
}
