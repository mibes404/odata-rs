pub mod error;

use error::ODataError;
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use url::Url;

pub struct ODataEndpoint {
    pub base_url: String,
    pub version: String,
    pub service: String,
    pub resources: Vec<ODataResource>,
    pub odata_context: Option<String>,
}

impl ODataEndpoint {
    pub fn new(base_url: &str, version: &str, service: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            version: version.to_string(),
            service: service.to_string(),
            resources: Vec::new(),
            odata_context: None,
        }
    }

    pub fn enrich(&mut self, service_document: ServiceDocument) {
        self.odata_context = Some(service_document.context);
        self.resources = service_document.value.into_iter().map(|value| value.into()).collect();
    }
}

impl TryFrom<&mut ODataEndpoint> for Url {
    type Error = url::ParseError;

    fn try_from(endpoint: &mut ODataEndpoint) -> Result<Self, Self::Error> {
        let uri = format!("{}/{}/{}/", endpoint.base_url, endpoint.version, endpoint.service);
        Url::parse(&uri)
    }
}

pub struct ODataResource {
    pub name: String,
    pub kind: ODataResourceKind,
    pub url: String,
    pub title: Option<String>,
    pub key: Option<String>,
}

#[derive(Default)]
pub enum ODataResourceKind {
    #[default]
    EntitySet,
    Singleton,
    FunctionImport,
    ServiceDocument,
}

impl TryFrom<&str> for ODataResource {
    type Error = ODataError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value)?;
        match url.path_segments() {
            None => Err(ODataError::IncompletePath),
            Some(parts) => {
                let name = parts.last().unwrap().to_string();
                let name = percent_decode_str(&name).decode_utf8_lossy();
                let (name, key) = extract_name_and_key(&name);
                Ok(Self {
                    name: name.to_string(),
                    kind: ODataResourceKind::EntitySet,
                    url: value.to_string(),
                    title: None,
                    key,
                })
            }
        }
    }
}

fn extract_name_and_key(name: &str) -> (&str, Option<String>) {
    if name.contains("('") && name.contains("')") {
        let mut parts = name.split("('");
        let name = parts.next().unwrap();
        let key = parts.next().unwrap();
        let key = key.trim_end_matches("')");
        let key = key.replace("''", "'");

        (name, Some(key))
    } else {
        (name, None)
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

impl From<ServiceDocumentValue> for ODataResource {
    fn from(value: ServiceDocumentValue) -> Self {
        Self {
            name: value.name,
            kind: match value.kind {
                Some(kind) => match kind.as_str() {
                    "Singleton" => ODataResourceKind::Singleton,
                    "FunctionImport" => ODataResourceKind::FunctionImport,
                    "ServiceDocument" => ODataResourceKind::ServiceDocument,
                    _ => ODataResourceKind::EntitySet,
                },
                None => ODataResourceKind::EntitySet,
            },
            url: value.url,
            title: value.title,
            key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_an_url_from_and_endpoint() {
        let mut endpoint = ODataEndpoint::new("http://services.odata.org", "V4", "TripPinService");
        let url = Url::try_from(&mut endpoint).expect("Failed to construct an URL from the endpoint");
        assert_eq!(url.as_str(), "http://services.odata.org/V4/TripPinService/");
    }

    #[test]
    fn can_interpret_service_document() {
        let mut endpoint = ODataEndpoint::new("http://services.odata.org", "V4", "TripPinService");
        let json = r#"{
            "@odata.context": "http://services.odata.org/V4/TripPinService/$metadata",
            "value": [
                {
                    "name": "Photos",
                    "kind": "EntitySet",
                    "url": "Photos"
                },
                {
                    "name": "People",
                    "kind": "EntitySet",
                    "url": "People"
                },
                {
                    "name": "Airlines",
                    "kind": "EntitySet",
                    "url": "Airlines"
                },
                {
                    "name": "Airports",
                    "kind": "EntitySet",
                    "url": "Airports"
                },
                {
                    "name": "Me",
                    "kind": "Singleton",
                    "url": "Me"
                },
                {
                    "name": "GetNearestAirport",
                    "kind": "FunctionImport",
                    "url": "GetNearestAirport"
                }
            ]
        }"#;

        let service_document: ServiceDocument =
            serde_json::from_str(json).expect("Failed to deserialize the service document");

        endpoint.enrich(service_document);

        assert_eq!(
            endpoint.odata_context,
            Some("http://services.odata.org/V4/TripPinService/$metadata".to_string())
        );
        assert_eq!(endpoint.resources.len(), 6);
    }

    #[test]
    fn can_create_a_resource_from_a_url() {
        let url = "http://services.odata.org/V4/TripPinService/People('russellwhyte')";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.name, "People");
        assert_eq!(resource.key, Some("russellwhyte".to_string()))
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_quotes() {
        let url = "http://services.odata.org/V4/TripPinService/People('O''Neil')";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.name, "People");
        assert_eq!(resource.key, Some("O'Neil".to_string()))
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_escaped_characters() {
        let url = "http://services.odata.org/V4/TripPinService/People%28%27O%27%27Neil%27%29";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.name, "People");
        assert_eq!(resource.key, Some("O'Neil".to_string()))
    }
}
