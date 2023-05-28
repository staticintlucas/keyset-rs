use std::fmt;

use crate::kle::InvalidKleLayout;
use crate::utils::InvalidColor;

#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorImpl>,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum ErrorImpl {
    JsonParseError(serde_json::Error),
    TomlParseError(toml::de::Error),
    FontParseError(ttf_parser::FaceParsingError),
    InvalidKleLayout(InvalidKleLayout),
    InvalidColor(InvalidColor),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.inner {
            ErrorImpl::JsonParseError(error) => write!(f, "error parsing JSON: {error}"),
            ErrorImpl::TomlParseError(error) => write!(f, "error parsing TOML: {error}"),
            ErrorImpl::FontParseError(error) => write!(f, "error parsing font: {error}"),
            ErrorImpl::InvalidKleLayout(error) => write!(f, "error parsing KLE layout: {error}"),
            ErrorImpl::InvalidColor(error) => write!(f, "error parsing color: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &*self.inner {
            ErrorImpl::JsonParseError(error) => Some(error),
            ErrorImpl::TomlParseError(error) => Some(error),
            ErrorImpl::FontParseError(error) => Some(error),
            ErrorImpl::InvalidKleLayout(_) | ErrorImpl::InvalidColor(_) => None,
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

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Self {
            inner: Box::new(ErrorImpl::TomlParseError(error)),
        }
    }
}

impl From<ttf_parser::FaceParsingError> for Error {
    fn from(error: ttf_parser::FaceParsingError) -> Self {
        Self {
            inner: Box::new(ErrorImpl::FontParseError(error)),
        }
    }
}

impl From<InvalidKleLayout> for Error {
    fn from(error: InvalidKleLayout) -> Self {
        Self {
            inner: Box::new(ErrorImpl::InvalidKleLayout(error)),
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
    use super::*;

    use crate::utils::Color;
    use crate::KleLayout;

    fn json_parse_error() -> Error {
        serde_json::from_str::<serde_json::Value>("invalid")
            .unwrap_err()
            .into()
    }

    fn toml_parse_error() -> Error {
        toml::from_str::<toml::Value>("invalid").unwrap_err().into()
    }

    fn font_parse_error() -> Error {
        ttf_parser::Face::parse(b"invalid", 0).unwrap_err().into()
    }

    fn invalid_key_size() -> Error {
        let kle = r#"[[{"w": 1, "h": 1, "x2": 1, "y2": 1, "w2": 1, "h2": 1}, "A"]]"#;
        let layout = KleLayout::new(kle).unwrap();
        layout.into_keys_iter().next().unwrap().unwrap_err().into()
    }

    fn invalid_color() -> Error {
        Color::from_hex("invalid").unwrap_err().into()
    }

    #[test]
    fn test_display_error() {
        let config = vec![
            (
                json_parse_error(),
                "error parsing JSON: expected value at line 1 column 1",
            ),
            (
                toml_parse_error(),
                r#"error parsing TOML: TOML parse error at line 1, column 8
  |
1 | invalid
  |        ^
expected `.`, `=`
"#,
            ),
            (font_parse_error(), "error parsing font: unknown magic"),
            (
                invalid_key_size(),
                "error parsing KLE layout: Unsupported non-standard key size \
                (w: 1.00, h: 1.00, x2: 1.00, y2: 1.00, w2: 1.00, h2: 1.00). \
                Note ISO enter and stepped caps are supported as special cases",
            ),
            (
                invalid_color(),
                "error parsing color: invalid hex code invalid",
            ),
        ];

        for (err, fmt) in config {
            assert_eq!(format!("{err}"), fmt);
        }
    }

    #[test]
    fn test_error_source() {
        use std::error::Error;

        let config = vec![
            (json_parse_error(), true),
            (toml_parse_error(), true),
            (font_parse_error(), true),
            (invalid_key_size(), false),
            (invalid_color(), false),
        ];

        for (err, has_source) in config {
            assert_eq!(err.source().is_some(), has_source);
        }
    }
}
