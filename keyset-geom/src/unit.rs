use std::ops;

use isclose::IsClose;

use crate::Scale;

/// Trait for Unit types
pub trait Unit:
    Sized
    + ops::Add<Self, Output = Self>
    + ops::AddAssign<Self>
    + ops::Sub<Self, Output = Self>
    + ops::SubAssign<Self>
    + ops::Mul<f32, Output = Self>
    + ops::MulAssign<f32>
    + ops::Div<Self, Output = f32>
    + ops::Div<f32, Output = Self>
    + ops::DivAssign<f32>
    + ops::Neg<Output = Self>
    + IsClose<f32>
// TODO: seems to trigger rust-lang/rust#96634
// where
//     f32: ops::Mul<Self, Output = Self>,
{
}

/// Keyboard Unit, usually 19.05 mm or 0.75 in
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct KeyUnit(pub f32);

impl KeyUnit {
    const PER_DOT: f32 = 1.0 / Dot::PER_UNIT;
    const PER_MM: f32 = 1.0 / Mm::PER_UNIT;
    const PER_INCH: f32 = 1.0 / Inch::PER_UNIT;
}

impl From<Dot> for KeyUnit {
    #[inline]
    fn from(value: Dot) -> Self {
        Self(value.0 * Self::PER_DOT)
    }
}

impl From<Mm> for KeyUnit {
    #[inline]
    fn from(value: Mm) -> Self {
        Self(value.0 * Self::PER_MM)
    }
}

impl From<Inch> for KeyUnit {
    #[inline]
    fn from(value: Inch) -> Self {
        Self(value.0 * Self::PER_INCH)
    }
}

impl From<f32> for KeyUnit {
    #[inline]
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<KeyUnit> for f32 {
    #[inline]
    fn from(value: KeyUnit) -> Self {
        value.0
    }
}

impl ops::Add for KeyUnit {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign for KeyUnit {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::Sub for KeyUnit {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::SubAssign for KeyUnit {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl ops::Mul<f32> for KeyUnit {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::Mul<KeyUnit> for f32 {
    type Output = KeyUnit;

    #[inline]
    fn mul(self, rhs: KeyUnit) -> KeyUnit {
        KeyUnit(self * rhs.0)
    }
}

impl ops::MulAssign<f32> for KeyUnit {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl ops::Div for KeyUnit {
    type Output = f32;

    #[inline]
    fn div(self, rhs: Self) -> f32 {
        self.0 / rhs.0
    }
}

impl ops::Div<f32> for KeyUnit {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs)
    }
}

impl ops::DivAssign<f32> for KeyUnit {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl ops::Neg for KeyUnit {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl IsClose<f32> for KeyUnit {
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

impl Unit for KeyUnit {}

/// Dot, a.k.a. drawing unit
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Dot(pub f32);

impl Dot {
    const PER_UNIT: f32 = 1000.0;
    const PER_MM: f32 = Self::PER_UNIT / Mm::PER_UNIT;
    const PER_INCH: f32 = Self::PER_UNIT / Inch::PER_UNIT;
}

impl From<KeyUnit> for Dot {
    #[inline]
    fn from(value: KeyUnit) -> Self {
        Self(value.0 * Self::PER_UNIT)
    }
}

impl From<Mm> for Dot {
    #[inline]
    fn from(value: Mm) -> Self {
        Self(value.0 * Self::PER_MM)
    }
}

impl From<Inch> for Dot {
    #[inline]
    fn from(value: Inch) -> Self {
        Self(value.0 * Self::PER_INCH)
    }
}

impl From<f32> for Dot {
    #[inline]
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Dot> for f32 {
    #[inline]
    fn from(value: Dot) -> Self {
        value.0
    }
}

impl ops::Add for Dot {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign for Dot {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::Sub for Dot {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::SubAssign for Dot {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl ops::Mul<f32> for Dot {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::Mul<Dot> for f32 {
    type Output = Dot;

