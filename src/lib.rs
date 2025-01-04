pub mod client;
pub mod database;
pub mod error;
pub mod request;
pub mod response;

// Re-export commonly used items
pub use client::NotionClient;
pub use database::Database;
pub use error::NotionError;
pub use request::RequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_builder() {
        let builder = RequestBuilder::new("/databases")
            .query_param("page_size", "100")
            .json_body(json!({
                "filter": {
                    "property": "Status",
                    "select": {
                        "equals": "Done"
                    }
                }
            }));

        let url = builder.build_url("https://api.notion.com/v1");
        assert_eq!(url, "https://api.notion.com/v1/databases?page_size=100");
        assert!(builder.get_body().is_some());
    }
}
