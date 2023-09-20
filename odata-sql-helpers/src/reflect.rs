//! Reflect on the SeaOrm table definition and generate the EntityType from it.

use crate::get_column_names;
use odata_edm::edm::EntityType;
use sea_orm::{ColumnType, EntityTrait};

pub fn into_entity_type<E>() -> EntityType
where
    E: EntityTrait,
{
    let columns = get_column_names::<E>();
    let e = E::default();
    let table_name = e.table_name();
    let mut et = EntityType::new(table_name.to_string());

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
    }

    fn get_property<'p>(key: &str, properties: &'p [Property]) -> Option<&'p Property> {
        properties.iter().find(|p| p.name == key)
    }
}
