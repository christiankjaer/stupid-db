mod database;
mod query;
mod table;
mod table_definition;

use table::ColumnData;
use table::Row;

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

    let table_name = table_def.name.clone();

    db.new_table(table_def);
    db.insert(
        table_name,
        Row::new(vec![("Foo".into(), ColumnData::StringData("Bar".into()))]),
    );

    Ok(())
}
