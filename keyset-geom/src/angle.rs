use std::f32::consts;
use std::ops;

use isclose::IsClose;

/// An angle in radians
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Angle {
    radians: f32,
}

impl Angle {
    /// An angle of 0.0 radians
    pub const ZERO: Self = Self::radians(0.0);
    /// An angle of π radians (180°)
    pub const PI: Self = Self::radians(consts::PI);
    /// An angle of τ (2π) radians (360°)
    pub const TAU: Self = Self::radians(consts::TAU);
    /// An angle of π/2 radians (90°)
    pub const FRAC_PI_2: Self = Self::radians(consts::FRAC_PI_2);
    /// An angle of π/3 radians (60°)
    pub const FRAC_PI_3: Self = Self::radians(consts::FRAC_PI_3);
    /// An angle of π/4 radians (45°)
    pub const FRAC_PI_4: Self = Self::radians(consts::FRAC_PI_4);

    /// Creates a new [`Angle`] with the given value in radians
    #[inline]
    #[must_use]
    pub const fn radians(radians: f32) -> Self {
        Self { radians }
    }

    /// Returns the angle as an [`f32`] value measured in radians
    #[inline]
    #[must_use]
    pub const fn to_radians(self) -> f32 {
        self.radians
    }

    /// Creates a new [`Angle`] with the given value in degrees
    #[inline]
    #[must_use]
    pub fn degrees(degrees: f32) -> Self {
        Self {
            radians: degrees.to_radians(),
        }
    }

    /// Returns the angle as an [`f32`] value measured in degrees
    #[inline]
    #[must_use]
    pub fn to_degrees(self) -> f32 {
        self.radians.to_degrees()
    }

    /// Normalize the angle to the range [0..2π)
    #[inline]
    #[must_use]
    pub fn positive(self) -> Self {
        Self::radians(self.radians.rem_euclid(consts::TAU))
    }

    /// Normalize the angle to the range (-π..π]
    #[inline]
    #[must_use]
    pub fn signed(self) -> Self {
        Self::radians(consts::PI - (consts::PI - self.radians).rem_euclid(consts::TAU))
    }

    /// Returns the sine of the angle
    #[inline]
    #[must_use]
    pub fn sin(self) -> f32 {
        self.radians.sin()
    }

    /// Returns the cosine of the angle
    #[inline]
    #[must_use]
    pub fn cos(self) -> f32 {
        self.radians.cos()
    }

    /// Returns the tangent of the angle
    #[inline]
    #[must_use]
    pub fn tan(self) -> f32 {
        self.radians.tan()
    }

    /// Returns the sine and cosine of the angle
    #[inline]
    #[must_use]
    pub fn sin_cos(self) -> (f32, f32) {
        self.radians.sin_cos()
    }

    /// Returns the arcsine of the value as an [`Angle`]
    #[inline]
    #[must_use]
    pub fn asin(value: f32) -> Self {
        Self::radians(value.asin())
    }

    /// Returns the arccosine of the value as an [`Angle`]
    #[inline]
    #[must_use]
    pub fn acos(value: f32) -> Self {
        Self::radians(value.acos())
    }

    /// Returns the arctangent of the value as an [`Angle`]
    #[inline]
    #[must_use]
    pub fn atan(value: f32) -> Self {
        Self::radians(value.atan())
    }

    /// Returns the 2 argument arctangent of the values as an [`Angle`]
    #[inline]
    #[must_use]
    pub fn atan2(y: f32, x: f32) -> Self {
        Self::radians(f32::atan2(y, x))
    }
}

impl ops::Add for Angle {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::radians(self.radians + rhs.radians)
    }
}

impl ops::AddAssign for Angle {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.radians += rhs.radians;
    }
}

impl ops::Sub for Angle {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::radians(self.radians - rhs.radians)
    }
}

impl ops::SubAssign for Angle {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.radians -= rhs.radians;
    }
}

