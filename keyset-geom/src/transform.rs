use std::ops;

use isclose::IsClose;

use crate::{Angle, Unit};

/// A 2-dimensional scale
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Scale {
    /// The scaling factor for the `x` axis
    pub x: f32,
    /// The scaling factor for the `y` axis
    pub y: f32,
}

impl Scale {
    /// Create a new scale with the given scaling factors
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Create a scale with the same value for the `x` and `y` scaling factors
    #[inline]
    #[must_use]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }
}

impl ops::Mul for Scale {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::MulAssign for Scale {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::Div for Scale {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::DivAssign for Scale {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl IsClose<f32> for Scale {
    const ABS_TOL: f32 = <f32 as IsClose>::ABS_TOL;
    const REL_TOL: f32 = <f32 as IsClose>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.x.is_close_impl(&other.x, rel_tol, abs_tol)
            && self.y.is_close_impl(&other.y, rel_tol, abs_tol)
    }
}

/// A 2-dimensional translation
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Translate<U: Unit> {
    /// The translation along the `x` axis
    pub x: U,
    /// The translation along the `y` axis
    pub y: U,
}

impl<U> Translate<U>
where
    U: Unit,
{
    /// Create a new translation with the given distances
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: U::new(x),
            y: U::new(y),
        }
    }

    /// Create a new translation with the same value for the `x` and `y` distances
    #[inline]
    #[must_use]
    pub fn splat(v: f32) -> Self {
        Self {
            x: U::new(v),
            y: U::new(v),
        }
    }

    /// Create a new translation from unit distances
    #[inline]
    #[must_use]
    pub const fn from_units(x: U, y: U) -> Self {
        Self { x, y }
    }
}

impl<U> ops::Add for Translate<U>
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

impl<U> ops::AddAssign for Translate<U>
where
    U: Unit,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<U> ops::Sub for Translate<U>
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

impl<U> ops::SubAssign for Translate<U>
where
    U: Unit,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<U> IsClose<f32> for Translate<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <f32 as IsClose>::ABS_TOL;
    const REL_TOL: f32 = <f32 as IsClose>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.x.is_close_impl(&other.x, rel_tol, abs_tol)
            && self.y.is_close_impl(&other.y, rel_tol, abs_tol)
    }
}

/// A rotation about the origin
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(transparent)]
pub struct Rotate {
    /// The angle of rotation
    pub angle: Angle,
}

impl Rotate {
    /// Create a new rotation with the given angle in radians
    #[inline]
    #[must_use]
    pub const fn new(radians: f32) -> Self {
        Self {
            angle: Angle::new(radians),
        }
    }

    /// Creates a new rotation with the given angle in radians
    #[inline]
    #[must_use]
    pub const fn radians(radians: f32) -> Self {
        Self {
            angle: Angle::radians(radians),
        }
    }

    /// Creates a new rotation with the given angle in degrees
    #[inline]
    #[must_use]
    pub fn degrees(degrees: f32) -> Self {
        Self {
            angle: Angle::degrees(degrees),
        }
    }

    /// Creates a new rotation with the given angle
    #[inline]
    #[must_use]
    pub const fn from_angle(angle: Angle) -> Self {
        Self { angle }
    }
}

impl ops::Add for Rotate {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            angle: self.angle + rhs.angle,
        }
    }
}

impl ops::AddAssign for Rotate {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.angle += rhs.angle;
    }
}

impl ops::Sub for Rotate {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            angle: self.angle - rhs.angle,
        }
    }
}

impl ops::SubAssign for Rotate {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.angle -= rhs.angle;
    }
}

impl IsClose<f32> for Rotate {
    const ABS_TOL: f32 = <Angle as IsClose<f32>>::ABS_TOL;
    const REL_TOL: f32 = <Angle as IsClose<f32>>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.angle.is_close_impl(&other.angle, rel_tol, abs_tol)
    }
}

