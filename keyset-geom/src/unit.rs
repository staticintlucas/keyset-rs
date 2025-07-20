use isclose::IsClose;

use crate::Scale;

/// Keyboard Unit, usually 19.05 mm or 0.75 in
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Unit(pub f32);

impl Unit {
    const PER_DOT: f32 = 1.0 / Dot::PER_UNIT;
    const PER_MM: f32 = 1.0 / Mm::PER_UNIT;
    const PER_INCH: f32 = 1.0 / Inch::PER_UNIT;
}

impl From<Dot> for Unit {
    #[inline]
    fn from(value: Dot) -> Self {
        Self(value.0 * Self::PER_DOT)
    }
}

impl From<Mm> for Unit {
    #[inline]
    fn from(value: Mm) -> Self {
        Self(value.0 * Self::PER_MM)
    }
}

impl From<Inch> for Unit {
    #[inline]
    fn from(value: Inch) -> Self {
        Self(value.0 * Self::PER_INCH)
    }
}

impl From<f32> for Unit {
    #[inline]
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Unit> for f32 {
    #[inline]
    fn from(value: Unit) -> Self {
        value.0
    }
}

impl IsClose<f32> for Unit {
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

/// Dot, a.k.a. drawing unit
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Dot(pub f32);

impl Dot {
    const PER_UNIT: f32 = 1000.0;
    const PER_MM: f32 = Self::PER_UNIT / Mm::PER_UNIT;
    const PER_INCH: f32 = Self::PER_UNIT / Inch::PER_UNIT;
}

impl From<Unit> for Dot {
    #[inline]
    fn from(value: Unit) -> Self {
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

/// Millimeter
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Mm(pub f32);

impl Mm {
    const PER_UNIT: f32 = 19.05;
    const PER_DOT: f32 = Self::PER_UNIT / Dot::PER_UNIT;
    const PER_INCH: f32 = Self::PER_UNIT / Inch::PER_UNIT;
}

impl From<Unit> for Mm {
    #[inline]
    fn from(value: Unit) -> Self {
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

/// Inch
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Inch(pub f32);

impl Inch {
    const PER_UNIT: f32 = 0.75;
    const PER_DOT: f32 = Self::PER_UNIT / Dot::PER_UNIT;
    const PER_MM: f32 = Self::PER_UNIT / Mm::PER_UNIT;
}

impl From<Unit> for Inch {
    #[inline]
    fn from(value: Unit) -> Self {
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

// TODO: delete these when no longer used
/// Conversion factor for keyboard units to drawing units
pub const DOT_PER_UNIT: Scale<Unit, Dot> = Scale::new(Dot::PER_UNIT);
/// Conversion factor for keyboard units to millimeters
pub const MM_PER_UNIT: Scale<Unit, Mm> = Scale::new(Mm::PER_UNIT);
/// Conversion factor for keyboard units to inches
pub const INCH_PER_UNIT: Scale<Unit, Inch> = Scale::new(Inch::PER_UNIT);

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
    fn unit() {
        let unit = Unit::from(Dot(500.0));
        assert_is_close!(unit.0, 0.5);

        let unit = Unit::from(Mm(38.1));
        assert_is_close!(unit.0, 2.0);

        let unit = Unit::from(Inch(1.0));
        assert_is_close!(unit.0, 4.0 / 3.0);

        let unit = Unit::from(3.0);
        assert_is_close!(unit.0, 3.0);

        let flt = f32::from(Unit(2.5));
        assert_is_close!(flt, 2.5);

        let flt = f32::from(Unit(2.5));
        assert_is_close!(flt, 2.5);

        assert!(Unit(2.5).is_close(Unit(5.0 / 2.0)));
        assert!(!Unit(2.5).is_close(Unit(5.1 / 2.0)));
    }

    #[test]
    fn dot() {
        let unit = Dot::from(Unit(0.5));
        assert_is_close!(unit.0, 500.0);

        let unit = Dot::from(Mm(38.1));
        assert_is_close!(unit.0, 2000.0);

        let unit = Dot::from(Inch(1.0));
        assert_is_close!(unit.0, 4000.0 / 3.0);

        let unit = Dot::from(3.0);
        assert_is_close!(unit.0, 3.0);

        let flt = f32::from(Dot(2.5));
        assert_is_close!(flt, 2.5);

        let flt = f32::from(Dot(2.5));
        assert_is_close!(flt, 2.5);

        assert!(Dot(2.5).is_close(Dot(5.0 / 2.0)));
        assert!(!Dot(2.5).is_close(Dot(5.1 / 2.0)));
    }

    #[test]
    fn mm() {
        let unit = Mm::from(Unit(0.5));
        assert_is_close!(unit.0, 9.525);

        let unit = Mm::from(Dot(2000.0));
        assert_is_close!(unit.0, 38.1);

        let unit = Mm::from(Inch(1.0));
        assert_is_close!(unit.0, 25.4);

        let unit = Mm::from(3.0);
        assert_is_close!(unit.0, 3.0);

        let flt = f32::from(Mm(2.5));
        assert_is_close!(flt, 2.5);

        let flt = f32::from(Mm(2.5));
        assert_is_close!(flt, 2.5);

        assert!(Mm(2.5).is_close(Mm(5.0 / 2.0)));
        assert!(!Mm(2.5).is_close(Mm(5.1 / 2.0)));
    }

    #[test]
    fn inch() {
        let unit = Inch::from(Unit(4.0 / 3.0));
        assert_is_close!(unit.0, 1.0);

        let unit = Inch::from(Dot(2000.0));
        assert_is_close!(unit.0, 1.5);

        let unit = Inch::from(Mm(19.05));
        assert_is_close!(unit.0, 0.75);

        let unit = Inch::from(3.0);
        assert_is_close!(unit.0, 3.0);

        let flt = f32::from(Inch(2.5));
        assert_is_close!(flt, 2.5);

        let flt = f32::from(Inch(2.5));
        assert_is_close!(flt, 2.5);

        assert!(Inch(2.5).is_close(Inch(5.0 / 2.0)));
        assert!(!Inch(2.5).is_close(Inch(5.1 / 2.0)));
    }
}
