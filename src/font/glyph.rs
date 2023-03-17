use crate::utils::Path;

use log::warn;
use ttf_parser::{Face, GlyphId};

#[derive(Clone, Debug)]
pub struct Glyph {
    pub advance: f32,
    pub path: Path,
}

impl Glyph {
    pub fn new(face: &Face, gid: GlyphId) -> Option<Self> {
        let advance = face.glyph_hor_advance(gid).map_or_else(
            || {
                warn!("no horizontal advance for glyph");
                0.
            },
            f32::from,
        );

        let mut path = Path::new();
        face.outline_glyph(gid, &mut path)?;

        if path.data.is_empty() {
            None // unreachable!() - outline_glyph should already have returned None
        } else {
            Some(Self { advance, path })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_new() {
        let demo = std::fs::read("tests/fonts/demo.ttf").unwrap();
        let demo = Face::parse(&demo, 0).unwrap();

        let a = Glyph::new(&demo, GlyphId(1)).unwrap();
        assert_approx_eq!(a.advance, 540.);
        assert_eq!(a.path.data.len(), 15);

        let null = std::fs::read("tests/fonts/null.ttf").unwrap();
        let null = Face::parse(&null, 0).unwrap();

        let a = Glyph::new(&null, GlyphId(1));
        assert!(a.is_none()); // Glyph not found

        let notdef = Glyph::new(&null, GlyphId(0));
        assert!(notdef.is_none()); // Glyph has no outline
    }
}