impl ops::Mul<f32> for Angle {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self::radians(self.radians * rhs)
    }
}

impl ops::Mul<Angle> for f32 {
    type Output = Angle;

    #[inline]
    fn mul(self, rhs: Angle) -> Angle {
        Angle::radians(self * rhs.radians)
    }
}

impl ops::MulAssign<f32> for Angle {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.radians *= rhs;
    }
}

impl ops::Div for Angle {
    type Output = f32;

    #[inline]
    fn div(self, rhs: Self) -> f32 {
        self.radians / rhs.radians
    }
}

impl ops::Div<f32> for Angle {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self::radians(self.radians / rhs)
    }
}

impl ops::DivAssign<f32> for Angle {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.radians /= rhs;
    }
}

impl ops::Neg for Angle {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::radians(-self.radians)
    }
}

impl IsClose for Angle {
    type Tolerance = f32;
    const ZERO_TOL: Self::Tolerance = 0.0;
    const ABS_TOL: Self::Tolerance = <Self::Tolerance as IsClose>::ABS_TOL;
    const REL_TOL: Self::Tolerance = <Self::Tolerance as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.radians.is_close_tol(&other.radians, rel_tol, abs_tol)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn angle_radians() {
        let angle = Angle::radians(1.0);
        assert_is_close!(angle.radians, 1.0);
    }

    #[test]
    fn angle_to_radians() {
        let angle = Angle { radians: 1.0 };
        assert_is_close!(angle.to_radians(), 1.0);
    }

    #[test]
    fn angle_degrees() {
        let angle = Angle::degrees(180.0);
        assert_is_close!(angle.radians, consts::PI);
    }

    #[test]
    fn angle_to_degrees() {
        let angle = Angle {
            radians: consts::PI,
        };
        assert_is_close!(angle.to_degrees(), 180.0);
    }

    #[test]
    fn angle_positive() {
        let angle = Angle::radians(0.0);
        assert_is_close!(angle.positive().radians, 0.0);

        // Positive angles
        let angle = Angle::radians(consts::FRAC_PI_2);
        assert_is_close!(angle.positive().radians, consts::FRAC_PI_2);
        let angle = Angle::radians(consts::PI);
        assert_is_close!(angle.positive().radians, consts::PI);
        let angle = Angle::radians(3.0 * consts::FRAC_PI_2);
        assert_is_close!(angle.positive().radians, 3.0 * consts::FRAC_PI_2);
        let angle = Angle::radians(consts::TAU);
        assert_is_close!(angle.positive().radians, 0.0);
        let angle = Angle::radians(consts::TAU + consts::FRAC_PI_2);
        assert_is_close!(angle.positive().radians, consts::FRAC_PI_2);

        // Negative angles
        let angle = Angle::radians(-consts::FRAC_PI_2);
        assert_is_close!(angle.positive().radians, 3.0 * consts::FRAC_PI_2);
        let angle = Angle::radians(-consts::PI);
        assert_is_close!(angle.positive().radians, consts::PI);
        let angle = Angle::radians(-3.0 * consts::FRAC_PI_2);
        assert_is_close!(angle.positive().radians, consts::FRAC_PI_2);
        let angle = Angle::radians(-consts::TAU);
        assert_is_close!(angle.positive().radians, 0.0);
        let angle = Angle::radians(-consts::TAU - consts::FRAC_PI_2);
        assert_is_close!(angle.positive().radians, 3.0 * consts::FRAC_PI_2);
    }

