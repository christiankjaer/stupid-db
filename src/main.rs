mod query;
mod database;
mod table;
mod table_definition;

use crate::database::Database;
use crate::table_definition::Field;
use crate::table_definition::TableDefinition;
use crate::table_definition::Type;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut db = Database::new();

    let table_def = TableDefinition {
        name: "Table".to_string(),
        primary_key: "Foo".to_string(),
        fields: vec![Field {
            name: "Foo".to_string(),
            field_type: Type::StringType,
            optional: false,
        }],
    };

    db.new_table(table_def);

    Ok(())
}
