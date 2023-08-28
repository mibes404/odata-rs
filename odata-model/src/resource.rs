use super::*;

#[derive(Debug)]
pub struct ODataResource {
    pub entity: Entity,
    pub kind: ODataResourceKind,
    pub url: String,
    pub title: Option<String>,
    pub property: Option<String>,
    pub operation: Option<Operation>,
    pub relationships: Vec<Entity>,
    pub search: Option<String>,
    pub filters: Vec<(FieldFilter, Option<Chain>)>,
}

#[derive(Debug, Default)]
pub enum ODataResourceKind {
    #[default]
    EntitySet,
    Singleton,
    FunctionImport,
    ServiceDocument,
}

#[derive(Debug, PartialEq)]
pub enum Key {
    String(String),
    Number(i32),
    KeyValue((String, Value)),
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub key: Option<Key>,
}

#[derive(Debug)]
pub struct FieldFilter {
    pub not: bool,
    pub field: String,
    pub operation: FilterOperation,
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.key {
            Some(key) => write!(f, "{}({})", self.name, key),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FilterOperation {
    Eq(Value),
    Ne(Value),
    Gt(Value),
    Ge(Value),
    Lt(Value),
    Le(Value),
    In(Vec<Value>),
    Has(String),
    Function(String),
}

#[derive(Debug, PartialEq)]
pub enum Chain {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Count,
    Value,
    All,
}

impl TryFrom<&str> for Operation {
    type Error = ODataError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "$count" => Ok(Self::Count),
            "$value" => Ok(Self::Value),
            "$all" => Ok(Self::All),
            _ => Err(ODataError::InvalidOperation),
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::String(value) => write!(f, "{}", value),
            Key::Number(value) => write!(f, "{}", value),
            Key::KeyValue((name, value)) => write!(f, "{}={}", name, value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    String(String),
    Integer(i32),
    Decimal(rust_decimal::Decimal),
    QueryOption(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(value) => write!(f, "{}", value),
            Value::Integer(value) => write!(f, "{}", value),
            Value::QueryOption(value) => write!(f, "@{}", value),
            Value::Decimal(value) => write!(f, "{}", value),
        }
    }
}

const PARSE_PREFIX: &str = "http://services.odata.org/V4/TripPinService/";

impl TryFrom<&str> for ODataResource {
    type Error = ODataError;

    /// Try to create a resource from the path of an URL. The path is expected to start with the name of the resource.
    /// For example: People('russellwhyte')/FirstName
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim_start_matches('/');
        let value = format!("{PARSE_PREFIX}{value}");
        let url = Url::parse(&value)?;
        let mut result = match parse_path(&url, value) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        for (key, value) in url.query_pairs() {
            if key == "$search" {
                result.search = Some(value.to_string());
            }

            if key == "$filter" {
                result.filters = parse_filter(value.to_string())?;
            }
        }

        Ok(result)
    }
}

impl TryFrom<&mut Split<'_, char>> for FieldFilter {
    type Error = ODataError;

    fn try_from(parts: &mut Split<'_, char>) -> Result<Self, Self::Error> {
        let field = parts.next().ok_or(error::ODataError::IncompletePath)?;
        if field.to_lowercase() == "not" {
            let function = parts.next().ok_or(error::ODataError::IncompletePath)?;

            Ok(Self {
                not: true,
                field: field.to_string(),
                operation: FilterOperation::Function(function.to_string()),
            })
        } else {
            let operation = parts.next().ok_or(error::ODataError::IncompletePath)?;
            let value = parts.next().ok_or(error::ODataError::IncompletePath)?;

            Ok(Self {
                not: false,
                field: field.to_string(),
                operation: match operation.to_lowercase().as_str() {
                    "eq" => FilterOperation::Eq(extract_value(value)),
                    "ne" => FilterOperation::Ne(extract_value(value)),
                    "gt" => FilterOperation::Gt(extract_value(value)),
                    "ge" => FilterOperation::Ge(extract_value(value)),
                    "lt" => FilterOperation::Lt(extract_value(value)),
                    "le" => FilterOperation::Le(extract_value(value)),
                    "in" => FilterOperation::In(eat_in_list_parts(Some(value), parts)),
                    "has" => FilterOperation::Has(value.to_string()),
                    _ => FilterOperation::Function(value.to_string()),
                },
            })
        }
    }
}

fn eat_in_list_parts(mut first_value: Option<&str>, parts: &mut Split<'_, char>) -> Vec<Value> {
    let mut values = Vec::new();

    if let Some(first_value) = first_value.take().map(|v| v.trim_start_matches('(')) {
        let inner_parts = first_value.split(',');
        let inner_parts = eat_inner_list_parts(&mut inner_parts.into_iter());
        values.extend(inner_parts);
    }

    for value in parts.by_ref() {
        let inner_parts = value.split(',');
        let inner_parts = eat_inner_list_parts(&mut inner_parts.into_iter());
        values.extend(inner_parts);
    }

    values
}

fn eat_inner_list_parts(parts: &mut Split<'_, char>) -> Vec<Value> {
    let mut values = Vec::new();

    for value in parts.by_ref() {
        if value.is_empty() {
            continue;
        }

        if value.ends_with(')') {
            let value = value.trim_end_matches(')');
            values.push(extract_value(value));
            break;
        }

        values.push(extract_value(value));
    }

    values
}

