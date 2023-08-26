pub mod error;

pub use odata_model;

use error::ODataClientResult;
use odata_model::{ODataEndpoint, ServiceDocument};
use url::Url;

pub struct ODataClient {
    client: reqwest::Client,
    url: Url,
}

impl ODataClient {
    pub async fn init_with(endpoint: &mut ODataEndpoint) -> ODataClientResult<Self> {
        let client = reqwest::Client::new();
        let url: Url = endpoint.try_into()?;

        let service_document: ServiceDocument = client.get(url.clone()).send().await?.json().await?;
        endpoint.enrich(service_document);

        Ok(Self { client, url })
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}
