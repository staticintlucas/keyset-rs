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
#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)] // TODO

mod default;
mod error;
mod face;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{OnceLock, RwLock};

use geom::{BezPath, Rect, Shape};
use log::warn;
use ttf_parser::GlyphId;

pub use self::error::{Error, Result};
use face::Face;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Glyph {
    pub path: BezPath,
    pub bounds: Rect,
    pub advance: f64,
}

impl Glyph {
    fn parse_from(face: &Face, gid: GlyphId) -> Option<Self> {
        struct BezPathBuilder(BezPath);

        // GRCOV_EXCL_START // TODO these are pretty trivial but we could cover them in tests
        impl ttf_parser::OutlineBuilder for BezPathBuilder {
            fn move_to(&mut self, x: f32, y: f32) {
                // Y axis is flipped in fonts compared to SVGs
                self.0.move_to((x.into(), (-y).into()));
            }

            fn line_to(&mut self, x: f32, y: f32) {
                // Y axis is flipped in fonts compared to SVGs
                self.0.line_to((x.into(), (-y).into()));
            }

            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                // Y axis is flipped in fonts compared to SVGs
                self.0
                    .quad_to((x1.into(), (-y1).into()), (x.into(), (-y).into()));
            }

            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                // Y axis is flipped in fonts compared to SVGs
                self.0.curve_to(
                    (x1.into(), (-y1).into()),
                    (x2.into(), (-y2).into()),
                    (x.into(), (-y).into()),
                );
            }

            fn close(&mut self) {
                self.0.close_path();
            }
        }
        // GRCOV_EXCL_STOP

        let mut builder = BezPathBuilder(BezPath::new());

        let bounds = face.outline_glyph(gid, &mut builder)?;
        let path = builder.0;

        let bounds = Rect::new(
            f64::from(bounds.x_min),
            f64::from(bounds.y_min),
            f64::from(bounds.x_max),
            f64::from(bounds.y_max),
        );

        let advance = f64::from(face.glyph_hor_advance(gid)?);

        Some(Self {
            path,
            bounds,
            advance,
        })
    }
}

pub struct Font {
    face: Face,
    family: OnceLock<String>,
    name: OnceLock<String>,
    cap_height: OnceLock<f64>,
    x_height: OnceLock<f64>,
    notdef: OnceLock<Glyph>,
    glyphs: RwLock<HashMap<char, Option<Glyph>>>,
    kerning: RwLock<HashMap<(char, char), f64>>,
}

impl Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Font").field(self.name()).finish()
    }
}

impl Clone for Font {
    fn clone(&self) -> Self {
        Self {
            face: self.face.clone(),
            family: self.family.clone(),
            name: self.name.clone(),
            cap_height: self.cap_height.clone(),
            x_height: self.x_height.clone(),
            notdef: self.notdef.clone(),
            glyphs: RwLock::new(self.glyphs.read().unwrap().clone()),
            kerning: RwLock::new(self.kerning.read().unwrap().clone()),
        }
    }
}

impl Default for Font {
    fn default() -> Self {
        default::font().clone()
    }
}

impl Font {
    pub fn from_ttf(data: Vec<u8>) -> Result<Self> {
        Ok(Self {
            face: Face::from_ttf(data)?,
            family: OnceLock::new(),
            name: OnceLock::new(),
            cap_height: OnceLock::new(),
            x_height: OnceLock::new(),
            notdef: OnceLock::new(),
            glyphs: RwLock::new(HashMap::new()),
            kerning: RwLock::new(HashMap::new()),
        })
    }

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

    pub fn em_size(&self) -> f64 {
        f64::from(self.face.units_per_em())
    }

    pub fn cap_height(&self) -> f64 {
        *self.cap_height.get_or_init(|| {
            self.face
                .capital_height()
                .map(f64::from)
                .or_else(|| self.glyph('M').map(|g| g.path.bounding_box().height()))
                .unwrap_or_else(|| {
                    default::cap_height() / default::line_height() * self.line_height()
                })
        })
    }

    pub fn x_height(&self) -> f64 {
        *self.x_height.get_or_init(|| {
            self.face
                .x_height()
                .map(f64::from)
                .or_else(|| self.glyph('x').map(|g| g.path.bounding_box().height()))
                .unwrap_or_else(|| {
                    default::x_height() / default::line_height() * self.line_height()
                })
        })
    }

