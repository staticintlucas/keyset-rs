//! This crate contains the font loading and parsing logic used internally by [keyset].
//!
//! [keyset]: https://crates.io/crates/keyset

mod error;
mod face;

use std::sync::OnceLock;

use itertools::izip;
use rustybuzz::ttf_parser::name_id;
use rustybuzz::{BufferClusterLevel, ShapePlan, UnicodeBuffer};
use saturate::SaturatingInto;

use geom::{Angle, Length, Path, PathBuilder, Vector};

pub use self::error::{Error, Result};
use self::face::Face;

/// Unit within a font
#[derive(Debug, Clone, Copy)]
pub struct FontUnit;

/// A parsed font
#[derive(Debug, Clone)]
pub struct Font {
    face: Face,
    family: String,
    name: String,
    cap_height: Length<FontUnit>,
    x_height: Length<FontUnit>, // TODO is this used?
}

impl Default for Font {
    #[inline]
    fn default() -> Self {
        static FONT: OnceLock<Font> = OnceLock::new();

        FONT.get_or_init(|| {
            Self::from_ttf(include_bytes!(env!("DEFAULT_TTF")).to_vec())
                .unwrap_or_else(|_| unreachable!("default font is tested"))
        })
        .clone()
    }
}

impl Font {
    /// Parse a font from TrueType or OpenType format font data
    ///
    /// # Errors
    ///
    /// If there is an error parsing the font data, or some of the required font properties
    /// cannot be determined
    #[inline]
    pub fn from_ttf(data: Vec<u8>) -> Result<Self> {
        let face = Face::from_ttf(data)?;

        let family = face
            .name(name_id::FAMILY)
            .ok_or_else(|| Error::MissingProperty("font family".to_owned()))?;
        let name = face
            .name(name_id::FULL_NAME)
            .ok_or_else(|| Error::MissingProperty("full font name".to_owned()))?;

        let cap_height = Length::new(
            face.capital_height()
                .or_else(|| Some(face.glyph_bounds(face.glyph_index('H')?)?.height()))
                .ok_or_else(|| Error::MissingProperty("capital height".to_owned()))?
                .into(),
        );
        let x_height = Length::new(
            face.x_height()
                .or_else(|| Some(face.glyph_bounds(face.glyph_index('x')?)?.height()))
                .ok_or_else(|| Error::MissingProperty("x height".to_owned()))?
                .into(),
        );

        Ok(Self {
            face,
            family,
            name,
            cap_height,
            x_height,
        })
    }

    /// The font family name
    #[inline]
    #[must_use]
    pub fn family(&self) -> &str {
        &self.family
    }

    /// The font's full name
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The number font units per EM
    #[inline]
    #[must_use]
    pub fn em_size(&self) -> Length<FontUnit> {
        Length::new(self.face.units_per_em().into())
    }

    /// The capital height in font units
    ///
    /// Measures the height of the uppercase `'H'` if not set by the font
    #[inline]
    #[must_use]
    pub const fn cap_height(&self) -> Length<FontUnit> {
        self.cap_height
    }

    /// The x-height in font units
    ///
    /// Measures the height of the lowercase `'x'` if not set by the font
    #[inline]
    #[must_use]
    pub const fn x_height(&self) -> Length<FontUnit> {
        self.x_height
    }

    /// The font's ascender in font units
    #[inline]
    #[must_use]
    pub fn ascender(&self) -> Length<FontUnit> {
        Length::new(self.face.ascender().into())
    }

    /// The font's descender in font units
    ///
    /// Positive values are in a downwards direction
    #[inline]
    #[must_use]
    pub fn descender(&self) -> Length<FontUnit> {
        Length::new((-self.face.descender()).into())
    }

    /// The font's line gap in font units
    #[inline]
    #[must_use]
    pub fn line_gap(&self) -> Length<FontUnit> {
        Length::new(self.face.line_gap().into())
    }

    /// The font's line height in font units
    ///
    /// This is equal to `self.ascender() + self.descender() + self.line_gap()`
    #[inline]
    #[must_use]
    pub fn line_height(&self) -> Length<FontUnit> {
        self.ascender() + self.descender() + self.line_gap()
    }

