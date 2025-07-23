use isclose::IsClose;

use crate::{Length, Point, Unit};

/// A circle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle<U: Unit> {
    /// Center point
    pub center: Point<U>,
    /// Radius size
    pub radius: Length<U>,
}

impl<U> Circle<U>
where
    U: Unit,
{
    /// Create a new circle with the given center and radius.
    #[inline]
    #[must_use]
    pub const fn new(center: Point<U>, radius: Length<U>) -> Self {
        Self { center, radius }
    }

    /// Create a new circle with the given center and diameter.
    #[inline]
    #[must_use]
    pub fn from_center_and_diameter(center: Point<U>, diameter: Length<U>) -> Self {
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
    fn is_close_impl(&self, other: &Self, rel_tol: &f32, abs_tol: &f32) -> bool {
        self.center.is_close_impl(&other.center, rel_tol, abs_tol)
            && self.radius.is_close_impl(&other.radius, rel_tol, abs_tol)
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
        let circle = Circle::<Mm>::new(Point::new(1.0, 2.0), Length::new(0.5));

        assert_is_close!(circle.center, Point::new(1.0, 2.0));
        assert_is_close!(circle.radius, Length::new(0.5));
    }

    #[test]
    fn circle_from_center_and_diameter() {
        let circle = Circle::<Mm>::from_center_and_diameter(Point::new(1.0, 2.0), Length::new(2.0));

        assert_is_close!(circle.center, Point::new(1.0, 2.0));
        assert_is_close!(circle.radius, Length::new(1.0));
    }
}
