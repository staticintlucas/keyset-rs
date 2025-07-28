use std::ops;

use isclose::IsClose;

use crate::{
    ConvertFrom, ConvertInto as _, Path, Point, Rotate, Scale, Transform, Translate, Unit, Vector,
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
            min: self.min - Vector::from_units(rhs.left, rhs.top),
            max: self.max + Vector::from_units(rhs.right, rhs.bottom),
        }
    }
}

impl<U> ops::AddAssign<OffsetRect<U>> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: OffsetRect<U>) {
        self.min -= Vector::from_units(rhs.left, rhs.top);
        self.max += Vector::from_units(rhs.right, rhs.bottom);
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
            min: self.min + Vector::from_units(rhs.left, rhs.top),
            max: self.max - Vector::from_units(rhs.right, rhs.bottom),
        }
    }
}

impl<U> ops::SubAssign<OffsetRect<U>> for Rect<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: OffsetRect<U>) {
        self.min += Vector::from_units(rhs.left, rhs.top);
        self.max -= Vector::from_units(rhs.right, rhs.bottom);
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

impl<U> IsClose<f32> for Rect<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <U as IsClose<f32>>::ABS_TOL;
    const REL_TOL: f32 = <U as IsClose<f32>>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.min.is_close_impl(&other.min, rel_tol, abs_tol)
            && self.max.is_close_impl(&other.max, rel_tol, abs_tol)
    }
}

impl<U> ops::Mul<Rotate> for Rect<U>
where
    U: Unit,
{
    type Output = Path<U>;

    #[inline]
    fn mul(self, _rhs: Rotate) -> Self::Output {
        // self.to_path() * rhs
        todo!()
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
    fn mul(self, _rhs: Transform<U>) -> Self::Output {
        // self.to_path() * rhs
        todo!()
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

impl<U> IsClose<f32> for RoundRect<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <U as IsClose<f32>>::ABS_TOL;
    const REL_TOL: f32 = <U as IsClose<f32>>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.min.is_close_impl(&other.min, rel_tol, abs_tol)
            && self.max.is_close_impl(&other.max, rel_tol, abs_tol)
            && self.radii.is_close_impl(&other.radii, rel_tol, abs_tol)
    }
}

impl<U> ops::Mul<Rotate> for RoundRect<U>
where
    U: Unit,
{
    type Output = Path<U>;

    #[inline]
    fn mul(self, _rhs: Rotate) -> Self::Output {
        // self.to_path() * rhs
        todo!()
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
    fn mul(self, _rhs: Transform<U>) -> Self::Output {
        // self.to_path() * rhs
        todo!()
    }
}

/// A set of offsets for the top, right, bottom, and left sides of a rectangle
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OffsetRect<U: Unit> {
    pub(crate) top: U,
    pub(crate) right: U,
    pub(crate) bottom: U,
    pub(crate) left: U,
}

impl<U> OffsetRect<U>
where
    U: Unit,
{
    /// Create a new offset rectangle with the given offsets
    #[inline]
    #[must_use]
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: U::new(top),
            right: U::new(right),
            bottom: U::new(bottom),
            left: U::new(left),
        }
    }

    /// Create a new offset rectangle with the given offsets
    #[inline]
    pub const fn from_units(top: U, right: U, bottom: U, left: U) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
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
            top: value.top.convert_into(),
            right: value.right.convert_into(),
            bottom: value.bottom.convert_into(),
            left: value.left.convert_into(),
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
            top: self.top + rhs.top,
            right: self.right + rhs.right,
            bottom: self.bottom + rhs.bottom,
            left: self.left + rhs.left,
        }
    }
}

impl<U> ops::AddAssign for OffsetRect<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.top += rhs.top;
        self.right += rhs.right;
        self.bottom += rhs.bottom;
        self.left += rhs.left;
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
            top: self.top - rhs.top,
            right: self.right - rhs.right,
            bottom: self.bottom - rhs.bottom,
            left: self.left - rhs.left,
        }
    }
}

