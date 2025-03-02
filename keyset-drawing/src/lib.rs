//! This crate contains the drawing functionality used to generate layout
//! diagrams by [keyset].
//!
//! [keyset]: https://crates.io/crates/keyset

mod error;
mod imp;
#[cfg(feature = "pdf")]
mod pdf;
#[cfg(feature = "png")]
mod png;
#[cfg(feature = "svg")]
mod svg;
#[cfg(not(any(feature = "pdf", feature = "png", feature = "svg")))]
compile_error!("no output format is enabled");

use std::fmt;

use font::Font;
use geom::{Dot, Length, Point, Rect, Size, Unit, DOT_PER_UNIT};
use key::Key;
use profile::Profile;

pub use error::Error;

pub(crate) use imp::{KeyDrawing, KeyPath};

/// A drawing
#[derive(Debug, Clone)]
pub struct Drawing {
    bounds: Rect<Unit>,
    keys: Box<[KeyDrawing]>,
    scale: f32,
}

impl Drawing {
    /// Encode the drawing as an SVG
    #[cfg(feature = "svg")]
    #[inline]
    #[must_use]
    pub fn to_svg(&self) -> String {
        svg::draw(self)
    }

    /// Encode the drawing as a PNG
    ///
    /// # Errors
    ///
    /// Returns [`Error::PngDimensionsError`] if the drawing is too large or too small to be
    /// encoded as a PNG.
    #[cfg(feature = "png")]
    #[inline]
    pub fn to_png(&self, ppi: f32) -> Result<Vec<u8>, Error> {
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

#[derive(Clone, Copy)]
struct NonExhaustive;

/// Template for generating a drawing
#[derive(Clone)]
pub struct Template {
    /// The keycap profile used for drawing keys
    pub profile: Profile,
    /// The font used for drawing legends
    pub font: Font,
    /// The scale used for the drawing
    pub scale: f32,
    /// The outline width for drawing key edges
    pub outline_width: Length<Dot>,
    /// Whether to show the keys in the drawing. Does not affect legends
    pub show_keys: bool,
    /// Show the margin used for legend alignment. Useful for debug purposes
    pub show_margin: bool,
    /// Hidden field to enforce non-exhaustive struct while still allowing instantiation using
    /// `..Default::default()` functional update syntax
    #[allow(private_interfaces)]
    #[doc(hidden)]
    pub __non_exhaustive: NonExhaustive,
}

impl Template {
    /// Draw the given keys using this template
    #[must_use]
    pub fn draw(&self, keys: &[Key]) -> Drawing {
        let bounds = keys
            .iter()
            .map(|k| k.shape.outer_rect().translate(k.position.to_vector()))
            .fold(
                Rect::from_origin_and_size(Point::origin(), Size::new(1.0, 1.0)),
                |rect, key| Rect::new(rect.min.min(key.min), rect.max.max(key.max)),
            );

        let keys = keys.iter().map(|key| KeyDrawing::new(key, self)).collect();

        Drawing {
            bounds,
            keys,
            scale: self.scale,
        }
    }
}

impl Default for Template {
    #[inline]
    fn default() -> Self {
        Self {
            profile: Profile::default(),
            font: Font::default(),
            scale: 1.0,
            outline_width: Length::new(0.01) * DOT_PER_UNIT,
            show_keys: true,
            show_margin: false,
            __non_exhaustive: NonExhaustive,
        }
    }
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("Template");
        dbg.field("profile", &self.profile)
            .field("font", &self.font)
            .field("scale", &self.scale)
            .field("outline_width", &self.outline_width)
            .field("show_keys", &self.show_keys)
            .field("show_margin", &self.show_margin);

        #[cfg(clippy)] // Suppress clippy::missing_fields_in_debug but only for this one field
        dbg.field("__non_exhaustive", &"NonExhaustive");

        dbg.finish()
    }
}

#[cfg(test)]
mod tests {
    use geom::{Mm, DOT_PER_MM};
    use isclose::assert_is_close;
    use profile::Profile;

    use super::*;

    #[test]
    fn template() {
        let template = Template::default();

        assert_is_close!(template.scale, 1.0);
        assert_eq!(template.font.num_glyphs(), 1); // .notdef

        let profile = Profile::default();
        let font = Font::from_ttf(std::fs::read(env!("DEMO_TTF")).unwrap()).unwrap();
        let template = Template {
            profile,
            font,
            scale: 2.0,
            outline_width: Length::new(20.0),
            show_keys: false,
            show_margin: true,
            ..Template::default()
        };

        assert_is_close!(
            template.profile.typ.depth(),
            Length::<Mm>::new(1.0) * DOT_PER_MM
        );
        assert_eq!(template.font.num_glyphs(), 3); // .notdef, A, V
        assert_is_close!(template.scale, 2.0);
    }

    #[test]
    fn template_debug() {
        let template = Template::default();

        assert_eq!(
            format!("{template:?}"),
            format!(
                "Template {{ profile: {:?}, font: {:?}, scale: {:?}, outline_width: {:?}, \
                    show_keys: {:?}, show_margin: {:?} }}",
                Profile::default(),
                Font::default(),
                1.0,
                10.0,
                true,
                false
            ),
        );
    }

    #[test]
    fn template_draw() {
        let template = Template::default();
        let keys = [Key::example()];

        let drawing = template.draw(&keys);

        assert_is_close!(drawing.bounds.width(), 1.0);
        assert_is_close!(drawing.bounds.height(), 1.0);
        assert_eq!(drawing.keys.len(), 1);
        assert_is_close!(drawing.scale, template.scale);
    }
}
