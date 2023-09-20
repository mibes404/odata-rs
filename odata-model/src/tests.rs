use super::*;
use rust_decimal_macros::dec;

#[test]
fn can_construct_an_url_from_and_endpoint() {
    let mut endpoint = ODataEndpoint::new("http://services.odata.org", Some("V4"), "TripPinService");
    let url = Url::try_from(&mut endpoint).expect("Failed to construct an URL from the endpoint");
    assert_eq!(url.as_str(), "http://services.odata.org/V4/TripPinService/");
}

#[test]
fn can_interpret_service_document() {
    let mut endpoint = ODataEndpoint::new("http://services.odata.org", Some("V4"), "TripPinService");
    let json = r#"{
            "@odata.context": "http://services.odata.org/V4/TripPinService/$metadata",
            "value": [
                {
                    "name": "Photos",
                    "kind": "EntitySet",
                    "url": "Photos"
                },
                {
                    "name": "People",
                    "kind": "EntitySet",
                    "url": "People"
                },
                {
                    "name": "Airlines",
                    "kind": "EntitySet",
                    "url": "Airlines"
                },
                {
                    "name": "Airports",
                    "kind": "EntitySet",
                    "url": "Airports"
                },
                {
                    "name": "Me",
                    "kind": "Singleton",
                    "url": "Me"
                },
                {
                    "name": "GetNearestAirport",
                    "kind": "FunctionImport",
                    "url": "GetNearestAirport"
                }
            ]
        }"#;

    let service_document: ServiceDocument =
        serde_json::from_str(json).expect("Failed to deserialize the service document");

    endpoint.enrich(service_document);

    assert_eq!(
        endpoint.odata_context,
        Some("http://services.odata.org/V4/TripPinService/$metadata".to_string())
    );
    assert_eq!(endpoint.resources.len(), 6);
}

#[test]
fn can_create_a_resource_from_a_url() {
    let url = "People('russellwhyte')";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "People");
    assert_eq!(resource.entity.key.unwrap().to_string(), "russellwhyte")
}

#[test]
fn can_create_a_resource_from_a_url_with_quotes() {
    let url = "People('O''Neil')";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "People");
    assert_eq!(resource.entity.key.unwrap().to_string(), "O'Neil")
}

#[test]
fn can_create_a_resource_from_a_url_with_escaped_characters() {
    let url = "People%28%27O%27%27Neil%27%29";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "People");
    assert_eq!(resource.entity.key.unwrap().to_string(), "O'Neil")
}

#[test]
fn can_create_a_resource_from_a_url_with_a_numeric_key() {
    let url = "Categories(1)";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Categories");
    assert_eq!(resource.entity.key.unwrap().to_string(), "1")
}

#[test]
fn can_create_a_resource_from_a_url_with_a_query_option() {
    let url = "ProductsByColor(color=@color)?@color='red'";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "ProductsByColor");
    let key = resource.entity.key.unwrap();
    assert_eq!(key.to_string(), "color=@color");
    if let Key::KeyValue((key, value)) = key {
        assert_eq!(key, "color");
        assert_eq!(value.to_string(), "@color");
    }
}

#[test]
fn can_create_a_resource_from_a_url_with_a_property() {
    let url = "People('russellwhyte')/FirstName";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "People");
    assert_eq!(resource.entity.key.unwrap().to_string(), "russellwhyte");
    assert_eq!(resource.property.unwrap(), "FirstName");
    assert!(resource.operation.is_none());
}

#[test]
fn can_create_a_resource_from_a_url_with_a_property_value() {
    let url = "People('russellwhyte')/FirstName/$value";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "People");
    assert_eq!(resource.entity.key.unwrap().to_string(), "russellwhyte");
    assert_eq!(resource.property.unwrap(), "FirstName");
    assert_eq!(resource.operation.unwrap(), Operation::Value);
}

#[test]
fn can_create_a_resource_from_a_url_with_a_count_operation() {
    let url = "People/$count";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "People");
    assert_eq!(resource.operation.unwrap(), Operation::Count);
}

