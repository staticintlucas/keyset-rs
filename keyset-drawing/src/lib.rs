mod imp;
#[cfg(feature = "pdf")]
mod pdf;
#[cfg(feature = "png")]
mod png;
#[cfg(feature = "svg")]
mod svg;

use font::Font;
use geom::{Point, Rect, Size};
use key::Key;
use profile::Profile;

pub(crate) use imp::{KeyDrawing, Path};

#[derive(Debug, Clone)]
pub struct Drawing {
    bounds: Rect,
    keys: Vec<imp::KeyDrawing>,
    scale: f64,
}

impl Drawing {
    #[must_use]
    pub fn new(keys: &[Key], options: &Options) -> Self {
        let bounds = keys
            .iter()
            .map(|k| k.shape.bounds().with_origin(k.position))
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
            scale: options.scale,
        }
    }

    #[cfg(feature = "pdf")]
    #[must_use]
    pub fn to_svg(&self) -> String {
        svg::draw(self)
    }

    #[cfg(feature = "pdf")]
    #[must_use]
    pub fn to_png(&self, dpi: f64) -> Vec<u8> {
        png::draw(self, dpi)
    }

    #[cfg(feature = "pdf")]
    #[must_use]
    pub fn to_pdf(&self) -> Vec<u8> {
        pdf::draw(self)
    }

    #[cfg(feature = "pdf")]
    #[must_use]
    pub fn to_ai(&self) -> Vec<u8> {
        // An Illustrator file typically contains both an Illustrator-native and a PDF copy of an
        // image. Most other software (including Adobe's own InDesign) use the PDF data and not the
        // native Illustrator format. Illustrator also happily opens a PDF with .ai file extension,
        // so we just generate a PDF when exporting an Illustrator file. I have not come across any
        // compatibility issues using this approach, but I do recommend checking your files in
        // Illustrator just in case.
        pdf::draw(self)
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    profile: Profile,
    font: Font,
    scale: f64,
    outline_width: f64,
    show_keys: bool,
    show_margin: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            profile: Profile::default(),
            font: Font::default(),
            scale: 1.,
            outline_width: 10.,
            show_keys: true,
            show_margin: false,
        }
    }
}

impl Options {
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

    pub fn scale(&mut self, scale: f64) -> &mut Self {
        self.scale = scale;
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

    use profile::Profile;

    use super::*;

    #[test]
    fn test_drawing_options() {
        let options = Options::default();

        assert_approx_eq!(options.scale, 1.);
        assert_eq!(options.font.glyphs().len(), 0);

        let profile = Profile::default();
        let font = Font::from_ttf(std::fs::read(env!("DEMO_TTF")).unwrap()).unwrap();
        let mut options = Options::new();
        options
            .profile(profile)
            .font(font)
            .scale(2.)
            .outline_width(20.)
            .show_keys(false)
            .show_margin(true);

        assert_eq!(options.profile.typ.depth(), 1.0);
        assert_eq!(options.font.glyphs().len(), 2);
        assert_eq!(options.scale, 2.);
    }

    #[test]
    fn test_drawing_options_draw() {
        let options = Options::new();
        let keys = [Key::example()];

        let drawing = options.draw(&keys);

        assert_eq!(drawing.bounds.width(), 1.);
        assert_eq!(drawing.bounds.height(), 1.);
        assert_eq!(drawing.keys.len(), 1);
        assert_eq!(drawing.scale, options.scale);
    }
}
