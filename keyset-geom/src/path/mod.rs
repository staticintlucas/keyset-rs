mod arc_to_bezier;
mod segment;
mod to_path;

use std::ops::{Add, Div, DivAssign, Mul, MulAssign};

use arc_to_bezier::arc_to_bezier;

#[allow(clippy::module_name_repetitions)] // rust-lang/rust-clippy#8524
pub use segment::PathSegment;
#[allow(clippy::module_name_repetitions)] // rust-lang/rust-clippy#8524
pub use to_path::ToPath;

use crate::{Angle, Length, Point, Rect, Scale, Transform, Vector};

/// A 2-dimensional path represented by a number of path segments
#[derive(Debug)]
pub struct Path<U> {
    /// The path segments that make up the path
    pub data: Box<[PathSegment<U>]>,
    /// The bounds of the path
    pub bounds: Rect<U>,
}

// Impl here rather than derive so we don't require U: Clone everywhere
impl<U> Clone for Path<U> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            bounds: self.bounds,
        }
    }
}

impl<U> Default for Path<U> {
    #[inline]
    fn default() -> Self {
        Self {
            data: Box::default(),
            bounds: Rect::default(),
        }
    }
}

impl<U> Path<U> {
    /// Create a new empty path
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create a new [`PathBuilder`], equivalent to calling [`PathBuilder::new()`]
    #[inline]
    #[must_use]
    pub fn builder() -> PathBuilder<U> {
        PathBuilder::new()
    }

    /// Create a new [`PathBuilder`] with a given capacity, equivalent to calling
    /// [`PathBuilder::with_capacity()`]
    #[inline]
    #[must_use]
    pub fn builder_with_capacity(capacity: usize) -> PathBuilder<U> {
        PathBuilder::with_capacity(capacity)
    }

    /// Create a path by joining a slice of paths
    #[inline]
    #[must_use]
    pub fn from_slice(slice: &[Self]) -> Self {
        let capacity = slice
            .iter()
            .map(|el| {
                el.len() + usize::from(!matches!(el.data.first(), Some(&PathSegment::Move(..))))
            })
            .sum();

        let data = slice
            .iter()
            .fold(Vec::with_capacity(capacity), |mut vec, path| {
                if !matches!(path.data.first(), Some(&PathSegment::Move(..))) {
                    vec.push(PathSegment::Move(Point::origin()));
                }
                vec.extend(path.data.iter());
                vec
            })
            .into_boxed_slice();

        let bounds = slice
            .iter()
            .map(|p| p.bounds)
            .reduce(|a, b| a.union(&b))
            .unwrap_or_else(Rect::zero);

        Self { data, bounds }
    }

    /// The number of segments in the path
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    /// If the path is empty
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Translate the path
    #[inline]
    #[must_use]
    pub fn translate(self, by: Vector<U>) -> Self {
        Self {
            data: self.iter().map(|seg| seg.translate(by)).collect(),
            bounds: self.bounds.translate(by),
        }
    }

    /// Scale the path
    #[inline]
    #[must_use]
    pub fn scale(self, x: f32, y: f32) -> Self {
        Self {
            data: self.iter().map(|seg| seg.scale(x, y)).collect(),
            bounds: self.bounds.scale(x, y),
        }
    }

    /// Create an iterator over the path's segments
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, PathSegment<U>> {
        self.data.iter()
    }

    /// Create a mutable iterator over the path's segments
    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, PathSegment<U>> {
        self.data.iter_mut()
    }
}

impl<'a, U> IntoIterator for &'a Path<U> {
    type Item = &'a PathSegment<U>;
    type IntoIter = std::slice::Iter<'a, PathSegment<U>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, U> IntoIterator for &'a mut Path<U> {
    type Item = &'a mut PathSegment<U>;
    type IntoIter = std::slice::IterMut<'a, PathSegment<U>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<U> IntoIterator for Path<U> {
    type Item = PathSegment<U>;
    type IntoIter = std::vec::IntoIter<PathSegment<U>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        // into_vec is needed here, see rust-lang/rust#59878
        self.data.into_vec().into_iter()
    }
}

