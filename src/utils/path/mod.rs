mod arc_to_bezier;
mod segment;

use arc_to_bezier::arc_to_bezier;
use segment::PathSegment;

use super::{Point, Rect, Scale, Size};

#[derive(Debug, Clone)]
struct Path {
    data: Vec<PathSegment>,
    start: Point,
    point: Point,
    bounds: Rect,
}

impl Path {
    pub fn new() -> Self {
        Self {
            data: vec![],
            start: Point::new(0., 0.),
            point: Point::new(0., 0.),
            bounds: Rect::new(0., 0., 0., 0.),
        }
    }

    pub fn append(&mut self, other: Self) {
        if other.data.is_empty() {
            // Do nothing
        } else if self.data.is_empty() {
            *self = other;
        } else {
            let mov = if let PathSegment::Move(_) = other.data[0] {
                None
            } else {
                // Add leading move to 0,0 if we don't already start with a move
                Some(PathSegment::Move(Point::new(0., 0.)))
            };
            self.data.extend(mov.into_iter().chain(other.data));

            let sp = (
                self.bounds.position(),
                self.bounds.position() + self.bounds.size(),
            );
            let op = (
                other.bounds.position(),
                other.bounds.position() + other.bounds.size(),
            );
            self.bounds = Rect::from_points(
                Point::new(f32::min(sp.0.x, op.0.x), f32::min(sp.0.y, op.0.y)),
                Point::new(f32::max(sp.1.x, op.1.x), f32::max(sp.1.y, op.1.y)),
            );

            self.start = other.start;
            self.point = other.point;
        }
    }

    pub fn add(mut self, other: Self) -> Self {
        self.append(other);
        self
    }

    pub fn rel_move(self, d: Size) -> Self {
        let point = self.point;
        self.abs_move(point + d)
    }

    pub fn rel_line(mut self, d: Size) -> Self {
        self.data.push(PathSegment::Line(d));
        self.point += d;
        self.bounds = Self::update_bounds(self.bounds, self.point);
        self
    }

    pub fn rel_horiz_line(self, dx: f32) -> Self {
        let point = self.point;
        self.abs_horiz_line(point.x + dx)
    }

    pub fn rel_vert_line(self, dy: f32) -> Self {
        let point = self.point;
        self.abs_vert_line(point.y + dy)
    }

    pub fn rel_cubic_bezier(mut self, d1: Size, d2: Size, d: Size) -> Self {
        self.data.push(PathSegment::CubicBezier(d1, d2, d));
        self.point += d;
        self.bounds = Self::update_bounds(self.bounds, self.point);
        self
    }

    pub fn rel_smooth_cubic_bezier(self, d2: Size, d: Size) -> Self {
        let d1 = match self.data.last() {
            Some(&PathSegment::CubicBezier(_, prev_d2, prev_d)) => prev_d - prev_d2,
            _ => Size::new(0., 0.),
        };
        self.rel_cubic_bezier(d1, d2, d)
    }

    pub fn rel_quadratic_bezier(mut self, d1: Size, d: Size) -> Self {
        self.data.push(PathSegment::QuadraticBezier(d1, d));
        self.point += d;
        self.bounds = Self::update_bounds(self.bounds, self.point);
        self
    }

    pub fn rel_smooth_quadratic_bezier(self, d: Size) -> Self {
        let d1 = match self.data.last() {
            Some(&PathSegment::QuadraticBezier(prev_d2, prev_d)) => prev_d - prev_d2,
            _ => Size::new(0., 0.),
        };
        self.rel_quadratic_bezier(d1, d)
    }

    pub fn rel_arc(mut self, r: Size, xar: f32, laf: bool, sf: bool, d: Size) -> Self {
        for (d1, d2, d) in arc_to_bezier(r, xar, laf, sf, d) {
            self.data.push(PathSegment::CubicBezier(d1, d2, d));
            self.point += d;
            self.bounds = Self::update_bounds(self.bounds, self.point);
        }
        self
    }

