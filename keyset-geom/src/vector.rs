use std::ops;

use isclose::IsClose;

use crate::{Angle, ConvertFrom, ConvertInto as _, Rotate, Scale, Transform, Translate, Unit};

/// A 2 dimensional vector
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vector<U: Unit> {
    /// The `x` coordinate of the vector
    pub x: U,
    /// The `x` coordinate of the vector
    pub y: U,
}

impl<U> Vector<U>
where
    U: Unit,
{
    /// Create a new vector
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: U::new(x),
            y: U::new(y),
        }
    }

    /// Create a vector with the same value for the `x` and `y` coordinates
    #[inline]
    #[must_use]
    pub fn splat(v: f32) -> Self {
        Self {
            x: U::new(v),
            y: U::new(v),
        }
    }

    /// Create a new zero-length vector
    #[inline]
    #[must_use]
    pub fn zero() -> Self {
        Self {
            x: U::zero(),
            y: U::zero(),
        }
    }

    /// Create a new vector from unit values
    #[inline]
    #[must_use]
    pub const fn from_units(x: U, y: U) -> Self {
        Self { x, y }
    }

    /// Swap the `x` and `y` coordinates of the vector
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

    /// Returns the absolute value of the vector
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns the length of the vector
    #[inline]
    #[must_use]
    pub fn hypot(self) -> f32 {
        self.hypot2().sqrt()
    }

    /// Returns the square of the length of the vector
    #[inline]
    #[must_use]
    pub fn hypot2(self) -> f32 {
        self.x.get() * self.x.get() + self.y.get() * self.y.get()
    }

    /// Returns the angle of the vector from the x axis
    #[inline]
    #[must_use]
    pub fn angle(self) -> Angle {
        Angle::atan2(self.y.get(), self.x.get())
    }

    /// Linearly interpolate between two vectors
    #[inline]
    #[must_use]
    pub fn lerp(self, other: Self, factor: f32) -> Self {
        self + (other - self) * factor
    }
}

impl<U, V> ConvertFrom<Vector<V>> for Vector<U>
where
    U: Unit + ConvertFrom<V>,
    V: Unit,
{
    #[inline]
    fn convert_from(value: Vector<V>) -> Self {
        Self {
            x: value.x.convert_into(),
            y: value.y.convert_into(),
        }
    }
}

impl<U> ops::Add<Self> for Vector<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<U> ops::AddAssign<Self> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<U> ops::Sub<Self> for Vector<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<U> ops::SubAssign<Self> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<U> ops::Mul<f32> for Vector<U>
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

impl<U> ops::MulAssign<f32> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<U> ops::Div<f32> for Vector<U>
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

impl<U> ops::DivAssign<f32> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<U> ops::Neg for Vector<U>
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

impl<U> IsClose for Vector<U>
where
    U: Unit,
{
    type Tolerance = f32;
    const ZERO_TOL: Self::Tolerance = 0.0;
    const ABS_TOL: Self::Tolerance = <U as IsClose>::ABS_TOL;
    const REL_TOL: Self::Tolerance = <U as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.x.is_close_tol(&other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&other.y, rel_tol, abs_tol)
    }
}

impl<U> ops::Mul<Rotate> for Vector<U>
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

impl<U> ops::MulAssign<Rotate> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Rotate) {
        *self = *self * rhs;
    }
}

impl<U> ops::Mul<Scale> for Vector<U>
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

impl<U> ops::MulAssign<Scale> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Scale) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<U> ops::Div<Scale> for Vector<U>
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

impl<U> ops::DivAssign<Scale> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: Scale) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<U> ops::Mul<Translate<U>> for Vector<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, _rhs: Translate<U>) -> Self::Output {
        // Vectors are relative, so ignore translation
        self
    }
}

impl<U> ops::MulAssign<Translate<U>> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, _rhs: Translate<U>) {
        // Vectors are relative, so ignore translation
    }
}

impl<U> ops::Mul<Transform<U>> for Vector<U>
where
    U: Unit,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Transform<U>) -> Self::Output {
        // Vectors are relative, so ignore translation
        Self {
            x: self.x * rhs.a_xx + self.y * rhs.a_xy,
            y: self.x * rhs.a_yx + self.y * rhs.a_yy,
        }
    }
}

