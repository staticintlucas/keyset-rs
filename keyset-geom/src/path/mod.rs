mod arc_to_bezier;
mod segment;
mod to_path;

use std::borrow::Borrow;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};

use self::arc_to_bezier::arc_to_bezier;
pub use self::segment::PathSegment;
pub use self::to_path::ToPath;
use crate::{Angle, Length, Point, Rect, Scale, Transform, Unit, Vector};

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

impl<U> Path<U>
where
    U: Unit,
{
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
        slice.iter().collect()
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

impl<'a, U> IntoIterator for &'a Path<U>
where
    U: Unit,
{
    type Item = &'a PathSegment<U>;
    type IntoIter = std::slice::Iter<'a, PathSegment<U>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, U> IntoIterator for &'a mut Path<U>
where
    U: Unit,
{
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
        // TODO into_vec is needed here but is essentially free; see rust-lang/rust#59878
        self.data.into_vec().into_iter()
    }
}

impl<U, B> FromIterator<B> for Path<U>
where
    B: Borrow<Self>,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        let (data, bounds) = iter.next().map_or_else(
            || (Box::default(), Rect::zero()),
            |path| {
                let mut data = path.borrow().data.to_vec();
                let mut bounds: Rect<U> = path.borrow().bounds;

                for path in iter {
                    let path: &Self = path.borrow();
                    if !matches!(path.data.first(), Some(&PathSegment::Move(..))) {
                        data.push(PathSegment::Move(Point::origin()));
                    }
                    data.extend(path.data.iter());
                    bounds = Rect::new(
                        bounds.min.min(path.bounds.min),
                        bounds.max.max(path.bounds.max),
                    );
                }

                (data.into_boxed_slice(), bounds)
            },
        );

        Self { data, bounds }
    }
}

