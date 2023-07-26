use svg::node::Value;

use crate::utils::Path;
use crate::utils::{PathSegment, RoundRect, Vec2};
use crate::ToSvg;

#[derive(Debug, Clone, Copy)]
pub enum EdgeType {
    Line,
    CurveStretch,
    CurveLineCurve,
    InsetCurve,
}

pub trait KeyHelpers {
    fn start(rect: RoundRect) -> Self;

    fn corner_top_left(&mut self, rect: RoundRect);
    fn corner_top_right(&mut self, rect: RoundRect);
    fn corner_bottom_right(&mut self, rect: RoundRect);
    fn corner_bottom_left(&mut self, rect: RoundRect);

    fn edge_top(&mut self, rect: RoundRect, size: Vec2, typ: EdgeType, curve: f64);
    fn edge_right(&mut self, rect: RoundRect, size: Vec2, typ: EdgeType, curve: f64);
    fn edge_bottom(&mut self, rect: RoundRect, size: Vec2, typ: EdgeType, curve: f64);
    fn edge_left(&mut self, rect: RoundRect, size: Vec2, typ: EdgeType, curve: f64);

    fn radius(curve: f64, distance: f64) -> Vec2 {
        Vec2::from((curve.powf(2.) + (distance.powf(2.) / 4.)) / (2. * curve))
    }
}

impl KeyHelpers for Path {
    fn start(rect: RoundRect) -> Self {
        let mut slf = Self::new();
        slf.abs_move(rect.position() + Vec2::new(0., rect.radius().y));
        slf
    }

    fn corner_top_left(&mut self, rect: RoundRect) {
        self.rel_arc(
            rect.radius(),
            0.,
            false,
            true,
            rect.radius() * Vec2::new(1., -1.),
        );
    }

    fn corner_top_right(&mut self, rect: RoundRect) {
        self.rel_arc(rect.radius(), 0., false, true, rect.radius());
    }

    fn corner_bottom_right(&mut self, rect: RoundRect) {
        self.rel_arc(
            rect.radius(),
            0.,
            false,
            true,
            rect.radius() * Vec2::new(-1., 1.),
        );
    }

    fn corner_bottom_left(&mut self, rect: RoundRect) {
        self.rel_arc(rect.radius(), 0., false, true, rect.radius() * -1.);
    }

    fn edge_top(&mut self, rect: RoundRect, size: Vec2, typ: EdgeType, curve: f64) {
        let rect_dx = rect.size().x - 2. * rect.radius().x;
        let size_dx = size.x - 1e3;
        let dx = rect_dx + size_dx;
        match typ {
            EdgeType::Line => self.rel_horiz_line(dx),
            EdgeType::CurveLineCurve if size_dx > 0.01 => {
                let radius = Self::radius(curve, rect_dx);
                self.rel_arc(radius, 0., false, true, Vec2::new(rect_dx / 2., -curve));
                self.rel_horiz_line(size_dx);
                self.rel_arc(radius, 0., false, true, Vec2::new(rect_dx / 2., curve));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let radius = Self::radius(curve, dx);
                self.rel_arc(radius, 0., false, true, Vec2::new(dx, 0.));
            }
            EdgeType::InsetCurve => {
                let radius = Self::radius(curve, dx);
                self.rel_arc(radius, 0., false, false, Vec2::new(dx, 0.));
            }
        }
    }

    fn edge_right(&mut self, rect: RoundRect, size: Vec2, typ: EdgeType, curve: f64) {
        let rect_dy = rect.size().y - 2. * rect.radius().y;
        let size_dy = size.y - 1e3;
        let dy = rect_dy + size_dy;
        match typ {
            EdgeType::Line => self.rel_vert_line(dy),
            EdgeType::CurveLineCurve if size_dy > 0.01 => {
                let radius = Self::radius(curve, rect_dy);
                self.rel_arc(radius, 0., false, true, Vec2::new(curve, rect_dy / 2.));
                self.rel_vert_line(size_dy);
                self.rel_arc(radius, 0., false, true, Vec2::new(-curve, rect_dy / 2.));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let radius = Self::radius(curve, dy);
                self.rel_arc(radius, 0., false, true, Vec2::new(0., dy));
            }
            EdgeType::InsetCurve => {
                let radius = Self::radius(curve, dy);
                self.rel_arc(radius, 0., false, false, Vec2::new(0., dy));
            }
        }
    }

    fn edge_bottom(&mut self, rect: RoundRect, size: Vec2, typ: EdgeType, curve: f64) {
        let rect_dx = rect.size().x - 2. * rect.radius().x;
        let size_dx = size.x - 1e3;
        let dx = rect_dx + size_dx;
        match typ {
            EdgeType::Line => self.rel_horiz_line(-dx),
            EdgeType::CurveLineCurve if size_dx > 0.01 => {
                let radius = Self::radius(curve, rect_dx);
                self.rel_arc(radius, 0., false, true, Vec2::new(-rect_dx / 2., curve));
                self.rel_horiz_line(-size_dx);
                self.rel_arc(radius, 0., false, true, Vec2::new(-rect_dx / 2., -curve));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let radius = Self::radius(curve, dx);
                self.rel_arc(radius, 0., false, true, Vec2::new(-dx, 0.));
            }
            EdgeType::InsetCurve => {
                let radius = Self::radius(curve, dx);
                self.rel_arc(radius, 0., false, false, Vec2::new(-dx, 0.));
            }
        }
    }

