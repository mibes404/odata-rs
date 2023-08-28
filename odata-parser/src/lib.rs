use odata_model::{error::ODataResult, resource::ODataResource, ODataEndpoint};

pub use odata_model;

/// Parse an OData v4 request Url
/// ```rust
/// use odata_parser::parse_url;
/// use odata_model::{ODataEndpoint, resource::{ODataResource, ODataResourceKind}};
///
/// let endpoint = ODataEndpoint::new("http://services.odata.org", Some("V4"), "TripPinService");
/// let url = "http://services.odata.org/V4/TripPinService/People('O''Neil')";
/// let resource = parse_url(&endpoint, url).expect("Failed to create a resource from the URL");
///
/// assert_eq!(resource.entity.name, "People");
/// assert_eq!(resource.entity.key.unwrap().to_string(), "O'Neil");
/// ```
pub fn parse_url(endpoint: &ODataEndpoint, url: &str) -> ODataResult<ODataResource> {
    let resource = endpoint.parse_resource(url)?;
    Ok(resource)
}