impl<U, V> Mul<Scale<U, V>> for Path<U> {
    type Output = Path<V>;

    #[inline]
    fn mul(self, scale: Scale<U, V>) -> Self::Output {
        Self::Output {
            data: self.iter().map(|&seg| seg * scale).collect(),
            bounds: self.bounds * scale,
        }
    }
}

impl<U, V> Mul<Transform<U, V>> for Path<U> {
    type Output = Path<V>;

    #[inline]
    fn mul(self, transform: Transform<U, V>) -> Self::Output {
        let data: Box<_> = self.into_iter().map(|seg| seg * transform).collect();
        let bounds = calculate_bounds(&data);
        Self::Output { data, bounds }
    }
}

impl<U> MulAssign<Scale<U, U>> for Path<U> {
    #[inline]
    fn mul_assign(&mut self, scale: Scale<U, U>) {
        self.data.iter_mut().for_each(|seg| *seg *= scale);
        self.bounds *= scale;
    }
}

impl<U> MulAssign<Transform<U, U>> for Path<U> {
    #[inline]
    fn mul_assign(&mut self, transform: Transform<U, U>) {
        self.data.iter_mut().for_each(|seg| *seg *= transform);
        self.bounds = calculate_bounds(&self.data);
    }
}

impl<U, V> Div<Scale<V, U>> for Path<U> {
    type Output = Path<V>;

    #[inline]
    fn div(self, scale: Scale<V, U>) -> Self::Output {
        Self::Output {
            data: self.iter().map(|&seg| seg / scale).collect(),
            bounds: self.bounds / scale,
        }
    }
}

impl<U> DivAssign<Scale<U, U>> for Path<U> {
    #[inline]
    fn div_assign(&mut self, scale: Scale<U, U>) {
        self.data.iter_mut().for_each(|seg| *seg /= scale);
        self.bounds /= scale;
    }
}

/// A builder for [`Path`]s
#[allow(clippy::module_name_repetitions)] // rust-lang/rust-clippy#8524
#[derive(Debug)]
pub struct PathBuilder<U> {
    data: Vec<PathSegment<U>>,
    start: Point<U>,
    point: Point<U>,
    bounds: Rect<U>,
}

// Impl here rather than derive so we don't require U: Clone everywhere
impl<U> Clone for PathBuilder<U> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            start: self.start,
            point: self.point,
            bounds: self.bounds,
        }
    }
}

