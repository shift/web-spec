use thiserror::Error;

pub type Result<T> = std::result::Result<T, WebSpecError>;

#[derive(Error, Debug)]
pub enum WebSpecError {
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

    #[error("Script execution error: {0}")]
    Script(String),
}

impl From<String> for WebSpecError {
    fn from(s: String) -> Self {
        WebSpecError::Browser(s)
    }
}

#[cfg(feature = "chromiumoxide-backend")]
impl From<chromiumoxide::error::CdpError> for WebSpecError {
    fn from(e: chromiumoxide::error::CdpError) -> Self {
        WebSpecError::Browser(e.to_string())
    }
}

impl From<serde_json::Error> for WebSpecError {
    fn from(e: serde_json::Error) -> Self {
        WebSpecError::Conversion(e.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for WebSpecError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        WebSpecError::Timeout
    }
}