    pub fn close(mut self) -> Self {
        self.data.push(PathSegment::Close);
        self.point = self.start;
        self
    }

    pub fn abs_move(mut self, p: Point) -> Self {
        self.bounds = if self.data.is_empty() {
            Rect::from_points(p, p)
        } else {
            Self::update_bounds(self.bounds, p)
        };
        self.data.push(PathSegment::Move(p));
        self.start = p;
        self.point = p;
        self
    }

    pub fn abs_line(self, p: Point) -> Self {
        let point = self.point;
        self.rel_line(p - point)
    }

    pub fn abs_horiz_line(self, x: f32) -> Self {
        let point = Point::new(x, self.point.y);
        self.abs_line(point)
    }

    pub fn abs_vert_line(self, y: f32) -> Self {
        let point = Point::new(self.point.x, y);
        self.abs_line(point)
    }

    pub fn abs_cubic_bezier(self, p1: Point, p2: Point, p: Point) -> Self {
        let point = self.point;
        self.rel_cubic_bezier(p1 - point, p2 - point, p - point)
    }

    pub fn abs_smooth_cubic_bezier(self, p2: Point, p: Point) -> Self {
        let point = self.point;
        self.rel_smooth_cubic_bezier(p2 - point, p - point)
    }

    pub fn abs_quadratic_bezier(self, p1: Point, p: Point) -> Self {
        let point = self.point;
        self.rel_quadratic_bezier(p1 - point, p - point)
    }

    pub fn abs_smooth_quadratic_bezier(self, p: Point) -> Self {
        let point = self.point;
        self.rel_smooth_quadratic_bezier(p - point)
    }

    pub fn abs_arc(self, r: Size, xar: f32, laf: bool, sf: bool, p: Point) -> Self {
        let point = self.point;
        self.rel_arc(r, xar, laf, sf, p - point)
    }

    pub fn scale(self, scale: Scale) -> Self {
        Self {
            data: self.data.into_iter().map(|s| s.scale(scale)).collect(),
            start: self.start * scale,
            point: self.point * scale,
            bounds: self.bounds * scale,
        }
    }

    pub fn translate(self, dist: Size) -> Self {
        Self {
            data: self.data.into_iter().map(|s| s.translate(dist)).collect(),
            start: self.start + dist,
            point: self.point + dist,
            bounds: Rect::from_point_and_size(self.bounds.position() + dist, self.bounds.size()),
        }
    }

    pub fn rotate(self, angle: f32) -> Self {
        let data = self.data.into_iter().map(|s| s.rotate(angle)).collect();
        let bounds = Self::recalculate_bounds(&data);
        Self {
            data,
            start: self.start.rotate(angle),
            point: self.point.rotate(angle),
            bounds,
        }
    }

    pub fn skew_x(self, angle: f32) -> Self {
        let tan = angle.tan();
        let data = self.data.into_iter().map(|s| s.skew_x(angle)).collect();
        let bounds = Self::recalculate_bounds(&data);
        Self {
            data,
            start: self.start + Size::new(-self.start.y * tan, 0.),
            point: self.point + Size::new(-self.point.y * tan, 0.),
            bounds,
        }
    }

    pub fn skew_y(self, angle: f32) -> Self {
        let tan = angle.tan();
        let data = self.data.into_iter().map(|s| s.skew_y(angle)).collect();
        let bounds = Self::recalculate_bounds(&data);
        Self {
            data,
            start: self.start + Size::new(0., self.start.x * tan),
            point: self.point + Size::new(0., self.point.x * tan),
            bounds,
        }
    }

