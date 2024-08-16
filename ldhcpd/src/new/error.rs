use std::path::PathBuf;

/// An error that can occur during initialization
#[derive(Debug)]
pub enum CreationError {
    /// Parsing the arguments failed
    ArgumentParseFailed(argparse::Error<'static>),

    /// Unable to open a log output
    OpenLogOutputFailed(oak::OpenStdLogOutputError),

    /// Unable to create the log controller
    CreateLogControllerFailed(std::io::Error),

    /// Unable to open the database
    OpenDatabaseFailed(sqlite::SQLiteError, PathBuf),

    /// Unable to migrate the database
    MigrateDatabaseFailed(sql_migrations::MigrationError, PathBuf),
}

impl std::error::Error for CreationError {}

impl std::fmt::Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreationError::ArgumentParseFailed(error) => {
                write!(f, "invalid arguments - {}", error)
            }
            CreationError::OpenLogOutputFailed(error) => {
                write!(f, "unable to open a log output - {}", error)
            }
            CreationError::CreateLogControllerFailed(error) => error.fmt(f),
            CreationError::OpenDatabaseFailed(error, path) => write!(
                f,
                "unable to open database \"{}\" - {}",
                path.display(),
                error
            ),
            CreationError::MigrateDatabaseFailed(error, path) => write!(
                f,
                "unable to migrate database \"{}\" - {}",
                path.display(),
                error
            ),
        }
    }
}
impl From<argparse::Error<'static>> for CreationError {
    fn from(error: argparse::Error<'static>) -> Self {
        CreationError::ArgumentParseFailed(error)
    }
}

impl From<oak::OpenStdLogOutputError> for CreationError {
    fn from(error: oak::OpenStdLogOutputError) -> Self {
        CreationError::OpenLogOutputFailed(error)
    }
}
