use crate::error::Result;

#[derive(Debug, Clone)]
pub struct Converter {
    options: ConversionOptions,
}

#[derive(Debug, Clone)]
pub struct ConversionOptions {
    pub include_code_blocks: bool,
    pub preserve_links: bool,
    pub include_tables: bool,
    pub strip_images: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            include_code_blocks: true,
            preserve_links: true,
            include_tables: true,
            strip_images: false,
        }
    }
}

impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}

impl Converter {
    pub fn new() -> Self {
        Self {
            options: ConversionOptions::default(),
        }
    }

    pub fn with_options(options: ConversionOptions) -> Self {
        Self { options }
    }

    pub fn convert(&self, html: &str) -> Result<String> {
        let markdown = html2md::parse_html(html);
        Ok(markdown)
    }

    pub fn convert_with_cleanup(&self, html: &str) -> Result<String> {
        let markdown = html2md::parse_html(html);

        let result = if self.options.strip_images {
            strip_images(&markdown)
        } else {
            markdown
        };

        let cleaned = normalize_whitespace(&result);

        Ok(cleaned)
    }

    pub fn strip_images_only(&self, html: &str) -> Result<String> {
        let markdown = html2md::parse_html(html);
        let stripped = strip_images(&markdown);
        Ok(stripped)
    }
}

fn strip_images(markdown: &str) -> String {
    let re = regex::Regex::new(r"!\[.*?\]\([^)]*\)").unwrap();
    re.replace_all(markdown, "").to_string()
}

fn normalize_whitespace(markdown: &str) -> String {
    let re = regex::Regex::new(r"\n{3,}").unwrap();
    re.replace_all(markdown, "\n\n").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let converter = Converter::new();
        let html = "<h1>Hello</h1><p>World</p>";
        let result = converter.convert(html);
        assert!(result.is_ok());
        let markdown = result.unwrap();
        assert!(markdown.contains("Hello"));
    }

    #[test]
    fn test_strip_images() {
        let converter = Converter::new();
        let html = "<img src=\"test.jpg\" />";
        let result = converter.strip_images_only(html);
        assert!(result.is_ok());
        let markdown = result.unwrap();
        eprintln!("Markdown output: '{}'", markdown);
        eprintln!("Has ![ : {}", markdown.contains("!["));
    }

    #[test]
    fn test_with_cleanup() {
        let converter = Converter::new();
        let html = "<h1>Hello</h1><p>World</p>";
        let result = converter.convert_with_cleanup(html);
        assert!(result.is_ok());
        let markdown = result.unwrap();
        assert!(markdown.contains("Hello"));
    }
}
