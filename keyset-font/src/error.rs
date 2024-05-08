use std::fmt;

use ttf_parser::FaceParsingError;

/// A font error
#[derive(Debug, Clone, Copy)]
pub struct Error(FaceParsingError);

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<FaceParsingError> for Error {
    #[inline]
    fn from(error: FaceParsingError) -> Self {
        Self(error)
    }
}

/// A [`Result`](std::result::Result) where the error type is [`Error`]
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use ttf_parser::Face;

    use super::*;

    #[test]
    fn error_fmt() {
        let error = crate::Font::from_ttf(b"invalid".to_vec()).unwrap_err();

        assert_eq!(format!("{error}"), "unknown magic");
    }

    #[test]
    fn error_source() {
        let error = crate::Font::from_ttf(b"invalid".to_vec()).unwrap_err();

        assert!(error.source().is_some());
        assert_eq!(format!("{}", error.source().unwrap()), "unknown magic");
    }

    #[test]
    fn error_from() {
        let result = Face::parse(b"invalid", 0);
        let error: Error = result.unwrap_err().into();

        assert_eq!(format!("{error}"), "unknown magic");
    }
}
