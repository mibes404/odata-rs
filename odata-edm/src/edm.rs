//! This crate contains the yaserde structures for the odata CSDL 4.01 schema.
//! The generated structures are based on the XSD schema definition in https://raw.githubusercontent.com/oasis-tcs/odata-csdl-schemas/main/schemas/edmx.xsd

use yaserde_derive::{YaDeserialize, YaSerialize};

/// The Edmx element is the root element of an XML document that conforms to the CSDL specification.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Edmx",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Edmx {
    /// The Edmx element MUST contain one or more DataServices elements.
    #[yaserde(prefix = "edmx", rename = "DataServices")]
    pub data_services: DataServices,

    /// The Version attribute is a string that specifies the version of the CSDL document.
    #[yaserde(attribute, rename = "Version")]
    pub version: String,
}

/// The DataServices element is the root element of an XML document that conforms to the CSDL specification.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "DataServices",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct DataServices {
    /// The DataServices element MUST contain one or more Schema elements.
    #[yaserde(prefix = "edmx", rename = "Schema")]
    pub schema: Vec<Schema>,
}

/// The Schema element defines a namespace for a schema and contains the type definitions and annotations for that schema.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Schema",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Schema {
    /// The Namespace attribute is a string that defines a namespace for the schema.
    #[yaserde(attribute, rename = "Namespace")]
    pub namespace: String,

    /// The Alias attribute is a string that defines an alias for the schema.
    #[yaserde(attribute, rename = "Alias")]
    pub alias: Option<String>,

    /// The Schema element MUST contain zero or more EntityType elements.
    #[yaserde(prefix = "edmx", rename = "EntityType")]
    pub entity_type: Vec<EntityType>,

    /// The Schema element MUST contain zero or more ComplexType elements.
    #[yaserde(prefix = "edmx", rename = "ComplexType")]
    pub complex_type: Vec<ComplexType>,

    /// The Schema element MUST contain zero or more EnumType elements.
    #[yaserde(prefix = "edmx", rename = "EnumType")]
    pub enum_type: Vec<EnumType>,

    /// The Schema element MUST contain zero or more TypeDefinition elements.
    #[yaserde(prefix = "edmx", rename = "TypeDefinition")]
    pub type_definition: Vec<TypeDefinition>,

    /// The Schema element MUST contain zero or more Action elements.
    #[yaserde(prefix = "edmx", rename = "Action")]
    pub action: Vec<Action>,

    /// The Schema element MUST contain zero or more Function elements.
    #[yaserde(prefix = "edmx", rename = "Function")]
    pub function: Vec<Function>,

    /// The Schema element MUST contain zero or more Term elements.
    #[yaserde(prefix = "edmx", rename = "Term")]
    pub term: Vec<Term>,

    /// The Schema element MUST contain zero or more EntityContainer elements.
    #[yaserde(prefix = "edmx", rename = "EntityContainer")]
    pub entity_container: Vec<EntityContainer>,

    /// The Schema element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The EntityType element defines a type that represents a collection of properties.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "EntityType",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct EntityType {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the EntityType.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The BaseType attribute is a QualifiedName that specifies the base type of the EntityType.
    #[yaserde(attribute, rename = "BaseType")]
    pub base_type: Option<String>,

    /// The EntityType element MUST contain zero or more Property elements.
    #[yaserde(prefix = "edmx", rename = "Property")]
    pub property: Vec<Property>,

    /// The EntityType element MUST contain zero or more NavigationProperty elements.
    #[yaserde(prefix = "edmx", rename = "NavigationProperty")]
    pub navigation_property: Vec<NavigationProperty>,

    /// The EntityType element MUST contain zero or more Key elements.
    #[yaserde(prefix = "edmx", rename = "Key")]
    pub key: Vec<Key>,

    /// The EntityType element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The ComplexType element defines a type that represents a collection of properties.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "ComplexType",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct ComplexType {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the ComplexType.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The BaseType attribute is a QualifiedName that specifies the base type of the ComplexType.
    #[yaserde(attribute, rename = "BaseType")]
    pub base_type: Option<String>,

    /// The ComplexType element MUST contain zero or more Property elements.
    #[yaserde(prefix = "edmx", rename = "Property")]
    pub property: Vec<Property>,

    /// The ComplexType element MUST contain zero or more NavigationProperty elements.
    #[yaserde(prefix = "edmx", rename = "NavigationProperty")]
    pub navigation_property: Vec<NavigationProperty>,

    /// The ComplexType element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The EnumType element defines a type that represents a collection of named values.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "EnumType",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct EnumType {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the EnumType.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The UnderlyingType attribute is a SimpleIdentifier that specifies the underlying type of the EnumType.
    #[yaserde(attribute, rename = "UnderlyingType")]
    pub underlying_type: Option<String>,

    /// The IsFlags attribute is a Boolean that specifies whether the EnumType is a flags type.
    #[yaserde(attribute, rename = "IsFlags")]
    pub is_flags: Option<bool>,

    /// The EnumType element MUST contain one or more Member elements.
    #[yaserde(prefix = "edmx", rename = "Member")]
    pub member: Vec<Member>,

    /// The EnumType element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The TypeDefinition element defines a type that represents a collection of properties.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "TypeDefinition",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct TypeDefinition {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the TypeDefinition.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The UnderlyingType attribute is a QualifiedName that specifies the underlying type of the TypeDefinition.
    #[yaserde(attribute, rename = "UnderlyingType")]
    pub underlying_type: Option<String>,

    /// The TypeDefinition element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The Action element defines an action that can be performed.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Action",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Action {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the Action.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The IsBound attribute is a Boolean that specifies whether the Action is bound.
    #[yaserde(attribute, rename = "IsBound")]
    pub is_bound: Option<bool>,

    /// The EntitySetPath attribute is a string that specifies the entity set path of the Action.
    #[yaserde(attribute, rename = "EntitySetPath")]
    pub entity_set_path: Option<String>,

    /// The Action element MUST contain zero or more Parameter elements.
    #[yaserde(prefix = "edmx", rename = "Parameter")]
    pub parameter: Vec<Parameter>,

    /// The Action element MUST contain zero or more ReturnType elements.
    #[yaserde(prefix = "edmx", rename = "ReturnType")]
    pub return_type: Vec<ReturnType>,

    /// The Action element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The Function element defines a function that can be invoked.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Function",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Function {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the Function.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The IsBound attribute is a Boolean that specifies whether the Function is bound.
    #[yaserde(attribute, rename = "IsBound")]
    pub is_bound: Option<bool>,

    /// The IsComposable attribute is a Boolean that specifies whether the Function is composable.
    #[yaserde(attribute, rename = "IsComposable")]
    pub is_composable: Option<bool>,

    /// The EntitySetPath attribute is a string that specifies the entity set path of the Function.
    #[yaserde(attribute, rename = "EntitySetPath")]
    pub entity_set_path: Option<String>,

    /// The Function element MUST contain zero or more Parameter elements.
    #[yaserde(prefix = "edmx", rename = "Parameter")]
    pub parameter: Vec<Parameter>,

    /// The Function element MUST contain zero or more ReturnType elements.
    #[yaserde(prefix = "edmx", rename = "ReturnType")]
    pub return_type: Vec<ReturnType>,

    /// The Function element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The Term element defines a term that can be used in an annotation.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Term",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Term {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the Term.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The Type attribute is a QualifiedName that specifies the type of the Term.
    #[yaserde(attribute, rename = "Type")]
    pub _type: Option<String>,

    /// The AppliesTo attribute is a string that specifies the AppliesTo of the Term.
    #[yaserde(attribute, rename = "AppliesTo")]
    pub applies_to: Option<String>,

    /// The BaseTerm attribute is a QualifiedName that specifies the base term of the Term.
    #[yaserde(attribute, rename = "BaseTerm")]
    pub base_term: Option<String>,

    /// The DefaultValue attribute is a string that specifies the default value of the Term.
    #[yaserde(attribute, rename = "DefaultValue")]
    pub default_value: Option<String>,

    /// The Nullable attribute is a Boolean that specifies whether the Term is nullable.
    #[yaserde(attribute, rename = "Nullable")]
    pub nullable: Option<bool>,

    /// The MaxLength attribute is a non-negative integer that specifies the maximum length of the Term.
    #[yaserde(attribute, rename = "MaxLength")]
    pub max_length: Option<String>,

    /// The Precision attribute is a non-negative integer that specifies the precision of the Term.
    #[yaserde(attribute, rename = "Precision")]
    pub precision: Option<String>,

    /// The Scale attribute is a non-negative integer that specifies the scale of the Term.
    #[yaserde(attribute, rename = "Scale")]
    pub scale: Option<String>,

    /// The SRID attribute is a non-negative integer that specifies the SRID of the Term.
    #[yaserde(attribute, rename = "SRID")]
    pub srid: Option<String>,

    /// The Term element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The EntityContainer element defines an entity container.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "EntityContainer",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct EntityContainer {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the EntityContainer.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The Extends attribute is a QualifiedName that specifies the base type of the EntityContainer.
    #[yaserde(attribute, rename = "Extends")]
    pub extends: Option<String>,

    /// The EntityContainer element MUST contain zero or more EntitySet elements.
    #[yaserde(prefix = "edmx", rename = "EntitySet")]
    pub entity_set: Vec<EntitySet>,

    /// The EntityContainer element MUST contain zero or more Singleton elements.
    #[yaserde(prefix = "edmx", rename = "Singleton")]
    pub singleton: Vec<Singleton>,

    /// The EntityContainer element MUST contain zero or more ActionImport elements.
    #[yaserde(prefix = "edmx", rename = "ActionImport")]
    pub action_import: Vec<ActionImport>,

    /// The EntityContainer element MUST contain zero or more FunctionImport elements.
    #[yaserde(prefix = "edmx", rename = "FunctionImport")]
    pub function_import: Vec<FunctionImport>,

    /// The EntityContainer element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The Annotation element defines an annotation.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Annotation",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Annotation {
    /// The Term attribute is a QualifiedName that specifies the term of the Annotation.
    #[yaserde(attribute, rename = "Term")]
    pub term: String,

    /// The Qualifier attribute is a SimpleIdentifier that specifies the qualifier of the Annotation.
    #[yaserde(attribute, rename = "Qualifier")]
    pub qualifier: Option<String>,

    /// The Path attribute is a string that specifies the path of the Annotation.
    #[yaserde(attribute, rename = "Path")]
    pub path: Option<String>,

    /// The Target attribute is a string that specifies the target of the Annotation.
    #[yaserde(attribute, rename = "Target")]
    pub target: Option<String>,

    /// The Annotation element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,

    /// The Annotation element MUST contain zero or more PropertyValue elements.
    #[yaserde(prefix = "edmx", rename = "PropertyValue")]
    pub property_value: Vec<PropertyValue>,

    /// The Collection element defines a collection.
    #[yaserde(prefix = "edmx", rename = "Collection")]
    pub collection: Vec<Collection>,

    /// These properties match with the GInlineExpressions type.
    /// The String element defines a string.
    #[yaserde(prefix = "edmx", rename = "String")]
    pub string: Vec<Body>,

    /// The Binary element defines a binary.
    #[yaserde(prefix = "edmx", rename = "Binary")]
    pub binary: Vec<Body>,

    /// The Bool element defines a bool.
    #[yaserde(prefix = "edmx", rename = "Bool")]
    pub bool: Vec<Body>,

    /// The Date element defines a date.
    #[yaserde(prefix = "edmx", rename = "Date")]
    pub date: Vec<Body>,

    /// The DateTimeOffset element defines a date time offset.
    #[yaserde(prefix = "edmx", rename = "DateTimeOffset")]
    pub date_time_offset: Vec<Body>,

    /// The Decimal element defines a decimal.
    #[yaserde(prefix = "edmx", rename = "Decimal")]
    pub decimal: Vec<Body>,

    /// The Duration element defines a duration.
    #[yaserde(prefix = "edmx", rename = "Duration")]
    pub duration: Vec<Body>,

    /// The EnumMember element defines an enum member.
    #[yaserde(prefix = "edmx", rename = "EnumMember")]
    pub enum_member: Vec<EnumMember>,

    /// The Float element defines a float.
    #[yaserde(prefix = "edmx", rename = "Float")]
    pub float: Vec<Body>,

    /// The Guid element defines a guid.
    #[yaserde(prefix = "edmx", rename = "Guid")]
    pub guid: Vec<Body>,

    /// The Int element defines an int.
    #[yaserde(prefix = "edmx", rename = "Int")]
    pub int: Vec<Body>,

    /// The TimeOfDay element defines a time of day.
    #[yaserde(prefix = "edmx", rename = "TimeOfDay")]
    pub time_of_day: Vec<Body>,

    /// The PropertyPath element defines a property path.
    #[yaserde(prefix = "edmx", rename = "PropertyPath")]
    pub property_path: Vec<Body>,

    /// The NavigationPropertyPath element defines a navigation property path.
    #[yaserde(prefix = "edmx", rename = "NavigationPropertyPath")]
    pub navigation_property_path: Vec<Body>,

    /// The AnnotationPath element defines an annotation path.
    #[yaserde(prefix = "edmx", rename = "AnnotationPath")]
    pub annotation_path: Vec<Body>,

    /// The Null element defines a null.
    #[yaserde(prefix = "edmx", rename = "Null")]
    pub null: Vec<Body>,

    /// The LabeledElement element defines a labeled element.
    #[yaserde(prefix = "edmx", rename = "LabeledElement")]
    pub labeled_element: Vec<Body>,

    /// The UrlRefence element defines a url reference.
    #[yaserde(prefix = "edmx", rename = "UrlRefence")]
    pub url_refence: Vec<Body>,
}

