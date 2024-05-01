//! This crate contains the font loading and parsing logic used internally by [keyset].
//!
//! [keyset]: https://crates.io/crates/keyset

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

mod default;
mod error;
mod face;

use std::fmt::Debug;
use std::sync::OnceLock;

use dashmap::DashMap;
use geom::{Angle, Length, Path, PathBuilder, Point};
use log::warn;
use ttf_parser::GlyphId;

pub use self::error::{Error, Result};
use face::Face;

/// Unit within a font
#[derive(Debug, Clone, Copy)]
pub struct FontUnit;

/// A glyph loaded from a [`Font`]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Glyph {
    /// The outline of the glyph
    pub path: Path<FontUnit>,
    /// The glyphs horizontal advance
    pub advance: Length<FontUnit>,
}

impl Glyph {
    fn parse_from(face: &Face, gid: GlyphId) -> Option<Self> {
        struct PathBuilderWrapper(PathBuilder<FontUnit>);

        // GRCOV_EXCL_START // TODO these are pretty trivial but we could cover them in tests
        impl ttf_parser::OutlineBuilder for PathBuilderWrapper {
            fn move_to(&mut self, x: f32, y: f32) {
                self.0.abs_move(Point::new(x, y));
            }

            fn line_to(&mut self, x: f32, y: f32) {
                self.0.abs_line(Point::new(x, y));
            }

            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                self.0
                    .abs_quadratic_bezier(Point::new(x1, y1), Point::new(x, y));
            }

            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                self.0
                    .abs_cubic_bezier(Point::new(x1, y1), Point::new(x2, y2), Point::new(x, y));
            }

            fn close(&mut self) {
                self.0.close();
            }
        }
        // GRCOV_EXCL_STOP

        let mut builder = PathBuilderWrapper(Path::builder());

        let _ = face.outline_glyph(gid, &mut builder);
        let path = builder.0.build();

        let advance = Length::new(face.glyph_hor_advance(gid)?.into());

        Some(Self { path, advance })
    }
}

/// A parsed font
#[derive(Clone)]
pub struct Font {
    face: Face,
    family: OnceLock<String>,
    name: OnceLock<String>,
    cap_height: OnceLock<Length<FontUnit>>,
    x_height: OnceLock<Length<FontUnit>>,
    notdef: OnceLock<Glyph>,
    glyphs: DashMap<char, Option<Glyph>>,
    kerning: DashMap<(char, char), Length<FontUnit>>,
}

impl Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Font").field(self.name()).finish()
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::default_ref().clone()
    }
}

