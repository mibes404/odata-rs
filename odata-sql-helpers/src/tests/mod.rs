use crate::tests::test_model::Model;
use crate::{get_column_names, WithODataExt};
use odata_model::resource::ODataResource;
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
        r#"SELECT "cake"."id", "cake"."first_name", "cake"."last_name", "cake"."doc" FROM "cake" WHERE LOWER("id") LIKE '%john%' OR LOWER("first_name") LIKE '%john%' OR LOWER("last_name") LIKE '%john%' OR LOWER("doc") LIKE '%john%'"#,
        query
    );
}
