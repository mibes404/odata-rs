use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, State},
    response::IntoResponse,
};
use http::{request::Parts, StatusCode};
use odata_model::{model::ODataModel, resource::ODataResource};

pub mod response;

/// Extracts a [`ODataResource`] from the request.
pub struct ExtractODataResource(pub ODataResource);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractODataResource
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let uri = &parts.uri;
        if let Ok(odata_resource) = ODataResource::try_from(uri) {
            Ok(ExtractODataResource(odata_resource))
        } else {
            Err((StatusCode::BAD_REQUEST, "request is not a valid OData resource"))
        }
    }
}

pub trait WithODataModelExt {
    fn odata_model(&self) -> &ODataModel;
}

pub async fn serve_edm<S>(State(state): State<Arc<S>>) -> impl IntoResponse
where
    S: WithODataModelExt,
{
    let odata_model = state.odata_model();
    let edm = odata_model.edm();
    let xml = quick_xml::se::to_string(edm).expect("Failed to serialize EDM");

    http::Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/xml")
        .body(xml)
        .unwrap()
}
