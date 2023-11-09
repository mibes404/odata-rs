use heck::ToSnakeCase;
use odata_model::resource::{Chain, FieldFilter, FilterOperation, Filters, ODataResource, Value};
use odata_model::resource::{OrderBy, OrderByDirection};
use sea_orm::entity::prelude::*;
use sea_orm::entity::Iterable;
use sea_orm::{
    sea_query::{ColumnRef, Expr, Func, IntoCondition, SimpleExpr},
    Condition, EntityTrait, QueryFilter, QueryOrder, Select,
};
use sea_orm::{IntoSimpleExpr, Order, QuerySelect};

pub mod reflect;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct ColumnList {
    keys: Vec<String>,
    values: Vec<ColumnValue>,
}

#[derive(Debug)]
pub struct PrimaryKeys {
    p_keys: Vec<String>,
}

impl PrimaryKeys {
    pub fn new(p_keys: Vec<String>) -> Self {
        Self { p_keys }
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.p_keys.iter().map(|k| k.as_str())
    }

    pub fn keys(&self) -> Vec<&str> {
        self.p_keys.iter().map(|k| k.as_str()).collect()
    }
}

impl FromIterator<String> for PrimaryKeys {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut p_keys = Vec::new();

        for key in iter {
            p_keys.push(key);
        }

        Self { p_keys }
    }
}

#[derive(Clone, Debug)]
pub struct ColumnValue {
    pub column: SimpleExpr,
    pub def: ColumnDef,
}

impl From<(SimpleExpr, ColumnDef)> for ColumnValue {
    fn from((column, def): (SimpleExpr, ColumnDef)) -> Self {
        Self { column, def }
    }
}

pub struct ColumnListIterator<'a> {
    items: &'a ColumnList,
    index: usize,
}

impl ColumnList {
    pub fn contains_key(&self, key: &str) -> bool {
        self.keys.contains(&key.to_string())
    }

    pub fn get(&self, key: &str) -> Option<&ColumnValue> {
        let pos = self.keys.iter().position(|k| k == key)?;
        self.values.get(pos)
    }

    pub fn iter(&self) -> ColumnListIterator {
        ColumnListIterator { items: self, index: 0 }
    }

    pub fn keys(&self) -> Vec<&str> {
        self.keys.iter().map(|k| k.as_str()).collect()
    }
}

impl FromIterator<(String, ColumnValue)> for ColumnList {
    fn from_iter<T: IntoIterator<Item = (String, ColumnValue)>>(iter: T) -> Self {
        let mut keys = Vec::new();
        let mut values = Vec::new();

        for (key, value) in iter {
            keys.push(key);
            values.push(value);
        }

        Self { keys, values }
    }
}

impl<'i> Iterator for ColumnListIterator<'i> {
    type Item = (&'i str, &'i ColumnValue);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.keys.len() {
            let key = &self.items.keys[self.index];
            let value = &self.items.values[self.index];
            self.index += 1;
            Some((key.as_str(), value))
        } else {
            None
        }
    }
}

/// Apply the ODataResource filter to the SeaOrm query
/// ```ignore
/// use odata_model::resource::ODataResource;
/// use self::WithFilterExt;
///
/// let filter = ODataResource::default();
/// SomeEntity::find().with_odata_filter(&filter);
/// ```
pub trait WithODataExt<E>
where
    E: EntityTrait,
{
    fn with_odata_resource(self, resource: &ODataResource) -> Self;
}

impl<E> WithODataExt<E> for Select<E>
where
    E: EntityTrait,
{
    fn with_odata_resource(self, resource: &ODataResource) -> Self {
        let (_pkeys, columns) = get_column_names::<E>();

        let mut query = self.filter(condition_with_filter(resource, &columns));

        for order_by in &resource.order_by {
            let OrderBy { field, direction } = order_by;
            let field = field.to_snake_case();
            let col = SimpleColumn(field).into_simple_expr();
            let order = if direction == &OrderByDirection::Desc {
                Order::Desc
            } else {
                Order::Asc
            };
            query = query.order_by(col, order)
        }

        // top and skip
        if let Some(skip) = resource.skip {
            query = query.offset(Some(skip as u64));
        }

        if let Some(top) = resource.top {
            query = query.limit(Some(top as u64));
        }

        query
    }
}

#[derive(Debug, Clone)]
pub struct SimpleColumn(String);

impl Iden for SimpleColumn {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        let _ = s.write_str(&self.0);
    }
}

impl IntoSimpleExpr for SimpleColumn {
    fn into_simple_expr(self) -> SimpleExpr {
        SimpleExpr::Column(ColumnRef::Column(SeaRc::new(self)))
    }
}

pub fn get_column_names<E: EntityTrait>() -> (PrimaryKeys, ColumnList) {
    let p_keys = E::PrimaryKey::iter()
        .map(|pkey| pkey.into_column().as_column_ref())
        .map(|(_entity, col)| col.to_string())
        .collect();

    let column_list = E::Column::iter()
        .map(|col| (col.as_column_ref(), col.def()))
        .map(|((_entity, col), def)| {
            (
                col.to_string(),
                ColumnValue::from((SimpleExpr::Column(ColumnRef::Column(col)), def)),
            )
        })
        .collect();

    (p_keys, column_list)
}