    #[test]
    fn angle_signed() {
        let angle = Angle::radians(0.0);
        assert_is_close!(angle.signed().radians, 0.0);

        // Positive angles
        let angle = Angle::radians(consts::FRAC_PI_2);
        assert_is_close!(angle.signed().radians, consts::FRAC_PI_2);
        let angle = Angle::radians(consts::PI);
        assert_is_close!(angle.signed().radians, consts::PI);
        let angle = Angle::radians(3.0 * consts::FRAC_PI_2);
        assert_is_close!(angle.signed().radians, -consts::FRAC_PI_2);
        let angle = Angle::radians(consts::TAU);
        assert_is_close!(angle.signed().radians, 0.0);
        let angle = Angle::radians(consts::TAU + consts::FRAC_PI_2);
        assert_is_close!(angle.signed().radians, consts::FRAC_PI_2);

        // Negative angles
        let angle = Angle::radians(-consts::FRAC_PI_2);
        assert_is_close!(angle.signed().radians, -consts::FRAC_PI_2);
        let angle = Angle::radians(-consts::PI);
        assert_is_close!(angle.signed().radians, consts::PI);
        let angle = Angle::radians(-3.0 * consts::FRAC_PI_2);
        assert_is_close!(angle.signed().radians, consts::FRAC_PI_2);
        let angle = Angle::radians(-consts::TAU);
        assert_is_close!(angle.signed().radians, 0.0);
        let angle = Angle::radians(-consts::TAU - consts::FRAC_PI_2);
        assert_is_close!(angle.signed().radians, -consts::FRAC_PI_2);
    }

    #[test]
    fn angle_sin() {
        let angle = Angle::radians(0.0);
        assert_is_close!(angle.sin(), 0.0);

        let angle = Angle::radians(consts::FRAC_PI_2);
        assert_is_close!(angle.sin(), 1.0);

        let angle = Angle::radians(consts::PI);
        assert_is_close!(angle.sin(), 0.0);

        let angle = Angle::radians(3.0 * consts::FRAC_PI_2);
        assert_is_close!(angle.sin(), -1.0);

        let angle = Angle::radians(consts::TAU);
        assert_is_close!(angle.sin(), 0.0);
    }

    #[test]
    fn angle_cos() {
        let angle = Angle::radians(0.0);
        assert_is_close!(angle.cos(), 1.0);

        let angle = Angle::radians(consts::FRAC_PI_2);
        assert_is_close!(angle.cos(), 0.0);

        let angle = Angle::radians(consts::PI);
        assert_is_close!(angle.cos(), -1.0);

        let angle = Angle::radians(3.0 * consts::FRAC_PI_2);
        assert_is_close!(angle.cos(), 0.0);

        let angle = Angle::radians(consts::TAU);
        assert_is_close!(angle.cos(), 1.0);
    }

    #[test]
    fn angle_tan() {
        let angle = Angle::radians(0.0);
        assert_is_close!(angle.tan(), 0.0);

        let angle = Angle::radians(consts::FRAC_PI_2);
        assert!(angle.tan().abs() > 1e6); // We don't always get inf due to f32 precision

        let angle = Angle::radians(consts::PI);
        assert_is_close!(angle.tan(), 0.0);

        let angle = Angle::radians(3.0 * consts::FRAC_PI_2);
        assert!(angle.tan().abs() > 1e6); // We don't always get inf due to f32 precision

        let angle = Angle::radians(consts::TAU);
        assert_is_close!(angle.tan(), 0.0);
    }

    #[test]
    fn angle_sin_cos() {
        let angle = Angle::radians(0.0);
        let (sin, cos) = angle.sin_cos();
        assert_is_close!(sin, 0.0);
        assert_is_close!(cos, 1.0);

        let angle = Angle::radians(consts::FRAC_PI_2);
        let (sin, cos) = angle.sin_cos();
        assert_is_close!(sin, 1.0);
        assert_is_close!(cos, 0.0);

        let angle = Angle::radians(consts::PI);
        let (sin, cos) = angle.sin_cos();
        assert_is_close!(sin, 0.0);
        assert_is_close!(cos, -1.0);

        let angle = Angle::radians(3.0 * consts::FRAC_PI_2);
        let (sin, cos) = angle.sin_cos();
        assert_is_close!(sin, -1.0);
        assert_is_close!(cos, 0.0);

        let angle = Angle::radians(consts::TAU);
        let (sin, cos) = angle.sin_cos();
        assert_is_close!(sin, 0.0);
        assert_is_close!(cos, 1.0);
    }

