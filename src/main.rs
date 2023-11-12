mod query;
mod table;
mod table_definition;

use std::collections::HashMap;

use table::Table;

use crate::table_definition::Field;
use crate::table_definition::TableDefinition;
use crate::table_definition::Type;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut tables: HashMap<String, Table> = HashMap::new();

    let table_def = TableDefinition {
        name: "Table".to_string(),
        primary_key: "Foo".to_string(),
        fields: vec![Field {
            name: "Foo".to_string(),
            field_type: Type::StringType,
            optional: false,
        }],
    };

    tables.insert("Table".to_string(), Table::new(table_def));

    Ok(())
}
