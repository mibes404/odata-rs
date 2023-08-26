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
    pub key: Option<Key>,
}

#[derive(Default)]
pub enum ODataResourceKind {
    #[default]
    EntitySet,
    Singleton,
    FunctionImport,
    ServiceDocument,
}

pub enum Key {
    String(String),
    Number(i32),
    KeyValue((String, Value)),
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::String(value) => write!(f, "{}", value),
            Key::Number(value) => write!(f, "{}", value),
            Key::KeyValue((name, value)) => write!(f, "{}={}", name, value),
        }
    }
}

pub enum Value {
    String(String),
    Number(i32),
    QueryOption(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(value) => write!(f, "{}", value),
            Value::Number(value) => write!(f, "{}", value),
            Value::QueryOption(value) => write!(f, "@{}", value),
        }
    }
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

/// Extract the name and key from a resource name, e.g. People('O''Neil') -> (People, Some(O'Neil))
fn extract_name_and_key(name: &str) -> (&str, Option<Key>) {
    if name.contains("('") && name.contains("')") {
        let mut parts = name.split("('");
        let name = parts.next().unwrap();
        let key = parts.next().unwrap();
        let key = key.trim_end_matches("')");
        let key = key.replace("''", "'");

        return (name, Some(Key::String(key)));
    }

    if name.contains('(') && name.contains(')') {
        let mut parts = name.split('(');
        let name = parts.next().unwrap();
        let key = parts.next().unwrap();
        let key = key.trim_end_matches(')');

        if key.contains('=') {
            let mut parts = key.split('=');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();

            if value.starts_with('\'') && value.ends_with('\'') {
                let value = value.trim_start_matches('\'').trim_end_matches('\'');
                return (
                    name,
                    Some(Key::KeyValue((key.to_string(), Value::String(value.to_string())))),
                );
            }

            if value.starts_with('@') {
                let value = value.trim_start_matches('@');
                return (
                    name,
                    Some(Key::KeyValue((key.to_string(), Value::QueryOption(value.to_string())))),
                );
            }

            if let Ok(num) = value.parse::<i32>() {
                return (name, Some(Key::KeyValue((key.to_string(), Value::Number(num)))));
            }

            return (name, None);
        }

        if let Ok(num) = key.parse::<i32>() {
            return (name, Some(Key::Number(num)));
        }
    }

    (name, None)
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
        assert_eq!(resource.key.unwrap().to_string(), "russellwhyte")
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_quotes() {
        let url = "http://services.odata.org/V4/TripPinService/People('O''Neil')";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.name, "People");
        assert_eq!(resource.key.unwrap().to_string(), "O'Neil")
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_escaped_characters() {
        let url = "http://services.odata.org/V4/TripPinService/People%28%27O%27%27Neil%27%29";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.name, "People");
        assert_eq!(resource.key.unwrap().to_string(), "O'Neil")
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_a_numeric_key() {
        let url = "http://host/service/Categories(1)";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.name, "Categories");
        assert_eq!(resource.key.unwrap().to_string(), "1")
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_a_query_option() {
        let url = "http://host/service/ProductsByColor(color=@color)?@color='red'";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.name, "ProductsByColor");
        let key = resource.key.unwrap();
        assert_eq!(key.to_string(), "color=@color");
        if let Key::KeyValue((key, value)) = key {
            assert_eq!(key, "color");
            assert_eq!(value.to_string(), "@color");
        }
    }
}
