use axum::{routing::get, Json, Router};
use odata_web_helpers::ExtractODataResource;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build our application with a single route
    // try with something like: http://localhost:8080/V4/TripPinService/Products?$filter=Name eq 'Milk' and Price lt 2.55
    let app = Router::new().route("/V4/TripPinService/Products", get(parse_odata_request_handler));

    // run it with hyper on localhost:8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("server failed to start");

    Ok(())
}

async fn parse_odata_request_handler(ExtractODataResource(odata_resource): ExtractODataResource) -> Json<Value> {
    let response = format!("{:?}", odata_resource);
    eprintln!("Parsed the resource: {response}");
    Json(json!({ "parsed resource": response }))
}
