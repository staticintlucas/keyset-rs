mod arc_to_bezier;
mod segment;

use arc_to_bezier::arc_to_bezier;
pub use segment::PathSegment;

use super::{Point, Rect, Scale, Size};

#[derive(Debug, Clone)]
pub struct Path {
    pub data: Vec<PathSegment>,
    start: Point,
    point: Point,
    pub bounds: Rect,
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

    pub fn rel_move(&mut self, d: Size) {
        let point = self.point;
        self.abs_move(point + d);
    }

    pub fn rel_line(&mut self, d: Size) {
        self.data.push(PathSegment::Line(d));
        self.point += d;
        self.bounds = Self::update_bounds(self.bounds, self.point);
    }

    pub fn rel_horiz_line(&mut self, dx: f32) {
        let point = self.point;
        self.abs_horiz_line(point.x + dx);
    }

    pub fn rel_vert_line(&mut self, dy: f32) {
        let point = self.point;
        self.abs_vert_line(point.y + dy);
    }

    pub fn rel_cubic_bezier(&mut self, d1: Size, d2: Size, d: Size) {
        self.data.push(PathSegment::CubicBezier(d1, d2, d));
        self.point += d;
        self.bounds = Self::update_bounds(self.bounds, self.point);
    }

    pub fn rel_smooth_cubic_bezier(&mut self, d2: Size, d: Size) {
        let d1 = match self.data.last() {
            Some(&PathSegment::CubicBezier(_, prev_d2, prev_d)) => prev_d - prev_d2,
            _ => Size::new(0., 0.),
        };
        self.rel_cubic_bezier(d1, d2, d);
    }

    pub fn rel_quadratic_bezier(&mut self, d1: Size, d: Size) {
        self.data.push(PathSegment::QuadraticBezier(d1, d));
        self.point += d;
        self.bounds = Self::update_bounds(self.bounds, self.point);
    }

    pub fn rel_smooth_quadratic_bezier(&mut self, d: Size) {
        let d1 = match self.data.last() {
            Some(&PathSegment::QuadraticBezier(prev_d2, prev_d)) => prev_d - prev_d2,
            _ => Size::new(0., 0.),
        };
        self.rel_quadratic_bezier(d1, d);
    }

