/// An error that can occur during initialization
#[derive(Debug)]
pub enum InitializationError {
    /// Parsing the arguments failed
    ArgumentParseFailed(argparse::Error<'static>),
}

impl std::error::Error for InitializationError {}

impl std::fmt::Display for InitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitializationError::ArgumentParseFailed(error) => {
                write!(f, "invalid arguments - {}", error)
            }
        }
    }
}
impl From<argparse::Error<'static>> for InitializationError {
    fn from(error: argparse::Error<'static>) -> Self {
        InitializationError::ArgumentParseFailed(error)
    }
}
