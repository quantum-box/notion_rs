use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Common response structure for Notion API list endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse<T> {
    /// Array of objects
    pub results: Vec<T>,
    /// True if there are more results to fetch
    pub has_more: bool,
    /// Cursor to fetch next page, if has_more is true
    pub next_cursor: Option<String>,
}

/// Common response structure for Notion API object endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectResponse<T> {
    /// The object type
    pub object: String,
    /// The object data
    #[serde(flatten)]
    pub data: T,
}

/// Error response from Notion API
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// The error status code
    pub status: Option<i32>,
    /// The error code string
    pub code: String,
    /// The error message
    pub message: String,
}

/// Base properties shared by all Notion objects
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseProperties {
    /// Unique identifier of the object
    pub id: String,
    /// Type of the object (page, database, block, etc.)
    pub object: String,
    /// ISO 8601 date and time when this object was created
    pub created_time: String,
    /// ISO 8601 date and time when this object was last updated
    pub last_edited_time: String,
    /// Whether the object has been archived
    pub archived: bool,
    /// Additional properties specific to the object type
    #[serde(flatten)]
    pub properties: Value,
}

/// Represents a user in Notion
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier of the user
    pub id: String,
    /// Type of user (person or bot)
    pub object: String,
    /// User's name
    pub name: Option<String>,
    /// User's avatar URL
    pub avatar_url: Option<String>,
    /// Type-specific information
    #[serde(flatten)]
    pub type_specific: Value,
}

/// Retry configuration for handling rate limits
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Base delay in milliseconds between retries
    pub base_delay_ms: u64,
    /// Maximum delay in milliseconds between retries
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 5000,
        }
    }
}
