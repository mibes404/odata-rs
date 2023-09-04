use crate::tests::test_model::Model;
use crate::{get_column_names, WithODataExt};
use odata_model::resource::{ODataResource, OrderBy, OrderByDirection};
use sea_orm::{DbBackend, EntityTrait, ModelTrait, QueryTrait};

pub mod test_model;

#[test]
fn can_get_column_names_from_entity() {
    let columns = get_column_names::<<Model as ModelTrait>::Entity>();
    assert_eq!(vec!["id", "first_name", "last_name", "doc"], columns.keys());
}

fn build_query_with_filter(resource: &ODataResource) -> String {
    test_model::Entity::find()
        .with_odata_resource(resource)
        .build(DbBackend::Postgres)
        .to_string()
}

#[test]
fn can_generate_a_search_query() {
    let resource = ODataResource {
        search: Some("John".to_string()),
        ..Default::default()
    };

    let query = build_query_with_filter(&resource);
    assert_eq!(
        r#"SELECT "users"."id", "users"."first_name", "users"."last_name", "users"."doc" FROM "users" WHERE LOWER("id") LIKE '%john%' OR LOWER("first_name") LIKE '%john%' OR LOWER("last_name") LIKE '%john%' OR LOWER("doc") LIKE '%john%'"#,
        query
    );
}

#[test]
fn can_generate_order_by_query() {
    let resource = ODataResource {
        order_by: vec![
            OrderBy {
                field: "first_name".to_string(),
                direction: OrderByDirection::Desc,
            },
            OrderBy {
                field: "last_name".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let query = build_query_with_filter(&resource);
    assert_eq!(
        r#"SELECT "users"."id", "users"."first_name", "users"."last_name", "users"."doc" FROM "users" WHERE TRUE ORDER BY "first_name" DESC, "last_name" ASC"#,
        query
    );
}

#[test]
fn can_generate_an_and_query() {
    let resource = ODataResource::try_from("users?$filter=first_name eq 'John' and last_name eq 'Doe'")
        .expect("Failed to parse ODataResource");

    let query = build_query_with_filter(&resource);
    assert_eq!(
        r#"SELECT "users"."id", "users"."first_name", "users"."last_name", "users"."doc" FROM "users" WHERE "first_name" = 'John' AND "last_name" = 'Doe'"#,
        query
    );
}

#[test]
fn can_generate_an_or_query() {
    let resource = ODataResource::try_from("users?$filter=first_name eq 'John' or first_name eq 'Bill'")
        .expect("Failed to parse ODataResource");

    let query = build_query_with_filter(&resource);
    assert_eq!(
        r#"SELECT "users"."id", "users"."first_name", "users"."last_name", "users"."doc" FROM "users" WHERE "first_name" = 'John' OR "first_name" = 'Bill'"#,
        query
    );
}

#[test]
fn can_generate_a_combined_and_or_query() {
    let resource =
        ODataResource::try_from("users?$filter=first_name eq 'John' or first_name eq 'Bill' and last_name eq 'Doe'")
            .expect("Failed to parse ODataResource");

    let query = build_query_with_filter(&resource);
    assert_eq!(
        r#"SELECT "users"."id", "users"."first_name", "users"."last_name", "users"."doc" FROM "users" WHERE "first_name" = 'John' OR ("first_name" = 'Bill' AND "last_name" = 'Doe')"#,
        query
    );
}

#[test]
fn can_generate_a_query_with_groups() {
    let resource =
        ODataResource::try_from("users?$filter=(first_name eq 'John' or first_name eq 'Bill') and last_name eq 'Doe'")
            .expect("Failed to parse ODataResource");

    let query = build_query_with_filter(&resource);
    assert_eq!(
        r#"SELECT "users"."id", "users"."first_name", "users"."last_name", "users"."doc" FROM "users" WHERE ("first_name" = 'John' OR "first_name" = 'Bill') AND "last_name" = 'Doe'"#,
        query
    );
}
