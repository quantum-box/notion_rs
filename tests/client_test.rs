use dotenvy::dotenv;
use notion_rs::client::NotionClient;
use notion_rs::database::DatabaseQuery;

#[tokio::test]
async fn test_client_creation() {
    let client = NotionClient::new("dummy_token");
    // Test that invalid token results in error
    assert!(client.list_databases().await.is_err());
}

#[tokio::test]
async fn test_client_with_retry_config() {
    let client = NotionClient::new("dummy_token").with_retry_config(Default::default());
    // Test that invalid token results in error even with retry config
    assert!(client.list_databases().await.is_err());
}

// Integration tests using real Notion API token
#[cfg(feature = "integration")]
mod integration_tests {
    use super::*;

    fn get_test_token() -> String {
        dotenv().ok();
        std::env::var("NOTION_API_TOKEN").expect("NOTION_API_TOKEN must be set")
    }

    #[tokio::test]
    async fn test_list_databases() {
        let client = NotionClient::new(&get_test_token());
        let result = client.list_databases().await;
        assert!(result.is_ok(), "Failed to list databases: {:?}", result);
    }

    #[tokio::test]
    async fn test_database_operations() {
        let client = NotionClient::new(&get_test_token());

        // First, get an existing database from the workspace
        let databases = client.list_databases().await.unwrap();
        if databases.results.is_empty() {
            panic!("No databases found in the workspace. Please create at least one database with a 'Name' property in your Notion workspace.");
        }

        // Create a page in the existing database to use as our parent
        let page_result = client
            .create_database_page(&databases.results[0].id, "Test Parent Page")
            .await
            .unwrap();
        let parent_page_id = page_result.data.id.clone();

        // Create a test database
        let properties = serde_json::json!({
            "Name": {
                "title": {}
            },
            "Description": {
                "rich_text": {}
            }
        });

        let create_result = client
            .create_database(&parent_page_id, "Test Database", properties)
            .await;
        assert!(
            create_result.is_ok(),
            "Failed to create database: {:?}",
            create_result
        );

        let database = create_result.unwrap();
        let database_id = database.data.id;

        // Update the database
        let update_properties = serde_json::json!({
            "Tags": {
                "multi_select": {
                    "options": [
                        { "name": "Tag1", "color": "blue" },
                        { "name": "Tag2", "color": "red" }
                    ]
                }
            }
        });

        let update_result = client
            .update_database(
                &database_id,
                Some("Updated Test Database"),
                Some(update_properties),
            )
            .await;
        assert!(
            update_result.is_ok(),
            "Failed to update database: {:?}",
            update_result
        );

        // Query the database
        let query = DatabaseQuery {
            filter: None,
            sorts: None,
            page_size: Some(10),
            start_cursor: None,
        };

        let query_result = client.query_database(&database_id, query).await;
        assert!(
            query_result.is_ok(),
            "Failed to query database: {:?}",
            query_result
        );
    }
}
