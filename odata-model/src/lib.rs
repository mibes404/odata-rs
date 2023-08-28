pub mod error;
pub mod resource;

#[cfg(test)]
mod tests;

use error::{ODataError, ODataResult};
use percent_encoding::percent_decode_str;
use resource::*;
use serde::{Deserialize, Serialize};
use std::str::{FromStr, Split};
use url::Url;

pub struct ODataEndpoint {
    pub base_url: String,
    pub version: Option<String>,
    pub service: String,
    pub resources: Vec<ODataResource>,
    pub odata_context: Option<String>,
}

impl ODataEndpoint {
    pub fn new(base_url: &str, version: Option<&str>, service: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            version: version.map(|v| v.to_string()),
            service: service.to_string(),
            resources: Vec::new(),
            odata_context: None,
        }
    }

    pub fn enrich(&mut self, service_document: ServiceDocument) {
        self.odata_context = Some(service_document.context);
        self.resources = service_document.value.into_iter().map(|value| value.into()).collect();
    }

    pub fn parse_resource(&self, url: &str) -> ODataResult<ODataResource> {
        let url_path = url.trim_start_matches(self.to_string().as_str());
        let resource = ODataResource::try_from(url_path)?;
        Ok(resource)
    }
}

impl std::fmt::Display for ODataEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uri = if let Some(version) = &self.version {
            format!("{}/{}/{}/", self.base_url, version, self.service)
        } else {
            format!("{}/{}/", self.base_url, self.service)
        };

        write!(f, "{}", uri)
    }
}

impl TryFrom<&mut ODataEndpoint> for Url {
    type Error = url::ParseError;

    fn try_from(endpoint: &mut ODataEndpoint) -> Result<Self, Self::Error> {
        let uri = endpoint.to_string();
        Url::parse(&uri)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceDocument {
    #[serde(rename = "@odata.context")]
    pub context: String,
    pub value: Vec<ServiceDocumentValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceDocumentValue {
    pub name: String,
    pub kind: Option<String>,
    pub url: String,
    pub title: Option<String>,
}