    fn edge_left(&mut self, rect: RoundRect, size: Vec2, typ: EdgeType, curve: f64) {
        let rect_dy = rect.size().y - 2. * rect.radius().y;
        let size_dy = size.y - 1e3;
        let dy = rect_dy + size_dy;
        match typ {
            EdgeType::Line => self.rel_vert_line(-dy),
            EdgeType::CurveLineCurve if size_dy > 0.01 => {
                let radius = Self::radius(curve, rect_dy);
                self.rel_arc(radius, 0., false, true, Vec2::new(-curve, -rect_dy / 2.));
                self.rel_vert_line(size_dy);
                self.rel_arc(radius, 0., false, true, Vec2::new(curve, -rect_dy / 2.));
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let radius = Self::radius(curve, dy);
                self.rel_arc(radius, 0., false, true, Vec2::new(0., -dy));
            }
            EdgeType::InsetCurve => {
                let radius = Self::radius(curve, dy);
                self.rel_arc(radius, 0., false, false, Vec2::new(0., -dy));
            }
        }
    }
}

impl ToSvg for PathSegment {
    fn to_svg(&self) -> String {
        match *self {
            Self::Move(p) => format!(
                "M{} {}",
                (1e5 * p.x).floor() / 1e5,
                (1e5 * p.y).floor() / 1e5
            ),
            Self::Line(d) => format!(
                "l{} {}",
                (1e5 * d.x).floor() / 1e5,
                (1e5 * d.y).floor() / 1e5
            ),
            Self::CubicBezier(d1, d2, d) => format!(
                "c{} {} {} {} {} {}",
                (1e5 * d1.x).floor() / 1e5,
                (1e5 * d1.y).floor() / 1e5,
                (1e5 * d2.x).floor() / 1e5,
                (1e5 * d2.y).floor() / 1e5,
                (1e5 * d.x).floor() / 1e5,
                (1e5 * d.y).floor() / 1e5
            ),
            Self::QuadraticBezier(d1, d) => {
                format!(
                    "q{} {} {} {}",
                    (1e5 * d1.x).floor() / 1e5,
                    (1e5 * d1.y).floor() / 1e5,
                    (1e5 * d.x).floor() / 1e5,
                    (1e5 * d.y).floor() / 1e5
                )
            }
            Self::Close => "z".into(),
        }
    }
}

impl ToSvg for Path {
    fn to_svg(&self) -> String {
        self.data
            .iter()
            .fold(String::new(), |result, seg| result + seg.to_svg().as_str())
    }
}

impl From<Path> for Value {
    fn from(value: Path) -> Self {
        value.to_svg().into()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, SQRT_2};

    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_radius() {
        assert_approx_eq!(Path::radius(1. - FRAC_1_SQRT_2, SQRT_2), Vec2::from(1.));
    }

    #[test]
    fn test_corners() {
        let position = Vec2::new(200., 100.);
        let size = Vec2::from(600.);
        let radius = Vec2::from(50.);
        let rect = RoundRect::new(position, size, radius);
        let path = Path::start(rect);

        let corner_funcs: Vec<fn(&mut Path, RoundRect)> = vec![
            Path::corner_top_left,
            Path::corner_top_right,
            Path::corner_bottom_right,
            Path::corner_bottom_left,
        ];

        for func in corner_funcs {
            let mut path = path.clone();
            func(&mut path, rect);
            assert_eq!(path.data.len(), 2);
            assert_matches!(path.data[1], PathSegment::CubicBezier(..));
        }
    }

    #[test]
    fn test_edges() {
        let position = Vec2::new(200., 100.);
        let size = Vec2::from(600.);
        let radius = Vec2::from(50.);
        let rect = RoundRect::new(position, size, radius);
        let size = Vec2::from(2e3);
        let curve = 20.;
        let path = Path::start(rect);

        // TODO 1.69+ seems to allow us to remove type annotations (even though it's not mentioned
        // in the release notes). Remove it when MSRV >= 1.69
        let edge_funcs: Vec<fn(&mut Path, RoundRect, Vec2, EdgeType, f64)> = vec![
            Path::edge_top,
            Path::edge_right,
            Path::edge_bottom,
            Path::edge_left,
        ];
        let edge_type_len = vec![
            (EdgeType::Line, 1),
            (EdgeType::CurveStretch, 1),
            (EdgeType::CurveLineCurve, 3),
            (EdgeType::InsetCurve, 1),
        ];

        for func in edge_funcs {
            for &(edge_type, len) in &edge_type_len {
                let mut path = path.clone();
                func(&mut path, rect, size, edge_type, curve);

                assert_eq!(path.data.len(), len + 1);
            }
        }
    }

    #[test]
    fn test_path_to_svg() {
        let mut path = Path::new();
        path.abs_move(Vec2::ZERO);
        path.rel_line(Vec2::new(1., 1.));
        path.rel_cubic_bezier(Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5), Vec2::new(2., 0.));
        path.rel_quadratic_bezier(Vec2::new(0.5, -0.5), Vec2::new(1., 0.));
        path.close();

        assert_eq!(path.to_svg(), "M0 0l1 1c0.5 0.5 1.5 0.5 2 0q0.5 -0.5 1 0z");
    }
}
