use std::f64::consts::{FRAC_PI_2, PI};
use std::ops::{Add, Sub};

use kurbo::{Arc, ArcAppendIter, Ellipse, PathEl, Point, Rect, Shape, Size, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct RoundRect {
    rect: Rect,
    radii: Vec2,
}

impl RoundRect {
    #[inline]
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64, rx: f64, ry: f64) -> Self {
        Self::from_rect(Rect::new(x0, y0, x1, y1), Vec2::new(rx, ry))
    }

    #[inline]
    pub fn from_rect(rect: Rect, radii: impl Into<Vec2>) -> Self {
        let rect = rect.abs();
        let radii = radii.into();
        let radii = Vec2::new(
            radii.x.min(rect.width() / 2.0),
            radii.y.min(rect.height() / 2.0),
        );

        Self { rect, radii }
    }

    #[inline]
    pub fn from_points(p0: impl Into<Point>, p1: impl Into<Point>, radii: impl Into<Vec2>) -> Self {
        Self::from_rect(Rect::from_points(p0, p1), radii)
    }

    #[inline]
    pub fn from_origin_size(
        origin: impl Into<Point>,
        size: impl Into<Size>,
        radii: impl Into<Vec2>,
    ) -> Self {
        Self::from_rect(Rect::from_origin_size(origin, size), radii)
    }

    #[inline]
    pub fn from_center_size(
        origin: impl Into<Point>,
        size: impl Into<Size>,
        radii: impl Into<Vec2>,
    ) -> Self {
        Self::from_rect(Rect::from_center_size(origin, size), radii)
    }

    #[inline]
    pub fn width(&self) -> f64 {
        self.rect.width()
    }

    #[inline]
    pub fn height(&self) -> f64 {
        self.rect.height()
    }

    #[inline]
    pub fn radii(&self) -> Vec2 {
        self.radii
    }

    #[inline]
    pub fn rect(&self) -> Rect {
        self.rect
    }

    #[inline]
    pub fn origin(&self) -> Point {
        self.rect.origin()
    }

    #[inline]
    pub fn center(&self) -> Point {
        self.rect.center()
    }

    #[inline]
    pub fn size(&self) -> Size {
        self.rect().size()
    }

    #[inline]
    pub fn with_origin(self, origin: impl Into<Point>) -> Self {
        Self::from_origin_size(origin, self.size(), self.radii())
    }

    #[inline]
    pub fn with_size(self, size: impl Into<Size>) -> Self {
        Self::from_origin_size(self.origin(), size, self.radii())
    }

    #[inline]
    pub fn with_radii(self, radii: impl Into<Vec2>) -> Self {
        Self::from_origin_size(self.origin(), self.size(), radii)
    }
}

#[doc(hidden)]
pub struct RoundRectPathIter {
    idx: usize,
    rect: RectPathIter,
    arcs: [ArcAppendIter; 4],
}

impl Shape for RoundRect {
    type PathElementsIter<'iter> = RoundRectPathIter;

    fn path_elements(&self, tolerance: f64) -> RoundRectPathIter {
        let radii = self.radii();

        let build_arc_iter = |i, center, ellipse_radii| {
            let arc = Arc {
                center,
                radii: ellipse_radii,
                start_angle: FRAC_PI_2 * f64::from(i),
                sweep_angle: FRAC_PI_2,
                x_rotation: 0.0,
            };
            arc.append_iter(tolerance)
        };

        // Note: order follows the rectangle path iterator.
        let arcs = [
            build_arc_iter(
                2,
                Point {
                    x: self.rect.x0 + radii.x,
                    y: self.rect.y0 + radii.y,
                },
                radii,
            ),
            build_arc_iter(
                3,
                Point {
                    x: self.rect.x1 - radii.x,
                    y: self.rect.y0 + radii.y,
                },
                radii,
            ),
            build_arc_iter(
                0,
                Point {
                    x: self.rect.x1 - radii.x,
                    y: self.rect.y1 - radii.y,
                },
                radii,
            ),
            build_arc_iter(
                1,
                Point {
                    x: self.rect.x0 + radii.x,
                    y: self.rect.y1 - radii.y,
                },
                radii,
            ),
        ];

        let rect = RectPathIter {
            rect: self.rect,
            ix: 0,
            radii,
        };

        RoundRectPathIter { idx: 0, rect, arcs }
    }

