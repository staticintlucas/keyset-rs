use std::fmt;

use rustybuzz::ttf_parser::FaceParsingError;

/// A font permissions error
///
/// See: <https://learn.microsoft.com/en-us/typography/opentype/spec/os2#fstype>
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum PermissionError {
    /// We can't use the font because it has a restricted license
    RestrictedLicense,
    /// We can't use the font because it doesn't allow subsetting
    NoSubsetting,
    /// We can't use the font because it doesn't allow outline embedding
    BitmapEmbeddingOnly,
}

impl fmt::Display for PermissionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::RestrictedLicense => write!(f, "restricted license"),
            Self::NoSubsetting => write!(f, "no subsetting"),
            Self::BitmapEmbeddingOnly => write!(f, "bitmap embedding only"),
        }
    }
}

impl std::error::Error for PermissionError {}

/// A font error
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    /// There was an error parsing the font face
    ParsingError(FaceParsingError),
    /// The font's permissions don't allow us to use it
    PermissionError(PermissionError),
    /// Missing required property
    MissingProperty(String),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ParsingError(ref error) => write!(f, "error parsing font: {error}"),
            Self::PermissionError(ref error) => write!(f, "permissions error: {error}"),
            Self::MissingProperty(ref prop) => write!(f, "missing property {prop}"),
        }
    }
}

impl std::error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::ParsingError(ref error) => Some(error),
            Self::PermissionError(ref error) => Some(error),
            Self::MissingProperty(..) => None,
        }
    }
}

impl From<FaceParsingError> for Error {
    #[inline]
    fn from(error: FaceParsingError) -> Self {
        Self::ParsingError(error)
    }
}

impl From<PermissionError> for Error {
    #[inline]
    fn from(error: PermissionError) -> Self {
        Self::PermissionError(error)
    }
}

/// A [`Result`](std::result::Result) where the error type is [`Error`]
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use rustybuzz::ttf_parser;

    use super::*;
    use crate::{Face, Font};

    #[test]
    fn error_fmt() {
        let error = Font::from_ttf(b"invalid".to_vec()).unwrap_err();
        assert_eq!(format!("{error}"), "error parsing font: unknown magic");

        let error = Face::from_ttf(std::fs::read(env!("RESTRICTED_TTF")).unwrap()).unwrap_err();
        assert_eq!(format!("{error}"), "permissions error: restricted license");

        let error = Face::from_ttf(std::fs::read(env!("NO_SUBSET_TTF")).unwrap()).unwrap_err();
        assert_eq!(format!("{error}"), "permissions error: no subsetting");

        let error =
            Face::from_ttf(std::fs::read(env!("BITMAP_EMBED_ONLY_TTF")).unwrap()).unwrap_err();
        assert_eq!(
            format!("{error}"),
            "permissions error: bitmap embedding only"
        );

        let error = Font::from_ttf(std::fs::read(env!("NULL_TTF")).unwrap()).unwrap_err();
        assert_eq!(format!("{error}"), "missing property font family");
    }

    #[test]
    fn error_source() {
        let error = Face::from_ttf(b"invalid".to_vec()).unwrap_err();
        assert!(error.source().is_some());
        assert_eq!(format!("{}", error.source().unwrap()), "unknown magic");

        let error = Face::from_ttf(std::fs::read(env!("RESTRICTED_TTF")).unwrap()).unwrap_err();
        assert!(error.source().is_some());
        assert_eq!(format!("{}", error.source().unwrap()), "restricted license");

        let error = Font::from_ttf(std::fs::read(env!("NULL_TTF")).unwrap()).unwrap_err();
        assert!(error.source().is_none());
    }

    #[test]
    fn error_from() {
        let result = ttf_parser::Face::parse(b"invalid", 0);
        let error: Error = result.unwrap_err().into();
        assert_eq!(format!("{error}"), "error parsing font: unknown magic");

        let perm_error = PermissionError::RestrictedLicense;
        let error: Error = perm_error.into();
        assert_eq!(format!("{error}"), "permissions error: restricted license");
    }
}
