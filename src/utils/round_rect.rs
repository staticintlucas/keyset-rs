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

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.rect.is_finite() && self.radii.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.rect.is_nan() || self.radii.is_nan()
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
            _ => None,
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

    use kurbo::{Circle, Point, Rect, Shape};

    #[test]
    fn area() {
        let epsilon = 1e-9;

        // Extremum: 0.0 radius corner -> rectangle
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rounded_rect = RoundRect::new(0.0, 0.0, 100.0, 100.0, 0.0, 0.0);
        assert!((rect.area() - rounded_rect.area()).abs() < epsilon);

        // Extremum: half-size radius corner -> circle
        let circle = Circle::new((0.0, 0.0), 50.0);
        let rounded_rect = RoundRect::new(0.0, 0.0, 100.0, 100.0, 50.0, 50.0);
        assert!((circle.area() - rounded_rect.area()).abs() < epsilon);
    }

    #[test]
    fn winding() {
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
    fn bez_conversion() {
        let rect = RoundRect::new(-5.0, -5.0, 10.0, 20.0, 5.0, 5.0);
        let p = rect.to_path(1e-9);
        // Note: could be more systematic about tolerance tightness.
        let epsilon = 1e-7;
        assert!((rect.area() - p.area()).abs() < epsilon);
        assert_eq!(p.winding(Point::new(0.0, 0.0)), 1);
    }
}