    #[inline]
    fn area(&self) -> f64 {
        self.rect.area() - (4.0 - PI) * self.radii().x * self.radii().y
    }

    #[inline]
    fn perimeter(&self, accuracy: f64) -> f64 {
        self.rect.perimeter(accuracy) - 4.0 * (self.radii.x + self.radii.y)
            + Ellipse::new(Point::ORIGIN, self.radii, 0.0).perimeter(accuracy)
    }

    #[inline]
    fn winding(&self, mut pt: Point) -> i32 {
        let center = self.center();
        let Vec2 { x: rx, y: ry } = self.radii;

        if rx <= 0.0 || ry <= 0.0 {
            return self.bounding_box().winding(pt);
        }

        // 1. Translate the point relative to the center of the rectangle.
        pt.x -= center.x;
        pt.y -= center.y;

        // 2. This is the width and height of a rectangle with one corner at
        //    the center of the rounded rectangle, and another corner at the
        //    center of the relevant corner circle.
        let inside_half_width = (self.width() / 2.0 - rx).max(0.0);
        let inside_half_height = (self.height() / 2.0 - ry).max(0.0);

        // 3. Three things are happening here.
        //
        //    First, the x- and y-values are being reflected into the positive
        //    (bottom-right quadrant).
        //
        //    After reflecting, the points are clamped so that their x- and y-
        //    values can't be lower than the x- and y- values of the center of
        //    the corner ellipse, and the coordinate system is transformed
        //    again, putting (0, 0) at the center of the corner ellipse.
        let px = (pt.x.abs() - inside_half_width).max(0.0);
        let py = (pt.y.abs() - inside_half_height).max(0.0);

        // 4. The transforms above clamp all input points such that they will
        //    be inside the rounded rectangle if the corresponding output point
        //    (px, py) is inside a ellipse centered around the origin with the
        //    given radii.
        let inside = (px * px) / (rx * rx) + (py * py) / (ry * ry) <= 1.0;
        i32::from(inside)
    }

    #[inline]
    fn bounding_box(&self) -> Rect {
        self.rect.bounding_box()
    }
}

struct RectPathIter {
    rect: Rect,
    radii: Vec2,
    ix: usize,
}

// This is clockwise in a y-down coordinate system for positive area.
impl Iterator for RectPathIter {
    type Item = PathEl;

    fn next(&mut self) -> Option<PathEl> {
        self.ix += 1;
        match self.ix {
            1 => Some(PathEl::MoveTo(Point::new(
                self.rect.x0,
                self.rect.y0 + self.radii.y,
            ))),
            2 => Some(PathEl::LineTo(Point::new(
                self.rect.x1 - self.radii.x,
                self.rect.y0,
            ))),
            3 => Some(PathEl::LineTo(Point::new(
                self.rect.x1,
                self.rect.y1 - self.radii.y,
            ))),
            4 => Some(PathEl::LineTo(Point::new(
                self.rect.x0 + self.radii.x,
                self.rect.y1,
            ))),
            5 => Some(PathEl::ClosePath),
            _ => None, // GRCOV_EXCL_LINE - unreachable?
        }
    }
}

// This is clockwise in a y-down coordinate system for positive area.
impl Iterator for RoundRectPathIter {
    type Item = PathEl;