impl<U> PathBuilder<U> {
    /// Create a new [`PathBuilder`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: vec![],
            start: Point::origin(),
            point: Point::origin(),
            bounds: Rect::new(Point::origin(), Point::origin()),
        }
    }

    /// Create a new [`PathBuilder`] with the given capacity
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            ..Self::new()
        }
    }

    /// Build the [`Path`]
    #[inline]
    #[must_use]
    pub fn build(self) -> Path<U> {
        Path {
            data: self.data.into_boxed_slice(),
            bounds: self.bounds,
        }
    }

    /// Append another [`PathBuilder`] to the builder
    #[inline]
    pub fn extend(&mut self, other: Self) {
        if other.data.is_empty() {
            // Do nothing
        } else if self.data.is_empty() {
            *self = other;
        } else {
            // Add leading move to 0,0 if we don't already start with a move
            if !matches!(other.data[0], PathSegment::Move(..)) {
                self.data.push(PathSegment::Move(Point::origin()));
            }
            self.data.extend(other.data);
            self.bounds = Rect::new(
                Point::min(self.bounds.min, other.bounds.min),
                Point::max(self.bounds.max, other.bounds.max),
            );
            self.start = other.start;
            self.point = other.point;
        }
    }

    /// Add a move segment with relative distance
    #[inline]
    pub fn rel_move(&mut self, d: Vector<U>) {
        self.abs_move(self.point + d);
    }

    /// Add a line segment with relative distance
    #[inline]
    pub fn rel_line(&mut self, d: Vector<U>) {
        self.data.push(PathSegment::Line(d));
        self.point += d;
        self.bounds = update_bounds(self.bounds, self.point);
    }

    /// Add a horizontal line segment with relative distance
    #[inline]
    pub fn rel_horiz_line(&mut self, dx: Length<U>) {
        self.rel_line(Vector::new(dx.get(), 0.0));
    }

    /// Add a vertical line segment with relative distance
    #[inline]
    pub fn rel_vert_line(&mut self, dy: Length<U>) {
        self.rel_line(Vector::new(0.0, dy.get()));
    }

    /// Add a cubic Bézier segment with relative control points and distance
    #[inline]
    pub fn rel_cubic_bezier(&mut self, d1: Vector<U>, d2: Vector<U>, d: Vector<U>) {
        self.data.push(PathSegment::CubicBezier(d1, d2, d));
        self.point += d;
        self.bounds = update_bounds(self.bounds, self.point);
    }

    /// Add a smooth cubic Bézier segment with relative control point and distance
    #[inline]
    pub fn rel_smooth_cubic_bezier(&mut self, d2: Vector<U>, d: Vector<U>) {
        let d1 = match self.data.last() {
            Some(&PathSegment::CubicBezier(_, prev_d2, prev_d)) => prev_d - prev_d2,
            _ => Vector::zero(),
        };
        self.rel_cubic_bezier(d1, d2, d);
    }

    /// Add a quadratic Bézier segment with relative control point and distance
    #[inline]
    pub fn rel_quadratic_bezier(&mut self, d1: Vector<U>, d: Vector<U>) {
        self.data.push(PathSegment::QuadraticBezier(d1, d));
        self.point += d;
        self.bounds = update_bounds(self.bounds, self.point);
    }

    /// Add a smooth quadratic Bézier segment with relative distance
    #[inline]
    pub fn rel_smooth_quadratic_bezier(&mut self, d: Vector<U>) {
        let d1 = match self.data.last() {
            Some(&PathSegment::QuadraticBezier(prev_d1, prev_d)) => prev_d - prev_d1,
            _ => Vector::zero(),
        };
        self.rel_quadratic_bezier(d1, d);
    }

    /// Add an arc segment with relative distance
    #[inline]
    pub fn rel_arc(&mut self, r: Vector<U>, xar: Angle, laf: bool, sf: bool, d: Vector<U>) {
        for (d1, d2, d) in arc_to_bezier(r, xar, laf, sf, d) {
            self.data.push(PathSegment::CubicBezier(d1, d2, d));
            self.bounds = update_bounds(self.bounds, self.point + d);
            self.point += d;
        }
    }

    /// Close the path
    #[inline]
    pub fn close(&mut self) {
        self.data.push(PathSegment::Close);
        self.point = self.start;
    }

    /// Add a move segment with absolute distance
    #[inline]
    pub fn abs_move(&mut self, p: Point<U>) {
        self.bounds = if self.data.is_empty() {
            Rect::new(p, p)
        } else {
            update_bounds(self.bounds, p)
        };
        self.data.push(PathSegment::Move(p));
        self.start = p;
        self.point = p;
    }

    /// Add a line segment with absolute distance
    #[inline]
    pub fn abs_line(&mut self, p: Point<U>) {
        self.rel_line(p - self.point);
    }

    /// Add a horizontal line segment with absolute distance
    #[inline]
    pub fn abs_horiz_line(&mut self, x: Length<U>) {
        self.rel_horiz_line(x - Length::new(self.point.x));
    }

    /// Add a vertical line segment with absolute distance
    #[inline]
    pub fn abs_vert_line(&mut self, y: Length<U>) {
        self.rel_vert_line(y - Length::new(self.point.y));
    }

    /// Add a cubic Bézier segment with absolute control points and distance
    #[inline]
    pub fn abs_cubic_bezier(&mut self, p1: Point<U>, p2: Point<U>, p: Point<U>) {
        self.rel_cubic_bezier(p1 - self.point, p2 - self.point, p - self.point);
    }

    /// Add a smooth cubic Bézier segment with absolute control point and distance
    #[inline]
    pub fn abs_smooth_cubic_bezier(&mut self, p2: Point<U>, p: Point<U>) {
        self.rel_smooth_cubic_bezier(p2 - self.point, p - self.point);
    }

    /// Add a quadratic Bézier segment with absolute control point and distance
    #[inline]
    pub fn abs_quadratic_bezier(&mut self, p1: Point<U>, p: Point<U>) {
        self.rel_quadratic_bezier(p1 - self.point, p - self.point);
    }

    /// Add a smooth quadratic Bézier segment with absolute distance
    #[inline]
    pub fn abs_smooth_quadratic_bezier(&mut self, p: Point<U>) {
        self.rel_smooth_quadratic_bezier(p - self.point);
    }

    /// Add an arc segment with absolute distance
    #[inline]
    pub fn abs_arc(&mut self, r: Vector<U>, xar: Angle, laf: bool, sf: bool, p: Point<U>) {
        self.rel_arc(r, xar, laf, sf, p - self.point);
    }
}