/// A 2-dimensional affine transformation matrix in the form:
///
/// $$
/// \\begin{bmatrix}
/// a_{xx} & a_{xy} & t_{x} \\\\
/// a_{yx} & a_{yy} & t_{y} \\\\
///   0    &   0    &   1
/// \\end{bmatrix}
/// $$
///
/// Note: only the first 2 rows of the matrix are stored as the last row is
/// a constant \\([0, 0, 1]\\).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Transform<U: Unit> {
    /// Element of the affine transform matrix
    pub a_xx: f32,
    /// Element of the affine transform matrix
    pub a_xy: f32,
    /// Element of the affine transform matrix
    pub t_x: U,
    /// Element of the affine transform matrix
    pub a_yx: f32,
    /// Element of the affine transform matrix
    pub a_yy: f32,
    /// Element of the affine transform matrix
    pub t_y: U,
}

impl<U> Transform<U>
where
    U: Unit,
{
    /// Creates a new affine transform
    #[allow(clippy::similar_names)]
    #[inline]
    #[must_use]
    pub fn new(a_xx: f32, a_xy: f32, t_x: f32, a_yx: f32, a_yy: f32, t_y: f32) -> Self {
        Self {
            a_xx,
            a_xy,
            t_x: U::new(t_x),
            a_yx,
            a_yy,
            t_y: U::new(t_y),
        }
    }

    /// Create a single affine transform from applying the other transform after self
    ///
    /// Equivalent to `other * self`
    #[inline]
    #[must_use]
    #[allow(clippy::suspicious_operation_groupings)]
    pub fn then(self, other: impl Into<Self>) -> Self {
        let other: Self = other.into();
        Self {
            a_xx: self.a_xx * other.a_xx + self.a_yx * other.a_xy, // + 0.0 * other.t_x,
            a_xy: self.a_xy * other.a_xx + self.a_yy * other.a_xy, // + 0.0 * other.t_x,
            t_x: self.t_x * other.a_xx + self.t_y * other.a_xy + /* 1.0 * */ other.t_x,
            a_yx: self.a_xx * other.a_yx + self.a_yx * other.a_yy, // + 0.0 * other.t_y,
            a_yy: self.a_xy * other.a_yx + self.a_yy * other.a_yy, // + 0.0 * other.t_y,
            t_y: self.t_x * other.a_yx + self.t_y * other.a_yy + /* 1.0 * */ other.t_y,
            // 0.0: self.a_xx * 0.0 + self.a_yx * 0.0 + 0.0 * other.t_x,
            // 0.0: self.a_xy * 0.0 + self.a_yy * 0.0 + 0.0 * other.t_y,
            // 1.0: self.t_x * 0.0 + self.t_y * 0.0 + 1.0 * 1.0,
        }
    }
}

impl<U> From<Scale> for Transform<U>
where
    U: Unit,
{
    #[inline]
    fn from(value: Scale) -> Self {
        Self {
            a_xx: value.x,
            a_xy: 0.0,
            t_x: U::new(0.0),
            a_yx: 0.0,
            a_yy: value.y,
            t_y: U::new(0.0),
        }
    }
}

impl<U> From<Translate<U>> for Transform<U>
where
    U: Unit,
{
    #[inline]
    fn from(value: Translate<U>) -> Self {
        Self {
            a_xx: 1.0,
            a_xy: 0.0,
            t_x: value.x,
            a_yx: 0.0,
            a_yy: 1.0,
            t_y: value.y,
        }
    }
}

impl<U> From<Rotate> for Transform<U>
where
    U: Unit,
{
    #[inline]
    fn from(value: Rotate) -> Self {
        let (sin, cos) = value.angle.sin_cos();
        Self {
            a_xx: cos,
            a_xy: -sin,
            t_x: U::new(0.0),
            a_yx: sin,
            a_yy: cos,
            t_y: U::new(0.0),
        }
    }
}

