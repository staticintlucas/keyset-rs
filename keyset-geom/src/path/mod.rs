mod arc_to_bezier;
mod segment;

use core::f32;
use std::ops;

use self::arc_to_bezier::arc_to_bezier;
pub use self::segment::PathSegment;
use crate::{Angle, Length, Point, Rect, Rotate, Scale, Transform, Translate, Unit, Vector};

/// A 2-dimensional path represented by a number of path segments
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Path<U: Unit> {
    /// The path segments that make up the path
    pub data: Box<[PathSegment<U>]>,
    /// The bounds of the path
    pub bounds: Rect<U>,
}

impl<U> Path<U>
where
    U: Unit,
{
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

    /// Create an empty path
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            data: Box::new([]),
            bounds: Rect::empty(),
        }
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

impl<U> IntoIterator for Path<U>
where
    U: Unit,
{
    type Item = PathSegment<U>;
    type IntoIter = std::vec::IntoIter<PathSegment<U>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        // TODO into_vec is needed here but is essentially free; see rust-lang/rust#59878
        self.data.into_vec().into_iter()
    }
}

impl<U> FromIterator<Self> for Path<U>
where
    U: Unit,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        let Some(path) = iter.next() else {
            return Self::empty();
        };
        let mut data = path.data.into_vec();
        let mut bounds = path.bounds;

        for path in iter {
            if !matches!(path.data.first(), Some(&PathSegment::Move(..))) {
                data.push(PathSegment::Move(Point::origin()));
            }
            data.extend(path.data.iter());
            bounds = Rect::new(
                bounds.min.min(path.bounds.min),
                bounds.max.max(path.bounds.max),
            );
        }

        Self {
            data: data.into_boxed_slice(),
            bounds,
        }
    }
}

impl<'a, U> FromIterator<&'a Self> for Path<U>
where
    U: Unit,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a Self>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        iter.next().map_or_else(
            || Self::empty(),
            |path| {
                let mut data = path.data.to_vec();
                let mut bounds: Rect<U> = path.bounds;

                for path in iter {
                    if !matches!(path.data.first(), Some(&PathSegment::Move(..))) {
                        data.push(PathSegment::Move(Point::origin()));
                    }
                    data.extend(path.data.iter());
                    bounds = Rect::new(
                        bounds.min.min(path.bounds.min),
                        bounds.max.max(path.bounds.max),
                    );
                }

                Self {
                    data: data.into_boxed_slice(),
                    bounds,
                }
            },
        )
    }
}

impl<U> ops::Mul<f32> for Path<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            data: self
                .data
                .iter()
                .map(|&seg| seg * rhs)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            bounds: self.bounds * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for Path<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.data.iter_mut().for_each(|seg| *seg *= rhs);
        self.bounds *= rhs;
    }
}

impl<U> ops::Div<f32> for Path<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            data: self
                .data
                .iter()
                .map(|&seg| seg / rhs)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            bounds: self.bounds / rhs,
        }
    }
}

impl<U> ops::DivAssign<f32> for Path<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.data.iter_mut().for_each(|seg| *seg /= rhs);
        self.bounds /= rhs;
    }
}

impl<U> ops::Mul<Rotate> for Path<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Rotate) -> Self::Output {
        let data: Box<[_]> = self
            .data
            .into_vec()
            .into_iter()
            .map(|seg| seg * rhs)
            .collect();
        let bounds = calculate_bounds(&data);

        Self { data, bounds }
    }
}

impl<U> ops::MulAssign<Rotate> for Path<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Rotate) {
        self.data.iter_mut().for_each(|seg| *seg *= rhs);
        self.bounds = calculate_bounds(&self.data);
    }
}

impl<U> ops::Mul<Scale> for Path<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Scale) -> Self::Output {
        Self {
            data: self
                .data
                .into_vec()
                .into_iter()
                .map(|seg| seg * rhs)
                .collect(),
            bounds: self.bounds * rhs,
        }
    }
}

impl<U> ops::MulAssign<Scale> for Path<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Scale) {
        self.data.iter_mut().for_each(|seg| *seg *= rhs);
        self.bounds *= rhs;
    }
}

impl<U> ops::Div<Scale> for Path<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Scale) -> Self::Output {
        Self {
            data: self
                .data
                .into_vec()
                .into_iter()
                .map(|seg| seg / rhs)
                .collect(),
            bounds: self.bounds / rhs,
        }
    }
}