/// The EntitySet element defines an entity set.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "EntitySet",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct EntitySet {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the EntitySet.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The EntityType attribute is a QualifiedName that specifies the entity type of the EntitySet.
    #[yaserde(attribute, rename = "EntityType")]
    pub entity_type: String,

    /// The EntitySet element MUST contain zero or more NavigationPropertyBinding elements.
    #[yaserde(prefix = "edmx", rename = "NavigationPropertyBinding")]
    pub navigation_property_binding: Vec<NavigationPropertyBinding>,

    /// The EntitySet element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The Singleton element defines a singleton.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Singleton",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Singleton {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the Singleton.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The Type attribute is a QualifiedName that specifies the type of the Singleton.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,

    /// The Singleton element MUST contain zero or more NavigationPropertyBinding elements.
    #[yaserde(prefix = "edmx", rename = "NavigationPropertyBinding")]
    pub navigation_property_binding: Vec<NavigationPropertyBinding>,

    /// The Singleton element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The ActionImport element defines an action import.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "ActionImport",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct ActionImport {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the ActionImport.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The Action attribute is a QualifiedName that specifies the action of the ActionImport.
    #[yaserde(attribute, rename = "Action")]
    pub action: String,

    /// The EntitySet attribute is a string that specifies the entity set of the ActionImport.
    #[yaserde(attribute, rename = "EntitySet")]
    pub entity_set: Option<String>,

    /// The ActionImport element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,

    /// The ActionImport element MUST contain zero or more Parameter elements.
    #[yaserde(prefix = "edmx", rename = "Parameter")]
    pub parameter: Vec<Parameter>,
}

