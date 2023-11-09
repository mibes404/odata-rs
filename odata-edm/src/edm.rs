//! This crate contains the serde annotated structures for the odata CSDL 4.01 schema using the quick-xml crate.
//! The generated structures are based on the XSD schema definition in https://raw.githubusercontent.com/oasis-tcs/odata-csdl-schemas/main/schemas/edmx.xsd
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "edmx:Edmx")]
pub struct Edmx {
    #[serde(rename = "@Version")]
    pub version: String,
    #[serde(rename = "@xmlns:edmx", skip_serializing_if = "Option::is_none")]
    pub xmlns: Option<String>,
    #[serde(rename = "DataServices")]
    pub data_services: DataServices,
}

impl Default for Edmx {
    fn default() -> Self {
        Self {
            version: "4.01".to_string(),
            xmlns: None,
            data_services: DataServices { schema: Vec::new() },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "edmx:DataServices")]
pub struct DataServices {
    #[serde(rename = "Schema")]
    pub schema: Vec<Schema>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Schema")]
pub struct Schema {
    #[serde(rename = "@Namespace")]
    pub namespace: String,
    #[serde(rename = "@xmlns", skip_serializing_if = "Option::is_none")]
    pub xmlns: Option<String>,
    #[serde(rename = "EntityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<Vec<EntityType>>,
    #[serde(rename = "ComplexType", skip_serializing_if = "Option::is_none")]
    pub complex_type: Option<Vec<ComplexType>>,
    #[serde(rename = "EnumType", skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<Vec<EnumType>>,
    #[serde(rename = "TypeDefinition", skip_serializing_if = "Option::is_none")]
    pub type_definition: Option<Vec<TypeDefinition>>,
    #[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
    pub term: Option<Vec<Term>>,
    #[serde(rename = "Annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotations>>,
    #[serde(rename = "Action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<Action>>,
    #[serde(rename = "Function", skip_serializing_if = "Option::is_none")]
    pub function: Option<Vec<Function>>,
    #[serde(rename = "EntityContainer", skip_serializing_if = "Option::is_none")]
    pub entity_container: Option<Vec<EntityContainer>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "EntityType")]
pub struct EntityType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@HasStream", skip_serializing_if = "Option::is_none")]
    pub has_stream: Option<bool>,
    #[serde(rename = "@OpenType", skip_serializing_if = "Option::is_none")]
    pub open_type: Option<bool>,
    #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<Key>>,
    #[serde(rename = "Property", skip_serializing_if = "Option::is_none")]
    pub property: Option<Vec<Property>>,
    #[serde(rename = "NavigationProperty", skip_serializing_if = "Option::is_none")]
    pub navigation_property: Option<Vec<NavigationProperty>>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

impl EntityType {
    pub fn new(name: String) -> Self {
        Self {
            name,
            has_stream: None,
            open_type: None,
            key: None,
            property: None,
            navigation_property: None,
            annotation: None,
        }
    }

    pub fn add_property(&mut self, name: String, _type: String) {
        let property = Property {
            name,
            _type,
            nullable: None,
            default_value: None,
            max_length: None,
            precision: None,
            scale: None,
            unicode: None,
            srid: None,
            concurrency_mode: None,
            annotation: None,
        };

        if let Some(properties) = &mut self.property {
            properties.push(property);
        } else {
            self.property = Some(vec![property]);
        }
    }

    pub fn set_key<'k>(&mut self, keys: impl Iterator<Item = &'k str>) {
        let key = Key {
            property_ref: Some(keys.map(|k| PropertyRef { name: k.to_string() }).collect()),
        };

        self.key = Some(vec![key]);
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Key")]
pub struct Key {
    #[serde(rename = "PropertyRef", skip_serializing_if = "Option::is_none")]
    pub property_ref: Option<Vec<PropertyRef>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "PropertyRef")]
pub struct PropertyRef {
    #[serde(rename = "@Name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Property")]
pub struct Property {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "@Nullable", skip_serializing_if = "Option::is_none")]
    pub nullable: Option<String>,
    #[serde(rename = "DefaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "MaxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    #[serde(rename = "Precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,
    #[serde(rename = "Unicode", skip_serializing_if = "Option::is_none")]
    pub unicode: Option<String>,
    #[serde(rename = "SRID", skip_serializing_if = "Option::is_none")]
    pub srid: Option<String>,
    #[serde(rename = "ConcurrencyMode", skip_serializing_if = "Option::is_none")]
    pub concurrency_mode: Option<String>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "NavigationProperty")]
