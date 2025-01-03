use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

use crate::error::NotionError;
use crate::request::RequestBuilder;
use crate::response::RetryConfig;

const NOTION_API_BASE: &str = "https://api.notion.com/v1";

/// NotionClient handles all communication with the Notion API
pub struct NotionClient {
    http_client: Client,
    auth_token: String,
    retry_config: RetryConfig,
}

impl NotionClient {
    /// Creates a new NotionClient with the given authentication token
    pub fn new(auth_token: &str) -> Self {
        Self {
            http_client: Client::new(),
            auth_token: auth_token.to_string(),
            retry_config: RetryConfig::default(),
        }
    }

    /// Set custom retry configuration
    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Calculate delay for retry attempt using exponential backoff
    fn calculate_retry_delay(&self, attempt: u32, retry_after: Option<u64>) -> Duration {
        if let Some(seconds) = retry_after {
            return Duration::from_secs(seconds);
        }
        let base_delay = self.retry_config.base_delay_ms;
        let max_delay = self.retry_config.max_delay_ms;
        let delay = base_delay * 2u64.pow(attempt);
        Duration::from_millis(delay.min(max_delay))
    }

    /// Performs a GET request to the specified Notion API endpoint
    pub async fn get<T>(&self, request: RequestBuilder) -> Result<T, NotionError>
    where
        T: DeserializeOwned,
    {
        let url = request.build_url(NOTION_API_BASE);
        let mut attempt = 0u32;
        loop {
            let response = self
                .http_client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.auth_token))
                .header("Notion-Version", "2022-06-28")
                .send()
                .await?;

            let status = response.status();

            if status.is_success() {
                return Ok(response.json::<T>().await?);
            }

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse().ok());

                if attempt < self.retry_config.max_retries {
                    sleep(self.calculate_retry_delay(attempt, retry_after)).await;
                    attempt += 1;
                    continue;
                }
                return Err(NotionError::RateLimited {
                    retry_after: retry_after.unwrap_or(60),
                });
            }

            if status == reqwest::StatusCode::UNAUTHORIZED {
                return Err(NotionError::Unauthorized);
            }

            let error = response.json::<Value>().await?;
            return Err(NotionError::ApiError {
                code: error["code"].as_str().unwrap_or("unknown").to_string(),
                message: error["message"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string(),
            });
        }
    }

    /// Performs a POST request to the specified Notion API endpoint
    pub async fn post<T>(&self, request: RequestBuilder) -> Result<T, NotionError>
    where
        T: DeserializeOwned,
    {
        let url = request.build_url(NOTION_API_BASE);
        let body = request.get_body().ok_or_else(|| {
            NotionError::InvalidRequest("Request body is required for POST requests".to_string())
        })?;
        let mut attempt = 0u32;
        loop {
            let response = self
                .http_client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.auth_token))
                .header("Notion-Version", "2022-06-28")
                .json(&body)
                .send()
                .await?;

            let status = response.status();

            if status.is_success() {
                return Ok(response.json::<T>().await?);
            }

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse().ok());

                if attempt < self.retry_config.max_retries {
                    sleep(self.calculate_retry_delay(attempt, retry_after)).await;
                    attempt += 1;
                    continue;
                }
                return Err(NotionError::RateLimited {
                    retry_after: retry_after.unwrap_or(60),
                });
            }

            if status == reqwest::StatusCode::UNAUTHORIZED {
                return Err(NotionError::Unauthorized);
            }

            let error = response.json::<Value>().await?;
            return Err(NotionError::ApiError {
                code: error["code"].as_str().unwrap_or("unknown").to_string(),
                message: error["message"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string(),
            });
        }
    }

    /// Performs a PATCH request to the specified Notion API endpoint
    pub async fn patch<T>(&self, request: RequestBuilder) -> Result<T, NotionError>
    where
        T: DeserializeOwned,
    {
        let url = request.build_url(NOTION_API_BASE);
        let body = request.get_body().ok_or_else(|| {
            NotionError::InvalidRequest("Request body is required for PATCH requests".to_string())
        })?;
        let mut attempt = 0u32;
        loop {
            let response = self
                .http_client
                .patch(&url)
                .header("Authorization", format!("Bearer {}", self.auth_token))
                .header("Notion-Version", "2022-06-28")
                .json(&body)
                .send()
                .await?;

            let status = response.status();

            if status.is_success() {
                return Ok(response.json::<T>().await?);
            }

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse().ok());

                if attempt < self.retry_config.max_retries {
                    sleep(self.calculate_retry_delay(attempt, retry_after)).await;
                    attempt += 1;
                    continue;
                }
                return Err(NotionError::RateLimited {
                    retry_after: retry_after.unwrap_or(60),
                });
            }

            if status == reqwest::StatusCode::UNAUTHORIZED {
                return Err(NotionError::Unauthorized);
            }

            let error = response.json::<Value>().await?;
            return Err(NotionError::ApiError {
                code: error["code"].as_str().unwrap_or("unknown").to_string(),
                message: error["message"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string(),
            });
        }
    }

    /// Performs a DELETE request to the specified Notion API endpoint
    pub async fn delete<T>(&self, request: RequestBuilder) -> Result<T, NotionError>
    where
        T: DeserializeOwned,
    {
        let url = request.build_url(NOTION_API_BASE);
        let mut attempt = 0u32;
        loop {
            let response = self
                .http_client
                .delete(&url)
                .header("Authorization", format!("Bearer {}", self.auth_token))
                .header("Notion-Version", "2022-06-28")
                .send()
                .await?;

            let status = response.status();

            if status.is_success() {
                return Ok(response.json::<T>().await?);
            }

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse().ok());

                if attempt < self.retry_config.max_retries {
                    sleep(self.calculate_retry_delay(attempt, retry_after)).await;
                    attempt += 1;
                    continue;
                }
                return Err(NotionError::RateLimited {
                    retry_after: retry_after.unwrap_or(60),
                });
            }

            if status == reqwest::StatusCode::UNAUTHORIZED {
                return Err(NotionError::Unauthorized);
            }

            let error = response.json::<Value>().await?;
            return Err(NotionError::ApiError {
                code: error["code"].as_str().unwrap_or("unknown").to_string(),
                message: error["message"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string(),
            });
        }
    }
}