impl<U> Default for PathBuilder<U> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<U> Add for PathBuilder<U> {
    type Output = Self;

    /// Return a new [`PathBuilder`] by appending another [`PathBuilder`] to [`self`]
    #[inline]
    fn add(mut self, other: Self) -> Self::Output {
        self.extend(other);
        self
    }
}

#[inline]
fn update_bounds<U>(bounds: Rect<U>, p: Point<U>) -> Rect<U> {
    Rect::new(Point::min(bounds.min, p), Point::max(bounds.max, p))
}

fn calculate_bounds<U>(data: &[PathSegment<U>]) -> Rect<U> {
    // Add leading move to (0, 0) if we don't already start with a move
    let mov = (!matches!(data.first(), Some(&PathSegment::Move(..))))
        .then_some(PathSegment::Move(Point::zero()));

    Rect::from_points(
        mov.iter()
            .chain(data.iter())
            .scan(Point::origin(), |point, seg| {
                *point = match *seg {
                    PathSegment::Move(p) => p,
                    PathSegment::Line(d)
                    | PathSegment::CubicBezier(_, _, d)
                    | PathSegment::QuadraticBezier(_, d) => *point + d,
                    // Close doesn't have a point, but there's no harm in returning the previous
                    // point to simplify the logic
                    PathSegment::Close => *point,
                };
                Some(*point)
            }),
    )
}