    fn next(&mut self) -> Option<PathEl> {
        if self.idx > 4 {
            return None;
        }

        // Iterate between rectangle and arc iterators.
        // Rect iterator will start and end the path.

        // Initial point set by the rect iterator
        if self.idx == 0 {
            self.idx += 1;
            return self.rect.next();
        }

        // Generate the arc curve elements.
        // If we reached the end of the arc, add a line towards next arc (rect iterator).
        if let Some(elem) = self.arcs[self.idx - 1].next() {
            Some(elem)
        } else {
            self.idx += 1;
            self.rect.next()
        }
    }
}

impl Add<Vec2> for RoundRect {
    type Output = RoundRect;

    #[inline]
    fn add(self, v: Vec2) -> RoundRect {
        RoundRect::from_rect(self.rect + v, self.radii)
    }
}

impl Sub<Vec2> for RoundRect {
    type Output = RoundRect;

    #[inline]
    fn sub(self, v: Vec2) -> RoundRect {
        RoundRect::from_rect(self.rect - v, self.radii)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;
    use kurbo::{Circle, Point, Rect, Shape};

    #[test]
    fn test_round_rect_new() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75);

        assert_eq!(rect.rect.x0, 1.0);
        assert_eq!(rect.rect.y0, 2.0);
        assert_eq!(rect.rect.x1, 3.0);
        assert_eq!(rect.rect.y1, 5.0);
        assert_eq!(rect.radii.x, 0.25);
        assert_eq!(rect.radii.y, 0.75);
    }

    #[test]
    fn test_round_rect_from_rect() {
        let rect = RoundRect::from_rect(Rect::new(1.0, 2.0, 3.0, 5.0), (0.25, 0.75));

        assert_eq!(rect.rect.x0, 1.0);
        assert_eq!(rect.rect.y0, 2.0);
        assert_eq!(rect.rect.x1, 3.0);
        assert_eq!(rect.rect.y1, 5.0);
        assert_eq!(rect.radii.x, 0.25);
        assert_eq!(rect.radii.y, 0.75);
    }

    #[test]
    fn test_round_rect_from_points() {
        let rect = RoundRect::from_points((1.0, 2.0), (3.0, 5.0), (0.25, 0.75));

        assert_eq!(rect.rect.x0, 1.0);
        assert_eq!(rect.rect.y0, 2.0);
        assert_eq!(rect.rect.x1, 3.0);
        assert_eq!(rect.rect.y1, 5.0);
        assert_eq!(rect.radii.x, 0.25);
        assert_eq!(rect.radii.y, 0.75);
    }

    #[test]
    fn test_round_rect_from_origin_size() {
        let rect = RoundRect::from_origin_size((1.0, 2.0), (2.0, 3.0), (0.25, 0.75));

        assert_eq!(rect.rect.x0, 1.0);
        assert_eq!(rect.rect.y0, 2.0);
        assert_eq!(rect.rect.x1, 3.0);
        assert_eq!(rect.rect.y1, 5.0);
        assert_eq!(rect.radii.x, 0.25);
        assert_eq!(rect.radii.y, 0.75);
    }

    #[test]
    fn test_round_rect_from_center_size() {
        let rect = RoundRect::from_center_size((2.0, 3.5), (2.0, 3.0), (0.25, 0.75));

        assert_eq!(rect.rect.x0, 1.0);
        assert_eq!(rect.rect.y0, 2.0);
        assert_eq!(rect.rect.x1, 3.0);
        assert_eq!(rect.rect.y1, 5.0);
        assert_eq!(rect.radii.x, 0.25);
        assert_eq!(rect.radii.y, 0.75);
    }

    #[test]
    fn test_round_rect_width() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75);

        assert_eq!(rect.width(), 2.0);
    }

    #[test]
    fn test_round_rect_height() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75);

        assert_eq!(rect.height(), 3.0);
    }

    #[test]
    fn test_round_rect_radii() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75);

        assert_eq!(rect.radii(), Vec2::new(0.25, 0.75));
    }

    #[test]
    fn test_round_rect_rect() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75);

        assert_eq!(rect.rect(), Rect::new(1.0, 2.0, 3.0, 5.0));
    }

    #[test]
    fn test_round_rect_origin() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75);

        assert_eq!(rect.origin(), Point::new(1.0, 2.0));
    }

    #[test]
    fn test_round_rect_center() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75);

        assert_eq!(rect.center(), Point::new(2.0, 3.5));
    }

    #[test]
    fn test_round_rect_size() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75);

        assert_eq!(rect.size(), Size::new(2.0, 3.0));
    }

    #[test]
    fn test_round_rect_with_origin() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75).with_origin((2.0, 4.0));

        assert_eq!(rect.origin(), Point::new(2.0, 4.0));
    }

    #[test]
    fn test_round_rect_with_size() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75).with_size((3.0, 1.0));

        assert_eq!(rect.size(), Size::new(3.0, 1.0));
    }

    #[test]
    fn test_round_rect_with_radii() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75).with_radii((0.5, 0.25));

        assert_eq!(rect.radii(), Vec2::new(0.5, 0.25));
    }

    #[test]
    fn test_round_rect_path_elements() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75);
        let path = rect.to_path(1e-9);

        assert_approx_eq!(rect.area(), path.area());
        assert_eq!(path.winding(Point::new(2.0, 3.0)), 1);
    }

    #[test]
    fn test_round_rect_area() {
        // Extremum: 0.0 radius corner -> rectangle
        let ref_rect = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect = RoundRect::new(0.0, 0.0, 10.0, 10.0, 0.0, 0.0);
        assert_approx_eq!(ref_rect.area(), rect.area());

        // Extremum: half-size radius corner -> circle
        let circle = Circle::new((0.0, 0.0), 5.0);
        let rect = RoundRect::new(0.0, 0.0, 10.0, 10.0, 5.0, 5.0);
        assert_approx_eq!(circle.area(), rect.area());
    }

    #[test]
    fn test_round_rect_perimeter() {
        // Extremum: 0.0 radius corner -> rectangle
        let rect = RoundRect::new(0.0, 0.0, 10.0, 10.0, 0.0, 0.0);
        assert_approx_eq!(rect.perimeter(1.0), 40.0);

        // Extremum: half-size radius corner -> circle
        let rect = RoundRect::new(0.0, 0.0, 10.0, 10.0, 5.0, 5.0);
        assert_approx_eq!(rect.perimeter(1.0), 10. * PI, 0.01);
    }

    #[test]
    fn test_round_rect_winding() {
        let rect = RoundRect::new(-5.0, -5.0, 10.0, 20.0, 5.0, 5.0);
        assert_eq!(rect.winding(Point::new(0.0, 0.0)), 1);
        assert_eq!(rect.winding(Point::new(-5.0, 0.0)), 1); // left edge
        assert_eq!(rect.winding(Point::new(0.0, 20.0)), 1); // bottom edge
        assert_eq!(rect.winding(Point::new(10.0, 20.0)), 0); // bottom-right corner
        assert_eq!(rect.winding(Point::new(-5.0, 20.0)), 0); // bottom-left corner
        assert_eq!(rect.winding(Point::new(-10.0, 0.0)), 0);

        let rect = RoundRect::new(-10.0, -20.0, 10.0, 20.0, 0.0, 0.0); // rectangle
        assert_eq!(rect.winding(Point::new(10.0, 20.0)), 0); // bottom-right corner
    }

    #[test]
    fn test_round_rect_add_sub() {
        let rect = RoundRect::new(1.0, 2.0, 3.0, 5.0, 0.25, 0.75) + Vec2::new(2.0, 3.0);

        assert_eq!(rect.rect, Rect::new(3.0, 5.0, 5.0, 8.0));
        assert_eq!(rect.radii, Vec2::new(0.25, 0.75));

        let rect = rect - Vec2::new(1.0, 4.0);

        assert_eq!(rect.rect, Rect::new(2.0, 1.0, 4.0, 4.0));
        assert_eq!(rect.radii, Vec2::new(0.25, 0.75));
    }
}