impl<U> ops::DivAssign<Scale> for Path<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: Scale) {
        self.data.iter_mut().for_each(|seg| *seg /= rhs);
        self.bounds /= rhs;
    }
}

impl<U> ops::Mul<Translate<U>> for Path<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Translate<U>) -> Self::Output {
        Self {
            data: self
                .data
                .into_vec()
                .into_iter()
                .map(|seg| seg * rhs)
                .collect(),
            bounds: self.bounds * rhs,
        }
    }
}

impl<U> ops::MulAssign<Translate<U>> for Path<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Translate<U>) {
        self.data.iter_mut().for_each(|seg| *seg *= rhs);
        self.bounds *= rhs;
    }
}

impl<U> ops::Mul<Transform<U>> for Path<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Transform<U>) -> Self::Output {
        let data: Box<[_]> = self
            .data
            .into_vec()
            .into_iter()
            .map(|seg| seg * rhs)
            .collect();
        let bounds = calculate_bounds(&data);

        Self { data, bounds }
    }
}

impl<U> ops::MulAssign<Transform<U>> for Path<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Transform<U>) {
        self.data.iter_mut().for_each(|seg| *seg *= rhs);
        self.bounds = calculate_bounds(&self.data);
    }
}

/// A builder for [`Path`]s
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathBuilder<U: Unit> {
    data: Vec<PathSegment<U>>,
    start: Point<U>,
    point: Point<U>,
    bounds: Rect<U>,
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
            data: Vec::new(),
            start: Point::origin(),
            point: Point::origin(),
            bounds: Rect::empty(),
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
    pub fn extend(&mut self, other: &Self) {
        *self += other;
    }

    /// Append another [`PathBuilder`] to the builder
    #[inline]
    pub fn extend_from_path(&mut self, other: &Path<U>) {
        *self += other;
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
        self.bounds = Rect::new(
            self.bounds.min.min(self.point),
            self.bounds.max.max(self.point),
        );
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
        self.bounds = Rect::new(
            self.bounds.min.min(self.point),
            self.bounds.max.max(self.point),
        );
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
        self.bounds = Rect::new(
            self.bounds.min.min(self.point),
            self.bounds.max.max(self.point),
        );
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
            self.point += d;
            self.bounds = Rect::new(
                self.bounds.min.min(self.point),
                self.bounds.max.max(self.point),
            );
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
            Rect::new(self.bounds.min.min(p), self.bounds.max.max(p))
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
        self.rel_horiz_line(x - Length::from_unit(self.point.x));
    }

    /// Add a vertical line segment with absolute distance
    #[inline]
    pub fn abs_vert_line(&mut self, y: Length<U>) {
        self.rel_vert_line(y - Length::from_unit(self.point.y));
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

impl<U> Default for PathBuilder<U>
where
    U: Unit,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<U> ops::Add<Self> for PathBuilder<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl<U> ops::Add<&Self> for PathBuilder<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<U> ops::Add<Path<U>> for PathBuilder<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Path<U>) -> Self::Output {
        self + &rhs
    }
}

impl<U> ops::Add<&Path<U>> for PathBuilder<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: &Path<U>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<U> ops::AddAssign<Self> for PathBuilder<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl<U> ops::AddAssign<&Self> for PathBuilder<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        if let Some(&first) = rhs.data.first() {
            // Add leading move to 0,0 if we don't already start with a move
            if !matches!(first, PathSegment::Move(..)) {
                self.data.push(PathSegment::Move(Point::origin()));
            }
            self.data.extend(rhs.data.iter());
            self.bounds = Rect::new(
                self.bounds.min.min(rhs.bounds.min),
                self.bounds.max.max(rhs.bounds.max),
            );
            self.start = rhs.start;
            self.point = rhs.point;
        }
    }
}

impl<U> ops::AddAssign<Path<U>> for PathBuilder<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Path<U>) {
        *self += &rhs;
    }
}

