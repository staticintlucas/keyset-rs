use std::fmt;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "toml")]
    TomlParseError(toml::de::Error),
    #[cfg(feature = "json")]
    JsonParseError(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            #[cfg(feature = "toml")]
            Self::TomlParseError(ref error) => write!(f, "{error}"),
            #[cfg(feature = "json")]
            Self::JsonParseError(ref error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            #[cfg(feature = "toml")]
            Self::TomlParseError(ref error) => Some(error),
            #[cfg(feature = "json")]
            Self::JsonParseError(ref error) => Some(error),
        }
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Self::TomlParseError(error)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use assert_matches::assert_matches;

    use crate::Profile;

    use super::*;

    #[test]
    fn error_fmt() {
        #[cfg(feature = "toml")]
        {
            #[allow(deprecated)]
            let error = Profile::from_toml("null").unwrap_err();

            assert_eq!(
                format!("{error}"),
                indoc::indoc!(
                    "
                    TOML parse error at line 1, column 5
                      |
                    1 | null
                      |     ^
                    expected `.`, `=`
                    "
                )
            );
        }
        #[cfg(feature = "json")]
        {
            let error = Profile::from_json("null").unwrap_err();

            assert_eq!(
                format!("{error}"),
                "invalid type: null, expected struct RawProfileData at line 1 column 4"
            );
        }
    }

    #[test]
    fn error_source() {
        #[cfg(feature = "toml")]
        {
            #[allow(deprecated)]
            let error = Profile::from_toml("null").unwrap_err();

            assert!(error.source().is_some());
            assert_eq!(
                format!("{}", error.source().unwrap()),
                indoc::indoc!(
                    "
                    TOML parse error at line 1, column 5
                      |
                    1 | null
                      |     ^
                    expected `.`, `=`
                    "
                )
            );
        }
        #[cfg(feature = "json")]
        {
            let error = Profile::from_json("null").unwrap_err();

            assert!(error.source().is_some());
            assert_eq!(
                format!("{}", error.source().unwrap()),
                "invalid type: null, expected struct RawProfileData at line 1 column 4"
            );
        }
    }

    #[test]
    fn error_from() {
        #[cfg(feature = "toml")]
        {
            let result: std::result::Result<i32, toml::de::Error> = toml::from_str("null");
            let error: Error = result.unwrap_err().into();

            assert_matches!(error, Error::TomlParseError(..));
        }
        #[cfg(feature = "json")]
        {
            let result: std::result::Result<i32, serde_json::Error> = serde_json::from_str("null");
            let error: Error = result.unwrap_err().into();

            assert_matches!(error, Error::JsonParseError(..));
        }
    }
}
