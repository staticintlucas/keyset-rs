use std::borrow::Borrow;

use isclose::IsClose;

use crate::{Dist, Point, Unit};

/// A circle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle<U: Unit> {
    /// Center point
    pub center: Point<U>,
    /// Radius size
    pub radius: Dist<U>,
}

impl<U> Circle<U>
where
    U: Unit,
{
    /// Create a new circle with the given center and radius.
    #[inline]
    #[must_use]
    pub const fn new(center: Point<U>, radius: Dist<U>) -> Self {
        Self { center, radius }
    }

    /// Create a new circle with the given center and diameter.
    #[inline]
    #[must_use]
    pub fn from_center_and_diameter(center: Point<U>, diameter: Dist<U>) -> Self {
        Self::new(center, diameter / 2.0)
    }
}

impl<U> IsClose<f32> for Circle<U>
where
    U: Unit,
{
    const ABS_TOL: f32 = f32::ABS_TOL;
    const REL_TOL: f32 = f32::REL_TOL;

    #[inline]
    fn is_close_tol(
        &self,
        other: impl Borrow<Self>,
        rel_tol: impl Borrow<f32>,
        abs_tol: impl Borrow<f32>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &f32, &f32) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.center.is_close_tol(other.center, rel_tol, abs_tol)
            && self.radius.is_close_tol(other.radius, rel_tol, abs_tol)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use isclose::assert_is_close;

    use crate::Mm;

    use super::*;

    #[test]
    fn circle_new() {
        let circle = Circle::new(Point::new(1.0, 2.0), Dist::new(Mm(0.5)));

        assert_is_close!(circle.center, Point::new(1.0, 2.0));
        assert_is_close!(circle.radius, Dist::new(Mm(0.5)));
    }

    #[test]
    fn circle_from_center_and_diameter() {
        let circle = Circle::from_center_and_diameter(Point::new(1.0, 2.0), Dist::new(Mm(2.0)));

        assert_is_close!(circle.center, Point::new(1.0, 2.0));
        assert_is_close!(circle.radius, Dist::new(Mm(1.0)));
    }
}
