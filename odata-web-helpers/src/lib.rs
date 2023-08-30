use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::{request::Parts, StatusCode};
use odata_model::resource::ODataResource;

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
