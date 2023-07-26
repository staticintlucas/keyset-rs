mod arc_to_bezier;
mod segment;

use arc_to_bezier::arc_to_bezier;
pub use segment::PathSegment;

use super::{Rect, Vec2};

#[derive(Debug, Clone)]
pub struct Path {
    pub data: Vec<PathSegment>,
    start: Vec2,
    point: Vec2,
    pub bounds: Rect,
}

impl Path {
    pub fn new() -> Self {
        Self {
            data: vec![],
            start: Vec2::ZERO,
            point: Vec2::ZERO,
            bounds: Rect::new(Vec2::ZERO, Vec2::ZERO),
        }
    }

    pub fn append(&mut self, other: Self) {
        if other.data.is_empty() {
            // Do nothing
        } else if self.data.is_empty() {
            *self = other;
        } else {
            // Add leading move to 0,0 if we don't already start with a move
            if !matches!(other.data[0], PathSegment::Move(..)) {
                self.data.push(PathSegment::Move(Vec2::ZERO));
            }
            self.data.extend(other.data);

            let sp = (
                self.bounds.position(),
                self.bounds.position() + self.bounds.size(),
            );
            let op = (
                other.bounds.position(),
                other.bounds.position() + other.bounds.size(),
            );
            self.bounds = Rect::from_points(Vec2::min(sp.0, op.0), Vec2::max(sp.1, op.1));

            self.start = other.start;
            self.point = other.point;
        }
    }

    pub fn add(mut self, other: Self) -> Self {
        self.append(other);
        self
    }

    pub fn rel_move(&mut self, d: Vec2) {
        let point = self.point;
        self.abs_move(point + d);
    }

    pub fn rel_line(&mut self, d: Vec2) {
        self.data.push(PathSegment::Line(d));
        self.point += d;
        self.bounds = Self::update_bounds(self.bounds, self.point);
    }

    pub fn rel_horiz_line(&mut self, dx: f64) {
        let point = self.point;
        self.abs_horiz_line(point.x + dx);
    }

    pub fn rel_vert_line(&mut self, dy: f64) {
        let point = self.point;
        self.abs_vert_line(point.y + dy);
    }

    pub fn rel_cubic_bezier(&mut self, d1: Vec2, d2: Vec2, d: Vec2) {
        self.data.push(PathSegment::CubicBezier(d1, d2, d));
        self.point += d;
        self.bounds = Self::update_bounds(self.bounds, self.point);
    }

    pub fn rel_smooth_cubic_bezier(&mut self, d2: Vec2, d: Vec2) {
        let d1 = match self.data.last() {
            Some(&PathSegment::CubicBezier(_, prev_d2, prev_d)) => prev_d - prev_d2,
            _ => Vec2::ZERO,
        };
        self.rel_cubic_bezier(d1, d2, d);
    }

    pub fn rel_quadratic_bezier(&mut self, d1: Vec2, d: Vec2) {
        self.data.push(PathSegment::QuadraticBezier(d1, d));
        self.point += d;
        self.bounds = Self::update_bounds(self.bounds, self.point);
    }

    pub fn rel_smooth_quadratic_bezier(&mut self, d: Vec2) {
        let d1 = match self.data.last() {
            Some(&PathSegment::QuadraticBezier(prev_d2, prev_d)) => prev_d - prev_d2,
            _ => Vec2::ZERO,
        };
        self.rel_quadratic_bezier(d1, d);
    }

    pub fn rel_arc(&mut self, r: Vec2, xar: f64, laf: bool, sf: bool, d: Vec2) {
        for (d1, d2, d) in arc_to_bezier(r, xar, laf, sf, d) {
            self.data.push(PathSegment::CubicBezier(d1, d2, d));
            self.point += d;
            self.bounds = Self::update_bounds(self.bounds, self.point);
        }
    }

    pub fn close(&mut self) {
        self.data.push(PathSegment::Close);
        self.point = self.start;
    }

    pub fn abs_move(&mut self, p: Vec2) {
        self.bounds = if self.data.is_empty() {
            Rect::from_points(p, p)
        } else {
            Self::update_bounds(self.bounds, p)
        };
        self.data.push(PathSegment::Move(p));
        self.start = p;
        self.point = p;
    }

    pub fn abs_line(&mut self, p: Vec2) {
        let point = self.point;
        self.rel_line(p - point);
    }

