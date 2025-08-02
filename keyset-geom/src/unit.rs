use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops;

use isclose::IsClose;

/// Trait for Unit types
pub trait Unit:
    Sized
    + Copy
    + Clone
    + Debug
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

    /// Convert the unit to another unit using the given conversion
    #[inline]
    fn convert<V: Unit>(self, conversion: Conversion<V, Self>) -> V {
        V::new(self.get() * conversion.get())
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
                    $crate::__paste! {
                        $(const [<PER_ $name:snake:upper>]: f32 = $self_conv / $conv;)+
                    }
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

        $(
            $(#[$attr])*
            #[derive(Clone, Copy, Debug, Default)]
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

            impl $crate::__IsClose for $name {
                type Tolerance = f32;
                const ZERO_TOL: Self::Tolerance = 0.0;
                const ABS_TOL: Self::Tolerance = <Self::Tolerance as $crate::__IsClose>::ABS_TOL;
                const REL_TOL: Self::Tolerance = <Self::Tolerance as $crate::__IsClose>::REL_TOL;

                #[inline]
                fn is_close_tol(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
                    self.0.is_close_tol(&other.0, abs_tol, rel_tol)
                }
            }

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

/// A conversion between two units
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Conversion<Dst: Unit, Src: Unit> {
    factor: f32,
    _phantom: PhantomData<(Dst, Src)>,
}

impl<Dst, Src> Conversion<Dst, Src>
where
    Dst: Unit,
    Src: Unit,
{
    /// Create a new conversion with the given conversion factor
    #[inline]
    #[must_use]
    pub const fn new(factor: f32) -> Self {
        Self {
            factor,
            _phantom: PhantomData,
        }
    }

    /// Gets the conversion factor
    #[inline]
    #[must_use]
    pub const fn get(self) -> f32 {
        self.factor
    }

    /// Create a new conversion from two equivalent values in different units
    #[inline]
    pub fn from(dest: Dst, source: Src) -> Self {
        Self {
            factor: dest.get() / source.get(),
            _phantom: PhantomData,
        }
    }

    /// Inverts the conversion
    #[inline]
    #[must_use]
    pub fn inverse(self) -> Conversion<Src, Dst> {
        Conversion {
            factor: 1.0 / self.factor,
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
    fn key_unit_convert() {
        let conversion = Conversion::<Mm, KeyUnit> {
            factor: 19.05,
            _phantom: PhantomData,
        };

        let key_unit = KeyUnit(4.0 / 3.0);
        assert_is_close!(key_unit.convert(conversion), Mm(25.4));
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
    fn dot_convert() {
        let conversion = Conversion::<KeyUnit, Dot> {
            factor: 1.0 / 1000.0,
            _phantom: PhantomData,
        };

        let dot = Dot(2500.0);
        assert_is_close!(dot.convert(conversion), KeyUnit(2.5));
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
    fn mm_convert() {
        let conversion = Conversion::<Dot, Mm> {
            factor: 100_000.0 / 1905.0,
            _phantom: PhantomData,
        };

        let mm = Mm(25.4);
        assert_is_close!(mm.convert(conversion), Dot(4000.0 / 3.0));
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
    fn inch_convert() {
        let conversion = Conversion::<Mm, Inch> {
            factor: 25.4,
            _phantom: PhantomData,
        };

        let inch = Inch(0.75);
        assert_is_close!(inch.convert(conversion), Mm(19.05));
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
    }

    #[test]
    fn conversion_new() {
        let conversion = Conversion::<Mm, Inch>::new(25.4);
        assert_is_close!(conversion.factor, 25.4);
    }

    #[test]
    fn conversion_get() {
        let conversion = Conversion::<Mm, Inch> {
            factor: 25.4,
            _phantom: PhantomData,
        };
        assert_is_close!(conversion.get(), 25.4);
    }

    #[test]
    fn conversion_from() {
        let conversion = Conversion::from(Mm(25.4), Inch(1.0));
        assert_is_close!(conversion.factor, 25.4);
    }

    #[test]
    fn conversion_inverse() {
        let conversion = Conversion::<Mm, Inch> {
            factor: 25.4,
            _phantom: PhantomData,
        }
        .inverse();
        assert_is_close!(conversion.factor, 1.0 / 25.4);
    }
}
