//! Reflect on the SeaOrm table definition and generate the EntityType from it.

use crate::get_column_names;
use odata_edm::edm::EntityType;
use odata_model::model::ODataModel;
use sea_orm::{ColumnType, EntityTrait};

pub fn into_entity_type<E>() -> EntityType
where
    E: EntityTrait,
{
    let (p_keys, columns) = get_column_names::<E>();
    let e = E::default();
    let table_name = e.table_name();

    let mut et = EntityType::new(table_name.to_string());
    et.set_key(p_keys.iter());

    for (key, value) in columns.iter() {
        // Get the OData Property type from the SeaOrm column definition
        let c_ref = match value.def.get_column_type() {
            ColumnType::Char(_) => "Edm.String",
            ColumnType::String(_) => "Edm.String",
            ColumnType::Integer => "Edm.Int32",
            ColumnType::BigInteger => "Edm.Int64",
            ColumnType::SmallInteger => "Edm.Int16",
            ColumnType::TinyInteger => "Edm.Byte",
            ColumnType::Unsigned => "Edm.Int32",
            ColumnType::BigUnsigned => "Edm.Int64",
            ColumnType::SmallUnsigned => "Edm.Int16",
            ColumnType::TinyUnsigned => "Edm.Byte",
            ColumnType::Float => "Edm.Decimal",
            ColumnType::Double => "Edm.Decimal",
            ColumnType::Decimal(_) => "Edm.Decimal",
            ColumnType::Money(_) => "Edm.Decimal",
            ColumnType::Boolean => "Edm.Boolean",
            ColumnType::DateTime => "Edm.DateTimeOffset",
            ColumnType::Date => "Edm.Date",
            ColumnType::Time => "Edm.TimeOfDay",
            ColumnType::Timestamp => "Edm.DateTimeOffset",
            ColumnType::Year(_) => "Edm.Int32",
            ColumnType::Binary(_) => "Edm.Binary",
            ColumnType::VarBinary(_) => "Edm.Binary",
            ColumnType::JsonBinary => "Edm.Binary",
            ColumnType::Json => "Edm.String",
            ColumnType::Text => "Edm.String",
            ColumnType::Enum { name: _, variants: _ } => "Edm.String",
            ColumnType::Uuid => "Edm.Guid",
            ColumnType::Array(_) => "Edm.String",
            ColumnType::Custom(_) => "Edm.String",
            ColumnType::Inet => "Edm.String",
            ColumnType::Cidr => "Edm.String",
            ColumnType::MacAddr => "Edm.String",
            _ => "Edm.String",
        };
        et.add_property(key.to_string(), c_ref.to_string());
    }

    et
}

pub fn model_with_entity<E>(model: ODataModel) -> ODataModel
where
    E: EntityTrait,
{
    let et = into_entity_type::<E>();
    model.with_entity_type(et)
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
        assert_eq!("Edm.Int32", id._type);

        let key = et.key.expect("key");
        assert_eq!(1, key.len());
        assert_eq!("id", key[0].property_ref.as_ref().expect("property_ref")[0].name);
    }

    fn get_property<'p>(key: &str, properties: &'p [Property]) -> Option<&'p Property> {
        properties.iter().find(|p| p.name == key)
    }

    #[test]
    fn can_build_odata_model_from_db() {
        let model = ODataModel::default();
        let model = model_with_entity::<<Model as ModelTrait>::Entity>(model);
        let resource = model.get_resource("users").expect("users");
        assert_eq!("users", resource.entity.name);

        let et = model.get_entity_type(resource).expect("entity_type");
        assert_eq!("users", et.name);
    }
}
