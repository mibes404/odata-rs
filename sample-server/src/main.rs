use odata_parser::{odata_model::ODataEndpoint, parse_url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = ODataEndpoint::new("http://services.odata.org", Some("V4"), "TripPinService");
    let url = "http://services.odata.org/V4/TripPinService/Products?$filter=Name eq 'Milk' and Price lt 2.55";
    let resource = parse_url(&endpoint, url).expect("Failed to create a resource from the URL");

    println!("Parsed the resource: {:?}", resource);

    Ok(())
}
