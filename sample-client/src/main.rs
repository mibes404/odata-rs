use odata_client::{odata_model::ODataEndpoint, ODataClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut model = ODataEndpoint::new("http://services.odata.org", Some("V4"), "TripPinService");
    let client = ODataClient::init_with(&mut model).await?;

    println!("Found these resources @ {}", client.url());
    for resource in model.resources {
        println!("  - {}", resource.name);
    }

    Ok(())
}
