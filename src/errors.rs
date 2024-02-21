use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LioneError {
    MethodNotFound(String),
    ParseError(String),
    EmptyRequest,
    IoError(String),
    TodoError
}

impl fmt::Display for LioneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LioneError::MethodNotFound(method) => write!(f, "HTTP Method {} wasn't found!", method),
            LioneError::ParseError(msg) => write!(f, "Coulnd't parse JSON: {}", msg),
            LioneError::EmptyRequest => write!(f, "Empty request"),
            LioneError::IoError(msg) => write!(f, "IoError: {}", msg),
            LioneError::TodoError => write!(f, "TodoError")
        }
    }
}

impl Error for LioneError {}

impl From<serde_json::Error> for LioneError {
    fn from(err: serde_json::Error) -> LioneError {
        LioneError::ParseError(err.to_string())
    }
}

impl From<IoError> for LioneError {
    fn from(err: IoError) -> LioneError {
        LioneError::IoError(err.to_string())
    }
}