#[test]
fn can_create_a_resource_from_a_url_with_related_entities() {
    let url = "People('russellwhyte')/Friends('scottketchum')/AddressInfo";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "People");
    assert_eq!(resource.relationships.len(), 1);
    let relationship = &resource.relationships[0];
    assert_eq!(relationship.name, "Friends");
    assert_eq!(relationship.key.as_ref().unwrap().to_string(), "scottketchum");
    assert_eq!(resource.property.unwrap(), "AddressInfo");
}

#[test]
fn can_create_a_resource_from_a_url_with_a_search_operation() {
    let url = "People?$search=russellwhyte";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "People");
    assert_eq!(resource.search.unwrap(), "russellwhyte");
}

#[test]
fn can_create_a_resource_from_a_url_with_a_filter_eq_operation() {
    let url = "Products?$filter=Name eq 'Milk'";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 1);
    let (filter, _) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Name");
    assert_eq!(
        contents.operation,
        FilterOperation::Eq(Value::String("Milk".to_string()))
    );
}

#[test]
fn can_create_a_resource_from_a_url_with_a_filter_ne_operation() {
    let url = "Products?$filter=Name ne 'Milk'";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 1);
    let (filter, _) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Name");
    assert_eq!(
        contents.operation,
        FilterOperation::Ne(Value::String("Milk".to_string()))
    );
}

#[test]
fn can_create_a_resource_from_a_url_with_a_filter_eq_and_lt_operation() {
    let url = "Products?$filter=Name eq 'Milk' and Price lt 2.55";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 2);
    let (filter, chain) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Name");
    assert_eq!(
        contents.operation,
        FilterOperation::Eq(Value::String("Milk".to_string()))
    );
    assert_eq!(chain, &Some(Chain::And));
    let (filter, _) = &resource.filters.contents()[1];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Price");
    assert_eq!(contents.operation, FilterOperation::Lt(Value::Decimal(dec!(2.55))));
}

#[test]
fn can_create_a_resource_from_a_url_with_a_filter_eq_or_lt_operation() {
    let url = "Products?$filter=Name eq 'Milk' or Price lt 2.55";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 2);
    let (filter, chain) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Name");
    assert_eq!(chain, &Some(Chain::Or));
    assert_eq!(
        contents.operation,
        FilterOperation::Eq(Value::String("Milk".to_string()))
    );
    let (filter, _chain) = &resource.filters.contents()[1];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Price");
    assert_eq!(contents.operation, FilterOperation::Lt(Value::Decimal(dec!(2.55))));
}

#[test]
fn can_create_a_resource_from_a_url_with_a_space_in_value() {
    let url = "Products?$filter=Name eq 'Chocolate Milk'";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 1);
    let (filter, _) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Name");
    assert_eq!(
        contents.operation,
        FilterOperation::Eq(Value::String("Chocolate Milk".to_string()))
    );
}

#[test]
fn can_create_a_resource_from_a_url_with_multiple_spaces_in_value() {
    let url = "Products?$filter=Name eq 'Very nice Chocolate Milk'";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 1);
    let (filter, _) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Name");
    assert_eq!(
        contents.operation,
        FilterOperation::Eq(Value::String("Very nice Chocolate Milk".to_string()))
    );
}

#[test]
fn can_create_a_resource_from_a_url_with_a_filter_in_operation() {
    let url = "Products?$filter=Name in ('Milk', 'Butter', 'Cheese')";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 1);
    let (filter, _chain) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Name");
    assert_eq!(
        contents.operation,
        FilterOperation::In(vec![
            Value::String("Milk".to_string()),
            Value::String("Butter".to_string()),
            Value::String("Cheese".to_string())
        ])
    );
}

#[test]
fn can_create_a_resource_from_a_url_with_a_filter_in_operation_wo_spaces() {
    let url = "Products?$filter=Name in ('Milk','Butter','Cheese')";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 1);
    let (filter, _chain) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Name");
    assert_eq!(
        contents.operation,
        FilterOperation::In(vec![
            Value::String("Milk".to_string()),
            Value::String("Butter".to_string()),
            Value::String("Cheese".to_string())
        ])
    );
}

#[test]
fn can_create_a_resource_from_a_url_with_a_filter_in_numbers() {
    let url = "Products?$filter=Price in (1,2,3)";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 1);
    let (filter, _chain) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert_eq!(contents.field, "Price");
    assert_eq!(
        contents.operation,
        FilterOperation::In(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3),])
    );
}

