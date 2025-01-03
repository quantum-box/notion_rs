use notion_rs::client::NotionClient;
use notion_rs::error::NotionError;

#[tokio::test]
async fn test_client_creation() {
    let client = NotionClient::new("dummy_token");
    assert_eq!(client.auth_token(), "dummy_token");
}

#[tokio::test]
async fn test_client_with_retry_config() {
    let client = NotionClient::new("dummy_token")
        .with_retry_config(Default::default());
    assert!(client.retry_config().max_retries > 0);
}

// These tests will be implemented once we receive the Notion API token
#[cfg(feature = "integration")]
mod integration_tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_get_database() {
        // TODO: Implement with actual token
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_database() {
        // TODO: Implement with actual token
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_database() {
        // TODO: Implement with actual token
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_database() {
        // TODO: Implement with actual token
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_database() {
        // TODO: Implement with actual token
    }
}