pub struct NavigationProperty {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "@Nullable", skip_serializing_if = "Option::is_none")]
    pub nullable: Option<String>,
    #[serde(rename = "Partner", skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    #[serde(rename = "ContainsTarget", skip_serializing_if = "Option::is_none")]
    pub contains_target: Option<String>,
    #[serde(rename = "ReferentialConstraint", skip_serializing_if = "Option::is_none")]
    pub referential_constraint: Option<ReferentialConstraint>,
    #[serde(rename = "OnDelete", skip_serializing_if = "Option::is_none")]
    pub on_delete: Option<OnDelete>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ReferentialConstraint")]
pub struct ReferentialConstraint {
    #[serde(rename = "Principal")]
    pub principal: Principal,
    #[serde(rename = "Dependent")]
    pub dependent: Dependent,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Principal")]
pub struct Principal {
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "PropertyRef", skip_serializing_if = "Option::is_none")]
    pub property_ref: Option<Vec<PropertyRef>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Dependent")]
pub struct Dependent {
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "PropertyRef", skip_serializing_if = "Option::is_none")]
    pub property_ref: Option<Vec<PropertyRef>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "OnDelete")]
pub struct OnDelete {
    #[serde(rename = "Action")]
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Annotation")]
pub struct Annotation {
    #[serde(rename = "@Term")]
    pub term: String,
    #[serde(rename = "Qualifier", skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "@Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "@String", skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(rename = "@Bool", skip_serializing_if = "Option::is_none")]
    pub bool: Option<String>,
    #[serde(rename = "@Int", skip_serializing_if = "Option::is_none")]
    pub int: Option<String>,
    #[serde(rename = "@Float", skip_serializing_if = "Option::is_none")]
    pub float: Option<String>,
    #[serde(rename = "@Decimal", skip_serializing_if = "Option::is_none")]
    pub decimal: Option<String>,
    #[serde(rename = "DateTimeOffset", skip_serializing_if = "Option::is_none")]
    pub date_time_offset: Option<String>,
    #[serde(rename = "Duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "Guid", skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(rename = "Binary", skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
    #[serde(rename = "Collection", skip_serializing_if = "Option::is_none")]
    pub collection: Option<Vec<Collection>>,
    #[serde(rename = "Record", skip_serializing_if = "Option::is_none")]
    pub record: Option<Vec<Record>>,
    #[serde(rename = "LabeledElement", skip_serializing_if = "Option::is_none")]
    pub labeled_element: Option<Vec<LabeledElement>>,
    #[serde(rename = "Null", skip_serializing_if = "Option::is_none")]
    pub null: Option<Vec<Null>>,
    #[serde(rename = "EnumMember", skip_serializing_if = "Option::is_none")]
    pub enum_member: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Collection")]
pub struct Collection {
    #[serde(rename = "String", skip_serializing_if = "Option::is_none")]
    pub string: Option<Vec<String>>,
    #[serde(rename = "Bool", skip_serializing_if = "Option::is_none")]
    pub bool: Option<Vec<bool>>,
    #[serde(rename = "Int", skip_serializing_if = "Option::is_none")]
    pub int: Option<Vec<i32>>,
    #[serde(rename = "Float", skip_serializing_if = "Option::is_none")]
    pub float: Option<Vec<f32>>,
    #[serde(rename = "Decimal", skip_serializing_if = "Option::is_none")]
    pub decimal: Option<Vec<String>>,
    #[serde(rename = "DateTimeOffset", skip_serializing_if = "Option::is_none")]
    pub date_time_offset: Option<Vec<String>>,
    #[serde(rename = "Duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Vec<String>>,
    #[serde(rename = "Guid", skip_serializing_if = "Option::is_none")]
    pub guid: Option<Vec<String>>,
    #[serde(rename = "Binary", skip_serializing_if = "Option::is_none")]
    pub binary: Option<Vec<String>>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
    #[serde(rename = "Collection", skip_serializing_if = "Option::is_none")]
    pub collection: Option<Vec<Collection>>,
    #[serde(rename = "Record", skip_serializing_if = "Option::is_none")]
    pub record: Option<Vec<Record>>,
    #[serde(rename = "LabeledElement", skip_serializing_if = "Option::is_none")]
    pub labeled_element: Option<Vec<LabeledElement>>,
    #[serde(rename = "Null", skip_serializing_if = "Option::is_none")]
    pub null: Option<Vec<Null>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Record")]
pub struct Record {
    #[serde(rename = "PropertyValue", skip_serializing_if = "Option::is_none")]
    pub property_value: Option<Vec<PropertyValue>>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "PropertyValue")]
pub struct PropertyValue {
    #[serde(rename = "@Property")]
    pub property: String,
    #[serde(rename = "@Bool", skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "NavigationPropertyPath", skip_serializing_if = "Option::is_none")]
    pub navigation_property_path: Option<String>,
    #[serde(rename = "AnnotationPath", skip_serializing_if = "Option::is_none")]
    pub annotation_path: Option<String>,
    #[serde(rename = "PropertyPath", skip_serializing_if = "Option::is_none")]
    pub property_path: Option<String>,
    #[serde(rename = "Null", skip_serializing_if = "Option::is_none")]
    pub null: Option<Vec<Null>>,
    #[serde(rename = "Record", skip_serializing_if = "Option::is_none")]
    pub record: Option<Vec<Record>>,
    #[serde(rename = "Collection", skip_serializing_if = "Option::is_none")]
    pub collection: Option<Vec<Collection>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "LabeledElement")]
pub struct LabeledElement {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "String", skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(rename = "Bool", skip_serializing_if = "Option::is_none")]
    pub bool: Option<String>,
    #[serde(rename = "Int", skip_serializing_if = "Option::is_none")]
    pub int: Option<String>,
    #[serde(rename = "Float", skip_serializing_if = "Option::is_none")]
    pub float: Option<String>,
    #[serde(rename = "Decimal", skip_serializing_if = "Option::is_none")]
    pub decimal: Option<String>,
    #[serde(rename = "DateTimeOffset", skip_serializing_if = "Option::is_none")]
    pub date_time_offset: Option<String>,
    #[serde(rename = "Duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "Guid", skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(rename = "Binary", skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
    #[serde(rename = "Collection", skip_serializing_if = "Option::is_none")]
    pub collection: Option<Vec<Collection>>,
    #[serde(rename = "Record", skip_serializing_if = "Option::is_none")]
    pub record: Option<Vec<Record>>,
    #[serde(rename = "LabeledElement", skip_serializing_if = "Option::is_none")]
    pub labeled_element: Option<Vec<LabeledElement>>,
    #[serde(rename = "Null", skip_serializing_if = "Option::is_none")]
    pub null: Option<Vec<Null>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Null")]
pub struct Null {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ComplexType")]
pub struct ComplexType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Property", skip_serializing_if = "Option::is_none")]
    pub property: Option<Vec<Property>>,
    #[serde(rename = "NavigationProperty", skip_serializing_if = "Option::is_none")]
    pub navigation_property: Option<Vec<NavigationProperty>>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "EnumType")]