    #[inline]
    fn mul(self, rhs: Dot) -> Dot {
        Dot(self * rhs.0)
    }
}

impl ops::MulAssign<f32> for Dot {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl ops::Div for Dot {
    type Output = f32;

    #[inline]
    fn div(self, rhs: Self) -> f32 {
        self.0 / rhs.0
    }
}

impl ops::Div<f32> for Dot {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs)
    }
}

impl ops::DivAssign<f32> for Dot {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl ops::Neg for Dot {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl IsClose<f32> for Dot {
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

impl Unit for Dot {}

/// Millimeter
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Mm(pub f32);

impl Mm {
    const PER_UNIT: f32 = 19.05;
    const PER_DOT: f32 = Self::PER_UNIT / Dot::PER_UNIT;
    const PER_INCH: f32 = Self::PER_UNIT / Inch::PER_UNIT;
}

impl From<KeyUnit> for Mm {
    #[inline]
    fn from(value: KeyUnit) -> Self {
        Self(value.0 * Self::PER_UNIT)
    }
}

impl From<Dot> for Mm {
    #[inline]
    fn from(value: Dot) -> Self {
        Self(value.0 * Self::PER_DOT)
    }
}

impl From<Inch> for Mm {
    #[inline]
    fn from(value: Inch) -> Self {
        Self(value.0 * Self::PER_INCH)
    }
}

impl From<f32> for Mm {
    #[inline]
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Mm> for f32 {
    #[inline]
    fn from(value: Mm) -> Self {
        value.0
    }
}

impl ops::Add for Mm {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign for Mm {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::Sub for Mm {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::SubAssign for Mm {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl ops::Mul<f32> for Mm {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::Mul<Mm> for f32 {
    type Output = Mm;

    #[inline]
    fn mul(self, rhs: Mm) -> Mm {
        Mm(self * rhs.0)
    }
}

impl ops::MulAssign<f32> for Mm {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl ops::Div for Mm {
    type Output = f32;

    #[inline]
    fn div(self, rhs: Self) -> f32 {
        self.0 / rhs.0
    }
}

impl ops::Div<f32> for Mm {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs)
    }
}

impl ops::DivAssign<f32> for Mm {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl ops::Neg for Mm {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl IsClose<f32> for Mm {
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

impl Unit for Mm {}

/// Inch
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Inch(pub f32);

impl Inch {
    const PER_UNIT: f32 = 0.75;
    const PER_DOT: f32 = Self::PER_UNIT / Dot::PER_UNIT;
    const PER_MM: f32 = Self::PER_UNIT / Mm::PER_UNIT;
}

impl From<KeyUnit> for Inch {
    #[inline]
    fn from(value: KeyUnit) -> Self {
        Self(value.0 * Self::PER_UNIT)
    }
}

impl From<Dot> for Inch {
    #[inline]
    fn from(value: Dot) -> Self {
        Self(value.0 * Self::PER_DOT)
    }
}

impl From<Mm> for Inch {
    #[inline]
    fn from(value: Mm) -> Self {
        Self(value.0 * Self::PER_MM)
    }
}

impl From<f32> for Inch {
    #[inline]
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Inch> for f32 {
    #[inline]
    fn from(value: Inch) -> Self {
        value.0
    }
}

impl ops::Add for Inch {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign for Inch {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::Sub for Inch {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::SubAssign for Inch {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl ops::Mul<f32> for Inch {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::Mul<Inch> for f32 {
    type Output = Inch;

    #[inline]
    fn mul(self, rhs: Inch) -> Inch {
        Inch(self * rhs.0)
    }
}

impl ops::MulAssign<f32> for Inch {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl ops::Div for Inch {
    type Output = f32;

