mod error;
mod glyph;
mod kerning;

use std::collections::HashMap;

use geom::Shape;
use log::warn;
use ttf_parser::{cmap, name_id, Face, GlyphId};

pub use self::error::{Error, Result};
pub use self::glyph::Glyph;
pub use self::kerning::Kerning;

#[derive(Clone, Debug)]
pub struct Font {
    pub name: String,
    pub em_size: f64,
    pub cap_height: f64,
    pub x_height: f64,
    pub ascent: f64,
    pub descent: f64,
    pub line_height: f64,
    pub slope: f64,
    pub glyphs: HashMap<char, Glyph>,
    pub notdef: Glyph,
    pub kerning: Kerning,
}

impl Default for Font {
    fn default() -> Self {
        let cap_height = 714.;
        let x_height = 523.;
        let ascent = 952.;
        let descent = 213.;
        let slope = 0.;

        Self {
            name: String::new(),
            em_size: 1000.,
            cap_height,
            x_height,
            ascent,
            descent,
            line_height: ascent + descent,
            slope,
            glyphs: HashMap::new(),
            notdef: Glyph::notdef(cap_height, slope),
            kerning: Kerning::new(),
        }
    }
}

impl Font {
    pub fn from_ttf(data: &[u8]) -> Result<Self> {
        let face = Face::parse(data, 0)?;

        let name = face
            .names()
            .into_iter()
            .filter(|n| n.name_id == name_id::FAMILY)
            .find_map(|n| n.to_string())
            .map_or_else(
                || {
                    warn!("cannot read font family name");
                    "unknown".to_owned()
                },
                |name| name,
            );

        let em_size = f64::from(face.units_per_em());
        let cap_height = face.capital_height().map(f64::from);
        let x_height = face.x_height().map(f64::from);
        let ascent = f64::from(face.ascender());
        let descent = f64::from(-face.descender());
        let line_height = ascent + descent + f64::from(face.line_gap());
        let slope = face.italic_angle().map_or(0., f64::from);

        let codepoints = face.tables().cmap.map_or_else(
            || {
                warn!("no CMAP table in font {name:?}");
                vec![]
            },
            |cmap| {
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
            },
        );

        if codepoints.is_empty() {
            warn!("no valid Unicode code points font {name:?}");
        }

        let glyphs: HashMap<_, _> = codepoints
            .iter()
            .filter_map(|&cp| Some((cp, face.glyph_index(cp)?)))
            .filter_map(|(cp, gid)| Some((cp, Glyph::new(&face, gid)?)))
            .collect();

        let cap_height = cap_height
            .or_else(|| Some(glyphs.get(&'X')?.path.bounding_box().size().height))
            .unwrap_or(0.6 * line_height); // TODO is there a better default?
        let x_height = x_height
            .or_else(|| Some(glyphs.get(&'x')?.path.bounding_box().size().height))
            .unwrap_or(0.4 * line_height); // TODO is there a better default?

        let notdef = Glyph::new(&face, GlyphId(0)).map_or_else(
            || {
                warn!("no valid outline for glyph .notdef in font {name:?}");
                Glyph::notdef(cap_height, slope)
            },
            |glyph| glyph,
        );

        let kerning = face.tables().kern.map_or_else(Kerning::new, |kern| {
            let mut kerning = Kerning::new();

            // TODO this is slow AF
            let ch_gid = codepoints
                .iter()
                .copied()
                .filter_map(|cp| Some((cp, face.glyph_index(cp)?)));
            for (l_ch, l_gid) in ch_gid.clone() {
                for (r_ch, r_gid) in ch_gid.clone() {
                    if let Some(kern) = kern
                        .subtables
                        .into_iter()
                        .find_map(|st| st.glyphs_kerning(l_gid, r_gid))
                    {
                        kerning.set(l_ch, r_ch, f64::from(kern));
                    }
                }
            }

            kerning
        });

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

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_font_default() {
        let font = Font::default();

        assert_eq!(font.name, "");
        assert_approx_eq!(font.em_size, 1e3);
        assert_approx_eq!(font.cap_height, 714.);
        assert_approx_eq!(font.x_height, 523.);
        assert_approx_eq!(font.ascent, 952.);
        assert_approx_eq!(font.descent, 213.);
        assert_approx_eq!(font.line_height, 1165.);
        assert_approx_eq!(font.slope, 0.);
        assert_eq!(font.glyphs.len(), 0);
        assert_eq!(font.kerning.len(), 0);
    }

    #[test]
    fn test_from_ttf() {
        let data =
            std::fs::read(concat!(env!("CARGO_WORKSPACE_DIR"), "tests/fonts/demo.ttf")).unwrap();
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

        let data =
            std::fs::read(concat!(env!("CARGO_WORKSPACE_DIR"), "tests/fonts/null.ttf")).unwrap();
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
}