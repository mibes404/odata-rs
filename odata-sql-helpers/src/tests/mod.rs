use crate::get_column_names;
use crate::tests::test_model::Model;
use sea_orm::ModelTrait;

pub mod test_model;

#[test]
fn can_get_column_names_from_entity() {
    let columns = get_column_names::<<Model as ModelTrait>::Entity>();
    assert!(columns.contains_key("id"));
    assert!(columns.contains_key("name"));
}
