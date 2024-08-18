use sqlite::{SQLite3ExecuteError, SQLite3FromRowError, SQLiteError};

/// An error that can occur while interacting with a database
#[derive(Debug)]
pub enum DatabaseError {
    Execute(SQLite3ExecuteError),
    Prepare(SQLiteError),
    FromRow(SQLite3FromRowError),
}

impl std::error::Error for DatabaseError {}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::Execute(error) => error.fmt(f),
            DatabaseError::Prepare(error) => error.fmt(f),
            DatabaseError::FromRow(error) => error.fmt(f),
        }
    }
}

impl From<SQLite3ExecuteError> for DatabaseError {
    fn from(error: SQLite3ExecuteError) -> Self {
        DatabaseError::Execute(error)
    }
}

impl From<SQLiteError> for DatabaseError {
    fn from(error: SQLiteError) -> Self {
        DatabaseError::Prepare(error)
    }
}

impl From<SQLite3FromRowError> for DatabaseError {
    fn from(error: SQLite3FromRowError) -> Self {
        DatabaseError::FromRow(error)
    }
}