    pub fn abs_horiz_line(&mut self, x: f64) {
        let point = Vec2::new(x, self.point.y);
        self.abs_line(point);
    }

    pub fn abs_vert_line(&mut self, y: f64) {
        let point = Vec2::new(self.point.x, y);
        self.abs_line(point);
    }

    pub fn abs_cubic_bezier(&mut self, p1: Vec2, p2: Vec2, p: Vec2) {
        let point = self.point;
        self.rel_cubic_bezier(p1 - point, p2 - point, p - point);
    }

    pub fn abs_smooth_cubic_bezier(&mut self, p2: Vec2, p: Vec2) {
        let point = self.point;
        self.rel_smooth_cubic_bezier(p2 - point, p - point);
    }

    pub fn abs_quadratic_bezier(&mut self, p1: Vec2, p: Vec2) {
        let point = self.point;
        self.rel_quadratic_bezier(p1 - point, p - point);
    }

    pub fn abs_smooth_quadratic_bezier(&mut self, p: Vec2) {
        let point = self.point;
        self.rel_smooth_quadratic_bezier(p - point);
    }

    pub fn abs_arc(&mut self, r: Vec2, xar: f64, laf: bool, sf: bool, p: Vec2) {
        let point = self.point;
        self.rel_arc(r, xar, laf, sf, p - point);
    }

    pub fn scale(&mut self, scale: Vec2) {
        self.data.iter_mut().for_each(|s| s.scale(scale));
        self.start *= scale;
        self.point *= scale;
        self.bounds *= scale;
    }

    pub fn translate(&mut self, dist: Vec2) {
        self.data.iter_mut().for_each(|s| s.translate(dist));
        self.start += dist;
        self.point += dist;
        self.bounds = Rect::new(self.bounds.position() + dist, self.bounds.size());
    }

    pub fn rotate(&mut self, angle: f64) {
        self.data.iter_mut().for_each(|s| s.rotate(angle));
        self.start = self.start.rotate(angle);
        self.point = self.point.rotate(angle);
        self.bounds = Self::recalculate_bounds(&self.data);
    }

    pub fn skew_x(&mut self, angle: f64) {
        let tan = angle.tan();
        self.data.iter_mut().for_each(|s| s.skew_x(angle));
        self.start += Vec2::new(-self.start.y * tan, 0.);
        self.point += Vec2::new(-self.point.y * tan, 0.);
        self.bounds = Self::recalculate_bounds(&self.data);
    }

    pub fn skew_y(&mut self, angle: f64) {
        let tan = angle.tan();
        self.data.iter_mut().for_each(|s| s.skew_y(angle));
        self.start += Vec2::new(0., self.start.x * tan);
        self.point += Vec2::new(0., self.point.x * tan);
        self.bounds = Self::recalculate_bounds(&self.data);
    }

    fn recalculate_bounds(data: &Vec<PathSegment>) -> Rect {
        use PathSegment::{Close, CubicBezier, Line, Move, QuadraticBezier};

        if data.is_empty() {
            Rect::new(Vec2::ZERO, Vec2::ZERO)
        } else {
            let mov = if let Move(_) = data[0] {
                None
            } else {
                // Add leading move to 0,0 if we don't already start with a move
                Some(Move(Vec2::ZERO))
            };
            // We need a reference here since we don't want to consume self.data
            let (min, max) = mov
                .iter()
                .chain(data.iter())
                .filter(|seg| !matches!(seg, Close))
                .scan(Vec2::ZERO, |point, seg| {
                    *point = match *seg {
                        Move(p) => p,
                        Line(d) | CubicBezier(_, _, d) | QuadraticBezier(_, d) => *point + d,
                        Close => unreachable!(),
                    };
                    Some(*point)
                })
                .fold(
                    (Vec2::from(f64::INFINITY), Vec2::from(f64::NEG_INFINITY)),
                    |(min, max), p| (Vec2::min(min, p), Vec2::max(max, p)),
                );

            Rect::from_points(min, max)
        }
    }