    fn recalculate_bounds(data: &Vec<PathSegment>) -> Rect {
        use PathSegment::{Close, CubicBezier, Line, Move, QuadraticBezier};

        if data.is_empty() {
            Rect::new(0., 0., 0., 0.)
        } else {
            let mov = if let Move(_) = data[0] {
                None
            } else {
                // Add leading move to 0,0 if we don't already start with a move
                Some(Move(Point::new(0., 0.)))
            };
            // We need a reference here since we don't want to consume self.data
            let (min, max) = mov
                .iter()
                .chain(data.iter())
                .filter(|seg| !matches!(seg, Close))
                .scan(Point::new(0., 0.), |point, seg| {
                    *point = match *seg {
                        Move(p) => p,
                        Line(d) | CubicBezier(_, _, d) | QuadraticBezier(_, d) => *point + d,
                        Close => unreachable!(),
                    };
                    Some(*point)
                })
                .fold(
                    (
                        Point::new(f32::INFINITY, f32::INFINITY),
                        Point::new(f32::NEG_INFINITY, f32::NEG_INFINITY),
                    ),
                    |(min, max), p| {
                        (
                            Point::new(f32::min(min.x, p.x), f32::min(min.y, p.y)),
                            Point::new(f32::max(max.x, p.x), f32::max(max.y, p.y)),
                        )
                    },
                );

            Rect::from_points(min, max)
        }
    }