pub fn condition_with_filter(resource: &ODataResource, table_columns: &ColumnList) -> impl IntoCondition {
    let mut condition = Condition::all();

    if let Some(search) = &resource.search {
        let mut search_condition = Condition::any();

        for (_id, col) in table_columns.iter() {
            search_condition = search_condition.add(like_opp(col.clone(), search));
        }

        condition = condition.add(search_condition);
    }

    if !resource.filters.is_empty() {
        let filters = &resource.filters;
        let filter_condition = build_condition(filters, table_columns);
        condition = condition.add(filter_condition);
    }

    condition
}

fn build_condition(filters: &Filters, table_columns: &ColumnList) -> Condition {
    let (mut condition, and_groups) = build_condition_from_chain(filters);
    let mut grouped_condition: Option<Condition> = None;

    for (pos, (field_filter, _chain)) in filters.iter().enumerate() {
        match field_filter {
            FieldFilter::Contents(c) => {
                if and_groups.within_group(pos) {
                    if grouped_condition.is_none() {
                        grouped_condition = Some(Condition::all())
                    }
                } else if let Some(use_grouped_condition) = grouped_condition.take() {
                    // add the previous grouped condition to the top level condition
                    condition = condition.add(use_grouped_condition);
                }

                let snaked = c.field.to_snake_case();
                if let Some(col) = table_columns.get(&snaked) {
                    let col = &col.column;
                    if let Some(use_grouped_condition) = grouped_condition.take() {
                        grouped_condition =
                            Some(use_grouped_condition.add(compare_opp(col.clone(), &c.operation, c.not)));
                    } else {
                        condition = condition.add(compare_opp(col.clone(), &c.operation, c.not));
                    }
                }
            }
            FieldFilter::Nested((not, filters)) => {
                let mut contents_condition = build_condition(filters, table_columns);
                if *not {
                    contents_condition = contents_condition.not();
                }
                condition = condition.add(contents_condition);
            }
        }
    }

    if let Some(use_grouped_condition) = grouped_condition.take() {
        condition = condition.add(use_grouped_condition);
    }

    condition
}

struct AndGroups(Vec<(usize, usize)>);

impl AndGroups {
    /// Determine if the provided pos is within a group of ANDs
    fn within_group(&self, pos: usize) -> bool {
        self.0.iter().any(|(start, end)| pos >= *start && pos <= *end)
    }
}

fn build_condition_from_chain(filters: &Filters) -> (Condition, AndGroups) {
    let mut top_level_condition = Condition::all();

    let mut first_and_pos: Option<usize> = None;
    let mut and_groups: Vec<(usize, usize)> = vec![];

    for (pos, (_filter, chain)) in filters.0.iter().enumerate() {
        if let Some(chain) = chain {
            match chain {
                Chain::And => {
                    if first_and_pos.is_none() {
                        first_and_pos = Some(pos);
                    }
                }
                Chain::Or => {
                    if let Some(use_first_and_pos) = first_and_pos {
                        if use_first_and_pos != pos - 1 {
                            and_groups.push((use_first_and_pos, pos - 1));
                        }

                        first_and_pos = None;
                    }
                    top_level_condition = Condition::any();
                }
            }
        }
    }

    // close the group if we have one
    if let Some(use_first_and_pos) = first_and_pos {
        let pos = filters.0.len() - 1;
        if use_first_and_pos != pos {
            and_groups.push((use_first_and_pos, pos));
        }
    }

    (top_level_condition, AndGroups(and_groups))
}

fn like_opp(column: ColumnValue, pattern: &str) -> SimpleExpr {
    let column = column.column;
    let like = format!("%{}%", pattern.to_lowercase());
    Expr::expr(Func::lower(column)).like(like)
}

fn into_simple_expr(v: &Value) -> SimpleExpr {
    match v {
        Value::String(s) => SimpleExpr::from(s),
        Value::Integer(n) => (*n).into(),
        Value::Decimal(d) => (*d).into(),
        Value::Boolean(b) => (*b).into(),
        _ => panic!("Not implemented"),
    }
}

fn null_or(value: &Value, col: ColumnRef, expr: SimpleExpr) -> SimpleExpr {
    match value {
        Value::Null => Expr::col(col).is_null(),
        _ => expr,
    }
}

fn not_null_or(value: &Value, col: ColumnRef, expr: SimpleExpr) -> SimpleExpr {
    match value {
        Value::Null => Expr::col(col).is_not_null(),
        _ => expr,
    }
}

fn compare_opp(column: SimpleExpr, operation: &FilterOperation, negate: bool) -> SimpleExpr {
    let expression = if let SimpleExpr::Column(col) = column {
        match operation {
            FilterOperation::Eq(value) => null_or(value, col.clone(), Expr::col(col).eq(into_simple_expr(value))),
            FilterOperation::Ne(value) => not_null_or(value, col.clone(), Expr::col(col).ne(into_simple_expr(value))),
            FilterOperation::Gt(value) => Expr::col(col).gt(into_simple_expr(value)),
            FilterOperation::Ge(value) => Expr::col(col).gte(into_simple_expr(value)),
            FilterOperation::Lt(value) => Expr::col(col).lt(into_simple_expr(value)),
            FilterOperation::Le(value) => Expr::col(col).lte(into_simple_expr(value)),
            FilterOperation::In(values) => {
                let values = values.iter().map(into_simple_expr);
                Expr::col(col).is_in(values)
            }
            FilterOperation::Has(_) => todo!(),
            FilterOperation::Function(_) => todo!(),
        }
    } else {
        panic!("Not implemented for non-columns");
    };

    if negate {
        expression.not()
    } else {
        expression
    }
}
