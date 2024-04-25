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

#[cfg(test)]
mod tests {
    use euclid::{approxeq::ApproxEq, Length};

    use super::*;

    #[test]
    fn circle_new() {
        let circle = Circle::<()>::new(Point::new(1.0, 2.0), Length::new(0.5));

        assert!(circle.center.approx_eq(&Point::new(1.0, 2.0)));
        assert!(circle.radius.approx_eq(&Length::new(0.5)));
    }

    #[test]
    fn circle_from_center_and_diameter() {
        let circle = Circle::<()>::from_center_and_diameter(Point::new(1.0, 2.0), Length::new(2.0));

        assert!(circle.center.approx_eq(&Point::new(1.0, 2.0)));
        assert!(circle.radius.approx_eq(&Length::new(1.0)));
    }
}
