use yaserde::de::from_str;

use super::edm::Edmx;

#[test]
fn can_deserialize_sample() {
    const SAMPLE_EDM: &str = include_str!("../test_data/sample_edm.xml");
    let edmx: Edmx = from_str(SAMPLE_EDM).expect("Failed to deserialize sample EDM");
    assert_eq!(edmx.version, "4.0");
    assert_eq!(edmx.data_services.schema.len(), 1);

    let schema = &edmx.data_services.schema[0];
    assert_eq!(schema.entity_type.len(), 9);

    let first_entity_type = &schema.entity_type[0];
    assert_eq!(first_entity_type.name, "Photo");
    assert_eq!(first_entity_type.annotation.len(), 1);
    assert_eq!(first_entity_type.key.len(), 1);

    let first_key = &first_entity_type.key[0];
    assert_eq!(first_key.property_ref.len(), 1);
    assert_eq!(first_key.property_ref[0].name, "Id");

    let first_annotation = &first_entity_type.annotation[0];
    assert_eq!(first_annotation.term, "Org.OData.Core.V1.AcceptableMediaTypes");
    assert_eq!(first_annotation.collection.len(), 1);
}
