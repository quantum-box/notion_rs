use reqwest::Error as ReqwestError;
use std::fmt;

#[derive(Debug)]
pub enum NotionError {
    /// HTTP client errors from reqwest
    HttpError(ReqwestError),
    /// Rate limit exceeded
    RateLimited {
        /// Time to wait before retrying (in seconds)
        retry_after: u64,
    },
    /// Authentication failed
    Unauthorized,
    /// Invalid request parameters
    InvalidRequest(String),
    /// Notion API returned an error
    ApiError {
        /// Error code from Notion
        code: String,
        /// Error message from Notion
        message: String,
    },
}

impl fmt::Display for NotionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotionError::HttpError(e) => write!(f, "HTTP request failed: {}", e),
            NotionError::RateLimited { retry_after } => {
                write!(
                    f,
                    "Rate limit exceeded. Retry after {} seconds",
                    retry_after
                )
            }
            NotionError::Unauthorized => write!(f, "Authentication failed"),
            NotionError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            NotionError::ApiError { code, message } => {
                write!(f, "Notion API error {}: {}", code, message)
            }
        }
    }
}

impl std::error::Error for NotionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NotionError::HttpError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ReqwestError> for NotionError {
    fn from(error: ReqwestError) -> Self {
        NotionError::HttpError(error)
    }
}
