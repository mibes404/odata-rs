//! Reflect on the SeaOrm table definition and generate the EntityType from it.

use crate::get_column_names;
use odata_edm::edm::EntityType;
use sea_orm::EntityTrait;

pub fn into_entity_type<E>() -> EntityType
where
    E: EntityTrait,
{
    let columns = get_column_names::<E>();
    let e = E::default();
    let table_name = e.table_name();
    let mut et = EntityType::new(table_name.to_string());

    for column in columns.keys() {
        et.add_property(column.to_string(), "Edm.String".to_string());
    }

    et
}

#[cfg(test)]
mod tests {
    use odata_edm::edm::Property;
    use sea_orm::ModelTrait;

    use super::*;
    use crate::tests::test_model::Model;

    #[test]
    fn can_generate_edm_entity_from_model() {
        let et = into_entity_type::<<Model as ModelTrait>::Entity>();
        assert_eq!("users", et.name);

        let properties = et.property.expect("properties");
        assert_eq!(4, properties.len());

        let id = get_property("id", &properties).expect("id");
        assert_eq!("Edm.String", id._type);
    }

    fn get_property<'p>(key: &str, properties: &'p [Property]) -> Option<&'p Property> {
        properties.iter().find(|p| p.name == key)
    }
}
