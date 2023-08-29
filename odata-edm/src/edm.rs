//! This crate contains the serde annotated structures for the odata CSDL 4.01 schema using the quick-xml crate.
//! The generated structures are based on the XSD schema definition in https://raw.githubusercontent.com/oasis-tcs/odata-csdl-schemas/main/schemas/edmx.xsd
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Edmx")]
pub struct Edmx {
    #[serde(rename = "@Version")]
    pub version: String,
    #[serde(rename = "DataServices")]
    pub data_services: DataServices,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "DataServices")]
pub struct DataServices {
    #[serde(rename = "Schema")]
    pub schema: Option<Vec<Schema>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Schema")]
pub struct Schema {
    #[serde(rename = "@Namespace")]
    pub namespace: String,
    #[serde(rename = "EntityType")]
    pub entity_type: Option<Vec<EntityType>>,
    #[serde(rename = "ComplexType")]
    pub complex_type: Option<Vec<ComplexType>>,
    #[serde(rename = "EnumType")]
    pub enum_type: Option<Vec<EnumType>>,
    #[serde(rename = "TypeDefinition")]
    pub type_definition: Option<Vec<TypeDefinition>>,
    #[serde(rename = "Term")]
    pub term: Option<Vec<Term>>,
    #[serde(rename = "Annotations")]
    pub annotations: Option<Vec<Annotations>>,
    #[serde(rename = "Action")]
    pub action: Option<Vec<Action>>,
    #[serde(rename = "Function")]
    pub function: Option<Vec<Function>>,
    #[serde(rename = "EntityContainer")]
    pub entity_container: Option<Vec<EntityContainer>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "EntityType")]
pub struct EntityType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Key")]
    pub key: Option<Vec<Key>>,
    #[serde(rename = "Property")]
    pub property: Option<Vec<Property>>,
    #[serde(rename = "NavigationProperty")]
    pub navigation_property: Option<Vec<NavigationProperty>>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Key")]
pub struct Key {
    #[serde(rename = "PropertyRef")]
    pub property_ref: Option<Vec<PropertyRef>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "PropertyRef")]
pub struct PropertyRef {
    #[serde(rename = "@Name")]
    pub name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Property")]
pub struct Property {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "@Nullable")]
    pub nullable: Option<String>,
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,
    #[serde(rename = "MaxLength")]
    pub max_length: Option<String>,
    #[serde(rename = "Precision")]
    pub precision: Option<String>,
    #[serde(rename = "Scale")]
    pub scale: Option<String>,
    #[serde(rename = "Unicode")]
    pub unicode: Option<String>,
    #[serde(rename = "SRID")]
    pub srid: Option<String>,
    #[serde(rename = "ConcurrencyMode")]
    pub concurrency_mode: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "NavigationProperty")]
pub struct NavigationProperty {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "@Nullable")]
    pub nullable: Option<String>,
    #[serde(rename = "Partner")]
    pub partner: Option<String>,
    #[serde(rename = "ContainsTarget")]
    pub contains_target: Option<String>,
    #[serde(rename = "ReferentialConstraint")]
    pub referential_constraint: Option<ReferentialConstraint>,
    #[serde(rename = "OnDelete")]
    pub on_delete: Option<OnDelete>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "ReferentialConstraint")]
pub struct ReferentialConstraint {
    #[serde(rename = "Principal")]
    pub principal: Principal,
    #[serde(rename = "Dependent")]
    pub dependent: Dependent,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Principal")]
pub struct Principal {
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "PropertyRef")]
    pub property_ref: Option<Vec<PropertyRef>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Dependent")]
pub struct Dependent {
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "PropertyRef")]
    pub property_ref: Option<Vec<PropertyRef>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "OnDelete")]