    pub fn ascent(&self) -> f64 {
        f64::from(self.face.ascender())
    }

    pub fn descent(&self) -> f64 {
        -f64::from(self.face.descender())
    }

    pub fn line_gap(&self) -> f64 {
        f64::from(self.face.line_gap())
    }

    pub fn line_height(&self) -> f64 {
        self.ascent() + self.descent() + self.line_gap()
    }

    pub fn slope(&self) -> Option<f64> {
        self.face.italic_angle().map(f64::from)
    }

    pub fn num_glyphs(&self) -> usize {
        usize::from(self.face.number_of_glyphs())
    }

    pub fn glyph(&self, char: char) -> Option<Glyph> {
        self.glyphs
            .write()
            .unwrap()
            .entry(char)
            .or_insert_with(|| {
                self.face
                    .glyph_index(char)
                    .and_then(|gid| Glyph::parse_from(&self.face, gid))
            })
            .clone()
    }

    pub fn glyph_or_default(&self, char: char) -> Glyph {
        self.glyph(char).unwrap_or_else(|| self.notdef())
    }

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

    pub fn kerning(&self, left: char, right: char) -> f64 {
        *self
            .kerning
            .write()
            .unwrap()
            .entry((left, right))
            .or_insert_with(|| {
                if let (Some(lhs), Some(rhs)) =
                    (self.face.glyph_index(left), self.face.glyph_index(right))
                {
                    self.face.glyphs_kerning(lhs, rhs).map_or(0.0, f64::from)
                } else {
                    0.0
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn glyph_parse_from() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        let a = Glyph::parse_from(&face, GlyphId(1)).unwrap();
        assert_approx_eq!(a.advance, 540.0);
        assert_approx_eq!(a.bounds.width(), 535.0);
        assert_approx_eq!(a.bounds.height(), 656.0);

        let v = Glyph::parse_from(&face, GlyphId(2)).unwrap();
        assert_approx_eq!(v.advance, 540.0);
        assert_approx_eq!(v.bounds.width(), 535.0);
        assert_approx_eq!(v.bounds.height(), 656.0);
    }

    #[test]
    fn font_debug() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_eq!(format!("{:?}", font), r#"Font("demo regular")"#);
    }

    #[test]
    fn font_clone() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

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
        assert_eq!(font.glyphs.read().unwrap().len(), 0);
        assert_eq!(font.kerning.read().unwrap().len(), 0);
    }

    #[test]
    fn font_properties() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_eq!(font.family(), "demo");
        assert_eq!(font.name(), "demo regular");
        assert_approx_eq!(font.em_size(), 1000.0);
        assert_approx_eq!(font.cap_height(), 650.0);
        assert_approx_eq!(font.x_height(), 450.0);
        assert_approx_eq!(font.ascent(), 1024.0);
        assert_approx_eq!(font.descent(), 400.0);
        assert_approx_eq!(font.line_gap(), 0.0);
        assert_approx_eq!(font.line_height(), 1424.0);
        assert_eq!(font.slope(), None);
        assert_eq!(font.num_glyphs(), 3);

        let data = std::fs::read(env!("NULL_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        let line_scaling = font.line_height() / default::line_height();
        assert_eq!(font.family(), "unknown");
        assert_eq!(font.name(), "unknown");
        assert_approx_eq!(font.em_size(), 1000.0);
        assert_approx_eq!(font.cap_height(), default::cap_height() * line_scaling);
        assert_approx_eq!(font.x_height(), default::x_height() * line_scaling);
        assert_approx_eq!(font.ascent(), 600.0);
        assert_approx_eq!(font.descent(), 400.0);
        assert_approx_eq!(font.line_gap(), 200.0);
        assert_approx_eq!(font.line_height(), 1200.0);
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

        assert_ne!(font.glyph_or_default('A').path, font.notdef().path);
        assert_eq!(font.glyph_or_default('B').path, font.notdef().path);
    }

    #[test]
    fn font_notdef() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_eq!(font.notdef().path.elements().len(), 12);

        let data = std::fs::read(env!("NULL_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_eq!(font.notdef().path.elements().len(), 26);
    }

    #[test]
    fn font_kerning() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let font = Font::from_ttf(data).unwrap();

        assert_approx_eq!(font.kerning('A', 'V'), -70.0);
        assert_approx_eq!(font.kerning('A', 'B'), 0.0);
    }
}