/// The FunctionImport element defines a function import.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "FunctionImport",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct FunctionImport {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the FunctionImport.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The Function attribute is a QualifiedName that specifies the function of the FunctionImport.
    #[yaserde(attribute, rename = "Function")]
    pub function: String,

    /// The EntitySet attribute is a string that specifies the entity set of the FunctionImport.
    #[yaserde(attribute, rename = "EntitySet")]
    pub entity_set: Option<String>,

    /// The IncludeInServiceDocument attribute is a Boolean that specifies whether the FunctionImport is included in the service document.
    #[yaserde(attribute, rename = "IncludeInServiceDocument")]
    pub include_in_service_document: Option<bool>,

    /// The FunctionImport element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,

    /// The FunctionImport element MUST contain zero or more Parameter elements.
    #[yaserde(prefix = "edmx", rename = "Parameter")]
    pub parameter: Vec<Parameter>,
}

/// The NavigationPropertyBinding element defines a navigation property binding.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "NavigationPropertyBinding",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct NavigationPropertyBinding {
    /// The Path attribute is a string that specifies the path of the NavigationPropertyBinding.
    #[yaserde(attribute, rename = "Path")]
    pub path: String,

    /// The Target attribute is a string that specifies the target of the NavigationPropertyBinding.
    #[yaserde(attribute, rename = "Target")]
    pub target: String,
}

