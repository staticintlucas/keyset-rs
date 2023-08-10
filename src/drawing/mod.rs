mod imp;
mod png;
mod svg;

use kurbo::{Point, Rect, Size};

use crate::{error::Result, Font, Key, Profile};

pub(crate) use imp::{KeyDrawing, Path};
pub(crate) use png::PngEncodingError;

#[derive(Debug, Clone)]
pub struct Drawing {
    bounds: Rect,
    keys: Vec<imp::KeyDrawing>,
    scale: f64,
}

impl Drawing {
    pub fn new(keys: &[Key], options: &DrawingOptions) -> Self {
        let bounds = keys
            .iter()
            .map(|k| Rect::from_origin_size(k.position, k.shape.size()))
            .fold(
                Rect::from_origin_size(Point::ORIGIN, Size::new(1., 1.)),
                |rect, key| rect.union(key),
            );

        let keys = keys
            .iter()
            .map(|key| imp::KeyDrawing::new(key, options))
            .collect();

        Self {
            bounds,
            keys,
            scale: options.dpi * 0.75, // 1u = 0.75in => dpu = 0.75*dpi
        }
    }

    pub fn to_svg(&self) -> String {
        svg::draw(self)
    }

    pub fn to_png(&self) -> Result<Vec<u8>> {
        png::draw(self)
    }
}

#[derive(Debug, Clone)]
pub struct DrawingOptions {
    profile: Profile,
    font: Font,
    dpi: f64,
    outline_width: f64,
    show_keys: bool,
    show_margin: bool,
}

impl Default for DrawingOptions {
    fn default() -> Self {
        Self {
            profile: Profile::default(),
            font: Font::default(),
            dpi: 96.,
            outline_width: 10.,
            show_keys: true,
            show_margin: false,
        }
    }
}

impl DrawingOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn profile(&mut self, profile: Profile) -> &mut Self {
        self.profile = profile;
        self
    }

    pub fn font(&mut self, font: Font) -> &mut Self {
        self.font = font;
        self
    }

    pub fn dpi(&mut self, dpi: f64) -> &mut Self {
        self.dpi = dpi;
        self
    }

    pub fn outline_width(&mut self, outline_width: f64) -> &mut Self {
        self.outline_width = outline_width;
        self
    }

    pub fn show_keys(&mut self, show_keys: bool) -> &mut Self {
        self.show_keys = show_keys;
        self
    }

    pub fn show_margin(&mut self, show_margin: bool) -> &mut Self {
        self.show_margin = show_margin;
        self
    }

    #[must_use]
    pub fn draw(&self, keys: &[Key]) -> Drawing {
        Drawing::new(keys, self)
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;

    use super::*;

    use crate::profile::{Profile, ProfileType};

    #[test]
    fn test_drawing_options() {
        let options = DrawingOptions::default();

        assert_approx_eq!(options.dpi, 96.);
        assert_eq!(options.font.glyphs.len(), 0);

        let profile = Profile::default();
        let font = Font::from_ttf(&std::fs::read("tests/fonts/demo.ttf").unwrap()).unwrap();
        let mut options = DrawingOptions::new();
        options
            .profile(profile)
            .font(font)
            .dpi(192.)
            .show_keys(false)
            .show_margin(true);

        assert_matches!(
            options.profile.profile_type,
            ProfileType::Cylindrical { .. }
        );
        assert_eq!(options.font.glyphs.len(), 2);
        assert_eq!(options.dpi, 192.);
    }
}
