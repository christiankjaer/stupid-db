pub struct Query {
    pub projection: String,
    pub selection: String,
}

pub enum ParseError {
    UnexpectedToken
}

type ParseResult<T> = Result<T, ParseError>;