/// Extract filters from a query string. It supports these patterns:
/// - Name eq 'Milk'
/// - Name ne 'Milk'
/// - Name gt 'Milk'
/// - Name ge 'Milk'
/// - Name lt 'Milk'
/// - Name le 'Milk'
/// - Name eq 'Milk' and Price lt 2.55
/// - Name eq 'Milk' or Price lt 2.55
/// - not endswith(Name,'ilk')
/// - style has Sales.Pattern'Yellow'
/// - Name in ('Milk', 'Cheese')
fn parse_filter(filter_value: String) -> ODataResult<Vec<(FieldFilter, Option<Chain>)>> {
    let mut filters = Vec::new();
    let mut parts = filter_value.split(' ');

    loop {
        let filter = FieldFilter::try_from(&mut parts)?;
        if let Some(chain) = parts.next() {
            let chain = match chain {
                "and" => Chain::And,
                "or" => Chain::Or,
                _ => panic!("Invalid chain"),
            };

            filters.push((filter, Some(chain)));
        } else {
            filters.push((filter, None));
            break;
        }
    }

    Ok(filters)
}

fn parse_path(url: &Url, value: String) -> ODataResult<ODataResource> {
    match url.path_segments() {
        None => Err(ODataError::IncompletePath),
        Some(mut parts) => {
            let Some(name) = parts.nth(2) else {
                return Err(ODataError::IncompletePath);
            };

            let name = percent_decode_str(name).decode_utf8_lossy();
            let entity = extract_entity(&name);

            let mut part;
            let mut relationships = vec![];
            let mut property: Option<String> = None;
            let mut operation = None;

            loop {
                part = interpret_next_part(&mut parts);

                match part {
                    Some(NextPart::Part(part)) => {
                        if let Some(property) = property.take() {
                            // there was more to parse, so this isn't the end of the resource, i.e. not a property
                            relationships.push(extract_entity(&property));
                        }

                        property = Some(part.to_string());
                    }
                    Some(NextPart::Operation(part)) => {
                        operation = Some(part);
                    }
                    None => break,
                };
            }

            Ok(ODataResource {
                entity,
                kind: ODataResourceKind::EntitySet,
                url: value.to_string(),
                title: None,
                property,
                operation,
                relationships,
                search: None,
                filters: Vec::new(),
            })
        }
    }
}

enum NextPart<'s> {
    Part(&'s str),
    Operation(Operation),
}

fn interpret_next_part<'p>(parts: &'p mut Split<'_, char>) -> Option<NextPart<'p>> {
    let part = parts.next();

    part.map(|part| match Operation::try_from(part) {
        Ok(operation) => NextPart::Operation(operation),
        _ => NextPart::Part(part),
    })
}

/// Extract the name and key from a resource name, e.g. People('O''Neil') -> (People, Some(O'Neil))
fn extract_entity(name: &str) -> Entity {
    if name.contains("('") && name.contains("')") {
        let mut parts = name.split("('");
        let name = parts.next().unwrap();
        let key = parts.next().unwrap();
        let key = key.trim_end_matches("')");
        let key = key.replace("''", "'");

        return Entity {
            name: name.to_string(),
            key: Some(Key::String(key)),
        };
    }

    if name.contains('(') && name.contains(')') {
        let mut parts = name.split('(');
        let name = parts.next().unwrap();
        let key = parts.next().unwrap();
        let key = key.trim_end_matches(')');

        if key.contains('=') {
            let mut parts = key.split('=');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();

            if value.starts_with('\'') && value.ends_with('\'') {
                let value = value.trim_start_matches('\'').trim_end_matches('\'');
                return Entity {
                    name: name.to_string(),
                    key: Some(Key::KeyValue((key.to_string(), Value::String(value.to_string())))),
                };
            }

            if value.starts_with('@') {
                let value = value.trim_start_matches('@');
                return Entity {
                    name: name.to_string(),
                    key: Some(Key::KeyValue((key.to_string(), Value::QueryOption(value.to_string())))),
                };
            }

            if let Ok(num) = value.parse::<i32>() {
                return Entity {
                    name: name.to_string(),
                    key: Some(Key::KeyValue((key.to_string(), Value::Integer(num)))),
                };
            }

            return Entity {
                name: name.to_string(),
                key: None,
            };
        }

        if let Ok(num) = key.parse::<i32>() {
            return Entity {
                name: name.to_string(),
                key: Some(Key::Number(num)),
            };
        }
    }

    Entity {
        name: name.to_string(),
        key: None,
    }
}

fn extract_value(value: &str) -> Value {
    if value.starts_with('\'') && value.ends_with('\'') {
        let value = value.trim_start_matches('\'').trim_end_matches('\'');
        return Value::String(value.to_string());
    }

    if value.starts_with('@') {
        let value = value.trim_start_matches('@');
        return Value::QueryOption(value.to_string());
    }

    if let Ok(num) = value.parse::<i32>() {
        return Value::Integer(num);
    }

    if let Ok(num) = rust_decimal::Decimal::from_str(value) {
        return Value::Decimal(num);
    }

    Value::Null
}

impl From<ServiceDocumentValue> for ODataResource {
    fn from(value: ServiceDocumentValue) -> Self {
        Self {
            entity: extract_entity(&value.name),
            kind: match value.kind {
                Some(kind) => match kind.as_str() {
                    "Singleton" => ODataResourceKind::Singleton,
                    "FunctionImport" => ODataResourceKind::FunctionImport,
                    "ServiceDocument" => ODataResourceKind::ServiceDocument,
                    _ => ODataResourceKind::EntitySet,
                },
                None => ODataResourceKind::EntitySet,
            },
            url: value.url,
            title: value.title,
            property: None,
            operation: None,
            relationships: Vec::new(),
            search: None,
            filters: Vec::new(),
        }
    }
}
