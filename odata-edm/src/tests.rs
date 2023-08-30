use super::edm::Edmx;
use quick_xml::de::from_str;

#[test]
fn can_deserialize_sample() {
    const SAMPLE_EDM: &str = include_str!("../test_data/sample_edm.xml");
    let edmx: Edmx = from_str(SAMPLE_EDM).expect("Failed to deserialize sample EDM");

    assert_eq!(edmx.version, "4.0");
    let schema = edmx.data_services.schema;
    assert_eq!(schema.len(), 1);

    let schema = &schema[0];
    assert_eq!(schema.entity_type.as_ref().unwrap().len(), 9);

    let first_entity_type = &schema.entity_type.as_ref().unwrap()[0];
    assert_eq!(first_entity_type.name, "Photo");
    assert_eq!(first_entity_type.annotation.as_ref().unwrap().len(), 1);
    assert_eq!(first_entity_type.key.as_ref().unwrap().len(), 1);

    let first_key = &first_entity_type.key.as_ref().unwrap()[0];
    assert_eq!(first_key.property_ref.as_ref().unwrap().len(), 1);
    assert_eq!(first_key.property_ref.as_ref().unwrap()[0].name, "Id");

    let first_annotation = &first_entity_type.annotation.as_ref().unwrap()[0];
    assert_eq!(first_annotation.term, "Org.OData.Core.V1.AcceptableMediaTypes");

    let collection = first_annotation.collection.as_ref().unwrap();
    assert_eq!(collection.len(), 1);
    assert_eq!(collection[0].string.as_ref().unwrap()[0], "image/jpeg");
}

#[test]
fn can_serialize_example() {
    const SAMPLE_EDM: &str = include_str!("../test_data/sample_edm.xml");
    let edmx: Edmx = from_str(SAMPLE_EDM).expect("Failed to deserialize sample EDM");

    let xml_header = r#"<?xml version="1.0" encoding="utf-8"?>"#;
    let xml = quick_xml::se::to_string(&edmx).expect("Failed to serialize sample EDM");
    let xml = format!("{}\n{}", xml_header, xml);
    eprintln!("{}", xml);

    let _edmx: Edmx = from_str(&xml).expect("Failed to deserialize sample EDM");
}