/// The Parameter element defines a parameter.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Parameter",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Parameter {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the Parameter.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The Type attribute is a QualifiedName that specifies the type of the Parameter.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,

    /// The Nullable attribute is a Boolean that specifies whether the Parameter is nullable.
    #[yaserde(attribute, rename = "Nullable")]
    pub nullable: Option<bool>,

    /// The MaxLength attribute is a non-negative integer that specifies the maximum length of the Parameter.
    #[yaserde(attribute, rename = "MaxLength")]
    pub max_length: Option<String>,

    /// The Precision attribute is a non-negative integer that specifies the precision of the Parameter.
    #[yaserde(attribute, rename = "Precision")]
    pub precision: Option<String>,

    /// The Scale attribute is a non-negative integer that specifies the scale of the Parameter.
    #[yaserde(attribute, rename = "Scale")]
    pub scale: Option<String>,

    /// The SRID attribute is a non-negative integer that specifies the SRID of the Parameter.
    #[yaserde(attribute, rename = "SRID")]
    pub srid: Option<String>,

    /// The DefaultValue attribute is a string that specifies the default value of the Parameter.
    #[yaserde(attribute, rename = "DefaultValue")]
    pub default_value: Option<String>,

    /// The Unicode attribute is a Boolean that specifies whether the Parameter is Unicode.
    #[yaserde(attribute, rename = "Unicode")]
    pub unicode: Option<bool>,

    /// The Collation attribute is a string that specifies the collation of the Parameter.
    #[yaserde(attribute, rename = "Collation")]
    pub collation: Option<String>,

    /// The Parameter element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,

    /// The Parameter element MUST contain zero or more ReferentialConstraint elements.
    #[yaserde(prefix = "edmx", rename = "ReferentialConstraint")]
    pub referential_constraint: Vec<ReferentialConstraint>,
}

