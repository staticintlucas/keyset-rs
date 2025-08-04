use std::ops;

use isclose::IsClose;

use crate::{
    Conversion, ConvertFrom, ConvertInto as _, Path, PathSegment, Point, Rotate, Scale, Transform,
    Translate, Unit, Vector,
};

/// A 2 dimensional rectangle
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect<U: Unit> {
    /// The minimum pont (top left corner) of the rectangle
    pub min: Point<U>,
    /// The maximum pont (bottom right corner) of the rectangle
    pub max: Point<U>,
}

impl<U> Rect<U>
where
    U: Unit,
{
    /// Create a new rectangle with the given minimum and maximum points
    #[inline]
    pub const fn new(min: Point<U>, max: Point<U>) -> Self {
        Self { min, max }
    }

    /// Create a new rectangle with the given origin point and size vector
    #[inline]
    pub fn from_origin_and_size(origin: Point<U>, size: Vector<U>) -> Self {
        Self {
            min: origin,
            max: origin + size,
        }
    }

    /// Create a new rectangle with the given center point and size vector
    #[inline]
    pub fn from_center_and_size(center: Point<U>, size: Vector<U>) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    /// Create a new empty rectangle
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            min: Point::origin(),
            max: Point::origin(),
        }
    }

    /// Returns the size of the rectangle
    #[inline]
    pub fn size(&self) -> Vector<U> {
        self.max - self.min
    }

    /// Returns the width of the rectangle
    #[inline]
    pub fn width(&self) -> U {
        self.max.x - self.min.x
    }

    /// Returns the height of the rectangle
    #[inline]
    pub fn height(&self) -> U {
        self.max.y - self.min.y
    }

    /// Returns the center point of the rectangle
    #[inline]
    pub fn center(&self) -> Point<U> {
        self.min.lerp(self.max, 0.5)
    }

    /// Returns the union of two rectangles
    #[inline]
    #[must_use]
    pub fn union(self, rhs: Self) -> Self {
        Self {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }

    /// Converts the rectangle to a [`Path`]
    #[inline]
    pub fn to_path(self) -> Path<U> {
        Path {
            data: Box::new([
                PathSegment::Move(self.min),
                PathSegment::Line(Vector::new(self.width(), U::zero())),
                PathSegment::Line(Vector::new(U::zero(), self.height())),
                PathSegment::Line(Vector::new(-self.width(), U::zero())),
                PathSegment::Close,
            ]),
            bounds: self,
        }
    }
}

impl<U, V> ConvertFrom<Rect<V>> for Rect<U>
where
    U: Unit + ConvertFrom<V>,
    V: Unit,
{
    #[inline]
    fn convert_from(value: Rect<V>) -> Self {
        Self {
            min: value.min.convert_into(),
            max: value.max.convert_into(),
        }
    }
}

impl<U> ops::Add<OffsetRect<U>> for Rect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: OffsetRect<U>) -> Self::Output {
        Self {
            min: self.min - rhs.min,
            max: self.max + rhs.max,
        }
    }
}

impl<U> ops::AddAssign<OffsetRect<U>> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: OffsetRect<U>) {
        self.min -= rhs.min;
        self.max += rhs.max;
    }
}

impl<U> ops::Sub<OffsetRect<U>> for Rect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: OffsetRect<U>) -> Self::Output {
        Self {
            min: self.min + rhs.min,
            max: self.max - rhs.max,
        }
    }
}

impl<U> ops::SubAssign<OffsetRect<U>> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: OffsetRect<U>) {
        self.min += rhs.min;
        self.max -= rhs.max;
    }
}

impl<U> ops::Sub<Self> for Rect<U>
where
    U: Unit,
{
    type Output = OffsetRect<U>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        OffsetRect {
            min: rhs.min - self.min,
            max: self.max - rhs.max,
        }
    }
}

impl<U> ops::Mul<f32> for Rect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.min *= rhs;
        self.max *= rhs;
    }
}

impl<U> ops::Div<f32> for Rect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

impl<U> ops::DivAssign<f32> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.min /= rhs;
        self.max /= rhs;
    }
}

impl<U> IsClose for Rect<U>
where
    U: Unit,
{
    type Tolerance = f32;
    const ZERO_TOL: Self::Tolerance = 0.0;
    const ABS_TOL: Self::Tolerance = <U as IsClose>::ABS_TOL;
    const REL_TOL: Self::Tolerance = <U as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.min.is_close_tol(&other.min, rel_tol, abs_tol)
            && self.max.is_close_tol(&other.max, rel_tol, abs_tol)
    }
}

impl<U> ops::Mul<Rotate> for Rect<U>
where
    U: Unit,
{
    type Output = Path<U>;

    #[inline]
    fn mul(self, rhs: Rotate) -> Self::Output {
        self.to_path() * rhs
    }
}

impl<U> ops::Mul<Scale> for Rect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Scale) -> Self::Output {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl<U> ops::MulAssign<Scale> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Scale) {
        self.min *= rhs;
        self.max *= rhs;
    }
}

impl<U> ops::Div<Scale> for Rect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Scale) -> Self::Output {
        Self {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

impl<U> ops::DivAssign<Scale> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: Scale) {
        self.min /= rhs;
        self.max /= rhs;
    }
}

impl<U> ops::Mul<Translate<U>> for Rect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Translate<U>) -> Self::Output {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl<U> ops::MulAssign<Translate<U>> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Translate<U>) {
        self.min *= rhs;
        self.max *= rhs;
    }
}

impl<U> ops::Mul<Transform<U>> for Rect<U>
where
    U: Unit,
{
    type Output = Path<U>;

    #[inline]
    fn mul(self, rhs: Transform<U>) -> Self::Output {
        self.to_path() * rhs
    }
}

