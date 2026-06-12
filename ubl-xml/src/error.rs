// XML error types.

use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// XML parsing/writing error from quick-xml
    Xml(String),
    /// Missing required field or element
    MissingField(String),
    /// Unexpected XML structure
    UnexpectedStructure(String),
    /// I/O error
    Io(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xml(msg) => write!(f, "XML error: {msg}"),
            Self::MissingField(name) => write!(f, "missing required field: {name}"),
            Self::UnexpectedStructure(msg) => write!(f, "unexpected XML structure: {msg}"),
            Self::Io(err) => write!(f, "I/O error: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Self::Xml(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