impl<U> ops::MulAssign<Transform<U>> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Transform<U>) {
        *self = *self * rhs;
    }
}

impl<U> ops::Div<Self> for Vector<U>
where
    U: Unit,
{
    type Output = Scale;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Scale {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<U> From<Translate<U>> for Vector<U>
where
    U: Unit,
{
    #[inline]
    fn from(value: Translate<U>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl<U> From<Vector<U>> for Translate<U>
where
    U: Unit,
{
    #[inline]
    fn from(value: Vector<U>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{Inch, Mm};

    use super::*;

    #[test]
    fn vector_new() {
        let vector = Vector::<Mm>::new(2.0, 3.0);
        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(3.0));
    }

    #[test]
    fn vector_splat() {
        let vector = Vector::<Mm>::splat(2.0);
        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(2.0));
    }

    #[test]
    fn vector_zero() {
        let vector = Vector::<Mm>::zero();
        assert_is_close!(vector.x, Mm(0.0));
        assert_is_close!(vector.y, Mm(0.0));
    }

    #[test]
    fn vector_from_units() {
        let vector = Vector::from_units(Mm(2.0), Mm(3.0));
        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(3.0));
    }

    #[test]
    fn vector_swap_xy() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        }
        .swap_xy();
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(2.0));
    }

    #[test]
    fn vector_cmp() {
        let vector1 = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let vector2 = Vector {
            x: Mm(4.0),
            y: Mm(-3.5),
        };

        assert_is_close!(vector1.max(vector2).x, Mm(4.0));
        assert_is_close!(vector1.max(vector2).y, Mm(3.0));

        assert_is_close!(vector1.min(vector2).x, Mm(2.0));
        assert_is_close!(vector1.min(vector2).y, Mm(-3.5));
    }

    #[test]
    fn vector_abs() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        assert_is_close!(vector.abs().x, Mm(2.0));
        assert_is_close!(vector.abs().y, Mm(3.0));

        let vector = Vector {
            x: Mm(4.0),
            y: Mm(-3.5),
        };
        assert_is_close!(vector.abs().x, Mm(4.0));
        assert_is_close!(vector.abs().y, Mm(3.5));
    }

    #[test]
    fn vector_hypot() {
        let vector = Vector {
            x: Mm(3.0),
            y: Mm(4.0),
        };
        assert_is_close!(vector.hypot(), 5.0);
        assert_is_close!(vector.hypot2(), 25.0);

        let vector = Vector {
            x: Mm(12.0),
            y: Mm(-5.0),
        };
        assert_is_close!(vector.hypot(), 13.0);
        assert_is_close!(vector.hypot2(), 169.0);
    }

    #[test]
    fn vector_angle() {
        let vector = Vector {
            x: Mm(12.0_f32.sqrt()),
            y: Mm(2.0),
        };
        assert_is_close!(vector.angle(), Angle::degrees(30.0));
    }

    #[test]
    fn vector_lerp() {
        let start = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let end = Vector {
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
    fn vector_convert_from() {
        let vector = Vector::<Mm>::convert_from(Vector {
            x: Inch(0.75),
            y: Inch(1.0),
        });
        assert_is_close!(vector.x, Mm(19.05));
        assert_is_close!(vector.y, Mm(25.4));
    }

    #[test]
    fn vector_add() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } + Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(3.5));
    }

    #[test]
    fn vector_add_assign() {
        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector += Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(3.5));
    }

    #[test]
    fn vector_sub() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } - Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(vector.x, Mm(1.0));
        assert_is_close!(vector.y, Mm(2.5));
    }

    #[test]
    fn vector_sub_assign() {
        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector -= Vector {
            x: Mm(1.0),
            y: Mm(0.5),
        };
        assert_is_close!(vector.x, Mm(1.0));
        assert_is_close!(vector.y, Mm(2.5));
    }

    #[test]
    fn vector_mul_f32() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } * 1.5;
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(4.5));

        // TODO: see comment by Unit
        // let vector = 1.5 * Vector{ x: Mm(2.0), y: Mm(3.0) };
        // assert_is_close!(vector.x, Mm(3.0));
        // assert_is_close!(vector.y, Mm(4.5));
    }

    #[test]
    fn vector_mul_assign_f32() {
        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector *= 1.5;
        assert_is_close!(vector.x, Mm(3.0));
        assert_is_close!(vector.y, Mm(4.5));
    }

    #[test]
    fn vector_div_f32() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } / 1.5;
        assert_is_close!(vector.x, Mm(4.0 / 3.0));
        assert_is_close!(vector.y, Mm(2.0));
    }

    #[test]
    fn vector_div_assign_f32() {
        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector /= 1.5;
        assert_is_close!(vector.x, Mm(4.0 / 3.0));
        assert_is_close!(vector.y, Mm(2.0));
    }

    #[test]
    fn vector_neg() {
        let vector = -Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        assert_is_close!(vector.x, -Mm(2.0));
        assert_is_close!(vector.y, -Mm(3.0));
    }

    #[test]
    fn vector_is_close() {
        assert!(Vector {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(&Vector {
            x: Mm(4.0 * 0.5),
            y: Mm(2.0 * 1.5)
        }));
        assert!(!Vector {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(&Vector {
            x: Mm(4.1 * 0.5),
            y: Mm(2.0 * 1.5)
        }));
        assert!(!Vector {
            x: Mm(2.0),
            y: Mm(3.0)
        }
        .is_close(&Vector {
            x: Mm(4.0 * 0.5),
            y: Mm(2.1 * 1.5)
        }));
    }

    #[test]
    fn vector_rotate() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } * Rotate::degrees(135.0);

        assert_is_close!(vector.x, Mm(-2.5 * std::f32::consts::SQRT_2));
        assert_is_close!(vector.y, Mm(-0.5 * std::f32::consts::SQRT_2));

        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector *= Rotate::degrees(135.0);

        assert_is_close!(vector.x, Mm(-2.5 * std::f32::consts::SQRT_2));
        assert_is_close!(vector.y, Mm(-0.5 * std::f32::consts::SQRT_2));
    }

    #[test]
    fn vector_scale() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } * Scale::new(2.0, 0.5);

        assert_is_close!(vector.x, Mm(4.0));
        assert_is_close!(vector.y, Mm(1.5));

        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector *= Scale::new(2.0, 0.5);

        assert_is_close!(vector.x, Mm(4.0));
        assert_is_close!(vector.y, Mm(1.5));

        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } / Scale::new(2.0, 0.5);

        assert_is_close!(vector.x, Mm(1.0));
        assert_is_close!(vector.y, Mm(6.0));

        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector /= Scale::new(2.0, 0.5);

        assert_is_close!(vector.x, Mm(1.0));
        assert_is_close!(vector.y, Mm(6.0));
    }

    #[test]
    fn vector_translate() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } * Translate::new(2.0, -1.0);

        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(3.0));

        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector *= Translate::new(2.0, -1.0);

        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(3.0));
    }

    #[test]
    fn vector_transform() {
        let transform = Transform::new(1.0, 0.5, -1.0, -0.5, 1.5, 2.0);
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        } * transform;

        assert_is_close!(vector.x, Mm(3.5));
        assert_is_close!(vector.y, Mm(3.5));

        let mut vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        vector *= transform;

        assert_is_close!(vector.x, Mm(3.5));
        assert_is_close!(vector.y, Mm(3.5));
    }

    #[test]
    fn vector_div_vector() {
        let vector1 = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let vector2 = Vector {
            x: Mm(4.0),
            y: Mm(1.5),
        };
        let scale = vector1 / vector2;

        assert_is_close!(scale.x, 0.5);
        assert_is_close!(scale.y, 2.0);
    }

    #[test]
    fn vector_into_translate() {
        let vector = Vector {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let translate = Translate::from(vector);

        assert_is_close!(translate.x, Mm(2.0));
        assert_is_close!(translate.y, Mm(3.0));
    }

    #[test]
    fn vector_from_translate() {
        let translate = Translate {
            x: Mm(2.0),
            y: Mm(3.0),
        };
        let vector = Vector::from(translate);

        assert_is_close!(vector.x, Mm(2.0));
        assert_is_close!(vector.y, Mm(3.0));
    }
}
