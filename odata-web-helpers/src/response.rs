//! Response helpers for OData web services.

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::Value;

const ODATA_VERSION_HEADER: &str = "OData-Version";
const ETAG_HEADER: &str = "ETag";
const ODATA_VERSION: &str = "4.0";

/// A response that will be serialized as OData JSON.
pub struct ODataResponse<T>
where
    T: Serialize,
{
    body: T,
    e_tag: Option<String>,
}

impl<T> ODataResponse<T>
where
    T: Serialize,
{
    pub fn new(body: T) -> Self {
        Self { body, e_tag: None }
    }

    pub fn with_etag(mut self, e_tag: String) -> Self {
        self.e_tag = Some(e_tag);
        self
    }
}

fn build_odata_body<T>(body: T) -> Json<Value>
where
    T: Serialize,
{
    let mut body: Value = serde_json::to_value(body).expect("failed to serialize response body");
    if body.is_object() {
        let body = body.as_object_mut().unwrap();
        if !body.contains_key("@odata.context") {
            body.insert("@odata.context".to_string(), Value::String("$metadata".to_string()));
        }
    }

    Json(body)
}

impl<T> IntoResponse for ODataResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = build_odata_body(self.body);
        let mut res = body.into_response();
        let headers = res.headers_mut();
        headers.insert(ODATA_VERSION_HEADER, ODATA_VERSION.parse().unwrap());

        if let Some(e_tag) = self.e_tag {
            headers.insert(ETAG_HEADER, e_tag.parse().expect("invalid ETag header value"));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_context_to_body() {
        let json = serde_json::json!({
            "foo": "bar"
        });

        let body = build_odata_body(json);
        let body = body.0;
        assert!(body.is_object());
        let body = body.as_object().unwrap();
        assert!(body.contains_key("@odata.context"));
    }
}
