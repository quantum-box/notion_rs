use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::request::RequestBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub id: String,
    pub title: Vec<RichText>,
    pub properties: Value,
    pub url: String,
    pub created_time: String,
    pub last_edited_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RichText {
    pub plain_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DatabaseQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<Sort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct Sort {
    pub property: String,
    pub direction: SortDirection,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl Database {
    pub fn search_request(query: Option<&str>) -> RequestBuilder {
        let mut body = serde_json::json!({
            "filter": {
                "property": "object",
                "value": "database"
            }
        });
        
        if let Some(q) = query {
            body["query"] = serde_json::json!(q);
        }
        
        RequestBuilder::new("/search")
            .method("POST")
            .json_body(body)
    }

    pub fn list_request() -> RequestBuilder {
        // Use search endpoint with database filter instead of deprecated /databases endpoint
        Self::search_request(None)
    }

    pub fn get_request(database_id: &str) -> RequestBuilder {
        RequestBuilder::new(&format!("/databases/{}", database_id))
    }

    pub fn create_request(parent_page_id: &str, title: &str, properties: Value) -> RequestBuilder {
        RequestBuilder::new("/databases")
            .method("POST")
            .json_body(serde_json::json!({
                "parent": { "type": "page_id", "page_id": parent_page_id },
                "title": [{
                    "type": "text",
                    "text": { "content": title }
                }],
                "properties": properties
            }))
    }

    pub fn update_request(
        database_id: &str,
        title: Option<&str>,
        properties: Option<Value>,
    ) -> RequestBuilder {
        let mut body = serde_json::Map::new();

        if let Some(title_str) = title {
            body.insert(
                "title".to_string(),
                serde_json::json!([{
                    "type": "text",
                    "text": { "content": title_str }
                }]),
            );
        }

        if let Some(props) = properties {
            body.insert("properties".to_string(), props);
        }

        RequestBuilder::new(&format!("/databases/{}", database_id))
            .method("PATCH")
            .json_body(serde_json::Value::Object(body))
    }

    pub fn query_request(database_id: &str, query: DatabaseQuery) -> RequestBuilder {
        RequestBuilder::new(&format!("/databases/{}/query", database_id))
            .method("POST")
            .json_body(serde_json::to_value(query).unwrap())
    }
}
