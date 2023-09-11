use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    #[cfg(feature = "toml")]
    TomlParseError(toml::de::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "toml")]
            Self::TomlParseError(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "toml")]
            Self::TomlParseError(error) => Some(error),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        #[cfg(feature = "toml")]
        Self::TomlParseError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
