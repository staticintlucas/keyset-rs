use std::ops;

use isclose::IsClose;

use crate::new_api::{Point, Vector};
use crate::{ConvertFrom, ConvertInto as _, Unit};

/// A 2 dimensional rectangle
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect<U: Unit> {
    pub(crate) min: Point<U>,
    pub(crate) max: Point<U>,
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

/// A 2 dimensional rectangle with rounded corners
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoundRect<U: Unit> {
    pub(crate) min: Point<U>,
    pub(crate) max: Point<U>,
    pub(crate) radii: Vector<U>,
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

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{Inch, Mm};

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
    fn rect_from_unit() {
        let rect = Rect::<Mm>::convert_from(Rect {
            min: Point::new(Inch(0.0), Inch(0.75)),
            max: Point::new(Inch(1.0), Inch(1.5)),
        });
        assert_is_close!(rect.min, Point::new(Mm(0.0), Mm(19.05)));
        assert_is_close!(rect.max, Point::new(Mm(25.4), Mm(38.1)));
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
        .is_close(Rect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
        }));
        assert!(!Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        }
        .is_close(Rect {
            min: Point::new(Mm(0.1), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
        }));
        assert!(!Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        }
        .is_close(Rect {
            min: Point::new(Mm(0.0), Mm(2.1)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
        }));
        assert!(!Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        }
        .is_close(Rect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.1), Mm(2.0)) * 2.0,
        }));
        assert!(!Rect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
        }
        .is_close(Rect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.1)) * 2.0,
        }));
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
    fn round_rect_from_origin_and_size() {
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
    fn round_rect_from_center_and_size() {
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
    fn round_rect_from_unit() {
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
        .is_close(RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(RoundRect {
            min: Point::new(Mm(0.1), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(RoundRect {
            min: Point::new(Mm(0.0), Mm(2.1)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.1), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.1)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.6), Mm(3.0)) / 3.0,
        }));
        assert!(!RoundRect {
            min: Point::new(Mm(0.0), Mm(1.0)),
            max: Point::new(Mm(2.0), Mm(4.0)),
            radii: Vector::new(Mm(0.5), Mm(1.0)),
        }
        .is_close(RoundRect {
            min: Point::new(Mm(0.0), Mm(2.0)) * 0.5,
            max: Point::new(Mm(1.0), Mm(2.0)) * 2.0,
            radii: Vector::new(Mm(1.5), Mm(3.1)) / 3.0,
        }));
    }
}