#[test]
fn can_create_a_resource_from_a_url_with_a_not_function_filter() {
    let url = "Products?$filter=not endswith(Name,'ilk')";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 1);
    let (filter, _chain) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert!(contents.not);
    assert_eq!(
        contents.operation,
        FilterOperation::Function("endswith(Name,'ilk')".to_string())
    );
}

#[test]
fn can_create_a_resource_from_a_url_with_a_has_filter() {
    let url = "Products?$filter=style has Sales.Pattern'Yellow'";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "Products");
    assert_eq!(resource.filters.len(), 1);
    let (filter, _chain) = &resource.filters.contents()[0];
    let contents = filter.contents().unwrap();
    assert!(!contents.not);
    assert_eq!(contents.field, "style");
    assert_eq!(
        contents.operation,
        FilterOperation::Has("Sales.Pattern'Yellow'".to_string())
    );
}

#[test]
fn can_create_a_resource_with_a_logical_filter() {
    let url = "People?$filter=(not(contains(FirstName,'Q')) or (Gender eq Microsoft.OData.SampleService.Models.TripPin.PersonGender'Male')) and not(LastName eq 'Ketchum')";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.filters.0.len(), 2);
    let (filter, chain) = &resource.filters.contents()[0];
    let (not, nested) = filter.nested().unwrap();
    assert!(!not);
    assert_eq!(chain, &Some(Chain::And));
    assert_eq!(nested.0.len(), 2);

    // First filter
    let (filter, _chain) = &nested.0[0];
    let FieldFilterContents {
        not,
        field: _field,
        operation,
    } = filter.contents().unwrap();
    assert!(not);
    assert_eq!(
        operation,
        &FilterOperation::Function("contains(FirstName,'Q')".to_string())
    );

    // Second filter
    let (filter, _chain) = &nested.0[1];
    let FieldFilterContents { not, field, operation } = filter.contents().unwrap();
    assert!(!not);
    assert_eq!(field, "Gender");
    assert_eq!(
        operation,
        &FilterOperation::Eq(Value::String(
            "Microsoft.OData.SampleService.Models.TripPin.PersonGender'Male'".to_string()
        ))
    );

    // Third filter
    let (filter, _chain) = &resource.filters.contents()[1];
    let FieldFilterContents { not, field, operation } = filter.contents().unwrap();
    assert!(not);
    assert_eq!(field, "LastName");
    assert_eq!(operation, &FilterOperation::Eq(Value::String("Ketchum".to_string())));
}

#[test]
fn can_create_a_resource_from_a_url_with_format() {
    let url = "People?$format=application/json;odata.metadata=minimal;odata.streaming=true";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.entity.name, "People");
    assert_eq!(resource.requested_format.format, "application/json");
    assert_eq!(resource.requested_format.metadata, ODataMetaData::Minimal);
    assert!(resource.requested_format.streaming);
}

#[test]
fn can_parse_correct_top_and_skip_values() {
    let url = "People?$top=10&$skip=20";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.top, Some(10));
    assert_eq!(resource.skip, Some(20));
}

#[test]
fn can_detect_invalid_top_or_skip_values() {
    let url = "People?$top=-10&$skip=20";
    let resource = ODataResource::try_from(url);
    assert!(resource.is_err());

    let url = "People?$top=10&$skip=-20";
    let resource = ODataResource::try_from(url);
    assert!(resource.is_err());
}

#[test]
fn can_parse_orderby() {
    let url = "People?$orderby=BaseRate asc";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.order_by.len(), 1);
    let OrderBy { field, direction } = &resource.order_by[0];
    assert_eq!(field, "BaseRate");
    assert_eq!(direction, &OrderByDirection::Asc);
}

#[test]
fn can_parse_orderby_with_two_fields() {
    let url = "People?$orderby=Rating desc,BaseRate";
    let resource = ODataResource::try_from(url).expect("Failed to create a resource from the URL");
    assert_eq!(resource.order_by.len(), 2);
    let OrderBy { field, direction } = &resource.order_by[0];
    assert_eq!(field, "Rating");
    assert_eq!(direction, &OrderByDirection::Desc);

    let OrderBy { field, direction } = &resource.order_by[1];
    assert_eq!(field, "BaseRate");
    assert_eq!(direction, &OrderByDirection::Asc);
}
