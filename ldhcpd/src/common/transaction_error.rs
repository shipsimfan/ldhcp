use router::{
    sqlite::{SQLite3ExecuteError, SQLite3FromRowError, SQLiteError},
    ContentType, HTTPResponse, HTTPStatus, IntoResponse,
};

pub enum SQLTransactionError {
    Transaction(SQLite3ExecuteError),
    Statements(SQLite3FromRowError),
    Custom(&'static str),
}

impl From<&'static str> for SQLTransactionError {
    fn from(str: &'static str) -> Self {
        SQLTransactionError::Custom(str)
    }
}

impl From<SQLite3ExecuteError> for SQLTransactionError {
    fn from(error: SQLite3ExecuteError) -> Self {
        SQLTransactionError::Transaction(error)
    }
}

impl From<SQLite3FromRowError> for SQLTransactionError {
    fn from(error: SQLite3FromRowError) -> Self {
        SQLTransactionError::Statements(error)
    }
}

impl From<SQLiteError> for SQLTransactionError {
    fn from(error: SQLiteError) -> Self {
        SQLTransactionError::Statements(SQLite3FromRowError::Database(error))
    }
}

impl<'a> IntoResponse<'a> for SQLTransactionError {
    fn into_response(self, content_type: ContentType) -> HTTPResponse<'a> {
        HTTPResponse::new(
            HTTPStatus::InternalServerError,
            match self {
                SQLTransactionError::Transaction(error) => {
                    content_type.serialize(&error.to_string())
                }
                SQLTransactionError::Statements(error) => {
                    content_type.serialize(&error.to_string())
                }
                SQLTransactionError::Custom(error) => content_type.serialize(error),
            }
            .unwrap(),
            content_type.as_bytes(),
        )
    }
}
