use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops;

use isclose::IsClose;

use crate::{Angle, Vector};

/// Trait for Unit types
pub trait Unit:
    Sized
    + Copy
    + Clone
    + Debug
    + PartialEq
    + PartialOrd
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
    + IsClose<Self, Tolerance = f32>
// TODO: seems to trigger rust-lang/rust#96634
// where
//     f32: ops::Mul<Self, Output = Self>,
{
    /// Create a new instance of the unit from a value
    fn new(value: f32) -> Self;

    /// Create a new instance of the unit from a value
    fn get(self) -> f32;

    /// Return a zero value
    #[inline]
    #[must_use]
    fn zero() -> Self {
        Self::new(0.0)
    }

    /// Return the minimum of two units
    #[inline]
    #[must_use]
    fn min(self, rhs: Self) -> Self {
        Self::new(f32::min(self.get(), rhs.get()))
    }

    /// Return the maximum of two units
    #[inline]
    #[must_use]
    fn max(self, rhs: Self) -> Self {
        Self::new(f32::max(self.get(), rhs.get()))
    }

    /// Return the absolute value of the unit
    #[inline]
    #[must_use]
    fn abs(self) -> Self {
        Self::new(self.get().abs())
    }
}

/// Convenience trait for converting units
///
/// Used since just [`From`] will cause conflicts with generic impls in core
pub trait ConvertFrom<U> {
    /// Perform the conversion
    fn convert_from(value: U) -> Self;
}

/// Convenience trait for converting units
///
/// This should not be implemented directly, implement [`ConvertFrom`] instead
pub trait ConvertInto<U> {
    /// Perform the conversion
    fn convert_into(self) -> U;
}

impl<U, V> ConvertInto<V> for U
where
    V: ConvertFrom<U>,
{
    #[inline]
    fn convert_into(self) -> V {
        V::convert_from(self)
    }
}

/// Declares a unit system used for all types in the geom crate.
///
/// Each unit is given a name and an equivalent measure which is used to
/// implement conversion functions. Each unit can also optionally be given a
/// set of attributes (for doc comments, for example) and a visibility.
///
/// # Example
///
/// ```
/// # use keyset_geom::declare_units;
///
/// declare_units! {
///     #[doc  = "Keyboard Unit, usually 19.05 mm or 0.75 in"]
///     pub KeyUnit = 1.0;
///
///     #[doc  = "Dot, a.k.a. drawing unit"]
///     pub Dot = 1000.0;
///
///     #[doc = "Millimeter"]
///     pub Mm = 19.05;
///
///     #[doc = "Inch"]
///     pub Inch = 0.75;
/// }
/// ```
#[macro_export]
macro_rules! declare_units {
    ($($(#[$attr:meta])* $vis:vis $name:ident = $conv:literal);+ $(;)?) => {
        macro_rules! per_consts {
            ($self_name:ident, $self_conv:literal) => {
                impl $self_name {
                    $(
                        $crate::__paste! {
                            #[doc = "Conversion ratio between [`" $name "`] and [`" $self_name "`]"]
                            pub const [<PER_ $name:snake:upper>]: f32 = $self_conv / $conv;
                        }
                    )+
                }
            }
        }

        macro_rules! convert_from_impls {
            ($self_name:ident) => {
                $(
                    impl $crate::ConvertFrom<$name> for $self_name {
                        #[inline]
                        fn convert_from(value: $name) -> Self {
                            $crate::__paste! {
                                Self(value.0 * Self::[<PER_ $name:snake:upper>])
                            }
                        }
                    }
                )+
            }
        }

        macro_rules! is_close_impls {
            ($self_name:ident) => {
                $(
                    impl $crate::__IsClose<$name> for $self_name {
                        type Tolerance = f32;
                        const ZERO_TOL: Self::Tolerance = 0.0;
                        const ABS_TOL: Self::Tolerance = <Self::Tolerance as $crate::__IsClose>::ABS_TOL;
                        const REL_TOL: Self::Tolerance = <Self::Tolerance as $crate::__IsClose>::REL_TOL;

                        #[inline]
                        fn is_close_tol(&self, other: &$name, rel_tol: &f32, abs_tol: &f32) -> bool {
                            self.0.is_close_tol(&<Self as $crate::ConvertFrom<_>>::convert_from(*other).0, abs_tol, rel_tol)
                        }
                    }
                )+
            }
        }

        $(
            $(#[$attr])*
            #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
            #[repr(transparent)]
            $vis struct $name (pub f32);

            per_consts!($name, $conv);

            convert_from_impls!($name);

            impl ::std::ops::Add for $name {
                type Output = Self;

                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    Self(self.0 + rhs.0)
                }
            }

            impl ::std::ops::AddAssign for $name {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    self.0 += rhs.0;
                }
            }

            impl ::std::ops::Sub for $name {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: Self) -> Self {
                    Self(self.0 - rhs.0)
                }
            }

            impl ::std::ops::SubAssign for $name {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 -= rhs.0;
                }
            }

            impl ::std::ops::Mul<f32> for $name {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: f32) -> Self {
                    Self(self.0 * rhs)
                }
            }

            impl ::std::ops::Mul<$name> for f32 {
                type Output = $name;

                #[inline]
                fn mul(self, rhs: $name) -> $name {
                    $name(self * rhs.0)
                }
            }

            impl ::std::ops::MulAssign<f32> for $name {
                #[inline]
                fn mul_assign(&mut self, rhs: f32) {
                    self.0 *= rhs;
                }
            }

            impl ::std::ops::Div for $name {
                type Output = f32;

                #[inline]
                fn div(self, rhs: Self) -> f32 {
                    self.0 / rhs.0
                }
            }

            impl ::std::ops::Div<f32> for $name {
                type Output = Self;

                #[inline]
                fn div(self, rhs: f32) -> Self {
                    Self(self.0 / rhs)
                }
            }

            impl ::std::ops::DivAssign<f32> for $name {
                #[inline]
                fn div_assign(&mut self, rhs: f32) {
                    self.0 /= rhs;
                }
            }

            impl ::std::ops::Neg for $name {
                type Output = Self;

                #[inline]
                fn neg(self) -> Self {
                    Self(-self.0)
                }
            }

            is_close_impls!($name);

            impl $crate::Unit for $name {
                #[inline]
                fn new(value: f32) -> Self {
                    Self(value)
                }

                #[inline]
                fn get(self) -> f32 {
                    self.0
                }
            }
        )+
    };
}

