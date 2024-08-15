/// An error that can occur during initialization
#[derive(Debug)]
pub enum InitializationError {
    /// Parsing the arguments failed
    ArgumentParseFailed(argparse::Error<'static>),

    /// Unable to open a log output
    OpenLogOutputFailed(std::io::Error),

    /// Unable to create the log controller
    CreateLogControllerFailed(std::io::Error),
}

impl std::error::Error for InitializationError {}

impl std::fmt::Display for InitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitializationError::ArgumentParseFailed(error) => {
                write!(f, "invalid arguments - {}", error)
            }
            InitializationError::OpenLogOutputFailed(error) => {
                write!(f, "unable to open a log output - {}", error)
            }
            InitializationError::CreateLogControllerFailed(error) => {
                write!(f, "unable to create the log controller - {}", error)
            }
        }
    }
}
impl From<argparse::Error<'static>> for InitializationError {
    fn from(error: argparse::Error<'static>) -> Self {
        InitializationError::ArgumentParseFailed(error)
    }
}
