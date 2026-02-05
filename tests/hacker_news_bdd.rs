// BDD integration tests for Hacker News post title extraction
// These tests follow Given-When-Then structure

use regex::Regex;
use web_spec::{Browser, BrowserType};

#[cfg(test)]
mod hacker_news_tests {
    use super::*;

    // Helper structure to hold test context
    struct HackerNewsContext {
        html: Option<String>,
        post_titles: Vec<String>,
        backend_name: String,
    }

    // GIVEN: A browser backend is available
    fn given_webdriver_backend() -> HackerNewsContext {
        HackerNewsContext {
            html: None,
            post_titles: Vec::new(),
            backend_name: "WebDriver".to_string(),
        }
    }

    #[cfg(feature = "chromiumoxide-backend")]
    fn given_chromiumoxide_backend() -> HackerNewsContext {
        HackerNewsContext {
            html: None,
            post_titles: Vec::new(),
            backend_name: "Chromiumoxide".to_string(),
        }
    }

    // WHEN: Navigate to Hacker News
    async fn when_navigate_to_hacker_news(
        context: &mut HackerNewsContext,
        browser_type: BrowserType,
    ) {
        let mut browser = Browser::new(browser_type).await.expect(&format!(
            "{} browser should initialize",
            context.backend_name
        ));
        browser
            .navigate_to("https://news.ycombinator.com/news")
            .await
            .expect("Navigation should succeed");
        browser.wait_for_load().await.expect("Wait should complete");

        let html = browser.get_html().await.expect("HTML should be extracted");
        context.html = Some(html);
    }

    #[cfg(feature = "chromiumoxide-backend")]
    async fn when_navigate_chromiumoxide(context: &mut HackerNewsContext) {
        let mut browser = Browser::new_chromiumoxide()
            .await
            .expect("Chromiumoxide browser should initialize");
        browser
            .navigate_to("https://news.ycombinator.com/news")
            .await
            .expect("Navigation should succeed");
        browser.wait_for_load().await.expect("Wait should complete");

        let html = browser.get_html().await.expect("HTML should be extracted");
        context.html = Some(html);
    }

    // WHEN: Extract post titles from HTML
    fn when_extract_post_titles(context: &mut HackerNewsContext) {
        let html = context.html.as_ref().expect("HTML should be available");
        context.post_titles = extract_hacker_news_titles(html);
    }

    // THEN: Verify we can extract titles
    fn then_can_extract_titles(context: &HackerNewsContext) {
        let html = context.html.as_ref().expect("HTML should be available");
        assert!(
            html.contains("Hacker News"),
            "HTML should contain Hacker News title"
        );
        assert!(!html.is_empty(), "HTML should not be empty");
    }

    // THEN: Verify we have at least 10 posts
    fn then_at_least_10_titles(context: &HackerNewsContext) {
        assert!(
            context.post_titles.len() >= 10,
            "Should extract at least 10 post titles, got {}",
            context.post_titles.len()
        );
    }

    // THEN: Verify each title has text
    fn then_each_title_has_text(context: &HackerNewsContext) {
        for (i, title) in context.post_titles.iter().enumerate() {
            assert!(
                !title.trim().is_empty(),
                "Post title {} should not be empty",
                i + 1
            );
            assert!(
                title.len() > 5,
                "Post title {} should be meaningful length, got: '{}'",
                i + 1,
                title
            );
        }
    }

    // THEN: Display top 10 titles
    fn then_display_top_10_titles(context: &HackerNewsContext) {
        println!(
            "\n=== Top 10 Hacker News Posts ({}) ===",
            context.backend_name
        );
        for (i, title) in context.post_titles.iter().take(10).enumerate() {
            println!("{}. {}", i + 1, title.trim());
        }
    }

    // Helper function to extract Hacker News post titles
    pub fn extract_hacker_news_titles(html: &str) -> Vec<String> {
        let title_pattern = Regex::new(r#"<span class="titleline"><a[^>]*>([^<]+)</a>"#).unwrap();

        title_pattern
            .captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|title| {
                // Decode HTML entities and clean up whitespace
                title
                    .as_str()
                    .replace("&amp;", "&")
                    .replace("&#x27;", "'")
                    .replace("&quot;", "\"")
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&nbsp;", " ")
                    .trim()
                    .to_string()
            })
            .take(30) // Take first 30 posts
            .collect()
    }

