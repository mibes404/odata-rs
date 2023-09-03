use heck::ToSnakeCase;
use odata_model::resource::{Chain, FieldFilter, FilterOperation, Filters, ODataResource, Value};
use odata_model::resource::{OrderBy, OrderByDirection};
use sea_orm::entity::prelude::*;
use sea_orm::entity::Iterable;
use sea_orm::{
    sea_query::{ColumnRef, Expr, Func, IntoCondition, SimpleExpr},
    Condition, EntityTrait, QueryFilter, QueryOrder, Select,
};
use sea_orm::{IntoSimpleExpr, Order};

#[cfg(test)]
mod tests;

pub struct ColumnList {
    keys: Vec<String>,
    values: Vec<SimpleExpr>,
}

pub struct ColumnListIterator<'a> {
    items: &'a ColumnList,
    index: usize,
}

impl ColumnList {
    pub fn contains_key(&self, key: &str) -> bool {
        self.keys.contains(&key.to_string())
    }

    pub fn get(&self, key: &str) -> Option<&SimpleExpr> {
        self.values.get(self.keys.iter().position(|k| k == key)?)
    }

    pub fn iter(&self) -> ColumnListIterator {
        ColumnListIterator { items: self, index: 0 }
    }

    pub fn keys(&self) -> Vec<&str> {
        self.keys.iter().map(|k| k.as_str()).collect()
    }
}

impl FromIterator<(String, SimpleExpr)> for ColumnList {
    fn from_iter<T: IntoIterator<Item = (String, SimpleExpr)>>(iter: T) -> Self {
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
    type Item = (&'i str, &'i SimpleExpr);

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
        let columns = get_column_names::<E>();

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

pub fn get_column_names<E: EntityTrait>() -> ColumnList {
    E::Column::iter()
        .map(|col| col.as_column_ref())
        .map(|(_entity, col)| (col.to_string(), SimpleExpr::Column(ColumnRef::Column(col))))
        .collect()
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
    let mut condition = if let Some((_, chain)) = filters.0.first() {
        build_condition_from_chain(chain)
    } else {
        Condition::all()
    };

    for (field_filter, chain) in filters.iter() {
        let mut contents_condition = build_condition_from_chain(chain);

        match field_filter {
            FieldFilter::Contents(c) => {
                let snaked = c.field.to_snake_case();
                if let Some(col) = table_columns.get(&snaked) {
                    contents_condition = contents_condition.add(compare_opp(col.clone(), &c.operation, c.not));
                }

                if c.not {
                    contents_condition = contents_condition.not();
                }

                condition = condition.add(contents_condition);
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

    condition
}

fn build_condition_from_chain(chain: &Option<Chain>) -> Condition {
    if let Some(chain) = chain {
        match chain {
            Chain::And => Condition::all(),
            Chain::Or => Condition::any(),
        }
    } else {
        Condition::all()
    }
}

fn like_opp(column: SimpleExpr, pattern: &str) -> SimpleExpr {
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
