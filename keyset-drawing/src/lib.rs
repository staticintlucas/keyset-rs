//! This crate contains the drawing functionality used to generate layout
//! diagrams by [keyset].
//!
//! [keyset]: https://crates.io/crates/keyset

mod imp;
#[cfg(feature = "pdf")]
mod pdf;
#[cfg(feature = "png")]
mod png;
#[cfg(feature = "svg")]
mod svg;
#[cfg(not(any(feature = "pdf", feature = "png", feature = "svg")))]
compile_error!("no output format is enabled");

use font::Font;
use geom::{Dot, Length, Point, Rect, Size, Unit, DOT_PER_UNIT};
use key::Key;
use profile::Profile;

pub(crate) use imp::{KeyDrawing, KeyPath};

/// A drawing
#[derive(Debug, Clone)]
pub struct Drawing {
    bounds: Rect<Unit>,
    keys: Box<[KeyDrawing]>,
    scale: f32,
}

impl Drawing {
    /// Create a new drawing using the given options
    #[must_use]
    pub fn new(keys: &[Key], options: &Options<'_>) -> Self {
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

    /// Encode the drawing as an SVG
    #[cfg(feature = "svg")]
    #[inline]
    #[must_use]
    pub fn to_svg(&self) -> String {
        svg::draw(self)
    }

    /// Encode the drawing as a PNG
    #[cfg(feature = "png")]
    #[inline]
    #[must_use]
    pub fn to_png(&self, ppi: f32) -> Vec<u8> {
        png::draw(self, geom::Scale::new(ppi))
    }

    /// Encode the drawing as a PDF
    #[cfg(feature = "pdf")]
    #[inline]
    #[must_use]
    pub fn to_pdf(&self) -> Vec<u8> {
        pdf::draw(self)
    }

    /// Encode the drawing as an Illustrator file
    ///
    /// <div class="warning">
    ///
    /// An Illustrator file typically contains both an Illustrator-native and a
    /// PDF copy of a graphic. Illustrator will happily read a file containing
    /// only PDF data and most other software (including Adobe's own InDesign)
    /// only use the PDF copy.
    ///
    /// As Illustrator files are a proprietary format, we just generate a PDF
    /// when exporting an Illustrator file. There have not been any reports of
    /// compatibility issues using this approach, but it is recommend to check
    /// your files in Illustrator just in case.
    ///
    /// </div>
    #[cfg(feature = "pdf")]
    #[inline]
    #[must_use]
    pub fn to_ai(&self) -> Vec<u8> {
        pdf::draw(self)
    }
}

/// Options for generating a drawing
#[derive(Debug, Clone)]
pub struct Options<'a> {
    profile: &'a Profile,
    font: &'a Font,
    scale: f32,
    outline_width: Length<Dot>,
    show_keys: bool,
    show_margin: bool,
}

impl Default for Options<'_> {
    #[inline]
    fn default() -> Self {
        Self {
            profile: Profile::default_ref(),
            font: Font::default_ref(),
            scale: 1.0,
            outline_width: Length::new(0.01) * DOT_PER_UNIT,
            show_keys: true,
            show_margin: false,
        }
    }
}

impl<'a> Options<'a> {
    /// Create a new struct containing the default option
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the keycap profile used for drawing keys
    #[inline]
    #[must_use]
    pub const fn profile(self, profile: &'a Profile) -> Self {
        Self { profile, ..self }
    }

    /// Set the font used for drawing legends
    #[inline]
    #[must_use]
    pub const fn font(self, font: &'a Font) -> Self {
        Self { font, ..self }
    }

    /// Set the scale used for the drawing
    #[inline]
    #[must_use]
    pub const fn scale(self, scale: f32) -> Self {
        Self { scale, ..self }
    }

    /// Set the outline width for drawing key edges
    #[inline]
    #[must_use]
    pub const fn outline_width(self, outline_width: Length<Dot>) -> Self {
        Self {
            outline_width,
            ..self
        }
    }

    /// Whether to show the keys in the drawing. Does not affect legends
    #[inline]
    #[must_use]
    pub const fn show_keys(self, show_keys: bool) -> Self {
        Self { show_keys, ..self }
    }

    /// Show the margin used for legend alignment. Useful for debug purposes
    #[inline]
    #[must_use]
    pub const fn show_margin(self, show_margin: bool) -> Self {
        Self {
            show_margin,
            ..self
        }
    }

    /// Draw keys with the given options
    #[inline]
    #[must_use]
    pub fn draw(&self, keys: &[Key]) -> Drawing {
        Drawing::new(keys, self)
    }
}

#[cfg(test)]
mod tests {
    use geom::{Mm, DOT_PER_MM};
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

        assert_is_close!(
            options.profile.typ.depth(),
            Length::<Mm>::new(1.0) * DOT_PER_MM
        );
        assert_eq!(options.font.num_glyphs(), 3); // .notdef, A, V
        assert_is_close!(options.scale, 2.0);
    }

    #[test]
    fn test_drawing_options_draw() {
        let options = Options::new();
        let keys = [Key::example()];

        let drawing = options.draw(&keys);

        assert_is_close!(drawing.bounds.width(), 1.0);
        assert_is_close!(drawing.bounds.height(), 1.0);
        assert_eq!(drawing.keys.len(), 1);
        assert_is_close!(drawing.scale, options.scale);
    }
}
