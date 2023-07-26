use std::f64::consts::PI;

use crate::utils::{Path, Vec2};

use ttf_parser::{Face, GlyphId};

#[derive(Clone, Debug)]
pub struct Glyph {
    pub codepoint: Option<char>,
    pub advance: f64,
    pub path: Path,
}

impl Glyph {
    pub fn new(face: &Face, codepoint: Option<char>, gid: GlyphId) -> Option<Self> {
        let advance = f64::from(face.glyph_hor_advance(gid)?);

        let mut path = Path::new();
        face.outline_glyph(gid, &mut path);

        Some(Self {
            codepoint,
            advance,
            path,
        })
    }

    pub fn notdef(cap_height: f64, slope: f64) -> Self {
        let mut path = Path::new();

        path.abs_move(Vec2::ZERO); // M 0 0
        path.rel_vert_line(1000.); // v 1000
        path.rel_horiz_line(650.); // h 650
        path.rel_vert_line(-1000.); // v -1000
        path.rel_horiz_line(-650.); // h -650
        path.close(); // z

        path.abs_move(Vec2::new(80., 150.)); // M 80 150
        path.rel_line(Vec2::new(190., 350.)); // l 190 350
        path.rel_line(Vec2::new(-190., 350.)); // l -190 350
        path.rel_vert_line(-700.); // v -700
        path.close(); // z

        path.abs_move(Vec2::new(125., 920.)); // M 125 920
        path.rel_line(Vec2::new(200., -360.)); // l 200 -360
        path.rel_line(Vec2::new(200., 360.)); // l 200 360
        path.rel_horiz_line(-400.); // h -400
        path.close(); // z

        path.abs_move(Vec2::new(570., 850.)); // M 570 850
        path.rel_line(Vec2::new(-190., -350.)); // l -190 -350
        path.rel_line(Vec2::new(190., -350.)); // l 190 -350
        path.rel_vert_line(700.); // v 700
        path.close(); // z

        path.abs_move(Vec2::new(525., 80.)); // M 525 80
        path.rel_line(Vec2::new(-200., 360.)); // l -200 360
        path.rel_line(Vec2::new(-200., -360.)); // l -200 -360
        path.rel_horiz_line(400.); // h 400
        path.close(); // z

        path.scale(Vec2::from(cap_height / 1e3));
        path.skew_x(slope * PI / 180.);

        Self {
            codepoint: None,
            advance: path.bounds.size().x,
            path,
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

        let a = Glyph::new(&demo, Some('A'), GlyphId(1)).unwrap();
        assert_approx_eq!(a.advance, 540.);
        assert_eq!(a.path.data.len(), 15);

        let null = std::fs::read("tests/fonts/null.ttf").unwrap();
        let null = Face::parse(&null, 0).unwrap();

        let a = Glyph::new(&null, Some('A'), GlyphId(1));
        assert!(a.is_none()); // Glyph not found

        let notdef = Glyph::new(&null, None, GlyphId(0));
        assert!(notdef.is_none()); // Glyph has no outline
    }

    #[test]
    fn test_notdef() {
        let notdef = Glyph::notdef(500., 0.);

        assert_approx_eq!(notdef.path.bounds.position(), Vec2::new(0., 0.));
        assert_approx_eq!(notdef.path.bounds.size(), Vec2::new(325., 500.));
        assert_approx_eq!(notdef.advance, 325.);

        let notdef = Glyph::notdef(500., 45.);
        assert_approx_eq!(notdef.path.bounds.size(), Vec2::new(825., 500.));
    }
}