    fn update_bounds(bounds: Rect, p: Vec2) -> Rect {
        let (pt1, pt2) = (bounds.position(), bounds.position() + bounds.size());
        Rect::from_points(Vec2::min(pt1, p), Vec2::max(pt2, p))
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_path_new() {
        let paths = vec![Path::new(), Path::default()];

        for path in paths {
            assert!(path.data.is_empty());
            assert_approx_eq!(path.start, Vec2::ZERO);
            assert_approx_eq!(path.point, Vec2::ZERO);
            assert_approx_eq!(path.bounds, Rect::new(Vec2::ZERO, Vec2::ZERO));
        }
    }

    #[test]
    fn test_path_add() {
        let empty = Path::new();
        let mut line1 = Path::new();
        line1.abs_line(Vec2::new(1., 1.));

        let mut line2 = Path::new();
        line2.abs_line(Vec2::new(1., 0.));

        let mut line3 = Path::new();
        line3.abs_move(Vec2::new(0., 1.));
        line3.abs_line(Vec2::new(1., 0.));

        let mut angle = Path::new();
        angle.abs_line(Vec2::new(1., 1.));
        angle.abs_move(Vec2::ZERO);
        angle.abs_line(Vec2::new(1., 0.));

        let mut cross = Path::new();
        cross.abs_line(Vec2::new(1., 1.));
        cross.abs_move(Vec2::new(0., 1.));
        cross.abs_line(Vec2::new(1., 0.));

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
        let mut r#move = Path::new();
        r#move.abs_move(Vec2::ZERO);
        r#move.rel_move(Vec2::new(2., 2.));
        r#move.close();

        let mut line = Path::new();
        line.abs_line(Vec2::new(1., 1.));
        line.rel_line(Vec2::new(1., 1.));
        line.close();

        let mut vert_horiz = Path::new();
        vert_horiz.abs_vert_line(2.);
        vert_horiz.rel_horiz_line(2.);
        vert_horiz.close();

        let mut horiz_vert = Path::new();
        horiz_vert.abs_horiz_line(2.);
        horiz_vert.rel_vert_line(2.);
        horiz_vert.close();

        let mut curve1 = Path::new();
        curve1.abs_cubic_bezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.), Vec2::new(1., 1.));
        curve1.rel_smooth_quadratic_bezier(Vec2::new(1., 1.));

        let mut curve2 = Path::new();
        curve2.abs_quadratic_bezier(Vec2::new(0., 1.), Vec2::new(1., 1.));
        curve2.rel_smooth_cubic_bezier(Vec2::new(1., 0.5), Vec2::new(1., 1.));