    fn update_bounds(bounds: Rect, p: Point) -> Rect {
        let (pt1, pt2) = (bounds.position(), bounds.position() + bounds.size());
        Rect::from_points(
            Point::new(f32::min(pt1.x, p.x), f32::min(pt1.y, p.y)),
            Point::new(f32::max(pt2.x, p.x), f32::max(pt2.y, p.y)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_path_new() {
        let path = Path::new();

        assert!(path.data.is_empty());
        assert_approx_eq!(path.start, Point::new(0., 0.));
        assert_approx_eq!(path.point, Point::new(0., 0.));
        assert_approx_eq!(path.bounds, Rect::new(0., 0., 0., 0.));
    }

    #[test]
    fn test_path_add() {
        let empty = Path::new();
        let line1 = Path::new().abs_line(Point::new(1., 1.));
        let line2 = Path::new().abs_line(Point::new(1., 0.));
        let line3 = Path::new()
            .abs_move(Point::new(0., 1.))
            .abs_line(Point::new(1., 0.));
        let angle = Path::new()
            .abs_line(Point::new(1., 1.))
            .abs_move(Point::new(0., 0.))
            .abs_line(Point::new(1., 0.));
        let cross = Path::new()
            .abs_line(Point::new(1., 1.))
            .abs_move(Point::new(0., 1.))
            .abs_line(Point::new(1., 0.));
        let params = vec![
            (empty.clone(), empty.clone(), empty.clone()),
            (line1.clone(), empty.clone(), line1.clone()),
            (empty.clone(), line1.clone(), line1.clone()),
            (line1.clone(), line2.clone(), angle.clone()),
            (line1.clone(), line3.clone(), cross.clone()),
        ];

        for (first, second, expected) in params {
            let result = first.add(second);

            assert_eq!(result.data.len(), expected.data.len());
            assert_approx_eq!(result.start, expected.start);
            assert_approx_eq!(result.point, expected.point);
            assert_approx_eq!(result.bounds, expected.bounds);
        }
    }

    #[test]
    fn test_commands() {
        let params = vec![
            Path::new()
                .abs_move(Point::new(0., 0.))
                .rel_move(Size::new(2., 2.))
                .close(),
            Path::new()
                .abs_line(Point::new(1., 1.))
                .rel_line(Size::new(1., 1.))
                .close(),
            Path::new().abs_vert_line(2.).rel_horiz_line(2.).close(),
            Path::new().abs_horiz_line(2.).rel_vert_line(2.).close(),
            Path::new()
                .abs_cubic_bezier(Point::new(0., 0.5), Point::new(0.5, 1.), Point::new(1., 1.))
                .rel_smooth_quadratic_bezier(Size::new(1., 1.)),
            Path::new()
                .abs_quadratic_bezier(Point::new(0., 1.), Point::new(1., 1.))
                .rel_smooth_cubic_bezier(Size::new(1., 0.5), Size::new(1., 1.)),
            Path::new()
                .rel_cubic_bezier(Size::new(0., 0.5), Size::new(0.5, 1.), Size::new(1., 1.))
                .abs_smooth_cubic_bezier(Point::new(2., 1.5), Point::new(2., 2.))
                .close(),
            Path::new()
                .rel_quadratic_bezier(Size::new(0., 1.), Size::new(1., 1.))
                .abs_smooth_quadratic_bezier(Point::new(2., 2.))
                .close(),
            Path::new().abs_smooth_cubic_bezier(Point::new(0., 2.), Point::new(2., 2.)),
            Path::new().abs_smooth_quadratic_bezier(Point::new(2., 2.)),
            Path::new()
                .abs_arc(Size::new(1., 1.), 0., false, false, Point::new(1., 1.))
                .rel_arc(Size::new(1., 1.), 0., false, true, Size::new(1., 1.)),
        ];

        for path in params {
            assert_approx_eq!(path.bounds, Rect::new(0., 0., 2., 2.));
        }
    }

    #[test]
    fn test_path_scale() {
        let path = Path::new()
            .abs_move(Point::new(0., 0.))
            .rel_line(Size::new(1., 1.))
            .rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.))
            .rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.))
            .close();

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(
            path.clone().scale(Scale::new(0.5, 0.5)).bounds,
            Rect::new(0., 0., 2., 0.5)
        );
        assert_approx_eq!(
            path.scale(Scale::new(-1., -2.)).bounds,
            Rect::new(-4., -2., 4., 2.)
        );
    }

    #[test]
    fn test_translate() {
        let path = Path::new()
            .abs_move(Point::new(0., 0.))
            .rel_line(Size::new(1., 1.))
            .rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.))
            .rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.))
            .close();

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(
            path.translate(Size::new(2., 1.)).bounds,
            Rect::new(2., 1., 4., 1.)
        );
    }

    #[test]
    fn test_rotate() {
        let path = Path::new()
            .abs_move(Point::new(0., 0.))
            .rel_line(Size::new(1., 1.))
            .rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.))
            .rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.))
            .close();

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(path.rotate(FRAC_PI_2).bounds, Rect::new(-1., 0., 1., 4.));
    }

    #[test]
    fn test_skew_x() {
        let path = Path::new()
            .abs_move(Point::new(0., 0.))
            .rel_line(Size::new(1., 1.))
            .rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.))
            .rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.))
            .close();

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(path.skew_x(-FRAC_PI_4).bounds, Rect::new(0., 0., 5., 1.));
    }

    #[test]
    fn test_skew_y() {
        let path = Path::new()
            .abs_move(Point::new(0., 0.))
            .rel_line(Size::new(1., 1.))
            .rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.))
            .rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.))
            .close();

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(path.skew_y(FRAC_PI_4).bounds, Rect::new(0., 0., 4., 5.));
    }

    #[test]
    fn test_recalculate_bounds() {
        let path1 = Path::new().abs_line(Point::new(1., 1.)).close();
        let path2 = Path::new()
            .abs_move(Point::new(0., 0.))
            .abs_line(Point::new(1., -1.));
        let path3 = Path::new()
            .abs_cubic_bezier(Point::new(0., 0.5), Point::new(0.5, 1.), Point::new(1., 1.))
            .abs_quadratic_bezier(Point::new(2., 1.), Point::new(2., 2.));

        let params = vec![
            (Path::new(), Rect::new(0., 0., 0., 0.)),
            (path1.clone(), Rect::new(0., 0., 1., 1.)),
            (path2.clone(), Rect::new(0., -1., 1., 1.)),
            (path3.clone(), Rect::new(0., 0., 2., 2.)),
            (path1.add(path2), Rect::new(0., -1., 1., 2.)),
        ];

        for (path, expected) in params {
            let bounds = Path::recalculate_bounds(&path.data);
            assert_eq!(bounds, expected);
        }
    }
}