/// The ReferentialConstraint element defines a referential constraint.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "ReferentialConstraint",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct ReferentialConstraint {
    /// The Property attribute is a SimpleIdentifier that specifies the property of the ReferentialConstraint.
    #[yaserde(attribute, rename = "Property")]
    pub property: String,

    /// The ReferencedProperty attribute is a SimpleIdentifier that specifies the referenced property of the ReferentialConstraint.
    #[yaserde(attribute, rename = "ReferencedProperty")]
    pub referenced_property: String,
}

/// The ReturnType element defines a return type.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "ReturnType",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct ReturnType {
    /// The Type attribute is a QualifiedName that specifies the type of the ReturnType.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,

    /// The Nullable attribute is a Boolean that specifies whether the ReturnType is nullable.
    #[yaserde(attribute, rename = "Nullable")]
    pub nullable: Option<bool>,

    /// The MaxLength attribute is a non-negative integer that specifies the maximum length of the ReturnType.
    #[yaserde(attribute, rename = "MaxLength")]
    pub max_length: Option<String>,

    /// The Precision attribute is a non-negative integer that specifies the precision of the ReturnType.
    #[yaserde(attribute, rename = "Precision")]
    pub precision: Option<String>,

    /// The Scale attribute is a non-negative integer that specifies the scale of the ReturnType.
    #[yaserde(attribute, rename = "Scale")]
    pub scale: Option<String>,

    /// The SRID attribute is a non-negative integer that specifies the SRID of the ReturnType.
    #[yaserde(attribute, rename = "SRID")]
    pub srid: Option<String>,

    /// The ReturnType element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The Member element defines a member of an EnumType.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Member",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Member {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the Member.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The Value attribute is a string that specifies the value of the Member.
    #[yaserde(attribute, rename = "Value")]
    pub value: Option<String>,

    /// The Member element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The Key element defines a key.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Key",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Key {
    /// The PropertyRef element defines a property reference.
    #[yaserde(prefix = "edmx", rename = "PropertyRef")]
    pub property_ref: Vec<PropertyRef>,
}

/// The PropertyRef element defines a property reference.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "PropertyRef",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct PropertyRef {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the PropertyRef.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
}