    #[inline]
    fn div(self, rhs: Self) -> f32 {
        self.0 / rhs.0
    }
}

impl ops::Div<f32> for Inch {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs)
    }
}

impl ops::DivAssign<f32> for Inch {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl ops::Neg for Inch {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl IsClose<f32> for Inch {
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

impl Unit for Inch {}

// TODO: delete these when no longer used
/// Conversion factor for keyboard units to drawing units
pub const DOT_PER_UNIT: Scale<KeyUnit, Dot> = Scale::new(Dot::PER_UNIT);
/// Conversion factor for keyboard units to millimeters
pub const MM_PER_UNIT: Scale<KeyUnit, Mm> = Scale::new(Mm::PER_UNIT);
/// Conversion factor for keyboard units to inches
pub const INCH_PER_UNIT: Scale<KeyUnit, Inch> = Scale::new(Inch::PER_UNIT);

/// Conversion factor for Millimeters to Drawing Units
pub const DOT_PER_MM: Scale<Mm, Dot> = Scale::new(Dot::PER_MM);
/// Conversion factor for Inches to Drawing Units
pub const DOT_PER_INCH: Scale<Inch, Dot> = Scale::new(Dot::PER_INCH);

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn key_unit() {
        let key_unit = KeyUnit::from(Dot(500.0));
        assert_is_close!(key_unit.0, 0.5);

        let key_unit = KeyUnit::from(Mm(38.1));
        assert_is_close!(key_unit.0, 2.0);

        let key_unit = KeyUnit::from(Inch(1.0));
        assert_is_close!(key_unit.0, 4.0 / 3.0);

        let key_unit = KeyUnit::from(3.0);
        assert_is_close!(key_unit.0, 3.0);

        let flt = f32::from(KeyUnit(2.5));
        assert_is_close!(flt, 2.5);

        let flt = f32::from(KeyUnit(2.5));
        assert_is_close!(flt, 2.5);
    }

    #[test]
    fn key_unit_add() {
        let key_unit = KeyUnit(2.0) + KeyUnit(1.0);
        assert_is_close!(key_unit.0, 3.0);
    }

    #[test]
    fn key_unit_add_assign() {
        let mut key_unit = KeyUnit(2.0);
        key_unit += KeyUnit(1.0);
        assert_is_close!(key_unit.0, 3.0);
    }

    #[test]
    fn key_unit_sub() {
        let key_unit = KeyUnit(2.0) - KeyUnit(1.0);
        assert_is_close!(key_unit.0, 1.0);
    }

    #[test]
    fn key_unit_sub_assign() {
        let mut key_unit = KeyUnit(2.0);
        key_unit -= KeyUnit(1.0);
        assert_is_close!(key_unit.0, 1.0);
    }

    #[test]
    fn key_unit_mul() {
        let key_unit = KeyUnit(2.0) * 1.5;
        assert_is_close!(key_unit.0, 3.0);

        let key_unit = 2.0 * KeyUnit(1.5);
        assert_is_close!(key_unit.0, 3.0);
    }

    #[test]
    fn key_unit_mul_assign() {
        let mut key_unit = KeyUnit(2.0);
        key_unit *= 1.5;
        assert_is_close!(key_unit.0, 3.0);
    }

    #[test]
    fn key_unit_div() {
        let key_unit = KeyUnit(3.0) / 1.5;
        assert_is_close!(key_unit.0, 2.0);

        let ratio = KeyUnit(3.0) / KeyUnit(2.0);
        assert_is_close!(ratio, 1.5);
    }

    #[test]
    fn key_unit_div_assign() {
        let mut key_unit = KeyUnit(3.0);
        key_unit /= 1.5;
        assert_is_close!(key_unit.0, 2.0);
    }

    #[test]
    fn key_unit_neg() {
        let key_unit = -KeyUnit(2.0);
        assert_is_close!(key_unit.0, -2.0);
    }