impl<U> ops::SubAssign for OffsetRect<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.top -= rhs.top;
        self.right -= rhs.right;
        self.bottom -= rhs.bottom;
        self.left -= rhs.left;
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
            top: self.top * rhs,
            right: self.right * rhs,
            bottom: self.bottom * rhs,
            left: self.left * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for OffsetRect<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.top *= rhs;
        self.right *= rhs;
        self.bottom *= rhs;
        self.left *= rhs;
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
            top: self.top / rhs,
            right: self.right / rhs,
            bottom: self.bottom / rhs,
            left: self.left / rhs,
        }
    }
}

impl<U> ops::DivAssign<f32> for OffsetRect<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.top /= rhs;
        self.right /= rhs;
        self.bottom /= rhs;
        self.left /= rhs;
    }
}

impl<U> IsClose<f32> for OffsetRect<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <U as IsClose<f32>>::ABS_TOL;
    const REL_TOL: f32 = <U as IsClose<f32>>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.top.is_close_impl(&other.top, rel_tol, abs_tol)
            && self.right.is_close_impl(&other.right, rel_tol, abs_tol)
            && self.bottom.is_close_impl(&other.bottom, rel_tol, abs_tol)
            && self.left.is_close_impl(&other.left, rel_tol, abs_tol)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{Inch, Mm};

    use super::*;

    #[test]
    fn rect_new() {
        let rect = Rect::<Mm>::new(Point::new(0.0, 1.0), Point::new(2.0, 4.0));
        assert_is_close!(rect.min, Point::new(0.0, 1.0));
        assert_is_close!(rect.max, Point::new(2.0, 4.0));
    }

    #[test]
    fn rect_from_origin_and_size() {
        let rect = Rect::<Mm>::from_origin_and_size(Point::new(0.0, 1.0), Vector::new(2.0, 3.0));
        assert_is_close!(rect.min, Point::new(0.0, 1.0));
        assert_is_close!(rect.max, Point::new(2.0, 4.0));
    }

    #[test]
    fn rect_from_center_and_size() {
        let rect = Rect::<Mm>::from_center_and_size(Point::new(1.0, 2.5), Vector::new(2.0, 3.0));
        assert_is_close!(rect.min, Point::new(0.0, 1.0));
        assert_is_close!(rect.max, Point::new(2.0, 4.0));
    }

    #[test]
    fn rect_empty() {
        let rect = Rect::<Mm>::empty();
        assert_is_close!(rect.min, Point::new(0.0, 0.0));
        assert_is_close!(rect.max, Point::new(0.0, 0.0));
    }

    #[test]
    fn rect_size() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        assert_is_close!(rect.size(), Vector::new(2.0, 3.0));
    }

    #[test]
    fn rect_width() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        assert_is_close!(rect.width(), Mm(2.0));
    }

    #[test]
    fn rect_height() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        assert_is_close!(rect.height(), Mm(3.0));
    }

    #[test]
    fn rect_center() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        assert_is_close!(rect.center(), Point::new(1.0, 2.5));
    }

    #[test]
    fn rect_union() {
        let rect1 = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        let rect2 = Rect::<Mm> {
            min: Point::new(0.2, 0.5),
            max: Point::new(1.0, 6.5),
        };
        assert_is_close!(rect1.union(rect2).min, Point::new(0.0, 0.5));
        assert_is_close!(rect1.union(rect2).max, Point::new(2.0, 6.5));
    }

    #[test]
    fn rect_convert_from() {
        let rect = Rect::<Mm>::convert_from(Rect::<Inch> {
            min: Point::new(0.0, 0.75),
            max: Point::new(1.0, 1.5),
        });
        assert_is_close!(rect.min, Point::new(0.0, 19.05));
        assert_is_close!(rect.max, Point::new(25.4, 38.1));
    }

    #[test]
    fn rect_add_offset_rect() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        } + OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        };
        assert_is_close!(rect.min, Point::new(-0.5, -1.0));
        assert_is_close!(rect.max, Point::new(3.5, 5.0));
    }

    #[test]
    fn rect_add_assign_offset_rect() {
        let mut rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        rect += OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        };
        assert_is_close!(rect.min, Point::new(-0.5, -1.0));
        assert_is_close!(rect.max, Point::new(3.5, 5.0));
    }

    #[test]
    fn rect_sub_offset_rect() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        } - OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        };
        assert_is_close!(rect.min, Point::new(0.5, 3.0));
        assert_is_close!(rect.max, Point::new(0.5, 3.0));
    }

    #[test]
    fn rect_sub_assign_offset_rect() {
        let mut rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        rect -= OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        };
        assert_is_close!(rect.min, Point::new(0.5, 3.0));
        assert_is_close!(rect.max, Point::new(0.5, 3.0));
    }

    #[test]
    fn rect_mul_f32() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        } * 1.5;
        assert_is_close!(rect.min, Point::new(0.0, 1.5));
        assert_is_close!(rect.max, Point::new(3.0, 6.0));

        // TODO: see comment by Unit
        // let rect = 1.5 * Rect::<Mm> {
        //     min: Point::new(0.0, 1.0),
        //     max: Point::new(2.0, 4.0),
        // };
        // assert_is_close!(rect.min, Point::new(0.0, 1.5));
        // assert_is_close!(rect.max, Point::new(3.0, 6.0));
    }

    #[test]
    fn rect_mul_assign_f32() {
        let mut rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        rect *= 1.5;
        assert_is_close!(rect.min, Point::new(0.0, 1.5));
        assert_is_close!(rect.max, Point::new(3.0, 6.0));
    }

    #[test]
    fn rect_div_f32() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        } / 1.5;
        assert_is_close!(rect.min, Point::new(0.0, 2.0 / 3.0));
        assert_is_close!(rect.max, Point::new(4.0 / 3.0, 8.0 / 3.0));
    }

    #[test]
    fn rect_div_assign_f32() {
        let mut rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        rect /= 1.5;
        assert_is_close!(rect.min, Point::new(0.0, 2.0 / 3.0));
        assert_is_close!(rect.max, Point::new(4.0 / 3.0, 8.0 / 3.0));
    }

    #[test]
    fn rect_is_close() {
        assert!(Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        }
        .is_close(Rect {
            min: Point::new(0.0, 2.0) * 0.5,
            max: Point::new(1.0, 2.0) * 2.0,
        }));
        assert!(!Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        }
        .is_close(Rect {
            min: Point::new(0.1, 2.0) * 0.5,
            max: Point::new(1.0, 2.0) * 2.0,
        }));
        assert!(!Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        }
        .is_close(Rect {
            min: Point::new(0.0, 2.1) * 0.5,
            max: Point::new(1.0, 2.0) * 2.0,
        }));
        assert!(!Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        }
        .is_close(Rect {
            min: Point::new(0.0, 2.0) * 0.5,
            max: Point::new(1.1, 2.0) * 2.0,
        }));
        assert!(!Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        }
        .is_close(Rect {
            min: Point::new(0.0, 2.0) * 0.5,
            max: Point::new(1.0, 2.1) * 2.0,
        }));
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn rect_rotate() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        let rotate = Rotate::degrees(135.0);
        let path = rect * rotate;
        assert!(matches!(path, Path::<Mm> { .. }));
    }

    #[test]
    fn rect_scale() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        } * Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(0.0, 0.5));
        assert_is_close!(rect.max, Point::new(4.0, 2.0));

        let mut rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        rect *= Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(0.0, 0.5));
        assert_is_close!(rect.max, Point::new(4.0, 2.0));

        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        } / Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(0.0, 2.0));
        assert_is_close!(rect.max, Point::new(1.0, 8.0));

        let mut rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        rect /= Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(0.0, 2.0));
        assert_is_close!(rect.max, Point::new(1.0, 8.0));
    }

    #[test]
    fn rect_translate() {
        let rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        } * Translate::new(2.0, -1.0);

        assert_is_close!(rect.min, Point::new(2.0, 0.0));
        assert_is_close!(rect.max, Point::new(4.0, 3.0));

        let mut rect = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        };
        rect *= Translate::new(2.0, -1.0);

        assert_is_close!(rect.min, Point::new(2.0, 0.0));
        assert_is_close!(rect.max, Point::new(4.0, 3.0));
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn rect_transform() {
        let transform = Transform::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);
        let path = Rect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
        } * transform;
        assert!(matches!(path, Path::<Mm> { .. }));
    }

    #[test]
    fn round_rect_new() {
        let rect = RoundRect::<Mm>::new(
            Point::new(0.0, 1.0),
            Point::new(2.0, 4.0),
            Vector::new(0.5, 1.0),
        );
        assert_is_close!(rect.min, Point::new(0.0, 1.0));
        assert_is_close!(rect.max, Point::new(2.0, 4.0));
        assert_is_close!(rect.radii, Vector::new(0.5, 1.0));
    }

    #[test]
    fn round_rect_from_origin_size_and_radii() {
        let rect = RoundRect::<Mm>::from_origin_size_and_radii(
            Point::new(0.0, 1.0),
            Vector::new(2.0, 3.0),
            Vector::new(0.5, 1.0),
        );
        assert_is_close!(rect.min, Point::new(0.0, 1.0));
        assert_is_close!(rect.max, Point::new(2.0, 4.0));
        assert_is_close!(rect.radii, Vector::new(0.5, 1.0));
    }

    #[test]
    fn round_rect_from_center_size_and_radii() {
        let rect = RoundRect::<Mm>::from_center_size_and_radii(
            Point::new(1.0, 2.5),
            Vector::new(2.0, 3.0),
            Vector::new(0.5, 1.0),
        );
        assert_is_close!(rect.min, Point::new(0.0, 1.0));
        assert_is_close!(rect.max, Point::new(2.0, 4.0));
        assert_is_close!(rect.radii, Vector::new(0.5, 1.0));
    }

    #[test]
    fn round_rect_from_rect_and_radii() {
        let rect = RoundRect::<Mm>::from_rect_and_radii(
            Rect::new(Point::new(0.0, 1.0), Point::new(2.0, 4.0)),
            Vector::new(0.5, 1.0),
        );
        assert_is_close!(rect.min, Point::new(0.0, 1.0));
        assert_is_close!(rect.max, Point::new(2.0, 4.0));
        assert_is_close!(rect.radii, Vector::new(0.5, 1.0));
    }

    #[test]
    fn round_rect_to_rect() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        }
        .to_rect();
        assert_is_close!(rect.min, Point::new(0.0, 1.0));
        assert_is_close!(rect.max, Point::new(2.0, 4.0));
    }

    #[test]
    fn round_rect_size() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        assert_is_close!(rect.size(), Vector::new(2.0, 3.0));
    }

    #[test]
    fn round_rect_width() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        assert_is_close!(rect.width(), Mm(2.0));
    }

    #[test]
    fn round_rect_height() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        assert_is_close!(rect.height(), Mm(3.0));
    }

    #[test]
    fn round_rect_center() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        assert_is_close!(rect.center(), Point::new(1.0, 2.5));
    }

    #[test]
    fn round_rect_convert_from() {
        let rect = RoundRect::<Mm>::convert_from(RoundRect::<Inch> {
            min: Point::new(0.0, 0.75),
            max: Point::new(1.0, 1.5),
            radii: Vector::new(0.25, 0.5),
        });
        assert_is_close!(rect.min, Point::new(0.0, 19.05));
        assert_is_close!(rect.max, Point::new(25.4, 38.1));
        assert_is_close!(rect.radii, Vector::new(6.35, 12.7));
    }

    #[test]
    fn round_rect_mul_f32() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        } * 1.5;
        assert_is_close!(rect.min, Point::new(0.0, 1.5));
        assert_is_close!(rect.max, Point::new(3.0, 6.0));
        assert_is_close!(rect.radii, Vector::new(0.75, 1.5));

        // TODO: see comment by Unit
        // let rect = 1.5 * RoundRect {
        //     min: Point::new(0.0, 1.0),
        //     max: Point::new(2.0, 4.0),
        //     radii: Vector::new(0.5, 1.0),
        // };
        // assert_is_close!(rect.min, Point::new(0.0, 1.5));
        // assert_is_close!(rect.max, Point::new(3.0, 6.0));
        // assert_is_close!(rect.radii, Vector::new(0.75, 1.5));
    }

    #[test]
    fn round_rect_mul_assign_f32() {
        let mut rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        rect *= 1.5;
        assert_is_close!(rect.min, Point::new(0.0, 1.5));
        assert_is_close!(rect.max, Point::new(3.0, 6.0));
        assert_is_close!(rect.radii, Vector::new(0.75, 1.5));
    }

    #[test]
    fn round_rect_div_f32() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        } / 1.5;
        assert_is_close!(rect.min, Point::new(0.0, 2.0 / 3.0));
        assert_is_close!(rect.max, Point::new(4.0 / 3.0, 8.0 / 3.0));
        assert_is_close!(rect.radii, Vector::new(1.0 / 3.0, 2.0 / 3.0));
    }

    #[test]
    fn round_rect_div_assign_f32() {
        let mut rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        rect /= 1.5;
        assert_is_close!(rect.min, Point::new(0.0, 2.0 / 3.0));
        assert_is_close!(rect.max, Point::new(4.0 / 3.0, 8.0 / 3.0));
        assert_is_close!(rect.radii, Vector::new(1.0 / 3.0, 2.0 / 3.0));
    }

    #[test]
    fn round_rect_is_close() {
        assert!(RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        }
        .is_close(RoundRect {
            min: Point::new(0.0, 2.0) * 0.5,
            max: Point::new(1.0, 2.0) * 2.0,
            radii: Vector::new(1.5, 3.0) / 3.0,
        }));
        assert!(!RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        }
        .is_close(RoundRect {
            min: Point::new(0.1, 2.0) * 0.5,
            max: Point::new(1.0, 2.0) * 2.0,
            radii: Vector::new(1.5, 3.0) / 3.0,
        }));
        assert!(!RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        }
        .is_close(RoundRect {
            min: Point::new(0.0, 2.1) * 0.5,
            max: Point::new(1.0, 2.0) * 2.0,
            radii: Vector::new(1.5, 3.0) / 3.0,
        }));
        assert!(!RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        }
        .is_close(RoundRect {
            min: Point::new(0.0, 2.0) * 0.5,
            max: Point::new(1.1, 2.0) * 2.0,
            radii: Vector::new(1.5, 3.0) / 3.0,
        }));
        assert!(!RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        }
        .is_close(RoundRect {
            min: Point::new(0.0, 2.0) * 0.5,
            max: Point::new(1.0, 2.1) * 2.0,
            radii: Vector::new(1.5, 3.0) / 3.0,
        }));
        assert!(!RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        }
        .is_close(RoundRect {
            min: Point::new(0.0, 2.0) * 0.5,
            max: Point::new(1.0, 2.0) * 2.0,
            radii: Vector::new(1.6, 3.0) / 3.0,
        }));
        assert!(!RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        }
        .is_close(RoundRect {
            min: Point::new(0.0, 2.0) * 0.5,
            max: Point::new(1.0, 2.0) * 2.0,
            radii: Vector::new(1.5, 3.1) / 3.0,
        }));
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn round_rect_rotate() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        let rotate = Rotate::degrees(135.0);
        let path = rect * rotate;
        assert!(matches!(path, Path::<Mm> { .. }));
    }

    #[test]
    fn round_rect_scale() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        } * Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(0.0, 0.5));
        assert_is_close!(rect.max, Point::new(4.0, 2.0));
        assert_is_close!(rect.radii, Vector::new(1.0, 0.5));

        let mut rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        rect *= Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(0.0, 0.5));
        assert_is_close!(rect.max, Point::new(4.0, 2.0));
        assert_is_close!(rect.radii, Vector::new(1.0, 0.5));

        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        } / Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(0.0, 2.0));
        assert_is_close!(rect.max, Point::new(1.0, 8.0));
        assert_is_close!(rect.radii, Vector::new(0.25, 2.0));

        let mut rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        rect /= Scale::new(2.0, 0.5);

        assert_is_close!(rect.min, Point::new(0.0, 2.0));
        assert_is_close!(rect.max, Point::new(1.0, 8.0));
        assert_is_close!(rect.radii, Vector::new(0.25, 2.0));
    }

    #[test]
    fn round_rect_translate() {
        let rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        } * Translate::new(2.0, -1.0);

        assert_is_close!(rect.min, Point::new(2.0, 0.0));
        assert_is_close!(rect.max, Point::new(4.0, 3.0));
        assert_is_close!(rect.radii, Vector::new(0.5, 1.0));

        let mut rect = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        };
        rect *= Translate::new(2.0, -1.0);

        assert_is_close!(rect.min, Point::new(2.0, 0.0));
        assert_is_close!(rect.max, Point::new(4.0, 3.0));
        assert_is_close!(rect.radii, Vector::new(0.5, 1.0));
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn round_rect_transform() {
        let transform = Transform::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);
        let path = RoundRect::<Mm> {
            min: Point::new(0.0, 1.0),
            max: Point::new(2.0, 4.0),
            radii: Vector::new(0.5, 1.0),
        } * transform;
        assert!(matches!(path, Path::<Mm> { .. }));
    }

    #[test]
    fn offset_rect_new() {
        let rect = OffsetRect::<Mm>::new(2.0, 1.5, 1.0, 0.5);
        assert_is_close!(rect.top, Mm(2.0));
        assert_is_close!(rect.right, Mm(1.5));
        assert_is_close!(rect.bottom, Mm(1.0));
        assert_is_close!(rect.left, Mm(0.5));
    }

    #[test]
    fn offset_rect_from_units() {
        let rect = OffsetRect::from_units(Mm(2.0), Mm(1.5), Mm(1.0), Mm(0.5));
        assert_is_close!(rect.top, Mm(2.0));
        assert_is_close!(rect.right, Mm(1.5));
        assert_is_close!(rect.bottom, Mm(1.0));
        assert_is_close!(rect.left, Mm(0.5));
    }

    #[test]
    fn offset_rect_convert_from() {
        let rect: OffsetRect<Mm> = OffsetRect {
            top: Inch(2.0),
            right: Inch(1.5),
            bottom: Inch(1.0),
            left: Inch(0.5),
        }
        .convert_into();
        assert_is_close!(rect.top, Mm(50.8));
        assert_is_close!(rect.right, Mm(38.1));
        assert_is_close!(rect.bottom, Mm(25.4));
        assert_is_close!(rect.left, Mm(12.7));
    }

    #[test]
    fn offset_rect_add() {
        let rect = OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        } + OffsetRect {
            top: Mm(1.0),
            right: Mm(0.5),
            bottom: Mm(-1.5),
            left: Mm(0.5),
        };
        assert_is_close!(rect.top, Mm(3.0));
        assert_is_close!(rect.right, Mm(2.0));
        assert_is_close!(rect.bottom, Mm(-0.5));
        assert_is_close!(rect.left, Mm(1.0));
    }

    #[test]
    fn offset_rect_add_assign() {
        let mut rect = OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        };
        rect += OffsetRect {
            top: Mm(1.0),
            right: Mm(0.5),
            bottom: Mm(-1.5),
            left: Mm(0.5),
        };
        assert_is_close!(rect.top, Mm(3.0));
        assert_is_close!(rect.right, Mm(2.0));
        assert_is_close!(rect.bottom, Mm(-0.5));
        assert_is_close!(rect.left, Mm(1.0));
    }

    #[test]
    fn offset_rect_sub() {
        let rect = OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        } - OffsetRect {
            top: Mm(1.0),
            right: Mm(0.5),
            bottom: Mm(-1.5),
            left: Mm(0.5),
        };
        assert_is_close!(rect.top, Mm(1.0));
        assert_is_close!(rect.right, Mm(1.0));
        assert_is_close!(rect.bottom, Mm(2.5));
        assert_is_close!(rect.left, Mm(0.0));
    }

    #[test]
    fn offset_rect_sub_assign() {
        let mut rect = OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        };
        rect -= OffsetRect {
            top: Mm(1.0),
            right: Mm(0.5),
            bottom: Mm(-1.5),
            left: Mm(0.5),
        };
        assert_is_close!(rect.top, Mm(1.0));
        assert_is_close!(rect.right, Mm(1.0));
        assert_is_close!(rect.bottom, Mm(2.5));
        assert_is_close!(rect.left, Mm(0.0));
    }

    #[test]
    fn offset_rect_mul_f32() {
        let rect = OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        } * 1.5;
        assert_is_close!(rect.top, Mm(3.0));
        assert_is_close!(rect.right, Mm(2.25));
        assert_is_close!(rect.bottom, Mm(1.5));
        assert_is_close!(rect.left, Mm(0.75));
    }

    #[test]
    fn offset_rect_mul_assign_f32() {
        let mut rect = OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        };
        rect *= 1.5;
        assert_is_close!(rect.top, Mm(3.0));
        assert_is_close!(rect.right, Mm(2.25));
        assert_is_close!(rect.bottom, Mm(1.5));
        assert_is_close!(rect.left, Mm(0.75));
    }

    #[test]
    fn offset_rect_div_f32() {
        let rect: OffsetRect<Mm> = OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        } / 1.5;
        assert_is_close!(rect.top, Mm(4.0 / 3.0));
        assert_is_close!(rect.right, Mm(1.0));
        assert_is_close!(rect.bottom, Mm(2.0 / 3.0));
        assert_is_close!(rect.left, Mm(1.0 / 3.0));
    }

    #[test]
    fn offset_rect_div_assign_f32() {
        let mut rect = OffsetRect {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        };
        rect /= 1.5;
        assert_is_close!(rect.top, Mm(4.0 / 3.0));
        assert_is_close!(rect.right, Mm(1.0));
        assert_is_close!(rect.bottom, Mm(2.0 / 3.0));
        assert_is_close!(rect.left, Mm(1.0 / 3.0));
    }

    #[test]
    fn offset_rect_is_close() {
        assert!(OffsetRect::<Mm> {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        }
        .is_close(OffsetRect {
            top: Mm(1.0) * 2.0,
            right: Mm(3.0) / 2.0,
            bottom: Mm(0.5) * 2.0,
            left: Mm(1.0) / 2.0,
        }));
        assert!(!OffsetRect::<Mm> {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        }
        .is_close(OffsetRect {
            top: Mm(1.1) * 2.0,
            right: Mm(3.0) / 2.0,
            bottom: Mm(0.5) * 2.0,
            left: Mm(1.0) / 2.0,
        }));
        assert!(!OffsetRect::<Mm> {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        }
        .is_close(OffsetRect {
            top: Mm(1.0) * 2.0,
            right: Mm(3.1) / 2.0,
            bottom: Mm(0.5) * 2.0,
            left: Mm(1.0) / 2.0,
        }));
        assert!(!OffsetRect::<Mm> {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        }
        .is_close(OffsetRect {
            top: Mm(1.0) * 2.0,
            right: Mm(3.0) / 2.0,
            bottom: Mm(0.6) * 2.0,
            left: Mm(1.0) / 2.0,
        }));
        assert!(!OffsetRect::<Mm> {
            top: Mm(2.0),
            right: Mm(1.5),
            bottom: Mm(1.0),
            left: Mm(0.5),
        }
        .is_close(OffsetRect {
            top: Mm(1.0) * 2.0,
            right: Mm(3.0) / 2.0,
            bottom: Mm(0.5) * 2.0,
            left: Mm(1.1) / 2.0,
        }));
    }
}