/// The Property element defines a property.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Property",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Property {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the Property.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The Type attribute is a QualifiedName that specifies the type of the Property.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,

    /// The Nullable attribute is a Boolean that specifies whether the Property is nullable.
    #[yaserde(attribute, rename = "Nullable")]
    pub nullable: Option<bool>,

    /// The MaxLength attribute is a non-negative integer that specifies the maximum length of the Property.
    #[yaserde(attribute, rename = "MaxLength")]
    pub max_length: Option<String>,

    /// The Precision attribute is a non-negative integer that specifies the precision of the Property.
    #[yaserde(attribute, rename = "Precision")]
    pub precision: Option<String>,

    /// The Scale attribute is a non-negative integer that specifies the scale of the Property.
    #[yaserde(attribute, rename = "Scale")]
    pub scale: Option<String>,

    /// The SRID attribute is a non-negative integer that specifies the SRID of the Property.
    #[yaserde(attribute, rename = "SRID")]
    pub srid: Option<String>,

    /// The DefaultValue attribute is a string that specifies the default value of the Property.
    #[yaserde(attribute, rename = "DefaultValue")]
    pub default_value: Option<String>,

    /// The Unicode attribute is a Boolean that specifies whether the Property is Unicode.
    #[yaserde(attribute, rename = "Unicode")]
    pub unicode: Option<bool>,

    /// The Collation attribute is a string that specifies the collation of the Property.
    #[yaserde(attribute, rename = "Collation")]
    pub collation: Option<String>,

    /// The Property element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,

    /// The Property element MUST contain zero or more ReferentialConstraint elements.
    #[yaserde(prefix = "edmx", rename = "ReferentialConstraint")]
    pub referential_constraint: Vec<ReferentialConstraint>,
}

/// The NavigationProperty element defines a navigation property.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "NavigationProperty",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct NavigationProperty {
    /// The Name attribute is a SimpleIdentifier that specifies the name of the NavigationProperty.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    /// The Type attribute is a QualifiedName that specifies the type of the NavigationProperty.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,

    /// The Nullable attribute is a Boolean that specifies whether the NavigationProperty is nullable.
    #[yaserde(attribute, rename = "Nullable")]
    pub nullable: Option<bool>,

    /// The Partner attribute is a SimpleIdentifier that specifies the partner of the NavigationProperty.
    #[yaserde(attribute, rename = "Partner")]
    pub partner: Option<String>,

    /// The ContainsTarget attribute is a Boolean that specifies whether the NavigationProperty contains the target.
    #[yaserde(attribute, rename = "ContainsTarget")]
    pub contains_target: Option<bool>,

    /// The ReferentialConstraint element defines a referential constraint.
    #[yaserde(prefix = "edmx", rename = "ReferentialConstraint")]
    pub referential_constraint: Vec<ReferentialConstraint>,

    /// The OnDelete element defines an on delete.
    #[yaserde(prefix = "edmx", rename = "OnDelete")]
    pub on_delete: Option<OnDelete>,

    /// The NavigationProperty element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The OnDelete element defines an on delete.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "OnDelete",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct OnDelete {
    /// The Action attribute is a string that specifies the action of the OnDelete.
    #[yaserde(attribute, rename = "Action")]
    pub action: String,
}

/// The PropertyValue element defines a property value.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "PropertyValue",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct PropertyValue {
    /// The Property attribute is a SimpleIdentifier that specifies the property of the PropertyValue.
    #[yaserde(attribute, rename = "Property")]
    pub property: String,

    /// The Path attribute is a string that specifies the path of the PropertyValue.
    #[yaserde(attribute, rename = "Path")]
    pub path: Option<String>,

    /// The PropertyValue element MUST contain zero or more PropertyValue elements.
    #[yaserde(prefix = "edmx", rename = "PropertyValue")]
    pub property_value: Vec<PropertyValue>,

    /// The PropertyValue element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,

    /// The PropertyValue element MUST contain zero or more Record elements.
    #[yaserde(prefix = "edmx", rename = "Record")]
    pub record: Vec<Record>,

    /// The PropertyValue element MUST contain zero or more Collection elements.
    #[yaserde(prefix = "edmx", rename = "Collection")]
    pub collection: Vec<Collection>,
}

