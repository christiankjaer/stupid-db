use std::collections::{BTreeMap, HashMap};

use crate::table_definition::TableDefinition;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum ColumnData {
    StringData(String),
}

struct Row {
    columns: HashMap<String, ColumnData>,
}

pub struct Table {
    pub table_definition: TableDefinition,
    data: BTreeMap<ColumnData, Row>,
}

impl Table {
    pub fn new(table_def: TableDefinition) -> Self {
        Table {
            table_definition: table_def,
            data: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, row: Row) {
        let key = row
            .columns
            .get(&self.table_definition.primary_key)
            .unwrap()
            .clone();

        self.data.insert(key, row);
    }
}
