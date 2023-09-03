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
        // write!(f, "{}", self.message)
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
