mod glyph;
mod kerning;

use std::collections::HashMap;

use crate::error::Result;
use crate::utils::{Path, Vec2};

use itertools::Itertools;
use log::warn;
use ttf_parser::{cmap, name_id, Face, GlyphId, OutlineBuilder};

use self::glyph::Glyph;
use self::kerning::Kerning;

#[derive(Clone, Debug)]
pub struct Font {
    pub name: String,
    pub em_size: f32,
    pub cap_height: f32,
    pub x_height: f32,
    pub ascent: f32,
    pub descent: f32,
    pub line_height: f32,
    pub slope: f32,
    pub glyphs: HashMap<char, Glyph>,
    pub notdef: Glyph,
    pub kerning: Kerning,
}

impl Font {
    pub fn from_ttf(data: &[u8]) -> Result<Self> {
        let face = Face::parse(data, 0)?;

        let name = if let Some(name) = face
            .names()
            .into_iter()
            .filter(|n| n.name_id == name_id::FAMILY)
            .find_map(|n| n.to_string())
        {
            name
        } else {
            warn!("cannot read font family name");
            "unknown".to_owned()
        };

        let em_size = f32::from(face.units_per_em());
        let cap_height = face.capital_height().map(f32::from);
        let x_height = face.x_height().map(f32::from);
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
            .iter()
            .filter_map(|&cp| Some((cp, face.glyph_index(cp)?)))
            .filter_map(|(cp, gid)| Some((cp, Glyph::new(&face, Some(cp), gid)?)))
            .collect();

        let cap_height = cap_height
            .or_else(|| Some(glyphs.get(&'X')?.path.bounds.size().y))
            .unwrap_or(0.6 * line_height); // TODO is there a better default?
        let x_height = x_height
            .or_else(|| Some(glyphs.get(&'x')?.path.bounds.size().y))
            .unwrap_or(0.4 * line_height); // TODO is there a better default?

        let notdef = if let Some(glyph) = Glyph::new(&face, None, GlyphId(0)) {
            glyph
        } else {
            warn!("no valid outline for glyph .notdef in font {name:?}");
            Glyph::notdef(cap_height, slope)
        };

        let kerning = if let Some(kern) = face.tables().kern {
            let mut kerning = Kerning::new();
            // TODO this is slow AF
            codepoints
                .iter()
                .copied()
                .cartesian_product(codepoints.iter().copied())
                .filter_map(|(l, r)| Some(((l, r), (face.glyph_index(l)?, face.glyph_index(r)?))))
                .for_each(|((l, r), (gid_l, gid_r))| {
                    let kern = kern
                        .subtables
                        .into_iter()
                        .find_map(|st| st.glyphs_kerning(gid_l, gid_r))
                        .unwrap_or(0);
                    kerning.set(l, r, f32::from(kern));
                });
            kerning
        } else {
            Kerning::new()
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
            kerning,
        })
    }
}

impl OutlineBuilder for crate::utils::Path {
    fn move_to(&mut self, x: f32, y: f32) {
        // Y axis is flipped in fonts compared to SVGs
        self.abs_move(Vec2::new(x, -y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        // Y axis is flipped in fonts compared to SVGs
        self.abs_line(Vec2::new(x, -y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // Y axis is flipped in fonts compared to SVGs
        self.abs_quadratic_bezier(Vec2::new(x1, -y1), Vec2::new(x, -y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // Y axis is flipped in fonts compared to SVGs
        self.abs_cubic_bezier(Vec2::new(x1, -y1), Vec2::new(x2, -y2), Vec2::new(x, -y));
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
        assert_eq!(font.glyphs.len(), 2);
        assert_ne!(
            font.notdef.advance,
            Glyph::notdef(font.cap_height, font.slope).advance
        );
        assert_eq!(font.kerning.len(), 1);

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
            Glyph::notdef(null.cap_height, null.slope).advance
        );
        assert_eq!(null.kerning.len(), 0);
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
