use std::fmt;

use serde_json;

use crate::types::InvalidColor;

#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorImpl>,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum ErrorImpl {
    JsonParseError(serde_json::Error),
    InvalidColor(InvalidColor),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.inner {
            ErrorImpl::JsonParseError(error) => {
                write!(f, "error parsing JSON: {}", error)
            }
            ErrorImpl::InvalidColor(error) => {
                write!(f, "error parsing color: {}", error)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &*self.inner {
            ErrorImpl::JsonParseError(error) => Some(error),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self {
            inner: Box::new(ErrorImpl::JsonParseError(error)),
        }
    }
}

impl From<InvalidColor> for Error {
    fn from(error: InvalidColor) -> Self {
        Self {
            inner: Box::new(ErrorImpl::InvalidColor(error)),
        }
    }
}