    #[test]
    fn key_unit_is_close() {
        assert!(KeyUnit(2.5).is_close(KeyUnit(5.0 / 2.0)));
        assert!(!KeyUnit(2.5).is_close(KeyUnit(5.1 / 2.0)));
    }

    #[test]
    fn dot() {
        let dot = Dot::from(KeyUnit(0.5));
        assert_is_close!(dot.0, 500.0);

        let dot = Dot::from(Mm(38.1));
        assert_is_close!(dot.0, 2000.0);

        let dot = Dot::from(Inch(1.0));
        assert_is_close!(dot.0, 4000.0 / 3.0);

        let dot = Dot::from(3.0);
        assert_is_close!(dot.0, 3.0);

        let flt = f32::from(Dot(2.5));
        assert_is_close!(flt, 2.5);

        let flt = f32::from(Dot(2.5));
        assert_is_close!(flt, 2.5);
    }

    #[test]
    fn dot_add() {
        let dot = Dot(2.0) + Dot(1.0);
        assert_is_close!(dot.0, 3.0);
    }

    #[test]
    fn dot_add_assign() {
        let mut dot = Dot(2.0);
        dot += Dot(1.0);
        assert_is_close!(dot.0, 3.0);
    }

    #[test]
    fn dot_sub() {
        let dot = Dot(2.0) - Dot(1.0);
        assert_is_close!(dot.0, 1.0);
    }

    #[test]
    fn dot_sub_assign() {
        let mut dot = Dot(2.0);
        dot -= Dot(1.0);
        assert_is_close!(dot.0, 1.0);
    }

    #[test]
    fn dot_mul() {
        let dot = Dot(2.0) * 1.5;
        assert_is_close!(dot.0, 3.0);

        let dot = 2.0 * Dot(1.5);
        assert_is_close!(dot.0, 3.0);
    }

    #[test]
    fn dot_mul_assign() {
        let mut dot = Dot(2.0);
        dot *= 1.5;
        assert_is_close!(dot.0, 3.0);
    }

    #[test]
    fn dot_div() {
        let dot = Dot(3.0) / 1.5;
        assert_is_close!(dot.0, 2.0);

        let ratio = Dot(3.0) / Dot(2.0);
        assert_is_close!(ratio, 1.5);
    }

    #[test]
    fn dot_div_assign() {
        let mut dot = Dot(3.0);
        dot /= 1.5;
        assert_is_close!(dot.0, 2.0);
    }

    #[test]
    fn dot_neg() {
        let dot = -Dot(2.0);
        assert_is_close!(dot.0, -2.0);
    }

    #[test]
    fn dots_close() {
        assert!(Dot(2.5).is_close(Dot(5.0 / 2.0)));
        assert!(!Dot(2.5).is_close(Dot(5.1 / 2.0)));
    }

    #[test]
    fn mm() {
        let mm = Mm::from(KeyUnit(0.5));
        assert_is_close!(mm.0, 9.525);

        let mm = Mm::from(Dot(2000.0));
        assert_is_close!(mm.0, 38.1);

        let mm = Mm::from(Inch(1.0));
        assert_is_close!(mm.0, 25.4);

        let mm = Mm::from(3.0);
        assert_is_close!(mm.0, 3.0);

        let flt = f32::from(Mm(2.5));
        assert_is_close!(flt, 2.5);

        let flt = f32::from(Mm(2.5));
        assert_is_close!(flt, 2.5);
    }

    #[test]
    fn mm_add() {
        let mm = Mm(2.0) + Mm(1.0);
        assert_is_close!(mm.0, 3.0);
    }

    #[test]
    fn mm_add_assign() {
        let mut mm = Mm(2.0);
        mm += Mm(1.0);
        assert_is_close!(mm.0, 3.0);
    }

    #[test]
    fn mm_sub() {
        let mm = Mm(2.0) - Mm(1.0);
        assert_is_close!(mm.0, 1.0);
    }