impl<U> ops::AddAssign<&Path<U>> for PathBuilder<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: &Path<U>) {
        if let Some(&first) = rhs.data.first() {
            // Add leading move to 0,0 if we don't already start with a move
            if !matches!(first, PathSegment::Move(..)) {
                self.data.push(PathSegment::Move(Point::origin()));
            }
            self.data.extend(rhs.data.iter());
            self.bounds = Rect::new(
                self.bounds.min.min(rhs.bounds.min),
                self.bounds.max.max(rhs.bounds.max),
            );
            // Find the last move segment in the path
            let (idx, start) = rhs
                .data
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, &seg)| match seg {
                    PathSegment::Move(p) => Some((i, p)),
                    _ => None,
                })
                .unwrap_or_else(|| (0, Point::origin()));

            self.start = start;
            self.point = rhs
                .data
                .iter()
                .skip(idx)
                .fold(start, |point, &seg| match seg {
                    PathSegment::Move(p) => p,
                    PathSegment::Line(d)
                    | PathSegment::CubicBezier(_, _, d)
                    | PathSegment::QuadraticBezier(_, d) => point + d,
                    PathSegment::Close => start,
                });
        }
    }
}

impl<U> ops::Mul<f32> for PathBuilder<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            data: self.data.iter().map(|&seg| seg * rhs).collect(),
            bounds: self.bounds * rhs,
            start: self.start * rhs,
            point: self.point * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for PathBuilder<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.data.iter_mut().for_each(|seg| *seg *= rhs);
        self.bounds *= rhs;
        self.start *= rhs;
        self.point *= rhs;
    }
}

impl<U> ops::Div<f32> for PathBuilder<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            data: self.data.iter().map(|&seg| seg / rhs).collect(),
            bounds: self.bounds / rhs,
            start: self.start / rhs,
            point: self.point / rhs,
        }
    }
}

impl<U> ops::DivAssign<f32> for PathBuilder<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.data.iter_mut().for_each(|seg| *seg /= rhs);
        self.bounds /= rhs;
        self.start /= rhs;
        self.point /= rhs;
    }
}

