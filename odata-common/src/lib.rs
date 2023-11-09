use sea_orm::{sea_query::SimpleExpr, ColumnDef};

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