    #[test]
    fn mm_sub_assign() {
        let mut mm = Mm(2.0);
        mm -= Mm(1.0);
        assert_is_close!(mm.0, 1.0);
    }

    #[test]
    fn mm_mul() {
        let mm = Mm(2.0) * 1.5;
        assert_is_close!(mm.0, 3.0);

        let mm = 2.0 * Mm(1.5);
        assert_is_close!(mm.0, 3.0);
    }

    #[test]
    fn mm_mul_assign() {
        let mut mm = Mm(2.0);
        mm *= 1.5;
        assert_is_close!(mm.0, 3.0);
    }

    #[test]
    fn mm_div() {
        let mm = Mm(3.0) / 1.5;
        assert_is_close!(mm.0, 2.0);

        let ratio = Mm(3.0) / Mm(2.0);
        assert_is_close!(ratio, 1.5);
    }

    #[test]
    fn mm_div_assign() {
        let mut mm = Mm(3.0);
        mm /= 1.5;
        assert_is_close!(mm.0, 2.0);
    }

    #[test]
    fn mm_neg() {
        let mm = -Mm(2.0);
        assert_is_close!(mm.0, -2.0);
    }

    #[test]
    fn mm_is_close() {
        assert!(Mm(2.5).is_close(Mm(5.0 / 2.0)));
        assert!(!Mm(2.5).is_close(Mm(5.1 / 2.0)));
    }

    #[test]
    fn inch() {
        let inch = Inch::from(KeyUnit(4.0 / 3.0));
        assert_is_close!(inch.0, 1.0);

        let inch = Inch::from(Dot(2000.0));
        assert_is_close!(inch.0, 1.5);

        let inch = Inch::from(Mm(19.05));
        assert_is_close!(inch.0, 0.75);

        let inch = Inch::from(3.0);
        assert_is_close!(inch.0, 3.0);

        let flt = f32::from(Inch(2.5));
        assert_is_close!(flt, 2.5);

        let flt = f32::from(Inch(2.5));
        assert_is_close!(flt, 2.5);
    }

    #[test]
    fn inch_add() {
        let inch = Inch(2.0) + Inch(1.0);
        assert_is_close!(inch.0, 3.0);
    }

    #[test]
    fn inch_add_assign() {
        let mut inch = Inch(2.0);
        inch += Inch(1.0);
        assert_is_close!(inch.0, 3.0);
    }

    #[test]
    fn inch_sub() {
        let inch = Inch(2.0) - Inch(1.0);
        assert_is_close!(inch.0, 1.0);
    }

    #[test]
    fn inch_sub_assign() {
        let mut inch = Inch(2.0);
        inch -= Inch(1.0);
        assert_is_close!(inch.0, 1.0);
    }

    #[test]
    fn inch_mul() {
        let inch = Inch(2.0) * 1.5;
        assert_is_close!(inch.0, 3.0);

        let inch = 2.0 * Inch(1.5);
        assert_is_close!(inch.0, 3.0);
    }

    #[test]
    fn inch_mul_assign() {
        let mut inch = Inch(2.0);
        inch *= 1.5;
        assert_is_close!(inch.0, 3.0);
    }

    #[test]
    fn inch_div() {
        let inch = Inch(3.0) / 1.5;
        assert_is_close!(inch.0, 2.0);

        let ratio = Inch(3.0) / Inch(2.0);
        assert_is_close!(ratio, 1.5);
    }

    #[test]
    fn inch_div_assign() {
        let mut inch = Inch(3.0);
        inch /= 1.5;
        assert_is_close!(inch.0, 2.0);
    }

    #[test]
    fn inch_neg() {
        let inch = -Inch(2.0);
        assert_is_close!(inch.0, -2.0);
    }

    #[test]
    fn inch_is_close() {
        assert!(Inch(2.5).is_close(Inch(5.0 / 2.0)));
        assert!(!Inch(2.5).is_close(Inch(5.1 / 2.0)));
    }
}
