use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// Builder for constructing Notion API requests
pub struct RequestBuilder {
    /// The API endpoint path
    endpoint: String,
    /// Query parameters for the request
    query_params: HashMap<String, String>,
    /// JSON body for POST/PATCH requests
    body: Option<Value>,
}

impl RequestBuilder {
    /// Creates a new RequestBuilder with the given endpoint
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            query_params: HashMap::new(),
            body: None,
        }
    }

    /// Adds a query parameter to the request
    pub fn query_param<K, V>(mut self, key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        self.query_params.insert(key.to_string(), value.to_string());
        self
    }

    /// Adds multiple query parameters to the request
    pub fn query_params<K, V, I>(mut self, params: I) -> Self
    where
        K: ToString,
        V: ToString,
        I: IntoIterator<Item = (K, V)>,
    {
        for (key, value) in params {
            self.query_params.insert(key.to_string(), value.to_string());
        }
        self
    }

    /// Sets the request body from a serializable type
    pub fn body<T: Serialize>(mut self, body: &T) -> Result<Self, serde_json::Error> {
        self.body = Some(serde_json::to_value(body)?);
        Ok(self)
    }

    /// Sets the request body directly from a JSON Value
    pub fn json_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// Builds the complete endpoint URL with query parameters
    pub fn build_url(&self, base_url: &str) -> String {
        let mut url = format!("{}{}", base_url, self.endpoint);

        if !self.query_params.is_empty() {
            let params: Vec<String> = self
                .query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push('?');
            url.push_str(&params.join("&"));
        }

        url
    }

    /// Returns the request body if set
    pub fn get_body(&self) -> Option<&Value> {
        self.body.as_ref()
    }
}