pub struct OnDelete {
    #[serde(rename = "Action")]
    pub action: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Annotation")]
pub struct Annotation {
    #[serde(rename = "@Term")]
    pub term: String,
    #[serde(rename = "Qualifier")]
    pub qualifier: Option<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "@Target")]
    pub target: Option<String>,
    #[serde(rename = "String")]
    pub string: Option<String>,
    #[serde(rename = "Bool")]
    pub bool: Option<String>,
    #[serde(rename = "Int")]
    pub int: Option<String>,
    #[serde(rename = "Float")]
    pub float: Option<String>,
    #[serde(rename = "Decimal")]
    pub decimal: Option<String>,
    #[serde(rename = "DateTimeOffset")]
    pub date_time_offset: Option<String>,
    #[serde(rename = "Duration")]
    pub duration: Option<String>,
    #[serde(rename = "Guid")]
    pub guid: Option<String>,
    #[serde(rename = "Binary")]
    pub binary: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
    #[serde(rename = "Collection")]
    pub collection: Option<Vec<Collection>>,
    #[serde(rename = "Record")]
    pub record: Option<Vec<Record>>,
    #[serde(rename = "LabeledElement")]
    pub labeled_element: Option<Vec<LabeledElement>>,
    #[serde(rename = "Null")]
    pub null: Option<Vec<Null>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Collection")]
pub struct Collection {
    #[serde(rename = "String")]
    pub string: Option<Vec<String>>,
    #[serde(rename = "Bool")]
    pub bool: Option<Vec<bool>>,
    #[serde(rename = "Int")]
    pub int: Option<Vec<i32>>,
    #[serde(rename = "Float")]
    pub float: Option<Vec<f32>>,
    #[serde(rename = "Decimal")]
    pub decimal: Option<Vec<String>>,
    #[serde(rename = "DateTimeOffset")]
    pub date_time_offset: Option<Vec<String>>,
    #[serde(rename = "Duration")]
    pub duration: Option<Vec<String>>,
    #[serde(rename = "Guid")]
    pub guid: Option<Vec<String>>,
    #[serde(rename = "Binary")]
    pub binary: Option<Vec<String>>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
    #[serde(rename = "Collection")]
    pub collection: Option<Vec<Collection>>,
    #[serde(rename = "Record")]
    pub record: Option<Vec<Record>>,
    #[serde(rename = "LabeledElement")]
    pub labeled_element: Option<Vec<LabeledElement>>,
    #[serde(rename = "Null")]
    pub null: Option<Vec<Null>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Record")]
pub struct Record {
    #[serde(rename = "PropertyValue")]
    pub property_value: Option<Vec<PropertyValue>>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "PropertyValue")]
pub struct PropertyValue {
    #[serde(rename = "@Property")]
    pub property: String,
    #[serde(rename = "@Bool")]
    pub bool: Option<bool>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "NavigationPropertyPath")]
    pub navigation_property_path: Option<String>,
    #[serde(rename = "AnnotationPath")]
    pub annotation_path: Option<String>,
    #[serde(rename = "PropertyPath")]
    pub property_path: Option<String>,
    #[serde(rename = "Null")]
    pub null: Option<Vec<Null>>,
    #[serde(rename = "Record")]
    pub record: Option<Vec<Record>>,
    #[serde(rename = "Collection")]
    pub collection: Option<Vec<Collection>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "LabeledElement")]
pub struct LabeledElement {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "String")]
    pub string: Option<String>,
    #[serde(rename = "Bool")]
    pub bool: Option<String>,
    #[serde(rename = "Int")]
    pub int: Option<String>,
    #[serde(rename = "Float")]
    pub float: Option<String>,
    #[serde(rename = "Decimal")]
    pub decimal: Option<String>,
    #[serde(rename = "DateTimeOffset")]
    pub date_time_offset: Option<String>,
    #[serde(rename = "Duration")]
    pub duration: Option<String>,
    #[serde(rename = "Guid")]
    pub guid: Option<String>,
    #[serde(rename = "Binary")]
    pub binary: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
    #[serde(rename = "Collection")]
    pub collection: Option<Vec<Collection>>,
    #[serde(rename = "Record")]
    pub record: Option<Vec<Record>>,
    #[serde(rename = "LabeledElement")]
    pub labeled_element: Option<Vec<LabeledElement>>,
    #[serde(rename = "Null")]
    pub null: Option<Vec<Null>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Null")]
pub struct Null {}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "ComplexType")]
pub struct ComplexType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Property")]
    pub property: Option<Vec<Property>>,
    #[serde(rename = "NavigationProperty")]
    pub navigation_property: Option<Vec<NavigationProperty>>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "EnumType")]
