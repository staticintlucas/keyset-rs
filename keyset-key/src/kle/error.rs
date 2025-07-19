/// An error parsing a KLE layout
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An error in parsing the KLE JSON file
    JsonParseError(serde_json::Error),
    /// A key size not supported by `keyset`
    UnsupportedKeySize {
        /// The key's `w` value
        w: f32,
        /// The key's `h` value
        h: f32,
        /// The key's `x2` value
        x2: f32,
        /// The key's `y2` value
        y2: f32,
        /// The key's `w2` value
        w2: f32,
        /// The key's `h2` value
        h2: f32,
    },
}

impl std::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::UnsupportedKeySize {
                w,
                h,
                x2,
                y2,
                w2,
                h2,
            } => write!(
                f,
                "unsupported non-standard key size \
                (w: {w:.2}, h: {h:.2}, x2: {x2:.2}, y2: {y2:.2}, w2: {w2:.2}, h2: {h2:.2}). \
                Note only ISO enter and stepped caps are supported as special cases"
            ),
            Self::JsonParseError(ref error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::UnsupportedKeySize { .. } => None,
            Self::JsonParseError(ref error) => Some(error),
        }
    }
}

impl From<serde_json::Error> for Error {
    #[inline]
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}

/// A [`std::result::Result`] with [`Error`] as it's error type
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use std::error::Error as _;

    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn error_fmt() {
        let unsupported_key_size = Error::UnsupportedKeySize {
            w: 1.0,
            h: 1.0,
            x2: -0.25,
            y2: 0.0,
            w2: 1.5,
            h2: 1.0,
        };
        assert_eq!(
            format!("{unsupported_key_size}"),
            "unsupported non-standard key size (w: 1.00, h: 1.00, x2: -0.25, y2: 0.00, w2: 1.50, \
            h2: 1.00). Note only ISO enter and stepped caps are supported as special cases"
        );

        let json_parse_error: Error = serde_json::from_str::<i32>("error").unwrap_err().into();
        assert_eq!(
            format!("{json_parse_error}"),
            "expected value at line 1 column 1"
        );
    }

    #[test]
    fn error_source() {
        let unsupported_key_size = Error::UnsupportedKeySize {
            w: 1.0,
            h: 1.0,
            x2: -0.25,
            y2: 0.0,
            w2: 1.5,
            h2: 1.0,
        };
        assert!(unsupported_key_size.source().is_none());

        let json_parse_error: Error = serde_json::from_str::<i32>("error").unwrap_err().into();
        assert!(json_parse_error.source().is_some());
    }

    #[test]
    fn error_from() {
        let json_parse_error = serde_json::from_str::<i32>("error").unwrap_err();

        assert_matches!(json_parse_error.into(), Error::JsonParseError(..));
    }
}
