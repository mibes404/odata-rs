use odata_model::{error::ODataResult, ODataResource};

pub use odata_model;

/// Parse an OData v4 request Url
/// ```rust
/// use odata_parser::parse_url;
/// use odata_model::{ODataResource, ODataResourceKind};
///
/// let url = "http://services.odata.org/V4/TripPinService/People('O''Neil')";
/// let resource = parse_url(url).expect("Failed to create a resource from the URL");
///
/// assert_eq!(resource.name, "People");
/// assert_eq!(resource.key, Some("O'Neil".to_string()))
/// ```
pub fn parse_url(url: &str) -> ODataResult<ODataResource> {
    let resource = ODataResource::try_from(url)?;
    Ok(resource)
}
