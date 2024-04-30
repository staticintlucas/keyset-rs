#![warn(
    missing_docs,
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
#![allow(
    clippy::suboptimal_flops, // Optimiser is pretty good, and mul_add is pretty ugly
    missing_docs, // TODO
)]

mod imp;
#[cfg(feature = "pdf")]
mod pdf;
#[cfg(feature = "png")]
mod png;
#[cfg(feature = "svg")]
mod svg;

use font::Font;
use geom::{Dot, Inch, Length, Point, Rect, Scale, Size, Unit};
use key::Key;
use profile::Profile;

#[allow(unused_imports)] // Path is unused if no format is enabled, but who would do that?
pub(crate) use imp::{KeyDrawing, KeyPath};

#[derive(Debug, Clone)]
#[allow(dead_code)] // Struct fields are unused if no format is enabled, but who would do that?
pub struct Drawing {
    bounds: Rect<Unit>,
    keys: Vec<KeyDrawing>,
    scale: f32,
}

impl Drawing {
    #[must_use]
    pub fn new(keys: &[Key], options: &Options) -> Self {
        let bounds = keys
            .iter()
            .map(|k| k.shape.outer_rect().translate(k.position.to_vector()))
            .fold(
                Rect::from_origin_and_size(Point::origin(), Size::new(1.0, 1.0)),
                |rect, key| rect.union(&key),
            );

        let keys = keys
            .iter()
            .map(|key| KeyDrawing::new(key, options))
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
    pub fn to_png(&self, ppi: f32) -> Vec<u8> {
        png::draw(self, Scale::<Inch, png::Pixel>::new(ppi))
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
pub struct Options<'a> {
    profile: &'a Profile,
    font: &'a Font,
    scale: f32,
    outline_width: Length<Dot>,
    show_keys: bool,
    show_margin: bool,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            profile: &Profile::DEFAULT,
            font: Font::default_ref(),
            scale: 1.0,
            outline_width: Length::new(10.0),
            show_keys: true,
            show_margin: false,
        }
    }
}

impl<'a> Options<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn profile(self, profile: &'a Profile) -> Self {
        Self { profile, ..self }
    }

    #[must_use]
    pub const fn font(self, font: &'a Font) -> Self {
        Self { font, ..self }
    }

    #[must_use]
    pub const fn scale(self, scale: f32) -> Self {
        Self { scale, ..self }
    }

    #[must_use]
    pub const fn outline_width(self, outline_width: Length<Dot>) -> Self {
        Self {
            outline_width,
            ..self
        }
    }

    #[must_use]
    pub const fn show_keys(self, show_keys: bool) -> Self {
        Self { show_keys, ..self }
    }

    #[must_use]
    pub const fn show_margin(self, show_margin: bool) -> Self {
        Self {
            show_margin,
            ..self
        }
    }

    #[must_use]
    pub fn draw(&self, keys: &[Key]) -> Drawing {
        Drawing::new(keys, self)
    }
}

#[cfg(test)]
mod tests {
    use isclose::assert_is_close;
    use profile::Profile;

    use super::*;

    #[test]
    fn test_drawing_options() {
        let options = Options::default();

        assert_is_close!(options.scale, 1.0);
        assert_eq!(options.font.num_glyphs(), 1); // .notdef

        let profile = Profile::default();
        let font = Font::from_ttf(std::fs::read(env!("DEMO_TTF")).unwrap()).unwrap();
        let options = Options::new()
            .profile(&profile)
            .font(&font)
            .scale(2.0)
            .outline_width(Length::new(20.0))
            .show_keys(false)
            .show_margin(true);

        assert_is_close!(options.profile.typ.depth(), 1.0);
        assert_eq!(options.font.num_glyphs(), 3); // .notdef, A, V
        assert_is_close!(options.scale, 2.0);
    }

    #[test]
    fn test_drawing_options_draw() {
        let options = Options::new();
        let keys = [Key::example()];

        let drawing = options.draw(&keys);

        assert_is_close!(drawing.bounds.width(), 1.);
        assert_is_close!(drawing.bounds.height(), 1.);
        assert_eq!(drawing.keys.len(), 1);
        assert_is_close!(drawing.scale, options.scale);
    }
}