    /// The font's slope angle
    ///
    /// Clockwise (forward) angles are positive
    ///
    /// <div class="warning">
    /// This function will return `0` if the slope angle is not specified in the font
    /// </div>
    #[inline]
    #[must_use]
    pub fn slope(&self) -> Angle {
        Angle::degrees(-self.face.italic_angle())
    }

    /// The number of glyph outlines in the font
    #[inline]
    #[must_use]
    pub fn num_glyphs(&self) -> usize {
        self.face.number_of_glyphs().into()
    }

    /// Checks if the font has a glyph for the given char
    #[inline]
    #[must_use]
    pub fn has_glyph(&self, code_point: char) -> bool {
        self.face.glyph_index(code_point).is_some()
    }

    /// Renders a string of text to a path
    #[must_use]
    pub fn render_string(&self, text: &str) -> Path<FontUnit> {
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(text);
        buffer.guess_segment_properties(); // TODO set properties explicitly?
        buffer.set_cluster_level(BufferClusterLevel::MonotoneCharacters);

        // TODO: cache plan?
        let plan = ShapePlan::new(
            self.face.inner(),
            buffer.direction(),
            Some(buffer.script()),
            buffer.language().as_ref(),
            &[],
        );

        let glyph_buffer = rustybuzz::shape_with_plan(self.face.inner(), &plan, buffer);

        let infos = glyph_buffer.glyph_infos();
        let positions = glyph_buffer.glyph_positions();

        let capacity = infos
            .iter()
            .map(|info| info.glyph_id.saturating_into()) // guaranteed in u16 range by rustybuzz
            .map(|glyph_id| self.face.outline_length(glyph_id))
            .sum();

        let mut builder = PathBuilder::with_capacity(capacity);
        let mut position = Vector::zero();
        for (info, pos) in izip!(infos, positions) {
            let advance = Vector::new(
                pos.x_advance.saturating_into(),
                pos.y_advance.saturating_into(),
            );
            let offset = Vector::new(
                pos.x_offset.saturating_into(),
                pos.y_offset.saturating_into(),
            );

            self.face.outline_glyph(
                info.glyph_id.saturating_into(), // guaranteed to be in u16 range by rustybuzz
                &mut builder,
                position + offset,
            );

            position += advance;
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn font_default() {
        let default = Font::default();

        assert_matches!(default.face.number_of_glyphs(), 1); // Only .notdef
        assert_eq!(default.family, "default");
        assert_eq!(default.name, "default regular");
        assert_is_close!(default.cap_height, Length::new(714.0));
        assert_is_close!(default.x_height, Length::new(523.0));
    }

    #[test]
    fn font_from_ttf() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_eq!(font.face.number_of_glyphs(), 3);
        assert_eq!(font.family, "demo");
        assert_eq!(font.name, "demo regular");
        assert_eq!(font.cap_height, Length::new(650.0));
        assert_eq!(font.x_height, Length::new(450.0));
    }

    #[test]
    fn font_properties() {
        type Length = geom::Length<FontUnit>;

        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_eq!(font.family(), "demo");
        assert_eq!(font.name(), "demo regular");
        assert_is_close!(font.em_size(), Length::new(1000.0));
        assert_is_close!(font.cap_height(), Length::new(650.0));
        assert_is_close!(font.x_height(), Length::new(450.0));
        assert_is_close!(font.ascender(), Length::new(1024.0));
        assert_is_close!(font.descender(), Length::new(400.0));
        assert_is_close!(font.line_gap(), Length::new(0.0));
        assert_is_close!(font.line_height(), Length::new(1424.0));
        assert_is_close!(font.slope(), Angle::degrees(0.0));
        assert_eq!(font.num_glyphs(), 3);

        let data = std::fs::read(env!("NULL_TTF")).unwrap();
        let _err = Font::from_ttf(data).unwrap_err();
    }

    #[test]
    fn font_has_glyph() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert!(font.has_glyph('A'));
        assert!(font.has_glyph('V'));
        assert!(!font.has_glyph('P'));
    }

    #[test]
    fn font_render_string() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        let path = font.render_string("AV");
        assert_eq!(path.len(), 24);

        let path = font.render_string("P");
        assert_eq!(path.len(), 12); // == .notdef length
    }
}
