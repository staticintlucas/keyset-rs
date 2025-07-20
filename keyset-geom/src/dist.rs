use std::ops;

use isclose::IsClose;

use crate::{FromUnit, IntoUnit as _, Unit};

/// A one-dimensional distance with unit `U`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd)]
pub struct Dist<U: Unit> {
    dist: U,
}

impl<U> Dist<U>
where
    U: Unit,
{
    /// Create a new distance
    #[inline]
    pub const fn new(dist: U) -> Self {
        Self { dist }
    }
}

impl<U, V> FromUnit<Dist<V>> for Dist<U>
where
    U: Unit + FromUnit<V>,
    V: Unit,
{
    #[inline]
    fn from_unit(value: Dist<V>) -> Self {
        Self {
            dist: value.dist.into_unit(),
        }
    }
}

impl<U> From<f32> for Dist<U>
where
    U: Unit,
{
    #[inline]
    fn from(value: f32) -> Self {
        Self {
            dist: U::from(value),
        }
    }
}

impl<U> From<Dist<U>> for f32
where
    U: Unit,
{
    #[inline]
    fn from(value: Dist<U>) -> Self {
        value.dist.into()
    }
}

impl<U> ops::Add for Dist<U>
where
    U: Unit,
{
    type Output = Dist<<U as ops::Add>::Output>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            dist: self.dist + rhs.dist,
        }
    }
}

impl<U> ops::AddAssign for Dist<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.dist += rhs.dist;
    }
}

impl<U> ops::Sub for Dist<U>
where
    U: Unit,
{
    type Output = Dist<<U as ops::Sub>::Output>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            dist: self.dist - rhs.dist,
        }
    }
}

impl<U> ops::SubAssign for Dist<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.dist -= rhs.dist;
    }
}

impl<U> ops::Mul<f32> for Dist<U>
where
    U: Unit,
{
    type Output = Dist<<U as ops::Mul<f32>>::Output>;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            dist: self.dist * rhs,
        }
    }
}

impl<U> ops::MulAssign<f32> for Dist<U>
where
    U: Unit,
{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.dist *= rhs;
    }
}

impl<U> ops::Div<f32> for Dist<U>
where
    U: Unit,
{
    type Output = Dist<<U as ops::Div<f32>>::Output>;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            dist: self.dist / rhs,
        }
    }
}

impl<U> ops::Div for Dist<U>
where
    U: Unit,
{
    type Output = <U as ops::Div>::Output;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.dist / rhs.dist
    }
}

impl<U> ops::DivAssign<f32> for Dist<U>
where
    U: Unit,
{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.dist /= rhs;
    }
}

impl<U> ops::Neg for Dist<U>
where
    U: Unit,
{
    type Output = Dist<<U as ops::Neg>::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { dist: -self.dist }
    }
}

impl<U> IsClose<f32> for Dist<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <U as IsClose<f32>>::ABS_TOL;
    const REL_TOL: f32 = <U as IsClose<f32>>::REL_TOL;

    #[inline]
    fn is_close_tol(
        &self,
        other: impl std::borrow::Borrow<Self>,
        rel_tol: impl std::borrow::Borrow<f32>,
        abs_tol: impl std::borrow::Borrow<f32>,
    ) -> bool {
        <U as IsClose<f32>>::is_close_tol(&self.dist, other.borrow().dist, rel_tol, abs_tol)
    }
}

impl<U> Dist<U>
where
    U: Unit,
{
    /// Linearly interpolate between two distance values
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

    use crate::Mm;

    use super::*;

    #[test]
    fn dist_new() {
        let dist = Dist::new(Mm(2.0));
        assert_is_close!(dist.dist, Mm(2.0));
    }

    #[test]
    fn dist_add() {
        let dist = Dist { dist: Mm(2.0) } + Dist { dist: Mm(1.0) };
        assert_is_close!(dist.dist, Mm(3.0));
    }

    #[test]
    fn dist_add_assign() {
        let mut dist = Dist { dist: Mm(2.0) };
        dist += Dist { dist: Mm(1.0) };
        assert_is_close!(dist.dist, Mm(3.0));
    }

    #[test]
    fn dist_sub() {
        let dist = Dist { dist: Mm(2.0) } - Dist { dist: Mm(1.0) };
        assert_is_close!(dist.dist, Mm(1.0));
    }

    #[test]
    fn dist_sub_assign() {
        let mut dist = Dist { dist: Mm(2.0) };
        dist -= Dist { dist: Mm(1.0) };
        assert_is_close!(dist.dist, Mm(1.0));
    }

    #[test]
    fn dist_mul() {
        let dist = Dist { dist: Mm(2.0) } * 1.5;
        assert_is_close!(dist.dist, Mm(3.0));

        // TODO: see comment by Unit
        // let dist = 2.0 * Dist { dist: Mm(1.5) };
        // assert_is_close!(dist.dist, Mm(3.0));
    }

    #[test]
    fn dist_mul_assign() {
        let mut dist = Dist { dist: Mm(2.0) };
        dist *= 1.5;
        assert_is_close!(dist.dist, Mm(3.0));
    }

    #[test]
    fn dist_div() {
        let dist = Dist { dist: Mm(3.0) } / 1.5;
        assert_is_close!(dist.dist, Mm(2.0));

        let ratio = Dist { dist: Mm(3.0) } / Dist { dist: Mm(2.0) };
        assert_is_close!(ratio, 1.5);
    }

    #[test]
    fn dist_div_assign() {
        let mut dist = Dist { dist: Mm(3.0) };
        dist /= 1.5;
        assert_is_close!(dist.dist, Mm(2.0));
    }

    #[test]
    fn dist_neg() {
        let dist = -Dist { dist: Mm(2.0) };
        assert_is_close!(dist.dist, Mm(-2.0));
    }

    #[test]
    fn dist_is_close() {
        assert!(Dist { dist: Mm(2.5) }.is_close(Dist {
            dist: Mm(5.0 / 2.0)
        }));
        assert!(!Dist { dist: Mm(2.5) }.is_close(Dist {
            dist: Mm(5.1 / 2.0)
        }));
    }
}
