use std::f32::consts::PI;

use crate::utils::{Path, Point, Scale, Size};

use super::glyph::Glyph;

pub fn path(cap_height: f32, slope: f32) -> Glyph {
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

    Glyph {
        advance: path.bounds.w,
        path,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_notdef() {
        let notdef = path(500., 0.);

        assert_approx_eq!(notdef.path.bounds.position(), Point::new(0., 0.));
        assert_approx_eq!(notdef.path.bounds.size(), Size::new(325., 500.));
        assert_approx_eq!(notdef.advance, 325.);

        let notdef = path(500., 45.);
        assert_approx_eq!(notdef.path.bounds.size(), Size::new(825., 500.));
    }
}
