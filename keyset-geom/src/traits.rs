use crate::{Angle, Point, Rect, Size, Vector};

/// Trait to add additional constructor to `Rect`
pub trait ExtRect<U> {
    /// Create a new `Rect` given a center point and size
    fn from_center_and_size(center: Point<U>, size: Size<U>) -> Self;
}

impl<U> ExtRect<U> for Rect<U> {
    #[inline]
    fn from_center_and_size(center: Point<U>, size: Size<U>) -> Self {
        Self::from_origin_and_size(center - size / 2.0, size)
    }
}

/// Trait to rotate a `Vector`
pub trait ExtVec<T, U> {
    /// Rotate the vector by the given angle
    #[must_use]
    fn rotate(self, angle: T) -> Self;

    /// Negate the x component of the vector
    #[must_use]
    fn neg_x(self) -> Self;

    /// Negate the y component of the vector
    #[must_use]
    fn neg_y(self) -> Self;
}

impl<U> ExtVec<Angle, U> for Vector<U> {
    #[inline]
    fn rotate(self, angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    #[inline]
    fn neg_x(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(-x, y)
    }

    #[inline]
    fn neg_y(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(x, -y)
    }
}

impl<U> ExtVec<euclid::Angle<f64>, U> for euclid::Vector2D<f64, U> {
    #[inline]
    fn rotate(self, angle: euclid::Angle<f64>) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    #[inline]
    fn neg_x(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(-x, y)
    }

    #[inline]
    fn neg_y(self) -> Self {
        let (x, y) = self.to_tuple();
        Self::new(x, -y)
    }
}
