pub mod error;

use std::str::Split;

use error::{ODataError, ODataResult};
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
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

pub struct ODataResource {
    pub entity: Entity,
    pub kind: ODataResourceKind,
    pub url: String,
    pub title: Option<String>,
    pub property: Option<String>,
    pub operation: Option<Operation>,
    pub relationships: Vec<Entity>,
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

pub struct Entity {
    pub name: String,
    pub key: Option<Key>,
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.key {
            Some(key) => write!(f, "{}({})", self.name, key),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Count,
    Value,
}

impl TryFrom<&str> for Operation {
    type Error = ODataError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "$count" => Ok(Self::Count),
            "$value" => Ok(Self::Value),
            _ => Err(ODataError::InvalidOperation),
        }
    }
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

const PARSE_PREFIX: &str = "http://services.odata.org/V4/TripPinService/";

impl TryFrom<&str> for ODataResource {
    type Error = ODataError;

    /// Try to create a resource from the path of an URL. The path is expected to start with the name of the resource.
    /// For example: People('russellwhyte')/FirstName
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim_start_matches('/');
        let value = format!("{PARSE_PREFIX}{value}");
        let url = Url::parse(&value)?;
        match url.path_segments() {
            None => Err(ODataError::IncompletePath),
            Some(mut parts) => {
                let Some(name) = parts.nth(2) else {
                    return Err(ODataError::IncompletePath);
                };

                let name = percent_decode_str(name).decode_utf8_lossy();
                let entity = extract_entity(&name);

                let mut part;
                let mut relationships = vec![];
                let mut property: Option<String> = None;
                let mut operation = None;

                loop {
                    part = interpret_next_part(&mut parts);

                    match part {
                        Some(NextPart::Part(part)) => {
                            if let Some(property) = property.take() {
                                // there was more to parse, so this isn't the end of the resource, i.e. not a property
                                relationships.push(extract_entity(&property));
                            }

                            property = Some(part.to_string());
                        }
                        Some(NextPart::Operation(part)) => {
                            operation = Some(part);
                        }
                        None => break,
                    };
                }

                Ok(Self {
                    entity,
                    kind: ODataResourceKind::EntitySet,
                    url: value.to_string(),
                    title: None,
                    property,
                    operation,
                    relationships,
                })
            }
        }
    }
}

enum NextPart<'s> {
    Part(&'s str),
    Operation(Operation),
}

fn interpret_next_part<'p>(parts: &'p mut Split<'_, char>) -> Option<NextPart<'p>> {
    let part = parts.next();

    part.map(|part| match Operation::try_from(part) {
        Ok(operation) => NextPart::Operation(operation),
        _ => NextPart::Part(part),
    })
}

/// Extract the name and key from a resource name, e.g. People('O''Neil') -> (People, Some(O'Neil))
fn extract_entity(name: &str) -> Entity {
    if name.contains("('") && name.contains("')") {
        let mut parts = name.split("('");
        let name = parts.next().unwrap();
        let key = parts.next().unwrap();
        let key = key.trim_end_matches("')");
        let key = key.replace("''", "'");

        return Entity {
            name: name.to_string(),
            key: Some(Key::String(key)),
        };
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
                return Entity {
                    name: name.to_string(),
                    key: Some(Key::KeyValue((key.to_string(), Value::String(value.to_string())))),
                };
            }

            if value.starts_with('@') {
                let value = value.trim_start_matches('@');
                return Entity {
                    name: name.to_string(),
                    key: Some(Key::KeyValue((key.to_string(), Value::QueryOption(value.to_string())))),
                };
            }

            if let Ok(num) = value.parse::<i32>() {
                return Entity {
                    name: name.to_string(),
                    key: Some(Key::KeyValue((key.to_string(), Value::Number(num)))),
                };
            }

            return Entity {
                name: name.to_string(),
                key: None,
            };
        }

        if let Ok(num) = key.parse::<i32>() {
            return Entity {
                name: name.to_string(),
                key: Some(Key::Number(num)),
            };
        }
    }

    Entity {
        name: name.to_string(),
        key: None,
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
            entity: extract_entity(&value.name),
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
            property: None,
            operation: None,
            relationships: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_an_url_from_and_endpoint() {
        let mut endpoint = ODataEndpoint::new("http://services.odata.org", Some("V4"), "TripPinService");
        let url = Url::try_from(&mut endpoint).expect("Failed to construct an URL from the endpoint");
        assert_eq!(url.as_str(), "http://services.odata.org/V4/TripPinService/");
    }

    #[test]
    fn can_interpret_service_document() {
        let mut endpoint = ODataEndpoint::new("http://services.odata.org", Some("V4"), "TripPinService");
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
        let url = "People('russellwhyte')";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.entity.name, "People");
        assert_eq!(resource.entity.key.unwrap().to_string(), "russellwhyte")
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_quotes() {
        let url = "People('O''Neil')";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.entity.name, "People");
        assert_eq!(resource.entity.key.unwrap().to_string(), "O'Neil")
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_escaped_characters() {
        let url = "People%28%27O%27%27Neil%27%29";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.entity.name, "People");
        assert_eq!(resource.entity.key.unwrap().to_string(), "O'Neil")
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_a_numeric_key() {
        let url = "Categories(1)";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.entity.name, "Categories");
        assert_eq!(resource.entity.key.unwrap().to_string(), "1")
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_a_query_option() {
        let url = "ProductsByColor(color=@color)?@color='red'";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.entity.name, "ProductsByColor");
        let key = resource.entity.key.unwrap();
        assert_eq!(key.to_string(), "color=@color");
        if let Key::KeyValue((key, value)) = key {
            assert_eq!(key, "color");
            assert_eq!(value.to_string(), "@color");
        }
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_a_property() {
        let url = "People('russellwhyte')/FirstName";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.entity.name, "People");
        assert_eq!(resource.entity.key.unwrap().to_string(), "russellwhyte");
        assert_eq!(resource.property.unwrap(), "FirstName");
        assert!(resource.operation.is_none());
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_a_property_value() {
        let url = "People('russellwhyte')/FirstName/$value";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.entity.name, "People");
        assert_eq!(resource.entity.key.unwrap().to_string(), "russellwhyte");
        assert_eq!(resource.property.unwrap(), "FirstName");
        assert_eq!(resource.operation.unwrap(), Operation::Value);
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_a_count_operation() {
        let url = "People/$count";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.entity.name, "People");
        assert_eq!(resource.operation.unwrap(), Operation::Count);
    }

    #[test]
    fn can_create_a_resource_from_a_url_with_related_entities() {
        let url = "People('russellwhyte')/Friends('scottketchum')/AddressInfo";
        let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
        assert_eq!(resource.entity.name, "People");
        assert_eq!(resource.relationships.len(), 1);
        let relationship = &resource.relationships[0];
        assert_eq!(relationship.name, "Friends");
        assert_eq!(relationship.key.as_ref().unwrap().to_string(), "scottketchum");
        assert_eq!(resource.property.unwrap(), "AddressInfo");
    }
}