impl<Dst, Src> ops::Mul<Conversion<Dst, Src>> for Rect<Src>
where
    Dst: Unit,
    Src: Unit,
{
    type Output = Path<Dst>;

    #[inline]
    fn mul(self, rhs: Conversion<Dst, Src>) -> Self::Output {
        self.to_path() * rhs
    }
}

/// A 2 dimensional rectangle with rounded corners
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoundRect<U: Unit> {
    /// The minimum point (top left corner) of the rectangle
    pub min: Point<U>,
    /// The maximum point (bottom right corner) of the rectangle
    pub max: Point<U>,
    /// The radii for the corners of the rectangle
    pub radii: Vector<U>,
}

impl<U> RoundRect<U>
where
    U: Unit,
{
    /// Create a new rounded rectangle with the given minimum and maximum points and corner radii
    #[inline]
    pub const fn new(min: Point<U>, max: Point<U>, radii: Vector<U>) -> Self {
        Self { min, max, radii }
    }

    /// Create a new rounded rectangle with the given origin point and size vector and corner radii
    #[inline]
    pub fn from_origin_size_and_radii(origin: Point<U>, size: Vector<U>, radii: Vector<U>) -> Self {
        Self {
            min: origin,
            max: origin + size,
            radii,
        }
    }

    /// Create a new rounded rectangle with the given center point and size vector and corner radii
    #[inline]
    pub fn from_center_size_and_radii(center: Point<U>, size: Vector<U>, radii: Vector<U>) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
            radii,
        }
    }

    /// Create a new rounded rectangle from an existing rectangle and corner radii
    #[inline]
    pub const fn from_rect_and_radii(rect: Rect<U>, radii: Vector<U>) -> Self {
        Self {
            min: rect.min,
            max: rect.max,
            radii,
        }
    }

    /// Returns an equivalent rectangle without rounded corners
    #[inline]
    pub const fn to_rect(&self) -> Rect<U> {
        Rect {
            min: self.min,
            max: self.max,
        }
    }

    /// Returns the size of the rectangle
    #[inline]
    pub fn size(&self) -> Vector<U> {
        self.max - self.min
    }

    /// Returns the width of the rectangle
    #[inline]
    pub fn width(&self) -> U {
        self.max.x - self.min.x
    }

    /// Returns the height of the rectangle
    #[inline]
    pub fn height(&self) -> U {
        self.max.y - self.min.y
    }

    /// Returns the center point of the rectangle
    #[inline]
    pub fn center(&self) -> Point<U> {
        self.min.lerp(self.max, 0.5)
    }

    /// Converts the rectangle to a [`Path`]
    #[inline]
    pub fn to_path(self) -> Path<U> {
        const A: f32 = (4.0 / 3.0) * (std::f32::consts::SQRT_2 - 1.0);

        let (mx, my) = (self.min.x, self.min.y);
        let (rx, ry) = (self.radii.x, self.radii.y);

        Path {
            data: Box::new([
                PathSegment::Move(Point::new(mx, my + ry)),
                PathSegment::CubicBezier(
                    Vector::new(U::zero(), -ry * A),
                    Vector::new(rx * (1.0 - A), -ry),
                    Vector::new(rx, -ry),
                ),
                PathSegment::Line(Vector::new(self.width() - rx * 2.0, U::zero())),
                PathSegment::CubicBezier(
                    Vector::new(rx * A, U::zero()),
                    Vector::new(rx, ry * (1.0 - A)),
                    Vector::new(rx, ry),
                ),
                PathSegment::Line(Vector::new(U::zero(), self.height() - ry * 2.0)),
                PathSegment::CubicBezier(
                    Vector::new(U::zero(), ry * A),
                    Vector::new(-rx * (1.0 - A), ry),
                    Vector::new(-rx, ry),
                ),
                PathSegment::Line(Vector::new(-(self.width() - rx * 2.0), U::zero())),
                PathSegment::CubicBezier(
                    Vector::new(-rx * A, U::zero()),
                    Vector::new(-rx, -ry * (1.0 - A)),
                    Vector::new(-rx, -ry),
                ),
                PathSegment::Close,
            ]),
            bounds: self.to_rect(),
        }
    }
}

impl<U, V> ConvertFrom<RoundRect<V>> for RoundRect<U>
where
    U: Unit + ConvertFrom<V>,
    V: Unit,
{
    #[inline]
    fn convert_from(value: RoundRect<V>) -> Self {
        Self {
            min: value.min.convert_into(),
            max: value.max.convert_into(),
            radii: value.radii.convert_into(),
        }
    }
}

impl<U> ops::Mul<f32> for RoundRect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
            radii: self.radii * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for RoundRect<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.min *= rhs;
        self.max *= rhs;
        self.radii *= rhs;
    }
}

impl<U> ops::Div<f32> for RoundRect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            min: self.min / rhs,
            max: self.max / rhs,
            radii: self.radii / rhs,
        }
    }
}

impl<U> ops::DivAssign<f32> for RoundRect<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.min /= rhs;
        self.max /= rhs;
        self.radii /= rhs;
    }
}

impl<U> IsClose for RoundRect<U>
where
    U: Unit,
{
    type Tolerance = f32;
    const ZERO_TOL: Self::Tolerance = 0.0;
    const ABS_TOL: Self::Tolerance = <U as IsClose>::ABS_TOL;
    const REL_TOL: Self::Tolerance = <U as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.min.is_close_tol(&other.min, rel_tol, abs_tol)
            && self.max.is_close_tol(&other.max, rel_tol, abs_tol)
            && self.radii.is_close_tol(&other.radii, rel_tol, abs_tol)
    }
}

