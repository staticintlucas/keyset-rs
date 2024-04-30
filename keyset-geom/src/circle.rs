use std::borrow::Borrow;

use isclose::IsClose;

use crate::{Length, Point};

/// A circle
#[derive(Debug, PartialEq)]
pub struct Circle<U> {
    /// Center point
    pub center: Point<U>,
    /// Radius size
    pub radius: Length<U>,
}

// Impl here rather than derive so we don't require U: Clone everywhere
impl<U> Clone for Circle<U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<U> Copy for Circle<U> {}

impl<U> Circle<U> {
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

impl<U> IsClose<f32> for Circle<U> {
    const ABS_TOL: f32 = f32::ABS_TOL;
    const REL_TOL: f32 = f32::REL_TOL;

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
mod tests {
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn circle_new() {
        let circle = Circle::<()>::new(Point::new(1.0, 2.0), Length::new(0.5));

        assert_is_close!(circle.center, Point::<()>::new(1.0, 2.0));
        assert_is_close!(circle.radius, Length::<()>::new(0.5));
    }

    #[test]
    fn circle_from_center_and_diameter() {
        let circle = Circle::<()>::from_center_and_diameter(Point::new(1.0, 2.0), Length::new(2.0));

        assert_is_close!(circle.center, Point::<()>::new(1.0, 2.0));
        assert_is_close!(circle.radius, Length::<()>::new(1.0));
    }
}