    #[test]
    fn angle_asin() {
        let angle = Angle::asin(0.0);
        assert_is_close!(angle.radians, 0.0);

        let angle = Angle::asin(1.0);
        assert_is_close!(angle.radians, consts::FRAC_PI_2);

        let angle = Angle::asin(-1.0);
        assert_is_close!(angle.radians, -consts::FRAC_PI_2);
    }

    #[test]
    fn angle_acos() {
        let angle = Angle::acos(0.0);
        assert_is_close!(angle.radians, consts::FRAC_PI_2);

        let angle = Angle::acos(1.0);
        assert_is_close!(angle.radians, 0.0);

        let angle = Angle::acos(-1.0);
        assert_is_close!(angle.radians, consts::PI);
    }

    #[test]
    fn angle_atan() {
        let angle = Angle::atan(0.0);
        assert_is_close!(angle.radians, 0.0);

        let angle = Angle::atan(f32::INFINITY);
        assert_is_close!(angle.radians, consts::FRAC_PI_2);

        let angle = Angle::atan(f32::NEG_INFINITY);
        assert_is_close!(angle.radians, -consts::FRAC_PI_2);
    }

    #[test]
    fn angle_atan2() {
        let angle = Angle::atan2(0.0, 1.0);
        assert_is_close!(angle.radians, 0.0);

        let angle = Angle::atan2(1.0, 0.0);
        assert_is_close!(angle.radians, consts::FRAC_PI_2);

        let angle = Angle::atan2(0.0, -1.0);
        assert_is_close!(angle.radians, consts::PI);

        let angle = Angle::atan2(-1.0, 0.0);
        assert_is_close!(angle.radians, -consts::FRAC_PI_2);
    }

    #[test]
    fn angle_add() {
        let angle = Angle::radians(2.0) + Angle::radians(1.0);
        assert_is_close!(angle.radians, 3.0);
    }

    #[test]
    fn angle_add_assign() {
        let mut angle = Angle::radians(2.0);
        angle += Angle::radians(1.0);
        assert_is_close!(angle.radians, 3.0);
    }

    #[test]
    fn angle_sub() {
        let angle = Angle::radians(2.0) - Angle::radians(1.0);
        assert_is_close!(angle.radians, 1.0);
    }

    #[test]
    fn angle_sub_assign() {
        let mut angle = Angle::radians(2.0);
        angle -= Angle::radians(1.0);
        assert_is_close!(angle.radians, 1.0);
    }

    #[test]
    fn angle_mul() {
        let angle = Angle::radians(2.0) * 1.5;
        assert_is_close!(angle.radians, 3.0);

        let angle = 2.0 * Angle::radians(1.5);
        assert_is_close!(angle.radians, 3.0);
    }

    #[test]
    fn angle_mul_assign() {
        let mut angle = Angle::radians(2.0);
        angle *= 1.5;
        assert_is_close!(angle.radians, 3.0);
    }

    #[test]
    fn angle_div() {
        let angle = Angle::radians(3.0) / 1.5;
        assert_is_close!(angle.radians, 2.0);

        let ratio = Angle::radians(3.0) / Angle::radians(2.0);
        assert_is_close!(ratio, 1.5);
    }

    #[test]
    fn angle_div_assign() {
        let mut angle = Angle::radians(3.0);
        angle /= 1.5;
        assert_is_close!(angle.radians, 2.0);
    }

    #[test]
    fn angle_neg() {
        let angle = -Angle::radians(2.0);
        assert_is_close!(angle.radians, -2.0);
    }

    #[test]
    fn angle_is_close() {
        assert_is_close!(Angle::radians(consts::FRAC_PI_2), Angle::degrees(90.0));
        assert!(!Angle::radians(1.5).is_close(&Angle::radians(consts::FRAC_PI_2)));
    }
}
