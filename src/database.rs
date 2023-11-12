use std::collections::HashMap;

use crate::{
    table::{Row, Table},
    table_definition::TableDefinition,
};

pub struct Database {
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    pub fn new_table(&mut self, table_definition: TableDefinition) {
        let table_name = table_definition.name.clone();
        let table = Table::new(table_definition);
        self.tables.insert(table_name, table);
    }

    pub fn insert(&mut self, table_name: String, row: Row) {
        let table = self.tables.get_mut(&table_name).unwrap();
        table.insert(row);
    }
}
