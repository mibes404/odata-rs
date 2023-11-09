//! Response helpers for OData web services.

use std::collections::HashMap;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use odata_model::model::ODataModel;
use serde::Serialize;
use serde_json::{Map, Value};

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
    context: Option<String>,
}

impl<T> ODataResponse<T>
where
    T: Serialize,
{
    pub fn new(body: T, entity_id: &str, using_model: &ODataModel) -> Self {
        let context = using_model.context_for_entity(entity_id);

        Self {
            body,
            e_tag: None,
            context,
        }
    }

    pub fn with_etag(mut self, e_tag: String) -> Self {
        self.e_tag = Some(e_tag);
        self
    }
}

fn build_odata_body<T>(body: T, context: Option<String>) -> Json<Value>
where
    T: Serialize,
{
    let mut body: Value = serde_json::to_value(body).expect("failed to serialize response body");
    let mut response: Map<String, Value> = Map::new();

    if body.is_object() {
        let body = body.as_object_mut().unwrap();
        if let Some(context) = context {
            if !body.contains_key("@odata.context") {
                body.insert("@odata.context".to_string(), Value::String(context));
            }
        }
        response = body.clone();
    } else if body.is_array() {
        if let Some(context) = context {
            response.insert("@odata.context".to_string(), Value::String(context));
        }
        response.insert("value".to_string(), body);
    }

    let response = serde_json::to_value(response).expect("failed to serialize response body");
    Json(response)
}

impl<T> IntoResponse for ODataResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = build_odata_body(self.body, self.context);
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

        let body = build_odata_body(json, Some("Foo".to_string()));
        let body = body.0;
        assert!(body.is_object());
        let body = body.as_object().unwrap();
        assert!(body.contains_key("@odata.context"));
    }
}