pub struct EnumType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Member", skip_serializing_if = "Option::is_none")]
    pub member: Option<Vec<Member>>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Member")]
pub struct Member {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Value")]
    pub value: String,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "TypeDefinition")]
pub struct TypeDefinition {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "UnderlyingType")]
    pub underlying_type: String,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Term")]
pub struct Term {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "BaseTerm", skip_serializing_if = "Option::is_none")]
    pub base_term: Option<String>,
    #[serde(rename = "DefaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "@Nullable", skip_serializing_if = "Option::is_none")]
    pub nullable: Option<String>,
    #[serde(rename = "MaxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    #[serde(rename = "Precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,
    #[serde(rename = "Unicode", skip_serializing_if = "Option::is_none")]
    pub unicode: Option<String>,
    #[serde(rename = "SRID", skip_serializing_if = "Option::is_none")]
    pub srid: Option<String>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Annotations")]
pub struct Annotations {
    #[serde(rename = "@Target")]
    pub target: String,
    #[serde(rename = "Qualifier", skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Action")]
pub struct Action {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "IsBound", skip_serializing_if = "Option::is_none")]
    pub is_bound: Option<String>,
    #[serde(rename = "EntitySetPath", skip_serializing_if = "Option::is_none")]
    pub entity_set_path: Option<String>,
    #[serde(rename = "Parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<Vec<Parameter>>,
    #[serde(rename = "ReturnType", skip_serializing_if = "Option::is_none")]
    pub return_type: Option<ReturnType>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Parameter")]