impl Font {
    /// Returns a static reference to the default font
    ///
    /// This is equivalent to calling [`Default::default`] but returns a reference and avoids
    /// cloning any internal data
    #[must_use]
    pub fn default_ref() -> &'static Self {
        default::font()
    }

    /// Parse a font from TrueType or OpenType format font data
    ///
    /// # Errors
    ///
    /// If there is an error parsing the font data
    pub fn from_ttf(data: Vec<u8>) -> Result<Self> {
        Ok(Self {
            face: Face::from_ttf(data)?,
            family: OnceLock::new(),
            name: OnceLock::new(),
            cap_height: OnceLock::new(),
            x_height: OnceLock::new(),
            notdef: OnceLock::new(),
            glyphs: DashMap::new(),
            kerning: DashMap::new(),
        })
    }

    /// The font family name
    ///
    /// Returns `"unknown"` if the font does not specify a family name
    pub fn family(&self) -> &String {
        self.family.get_or_init(|| {
            self.face
                .names()
                .into_iter()
                .filter(|n| n.name_id == ttf_parser::name_id::FAMILY)
                .find_map(|n| n.to_string())
                .unwrap_or_else(|| {
                    warn!("cannot read font family name");
                    "unknown".to_owned()
                })
        })
    }

    /// The font's full name
    ///
    /// Returns `"unknown"` if the font does not specify a full name
    pub fn name(&self) -> &String {
        self.name.get_or_init(|| {
            self.face
                .names()
                .into_iter()
                .filter(|n| n.name_id == ttf_parser::name_id::FULL_NAME)
                .find_map(|n| n.to_string())
                .unwrap_or_else(|| {
                    warn!("cannot read font full name");
                    "unknown".to_owned()
                })
        })
    }

    /// The number font units per EM
    pub fn em_size(&self) -> Length<FontUnit> {
        Length::new(self.face.units_per_em().into())
    }

    /// The capital height in font units
    ///
    /// Measures the height of the uppercase `'M'` if it is not set. In case the font does not contain
    /// an uppercase `'M'`, a default value is returned
    pub fn cap_height(&self) -> Length<FontUnit> {
        *self.cap_height.get_or_init(|| {
            self.face
                .capital_height()
                .map(Into::into)
                .or_else(|| self.glyph('M').map(|g| g.path.bounds.height()))
                .map_or_else(
                    || default::cap_height() * (self.line_height() / default::line_height()),
                    Length::new,
                )
        })
    }

    /// The x-height in font units
    ///
    /// Measures the height of the lowercase `'x'` if it is not set. In case the font does not contain
    /// a lowercase `'x'`, a default value is returned
    pub fn x_height(&self) -> Length<FontUnit> {
        *self.x_height.get_or_init(|| {
            self.face
                .x_height()
                .map(Into::into)
                .or_else(|| self.glyph('x').map(|g| g.path.bounds.height()))
                .map_or_else(
                    || default::x_height() * (self.line_height() / default::line_height()),
                    Length::new,
                )
        })
    }

    /// The font's ascender in font units
    pub fn ascender(&self) -> Length<FontUnit> {
        Length::new(self.face.ascender().into())
    }

    /// The font's descender in font units
    pub fn descender(&self) -> Length<FontUnit> {
        Length::new((-self.face.descender()).into())
    }

    /// The font's line gap in font units
    pub fn line_gap(&self) -> Length<FontUnit> {
        Length::new(self.face.line_gap().into())
    }

    /// The font's line height in font units
    ///
    /// This is equal to `self.ascender() + self.descender() + self.line_gap()`
    pub fn line_height(&self) -> Length<FontUnit> {
        self.ascender() + self.descender() + self.line_gap()
    }

    /// The font's slope angle, anticlockwise being positive
    pub fn slope(&self) -> Option<Angle> {
        self.face.italic_angle().map(Angle::degrees) // Negate so forward = positive
    }

    /// The number of glyph outlines in the font
    pub fn num_glyphs(&self) -> usize {
        self.face.number_of_glyphs().into()
    }

    /// Returns the glyph for a given character if present in the font
    pub fn glyph(&self, char: char) -> Option<Glyph> {
        self.glyphs
            .entry(char)
            .or_insert_with(|| {
                self.face
                    .glyph_index(char)
                    .and_then(|gid| Glyph::parse_from(&self.face, gid))
            })
            .clone()
    }

    /// Returns the glyph for a given character, or the default replacement character if not present
    pub fn glyph_or_default(&self, char: char) -> Glyph {
        self.glyph(char).unwrap_or_else(|| self.notdef())
    }

    /// Returns the font's default replacement glyph, `.notdef`, or a builtin default if not present
    pub fn notdef(&self) -> Glyph {
        self.notdef
            .get_or_init(|| {
                Glyph::parse_from(&self.face, GlyphId(0)).unwrap_or_else(|| {
                    warn!("no valid outline for glyph .notdef in font");
                    default::notdef()
                })
            })
            .clone()
    }

    /// Returns the kerning between two characters' glyphs, or 0 if no kerning is specified in the
    /// font
    pub fn kerning(&self, left: char, right: char) -> Length<FontUnit> {
        *self.kerning.entry((left, right)).or_insert_with(|| {
            if let (Some(lhs), Some(rhs)) =
                (self.face.glyph_index(left), self.face.glyph_index(right))
            {
                Length::new(self.face.glyphs_kerning(lhs, rhs).map_or(0.0, Into::into))
            } else {
                Length::new(0.0)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use geom::Rect;
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn glyph_parse_from() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        let a = Glyph::parse_from(&face, GlyphId(1)).unwrap();
        assert_is_close!(a.advance, Length::new(540.0));
        assert_is_close!(
            a.path.bounds,
            Rect::new(Point::new(6.0, 0.0), Point::new(541.0, 656.0))
        );

        let v = Glyph::parse_from(&face, GlyphId(2)).unwrap();
        assert_is_close!(v.advance, Length::new(540.0));
        assert_is_close!(
            v.path.bounds,
            Rect::new(Point::new(6.0, 0.0), Point::new(541.0, 656.0))
        );
    }

    #[test]
    fn font_debug() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_eq!(format!("{font:?}"), r#"Font("demo regular")"#);
    }

    #[test]
    fn font_clone() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        #[allow(clippy::redundant_clone)]
        let _ = font.clone(); // Shouldn't panic
    }

    #[test]
    fn font_default() {
        let _ = Font::default(); // Shouldn't panic
    }

    #[test]
    fn font_from_ttf() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_matches!(font.face, Face { .. });
        assert!(font.family.get().is_none());
        assert!(font.name.get().is_none());
        assert!(font.cap_height.get().is_none());
        assert!(font.x_height.get().is_none());
        assert!(font.notdef.get().is_none());
        assert_eq!(font.glyphs.len(), 0);
        assert_eq!(font.kerning.len(), 0);
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
        assert_eq!(font.slope(), None);
        assert_eq!(font.num_glyphs(), 3);

        let data = std::fs::read(env!("NULL_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        let line_scaling = font.line_height() / default::line_height();
        assert_eq!(font.family(), "unknown");
        assert_eq!(font.name(), "unknown");
        assert_is_close!(font.em_size(), Length::new(1000.0));
        assert_is_close!(font.cap_height(), default::cap_height() * line_scaling);
        assert_is_close!(font.x_height(), default::x_height() * line_scaling);
        assert_is_close!(font.ascender(), Length::new(600.0));
        assert_is_close!(font.descender(), Length::new(400.0));
        assert_is_close!(font.line_gap(), Length::new(200.0));
        assert_is_close!(font.line_height(), Length::new(1200.0));
        assert_eq!(font.slope(), None);
        assert_eq!(font.num_glyphs(), 1); // Just .notdef
    }

    #[test]
    fn font_glyph() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert!(font.glyph('A').is_some());
        assert!(font.glyph('B').is_none());
    }

    #[test]
    fn font_glyph_or_default() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        let a = font.glyph_or_default('A').path;
        let b = font.glyph_or_default('B').path;
        let notdef = font.notdef().path;

        assert_ne!(a.len(), notdef.len());
        assert_eq!(b.len(), notdef.len());
        for (b_seg, notdef_seg) in b.data.iter().zip(notdef.data.iter()) {
            assert_is_close!(b_seg, notdef_seg);
        }
    }

    #[test]
    fn font_notdef() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_eq!(font.notdef().path.len(), 12);

        let data = std::fs::read(env!("NULL_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_eq!(font.notdef().path.len(), 26);
    }

    #[test]
    fn font_kerning() {
        type Length = geom::Length<FontUnit>;

        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_is_close!(font.kerning('A', 'V'), Length::new(-70.0));
        assert_is_close!(font.kerning('A', 'B'), Length::new(0.0));
    }
}
