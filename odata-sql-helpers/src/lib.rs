use heck::ToSnakeCase;
use odata_model::resource::{Chain, FieldFilter, FilterOperation, ODataResource, Value};
use sea_orm::entity::prelude::*;
use sea_orm::entity::Iterable;
use sea_orm::{
    sea_query::{ColumnRef, Expr, Func, IntoCondition, SimpleExpr},
    Condition, EntityTrait, QueryFilter, Select,
};

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

/// Apply the common Filter to the SeaOrm query
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

        // if let Some(sort_by) = &filter.sort_by {
        //     let snaked = sort_by.to_snake_case();
        //     let col = SqlCol::new(snaked);
        //     let order = if filter.descending { Order::Desc } else { Order::Asc };
        //     query = query.order_by(col, order)
        // }

        query
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
        let mut filter_condition = Condition::all();

        for (field_filter, chain) in resource.filters.iter() {
            filter_condition = if let Some(chain) = chain {
                match chain {
                    Chain::And => Condition::all(),
                    Chain::Or => Condition::any(),
                }
            } else {
                Condition::all()
            };

            match field_filter {
                FieldFilter::Contents(c) => {
                    let mut contents_condition = Condition::any();

                    let snaked = c.field.to_snake_case();
                    if let Some(col) = table_columns.get(&snaked) {
                        contents_condition = contents_condition.add(compare_opp(col.clone(), &c.operation, c.not));
                    }

                    filter_condition = filter_condition.add(contents_condition);
                }
                FieldFilter::Nested(nested) => {}
            }

            // let snaked = field.to_snake_case();
            // let col = SqlCol::new(snaked);
            // filter_condition = filter_condition.add(compare_opp(col, value));
        }

        condition = condition.add(filter_condition);
    }

    condition
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
            // value if value == "true" || value == "false" => {
            //     let val = bool::from_str(value).unwrap_or_default();
            //     Expr::col(col_val).eq(val)
            // }
            // value if value == "!true" || value == "!false" => {
            //     let val = bool::from_str(value).unwrap_or_default();
            //     Expr::col(col_val).ne(val)
            // }
            // value if value.starts_with("gtd") => {
            //     let timestamp_ms = i64::from_str(&pattern[3..]).unwrap_or_default();
            //     let date = date_from_ms(timestamp_ms);
            //     info!("Searching on date later than {} ({})", date, timestamp_ms);
            //     Expr::col(col_val).gt(date)
            // }
            // value if value.starts_with("gted") => {
            //     let timestamp_ms = i64::from_str(&pattern[4..]).unwrap_or_default();
            //     let date = date_from_ms(timestamp_ms);
            //     info!("Searching on date later than or equal to {} ({})", date, timestamp_ms);
            //     Expr::col(col_val).gte(date)
            // }
            // value if value.starts_with("gte") => {
            //     let ival = i32::from_str(&pattern[3..]).unwrap_or_default();
            //     Expr::col(col_val).gte(ival)
            // }
            // value if value.starts_with("gt") => {
            //     let ival = i32::from_str(&pattern[2..]).unwrap_or_default();
            //     Expr::col(col_val).gt(ival)
            // }
            // value if value.starts_with("ltd") => {
            //     let timestamp_ms = i64::from_str(&pattern[3..]).unwrap_or_default();
            //     let date = date_from_ms(timestamp_ms);
            //     info!("Searching on date later than {} ({})", date, timestamp_ms);
            //     Expr::col(col_val).lt(date)
            // }
            // value if value.starts_with("lted") => {
            //     let timestamp_ms = i64::from_str(&pattern[3..]).unwrap_or_default();
            //     let date = date_from_ms(timestamp_ms);
            //     info!("Searching on date later than or equal to {} ({})", date, timestamp_ms);
            //     Expr::col(col_val).lte(date)
            // }
            // value if value.starts_with("lte") => {
            //     let ival = i32::from_str(&pattern[3..]).unwrap_or_default();
            //     Expr::col(col_val).lte(ival)
            // }
            // value if value.starts_with("lt") => {
            //     let ival = i32::from_str(&pattern[2..]).unwrap_or_default();
            //     Expr::col(col_val).lt(ival)
            // }
            // value if value.starts_with("eq") => {
            //     let ival = i32::from_str(&pattern[2..]).unwrap_or_default();
            //     Expr::col(col_val).eq(ival)
            // }
            // value if value.starts_with("::") => {
            //     // pick from enumeration
            //     let shouty_field = format!("{}_", col_val.0.to_shouty_snake_case());
            //     let str_val = pattern[2..].trim_start_matches(&shouty_field).to_upper_camel_case();
            //     Expr::col(col_val).eq(str_val)
            // }
            // default => {
            //     let like = format!("%{}%", default.to_lowercase());
            //     info!("searching default {like}");
            //     Expr::expr(Func::lower(Expr::col(col_val))).like(like)
            // }
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
