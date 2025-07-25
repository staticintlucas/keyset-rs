use std::ops;

use isclose::IsClose;

use crate::new_api::Vector;
use crate::{ConvertFrom, ConvertInto as _, Unit};

/// A 2 dimensional point
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Point<U: Unit> {
    /// The `x` coordinate of the point
    pub x: U,
    /// The `y` coordinate of the point
    pub y: U,
}

impl<U> Point<U>
where
    U: Unit,
{
    /// Create a new point
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: U::new(x),
            y: U::new(y),
        }
    }

    /// Create a point with the same value for the `x` and `y` coordinates
    #[inline]
    #[must_use]
    pub fn splat(v: f32) -> Self {
        Self {
            x: U::new(v),
            y: U::new(v),
        }
    }

    /// Create a new point from unit values
    #[inline]
    #[must_use]
    pub const fn from_units(x: U, y: U) -> Self {
        Self { x, y }
    }

    /// Swap the `x` and `y` coordinates of the point
    #[inline]
    #[must_use]
    pub const fn swap_xy(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    /// Linearly interpolate between two points
    #[inline]
    #[must_use]
    pub fn lerp(self, other: Self, factor: f32) -> Self {
        self + (other - self) * factor
    }
}

impl<U, V> ConvertFrom<Point<V>> for Point<U>
where
    U: Unit + ConvertFrom<V>,
    V: Unit,
{
    #[inline]
    fn convert_from(value: Point<V>) -> Self {
        Self {
            x: value.x.convert_into(),
            y: value.y.convert_into(),
        }
    }
}

impl<U> ops::Add<Vector<U>> for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vector<U>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<U> ops::AddAssign<Vector<U>> for Point<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Vector<U>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<U> ops::Sub<Vector<U>> for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Vector<U>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<U> ops::Sub<Self> for Point<U>
where
    U: Unit,
{
    type Output = Vector<U>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<U> ops::SubAssign<Vector<U>> for Point<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Vector<U>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<U> ops::Mul<f32> for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for Point<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<U> ops::Div<f32> for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<U> ops::DivAssign<f32> for Point<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<U> ops::Neg for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<U> IsClose<f32> for Point<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <U as IsClose<f32>>::ABS_TOL;
    const REL_TOL: f32 = <U as IsClose<f32>>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.x.is_close_impl(&other.x, rel_tol, abs_tol)
            && self.y.is_close_impl(&other.y, rel_tol, abs_tol)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{Inch, Mm};

    use super::*;

    #[test]
    fn point_new() {
        let point = Point::<Mm>::new(2.0, 3.0);
        assert_is_close!(point.x, Mm(2.0));
        assert_is_close!(point.y, Mm(3.0));
    }

    #[test]
    fn point_splat() {
        let point = Point::<Mm>::splat(2.0);
        assert_is_close!(point.x, Mm(2.0));
        assert_is_close!(point.y, Mm(2.0));
    }

    #[test]
    fn point_from_units() {
        let point = Point::from_units(Mm(2.0), Mm(3.0));
        assert_is_close!(point.x, Mm(2.0));
        assert_is_close!(point.y, Mm(3.0));
    }

    #[test]
    fn point_swap_xy() {
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        }
        .swap_xy();
        assert_is_close!(point.x, Mm(3.0));
        assert_is_close!(point.y, Mm(2.0));
    }

    #[test]
    fn point_lerp() {
        let start = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let end = Point {
            x: Mm(1.0),
            y: Mm(0.5),
        };

        assert_is_close!(start.lerp(end, 0.0).x, Mm(2.0));
        assert_is_close!(start.lerp(end, 0.0).y, Mm(3.0));

        assert_is_close!(start.lerp(end, 0.5).x, Mm(1.5));
        assert_is_close!(start.lerp(end, 0.5).y, Mm(1.75));

        assert_is_close!(start.lerp(end, 1.0).x, Mm(1.0));
        assert_is_close!(start.lerp(end, 1.0).y, Mm(0.5));
    }

    #[test]
    fn point_convert_from() {
        let point = Point::<Mm>::convert_from(Point {
            x: Inch(0.75),
            y: Inch(1.0),
        });
        assert_is_close!(point.x, Mm(19.05));
        assert_is_close!(point.y, Mm(25.4));
    }

    #[test]
    fn point_add() {
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } + Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(point.x, Mm(3.0));
        assert_is_close!(point.y, Mm(3.5));
    }

    #[test]
    fn point_add_assign() {
        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point += Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(point.x, Mm(3.0));
        assert_is_close!(point.y, Mm(3.5));
    }

    #[test]
    fn point_sub() {
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } - Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(point.x, Mm(1.0));
        assert_is_close!(point.y, Mm(2.5));

        let vec = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } - Point {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(vec.x, Mm(1.0));
        assert_is_close!(vec.y, Mm(2.5));
    }

    #[test]
    fn point_sub_assign() {
        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point -= Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(point.x, Mm(1.0));
        assert_is_close!(point.y, Mm(2.5));

        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point -= Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(point.x, Mm(1.0));
        assert_is_close!(point.y, Mm(2.5));
    }

    #[test]
    fn point_mul_f32() {
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } * 1.5;
        assert_is_close!(point.x, Mm(3.0));
        assert_is_close!(point.y, Mm(4.5));

        // TODO: see comment by Unit
        // let point = 1.5 * Point{ x: Mm(2.0), y: Mm(3.0) };
        // assert_is_close!(point.x, Mm(3.0));
        // assert_is_close!(point.y, Mm(4.5));
    }

    #[test]
    fn point_mul_assign_f32() {
        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point *= 1.5;
        assert_is_close!(point.x, Mm(3.0));
        assert_is_close!(point.y, Mm(4.5));
    }

    #[test]
    fn point_div_f32() {
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } / 1.5;
        assert_is_close!(point.x, Mm(4.0 / 3.0));
        assert_is_close!(point.y, Mm(2.0));
    }

    #[test]
    fn point_div_assign_f32() {
        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point /= 1.5;
        assert_is_close!(point.x, Mm(4.0 / 3.0));
        assert_is_close!(point.y, Mm(2.0));
    }

    #[test]
    fn point_neg() {
        let point = -Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        assert_is_close!(point.x, -Mm(2.0));
        assert_is_close!(point.y, -Mm(3.0));
    }

    #[test]
    fn point_is_close() {
        assert!(Point {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(Point {
            x: Mm(4.0 * 0.5),
            y: Mm(2.0 * 1.5)
        }));
        assert!(!Point {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(Point {
            x: Mm(4.1 * 0.5),
            y: Mm(2.0 * 1.5)
        }));
        assert!(!Point {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(Point {
            x: Mm(4.0 * 0.5),
            y: Mm(2.1 * 1.5)
        }));
    }
}
