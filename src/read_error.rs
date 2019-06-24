#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParseErrorKind {
    PrimitiveError,
    OsuDbError,
    CollectionDbError,
    ScoresDbError,
    QueryError,
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DbFileParseError {
    kind: ParseErrorKind,
    message: String
}

impl DbFileParseError {
    pub fn new<T: Into<String>>(kind: ParseErrorKind, message: T) -> Self {
        DbFileParseError {
            kind,
            message: message.into()
        }
    }
}

pub type ParseFileResult<T> = std::result::Result<T, DbFileParseError>;