pub struct EnumType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Member")]
    pub member: Option<Vec<Member>>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Member")]
pub struct Member {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Value")]
    pub value: String,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "TypeDefinition")]
pub struct TypeDefinition {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "UnderlyingType")]
    pub underlying_type: String,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Term")]
pub struct Term {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "BaseTerm")]
    pub base_term: Option<String>,
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,
    #[serde(rename = "@Nullable")]
    pub nullable: Option<String>,
    #[serde(rename = "MaxLength")]
    pub max_length: Option<String>,
    #[serde(rename = "Precision")]
    pub precision: Option<String>,
    #[serde(rename = "Scale")]
    pub scale: Option<String>,
    #[serde(rename = "Unicode")]
    pub unicode: Option<String>,
    #[serde(rename = "SRID")]
    pub srid: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Annotations")]
pub struct Annotations {
    #[serde(rename = "@Target")]
    pub target: String,
    #[serde(rename = "Qualifier")]
    pub qualifier: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Action")]
pub struct Action {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "IsBound")]
    pub is_bound: Option<String>,
    #[serde(rename = "EntitySetPath")]
    pub entity_set_path: Option<String>,
    #[serde(rename = "Parameter")]
    pub parameter: Option<Vec<Parameter>>,
    #[serde(rename = "ReturnType")]
    pub return_type: Option<ReturnType>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Parameter")]
pub struct Parameter {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "@Nullable")]
    pub nullable: Option<String>,
    #[serde(rename = "MaxLength")]
    pub max_length: Option<String>,
    #[serde(rename = "Precision")]
    pub precision: Option<String>,
    #[serde(rename = "Scale")]
    pub scale: Option<String>,
    #[serde(rename = "Unicode")]
    pub unicode: Option<String>,
    #[serde(rename = "SRID")]
    pub srid: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "ReturnType")]
pub struct ReturnType {
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "@Nullable")]
    pub nullable: Option<String>,
    #[serde(rename = "MaxLength")]
    pub max_length: Option<String>,
    #[serde(rename = "Precision")]
    pub precision: Option<String>,
    #[serde(rename = "Scale")]
    pub scale: Option<String>,
    #[serde(rename = "Unicode")]
    pub unicode: Option<String>,
    #[serde(rename = "SRID")]
    pub srid: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Function")]
pub struct Function {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "IsBound")]
    pub is_bound: Option<String>,
    #[serde(rename = "EntitySetPath")]
    pub entity_set_path: Option<String>,
    #[serde(rename = "Parameter")]
    pub parameter: Option<Vec<Parameter>>,
    #[serde(rename = "ReturnType")]
    pub return_type: Option<ReturnType>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "EntityContainer")]
pub struct EntityContainer {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "EntitySet")]
    pub entity_set: Option<Vec<EntitySet>>,
    #[serde(rename = "Singleton")]
    pub singleton: Option<Vec<Singleton>>,
    #[serde(rename = "ActionImport")]
    pub action_import: Option<Vec<ActionImport>>,
    #[serde(rename = "FunctionImport")]
    pub function_import: Option<Vec<FunctionImport>>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "EntitySet")]
pub struct EntitySet {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@EntityType")]
    pub entity_type: String,
    #[serde(rename = "IncludeAnnotations")]
    pub include_annotations: Option<String>,
    #[serde(rename = "NavigationPropertyBinding")]
    pub navigation_property_binding: Option<Vec<NavigationPropertyBinding>>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "NavigationPropertyBinding")]
pub struct NavigationPropertyBinding {
    #[serde(rename = "@Path")]
    pub path: String,
    #[serde(rename = "@Target")]
    pub target: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Singleton")]
pub struct Singleton {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "NavigationPropertyBinding")]
    pub navigation_property_binding: Option<Vec<NavigationPropertyBinding>>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "ActionImport")]
pub struct ActionImport {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Action")]
    pub action: String,
    #[serde(rename = "EntitySet")]
    pub entity_set: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "FunctionImport")]
pub struct FunctionImport {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Function")]
    pub function: String,
    #[serde(rename = "EntitySet")]
    pub entity_set: Option<String>,
    #[serde(rename = "IncludeInServiceDocument")]
    pub include_in_service_document: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation: Option<Vec<Annotation>>,
}
