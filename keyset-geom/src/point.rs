use std::ops;

use isclose::IsClose;

use crate::{
    Conversion, ConvertFrom, ConvertInto as _, Rotate, Scale, Transform, Translate, Unit, Vector,
};

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
    pub const fn new(x: U, y: U) -> Self {
        Self { x, y }
    }

    /// Create a point with the same value for the `x` and `y` coordinates
    #[inline]
    #[must_use]
    pub const fn splat(v: U) -> Self {
        Self { x: v, y: v }
    }

    /// Create a point at the origin (0, 0)
    #[inline]
    #[must_use]
    pub fn origin() -> Self {
        Self {
            x: U::zero(),
            y: U::zero(),
        }
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

    /// Returns the minimum values `x` and `y` components from `self` and `other`
    #[inline]
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Returns the maximum values `x` and `y` components from `self` and `other`
    #[inline]
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
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

impl<U, V> IsClose<Point<V>> for Point<U>
where
    U: Unit + IsClose<V>,
    V: Unit,
{
    type Tolerance = <U as IsClose<V>>::Tolerance;
    const ZERO_TOL: Self::Tolerance = <U as IsClose<V>>::ZERO_TOL;
    const ABS_TOL: Self::Tolerance = <U as IsClose<V>>::ABS_TOL;
    const REL_TOL: Self::Tolerance = <U as IsClose<V>>::REL_TOL;

    #[inline]
    fn is_close_tol(
        &self,
        other: &Point<V>,
        rel_tol: &Self::Tolerance,
        abs_tol: &Self::Tolerance,
    ) -> bool {
        self.x.is_close_tol(&other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&other.y, rel_tol, abs_tol)
    }
}

impl<U> ops::Mul<Rotate> for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Rotate) -> Self::Output {
        let (sin, cos) = rhs.angle.sin_cos();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
}

impl<U> ops::MulAssign<Rotate> for Point<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Rotate) {
        *self = *self * rhs;
    }
}

impl<U> ops::Mul<Scale> for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Scale) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<U> ops::MulAssign<Scale> for Point<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Scale) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<U> ops::Div<Scale> for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Scale) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<U> ops::DivAssign<Scale> for Point<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: Scale) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<U> ops::Mul<Translate<U>> for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Translate<U>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<U> ops::MulAssign<Translate<U>> for Point<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Translate<U>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<U> ops::Mul<Transform<U>> for Point<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Transform<U>) -> Self::Output {
        Self {
            x: self.x * rhs.a_xx + self.y * rhs.a_xy + rhs.t_x,
            y: self.x * rhs.a_yx + self.y * rhs.a_yy + rhs.t_y,
        }
    }
}

impl<U> ops::MulAssign<Transform<U>> for Point<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Transform<U>) {
        *self = *self * rhs;
    }
}

impl<Dst, Src> ops::Mul<Conversion<Dst, Src>> for Point<Src>
where
    Dst: Unit,
    Src: Unit,
{
    type Output = Point<Dst>;

    #[inline]
    fn mul(self, rhs: Conversion<Dst, Src>) -> Self::Output {
        Point {
            x: Dst::new(self.x.get() * rhs.a_xx + self.y.get() * rhs.a_xy + rhs.t_x),
            y: Dst::new(self.x.get() * rhs.a_yx + self.y.get() * rhs.a_yy + rhs.t_y),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{declare_units, Inch, Mm};

    use super::*;

    #[test]
    fn point_new() {
        let point = Point::new(Mm(2.0), Mm(3.0));
        assert_is_close!(point.x, Mm(2.0));
        assert_is_close!(point.y, Mm(3.0));
    }

    #[test]
    fn point_splat() {
        let point = Point::splat(Mm(2.0));
        assert_is_close!(point.x, Mm(2.0));
        assert_is_close!(point.y, Mm(2.0));
    }

    #[test]
    fn point_origin() {
        let point = Point::<Mm>::origin();
        assert_is_close!(point.x, Mm(0.0));
        assert_is_close!(point.y, Mm(0.0));
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
    fn point_cmp() {
        let point1 = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let point2 = Point {
            x: Mm(4.0),
            y: Mm(-3.5),
        };

        assert_is_close!(point1.max(point2).x, Mm(4.0));
        assert_is_close!(point1.max(point2).y, Mm(3.0));

        assert_is_close!(point1.min(point2).x, Mm(2.0));
        assert_is_close!(point1.min(point2).y, Mm(-3.5));
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
        .is_close(&Point {
            x: Mm(4.0 * 0.5),
            y: Mm(2.0 * 1.5)
        }));
        assert!(!Point {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(&Point {
            x: Mm(4.1 * 0.5),
            y: Mm(2.0 * 1.5)
        }));
        assert!(!Point {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(&Point {
            x: Mm(4.0 * 0.5),
            y: Mm(2.1 * 1.5)
        }));
    }

    #[test]
    fn point_rotate() {
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } * Rotate::degrees(135.0);

        assert_is_close!(point.x, Mm(-2.5 * std::f32::consts::SQRT_2));
        assert_is_close!(point.y, Mm(-0.5 * std::f32::consts::SQRT_2));

        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point *= Rotate::degrees(135.0);

        assert_is_close!(point.x, Mm(-2.5 * std::f32::consts::SQRT_2));
        assert_is_close!(point.y, Mm(-0.5 * std::f32::consts::SQRT_2));
    }

    #[test]
    fn point_scale() {
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } * Scale::new(2.0, 0.5);

        assert_is_close!(point.x, Mm(4.0));
        assert_is_close!(point.y, Mm(1.5));

        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point *= Scale::new(2.0, 0.5);

        assert_is_close!(point.x, Mm(4.0));
        assert_is_close!(point.y, Mm(1.5));

        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } / Scale::new(2.0, 0.5);

        assert_is_close!(point.x, Mm(1.0));
        assert_is_close!(point.y, Mm(6.0));

        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point /= Scale::new(2.0, 0.5);

        assert_is_close!(point.x, Mm(1.0));
        assert_is_close!(point.y, Mm(6.0));
    }

    #[test]
    fn point_translate() {
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } * Translate::new(Mm(2.0), Mm(-1.0));

        assert_is_close!(point.x, Mm(4.0));
        assert_is_close!(point.y, Mm(2.0));

        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point *= Translate::new(Mm(2.0), Mm(-1.0));

        assert_is_close!(point.x, Mm(4.0));
        assert_is_close!(point.y, Mm(2.0));
    }

    #[test]
    fn point_transform() {
        let transform = Transform::new(1.0, 0.5, Mm(-1.0), -0.5, 1.5, Mm(2.0));
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } * transform;

        assert_is_close!(point.x, Mm(2.5));
        assert_is_close!(point.y, Mm(5.5));

        let mut point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        point *= transform;

        assert_is_close!(point.x, Mm(2.5));
        assert_is_close!(point.y, Mm(5.5));
    }

    #[test]
    fn point_convert() {
        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Test, Mm>::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);
        let point = Point {
            x: Mm(2.0),
            y: Mm(3.0),
        } * conv;

        assert_is_close!(point.x, Test(2.5));
        assert_is_close!(point.y, Test(5.5));
    }
}