impl<U, V> Mul<Scale<U, V>> for Path<U>
where
    U: Unit,
{
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

impl<U, V> Div<Scale<V, U>> for Path<U>
where
    U: Unit,
{
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

impl<U> Default for PathBuilder<U>
where
    U: Unit,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<U> PathBuilder<U>
where
    U: Unit,
{
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
        self.rel_line(Vector::new(dx.length.get(), 0.0));
    }

    /// Add a vertical line segment with relative distance
    #[inline]
    pub fn rel_vert_line(&mut self, dy: Length<U>) {
        self.rel_line(Vector::new(0.0, dy.length.get()));
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
        arc_to_bezier(r, xar, laf, sf, d, |d1, d2, d| {
            self.data.push(PathSegment::CubicBezier(d1, d2, d));
            self.bounds = update_bounds(self.bounds, self.point + d);
            self.point += d;
        });
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

impl<U> Add for PathBuilder<U>
where
    U: Unit,
{
    type Output = Self;

    /// Return a new [`PathBuilder`] by appending another [`PathBuilder`] to `self`
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
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use euclid::Scale;
    use isclose::assert_is_close;

    use super::*;
    use crate::{Mm, Size};

    #[test]
    fn test_path_clone() {
        let path = Path::<()> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };

        #[allow(clippy::redundant_clone)] // We want to test clone
        let path2 = path.clone();

        assert_eq!(path.data.len(), path2.data.len());
        assert_is_close!(path.bounds, path2.bounds);
    }

    #[test]
    fn test_path_empty() {
        let paths = [Path::<Mm>::default(), Path::empty()];

        for path in paths {
            assert!(path.data.is_empty());
            assert_is_close!(path.bounds, Rect::zero());
        }
    }

    #[test]
    fn test_path_from_slice() {
        let paths = [
            Path::<Mm>::empty(),
            Path {
                data: Box::new([
                    PathSegment::Move(Point::zero()),
                    PathSegment::Line(Vector::one()),
                ]),
                bounds: Rect::from_size(Size::splat(1.0)),
            },
            Path {
                data: Box::new([PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 0.5),
                    Vector::splat(1.0),
                )]),
                bounds: Rect::from_size(Size::splat(1.0)),
            },
        ];

        let expected = Path {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::Move(Point::zero()),
                PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 0.5),
                    Vector::splat(1.0),
                ),
            ]),
            bounds: Rect::from_size(Size::splat(1.0)),
        };

        let path = Path::from_slice(&paths);

        assert_is_close!(path.bounds, expected.bounds);
        for (p, e) in path.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p, e);
        }
    }

    #[test]
    fn test_path_len() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };

        assert_eq!(path.len(), path.data.len());
    }

    #[test]
    fn test_path_is_empty() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };

        assert!(Path::<Mm>::empty().is_empty());
        assert!(!path.is_empty());
    }

    #[test]
    fn test_path_from_iter() {
        let paths = [
            Path::<Mm>::empty(),
            Path {
                data: Box::new([
                    PathSegment::Move(Point::zero()),
                    PathSegment::Line(Vector::one()),
                ]),
                bounds: Rect::from_size(Size::splat(1.0)),
            },
            Path {
                data: Box::new([PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 0.5),
                    Vector::splat(1.0),
                )]),
                bounds: Rect::from_size(Size::splat(1.0)),
            },
        ];

        let expected = Path {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::Move(Point::zero()),
                PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 0.5),
                    Vector::splat(1.0),
                ),
            ]),
            bounds: Rect::from_size(Size::splat(1.0)),
        };

        let path: Path<_> = paths.iter().collect();

        assert_is_close!(path.bounds, expected.bounds);
        for (p, e) in path.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p, e);
        }
    }

    #[test]
    fn test_path_translate() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        }
        .translate(Vector::new(1.0, 2.0));

        let exp = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::new(1.0, 2.0)),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::new(1.0, 2.0), Point::new(4.0, 5.0)),
        };

        assert_is_close!(path.bounds, exp.bounds);
        for (p, e) in path.data.iter().zip(exp.data.iter()) {
            assert_is_close!(p, e);
        }
    }

    #[test]
    fn test_path_scale() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        }
        .scale(0.5, 0.5);

        let exp = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::splat(0.5)),
                PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 0.5),
                    Vector::splat(1.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(1.5)),
        };

        assert_is_close!(path.bounds, exp.bounds);
        for (p, e) in path.data.iter().zip(exp.data.iter()) {
            assert_is_close!(p, e);
        }
    }

    #[test]
    fn test_path_iter() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };

        for (p1, p2) in path.iter().zip(path.data.iter()) {
            assert!(std::ptr::eq(p1, p2));
        }
    }

    #[test]
    fn test_path_iter_mut() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };

        let mut path2 = path.clone();
        path2.iter_mut().for_each(|seg| *seg = seg.scale(2.0, 2.0));

        for (p1, p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1.scale(2.0, 2.0), p2);
        }
    }

    #[test]
    fn test_path_into_iter() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };

        for (p1, p2) in (&path).into_iter().zip(path.data.iter()) {
            assert!(std::ptr::eq(p1, p2));
        }

        let mut path2 = path.clone();
        (&mut path2)
            .into_iter()
            .for_each(|seg| *seg = seg.scale(2.0, 2.0));

        for (p1, p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1.scale(2.0, 2.0), p2);
        }

        let data = path.data.to_vec();

        for (p1, p2) in path.into_iter().zip(data.iter()) {
            assert_is_close!(p1, p2);
        }
    }

    #[test]
    fn test_path_mul() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };
        let scale = Scale::new(2.0);
        let transform = Transform::new(2.0, 0.5, 1.5, 1.0, 2.0, 3.0);

        let path2 = path.clone() * scale;
        for (p1, p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1.scale(scale.get(), scale.get()), p2);
        }

        let mut path2 = path.clone();
        path2 *= scale;

        for (p1, p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1.scale(scale.get(), scale.get()), p2);
        }

        let path2 = path.clone() * transform;

        for (p1, p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(*p1 * transform, p2);
        }

        let mut path2 = path.clone();
        path2 *= transform;

        for (p1, p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(*p1 * transform, p2);
        }
    }

    #[test]
    fn test_path_div() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };
        let scale = Scale::new(2.0);

        let path2 = path.clone() / scale;
        for (p1, p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1.scale(1.0 / scale.get(), 1.0 / scale.get()), p2);
        }

        let mut path2 = path.clone();
        path2 /= scale;

        for (p1, p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1.scale(1.0 / scale.get(), 1.0 / scale.get()), p2);
        }
    }

    #[test]
    fn test_path_builder_clone() {
        let builder = PathBuilder::<()> {
            data: vec![
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ],
            start: Point::zero(),
            point: Point::zero(),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };

        #[allow(clippy::redundant_clone)] // We want to test clone
        let builder2 = builder.clone();

        assert_eq!(builder.data.len(), builder2.data.len());
        assert_is_close!(builder.start, builder2.start);
        assert_is_close!(builder.point, builder2.point);
        assert_is_close!(builder.bounds, builder2.bounds);
    }

    #[test]
    fn test_path_builder_new() {
        let builders = [
            PathBuilder::<Mm>::new(),
            PathBuilder::default(),
            PathBuilder::with_capacity(0),
            Path::builder(),
            Path::builder_with_capacity(0),
        ];

        for builder in builders {
            assert!(builder.data.is_empty());
            assert_is_close!(builder.start, Point::origin());
            assert_is_close!(builder.point, Point::origin());
            assert_is_close!(builder.bounds, Rect::zero());
        }
    }

    #[test]
    fn test_path_builder_build() {
        let builder = PathBuilder::<Mm> {
            data: vec![
                PathSegment::Move(Point::zero()),
                PathSegment::Line(Vector::one()),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ],
            start: Point::zero(),
            point: Point::zero(),
            bounds: Rect::new(Point::zero(), Point::splat(3.0)),
        };

        let path = builder.clone().build();

        assert_eq!(builder.data.len(), path.data.len());
        assert_is_close!(builder.bounds, path.bounds);
    }

    #[test]
    fn test_path_builder_extend() {
        let empty = PathBuilder::<Mm>::new();
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

        for (mut first, second, expected) in params {
            first.extend(second);

            assert_eq!(first.data.len(), expected.data.len());
            assert_is_close!(first.start, expected.start);
            assert_is_close!(first.point, expected.point);
            assert_is_close!(first.bounds, expected.bounds);
        }
    }

    #[test]
    fn test_commands() {
        let mut mov = PathBuilder::<Mm>::new();
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
            Angle::ZERO,
            false,
            false,
            Point::new(1.0, 1.0),
        );
        arc.rel_arc(
            Vector::new(1.0, 1.0),
            Angle::ZERO,
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

    #[test]
    fn test_path_builder_add() {
        let empty = PathBuilder::<Mm>::new();
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
            let result = first + second;

            assert_eq!(result.data.len(), expected.data.len());
            assert_is_close!(result.start, expected.start);
            assert_is_close!(result.point, expected.point);
            assert_is_close!(result.bounds, expected.bounds);
        }
    }
}
