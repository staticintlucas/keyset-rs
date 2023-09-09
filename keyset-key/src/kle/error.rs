#[derive(Debug)]
pub enum Error {
    UnsupportedKeySize {
        w: f64,
        h: f64,
        x2: f64,
        y2: f64,
        w2: f64,
        h2: f64,
    },
    JsonParseError(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::JsonParseError(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::UnsupportedKeySize { .. } => None,
            Self::JsonParseError(error) => Some(error),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
pub mod tests {
    use std::error::Error as _;

    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn error_fmt() {
        let unsupported_key_size = Error::UnsupportedKeySize {
            w: 1.,
            h: 1.,
            x2: -0.25,
            y2: 0.,
            w2: 1.5,
            h2: 1.,
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
            w: 1.,
            h: 1.,
            x2: -0.25,
            y2: 0.,
            w2: 1.5,
            h2: 1.,
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
