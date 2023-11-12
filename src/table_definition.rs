pub enum Type {
    StringType,
}

pub struct Field {
    pub name: String,
    pub field_type: Type,
    pub optional: bool,
}

pub struct TableDefinition {
    pub name: String,
    pub primary_key: String,
    pub fields: Vec<Field>,
}
