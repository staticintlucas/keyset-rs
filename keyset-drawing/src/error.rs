use std::fmt;

use geom::Vector;

#[cfg(feature = "png")]
use crate::png::Pixel;

/// A drawing creation error
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Error {
    /// There are too many lines of text in a legend
    TooManyLines(usize),
    /// The drawing is larger than the maximum PNG dimensions
    #[cfg(feature = "png")]
    PngDimensionsError(Vector<Pixel>),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TooManyLines(lines) => write!(f, "too many lines ({lines}) in legend"),
            #[cfg(feature = "png")]
            Self::PngDimensionsError(dims) => write!(f, "invalid PNG dimensions {dims:?}"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use geom::{KeyUnit, Point};

    use crate::Template;

    #[cfg(feature = "png")]
    #[test]
    fn error_fmt() {
        let key1 = key::Key::example();
        let key2 = {
            let mut tmp = key1.clone();
            tmp.position = Point::new(KeyUnit(1e20), KeyUnit(1e20));
            tmp
        };

        let error = Template::default()
            .draw(&[key1, key2])
            .unwrap()
            .to_png(1.0)
            .unwrap_err();

        assert!(format!("{error}").starts_with("invalid PNG dimensions "));
    }
}
