use crate::table_definition::TableDefinition;

pub struct Query {
    pub projection: String,
    pub selection: String,
}

pub enum ParseError {
    UnexpectedToken,
}

type ParseResult<T> = Result<T, ParseError>;

pub enum TableCommand {
    CreateTable(TableDefinition),
    DropTable(String),
}
