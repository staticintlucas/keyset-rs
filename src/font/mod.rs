mod glyph;
mod notdef;

use std::collections::HashMap;

use crate::error::Result;
use crate::utils::{Path, Point};

use log::warn;
use ttf_parser::{cmap, Face, GlyphId, OutlineBuilder};

use self::glyph::Glyph;

#[derive(Clone, Debug)]
pub struct Font {
    name: String,
    em_size: f32,
    cap_height: f32,
    x_height: f32,
    ascent: f32,
    descent: f32,
    line_height: f32,
    slope: f32,
    glyphs: HashMap<char, Glyph>,
    notdef: Glyph,
}

impl Font {
    pub fn from_ttf(data: &[u8]) -> Result<Self> {
        let face = Face::parse(data, 0)?;

        let name = if let Some(name) = face
            .names()
            .into_iter()
            .filter(|n| n.name_id == 1)
            .find_map(|n| n.to_string())
        {
            name
        } else {
            warn!("cannot read font family name");
            "unknown".to_owned()
        };

        let em_size = f32::from(face.units_per_em());
        let cap_height = f32::from(face.capital_height().unwrap_or(0)); // TODO calculate default
        let x_height = f32::from(face.x_height().unwrap_or(0)); // TODO calculate default
        let ascent = f32::from(face.ascender());
        let descent = f32::from(-face.descender());
        let line_height = ascent + descent + f32::from(face.line_gap());
        let slope = face.italic_angle().unwrap_or(0.);

        let codepoints = if let Some(cmap) = face.tables().cmap {
            cmap.subtables
                .into_iter()
                .filter(cmap::Subtable::is_unicode) // Filter out non-unicode subtables
                .flat_map(|st| {
                    let mut codepoints = Vec::with_capacity(256);
                    // This is the only way to iterate code points in a subtable
                    st.codepoints(|cp| codepoints.push(cp));
                    codepoints
                })
                .filter_map(char::from_u32) // Convert to char, filtering out invalid
                .collect()
        } else {
            warn!("no CMAP table in font {name:?}");
            vec![]
        };
        if codepoints.is_empty() {
            warn!("no valid Unicode code points font {name:?}");
        }

        let glyphs: HashMap<_, _> = codepoints
            .into_iter()
            .filter_map(|cp| Some((cp, face.glyph_index(cp)?)))
            .filter_map(|(cp, gid)| Some((cp, Glyph::new(&face, gid)?)))
            .collect();

        let notdef = if let Some(glyph) = Glyph::new(&face, GlyphId(0)) {
            glyph
        } else {
            warn!("no valid outline for glyph .notdef in font {name:?}");
            self::notdef::path(cap_height, slope)
        };

        Ok(Self {
            name,
            em_size,
            cap_height,
            x_height,
            ascent,
            descent,
            line_height,
            slope,
            glyphs,
            notdef,
        })
    }
}

impl OutlineBuilder for crate::utils::Path {
    fn move_to(&mut self, x: f32, y: f32) {
        self.abs_move(Point::new(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.abs_line(Point::new(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.abs_quadratic_bezier(Point::new(x1, y1), Point::new(x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.abs_cubic_bezier(Point::new(x1, y1), Point::new(x2, y2), Point::new(x, y));
    }

    fn close(&mut self) {
        Path::close(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;

    #[test]
    fn test_from_ttf() {
        let data = std::fs::read("tests/fonts/demo.ttf").unwrap();
        let font = Font::from_ttf(&data).unwrap();

        assert_eq!(font.name, "demo");
        assert_approx_eq!(font.em_size, 1e3);
        assert_approx_eq!(font.cap_height, 650.);
        assert_approx_eq!(font.x_height, 450.);
        assert_approx_eq!(font.line_height, 1424.);
        assert_approx_eq!(font.slope, 0.);
        assert_eq!(font.glyphs.len(), 1);
        assert_ne!(
            font.notdef.advance,
            notdef::path(font.cap_height, font.slope).advance
        );

        let data = std::fs::read("tests/fonts/null.ttf").unwrap();
        let null = Font::from_ttf(&data).unwrap();

        assert_eq!(null.name, "unknown");
        assert_approx_eq!(null.em_size, 1e3);
        assert_approx_eq!(null.cap_height, 0.);
        assert_approx_eq!(null.x_height, 0.);
        assert_approx_eq!(null.line_height, 0.);
        assert_approx_eq!(null.slope, 0.);
        assert_eq!(null.glyphs.len(), 0);
        assert_eq!(
            null.notdef.advance,
            notdef::path(null.cap_height, null.slope).advance
        );
    }

    #[test]
    fn test_outline_builder() {
        use crate::utils::PathSegment;

        let mut path = Path::new();

        path.move_to(0., 0.);
        path.line_to(1., 1.);
        path.quad_to(2., 1., 2., 0.);
        path.curve_to(2., -0.5, 1.5, -1., 1., -1.);
        OutlineBuilder::close(&mut path);

        assert_eq!(path.data.len(), 5);
        assert_matches!(path.data[0], PathSegment::Move(..));
        assert_matches!(path.data[1], PathSegment::Line(..));
        assert_matches!(path.data[2], PathSegment::QuadraticBezier(..));
        assert_matches!(path.data[3], PathSegment::CubicBezier(..));
        assert_matches!(path.data[4], PathSegment::Close);
    }
}
