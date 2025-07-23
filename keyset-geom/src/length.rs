use std::ops;

use isclose::IsClose;

use crate::{ConvertFrom, ConvertInto as _, Unit};

/// A one-dimensional length with unit `U`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd)]
pub struct Length<U: Unit>(U);

impl<U> Length<U>
where
    U: Unit,
{
    /// Create a new length
    #[inline]
    pub const fn new(length: U) -> Self {
        Self(length)
    }
}

impl<U, V> ConvertFrom<Length<V>> for Length<U>
where
    U: Unit + ConvertFrom<V>,
    V: Unit,
{
    #[inline]
    fn convert_from(value: Length<V>) -> Self {
        Self(value.0.convert_into())
    }
}

impl<U> From<f32> for Length<U>
where
    U: Unit,
{
    #[inline]
    fn from(value: f32) -> Self {
        Self(U::from(value))
    }
}

impl<U> From<Length<U>> for f32
where
    U: Unit,
{
    #[inline]
    fn from(value: Length<U>) -> Self {
        value.0.into()
    }
}

impl<U> ops::Add for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Add>::Output>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<U> ops::AddAssign for Length<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<U> ops::Sub for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Sub>::Output>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<U> ops::SubAssign for Length<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<U> ops::Mul<f32> for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Mul<f32>>::Output>;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<U> ops::MulAssign<f32> for Length<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl<U> ops::Div<f32> for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Div<f32>>::Output>;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<U> ops::Div for Length<U>
where
    U: Unit,
{
    type Output = <U as ops::Div>::Output;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl<U> ops::DivAssign<f32> for Length<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl<U> ops::Neg for Length<U>
where
    U: Unit,
{
    type Output = Length<<U as ops::Neg>::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<U> IsClose<f32> for Length<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <U as IsClose<f32>>::ABS_TOL;
    const REL_TOL: f32 = <U as IsClose<f32>>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.0.is_close_impl(&other.0, rel_tol, abs_tol)
    }
}

impl<U> Length<U>
where
    U: Unit,
{
    /// Linearly interpolate between two length values
    #[inline]
    #[must_use]
    pub fn lerp(self, other: Self, factor: f32) -> Self {
        self * (1.0 - factor) + other * factor
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
        let length = Length::new(Mm(2.0));
        assert_is_close!(length.0, Mm(2.0));
    }

    #[test]
    fn length_from_unit() {
        let length = Length::<Mm>::convert_from(Length(Inch(0.75)));
        assert_is_close!(length.0, Mm(19.05));
    }

    #[test]
    fn length_from_f32() {
        let length = Length::<Mm>::from(2.0);
        assert_is_close!(length.0, Mm(2.0));
    }

    #[test]
    fn length_into_f32() {
        let length = Length(Mm(2.0));
        let value = f32::from(length);
        assert_is_close!(value, 2.0);
    }

    #[test]
    fn length_add() {
        let length = Length(Mm(2.0)) + Length(Mm(1.0));
        assert_is_close!(length.0, Mm(3.0));
    }

    #[test]
    fn length_add_assign() {
        let mut length = Length(Mm(2.0));
        length += Length(Mm(1.0));
        assert_is_close!(length.0, Mm(3.0));
    }

    #[test]
    fn length_sub() {
        let length = Length(Mm(2.0)) - Length(Mm(1.0));
        assert_is_close!(length.0, Mm(1.0));
    }

    #[test]
    fn length_sub_assign() {
        let mut length = Length(Mm(2.0));
        length -= Length(Mm(1.0));
        assert_is_close!(length.0, Mm(1.0));
    }

    #[test]
    fn length_mul() {
        let length = Length(Mm(2.0)) * 1.5;
        assert_is_close!(length.0, Mm(3.0));

        // TODO: see comment by Unit
        // let length = 2.0 * Dist (Mm(1.5));
        // assert_is_close!(length.0, Mm(3.0));
    }

    #[test]
    fn length_mul_assign() {
        let mut length = Length(Mm(2.0));
        length *= 1.5;
        assert_is_close!(length.0, Mm(3.0));
    }

    #[test]
    fn length_div() {
        let length = Length(Mm(3.0)) / 1.5;
        assert_is_close!(length.0, Mm(2.0));

        let ratio = Length(Mm(3.0)) / Length(Mm(2.0));
        assert_is_close!(ratio, 1.5);
    }

    #[test]
    fn length_div_assign() {
        let mut length = Length(Mm(3.0));
        length /= 1.5;
        assert_is_close!(length.0, Mm(2.0));
    }

    #[test]
    fn length_neg() {
        let length = -Length(Mm(2.0));
        assert_is_close!(length.0, Mm(-2.0));
    }

    #[test]
    fn length_is_close() {
        assert!(Length(Mm(2.5)).is_close(Length(Mm(5.0 / 2.0))));
        assert!(!Length(Mm(2.5)).is_close(Length(Mm(5.1 / 2.0))));
    }

    #[test]
    fn length_lerp() {
        let start = Length(Mm(2.0));
        let end = Length(Mm(4.0));

        assert_is_close!(start.lerp(end, 0.0).0, Mm(2.0));
        assert_is_close!(start.lerp(end, 0.5).0, Mm(3.0));
        assert_is_close!(start.lerp(end, 1.0).0, Mm(4.0));
    }
}
