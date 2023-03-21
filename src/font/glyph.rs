use std::f32::consts::PI;

use crate::utils::{Path, Point, Scale, Size};

use ttf_parser::{Face, GlyphId};

#[derive(Clone, Debug)]
pub struct Glyph {
    pub advance: f32,
    pub path: Path,
}

impl Glyph {
    pub fn new(face: &Face, gid: GlyphId) -> Option<Self> {
        let advance = f32::from(face.glyph_hor_advance(gid)?);

        let mut path = Path::new();
        let _ = face.outline_glyph(gid, &mut path);

        Some(Self { advance, path })
    }

    pub fn notdef(cap_height: f32, slope: f32) -> Self {
        let mut path = Path::new();

        path.abs_move(Point::new(0., 0.)); // M 0 0
        path.rel_vert_line(1000.); // v 1000
        path.rel_horiz_line(650.); // h 650
        path.rel_vert_line(-1000.); // v -1000
        path.rel_horiz_line(-650.); // h -650
        path.close(); // z

        path.abs_move(Point::new(80., 150.)); // M 80 150
        path.rel_line(Size::new(190., 350.)); // l 190 350
        path.rel_line(Size::new(-190., 350.)); // l -190 350
        path.rel_vert_line(-700.); // v -700
        path.close(); // z

        path.abs_move(Point::new(125., 920.)); // M 125 920
        path.rel_line(Size::new(200., -360.)); // l 200 -360
        path.rel_line(Size::new(200., 360.)); // l 200 360
        path.rel_horiz_line(-400.); // h -400
        path.close(); // z

        path.abs_move(Point::new(570., 850.)); // M 570 850
        path.rel_line(Size::new(-190., -350.)); // l -190 -350
        path.rel_line(Size::new(190., -350.)); // l 190 -350
        path.rel_vert_line(700.); // v 700
        path.close(); // z

        path.abs_move(Point::new(525., 80.)); // M 525 80
        path.rel_line(Size::new(-200., 360.)); // l -200 360
        path.rel_line(Size::new(-200., -360.)); // l -200 -360
        path.rel_horiz_line(400.); // h 400
        path.close(); // z

        path.scale(Scale::new(cap_height / 1e3, cap_height / 1e3));
        path.skew_x(slope * PI / 180.);

        Self {
            advance: path.bounds.w,
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

    #[test]
    fn test_notdef() {
        let notdef = Glyph::notdef(500., 0.);

        assert_approx_eq!(notdef.path.bounds.position(), Point::new(0., 0.));
        assert_approx_eq!(notdef.path.bounds.size(), Size::new(325., 500.));
        assert_approx_eq!(notdef.advance, 325.);

        let notdef = Glyph::notdef(500., 45.);
        assert_approx_eq!(notdef.path.bounds.size(), Size::new(825., 500.));
    }
}