pub struct Parameter {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "@Nullable", skip_serializing_if = "Option::is_none")]
    pub nullable: Option<String>,
    #[serde(rename = "MaxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    #[serde(rename = "Precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,
    #[serde(rename = "Unicode", skip_serializing_if = "Option::is_none")]
    pub unicode: Option<String>,
    #[serde(rename = "SRID", skip_serializing_if = "Option::is_none")]
    pub srid: Option<String>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ReturnType")]
pub struct ReturnType {
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "@Nullable", skip_serializing_if = "Option::is_none")]
    pub nullable: Option<String>,
    #[serde(rename = "MaxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    #[serde(rename = "Precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,
    #[serde(rename = "Unicode", skip_serializing_if = "Option::is_none")]
    pub unicode: Option<String>,
    #[serde(rename = "SRID", skip_serializing_if = "Option::is_none")]
    pub srid: Option<String>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Function")]
pub struct Function {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "IsBound", skip_serializing_if = "Option::is_none")]
    pub is_bound: Option<String>,
    #[serde(rename = "EntitySetPath", skip_serializing_if = "Option::is_none")]
    pub entity_set_path: Option<String>,
    #[serde(rename = "Parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<Vec<Parameter>>,
    #[serde(rename = "ReturnType", skip_serializing_if = "Option::is_none")]
    pub return_type: Option<ReturnType>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "EntityContainer")]
pub struct EntityContainer {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "EntitySet", skip_serializing_if = "Option::is_none")]
    pub entity_set: Option<Vec<EntitySet>>,
    #[serde(rename = "Singleton", skip_serializing_if = "Option::is_none")]
    pub singleton: Option<Vec<Singleton>>,
    #[serde(rename = "ActionImport", skip_serializing_if = "Option::is_none")]
    pub action_import: Option<Vec<ActionImport>>,
    #[serde(rename = "FunctionImport", skip_serializing_if = "Option::is_none")]
    pub function_import: Option<Vec<FunctionImport>>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "EntitySet")]
pub struct EntitySet {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@EntityType")]
    pub entity_type: String,
    #[serde(rename = "IncludeAnnotations", skip_serializing_if = "Option::is_none")]
    pub include_annotations: Option<String>,
    #[serde(rename = "NavigationPropertyBinding", skip_serializing_if = "Option::is_none")]
    pub navigation_property_binding: Option<Vec<NavigationPropertyBinding>>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "NavigationPropertyBinding")]
pub struct NavigationPropertyBinding {
    #[serde(rename = "@Path")]
    pub path: String,
    #[serde(rename = "@Target")]
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Singleton")]
pub struct Singleton {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub _type: String,
    #[serde(rename = "NavigationPropertyBinding", skip_serializing_if = "Option::is_none")]
    pub navigation_property_binding: Option<Vec<NavigationPropertyBinding>>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ActionImport")]
pub struct ActionImport {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Action")]
    pub action: String,
    #[serde(rename = "EntitySet", skip_serializing_if = "Option::is_none")]
    pub entity_set: Option<String>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "FunctionImport")]
pub struct FunctionImport {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Function")]
    pub function: String,
    #[serde(rename = "EntitySet", skip_serializing_if = "Option::is_none")]
    pub entity_set: Option<String>,
    #[serde(rename = "IncludeInServiceDocument", skip_serializing_if = "Option::is_none")]
    pub include_in_service_document: Option<String>,
    #[serde(rename = "Annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<Annotation>>,
}
