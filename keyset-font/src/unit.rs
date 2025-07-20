use std::ops;

use geom::Unit;
use isclose::IsClose;

/// Unit within a font
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct FontUnit(pub f32);

impl From<f32> for FontUnit {
    #[inline]
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<FontUnit> for f32 {
    #[inline]
    fn from(value: FontUnit) -> Self {
        value.0
    }
}

impl ops::Add for FontUnit {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign for FontUnit {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::Sub for FontUnit {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::SubAssign for FontUnit {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl ops::Mul<f32> for FontUnit {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::Mul<FontUnit> for f32 {
    type Output = FontUnit;

    #[inline]
    fn mul(self, rhs: FontUnit) -> FontUnit {
        FontUnit(self * rhs.0)
    }
}

impl ops::MulAssign<f32> for FontUnit {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl ops::Div for FontUnit {
    type Output = f32;

    #[inline]
    fn div(self, rhs: Self) -> f32 {
        self.0 / rhs.0
    }
}

impl ops::Div<f32> for FontUnit {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs)
    }
}

impl ops::DivAssign<f32> for FontUnit {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl ops::Neg for FontUnit {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl IsClose<f32> for FontUnit {
    const ABS_TOL: f32 = <f32 as IsClose>::ABS_TOL;
    const REL_TOL: f32 = <f32 as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(
        &self,
        other: impl std::borrow::Borrow<Self>,
        rel_tol: impl std::borrow::Borrow<f32>,
        abs_tol: impl std::borrow::Borrow<f32>,
    ) -> bool {
        self.0.is_close_tol(other.borrow().0, abs_tol, rel_tol)
    }
}

impl Unit for FontUnit {}