        let mut curve3 = Path::new();
        curve3.rel_cubic_bezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.), Vec2::new(1., 1.));
        curve3.abs_smooth_cubic_bezier(Vec2::new(2., 1.5), Vec2::new(2., 2.));
        curve3.close();

        let mut curve4 = Path::new();
        curve4.rel_quadratic_bezier(Vec2::new(0., 1.), Vec2::new(1., 1.));
        curve4.abs_smooth_quadratic_bezier(Vec2::new(2., 2.));
        curve4.close();

        let mut curve5 = Path::new();
        curve5.abs_smooth_cubic_bezier(Vec2::new(0., 2.), Vec2::new(2., 2.));

        let mut curve6 = Path::new();
        curve6.abs_smooth_quadratic_bezier(Vec2::new(2., 2.));

        let mut arc = Path::new();
        arc.abs_arc(Vec2::new(1., 1.), 0., false, false, Vec2::new(1., 1.));
        arc.rel_arc(Vec2::new(1., 1.), 0., false, true, Vec2::new(1., 1.));

        let params = vec![
            r#move, line, vert_horiz, horiz_vert, curve1, curve2, curve3, curve4, curve5, curve6,
            arc,
        ];

        for path in params {
            assert_approx_eq!(path.bounds, Rect::new(Vec2::ZERO, Vec2::new(2., 2.)));
        }
    }

    #[test]
    fn test_path_scale() {
        let mut path = Path::new();
        path.abs_move(Vec2::ZERO);
        path.rel_line(Vec2::new(1., 1.));
        path.rel_cubic_bezier(Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5), Vec2::new(2., 0.));
        path.rel_quadratic_bezier(Vec2::new(0.5, -0.5), Vec2::new(1., 0.));
        path.close();

        let mut scale1 = path.clone();
        scale1.scale(Vec2::new(0.5, 0.5));

        let mut scale2 = path.clone();
        scale2.scale(Vec2::new(-1., -2.));

        assert_approx_eq!(path.bounds, Rect::new(Vec2::ZERO, Vec2::new(4., 1.)));
        assert_approx_eq!(scale1.bounds, Rect::new(Vec2::ZERO, Vec2::new(2., 0.5)));
        assert_approx_eq!(
            scale2.bounds,
            Rect::new(Vec2::new(-4., -2.), Vec2::new(4., 2.))
        );
    }

    #[test]
    fn test_translate() {
        let mut path = Path::new();
        path.abs_move(Vec2::ZERO);
        path.rel_line(Vec2::new(1., 1.));
        path.rel_cubic_bezier(Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5), Vec2::new(2., 0.));
        path.rel_quadratic_bezier(Vec2::new(0.5, -0.5), Vec2::new(1., 0.));
        path.close();

        let mut translate = path.clone();
        translate.translate(Vec2::new(2., 1.));

        assert_approx_eq!(path.bounds, Rect::new(Vec2::ZERO, Vec2::new(4., 1.)));
        assert_approx_eq!(
            translate.bounds,
            Rect::new(Vec2::new(2., 1.), Vec2::new(4., 1.))
        );
    }

    #[test]
    fn test_rotate() {
        let mut path = Path::new();
        path.abs_move(Vec2::ZERO);
        path.rel_line(Vec2::new(1., 1.));
        path.rel_cubic_bezier(Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5), Vec2::new(2., 0.));
        path.rel_quadratic_bezier(Vec2::new(0.5, -0.5), Vec2::new(1., 0.));
        path.close();

        let mut rotate = path.clone();
        rotate.rotate(FRAC_PI_2);

        assert_approx_eq!(path.bounds, Rect::new(Vec2::ZERO, Vec2::new(4., 1.)));
        assert_approx_eq!(
            rotate.bounds,
            Rect::new(Vec2::new(-1., 0.), Vec2::new(1., 4.))
        );
    }

    #[test]
    fn test_skew_x() {
        let mut path = Path::new();
        path.abs_move(Vec2::ZERO);
        path.rel_line(Vec2::new(1., 1.));
        path.rel_cubic_bezier(Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5), Vec2::new(2., 0.));
        path.rel_quadratic_bezier(Vec2::new(0.5, -0.5), Vec2::new(1., 0.));
        path.close();

        let mut skew = path.clone();
        skew.skew_x(-FRAC_PI_4);

        assert_approx_eq!(path.bounds, Rect::new(Vec2::ZERO, Vec2::new(4., 1.)));
        assert_approx_eq!(skew.bounds, Rect::new(Vec2::ZERO, Vec2::new(5., 1.)));
    }

    #[test]
    fn test_skew_y() {
        let mut path = Path::new();
        path.abs_move(Vec2::ZERO);
        path.rel_line(Vec2::new(1., 1.));
        path.rel_cubic_bezier(Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5), Vec2::new(2., 0.));
        path.rel_quadratic_bezier(Vec2::new(0.5, -0.5), Vec2::new(1., 0.));
        path.close();

        let mut skew = path.clone();
        skew.skew_y(FRAC_PI_4);

        assert_approx_eq!(path.bounds, Rect::new(Vec2::ZERO, Vec2::new(4., 1.)));
        assert_approx_eq!(skew.bounds, Rect::new(Vec2::ZERO, Vec2::new(4., 5.)));
    }

    #[test]
    fn test_recalculate_bounds() {
        let mut path1 = Path::new();
        path1.abs_line(Vec2::new(1., 1.));
        path1.close();

        let mut path2 = Path::new();
        path2.abs_move(Vec2::ZERO);
        path2.abs_line(Vec2::new(1., -1.));

        let mut path3 = Path::new();
        path3.abs_cubic_bezier(Vec2::new(0., 0.5), Vec2::new(0.5, 1.), Vec2::new(1., 1.));
        path3.abs_quadratic_bezier(Vec2::new(2., 1.), Vec2::new(2., 2.));

        let params = vec![
            (Path::new(), Rect::new(Vec2::ZERO, Vec2::ZERO)),
            (path1.clone(), Rect::new(Vec2::ZERO, Vec2::new(1., 1.))),
            (
                path2.clone(),
                Rect::new(Vec2::new(0., -1.), Vec2::new(1., 1.)),
            ),
            (path3.clone(), Rect::new(Vec2::ZERO, Vec2::new(2., 2.))),
            (
                path1.add(path2),
                Rect::new(Vec2::new(0., -1.), Vec2::new(1., 2.)),
            ),
        ];

        for (path, expected) in params {
            let bounds = Path::recalculate_bounds(&path.data);
            assert_eq!(bounds, expected);
        }
    }
}