#[cfg(test)]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    use crate::{Angle, Size};

    #[test]
    fn test_path_new() {
        let paths: Vec<PathBuilder<()>> = vec![PathBuilder::new(), PathBuilder::default()];

        for path in paths {
            assert!(path.data.is_empty());
            assert_is_close!(path.start, Point::origin());
            assert_is_close!(path.point, Point::origin());
            assert_is_close!(path.bounds, Rect::zero());
        }
    }

    #[test]
    fn test_path_add() {
        let empty = PathBuilder::<()>::new();
        let mut line1 = PathBuilder::new();
        line1.abs_line(Point::new(1.0, 1.0));

        let mut line2 = PathBuilder::new();
        line2.abs_line(Point::new(1.0, 0.0));

        let mut line3 = PathBuilder::new();
        line3.abs_move(Point::new(0.0, 1.0));
        line3.abs_line(Point::new(1.0, 0.0));

        let mut angle = PathBuilder::new();
        angle.abs_line(Point::new(1.0, 1.0));
        angle.abs_move(Point::zero());
        angle.abs_line(Point::new(1.0, 0.0));

        let mut cross = PathBuilder::new();
        cross.abs_line(Point::new(1.0, 1.0));
        cross.abs_move(Point::new(0.0, 1.0));
        cross.abs_line(Point::new(1.0, 0.0));

        #[allow(clippy::redundant_clone)]
        let params = [
            (empty.clone(), empty.clone(), empty.clone()),
            (line1.clone(), empty.clone(), line1.clone()),
            (empty.clone(), line1.clone(), line1.clone()),
            (line1.clone(), line2.clone(), angle.clone()),
            (line1.clone(), line3.clone(), cross.clone()),
        ];

        for (first, second, expected) in params {
            let result = first.add(second);

            assert_eq!(result.data.len(), expected.data.len());
            assert_is_close!(result.start, expected.start);
            assert_is_close!(result.point, expected.point);
            assert_is_close!(result.bounds, expected.bounds);
        }
    }

    #[test]
    fn test_commands() {
        let mut mov = PathBuilder::<()>::new();
        mov.abs_move(Point::origin());
        mov.rel_move(Vector::new(2.0, 2.0));
        mov.close();

        let mut line = PathBuilder::new();
        line.abs_line(Point::new(1.0, 1.0));
        line.rel_line(Vector::new(1.0, 1.0));
        line.close();

        let mut vert_horiz = PathBuilder::new();
        vert_horiz.abs_vert_line(Length::new(2.0));
        vert_horiz.rel_horiz_line(Length::new(2.0));
        vert_horiz.close();

        let mut horiz_vert = PathBuilder::new();
        horiz_vert.abs_horiz_line(Length::new(2.0));
        horiz_vert.rel_vert_line(Length::new(2.0));
        horiz_vert.close();

        let mut curve1 = PathBuilder::new();
        curve1.abs_cubic_bezier(
            Point::new(0.0, 0.5),
            Point::new(0.5, 1.0),
            Point::new(1.0, 1.0),
        );
        curve1.rel_smooth_quadratic_bezier(Vector::new(1.0, 1.0));

        let mut curve2 = PathBuilder::new();
        curve2.abs_quadratic_bezier(Point::new(0.0, 1.0), Point::new(1.0, 1.0));
        curve2.rel_smooth_cubic_bezier(Vector::new(1.0, 0.5), Vector::new(1.0, 1.0));

        let mut curve3 = PathBuilder::new();
        curve3.rel_cubic_bezier(
            Vector::new(0.0, 0.5),
            Vector::new(0.5, 1.0),
            Vector::new(1.0, 1.0),
        );
        curve3.abs_smooth_cubic_bezier(Point::new(2.0, 1.5), Point::new(2.0, 2.0));
        curve3.close();

        let mut curve4 = PathBuilder::new();
        curve4.rel_quadratic_bezier(Vector::new(0.0, 1.0), Vector::new(1.0, 1.0));
        curve4.abs_smooth_quadratic_bezier(Point::new(2.0, 2.0));
        curve4.close();

        let mut curve5 = PathBuilder::new();
        curve5.abs_smooth_cubic_bezier(Point::new(0.0, 2.0), Point::new(2.0, 2.0));

        let mut curve6 = PathBuilder::new();
        curve6.abs_smooth_quadratic_bezier(Point::new(2.0, 2.0));

        let mut arc = PathBuilder::new();
        arc.abs_arc(
            Vector::new(1.0, 1.0),
            Angle::zero(),
            false,
            false,
            Point::new(1.0, 1.0),
        );
        arc.rel_arc(
            Vector::new(1.0, 1.0),
            Angle::zero(),
            false,
            true,
            Vector::new(1.0, 1.0),
        );

        let params = vec![
            mov, line, vert_horiz, horiz_vert, curve1, curve2, curve3, curve4, curve5, curve6, arc,
        ];

        for path in params {
            assert_is_close!(path.bounds, Rect::from_size(Size::splat(2.0)));
        }
    }
}