    pub fn rel_arc(&mut self, r: Size, xar: f32, laf: bool, sf: bool, d: Size) {
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

    pub fn abs_move(&mut self, p: Point) {
        self.bounds = if self.data.is_empty() {
            Rect::from_points(p, p)
        } else {
            Self::update_bounds(self.bounds, p)
        };
        self.data.push(PathSegment::Move(p));
        self.start = p;
        self.point = p;
    }

    pub fn abs_line(&mut self, p: Point) {
        let point = self.point;
        self.rel_line(p - point);
    }

    pub fn abs_horiz_line(&mut self, x: f32) {
        let point = Point::new(x, self.point.y);
        self.abs_line(point);
    }

    pub fn abs_vert_line(&mut self, y: f32) {
        let point = Point::new(self.point.x, y);
        self.abs_line(point);
    }

    pub fn abs_cubic_bezier(&mut self, p1: Point, p2: Point, p: Point) {
        let point = self.point;
        self.rel_cubic_bezier(p1 - point, p2 - point, p - point);
    }

    pub fn abs_smooth_cubic_bezier(&mut self, p2: Point, p: Point) {
        let point = self.point;
        self.rel_smooth_cubic_bezier(p2 - point, p - point);
    }

    pub fn abs_quadratic_bezier(&mut self, p1: Point, p: Point) {
        let point = self.point;
        self.rel_quadratic_bezier(p1 - point, p - point);
    }

    pub fn abs_smooth_quadratic_bezier(&mut self, p: Point) {
        let point = self.point;
        self.rel_smooth_quadratic_bezier(p - point);
    }

    pub fn abs_arc(&mut self, r: Size, xar: f32, laf: bool, sf: bool, p: Point) {
        let point = self.point;
        self.rel_arc(r, xar, laf, sf, p - point);
    }

    pub fn scale(&mut self, scale: Scale) {
        self.data.iter_mut().for_each(|s| s.scale(scale));
        self.start *= scale;
        self.point *= scale;
        self.bounds *= scale;
    }

    pub fn translate(&mut self, dist: Size) {
        self.data.iter_mut().for_each(|s| s.translate(dist));
        self.start += dist;
        self.point += dist;
        self.bounds = Rect::from_point_and_size(self.bounds.position() + dist, self.bounds.size());
    }

    pub fn rotate(&mut self, angle: f32) {
        self.data.iter_mut().for_each(|s| s.rotate(angle));
        self.start = self.start.rotate(angle);
        self.point = self.point.rotate(angle);
        self.bounds = Self::recalculate_bounds(&self.data);
    }

    pub fn skew_x(&mut self, angle: f32) {
        let tan = angle.tan();
        self.data.iter_mut().for_each(|s| s.skew_x(angle));
        self.start += Size::new(-self.start.y * tan, 0.);
        self.point += Size::new(-self.point.y * tan, 0.);
        self.bounds = Self::recalculate_bounds(&self.data);
    }

    pub fn skew_y(&mut self, angle: f32) {
        let tan = angle.tan();
        self.data.iter_mut().for_each(|s| s.skew_y(angle));
        self.start += Size::new(0., self.start.x * tan);
        self.point += Size::new(0., self.point.x * tan);
        self.bounds = Self::recalculate_bounds(&self.data);
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

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_path_new() {
        let paths = vec![Path::new(), Path::default()];

        for path in paths {
            assert!(path.data.is_empty());
            assert_approx_eq!(path.start, Point::new(0., 0.));
            assert_approx_eq!(path.point, Point::new(0., 0.));
            assert_approx_eq!(path.bounds, Rect::new(0., 0., 0., 0.));
        }
    }

    #[test]
    fn test_path_add() {
        let empty = Path::new();
        let mut line1 = Path::new();
        line1.abs_line(Point::new(1., 1.));

        let mut line2 = Path::new();
        line2.abs_line(Point::new(1., 0.));

        let mut line3 = Path::new();
        line3.abs_move(Point::new(0., 1.));
        line3.abs_line(Point::new(1., 0.));

        let mut angle = Path::new();
        angle.abs_line(Point::new(1., 1.));
        angle.abs_move(Point::new(0., 0.));
        angle.abs_line(Point::new(1., 0.));

        let mut cross = Path::new();
        cross.abs_line(Point::new(1., 1.));
        cross.abs_move(Point::new(0., 1.));
        cross.abs_line(Point::new(1., 0.));

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
        r#move.abs_move(Point::new(0., 0.));
        r#move.rel_move(Size::new(2., 2.));
        r#move.close();

        let mut line = Path::new();
        line.abs_line(Point::new(1., 1.));
        line.rel_line(Size::new(1., 1.));
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
        curve1.abs_cubic_bezier(Point::new(0., 0.5), Point::new(0.5, 1.), Point::new(1., 1.));
        curve1.rel_smooth_quadratic_bezier(Size::new(1., 1.));

        let mut curve2 = Path::new();
        curve2.abs_quadratic_bezier(Point::new(0., 1.), Point::new(1., 1.));
        curve2.rel_smooth_cubic_bezier(Size::new(1., 0.5), Size::new(1., 1.));

        let mut curve3 = Path::new();
        curve3.rel_cubic_bezier(Size::new(0., 0.5), Size::new(0.5, 1.), Size::new(1., 1.));
        curve3.abs_smooth_cubic_bezier(Point::new(2., 1.5), Point::new(2., 2.));
        curve3.close();

        let mut curve4 = Path::new();
        curve4.rel_quadratic_bezier(Size::new(0., 1.), Size::new(1., 1.));
        curve4.abs_smooth_quadratic_bezier(Point::new(2., 2.));
        curve4.close();

        let mut curve5 = Path::new();
        curve5.abs_smooth_cubic_bezier(Point::new(0., 2.), Point::new(2., 2.));

        let mut curve6 = Path::new();
        curve6.abs_smooth_quadratic_bezier(Point::new(2., 2.));

        let mut arc = Path::new();
        arc.abs_arc(Size::new(1., 1.), 0., false, false, Point::new(1., 1.));
        arc.rel_arc(Size::new(1., 1.), 0., false, true, Size::new(1., 1.));

        let params = vec![
            r#move, line, vert_horiz, horiz_vert, curve1, curve2, curve3, curve4, curve5, curve6,
            arc,
        ];

        for path in params {
            assert_approx_eq!(path.bounds, Rect::new(0., 0., 2., 2.));
        }
    }

    #[test]
    fn test_path_scale() {
        let mut path = Path::new();
        path.abs_move(Point::new(0., 0.));
        path.rel_line(Size::new(1., 1.));
        path.rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.));
        path.rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.));
        path.close();

        let mut scale1 = path.clone();
        scale1.scale(Scale::new(0.5, 0.5));

        let mut scale2 = path.clone();
        scale2.scale(Scale::new(-1., -2.));

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(scale1.bounds, Rect::new(0., 0., 2., 0.5));
        assert_approx_eq!(scale2.bounds, Rect::new(-4., -2., 4., 2.));
    }

    #[test]
    fn test_translate() {
        let mut path = Path::new();
        path.abs_move(Point::new(0., 0.));
        path.rel_line(Size::new(1., 1.));
        path.rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.));
        path.rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.));
        path.close();

        let mut translate = path.clone();
        translate.translate(Size::new(2., 1.));

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(translate.bounds, Rect::new(2., 1., 4., 1.));
    }

    #[test]
    fn test_rotate() {
        let mut path = Path::new();
        path.abs_move(Point::new(0., 0.));
        path.rel_line(Size::new(1., 1.));
        path.rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.));
        path.rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.));
        path.close();

        let mut rotate = path.clone();
        rotate.rotate(FRAC_PI_2);

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(rotate.bounds, Rect::new(-1., 0., 1., 4.));
    }

    #[test]
    fn test_skew_x() {
        let mut path = Path::new();
        path.abs_move(Point::new(0., 0.));
        path.rel_line(Size::new(1., 1.));
        path.rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.));
        path.rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.));
        path.close();

        let mut skew = path.clone();
        skew.skew_x(-FRAC_PI_4);

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(skew.bounds, Rect::new(0., 0., 5., 1.));
    }

    #[test]
    fn test_skew_y() {
        let mut path = Path::new();
        path.abs_move(Point::new(0., 0.));
        path.rel_line(Size::new(1., 1.));
        path.rel_cubic_bezier(Size::new(0.5, 0.5), Size::new(1.5, 0.5), Size::new(2., 0.));
        path.rel_quadratic_bezier(Size::new(0.5, -0.5), Size::new(1., 0.));
        path.close();

        let mut skew = path.clone();
        skew.skew_y(FRAC_PI_4);

        assert_approx_eq!(path.bounds, Rect::new(0., 0., 4., 1.));
        assert_approx_eq!(skew.bounds, Rect::new(0., 0., 4., 5.));
    }

    #[test]
    fn test_recalculate_bounds() {
        let mut path1 = Path::new();
        path1.abs_line(Point::new(1., 1.));
        path1.close();

        let mut path2 = Path::new();
        path2.abs_move(Point::new(0., 0.));
        path2.abs_line(Point::new(1., -1.));

        let mut path3 = Path::new();
        path3.abs_cubic_bezier(Point::new(0., 0.5), Point::new(0.5, 1.), Point::new(1., 1.));
        path3.abs_quadratic_bezier(Point::new(2., 1.), Point::new(2., 2.));

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
