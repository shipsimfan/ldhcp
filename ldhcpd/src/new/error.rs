/// An error that can occur during initialization
#[derive(Debug)]
pub enum CreationError {
    /// Parsing the arguments failed
    ArgumentParseFailed(argparse::Error<'static>),

    /// Unable to open a log output
    OpenLogOutputFailed(oak::OpenStdLogOutputError),

    /// Unable to create the log controller
    CreateLogControllerFailed(std::io::Error),
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
