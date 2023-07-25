mod svg;

pub use self::svg::ToSvg;

use crate::key::Key;
use crate::profile::Profile;
use crate::Font;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DrawingOptions {
    pub dpi: f32,
    pub show_keys: bool,
    pub show_margin: bool,
}

impl Default for DrawingOptions {
    fn default() -> Self {
        Self {
            dpi: 96.,
            show_keys: true,
            show_margin: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Drawing {
    keys: Vec<Key>,
    profile: Profile,
    font: Font,
    options: DrawingOptions,
}

impl Drawing {
    #[inline]
    #[must_use]
    pub fn new(keys: Vec<Key>, profile: Profile, font: Font, options: DrawingOptions) -> Self {
        Self {
            keys,
            profile,
            font,
            options,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::profile::Profile;
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_drawing_new() {
        let keys = vec![];
        let profile = Profile::default();
        let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();
        let options = DrawingOptions::default();
        let drawing = Drawing::new(keys, profile, font, options);

        assert_approx_eq!(drawing.options.dpi, 96.);
        assert_eq!(drawing.keys.len(), 0);
    }
}
