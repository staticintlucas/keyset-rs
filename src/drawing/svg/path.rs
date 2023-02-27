use svg::node::Value;

use crate::utils::{Path, Trim};
use crate::utils::{PathSegment, RoundRect, Size};
use crate::ToSvg;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    Line,
    CurveStretch,
    CurveLineCurve,
    InsetCurve,
}

pub trait KeyHelpers {
    fn start(rect: RoundRect) -> Self;

    fn corner_top_left(self, rect: RoundRect) -> Self;
    fn corner_top_right(self, rect: RoundRect) -> Self;
    fn corner_bottom_right(self, rect: RoundRect) -> Self;
    fn corner_bottom_left(self, rect: RoundRect) -> Self;

    fn edge_top(self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self;
    fn edge_right(self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self;
    fn edge_bottom(self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self;
    fn edge_left(self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self;

    fn radius(curve: f32, distance: f32) -> Size {
        let r = (curve.powf(2.) + (distance.powf(2.) / 4.)) / (2. * curve);
        Size::new(r, r)
    }
}

impl KeyHelpers for Path {
    fn start(rect: RoundRect) -> Self {
        Self::new().abs_move(rect.position() + Size::new(0., rect.ry))
    }

    fn corner_top_left(self, rect: RoundRect) -> Self {
        let radius = Size::new(rect.rx.abs(), rect.ry.abs());
        self.rel_arc(radius, 0., false, true, Size::new(rect.rx, -rect.ry))
    }

    fn corner_top_right(self, rect: RoundRect) -> Self {
        let radius = Size::new(rect.rx.abs(), rect.ry.abs());
        self.rel_arc(radius, 0., false, true, Size::new(rect.rx, rect.ry))
    }

    fn corner_bottom_right(self, rect: RoundRect) -> Self {
        let radius = Size::new(rect.rx.abs(), rect.ry.abs());
        self.rel_arc(radius, 0., false, true, Size::new(-rect.rx, rect.ry))
    }

    fn corner_bottom_left(self, rect: RoundRect) -> Self {
        let radius = Size::new(rect.rx.abs(), rect.ry.abs());
        self.rel_arc(radius, 0., false, true, Size::new(-rect.rx, -rect.ry))
    }

    fn edge_top(self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self {
        let rect_dx = rect.w - 2. * rect.rx;
        let size_dx = size.w - 1e3;
        let dx = rect_dx + size_dx;
        match typ {
            EdgeType::Line => self.rel_horiz_line(dx),
            EdgeType::CurveLineCurve if size_dx > 0.01 => {
                let radius = Self::radius(curve, rect_dx);
                self.rel_arc(radius, 0., false, true, Size::new(rect_dx / 2., -curve))
                    .rel_horiz_line(size_dx)
                    .rel_arc(radius, 0., false, true, Size::new(rect_dx / 2., curve))
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let radius = Self::radius(curve, dx);
                self.rel_arc(radius, 0., false, true, Size::new(dx, 0.))
            }
            EdgeType::InsetCurve => {
                let radius = Self::radius(curve, dx);
                self.rel_arc(radius, 0., false, false, Size::new(dx, 0.))
            }
        }
    }

    fn edge_right(self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self {
        let rect_dy = rect.h - 2. * rect.ry;
        let size_dy = size.h - 1e3;
        let dy = rect_dy + size_dy;
        match typ {
            EdgeType::Line => self.rel_vert_line(dy),
            EdgeType::CurveLineCurve if size_dy > 0.01 => {
                let radius = Self::radius(curve, rect_dy);
                self.rel_arc(radius, 0., false, true, Size::new(curve, rect_dy / 2.))
                    .rel_vert_line(size_dy)
                    .rel_arc(radius, 0., false, true, Size::new(-curve, rect_dy / 2.))
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let radius = Self::radius(curve, dy);
                self.rel_arc(radius, 0., false, true, Size::new(0., dy))
            }
            EdgeType::InsetCurve => {
                let radius = Self::radius(curve, dy);
                self.rel_arc(radius, 0., false, false, Size::new(0., dy))
            }
        }
    }

    fn edge_bottom(self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self {
        let rect_dx = rect.w - 2. * rect.rx;
        let size_dx = size.w - 1e3;
        let dx = rect_dx + size_dx;
        match typ {
            EdgeType::Line => self.rel_horiz_line(-dx),
            EdgeType::CurveLineCurve if size_dx > 0.01 => {
                let radius = Self::radius(curve, rect_dx);
                self.rel_arc(radius, 0., false, true, Size::new(-rect_dx / 2., curve))
                    .rel_horiz_line(-size_dx)
                    .rel_arc(radius, 0., false, true, Size::new(-rect_dx / 2., -curve))
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let radius = Self::radius(curve, dx);
                self.rel_arc(radius, 0., false, true, Size::new(-dx, 0.))
            }
            EdgeType::InsetCurve => {
                let radius = Self::radius(curve, dx);
                self.rel_arc(radius, 0., false, false, Size::new(-dx, 0.))
            }
        }
    }

    fn edge_left(self, rect: RoundRect, size: Size, typ: EdgeType, curve: f32) -> Self {
        let rect_dy = rect.h - 2. * rect.ry;
        let size_dy = size.h - 1e3;
        let dy = rect_dy + size_dy;
        match typ {
            EdgeType::Line => self.rel_vert_line(-dy),
            EdgeType::CurveLineCurve if size_dy > 0.01 => {
                let radius = Self::radius(curve, rect_dy);
                self.rel_arc(radius, 0., false, true, Size::new(-curve, -rect_dy / 2.))
                    .rel_vert_line(size_dy)
                    .rel_arc(radius, 0., false, true, Size::new(curve, -rect_dy / 2.))
            }
            EdgeType::CurveLineCurve | EdgeType::CurveStretch => {
                let radius = Self::radius(curve, dy);
                self.rel_arc(radius, 0., false, true, Size::new(0., -dy))
            }
            EdgeType::InsetCurve => {
                let radius = Self::radius(curve, dy);
                self.rel_arc(radius, 0., false, false, Size::new(0., -dy))
            }
        }
    }
}

impl ToSvg for PathSegment {
    fn to_svg(&self) -> String {
        match *self {
            Self::Move(p) => format!("M{} {}", Trim(p.x), Trim(p.y)),
            Self::Line(d) => format!("l{} {}", Trim(d.w), Trim(d.h)),
            Self::CubicBezier(d1, d2, d) => format!(
                "c{} {} {} {} {} {}",
                Trim(d1.w),
                Trim(d1.h),
                Trim(d2.w),
                Trim(d2.h),
                Trim(d.w),
                Trim(d.h)
            ),
            Self::QuadraticBezier(d1, d) => {
                format!("q{} {} {} {}", Trim(d1.w), Trim(d1.h), Trim(d.w), Trim(d.h))
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
    use std::f32::consts::{FRAC_1_SQRT_2, SQRT_2};

    use assert_approx_eq::assert_approx_eq;
    use assert_matches::assert_matches;
    use maplit::hashmap;

    use crate::utils::Point;

    use super::*;

    #[test]
    fn test_radius() {
        assert_approx_eq!(Path::radius(1. - FRAC_1_SQRT_2, SQRT_2), Size::new(1., 1.));
    }

    #[test]
    fn test_corners() {
        let rect = RoundRect::new(200., 100., 600., 600., 50., 50.);
        let path = Path::start(rect);

        let corner_funcs: Vec<fn(Path, RoundRect) -> Path> = vec![
            Path::corner_top_left,
            Path::corner_top_right,
            Path::corner_bottom_right,
            Path::corner_bottom_left,
        ];

        for func in corner_funcs {
            let path = func(path.clone(), rect);
            assert_eq!(path.data.len(), 2);
            assert_matches!(path.data[1], PathSegment::CubicBezier(..));
        }
    }

    #[test]
    fn test_edges() {
        let rect = RoundRect::new(200., 100., 600., 600., 50., 50.);
        let size = Size::new(2e3, 2e3);
        let curve = 20.;
        let path = Path::start(rect);

        let edge_funcs: Vec<fn(Path, RoundRect, Size, EdgeType, f32) -> Path> = vec![
            Path::edge_top,
            Path::edge_right,
            Path::edge_bottom,
            Path::edge_left,
        ];
        let edge_type_len = hashmap! {
            EdgeType::Line => 1,
            EdgeType::CurveStretch => 1,
            EdgeType::CurveLineCurve => 3,
            EdgeType::InsetCurve => 1,
        };

        for func in edge_funcs {
            for (&edge_type, &len) in &edge_type_len {
                let path = func(path.clone(), rect, size, edge_type, curve);

                assert_eq!(path.data.len(), len + 1);
            }
        }
    }

    #[test]
    fn test_path_to_svg() {
        let path = Path::new()
            .abs_move(Point::new(0., 0.))
            .rel_line(Size::new(1., 1.))
            .rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.))
            .rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.))
            .close();

        assert_eq!(path.to_svg(), "M0 0l1 1c0.5 0.5 1.5 0.5 2 0q0.5 -0.5 1 0z");
    }
}