impl<U, T> ops::Mul<T> for Transform<U>
where
    U: Unit,
    T: Into<Self>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        rhs.into().then(self)
    }
}

impl<U> IsClose<f32> for Transform<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = <f32 as IsClose>::ABS_TOL;
    const REL_TOL: f32 = <f32 as IsClose>::REL_TOL;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.a_xx.is_close_impl(&other.a_xx, rel_tol, abs_tol)
            && self.a_xy.is_close_impl(&other.a_xy, rel_tol, abs_tol)
            && self.t_x.is_close_impl(&other.t_x, rel_tol, abs_tol)
            && self.a_yx.is_close_impl(&other.a_yx, rel_tol, abs_tol)
            && self.a_yy.is_close_impl(&other.a_yy, rel_tol, abs_tol)
            && self.t_y.is_close_impl(&other.t_y, rel_tol, abs_tol)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::Mm;

    use super::*;

    #[test]
    fn scale_new() {
        let scale = Scale::new(2.0, 3.0);
        assert_is_close!(scale.x, 2.0);
        assert_is_close!(scale.y, 3.0);
    }

    #[test]
    fn scale_splat() {
        let scale = Scale::splat(4.0);
        assert_is_close!(scale.x, 4.0);
        assert_is_close!(scale.y, 4.0);
    }

    #[test]
    fn scale_mul() {
        let scale = Scale { x: 1.0, y: 3.0 } * Scale { x: 2.0, y: 0.5 };
        assert_is_close!(scale.x, 2.0);
        assert_is_close!(scale.y, 1.5);
    }

    #[test]
    fn scale_mul_assign() {
        let mut scale = Scale { x: 1.0, y: 3.0 };
        scale *= Scale { x: 2.0, y: 0.5 };
        assert_is_close!(scale.x, 2.0);
        assert_is_close!(scale.y, 1.5);
    }

    #[test]
    fn scale_div() {
        let scale = Scale { x: 1.0, y: 3.0 } / Scale { x: 2.0, y: 0.5 };
        assert_is_close!(scale.x, 0.5);
        assert_is_close!(scale.y, 6.0);
    }

    #[test]
    fn scale_div_assign() {
        let mut scale = Scale { x: 1.0, y: 3.0 };
        scale /= Scale { x: 2.0, y: 0.5 };
        assert_is_close!(scale.x, 0.5);
        assert_is_close!(scale.y, 6.0);
    }

    #[test]
    fn scale_is_close() {
        assert!(Scale { x: 1.0, y: 3.0 }.is_close(Scale {
            x: 2.0 * 0.5,
            y: 1.5 / 0.5
        }));
        assert!(!Scale { x: 1.0, y: 3.0 }.is_close(Scale {
            x: 2.1 * 0.5,
            y: 1.5 / 0.5
        }));
        assert!(!Scale { x: 1.0, y: 3.0 }.is_close(Scale {
            x: 2.0 * 0.5,
            y: 1.6 / 0.5
        }));
    }

    #[test]
    fn translate_new() {
        let translate = Translate::<Mm>::new(2.0, 3.0);
        assert_is_close!(translate.x, Mm(2.0));
        assert_is_close!(translate.y, Mm(3.0));
    }

    #[test]
    fn translate_splat() {
        let translate = Translate::<Mm>::splat(4.0);
        assert_is_close!(translate.x, Mm(4.0));
        assert_is_close!(translate.y, Mm(4.0));
    }

    #[test]
    fn translate_from_units() {
        let translate = Translate::from_units(Mm(2.0), Mm(3.0));
        assert_is_close!(translate.x, Mm(2.0));
        assert_is_close!(translate.y, Mm(3.0));
    }

    #[test]
    fn translate_add() {
        let translate = Translate {
            x: Mm(1.0),
            y: Mm(3.0),
        } + Translate {
            x: Mm(2.0),
            y: Mm(0.5),
        };
        assert_is_close!(translate.x, Mm(3.0));
        assert_is_close!(translate.y, Mm(3.5));
    }

    #[test]
    fn translate_add_assign() {
        let mut translate = Translate {
            x: Mm(1.0),
            y: Mm(3.0),
        };
        translate += Translate {
            x: Mm(2.0),
            y: Mm(0.5),
        };
        assert_is_close!(translate.x, Mm(3.0));
        assert_is_close!(translate.y, Mm(3.5));
    }

    #[test]
    fn translate_sub() {
        let translate = Translate {
            x: Mm(1.0),
            y: Mm(3.0),
        } - Translate {
            x: Mm(2.0),
            y: Mm(0.5),
        };
        assert_is_close!(translate.x, Mm(-1.0));
        assert_is_close!(translate.y, Mm(2.5));
    }

    #[test]
    fn translate_sub_assign() {
        let mut translate = Translate {
            x: Mm(1.0),
            y: Mm(3.0),
        };
        translate -= Translate {
            x: Mm(2.0),
            y: Mm(0.5),
        };
        assert_is_close!(translate.x, Mm(-1.0));
        assert_is_close!(translate.y, Mm(2.5));
    }

    #[test]
    fn translate_is_close() {
        assert!(Translate {
            x: Mm(1.0),
            y: Mm(3.0)
        }
        .is_close(Translate {
            x: Mm(2.0) * 0.5,
            y: Mm(1.5) / 0.5
        }));
        assert!(!Translate {
            x: Mm(1.0),
            y: Mm(3.0)
        }
        .is_close(Translate {
            x: Mm(2.1) * 0.5,
            y: Mm(1.5) / 0.5
        }));
        assert!(!Translate {
            x: Mm(1.0),
            y: Mm(3.0)
        }
        .is_close(Translate {
            x: Mm(2.0) * 0.5,
            y: Mm(1.6) / 0.5
        }));
    }

    #[test]
    fn rotate_new() {
        let rotate = Rotate::new(1.0);
        assert_is_close!(rotate.angle.to_radians(), 1.0);
    }

    #[test]
    fn rotate_radians() {
        let rotate = Rotate::radians(1.0);
        assert_is_close!(rotate.angle.to_radians(), 1.0);
    }

    #[test]
    fn rotate_degrees() {
        let rotate = Rotate::degrees(180.0);
        assert_is_close!(rotate.angle.to_radians(), std::f32::consts::PI);
    }

    #[test]
    fn rotate_from_angle() {
        let rotate = Rotate::from_angle(Angle::FRAC_PI_4);
        assert_is_close!(rotate.angle.to_radians(), std::f32::consts::FRAC_PI_4);
    }

    #[test]
    fn rotate_add() {
        let rotate = Rotate::radians(2.0) + Rotate::radians(1.0);
        assert_is_close!(rotate.angle.to_radians(), 3.0);
    }

    #[test]
    fn rotate_add_assign() {
        let mut rotate = Rotate::radians(2.0);
        rotate += Rotate::radians(1.0);
        assert_is_close!(rotate.angle.to_radians(), 3.0);
    }

    #[test]
    fn rotate_sub() {
        let rotate = Rotate::radians(2.0) - Rotate::radians(1.0);
        assert_is_close!(rotate.angle.to_radians(), 1.0);
    }

    #[test]
    fn rotate_sub_assign() {
        let mut rotate = Rotate::radians(2.0);
        rotate -= Rotate::radians(1.0);
        assert_is_close!(rotate.angle.to_radians(), 1.0);
    }

    #[test]
    fn rotate_is_close() {
        assert_is_close!(
            Rotate::radians(std::f32::consts::FRAC_PI_2),
            Rotate::degrees(90.0)
        );
        assert!(!Rotate::radians(1.5).is_close(Rotate::radians(std::f32::consts::FRAC_PI_2)));
    }

    #[test]
    fn transform_new() {
        let transform = Transform::<Mm>::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_is_close!(transform.a_xx, 1.0);
        assert_is_close!(transform.a_xy, 2.0);
        assert_is_close!(transform.t_x, Mm(3.0));
        assert_is_close!(transform.a_yx, 4.0);
        assert_is_close!(transform.a_yy, 5.0);
        assert_is_close!(transform.t_y, Mm(6.0));
    }

    #[test]
    fn transform_then() {
        let transform1 = Transform::<Mm> {
            a_xx: 1.0,
            a_xy: 2.0,
            t_x: Mm(3.0),
            a_yx: 4.0,
            a_yy: 5.0,
            t_y: Mm(6.0),
        };
        let transform2 = Transform {
            a_xx: 1.0,
            a_xy: 0.5,
            t_x: Mm(-1.0),
            a_yx: -0.5,
            a_yy: 1.5,
            t_y: Mm(2.0),
        };
        let transform = transform1.then(transform2);

        assert_is_close!(transform.a_xx, 3.0);
        assert_is_close!(transform.a_xy, 4.5);
        assert_is_close!(transform.t_x, Mm(5.0));
        assert_is_close!(transform.a_yx, 5.5);
        assert_is_close!(transform.a_yy, 6.5);
        assert_is_close!(transform.t_y, Mm(9.5));
    }

    #[test]
    fn transform_from_scale() {
        let scale = Scale::new(2.0, 0.5);
        let transform = Transform::<Mm>::from(scale);

        assert_is_close!(transform.a_xx, 2.0);
        assert_is_close!(transform.a_xy, 0.0);
        assert_is_close!(transform.t_x, Mm(0.0));
        assert_is_close!(transform.a_yx, 0.0);
        assert_is_close!(transform.a_yy, 0.5);
        assert_is_close!(transform.t_y, Mm(0.0));
    }

    #[test]
    fn transform_from_translate() {
        let translate = Translate::new(2.0, -1.0);
        let transform = Transform::<Mm>::from(translate);

        assert_is_close!(transform.a_xx, 1.0);
        assert_is_close!(transform.a_xy, 0.0);
        assert_is_close!(transform.t_x, Mm(2.0));
        assert_is_close!(transform.a_yx, 0.0);
        assert_is_close!(transform.a_yy, 1.0);
        assert_is_close!(transform.t_y, Mm(-1.0));
    }

    #[test]
    fn transform_from_rotate() {
        let rotate = Rotate::degrees(135.0);
        let transform = Transform::<Mm>::from(rotate);

        let sq12 = std::f32::consts::FRAC_1_SQRT_2;
        assert_is_close!(transform.a_xx, -sq12);
        assert_is_close!(transform.a_xy, -sq12);
        assert_is_close!(transform.t_x, Mm(0.0));
        assert_is_close!(transform.a_yx, sq12);
        assert_is_close!(transform.a_yy, -sq12);
        assert_is_close!(transform.t_y, Mm(0.0));
    }

    #[test]
    fn transform_mul() {
        let transform1 = Transform::<Mm> {
            a_xx: 1.0,
            a_xy: 2.0,
            t_x: Mm(3.0),
            a_yx: 4.0,
            a_yy: 5.0,
            t_y: Mm(6.0),
        };
        let transform2 = Transform {
            a_xx: 1.0,
            a_xy: 0.5,
            t_x: Mm(-1.0),
            a_yx: -0.5,
            a_yy: 1.5,
            t_y: Mm(2.0),
        };
        let transform = transform1 * transform2;

        assert_is_close!(transform.a_xx, 0.0);
        assert_is_close!(transform.a_xy, 3.5);
        assert_is_close!(transform.t_x, Mm(6.0));
        assert_is_close!(transform.a_yx, 1.5);
        assert_is_close!(transform.a_yy, 9.5);
        assert_is_close!(transform.t_y, Mm(12.0));
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn transform_is_close() {
        assert!(Transform {
            a_xx: 1.0,
            a_xy: 2.0,
            t_x: Mm(3.0),
            a_yx: 4.0,
            a_yy: 5.0,
            t_y: Mm(6.0),
        }
        .is_close(Transform {
            a_xx: 2.0 * 0.5,
            a_xy: 1.0 * 2.0,
            t_x: Mm(2.0) * 1.5,
            a_yx: 8.0 * 0.5,
            a_yy: 2.5 * 2.0,
            t_y: Mm(4.0) * 1.5,
        }));
        assert!(!Transform {
            a_xx: 1.0,
            a_xy: 2.0,
            t_x: Mm(3.0),
            a_yx: 4.0,
            a_yy: 5.0,
            t_y: Mm(6.0),
        }
        .is_close(Transform {
            a_xx: 2.1 * 0.5,
            a_xy: 1.0 * 2.0,
            t_x: Mm(2.0) * 1.5,
            a_yx: 8.0 * 0.5,
            a_yy: 2.5 * 2.0,
            t_y: Mm(4.0) * 1.5,
        }));
        assert!(!Transform {
            a_xx: 1.0,
            a_xy: 2.0,
            t_x: Mm(3.0),
            a_yx: 4.0,
            a_yy: 5.0,
            t_y: Mm(6.0),
        }
        .is_close(Transform {
            a_xx: 2.0 * 0.5,
            a_xy: 1.2 * 2.0,
            t_x: Mm(2.0) * 1.5,
            a_yx: 8.0 * 0.5,
            a_yy: 2.5 * 2.0,
            t_y: Mm(4.0) * 1.5,
        }));
        assert!(!Transform {
            a_xx: 1.0,
            a_xy: 2.0,
            t_x: Mm(3.0),
            a_yx: 4.0,
            a_yy: 5.0,
            t_y: Mm(6.0),
        }
        .is_close(Transform {
            a_xx: 2.0 * 0.5,
            a_xy: 1.0 * 2.0,
            t_x: Mm(2.1) * 1.5,
            a_yx: 8.0 * 0.5,
            a_yy: 2.5 * 2.0,
            t_y: Mm(4.0) * 1.5,
        }));
        assert!(!Transform {
            a_xx: 1.0,
            a_xy: 2.0,
            t_x: Mm(3.0),
            a_yx: 4.0,
            a_yy: 5.0,
            t_y: Mm(6.0),
        }
        .is_close(Transform {
            a_xx: 2.0 * 0.5,
            a_xy: 1.0 * 2.0,
            t_x: Mm(2.0) * 1.5,
            a_yx: 8.1 * 0.5,
            a_yy: 2.5 * 2.0,
            t_y: Mm(4.0) * 1.5,
        }));
        assert!(!Transform {
            a_xx: 1.0,
            a_xy: 2.0,
            t_x: Mm(3.0),
            a_yx: 4.0,
            a_yy: 5.0,
            t_y: Mm(6.0),
        }
        .is_close(Transform {
            a_xx: 2.0 * 0.5,
            a_xy: 1.0 * 2.0,
            t_x: Mm(2.0) * 1.5,
            a_yx: 8.0 * 0.5,
            a_yy: 2.6 * 2.0,
            t_y: Mm(4.0) * 1.5,
        }));
        assert!(!Transform {
            a_xx: 1.0,
            a_xy: 2.0,
            t_x: Mm(3.0),
            a_yx: 4.0,
            a_yy: 5.0,
            t_y: Mm(6.0),
        }
        .is_close(Transform {
            a_xx: 2.0 * 0.5,
            a_xy: 1.0 * 2.0,
            t_x: Mm(2.0) * 1.5,
            a_yx: 8.0 * 0.5,
            a_yy: 2.5 * 2.0,
            t_y: Mm(4.1) * 1.5,
        }));
    }
}
