use std::ops;

use isclose::IsClose;

use crate::{ConvertFrom, ConvertInto as _, Unit};

/// A one-dimensional length with unit `U`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd)]
pub struct Length<U: Unit> {
    /// The length value in units of `U`
    pub length: U,
}

impl<U> Length<U>
where
    U: Unit,
{
    /// Create a new length
    #[inline]
    #[must_use]
    pub fn new(length: f32) -> Self {
        Self {
            length: U::new(length),
        }
    }

    /// Create a new length from a unit
    #[inline]
    #[must_use]
    pub const fn from_unit(length: U) -> Self {
        Self { length }
    }

    /// Returns the minimum length from `self` and `other`
    #[inline]
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self {
            length: self.length.min(other.length),
        }
    }

    /// Returns the maximum length from `self` and `other`
    #[inline]
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self {
            length: self.length.max(other.length),
        }
    }

    /// Returns the absolute value of the length
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self {
            length: self.length.abs(),
        }
    }

    /// Linearly interpolate between two length values
    #[inline]
    #[must_use]
    pub fn lerp(self, other: Self, factor: f32) -> Self {
        self * (1.0 - factor) + other * factor
    }
}

impl<U, V> ConvertFrom<Length<V>> for Length<U>
where
    U: Unit + ConvertFrom<V>,
    V: Unit,
{
    #[inline]
    fn convert_from(value: Length<V>) -> Self {
        Self {
            length: value.length.convert_into(),
        }
    }
}

impl<U> ops::Add for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Add>::Output>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            length: self.length + rhs.length,
        }
    }
}

impl<U> ops::AddAssign for Length<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.length += rhs.length;
    }
}

impl<U> ops::Sub for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Sub>::Output>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            length: self.length - rhs.length,
        }
    }
}

impl<U> ops::SubAssign for Length<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.length -= rhs.length;
    }
}

impl<U> ops::Mul<f32> for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Mul<f32>>::Output>;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            length: self.length * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for Length<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.length *= rhs;
    }
}

impl<U> ops::Div<f32> for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Div<f32>>::Output>;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            length: self.length / rhs,
        }
    }
}

impl<U> ops::Div for Length<U>
where
    U: Unit,
{
    type Output = <U as ops::Div>::Output;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.length / rhs.length
    }
}

impl<U> ops::DivAssign<f32> for Length<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.length /= rhs;
    }
}

impl<U> ops::Neg for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Neg>::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            length: -self.length,
        }
    }
}

impl<U> IsClose for Length<U>
where
    U: Unit,
{
    type Tolerance = f32;
    const ZERO_TOL: Self::Tolerance = 0.0;
    const ABS_TOL: Self::Tolerance = <U as IsClose>::ABS_TOL;
    const REL_TOL: Self::Tolerance = <U as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.length.is_close_tol(&other.length, rel_tol, abs_tol)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::{Inch, Mm};

    use super::*;

    #[test]
    fn length_new() {
        let length = Length::<Mm>::new(2.0);
        assert_is_close!(length.length, Mm(2.0));
    }

    #[test]
    fn length_from_unit() {
        let length = Length::from_unit(Mm(2.0));
        assert_is_close!(length.length, Mm(2.0));
    }

    #[test]
    fn length_cmp() {
        let length1 = Length {
            length: Mm(4.0 / 3.0),
        };
        let length2 = Length { length: Mm(1.5) };

        assert_is_close!(length1.max(length2).length, Mm(1.5));
        assert_is_close!(length1.min(length2).length, Mm(4.0 / 3.0));
    }

    #[test]
    fn length_abs() {
        let length = Length { length: Mm(2.5) };
        assert_is_close!(length.abs().length, Mm(2.5));

        let length = Length { length: Mm(-2.5) };
        assert_is_close!(length.abs().length, Mm(2.5));
    }

    #[test]
    fn length_lerp() {
        let start = Length { length: Mm(2.0) };
        let end = Length { length: Mm(4.0) };

        assert_is_close!(start.lerp(end, 0.0).length, Mm(2.0));
        assert_is_close!(start.lerp(end, 0.5).length, Mm(3.0));
        assert_is_close!(start.lerp(end, 1.0).length, Mm(4.0));
    }

    #[test]
    fn length_convert_from() {
        let length = Length::<Mm>::convert_from(Length { length: Inch(0.75) });
        assert_is_close!(length.length, Mm(19.05));
    }

    #[test]
    fn length_add() {
        let length = Length { length: Mm(2.0) } + Length { length: Mm(1.0) };
        assert_is_close!(length.length, Mm(3.0));
    }

    #[test]
    fn length_add_assign() {
        let mut length = Length { length: Mm(2.0) };
        length += Length { length: Mm(1.0) };
        assert_is_close!(length.length, Mm(3.0));
    }

    #[test]
    fn length_sub() {
        let length = Length { length: Mm(2.0) } - Length { length: Mm(1.0) };
        assert_is_close!(length.length, Mm(1.0));
    }

    #[test]
    fn length_sub_assign() {
        let mut length = Length { length: Mm(2.0) };
        length -= Length { length: Mm(1.0) };
        assert_is_close!(length.length, Mm(1.0));
    }

    #[test]
    fn length_mul() {
        let length = Length { length: Mm(2.0) } * 1.5;
        assert_is_close!(length.length, Mm(3.0));

        // TODO: see comment by Unit
        // let length = 2.0 * Dist (Mm(1.5));
        // assert_is_close!(length.length, Mm(3.0));
    }

    #[test]
    fn length_mul_assign() {
        let mut length = Length { length: Mm(2.0) };
        length *= 1.5;
        assert_is_close!(length.length, Mm(3.0));
    }

    #[test]
    fn length_div() {
        let length = Length { length: Mm(3.0) } / 1.5;
        assert_is_close!(length.length, Mm(2.0));

        let ratio = Length { length: Mm(3.0) } / Length { length: Mm(2.0) };
        assert_is_close!(ratio, 1.5);
    }

    #[test]
    fn length_div_assign() {
        let mut length = Length { length: Mm(3.0) };
        length /= 1.5;
        assert_is_close!(length.length, Mm(2.0));
    }

    #[test]
    fn length_neg() {
        let length = -Length { length: Mm(2.0) };
        assert_is_close!(length.length, Mm(-2.0));
    }

    #[test]
    fn length_is_close() {
        assert!(Length { length: Mm(2.5) }.is_close(&Length {
            length: Mm(5.0 / 2.0)
        }));
        assert!(!Length { length: Mm(2.5) }.is_close(&Length {
            length: Mm(5.1 / 2.0)
        }));
    }
}
