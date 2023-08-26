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
