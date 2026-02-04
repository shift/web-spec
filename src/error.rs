use thiserror::Error;

pub type Result<T> = std::result::Result<T, Web2MarkdownError>;

#[derive(Error, Debug)]
pub enum Web2MarkdownError {
    #[error("Browser error: {0}")]
    Browser(String),

    #[error("Automation error: {0}")]
    Automation(String),

    #[error("Conversion error: {0}")]
    Conversion(String),

    #[error("WebDriver error: {0}")]
    WebDriver(#[from] thirtyfour::error::WebDriverError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Timeout waiting for element")]
    Timeout,

    #[error("Element not found")]
    NotFound,
}

impl From<String> for Web2MarkdownError {
    fn from(s: String) -> Self {
        Web2MarkdownError::Browser(s)
    }
}

#[cfg(feature = "chromiumoxide-backend")]
impl From<chromiumoxide::error::CdpError> for Web2MarkdownError {
    fn from(e: chromiumoxide::error::CdpError) -> Self {
        Web2MarkdownError::Browser(e.to_string())
    }
}

impl From<serde_json::Error> for Web2MarkdownError {
    fn from(e: serde_json::Error) -> Self {
        Web2MarkdownError::Conversion(e.to_string())
    }
}
