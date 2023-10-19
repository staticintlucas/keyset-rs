use geom::{Affine, BezPath, Point};
use ttf_parser::{Face, GlyphId, OutlineBuilder};

#[derive(Clone, Debug)]
pub struct Glyph {
    advance: f64,
    path: BezPath,
}

impl Glyph {
    #[must_use]
    pub fn new(face: &Face, gid: GlyphId) -> Option<Self> {
        let advance = f64::from(face.glyph_hor_advance(gid)?);

        let mut path = PathWrapper(BezPath::new());
        face.outline_glyph(gid, &mut path);
        let path = path.0;

        Some(Self { advance, path })
    }

    #[must_use]
    pub fn notdef(cap_height: f64, slope: f64) -> Self {
        let mut path = BezPath::new();

        path.move_to((0., 0.)); // M 0 0
        path.line_to((0., 1000.)); // V 1000
        path.line_to((650., 1000.)); // H 650
        path.line_to((650., 0.)); // V 0
        path.line_to((0., 0.)); // H 0
        path.close_path(); // Z

        path.move_to((80., 150.)); // M 80 150
        path.line_to((270., 500.)); // L 270 500
        path.line_to((80., 850.)); // L 80 850
        path.line_to((80., 150.)); // V 150
        path.close_path(); // Z

        path.move_to((125., 920.)); // M 125 920
        path.line_to((325., 560.)); // L 325 560
        path.line_to((525., 920.)); // L 525 920
        path.line_to((125., 920.)); // H 125
        path.close_path(); // Z

        path.move_to((570., 850.)); // M 570 850
        path.line_to((380., 500.)); // L 380 500
        path.line_to((570., 150.)); // L 570 150
        path.line_to((570., 850.)); // V 850
        path.close_path(); // Z

        path.move_to((525., 80.)); // M 525 80
        path.line_to((325., 440.)); // L 325 440
        path.line_to((125., 80.)); // L 125 80
        path.line_to((525., 80.)); // H 525
        path.close_path(); // Z

        let skew_x = slope.to_radians().tan();
        let scale = cap_height / 1e3;
        let affine = Affine::skew(skew_x, 0.).pre_scale(scale);

        path.apply_affine(affine);

        // let advance = path.bounding_box().size().width;
        let advance = scale * (650. + (1000. * skew_x).abs());

        Self { advance, path }
    }

    pub fn advance(&self) -> f64 {
        self.advance
    }

    pub fn path(&self) -> &BezPath {
        &self.path
    }
}

struct PathWrapper(pub BezPath);

impl From<BezPath> for PathWrapper {
    fn from(value: BezPath) -> Self {
        Self(value)
    }
}

impl From<PathWrapper> for BezPath {
    fn from(value: PathWrapper) -> Self {
        value.0
    }
}

impl OutlineBuilder for PathWrapper {
    fn move_to(&mut self, x: f32, y: f32) {
        // Y axis is flipped in fonts compared to SVGs
        self.0.move_to(Point::new(x.into(), (-y).into()));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        // Y axis is flipped in fonts compared to SVGs
        self.0.line_to(Point::new(x.into(), (-y).into()));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // Y axis is flipped in fonts compared to SVGs
        self.0.quad_to(
            Point::new(x1.into(), (-y1).into()),
            Point::new(x.into(), (-y).into()),
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // Y axis is flipped in fonts compared to SVGs
        self.0.curve_to(
            Point::new(x1.into(), (-y1).into()),
            Point::new(x2.into(), (-y2).into()),
            Point::new(x.into(), (-y).into()),
        );
    }

    fn close(&mut self) {
        self.0.close_path();
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;
    use geom::{PathEl, Shape};

    use super::*;

    #[test]
    fn test_new() {
        let demo =
            std::fs::read(concat!(env!("CARGO_WORKSPACE_DIR"), "tests/fonts/demo.ttf")).unwrap();
        let demo = Face::parse(&demo, 0).unwrap();

        let a = Glyph::new(&demo, GlyphId(1)).unwrap();
        assert_approx_eq!(a.advance(), 540.);
        assert_eq!(a.path().elements().len(), 15);

        let null =
            std::fs::read(concat!(env!("CARGO_WORKSPACE_DIR"), "tests/fonts/null.ttf")).unwrap();
        let null = Face::parse(&null, 0).unwrap();

        let a = Glyph::new(&null, GlyphId(1));
        assert!(a.is_none()); // Glyph not found

        let notdef = Glyph::new(&null, GlyphId(0));
        assert!(notdef.is_none()); // Glyph has no outline
    }

    #[test]
    fn test_notdef() {
        let notdef = Glyph::notdef(500., 0.);

        assert_approx_eq!(notdef.path().bounding_box().origin().x, 0.0);
        assert_approx_eq!(notdef.path().bounding_box().origin().y, 0.0);
        assert_approx_eq!(notdef.path().bounding_box().size().width, 325.0);
        assert_approx_eq!(notdef.path().bounding_box().size().height, 500.0);
        assert_approx_eq!(notdef.advance(), 325.);

        let notdef = Glyph::notdef(500., 45.);
        assert_approx_eq!(notdef.path().bounding_box().size().width, 825.0);
        assert_approx_eq!(notdef.path().bounding_box().size().height, 500.0);
    }

    #[test]
    fn test_outline_builder() {
        let mut path: PathWrapper = BezPath::new().into();

        path.move_to(0., 0.);
        path.line_to(1., 1.);
        path.quad_to(2., 1., 2., 0.);
        path.curve_to(2., -0.5, 1.5, -1., 1., -1.);
        path.close();

        let bez_path: BezPath = path.into();
        let els = bez_path.elements();

        assert_eq!(els.len(), 5);
        assert_matches!(els[0], PathEl::MoveTo(..));
        assert_matches!(els[1], PathEl::LineTo(..));
        assert_matches!(els[2], PathEl::QuadTo(..));
        assert_matches!(els[3], PathEl::CurveTo(..));
        assert_matches!(els[4], PathEl::ClosePath);
    }
}