/// The Record element defines a record.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Record",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Record {
    /// The Type attribute is a QualifiedName that specifies the type of the Record.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,

    /// The Record element MUST contain zero or more PropertyValue elements.
    #[yaserde(prefix = "edmx", rename = "PropertyValue")]
    pub property_value: Vec<PropertyValue>,

    /// The Record element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,
}

/// The Collection element defines a collection.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Collection",
    prefix = "edmx",
    namespace = "edmx: http://docs.oasis-open.org/odata/ns/edmx"
)]
pub struct Collection {
    /// The Type attribute is a QualifiedName that specifies the type of the Collection.
    #[yaserde(attribute, rename = "Type")]
    pub _type: Option<String>,

    /// The Collection element MUST contain zero or more PropertyValue elements.
    #[yaserde(prefix = "edmx", rename = "PropertyValue")]
    pub property_value: Vec<PropertyValue>,

    /// The Collection element MUST contain zero or more Annotation elements.
    #[yaserde(prefix = "edmx", rename = "Annotation")]
    pub annotation: Vec<Annotation>,

    /// These properties match with the GInlineExpressions type.
    /// The String element defines a string.
    #[yaserde(prefix = "edmx", rename = "String")]
    pub string: Vec<Body>,

    /// The Binary element defines a binary.
    #[yaserde(prefix = "edmx", rename = "Binary")]
    pub binary: Vec<Body>,

    /// The Bool element defines a bool.
    #[yaserde(prefix = "edmx", rename = "Bool")]
    pub bool: Vec<Body>,

    /// The Date element defines a date.
    #[yaserde(prefix = "edmx", rename = "Date")]
    pub date: Vec<Body>,

    /// The DateTimeOffset element defines a date time offset.
    #[yaserde(prefix = "edmx", rename = "DateTimeOffset")]
    pub date_time_offset: Vec<Body>,

    /// The Decimal element defines a decimal.
    #[yaserde(prefix = "edmx", rename = "Decimal")]
    pub decimal: Vec<Body>,

    /// The Duration element defines a duration.
    #[yaserde(prefix = "edmx", rename = "Duration")]
    pub duration: Vec<Body>,

    /// The EnumMember element defines an enum member.
    #[yaserde(prefix = "edmx", rename = "EnumMember")]
    pub enum_member: Vec<EnumMember>,

    /// The Float element defines a float.
    #[yaserde(prefix = "edmx", rename = "Float")]
    pub float: Vec<Body>,

    /// The Guid element defines a guid.
    #[yaserde(prefix = "edmx", rename = "Guid")]
    pub guid: Vec<Body>,

    /// The Int element defines an int.
    #[yaserde(prefix = "edmx", rename = "Int")]
    pub int: Vec<Body>,

    /// The TimeOfDay element defines a time of day.
    #[yaserde(prefix = "edmx", rename = "TimeOfDay")]
    pub time_of_day: Vec<Body>,

    /// The PropertyPath element defines a property path.
    #[yaserde(prefix = "edmx", rename = "PropertyPath")]
    pub property_path: Vec<Body>,

    /// The NavigationPropertyPath element defines a navigation property path.
    #[yaserde(prefix = "edmx", rename = "NavigationPropertyPath")]
    pub navigation_property_path: Vec<Body>,

    /// The AnnotationPath element defines an annotation path.
    #[yaserde(prefix = "edmx", rename = "AnnotationPath")]
    pub annotation_path: Vec<Body>,

    /// The Null element defines a null.
    #[yaserde(prefix = "edmx", rename = "Null")]
    pub null: Vec<Body>,

    /// The LabeledElement element defines a labeled element.
    #[yaserde(prefix = "edmx", rename = "LabeledElement")]
    pub labeled_element: Vec<Body>,

    /// The UrlRefence element defines a url reference.
    #[yaserde(prefix = "edmx", rename = "UrlRefence")]
    pub url_refence: Vec<Body>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(flatten)]
pub struct Body {
    /// The Value attribute contains the body of the String element
    pub value: String,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(text)]
pub struct EnumMember {
    /// The Value attribute contains the body of the EnumMember element
    pub value: String,
}
