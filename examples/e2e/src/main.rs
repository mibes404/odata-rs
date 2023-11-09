use anyhow::Result;
use axum::{extract::State, routing::get, Router};
use odata_model::model::ODataModel;
use odata_sql_helpers::{reflect::model_with_entity, WithODataExt};
use odata_web_helpers::{response::ODataResponse, serve_edm, ExtractODataResource, WithODataModelExt};
use sea_orm::{DatabaseBackend, DatabaseConnection, EntityTrait, MockDatabase, ModelTrait};
use serde_json::{json, Value};
use std::sync::Arc;
use test_model::Model as UserModel;

mod test_model;

#[derive(Default, Clone)]
struct MockedUserDB;

impl MockedUserDB {
    fn conn(&self) -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // First query result
                vec![UserModel {
                    id: 1,
                    first_name: "Bill".to_owned(),
                    last_name: "Gates".to_owned(),
                    doc: serde_json::json!({ "foo": "bar" }),
                }],
            ])
            .into_connection()
    }
}

struct AppState {
    db: MockedUserDB,
    model: ODataModel,
}

impl WithODataModelExt for AppState {
    fn odata_model(&self) -> &ODataModel {
        &self.model
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = MockedUserDB;
    let model = ODataModel::default();
    let model = model_with_entity::<<UserModel as ModelTrait>::Entity>(model);
    let app_state = Arc::new(AppState { db, model });

    // build our application with a single route
    // try with: curl localhost:8080/V4/UserService/Users
    let app = Router::new()
        .route("/V4/UserService/Users/$metadata", get(serve_edm))
        .route("/V4/UserService/Users", get(parse_odata_request_handler))
        .with_state(app_state);

    // run it with hyper on localhost:8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("server failed to start");

    Ok(())
}

async fn parse_odata_request_handler(
    State(state): State<Arc<AppState>>,
    ExtractODataResource(resource): ExtractODataResource,
) -> ODataResponse<Value> {
    let conn = state.db.conn();
    let query_results = test_model::Entity::find()
        .with_odata_resource(&resource)
        .into_json()
        .all(&conn)
        .await
        .expect("Failed to execute query");

    let body = json!(query_results);

    println!("model: {:?}", state.model);
    ODataResponse::<serde_json::Value>::new(body, "users", &state.model)
}
