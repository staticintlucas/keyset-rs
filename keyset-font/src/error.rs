use std::fmt;

#[derive(Debug)]
pub struct Error(ttf_parser::FaceParsingError);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<ttf_parser::FaceParsingError> for Error {
    fn from(error: ttf_parser::FaceParsingError) -> Self {
        Self(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::*;

    #[test]
    fn error_fmt() {
        let error = crate::Font::from_ttf(b"invalid").unwrap_err();

        assert_eq!(format!("{error}"), "unknown magic")
    }

    #[test]
    fn error_source() {
        let error = crate::Font::from_ttf(b"invalid").unwrap_err();

        assert!(error.source().is_some());
        assert_eq!(format!("{}", error.source().unwrap()), "unknown magic",)
    }

    #[test]
    fn error_from() {
        let result = ttf_parser::Face::parse(b"invalid", 0);
        let error: Error = result.unwrap_err().into();

        assert_eq!(format!("{error}"), "unknown magic")
    }
}
