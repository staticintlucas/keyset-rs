use std::fmt;

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
            ErrorImpl::InvalidColor(_) => None,
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::types::Color;

    fn json_parse_error() -> super::Error {
        let json = serde_json::from_str::<serde_json::Value>("invalid");
        json.unwrap_err().into()
    }

    #[test]
    fn test_display_error() {
        let json_parse_error = json_parse_error();
        assert_eq!(
            format!("{}", json_parse_error),
            "error parsing JSON: expected value at line 1 column 1"
        );

        let invalid_color = Color::from_hex("invalid").unwrap_err();
        assert_eq!(
            format!("{}", invalid_color),
            "error parsing color: invalid hex code invalid"
        );
    }

    #[test]
    fn test_error_source() {
        let json_parse_error = json_parse_error();
        assert!(json_parse_error.source().is_some());

        let invalid_color = Color::from_hex("invalid").unwrap_err();
        assert!(invalid_color.source().is_none());
    }
}