declare_units! {
    #[doc  = "Keyboard Unit, usually 19.05 mm or 0.75 in"]
    pub KeyUnit = 1.0;

    #[doc  = "Dot, a.k.a. drawing unit"]
    pub Dot = 1000.0;

    #[doc = "Millimeter"]
    pub Mm = 19.05;

    #[doc = "Inch"]
    pub Inch = 0.75;
}

/// A conversion between two unit systems.
///
/// This allows conversion between any coordinate systems that can be converted
/// using an affine transform. It cannot be used to convert between cartesian
/// and polar coordinates, for example.
///
/// The affine transform is in the form:
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Conversion<Dst: Unit, Src: Unit> {
    /// Element of the affine transform matrix
    pub a_xx: f32,
    /// Element of the affine transform matrix
    pub a_xy: f32,
    /// Element of the affine transform matrix
    pub t_x: f32,
    /// Element of the affine transform matrix
    pub a_yx: f32,
    /// Element of the affine transform matrix
    pub a_yy: f32,
    /// Element of the affine transform matrix
    pub t_y: f32,
    #[doc(hidden)]
    _phantom: PhantomData<(Dst, Src)>,
}

impl<Dst, Src> Conversion<Dst, Src>
where
    Dst: Unit,
    Src: Unit,
{
    /// Creates a new conversion with the given affine transform
    #[expect(clippy::similar_names, reason = "standard mathematical naming")]
    #[inline]
    #[must_use]
    pub const fn new(a_xx: f32, a_xy: f32, t_x: f32, a_yx: f32, a_yy: f32, t_y: f32) -> Self {
        Self {
            a_xx,
            a_xy,
            t_x,
            a_yx,
            a_yy,
            t_y,
            _phantom: PhantomData,
        }
    }

    /// Create a new conversion with the given scaling factors
    #[inline]
    #[must_use]
    pub const fn from_scale(x: f32, y: f32) -> Self {
        Self {
            a_xx: x,
            a_xy: 0.0,
            t_x: 0.0,
            a_yx: 0.0,
            a_yy: y,
            t_y: 0.0,
            _phantom: PhantomData,
        }
    }

    /// Create a new conversion with the given translations
    #[inline]
    #[must_use]
    pub const fn from_translate(x: f32, y: f32) -> Self {
        Self {
            a_xx: 1.0,
            a_xy: 0.0,
            t_x: x,
            a_yx: 0.0,
            a_yy: 1.0,
            t_y: y,
            _phantom: PhantomData,
        }
    }

    /// Create a new conversion with the given scaling factors
    #[inline]
    #[must_use]
    pub fn from_rotate(angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            a_xx: cos,
            a_xy: -sin,
            t_x: 0.0,
            a_yx: sin,
            a_yy: cos,
            t_y: 0.0,
            _phantom: PhantomData,
        }
    }

    /// Apply the given scaling factors after the current conversion
    #[inline]
    #[must_use]
    pub fn then_scale(self, x: f32, y: f32) -> Self {
        Self {
            a_xx: self.a_xx * x,
            a_xy: self.a_xy * x,
            t_x: self.t_x * x,
            a_yx: self.a_yx * y,
            a_yy: self.a_yy * y,
            t_y: self.t_y * y,
            _phantom: PhantomData,
        }
    }

    /// Apply the given translations after the current conversion
    #[inline]
    #[must_use]
    pub fn then_translate(self, translate: Vector<Dst>) -> Self {
        Self {
            a_xx: self.a_xx,
            a_xy: self.a_xy,
            t_x: self.t_x + translate.x.get(),
            a_yx: self.a_yx,
            a_yy: self.a_yy,
            t_y: self.t_y + translate.y.get(),
            _phantom: PhantomData,
        }
    }

    /// Apply the given rotation after the current conversion
    #[inline]
    #[must_use]
    pub fn then_rotate(self, angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            a_xx: self.a_xx * cos + self.a_yx * -sin,
            a_xy: self.a_xy * cos + self.a_yy * -sin,
            t_x: self.t_x * cos + self.t_y * -sin,
            a_yx: self.a_xx * sin + self.a_yx * cos,
            a_yy: self.a_xy * sin + self.a_yy * cos,
            t_y: self.t_x * sin + self.t_y * cos,
            _phantom: PhantomData,
        }
    }

    /// Apply the given scaling factors before the current conversion
    #[inline]
    #[must_use]
    pub fn pre_scale(self, x: f32, y: f32) -> Self {
        Self {
            a_xx: self.a_xx * x,
            a_xy: self.a_xy * x,
            t_x: self.t_x,
            a_yx: self.a_yx * y,
            a_yy: self.a_yy * y,
            t_y: self.t_y,
            _phantom: PhantomData,
        }
    }

    /// Apply the given translations before the current conversion
    #[inline]
    #[must_use]
    pub fn pre_translate(self, translate: Vector<Src>) -> Self {
        Self {
            a_xx: self.a_xx,
            a_xy: self.a_xy,
            t_x: self.a_xx * translate.x.get() + self.a_xy * translate.y.get() + self.t_x,
            a_yx: self.a_yx,
            a_yy: self.a_yy,
            t_y: self.a_yx * translate.x.get() + self.a_yy * translate.y.get() + self.t_y,
            _phantom: PhantomData,
        }
    }

    /// Apply the given rotation before the current conversion
    #[inline]
    #[must_use]
    pub fn pre_rotate(self, angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            a_xx: self.a_xx * cos + self.a_xy * sin,
            a_xy: self.a_xx * -sin + self.a_xy * cos,
            t_x: self.t_x,
            a_yx: self.a_yx * cos + self.a_yy * sin,
            a_yy: self.a_yx * -sin + self.a_yy * cos,
            t_y: self.t_y,
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn key_unit() {
        let key_unit = KeyUnit::convert_from(Dot(500.0));
        assert_is_close!(key_unit.0, 0.5);

        let key_unit = KeyUnit::convert_from(Mm(38.1));
        assert_is_close!(key_unit.0, 2.0);

        let key_unit = KeyUnit::convert_from(Inch(1.0));
        assert_is_close!(key_unit.0, 4.0 / 3.0);

        let key_unit = KeyUnit::new(3.0);
        assert_is_close!(key_unit.0, 3.0);

        let key_unit = KeyUnit(2.5);
        assert_is_close!(key_unit.get(), 2.5);

        let key_unit = KeyUnit::zero();
        assert_is_close!(key_unit.0, 0.0);
    }

    #[test]
    fn key_unit_cmp() {
        let key_unit1 = KeyUnit(4.0 / 3.0);
        let key_unit2 = KeyUnit(1.5);

        assert_is_close!(key_unit1.max(key_unit2).0, 1.5);
        assert_is_close!(key_unit1.min(key_unit2).0, 4.0 / 3.0);
    }

    #[test]
    fn key_unit_abs() {
        let key_unit = KeyUnit(2.5);
        assert_is_close!(key_unit.abs().0, 2.5);

        let key_unit = KeyUnit(-2.5);
        assert_is_close!(key_unit.abs().0, 2.5);
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
        assert!(KeyUnit(2.5).is_close(&KeyUnit(5.0 / 2.0)));
        assert!(!KeyUnit(2.5).is_close(&KeyUnit(5.1 / 2.0)));
        assert!(KeyUnit(2.5).is_close(&Dot(Dot::PER_KEY_UNIT * 5.0 / 2.0)));
        assert!(!KeyUnit(2.5).is_close(&Dot(Dot::PER_KEY_UNIT * 5.1 / 2.0)));
        assert!(KeyUnit(2.5).is_close(&Mm(Mm::PER_KEY_UNIT * 5.0 / 2.0)));
        assert!(!KeyUnit(2.5).is_close(&Mm(Mm::PER_KEY_UNIT * 5.1 / 2.0)));
        assert!(KeyUnit(2.5).is_close(&Inch(Inch::PER_KEY_UNIT * 5.0 / 2.0)));
        assert!(!KeyUnit(2.5).is_close(&Inch(Inch::PER_KEY_UNIT * 5.1 / 2.0)));
    }

    #[test]
    fn dot() {
        let dot = Dot::convert_from(KeyUnit(0.5));
        assert_is_close!(dot.0, 500.0);

        let dot = Dot::convert_from(Mm(38.1));
        assert_is_close!(dot.0, 2000.0);

        let dot = Dot::convert_from(Inch(1.0));
        assert_is_close!(dot.0, 4000.0 / 3.0);

        let dot = Dot::new(3.0);
        assert_is_close!(dot.0, 3.0);

        let dot = Dot(2.5);
        assert_is_close!(dot.get(), 2.5);

        let dot = Dot::zero();
        assert_is_close!(dot.0, 0.0);
    }

    #[test]
    fn dot_cmp() {
        let dot1 = Dot(4.0 / 3.0);
        let dot2 = Dot(1.5);

        assert_is_close!(dot1.max(dot2).0, 1.5);
        assert_is_close!(dot1.min(dot2).0, 4.0 / 3.0);
    }

    #[test]
    fn dot_abs() {
        let key_unit = Dot(2.5);
        assert_is_close!(key_unit.abs().0, 2.5);

        let key_unit = Dot(-2.5);
        assert_is_close!(key_unit.abs().0, 2.5);
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
        assert!(Dot(2.5).is_close(&Dot(5.0 / 2.0)));
        assert!(!Dot(2.5).is_close(&Dot(5.1 / 2.0)));
        assert!(Dot(2.5).is_close(&KeyUnit(KeyUnit::PER_DOT * 5.0 / 2.0)));
        assert!(!Dot(2.5).is_close(&KeyUnit(KeyUnit::PER_DOT * 5.1 / 2.0)));
        assert!(Dot(2.5).is_close(&Mm(Mm::PER_DOT * 5.0 / 2.0)));
        assert!(!Dot(2.5).is_close(&Mm(Mm::PER_DOT * 5.1 / 2.0)));
        assert!(Dot(2.5).is_close(&Inch(Inch::PER_DOT * 5.0 / 2.0)));
        assert!(!Dot(2.5).is_close(&Inch(Inch::PER_DOT * 5.1 / 2.0)));
    }

    #[test]
    fn mm() {
        let mm = Mm::convert_from(KeyUnit(0.5));
        assert_is_close!(mm.0, 9.525);

        let mm = Mm::convert_from(Dot(2000.0));
        assert_is_close!(mm.0, 38.1);

        let mm = Mm::convert_from(Inch(1.0));
        assert_is_close!(mm.0, 25.4);

        let mm = Mm::new(3.0);
        assert_is_close!(mm.0, 3.0);

        let mm = Mm(2.5);
        assert_is_close!(mm.get(), 2.5);

        let mm = Mm::zero();
        assert_is_close!(mm.0, 0.0);
    }

    #[test]
    fn mm_cmp() {
        let mm1 = Mm(4.0 / 3.0);
        let mm2 = Mm(1.5);

        assert_is_close!(mm1.max(mm2).0, 1.5);
        assert_is_close!(mm1.min(mm2).0, 4.0 / 3.0);
    }

    #[test]
    fn mm_abs() {
        let mm = Mm(2.5);
        assert_is_close!(mm.abs().0, 2.5);

        let mm = Mm(-2.5);
        assert_is_close!(mm.abs().0, 2.5);
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
        assert!(Mm(2.5).is_close(&Mm(5.0 / 2.0)));
        assert!(!Mm(2.5).is_close(&Mm(5.1 / 2.0)));
        assert!(Mm(2.5).is_close(&KeyUnit(KeyUnit::PER_MM * 5.0 / 2.0)));
        assert!(!Mm(2.5).is_close(&KeyUnit(KeyUnit::PER_MM * 5.1 / 2.0)));
        assert!(Mm(2.5).is_close(&Dot(Dot::PER_MM * 5.0 / 2.0)));
        assert!(!Mm(2.5).is_close(&Dot(Dot::PER_MM * 5.1 / 2.0)));
        assert!(Mm(2.5).is_close(&Inch(Inch::PER_MM * 5.0 / 2.0)));
        assert!(!Mm(2.5).is_close(&Inch(Inch::PER_MM * 5.1 / 2.0)));
    }

    #[test]
    fn inch() {
        let inch = Inch::convert_from(KeyUnit(4.0 / 3.0));
        assert_is_close!(inch.0, 1.0);

        let inch = Inch::convert_from(Dot(2000.0));
        assert_is_close!(inch.0, 1.5);

        let inch = Inch::convert_from(Mm(19.05));
        assert_is_close!(inch.0, 0.75);

        let inch = Inch::new(3.0);
        assert_is_close!(inch.0, 3.0);

        let inch = Inch(2.5);
        assert_is_close!(inch.get(), 2.5);

        let inch = Inch::zero();
        assert_is_close!(inch.0, 0.0);
    }

    #[test]
    fn inch_cmp() {
        let inch1 = Inch(4.0 / 3.0);
        let inch2 = Inch(1.5);

        assert_is_close!(inch1.max(inch2).0, 1.5);
        assert_is_close!(inch1.min(inch2).0, 4.0 / 3.0);
    }

    #[test]
    fn inch_abs() {
        let inch = Inch(2.5);
        assert_is_close!(inch.abs().0, 2.5);

        let inch = Inch(-2.5);
        assert_is_close!(inch.abs().0, 2.5);
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
        assert!(Inch(2.5).is_close(&Inch(5.0 / 2.0)));
        assert!(!Inch(2.5).is_close(&Inch(5.1 / 2.0)));
        assert!(Inch(2.5).is_close(&KeyUnit(KeyUnit::PER_INCH * 5.0 / 2.0)));
        assert!(!Inch(2.5).is_close(&KeyUnit(KeyUnit::PER_INCH * 5.1 / 2.0)));
        assert!(Inch(2.5).is_close(&Dot(Dot::PER_INCH * 5.0 / 2.0)));
        assert!(!Inch(2.5).is_close(&Dot(Dot::PER_INCH * 5.1 / 2.0)));
        assert!(Inch(2.5).is_close(&Mm(Mm::PER_INCH * 5.0 / 2.0)));
        assert!(!Inch(2.5).is_close(&Mm(Mm::PER_INCH * 5.1 / 2.0)));
    }

    #[test]
    fn conversion_new() {
        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_is_close!(conv.a_xx, 1.0);
        assert_is_close!(conv.a_xy, 2.0);
        assert_is_close!(conv.t_x, 3.0);
        assert_is_close!(conv.a_yx, 4.0);
        assert_is_close!(conv.a_yy, 5.0);
        assert_is_close!(conv.t_y, 6.0);
    }

    #[test]
    fn conversion_from_scale() {
        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::from_scale(0.5, 2.0);
        assert_is_close!(conv.a_xx, 0.5);
        assert_is_close!(conv.a_xy, 0.0);
        assert_is_close!(conv.t_x, 0.0);
        assert_is_close!(conv.a_yx, 0.0);
        assert_is_close!(conv.a_yy, 2.0);
        assert_is_close!(conv.t_y, 0.0);
    }

    #[test]
    fn conversion_from_translate() {
        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::from_translate(0.5, 2.0);
        assert_is_close!(conv.a_xx, 1.0);
        assert_is_close!(conv.a_xy, 0.0);
        assert_is_close!(conv.t_x, 0.5);
        assert_is_close!(conv.a_yx, 0.0);
        assert_is_close!(conv.a_yy, 1.0);
        assert_is_close!(conv.t_y, 2.0);
    }

    #[test]
    fn conversion_from_rotate() {
        use std::f32::consts::FRAC_1_SQRT_2;

        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::from_rotate(Angle::degrees(45.0));
        assert_is_close!(conv.a_xx, FRAC_1_SQRT_2);
        assert_is_close!(conv.a_xy, -FRAC_1_SQRT_2);
        assert_is_close!(conv.t_x, 0.0);
        assert_is_close!(conv.a_yx, FRAC_1_SQRT_2);
        assert_is_close!(conv.a_yy, FRAC_1_SQRT_2);
        assert_is_close!(conv.t_y, 0.0);
    }

    #[test]
    fn conversion_then_scale() {
        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).then_scale(0.5, 2.0);
        assert_is_close!(conv.a_xx, 0.5);
        assert_is_close!(conv.a_xy, 1.0);
        assert_is_close!(conv.t_x, 1.5);
        assert_is_close!(conv.a_yx, 8.0);
        assert_is_close!(conv.a_yy, 10.0);
        assert_is_close!(conv.t_y, 12.0);
    }

    #[test]
    fn conversion_then_translate() {
        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
            .then_translate(Vector::new(Mm(0.5), Mm(2.0)));
        assert_is_close!(conv.a_xx, 1.0);
        assert_is_close!(conv.a_xy, 2.0);
        assert_is_close!(conv.t_x, 3.5);
        assert_is_close!(conv.a_yx, 4.0);
        assert_is_close!(conv.a_yy, 5.0);
        assert_is_close!(conv.t_y, 8.0);
    }

    #[test]
    fn conversion_then_rotate() {
        use std::f32::consts::FRAC_1_SQRT_2;

        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
            .then_rotate(Angle::degrees(45.0));
        assert_is_close!(conv.a_xx, -3.0 * FRAC_1_SQRT_2);
        assert_is_close!(conv.a_xy, -3.0 * FRAC_1_SQRT_2);
        assert_is_close!(conv.t_x, -3.0 * FRAC_1_SQRT_2);
        assert_is_close!(conv.a_yx, 5.0 * FRAC_1_SQRT_2);
        assert_is_close!(conv.a_yy, 7.0 * FRAC_1_SQRT_2);
        assert_is_close!(conv.t_y, 9.0 * FRAC_1_SQRT_2);
    }

    #[test]
    fn conversion_pre_scale() {
        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).pre_scale(0.5, 2.0);
        assert_is_close!(conv.a_xx, 0.5);
        assert_is_close!(conv.a_xy, 1.0);
        assert_is_close!(conv.t_x, 3.0);
        assert_is_close!(conv.a_yx, 8.0);
        assert_is_close!(conv.a_yy, 10.0);
        assert_is_close!(conv.t_y, 6.0);
    }

    #[test]
    fn conversion_pre_translate() {
        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
            .pre_translate(Vector::new(Test(0.5), Test(2.0)));
        assert_is_close!(conv.a_xx, 1.0);
        assert_is_close!(conv.a_xy, 2.0);
        assert_is_close!(conv.t_x, 7.5);
        assert_is_close!(conv.a_yx, 4.0);
        assert_is_close!(conv.a_yy, 5.0);
        assert_is_close!(conv.t_y, 18.0);
    }

    #[test]
    fn conversion_pre_rotate() {
        use std::f32::consts::FRAC_1_SQRT_2;

        declare_units! {
            Test = 1.0;
        }

        let conv = Conversion::<Mm, Test>::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
            .pre_rotate(Angle::degrees(45.0));
        assert_is_close!(conv.a_xx, 3.0 * FRAC_1_SQRT_2);
        assert_is_close!(conv.a_xy, FRAC_1_SQRT_2);
        assert_is_close!(conv.t_x, 3.0);
        assert_is_close!(conv.a_yx, 9.0 * FRAC_1_SQRT_2);
        assert_is_close!(conv.a_yy, FRAC_1_SQRT_2);
        assert_is_close!(conv.t_y, 6.0);
    }
}