impl<U> ops::Mul<Rotate> for RoundRect<U>
where
    U: Unit,
{
    type Output = Path<U>;

    #[inline]
    fn mul(self, rhs: Rotate) -> Self::Output {
        self.to_path() * rhs
    }
}

impl<U> ops::Mul<Scale> for RoundRect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Scale) -> Self::Output {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
            radii: self.radii * rhs,
        }
    }
}

impl<U> ops::MulAssign<Scale> for RoundRect<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Scale) {
        self.min *= rhs;
        self.max *= rhs;
        self.radii *= rhs;
    }
}

impl<U> ops::Div<Scale> for RoundRect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Scale) -> Self::Output {
        Self {
            min: self.min / rhs,
            max: self.max / rhs,
            radii: self.radii / rhs,
        }
    }
}

impl<U> ops::DivAssign<Scale> for RoundRect<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: Scale) {
        self.min /= rhs;
        self.max /= rhs;
        self.radii /= rhs;
    }
}

impl<U> ops::Mul<Translate<U>> for RoundRect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Translate<U>) -> Self::Output {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
            radii: self.radii * rhs,
        }
    }
}

impl<U> ops::MulAssign<Translate<U>> for RoundRect<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Translate<U>) {
        self.min *= rhs;
        self.max *= rhs;
        self.radii *= rhs;
    }
}

impl<U> ops::Mul<Transform<U>> for RoundRect<U>
where
    U: Unit,
{
    type Output = Path<U>;

    #[inline]
    fn mul(self, rhs: Transform<U>) -> Self::Output {
        self.to_path() * rhs
    }
}

impl<Dst, Src> ops::Mul<Conversion<Dst, Src>> for RoundRect<Src>
where
    Dst: Unit,
    Src: Unit,
{
    type Output = Path<Dst>;

    #[inline]
    fn mul(self, rhs: Conversion<Dst, Src>) -> Self::Output {
        self.to_path() * rhs
    }
}

/// A set of offsets for the top, right, bottom, and left sides of a rectangle
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OffsetRect<U: Unit> {
    pub(crate) min: Vector<U>,
    pub(crate) max: Vector<U>,
}

impl<U> OffsetRect<U>
where
    U: Unit,
{
    /// Create a new offset rectangle with the given offsets
    #[inline]
    #[must_use]
    pub const fn new(top: U, right: U, bottom: U, left: U) -> Self {
        Self {
            min: Vector::new(left, top),
            max: Vector::new(right, bottom),
        }
    }

    /// Create a new offset rectangle with the same offset
    #[inline]
    #[must_use]
    pub const fn splat(value: U) -> Self {
        Self {
            min: Vector::splat(value),
            max: Vector::splat(value),
        }
    }

    /// Create a new offset rectangle with the given offsets
    #[inline]
    #[must_use]
    pub fn zero() -> Self {
        Self {
            min: Vector::zero(),
            max: Vector::zero(),
        }
    }
}

impl<U, V> ConvertFrom<OffsetRect<V>> for OffsetRect<U>
where
    U: Unit + ConvertFrom<V>,
    V: Unit,
{
    #[inline]
    fn convert_from(value: OffsetRect<V>) -> Self {
        Self {
            min: value.min.convert_into(),
            max: value.max.convert_into(),
        }
    }
}

impl<U> ops::Add for OffsetRect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            min: self.min + rhs.min,
            max: self.max + rhs.max,
        }
    }
}

impl<U> ops::AddAssign for OffsetRect<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.min += rhs.min;
        self.max += rhs.max;
    }
}

impl<U> ops::Sub for OffsetRect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            min: self.min - rhs.min,
            max: self.max - rhs.max,
        }
    }
}

impl<U> ops::SubAssign for OffsetRect<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.min -= rhs.min;
        self.max -= rhs.max;
    }
}

impl<U> ops::Mul<f32> for OffsetRect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for OffsetRect<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.min *= rhs;
        self.max *= rhs;
    }
}

impl<U> ops::Div<f32> for OffsetRect<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

impl<U> ops::DivAssign<f32> for OffsetRect<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.min /= rhs;
        self.max /= rhs;
    }
}

