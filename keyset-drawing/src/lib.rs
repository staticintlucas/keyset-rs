//! This crate contains the drawing functionality used to generate layout
//! diagrams by [keyset].
//!
//! [keyset]: https://crates.io/crates/keyset

#![cfg_attr(coverage, expect(unstable_features))]
#![cfg_attr(coverage, feature(coverage_attribute))]

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
use geom::{ConvertInto as _, Dot, KeyUnit, Point, Rect, Translate, Vector};
use key::Key;
use profile::Profile;

pub use self::error::Error;
pub(crate) use self::imp::{KeyDrawing, KeyPath};

/// A drawing
#[derive(Debug, Clone)]
pub struct Drawing {
    bounds: Rect<KeyUnit>,
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
        png::draw(self, ppi)
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
    pub outline_width: Dot,
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
    ///
    /// # Errors
    ///
    /// Returns an error if the size requested keys cannot be drawn
    #[inline]
    pub fn draw(&self, keys: &[Key]) -> Result<Drawing, Error> {
        let bounds = keys
            .iter()
            .map(|k| k.shape.outer_rect() * Translate::new(k.position.x, k.position.y))
            .fold(
                Rect::from_origin_and_size(
                    Point::origin(),
                    Vector::new(KeyUnit(1.0), KeyUnit(1.0)),
                ),
                |rect, key| Rect::new(rect.min.min(key.min), rect.max.max(key.max)),
            );

        let keys = keys
            .iter()
            .map(|key| KeyDrawing::new(key, self))
            .collect::<Result<_, _>>()?;

        Ok(Drawing {
            bounds,
            keys,
            scale: self.scale,
        })
    }
}

impl Default for Template {
    #[inline]
    fn default() -> Self {
        Self {
            profile: Profile::default(),
            font: Font::default(),
            scale: 1.0,
            outline_width: KeyUnit(0.01).convert_into(),
            show_keys: true,
            show_margin: false,
            __non_exhaustive: NonExhaustive,
        }
    }
}

impl fmt::Debug for Template {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("Template");
        let _ = dbg
            .field("profile", &self.profile)
            .field("font", &self.font)
            .field("scale", &self.scale)
            .field("outline_width", &self.outline_width)
            .field("show_keys", &self.show_keys)
            .field("show_margin", &self.show_margin);

        #[cfg(clippy)] // Suppress clippy::missing_fields_in_debug but only for this one field
        let _ = dbg.field("__non_exhaustive", &"NonExhaustive");

        dbg.finish()
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use geom::{ConvertFrom as _, Mm};

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
            outline_width: Mm(20.0).convert_into(),
            show_keys: false,
            show_margin: true,
            ..Template::default()
        };

        assert_is_close!(template.profile.typ.depth(), Dot::convert_from(Mm(1.0)));
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
                Dot(10.0),
                true,
                false
            ),
        );
    }

    #[test]
    fn template_draw() {
        let template = Template::default();
        let keys = [Key::example()];

        let drawing = template.draw(&keys).unwrap();

        assert_is_close!(drawing.bounds.width(), KeyUnit(1.0));
        assert_is_close!(drawing.bounds.height(), KeyUnit(1.0));
        assert_eq!(drawing.keys.len(), 1);
        assert_is_close!(drawing.scale, template.scale);
    }
}
