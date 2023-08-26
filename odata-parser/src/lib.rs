use odata_model::{error::ODataResult, ODataResource};

pub use odata_model;

pub fn parse_url(url: &str) -> ODataResult<ODataResource> {
    let resource = ODataResource::try_from(url)?;
    Ok(resource)
}