pub fn calculate_bounds<U: Unit>(data: &[PathSegment<U>]) -> Rect<U> {
    let (min, max) = if matches!(data.first(), Some(&PathSegment::Move(..))) {
        (Point::splat(f32::INFINITY), Point::splat(f32::NEG_INFINITY))
    } else {
        (Point::origin(), Point::origin()) // Implicit move to (0, 0) if we don't start with a move
    };

    let (min, max) = data
        .iter()
        .scan(Point::origin(), |point, seg| {
            *point = match *seg {
                PathSegment::Move(p) => p,
                PathSegment::Line(d)
                | PathSegment::CubicBezier(_, _, d)
                | PathSegment::QuadraticBezier(_, d) => *point + d,
                PathSegment::Close => *point,
            };
            Some(*point)
        })
        .fold((min, max), |(min, max), point| {
            (min.min(point), max.max(point))
        });

    Rect::new(min, max)
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use super::*;
    use crate::Mm;

    #[test]
    fn path_from_slice() {
        let paths = [
            Path::<Mm> {
                data: Box::new([]),
                bounds: Rect::empty(),
            },
            Path {
                data: Box::new([
                    PathSegment::Move(Point::origin()),
                    PathSegment::Line(Vector::splat(1.0)),
                ]),
                bounds: Rect::from_origin_and_size(Point::origin(), Vector::splat(1.0)),
            },
            Path {
                data: Box::new([PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 0.5),
                    Vector::splat(1.0),
                )]),
                bounds: Rect::from_origin_and_size(Point::origin(), Vector::splat(1.0)),
            },
        ];

        let expected = Path {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::Move(Point::origin()),
                PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 0.5),
                    Vector::splat(1.0),
                ),
            ]),
            bounds: Rect::from_origin_and_size(Point::origin(), Vector::splat(1.0)),
        };

        let path = Path::from_slice(&paths);

        assert_is_close!(path.bounds, expected.bounds);
        for (p, e) in path.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p, e);
        }
    }

    #[test]
    fn path_empty() {
        let path = Path::<Mm>::empty();

        assert_eq!(path.data.len(), 0);
        assert_is_close!(path.bounds.width(), Mm(0.0));
        assert_is_close!(path.bounds.height(), Mm(0.0));
    }

    #[test]
    fn path_len() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::splat(3.0)),
        };

        assert_eq!(path.len(), path.data.len());
    }

    #[test]
    fn path_is_empty() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::splat(3.0)),
        };
        assert!(!path.is_empty());

        let path = Path::<Mm> {
            data: Box::new([]),
            bounds: Rect::empty(),
        };
        assert!(path.is_empty());
    }

    #[test]
    fn path_iter() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::splat(3.0)),
        };

        for (p1, p2) in path.iter().zip(path.data.iter()) {
            assert!(std::ptr::eq(p1, p2));
        }
    }

    #[test]
    fn path_iter_mut() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::splat(3.0)),
        };

        let mut path2 = path.clone();
        path2.iter_mut().for_each(|seg| *seg *= 2.0);

        for (&p1, &p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1 * 2.0, p2);
        }
    }

    #[test]
    fn path_into_iter() {
        let mut path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::splat(3.0)),
        };
        let data = path.data.to_vec();

        for (p1, p2) in path.clone().into_iter().zip(data.iter()) {
            assert_is_close!(p1, p2);
        }

        for (p1, p2) in (&path).into_iter().zip(data.iter()) {
            assert_is_close!(p1, p2);
        }

        for (p1, p2) in (&mut path).into_iter().zip(data.iter()) {
            assert_is_close!(p1, p2);
        }
    }

    #[test]
    fn path_from_iter() {
        let paths = [];
        let expected = Path::<Mm> {
            data: Box::new([]),
            bounds: Rect::empty(),
        };

        let path: Path<_> = paths.clone().into_iter().collect();
        assert_is_close!(path.bounds, expected.bounds);
        for (p, e) in path.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p, e);
        }

        let path: Path<_> = paths.iter().collect();
        assert_is_close!(path.bounds, expected.bounds);
        for (p, e) in path.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p, e);
        }

        let paths = [
            Path::<Mm> {
                data: Box::new([]),
                bounds: Rect::empty(),
            },
            Path {
                data: Box::new([
                    PathSegment::Move(Point::origin()),
                    PathSegment::Line(Vector::splat(1.0)),
                ]),
                bounds: Rect::from_origin_and_size(Point::origin(), Vector::splat(1.0)),
            },
            Path {
                data: Box::new([PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 0.5),
                    Vector::splat(1.0),
                )]),
                bounds: Rect::from_origin_and_size(Point::origin(), Vector::splat(1.0)),
            },
        ];

        let expected = Path {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::Move(Point::origin()),
                PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 0.5),
                    Vector::splat(1.0),
                ),
            ]),
            bounds: Rect::from_origin_and_size(Point::origin(), Vector::splat(1.0)),
        };

        let path: Path<_> = paths.clone().into_iter().collect();
        assert_is_close!(path.bounds, expected.bounds);
        for (p, e) in path.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p, e);
        }

        let path: Path<_> = paths.iter().collect();
        assert_is_close!(path.bounds, expected.bounds);
        for (p, e) in path.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p, e);
        }
    }

    #[test]
    fn path_mul() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::splat(3.0)),
        };

        let path2 = path.clone() * 2.0;
        assert_is_close!(path2.bounds, path.bounds * 2.0);
        for (&p1, &p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1 * 2.0, p2);
        }

        let mut path2 = path.clone();
        path2 *= 2.0;
        assert_is_close!(path2.bounds, path.bounds * 2.0);
        for (&p1, &p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1 * 2.0, p2);
        }
    }

    #[test]
    fn path_div() {
        let path = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::splat(3.0)),
        };

        let path2 = path.clone() / 2.0;
        assert_is_close!(path2.bounds, path.bounds / 2.0);
        for (&p1, &p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1 / 2.0, p2);
        }

        let mut path2 = path.clone();
        path2 /= 2.0;
        assert_is_close!(path2.bounds, path.bounds / 2.0);
        for (&p1, &p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1 / 2.0, p2);
        }
    }

    #[test]
    fn path_rotate() {
        use std::f32::consts::SQRT_2;

        let rotate = Rotate::degrees(135.0);

        let input = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::new(1.0, 1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::new(2.0, 2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::new(3.0, 3.0)),
        };
        let expected = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::<Mm>::new(0.0, 0.0)),
                PathSegment::Line(Vector::new(-SQRT_2, 0.0)),
                PathSegment::CubicBezier(
                    Vector::new(-0.5 * SQRT_2, 0.5 * SQRT_2),
                    Vector::new(-1.5 * SQRT_2, 0.5 * SQRT_2),
                    Vector::new(-2.0 * SQRT_2, 0.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::new(-3.0 * SQRT_2, 0.0), Point::new(0.0, 0.0)),
        };

        let result = input.clone() * rotate;
        assert_eq!(result.len(), expected.len());
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }

        let mut result = input;
        result *= rotate;
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }
    }

    #[test]
    fn path_scale() {
        let scale = Scale::new(2.0, 0.5);

        let input = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::new(1.0, 1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::new(2.0, 2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::new(3.0, 3.0)),
        };
        let expected = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::<Mm>::new(0.0, 0.0)),
                PathSegment::Line(Vector::new(2.0, 0.5)),
                PathSegment::CubicBezier(
                    Vector::new(2.0, 0.0),
                    Vector::new(4.0, 0.5),
                    Vector::new(4.0, 1.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::new(6.0, 1.5)),
        };

        let result = input.clone() * scale;
        assert_eq!(result.len(), expected.len());
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }

        let mut result = input.clone();
        result *= scale;
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }

        let expected = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::<Mm>::new(0.0, 0.0)),
                PathSegment::Line(Vector::new(0.5, 2.0)),
                PathSegment::CubicBezier(
                    Vector::new(0.5, 0.0),
                    Vector::new(1.0, 2.0),
                    Vector::new(1.0, 4.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::new(1.5, 6.0)),
        };

        let result = input.clone() / scale;
        assert_eq!(result.len(), expected.len());
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }

        let mut result = input;
        result /= scale;
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }
    }

    #[test]
    fn path_translate() {
        let translate = Translate::new(2.0, -1.0);

        let input = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::new(1.0, 1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::new(2.0, 2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::new(3.0, 3.0)),
        };
        let expected = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::new(2.0, -1.0)),
                PathSegment::Line(Vector::new(1.0, 1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::new(2.0, 2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::new(2.0, -1.0), Point::new(5.0, 2.0)),
        };

        let result = input.clone() * translate;
        assert_eq!(result.len(), expected.len());
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }

        let mut result = input;
        result *= translate;
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }
    }

    #[test]
    fn path_transform() {
        let transform = Transform::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);

        let input = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::new(1.0, 1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::new(2.0, 2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::origin(), Point::new(3.0, 3.0)),
        };
        let expected = Path::<Mm> {
            data: Box::new([
                PathSegment::Move(Point::new(-1.0, 2.0)),
                PathSegment::Line(Vector::new(1.5, 1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, -0.5),
                    Vector::new(2.5, 0.5),
                    Vector::new(3.0, 2.0),
                ),
                PathSegment::Close,
            ]),
            bounds: Rect::new(Point::new(-1.0, 2.0), Point::new(3.5, 5.0)),
        };

        let result = input.clone() * transform;
        assert_eq!(result.len(), expected.len());
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }

        let mut result = input;
        result *= transform;
        assert_is_close!(result.bounds, expected.bounds);

        for (&p1, &p2) in result.data.iter().zip(expected.data.iter()) {
            assert_is_close!(p1, p2);
        }
    }

    #[test]
    fn path_builder_new() {
        let builders = [
            PathBuilder::<Mm>::new(),
            PathBuilder::default(),
            PathBuilder::with_capacity(10),
            Path::builder(),
            Path::builder_with_capacity(10),
        ];

        for builder in builders {
            assert!(builder.data.is_empty());
            assert_is_close!(builder.start, Point::origin());
            assert_is_close!(builder.point, Point::origin());
            assert_is_close!(builder.bounds, Rect::empty());
        }
    }

    #[test]
    fn path_builder_build() {
        let builder = PathBuilder::<Mm> {
            data: vec![
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::splat(1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::splat(2.0),
                ),
                PathSegment::Close,
            ],
            start: Point::origin(),
            point: Point::origin(),
            bounds: Rect::new(Point::origin(), Point::splat(3.0)),
        };

        let path = builder.clone().build();

        assert_eq!(builder.data.len(), path.data.len());
        assert_is_close!(builder.bounds, path.bounds);
    }

    #[test]
    fn path_builder_extend() {
        let empty = PathBuilder::<Mm>::new();

        let mut line1 = PathBuilder::new();
        line1.abs_line(Point::new(1.0, 1.0));

        let mut line2 = PathBuilder::new();
        line2.abs_move(Point::origin());
        line2.abs_line(Point::new(1.0, 1.0));

        let mut line3 = PathBuilder::new();
        line3.abs_line(Point::new(1.0, 0.0));

        let mut line4 = PathBuilder::new();
        line4.abs_move(Point::new(0.0, 1.0));
        line4.abs_line(Point::new(1.0, 0.0));

        let mut angle = PathBuilder::new();
        angle.abs_line(Point::new(1.0, 1.0));
        angle.abs_move(Point::origin());
        angle.abs_line(Point::new(1.0, 0.0));

        let mut cross = PathBuilder::new();
        cross.abs_line(Point::new(1.0, 1.0));
        cross.abs_move(Point::new(0.0, 1.0));
        cross.abs_line(Point::new(1.0, 0.0));

        #[allow(clippy::redundant_clone)]
        let params = [
            (empty.clone(), empty.clone(), empty.clone()),
            (line1.clone(), empty.clone(), line1.clone()),
            (empty.clone(), line1.clone(), line2.clone()),
            (line1.clone(), line3.clone(), angle.clone()),
            (line1.clone(), line4.clone(), cross.clone()),
        ];

        for (mut first, second, expected) in params {
            first.extend(&second);

            assert_eq!(first.data.len(), expected.data.len());
            assert_is_close!(first.start, expected.start);
            assert_is_close!(first.point, expected.point);
            assert_is_close!(first.bounds, expected.bounds);
        }
    }

    #[test]
    fn path_builder_extend_from_path() {
        let empty_bldr = PathBuilder::<Mm>::new();
        let empty_path = Path {
            data: Box::new([]),
            bounds: Rect::empty(),
        };

        let mut line1_bldr = PathBuilder::new();
        line1_bldr.abs_line(Point::new(1.0, 1.0));

        let line1_path = Path {
            data: Box::new([PathSegment::Line(Vector::new(1.0, 1.0))]),
            bounds: Rect::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0)),
        };

        let mut line2_bldr = PathBuilder::new();
        line2_bldr.abs_move(Point::origin());
        line2_bldr.abs_line(Point::new(1.0, 1.0));

        let line3_path = Path {
            data: Box::new([PathSegment::Line(Vector::new(1.0, 0.0))]),
            bounds: Rect::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0)),
        };

        let line4_path = Path {
            data: Box::new([
                PathSegment::Move(Point::new(0.0, 1.0)),
                PathSegment::Line(Vector::new(1.0, -1.0)),
            ]),
            bounds: Rect::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0)),
        };

        let mut angle_bldr = PathBuilder::new();
        angle_bldr.abs_line(Point::new(1.0, 1.0));
        angle_bldr.abs_move(Point::origin());
        angle_bldr.abs_line(Point::new(1.0, 0.0));

        let mut cross_bldr = PathBuilder::new();
        cross_bldr.abs_line(Point::new(1.0, 1.0));
        cross_bldr.abs_move(Point::new(0.0, 1.0));
        cross_bldr.abs_line(Point::new(1.0, 0.0));

        #[allow(clippy::redundant_clone)]
        let params = [
            (empty_bldr.clone(), empty_path.clone(), empty_bldr.clone()),
            (line1_bldr.clone(), empty_path.clone(), line1_bldr.clone()),
            (empty_bldr.clone(), line1_path.clone(), line2_bldr.clone()),
            (line1_bldr.clone(), line3_path.clone(), angle_bldr.clone()),
            (line1_bldr.clone(), line4_path.clone(), cross_bldr.clone()),
        ];

        for (mut first, second, expected) in params {
            first.extend_from_path(&second);

            assert_eq!(first.data.len(), expected.data.len());
            assert_is_close!(first.start, expected.start);
            assert_is_close!(first.point, expected.point);
            assert_is_close!(first.bounds, expected.bounds);
        }
    }

    #[test]
    fn path_builder_commands() {
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
            assert_is_close!(
                path.bounds,
                Rect::from_origin_and_size(Point::origin(), Vector::splat(2.0))
            );
        }
    }

    #[test]
    fn path_builder_add() {
        let empty = PathBuilder::<Mm>::new();

        let mut line1 = PathBuilder::new();
        line1.abs_line(Point::new(1.0, 1.0));

        let mut line2 = PathBuilder::new();
        line2.abs_move(Point::origin());
        line2.abs_line(Point::new(1.0, 1.0));

        let mut line3 = PathBuilder::new();
        line3.abs_line(Point::new(1.0, 0.0));

        let mut line4 = PathBuilder::new();
        line4.abs_move(Point::new(0.0, 1.0));
        line4.abs_line(Point::new(1.0, 0.0));

        let mut angle = PathBuilder::new();
        angle.abs_line(Point::new(1.0, 1.0));
        angle.abs_move(Point::origin());
        angle.abs_line(Point::new(1.0, 0.0));

        let mut cross = PathBuilder::new();
        cross.abs_line(Point::new(1.0, 1.0));
        cross.abs_move(Point::new(0.0, 1.0));
        cross.abs_line(Point::new(1.0, 0.0));

        #[allow(clippy::redundant_clone)]
        let params = [
            (empty.clone(), empty.clone(), empty.clone()),
            (line1.clone(), empty.clone(), line1.clone()),
            (empty.clone(), line1.clone(), line2.clone()),
            (line1.clone(), line3.clone(), angle.clone()),
            (line1.clone(), line4.clone(), cross.clone()),
        ];

        for (first, second, expected) in params.clone() {
            let result = first + second;

            assert_eq!(result.data.len(), expected.data.len());
            assert_is_close!(result.start, expected.start);
            assert_is_close!(result.point, expected.point);
            assert_is_close!(result.bounds, expected.bounds);
        }

        for (first, second, expected) in params.clone() {
            let result = first + &second;

            assert_eq!(result.data.len(), expected.data.len());
            assert_is_close!(result.start, expected.start);
            assert_is_close!(result.point, expected.point);
            assert_is_close!(result.bounds, expected.bounds);
        }

        for (mut first, second, expected) in params.clone() {
            first += second;

            assert_eq!(first.data.len(), expected.data.len());
            assert_is_close!(first.start, expected.start);
            assert_is_close!(first.point, expected.point);
            assert_is_close!(first.bounds, expected.bounds);
        }

        for (mut first, second, expected) in params {
            first += &second;

            assert_eq!(first.data.len(), expected.data.len());
            assert_is_close!(first.start, expected.start);
            assert_is_close!(first.point, expected.point);
            assert_is_close!(first.bounds, expected.bounds);
        }
    }

    #[test]
    fn path_builder_add_path() {
        let empty_bldr = PathBuilder::<Mm>::new();
        let empty_path = Path {
            data: Box::new([]),
            bounds: Rect::empty(),
        };

        let mut line1_bldr = PathBuilder::new();
        line1_bldr.abs_line(Point::new(1.0, 1.0));

        let line1_path = Path {
            data: Box::new([PathSegment::Line(Vector::new(1.0, 1.0))]),
            bounds: Rect::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0)),
        };

        let mut line2_bldr = PathBuilder::new();
        line2_bldr.abs_move(Point::origin());
        line2_bldr.abs_line(Point::new(1.0, 1.0));

        let line3_path = Path {
            data: Box::new([PathSegment::Line(Vector::new(1.0, 0.0))]),
            bounds: Rect::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0)),
        };

        let line4_path = Path {
            data: Box::new([
                PathSegment::Move(Point::new(0.0, 1.0)),
                PathSegment::Line(Vector::new(1.0, -1.0)),
            ]),
            bounds: Rect::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0)),
        };

        let mut angle_bldr = PathBuilder::new();
        angle_bldr.abs_line(Point::new(1.0, 1.0));
        angle_bldr.abs_move(Point::origin());
        angle_bldr.abs_line(Point::new(1.0, 0.0));

        let mut cross_bldr = PathBuilder::new();
        cross_bldr.abs_line(Point::new(1.0, 1.0));
        cross_bldr.abs_move(Point::new(0.0, 1.0));
        cross_bldr.abs_line(Point::new(1.0, 0.0));

        #[allow(clippy::redundant_clone)]
        let params = [
            (empty_bldr.clone(), empty_path.clone(), empty_bldr.clone()),
            (line1_bldr.clone(), empty_path.clone(), line1_bldr.clone()),
            (empty_bldr.clone(), line1_path.clone(), line2_bldr.clone()),
            (line1_bldr.clone(), line3_path.clone(), angle_bldr.clone()),
            (line1_bldr.clone(), line4_path.clone(), cross_bldr.clone()),
        ];

        for (first, second, expected) in params.clone() {
            let result = first + second;

            assert_eq!(result.data.len(), expected.data.len());
            assert_is_close!(result.start, expected.start);
            assert_is_close!(result.point, expected.point);
            assert_is_close!(result.bounds, expected.bounds);
        }

        for (first, second, expected) in params.clone() {
            let result = first + &second;

            assert_eq!(result.data.len(), expected.data.len());
            assert_is_close!(result.start, expected.start);
            assert_is_close!(result.point, expected.point);
            assert_is_close!(result.bounds, expected.bounds);
        }

        for (mut first, second, expected) in params.clone() {
            first += second;

            assert_eq!(first.data.len(), expected.data.len());
            assert_is_close!(first.start, expected.start);
            assert_is_close!(first.point, expected.point);
            assert_is_close!(first.bounds, expected.bounds);
        }

        for (mut first, second, expected) in params {
            first += &second;

            assert_eq!(first.data.len(), expected.data.len());
            assert_is_close!(first.start, expected.start);
            assert_is_close!(first.point, expected.point);
            assert_is_close!(first.bounds, expected.bounds);
        }
    }

    #[test]
    fn path_builder_mul() {
        let path = PathBuilder::<Mm> {
            data: vec![
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::new(1.0, 1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::new(2.0, 2.0),
                ),
                PathSegment::Close,
            ],
            bounds: Rect::new(Point::origin(), Point::new(3.0, 3.0)),
            start: Point::new(1.0, 2.0),
            point: Point::new(3.0, 4.0),
        };

        let path2 = path.clone() * 2.0;

        assert_eq!(path2.data.len(), path.data.len());
        assert_is_close!(
            path2.bounds,
            Rect::new(Point::new(0.0, 0.0), Point::new(6.0, 6.0))
        );
        assert_is_close!(path2.start, Point::new(2.0, 4.0));
        assert_is_close!(path2.point, Point::new(6.0, 8.0));
        for (&p1, &p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1 * 2.0, p2);
        }

        let mut path2 = path.clone();
        path2 *= 2.0;

        assert_eq!(path2.data.len(), path.data.len());
        assert_is_close!(
            path2.bounds,
            Rect::new(Point::new(0.0, 0.0), Point::new(6.0, 6.0))
        );
        assert_is_close!(path2.start, Point::new(2.0, 4.0));
        assert_is_close!(path2.point, Point::new(6.0, 8.0));
        for (&p1, &p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1 * 2.0, p2);
        }
    }

    #[test]
    fn path_builder_div() {
        let path = PathBuilder::<Mm> {
            data: vec![
                PathSegment::Move(Point::origin()),
                PathSegment::Line(Vector::new(1.0, 1.0)),
                PathSegment::CubicBezier(
                    Vector::new(1.0, 0.0),
                    Vector::new(2.0, 1.0),
                    Vector::new(2.0, 2.0),
                ),
                PathSegment::Close,
            ],
            bounds: Rect::new(Point::origin(), Point::new(3.0, 3.0)),
            start: Point::new(1.0, 2.0),
            point: Point::new(3.0, 4.0),
        };

        let path2 = path.clone() / 2.0;

        assert_eq!(path2.data.len(), path.data.len());
        assert_is_close!(
            path2.bounds,
            Rect::new(Point::new(0.0, 0.0), Point::new(1.5, 1.5))
        );
        assert_is_close!(path2.start, Point::new(0.5, 1.0));
        assert_is_close!(path2.point, Point::new(1.5, 2.0));
        for (&p1, &p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1 / 2.0, p2);
        }

        let mut path2 = path.clone();
        path2 /= 2.0;

        assert_eq!(path2.data.len(), path.data.len());
        assert_is_close!(
            path2.bounds,
            Rect::new(Point::new(0.0, 0.0), Point::new(1.5, 1.5))
        );
        assert_is_close!(path2.start, Point::new(0.5, 1.0));
        assert_is_close!(path2.point, Point::new(1.5, 2.0));
        for (&p1, &p2) in path.data.iter().zip(path2.data.iter()) {
            assert_is_close!(p1 / 2.0, p2);
        }
    }

    #[test]
    fn test_calculate_bounds() {
        let data = vec![];
        let bounds = calculate_bounds::<Mm>(&data);

        assert_is_close!(bounds, Rect::empty());

        let data = vec![
            PathSegment::Move(Point::new(1.0, -1.0)),
            PathSegment::Line(Vector::new(1.0, 1.0)),
            PathSegment::CubicBezier(
                Vector::new(1.0, 0.0),
                Vector::new(2.0, 1.0),
                Vector::new(2.0, 2.0),
            ),
            PathSegment::Close,
        ];
        let bounds = calculate_bounds::<Mm>(&data);

        assert_is_close!(
            bounds,
            Rect::new(Point::new(1.0, -1.0), Point::new(4.0, 2.0))
        );
    }
}