impl<U> IsClose for OffsetRect<U>
where
    U: Unit,
{
    type Tolerance = f32;
    const ZERO_TOL: Self::Tolerance = 0.0;
    const ABS_TOL: Self::Tolerance = <U as IsClose>::ABS_TOL;
    const REL_TOL: Self::Tolerance = <U as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.min.is_close_tol(&other.min, rel_tol, abs_tol)
            && self.max.is_close_tol(&other.max, rel_tol, abs_tol)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{declare_units, Angle, Inch, Mm, PathBuilder};

    use super::*;

    #[test]
    fn rect_new() {
        let rect = Rect::new(Point::new(Mm(0.0), Mm(1.0)), Point::new(Mm(2.0), Mm(4.0)));
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Point::new(Mm(2.0), Mm(4.0)));
    }

    #[test]
    fn rect_from_origin_and_size() {
        let rect =
            Rect::from_origin_and_size(Point::new(Mm(0.0), Mm(1.0)), Vector::new(Mm(2.0), Mm(3.0)));
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Point::new(Mm(2.0), Mm(4.0)));
    }

    #[test]
    fn rect_from_center_and_size() {
        let rect =
            Rect::from_center_and_size(Point::new(Mm(1.0), Mm(2.5)), Vector::new(Mm(2.0), Mm(3.0)));
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Point::new(Mm(2.0), Mm(4.0)));
    }

    #[test]
    fn rect_empty() {
        let rect = Rect::empty();
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(0.0)));
        assert_is_close!(rect.max, Point::new(Mm(0.0), Mm(0.0)));
    }

    #[test]
    fn rect_size() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        assert_is_close!(rect.size(), Vector::new(Mm(2.0), Mm(3.0)));
    }

    #[test]
    fn rect_width() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        assert_is_close!(rect.width(), Mm(2.0));
    }

    #[test]
    fn rect_height() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        assert_is_close!(rect.height(), Mm(3.0));
    }

    #[test]
    fn rect_center() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        assert_is_close!(rect.center(), Point::new(Mm(1.0), Mm(2.5)));
    }

    #[test]
    fn rect_union() {
        let rect1 = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        let rect2 = Rect {
            min: Point::new(Mm(0.2), Mm(0.5)),
            max: Point::new(Mm(1.0), Mm(6.5)),
        };
        assert_is_close!(rect1.union(rect2).min, Point::new(Mm(0.0), Mm(0.5)));
        assert_is_close!(rect1.union(rect2).max, Point::new(Mm(2.0), Mm(6.5)));
    }

    #[test]
    fn rect_to_path() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        let mut builder = PathBuilder::new();
        builder.abs_move(Point::new(Mm(0.0), Mm(1.0)));
        builder.rel_horiz_line(Mm(2.0));
        builder.rel_vert_line(Mm(3.0));
        builder.rel_horiz_line(Mm(-2.0));
        builder.close();
        let expected = builder.build();

        assert_eq!(rect.to_path().len(), expected.len());
        assert_is_close!(rect.to_path().bounds, expected.bounds);
        for (&res, &exp) in rect.to_path().iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn rect_convert_from() {
        let rect = Rect::<Mm>::convert_from(Rect {
            min: Point::new(Inch(0.0), Inch(0.75)),
            max: Point::new(Inch(1.0), Inch(1.5)),
        });
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(19.05)));
        assert_is_close!(rect.max, Point::new(Mm(25.4), Mm(38.1)));
    }

    #[test]
    fn rect_add_offset_rect() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        } + OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        };
        assert_is_close!(rect.min, Point::new(Mm(-0.5), Mm(-1.0)));
        assert_is_close!(rect.max, Point::new(Mm(3.5), Mm(5.0)));
    }

    #[test]
    fn rect_add_assign_offset_rect() {
        let mut rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        rect += OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        };
        assert_is_close!(rect.min, Point::new(Mm(-0.5), Mm(-1.0)));
        assert_is_close!(rect.max, Point::new(Mm(3.5), Mm(5.0)));
    }

    #[test]
    fn rect_sub_offset_rect() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        } - OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        };
        assert_is_close!(rect.min, Point::new(Mm(0.5), Mm(3.0)));
        assert_is_close!(rect.max, Point::new(Mm(0.5), Mm(3.0)));
    }

    #[test]
    fn rect_sub_assign_offset_rect() {
        let mut rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        rect -= OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        };
        assert_is_close!(rect.min, Point::new(Mm(0.5), Mm(3.0)));
        assert_is_close!(rect.max, Point::new(Mm(0.5), Mm(3.0)));
    }

    #[test]
    fn rect_mul_f32() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        } * 1.5;
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.5)));
        assert_is_close!(rect.max, Point::new(Mm(3.0), Mm(6.0)));

        // TODO: see comment by Unit
        // let rect = 1.5 * Rect {
        //     min: Point::new(Mm(0.0), Mm(1.0)),
        //     max: Point::new(Mm(2.0), Mm(4.0)),
        // };
        // assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.5)));
        // assert_is_close!(rect.max, Point::new(Mm(3.0), Mm(6.0)));
    }

    #[test]
    fn rect_mul_assign_f32() {
        let mut rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        rect *= 1.5;
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.5)));
        assert_is_close!(rect.max, Point::new(Mm(3.0), Mm(6.0)));
    }

    #[test]
    fn rect_div_f32() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        } / 1.5;
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(2.0 / 3.0)));
        assert_is_close!(rect.max, Point::new(Mm(4.0 / 3.0), Mm(8.0 / 3.0)));
    }

    #[test]
    fn rect_div_assign_f32() {
        let mut rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        rect /= 1.5;
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(2.0 / 3.0)));
        assert_is_close!(rect.max, Point::new(Mm(4.0 / 3.0), Mm(8.0 / 3.0)));
    }

    #[test]
    fn rect_is_close() {
        assert!(Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        }
        .is_close(&Rect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
        }));
        assert!(!Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        }
        .is_close(&Rect {
            min: Point::new(Mm(0.1), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
        }));
        assert!(!Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        }
        .is_close(&Rect {
            min: Point::new(Mm(0.0), Mm(2.1)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
        }));
        assert!(!Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        }
        .is_close(&Rect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.1), Mm(2.0)) * 2.0,
        }));
        assert!(!Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        }
        .is_close(&Rect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.1)) * 2.0,
        }));
    }

    #[test]
    fn rect_rotate() {
        use std::f32::consts::SQRT_2;

        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        let rotate = Rotate::degrees(135.0);
        let path = rect * rotate;

        let mut exp_bldr = PathBuilder::new();
        exp_bldr.abs_move(Point::new(Mm(-0.5 * SQRT_2), Mm(-0.5 * SQRT_2)));
        exp_bldr.rel_line(Vector::new(Mm(-SQRT_2), Mm(SQRT_2)));
        exp_bldr.rel_line(Vector::new(Mm(-1.5 * SQRT_2), Mm(-1.5 * SQRT_2)));
        exp_bldr.rel_line(Vector::new(Mm(SQRT_2), Mm(-SQRT_2)));
        exp_bldr.close();
        let expected = exp_bldr.build();

        assert_eq!(path.len(), expected.len());
        assert_is_close!(path.bounds, expected.bounds);
        for (&res, &exp) in path.iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn rect_scale() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        } * Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(0.5)));
        assert_is_close!(rect.max, Point::new(Mm(4.0), Mm(2.0)));

        let mut rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        rect *= Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(0.5)));
        assert_is_close!(rect.max, Point::new(Mm(4.0), Mm(2.0)));

        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        } / Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(2.0)));
        assert_is_close!(rect.max, Point::new(Mm(1.0), Mm(8.0)));

        let mut rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        rect /= Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(2.0)));
        assert_is_close!(rect.max, Point::new(Mm(1.0), Mm(8.0)));
    }

    #[test]
    fn rect_translate() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        } * Translate::new(Mm(2.0), Mm(-1.0));

        assert_is_close!(rect.min, Point::new(Mm(2.0), Mm(0.0)));
        assert_is_close!(rect.max, Point::new(Mm(4.0), Mm(3.0)));

        let mut rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        rect *= Translate::new(Mm(2.0), Mm(-1.0));

        assert_is_close!(rect.min, Point::new(Mm(2.0), Mm(0.0)));
        assert_is_close!(rect.max, Point::new(Mm(4.0), Mm(3.0)));
    }

    #[test]
    fn rect_transform() {
        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        let transform = Transform::new(1.0, 0.5, Mm(-1.0), -0.5, 1.5, Mm(2.0));
        let path = rect * transform;

        let mut exp_bldr = PathBuilder::new();
        exp_bldr.abs_move(Point::new(Mm(-0.5), Mm(3.5)));
        exp_bldr.rel_line(Vector::new(Mm(2.0), Mm(-1.0)));
        exp_bldr.rel_line(Vector::new(Mm(1.5), Mm(4.5)));
        exp_bldr.rel_line(Vector::new(Mm(-2.0), Mm(1.0)));
        exp_bldr.close();
        let expected = exp_bldr.build();

        assert_eq!(path.len(), expected.len());
        assert_is_close!(path.bounds, expected.bounds);
        for (&res, &exp) in path.iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn rect_convert() {
        declare_units! {
            Test = 1.0;
        }

        let rect = Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        };
        let conv = Conversion::<Test, Mm>::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);
        let path = rect * conv;

        let mut exp_bldr = PathBuilder::new();
        exp_bldr.abs_move(Point::new(Test(-0.5), Test(3.5)));
        exp_bldr.rel_line(Vector::new(Test(2.0), Test(-1.0)));
        exp_bldr.rel_line(Vector::new(Test(1.5), Test(4.5)));
        exp_bldr.rel_line(Vector::new(Test(-2.0), Test(1.0)));
        exp_bldr.close();
        let expected = exp_bldr.build();

        assert_eq!(path.len(), expected.len());
        assert_is_close!(path.bounds, expected.bounds);
        for (&res, &exp) in path.iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn round_rect_new() {
        let rect = RoundRect::new(
            Point::new(Mm(0.0), Mm(1.0)),
            Point::new(Mm(2.0), Mm(4.0)),
            Vector::new(Mm(0.5), Mm(1.0)),
        );
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Point::new(Mm(2.0), Mm(4.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.5), Mm(1.0)));
    }

    #[test]
    fn round_rect_from_origin_size_and_radii() {
        let rect = RoundRect::from_origin_size_and_radii(
            Point::new(Mm(0.0), Mm(1.0)),
            Vector::new(Mm(2.0), Mm(3.0)),
            Vector::new(Mm(0.5), Mm(1.0)),
        );
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Point::new(Mm(2.0), Mm(4.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.5), Mm(1.0)));
    }

    #[test]
    fn round_rect_from_center_size_and_radii() {
        let rect = RoundRect::from_center_size_and_radii(
            Point::new(Mm(1.0), Mm(2.5)),
            Vector::new(Mm(2.0), Mm(3.0)),
            Vector::new(Mm(0.5), Mm(1.0)),
        );
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Point::new(Mm(2.0), Mm(4.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.5), Mm(1.0)));
    }

    #[test]
    fn round_rect_from_rect_and_radii() {
        let rect = RoundRect::from_rect_and_radii(
            Rect::new(Point::new(Mm(0.0), Mm(1.0)), Point::new(Mm(2.0), Mm(4.0))),
            Vector::new(Mm(0.5), Mm(1.0)),
        );
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Point::new(Mm(2.0), Mm(4.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.5), Mm(1.0)));
    }

    #[test]
    fn round_rect_to_rect() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .to_rect();
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Point::new(Mm(2.0), Mm(4.0)));
    }

    #[test]
    fn round_rect_size() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        assert_is_close!(rect.size(), Vector::new(Mm(2.0), Mm(3.0)));
    }

    #[test]
    fn round_rect_width() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        assert_is_close!(rect.width(), Mm(2.0));
    }

    #[test]
    fn round_rect_height() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        assert_is_close!(rect.height(), Mm(3.0));
    }

    #[test]
    fn round_rect_center() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        assert_is_close!(rect.center(), Point::new(Mm(1.0), Mm(2.5)));
    }

    #[test]
    fn round_rect_to_path() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        let mut builder = PathBuilder::new();
        builder.abs_move(Point::new(Mm(0.0), Mm(2.0)));
        builder.rel_arc(
            Vector::new(Mm(0.5), Mm(1.0)),
            Angle::ZERO,
            false,
            true,
            Vector::new(Mm(0.5), Mm(-1.0)),
        );
        builder.rel_horiz_line(Mm(1.0));
        builder.rel_arc(
            Vector::new(Mm(0.5), Mm(1.0)),
            Angle::ZERO,
            false,
            true,
            Vector::new(Mm(0.5), Mm(1.0)),
        );
        builder.rel_vert_line(Mm(1.0));
        builder.rel_arc(
            Vector::new(Mm(0.5), Mm(1.0)),
            Angle::ZERO,
            false,
            true,
            Vector::new(Mm(-0.5), Mm(1.0)),
        );
        builder.rel_horiz_line(Mm(-1.0));
        builder.rel_arc(
            Vector::new(Mm(0.5), Mm(1.0)),
            Angle::ZERO,
            false,
            true,
            Vector::new(Mm(-0.5), Mm(-1.0)),
        );
        builder.close();
        let expected = builder.build();

        assert_eq!(rect.to_path().len(), expected.len());
        assert_is_close!(rect.to_path().bounds, expected.bounds);
        for (&res, &exp) in rect.to_path().iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn round_rect_convert_from() {
        let rect = RoundRect::<Mm>::convert_from(RoundRect {
            min: Point::new(Inch(0.0), Inch(0.75)),
            max: Point::new(Inch(1.0), Inch(1.5)),
            radii: Vector::new(Inch(0.25), Inch(0.5)),
        });
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(19.05)));
        assert_is_close!(rect.max, Point::new(Mm(25.4), Mm(38.1)));
        assert_is_close!(rect.radii, Vector::new(Mm(6.35), Mm(12.7)));
    }

    #[test]
    fn round_rect_mul_f32() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        } * 1.5;
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.5)));
        assert_is_close!(rect.max, Point::new(Mm(3.0), Mm(6.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.75), Mm(1.5)));

        // TODO: see comment by Unit
        // let rect = 1.5 * RoundRect {
        //     min: Point::new(Mm(0.0), Mm(1.0)),
        //     max: Point::new(Mm(2.0), Mm(4.0)),
        //     radii: Vector::new(Mm(0.5), Mm(1.0)),
        // };
        // assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.5)));
        // assert_is_close!(rect.max, Point::new(Mm(3.0), Mm(6.0)));
        // assert_is_close!(rect.radii, Vector::new(Mm(0.75), Mm(1.5)));
    }

    #[test]
    fn round_rect_mul_assign_f32() {
        let mut rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        rect *= 1.5;
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(1.5)));
        assert_is_close!(rect.max, Point::new(Mm(3.0), Mm(6.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.75), Mm(1.5)));
    }

    #[test]
    fn round_rect_div_f32() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        } / 1.5;
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(2.0 / 3.0)));
        assert_is_close!(rect.max, Point::new(Mm(4.0 / 3.0), Mm(8.0 / 3.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(1.0 / 3.0), Mm(2.0 / 3.0)));
    }

    #[test]
    fn round_rect_div_assign_f32() {
        let mut rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        rect /= 1.5;
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(2.0 / 3.0)));
        assert_is_close!(rect.max, Point::new(Mm(4.0 / 3.0), Mm(8.0 / 3.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(1.0 / 3.0), Mm(2.0 / 3.0)));
    }

    #[test]
    fn round_rect_is_close() {
        assert!(RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(&RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(&RoundRect {
            min: Point::new(Mm(0.1), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(&RoundRect {
            min: Point::new(Mm(0.0), Mm(2.1)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(&RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.1), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(&RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.1)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(&RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.6), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(&RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.1)) / 3.0,
        }));
    }

    #[test]
    fn round_rect_rotate() {
        use std::f32::consts::SQRT_2;

        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        let rotate = Rotate::degrees(135.0);
        let path = rect * rotate;

        let mut exp_bldr = PathBuilder::new();
        exp_bldr.abs_move(Point::new(Mm(-SQRT_2), Mm(-SQRT_2)));
        exp_bldr.rel_arc(
            Vector::new(Mm(0.5), Mm(1.0)),
            Angle::degrees(135.0),
            false,
            true,
            Vector::new(Mm(0.25 * SQRT_2), Mm(0.75 * SQRT_2)),
        );
        exp_bldr.rel_line(Vector::new(Mm(-0.5 * SQRT_2), Mm(0.5 * SQRT_2)));
        exp_bldr.rel_arc(
            Vector::new(Mm(0.5), Mm(1.0)),
            Angle::degrees(135.0),
            false,
            true,
            Vector::new(Mm(-0.75 * SQRT_2), Mm(-0.25 * SQRT_2)),
        );
        exp_bldr.rel_line(Vector::new(Mm(-0.5 * SQRT_2), Mm(-0.5 * SQRT_2)));
        exp_bldr.rel_arc(
            Vector::new(Mm(0.5), Mm(1.0)),
            Angle::degrees(135.0),
            false,
            true,
            Vector::new(Mm(-0.25 * SQRT_2), Mm(-0.75 * SQRT_2)),
        );
        exp_bldr.rel_line(Vector::new(Mm(0.5 * SQRT_2), Mm(-0.5 * SQRT_2)));
        exp_bldr.rel_arc(
            Vector::new(Mm(0.5), Mm(1.0)),
            Angle::degrees(135.0),
            false,
            true,
            Vector::new(Mm(0.75 * SQRT_2), Mm(0.25 * SQRT_2)),
        );
        exp_bldr.close();
        let expected = exp_bldr.build();

        assert_eq!(path.len(), expected.len());
        assert_is_close!(path.bounds, expected.bounds);
        for (&res, &exp) in path.iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn round_rect_scale() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        } * Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(0.5)));
        assert_is_close!(rect.max, Point::new(Mm(4.0), Mm(2.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(1.0), Mm(0.5)));

        let mut rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        rect *= Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(0.5)));
        assert_is_close!(rect.max, Point::new(Mm(4.0), Mm(2.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(1.0), Mm(0.5)));

        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        } / Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(2.0)));
        assert_is_close!(rect.max, Point::new(Mm(1.0), Mm(8.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.25), Mm(2.0)));

        let mut rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        rect /= Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(2.0)));
        assert_is_close!(rect.max, Point::new(Mm(1.0), Mm(8.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.25), Mm(2.0)));
    }

    #[test]
    fn round_rect_translate() {
        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        } * Translate::new(Mm(2.0), Mm(-1.0));

        assert_is_close!(rect.min, Point::new(Mm(2.0), Mm(0.0)));
        assert_is_close!(rect.max, Point::new(Mm(4.0), Mm(3.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.5), Mm(1.0)));

        let mut rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        rect *= Translate::new(Mm(2.0), Mm(-1.0));

        assert_is_close!(rect.min, Point::new(Mm(2.0), Mm(0.0)));
        assert_is_close!(rect.max, Point::new(Mm(4.0), Mm(3.0)));
        assert_is_close!(rect.radii, Vector::new(Mm(0.5), Mm(1.0)));
    }

    #[test]
    fn round_rect_transform() {
        const A: f32 = (4.0 / 3.0) * (std::f32::consts::SQRT_2 - 1.0);

        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        let transform = Transform::new(1.0, 0.5, Mm(-1.0), -0.5, 1.5, Mm(2.0));
        let path = rect * transform;

        let mut exp_bldr = PathBuilder::new();
        exp_bldr.abs_move(Point::new(Mm(0.0), Mm(5.0)));
        exp_bldr.rel_cubic_bezier(
            Vector::new(Mm(-0.5 * A), Mm(-1.5 * A)),
            Vector::new(Mm(-0.5 * A), Mm(-1.75 + 0.25 * A)),
            Vector::new(Mm(0.0), Mm(-1.75)),
        );
        exp_bldr.rel_line(Vector::new(Mm(1.0), Mm(-0.5)));
        exp_bldr.rel_cubic_bezier(
            Vector::new(Mm(0.5 * A), Mm(-0.25 * A)),
            Vector::new(Mm(1.0 - 0.5 * A), Mm(1.25 - 1.5 * A)),
            Vector::new(Mm(1.0), Mm(1.25)),
        );
        exp_bldr.rel_line(Vector::new(Mm(0.5), Mm(1.5)));
        exp_bldr.rel_cubic_bezier(
            Vector::new(Mm(0.5 * A), Mm(1.5 * A)),
            Vector::new(Mm(0.5 * A), Mm(1.75 - 0.25 * A)),
            Vector::new(Mm(0.0), Mm(1.75)),
        );
        exp_bldr.rel_line(Vector::new(Mm(-1.0), Mm(0.5)));
        exp_bldr.rel_cubic_bezier(
            Vector::new(Mm(-0.5 * A), Mm(0.25 * A)),
            Vector::new(Mm(-1.0 + 0.5 * A), Mm(-1.25 + 1.5 * A)),
            Vector::new(Mm(-1.0), Mm(-1.25)),
        );
        exp_bldr.close();
        let expected = exp_bldr.build();

        assert_eq!(path.len(), expected.len());
        // assert_is_close!(path.bounds, expected.bounds);
        for (&res, &exp) in path.iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn round_rect_convert() {
        declare_units! {
            Test = 1.0;
        }

        const A: f32 = (4.0 / 3.0) * (std::f32::consts::SQRT_2 - 1.0);

        let rect = RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        };
        let conv = Conversion::<Test, Mm>::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);
        let path = rect * conv;

        let mut exp_bldr = PathBuilder::new();
        exp_bldr.abs_move(Point::new(Test(0.0), Test(5.0)));
        exp_bldr.rel_cubic_bezier(
            Vector::new(Test(-0.5 * A), Test(-1.5 * A)),
            Vector::new(Test(-0.5 * A), Test(-1.75 + 0.25 * A)),
            Vector::new(Test(0.0), Test(-1.75)),
        );
        exp_bldr.rel_line(Vector::new(Test(1.0), Test(-0.5)));
        exp_bldr.rel_cubic_bezier(
            Vector::new(Test(0.5 * A), Test(-0.25 * A)),
            Vector::new(Test(1.0 - 0.5 * A), Test(1.25 - 1.5 * A)),
            Vector::new(Test(1.0), Test(1.25)),
        );
        exp_bldr.rel_line(Vector::new(Test(0.5), Test(1.5)));
        exp_bldr.rel_cubic_bezier(
            Vector::new(Test(0.5 * A), Test(1.5 * A)),
            Vector::new(Test(0.5 * A), Test(1.75 - 0.25 * A)),
            Vector::new(Test(0.0), Test(1.75)),
        );
        exp_bldr.rel_line(Vector::new(Test(-1.0), Test(0.5)));
        exp_bldr.rel_cubic_bezier(
            Vector::new(Test(-0.5 * A), Test(0.25 * A)),
            Vector::new(Test(-1.0 + 0.5 * A), Test(-1.25 + 1.5 * A)),
            Vector::new(Test(-1.0), Test(-1.25)),
        );
        exp_bldr.close();
        let expected = exp_bldr.build();

        assert_eq!(path.len(), expected.len());
        // assert_is_close!(path.bounds, expected.bounds);
        for (&res, &exp) in path.iter().zip(expected.iter()) {
            assert_is_close!(res, exp);
        }
    }

    #[test]
    fn offset_rect_new() {
        let rect = OffsetRect::new(Mm(2.0), Mm(1.5), Mm(1.0), Mm(0.5));
        assert_is_close!(rect.min, Vector::new(Mm(0.5), Mm(2.0)));
        assert_is_close!(rect.max, Vector::new(Mm(1.5), Mm(1.0)));
    }

    #[test]
    fn offset_rect_splat() {
        let rect = OffsetRect::splat(Mm(2.0));
        assert_is_close!(rect.min, Vector::new(Mm(2.0), Mm(2.0)));
        assert_is_close!(rect.max, Vector::new(Mm(2.0), Mm(2.0)));
    }

    #[test]
    fn offset_rect_convert_from() {
        let rect: OffsetRect<Mm> = OffsetRect {
            min: Vector::new(Inch(0.5), Inch(2.0)),
            max: Vector::new(Inch(1.5), Inch(1.0)),
        }
        .convert_into();
        assert_is_close!(rect.min, Vector::new(Mm(12.7), Mm(50.8)));
        assert_is_close!(rect.max, Vector::new(Mm(38.1), Mm(25.4)));
    }

    #[test]
    fn offset_rect_add() {
        let rect = OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        } + OffsetRect {
            min: Vector::new(Mm(0.5), Mm(1.0)),
            max: Vector::new(Mm(0.5), Mm(-1.5)),
        };
        assert_is_close!(rect.min, Vector::new(Mm(1.0), Mm(3.0)));
        assert_is_close!(rect.max, Vector::new(Mm(2.0), Mm(-0.5)));
    }

    #[test]
    fn offset_rect_add_assign() {
        let mut rect = OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        };
        rect += OffsetRect {
            min: Vector::new(Mm(0.5), Mm(1.0)),
            max: Vector::new(Mm(0.5), Mm(-1.5)),
        };
        assert_is_close!(rect.min, Vector::new(Mm(1.0), Mm(3.0)));
        assert_is_close!(rect.max, Vector::new(Mm(2.0), Mm(-0.5)));
    }

    #[test]
    fn offset_rect_sub() {
        let rect = OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        } - OffsetRect {
            min: Vector::new(Mm(0.5), Mm(1.0)),
            max: Vector::new(Mm(0.5), Mm(-1.5)),
        };
        assert_is_close!(rect.min, Vector::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Vector::new(Mm(1.0), Mm(2.5)));
    }

    #[test]
    fn offset_rect_sub_assign() {
        let mut rect = OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        };
        rect -= OffsetRect {
            min: Vector::new(Mm(0.5), Mm(1.0)),
            max: Vector::new(Mm(0.5), Mm(-1.5)),
        };
        assert_is_close!(rect.min, Vector::new(Mm(0.0), Mm(1.0)));
        assert_is_close!(rect.max, Vector::new(Mm(1.0), Mm(2.5)));
    }

    #[test]
    fn offset_rect_mul_f32() {
        let rect = OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        } * 1.5;
        assert_is_close!(rect.min, Vector::new(Mm(0.75), Mm(3.0)));
        assert_is_close!(rect.max, Vector::new(Mm(2.25), Mm(1.5)));
    }

    #[test]
    fn offset_rect_mul_assign_f32() {
        let mut rect = OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        };
        rect *= 1.5;
        assert_is_close!(rect.min, Vector::new(Mm(0.75), Mm(3.0)));
        assert_is_close!(rect.max, Vector::new(Mm(2.25), Mm(1.5)));
    }

    #[test]
    fn offset_rect_div_f32() {
        let rect: OffsetRect<Mm> = OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        } / 1.5;
        assert_is_close!(rect.min, Vector::new(Mm(1.0 / 3.0), Mm(4.0 / 3.0)));
        assert_is_close!(rect.max, Vector::new(Mm(1.0), Mm(2.0 / 3.0)));
    }

    #[test]
    fn offset_rect_div_assign_f32() {
        let mut rect = OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        };
        rect /= 1.5;
        assert_is_close!(rect.min, Vector::new(Mm(1.0 / 3.0), Mm(4.0 / 3.0)));
        assert_is_close!(rect.max, Vector::new(Mm(1.0), Mm(2.0 / 3.0)));
    }

    #[test]
    fn offset_rect_is_close() {
        assert!(OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        }
        .is_close(&OffsetRect {
            min: Vector::new(Mm(0.25), Mm(1.0)) * 2.0,
            max: Vector::new(Mm(3.0), Mm(2.0)) / 2.0,
        }));
        assert!(!OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        }
        .is_close(&OffsetRect {
            min: Vector::new(Mm(0.3), Mm(1.0)) * 2.0,
            max: Vector::new(Mm(3.0), Mm(2.0)) / 2.0,
        }));
        assert!(!OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        }
        .is_close(&OffsetRect {
            min: Vector::new(Mm(0.25), Mm(1.1)) * 2.0,
            max: Vector::new(Mm(3.0), Mm(2.0)) / 2.0,
        }));
        assert!(!OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        }
        .is_close(&OffsetRect {
            min: Vector::new(Mm(0.25), Mm(1.0)) * 2.0,
            max: Vector::new(Mm(3.1), Mm(2.0)) / 2.0,
        }));
        assert!(!OffsetRect {
            min: Vector::new(Mm(0.5), Mm(2.0)),
            max: Vector::new(Mm(1.5), Mm(1.0)),
        }
        .is_close(&OffsetRect {
            min: Vector::new(Mm(0.25), Mm(1.0)) * 2.0,
            max: Vector::new(Mm(3.0), Mm(2.1)) / 2.0,
        }));
    }
}