    // Test: Extract titles using WebDriver
    #[tokio::test]
    #[ignore] // Requires actual WebDriver server
    async fn given_webdriver_backend_when_navigate_to_hacker_news_then_extract_10_titles() {
        let mut context = given_webdriver_backend();

        when_navigate_to_hacker_news(&mut context, BrowserType::WebDriver).await;
        when_extract_post_titles(&mut context);

        then_can_extract_titles(&context);
        then_at_least_10_titles(&context);
        then_each_title_has_text(&context);
        then_display_top_10_titles(&context);
    }

    // Test: Extract titles using Chromiumoxide
    #[cfg(feature = "chromiumoxide-backend")]
    #[tokio::test]
    #[ignore] // Requires Chromium browser
    async fn given_chromiumoxide_backend_when_navigate_to_hacker_news_then_extract_10_titles() {
        let mut context = given_chromiumoxide_backend();

        when_navigate_chromiumoxide(&mut context).await;
        when_extract_post_titles(&mut context);

        then_can_extract_titles(&context);
        then_at_least_10_titles(&context);
        then_each_title_has_text(&context);
        then_display_top_10_titles(&context);
    }

    // Test: Verify HTML extraction works correctly
    #[test]
    fn given_hacker_news_html_when_extracting_titles_then_get_correct_count() {
        // GIVEN: Mock Hacker News HTML
        let mock_html = r#"
            <html>
                <body>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link1">First Post Title</a></span></td>
                    </tr>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link2">Second Post Title</a></span></td>
                    </tr>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link3">Third Post Title</a></span></td>
                    </tr>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link4">Fourth Post Title</a></span></td>
                    </tr>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link5">Fifth Post Title</a></span></td>
                    </tr>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link6">Sixth Post Title</a></span></td>
                    </tr>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link7">Seventh Post Title</a></span></td>
                    </tr>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link8">Eighth Post Title</a></span></td>
                    </tr>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link9">Ninth Post Title</a></span></td>
                    </tr>
                    <tr class="athing">
                        <td class="title"><span class="titleline"><a href="link10">Tenth Post Title</a></span></td>
                    </tr>
                </body>
            </html>
        "#;

        // WHEN: Extracting post titles
        let titles = extract_hacker_news_titles(mock_html);

        // THEN: Should extract 10 titles
        assert_eq!(titles.len(), 10, "Should extract 10 post titles");
        assert_eq!(titles[0], "First Post Title");
        assert_eq!(titles[1], "Second Post Title");
        assert_eq!(titles[9], "Tenth Post Title");
    }

    // Test: Verify title formatting
    #[test]
    fn given_extracted_titles_when_formatted_then_show_number_and_text() {
        // GIVEN: Mock extracted titles
        let titles = vec![
            "First Title".to_string(),
            "Second Title".to_string(),
            "Third Title".to_string(),
        ];

        // WHEN: Formatting for display
        let mut output = String::new();
        for (i, title) in titles.iter().take(3).enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, title));
        }

        // THEN: Should have numbered format
        assert!(output.contains("1. First Title"));
        assert!(output.contains("2. Second Title"));
        assert!(output.contains("3. Third Title"));
    }

    // Test: Handle HTML entities in titles
    #[test]
    fn given_html_with_entities_when_extracting_then_decode_correctly() {
        // GIVEN: HTML with entities
        let html = r##"<span class="titleline"><a href="#">C &amp; D</a></span><span class="titleline"><a href="#">Test &#x27;s</a></span>"##;

        // WHEN: Extracting titles
        let titles = extract_hacker_news_titles(html);

        // THEN: Should decode entities correctly
        assert_eq!(titles.len(), 2);
        assert!(titles[0].contains("C & D"));
        assert!(titles[1].contains("Test 's"));
    }
}
